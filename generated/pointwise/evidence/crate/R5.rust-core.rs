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
                        let a6 = 53u64;
                        if if nat_eq(a5, a6) {
                            true
                        } else {
                            let a7 = v1;
                            let a8 = 54u64;
                            nat_eq(a7, a8)
                        } {
                            Ok(Adt9::C0(f2({
                                let a9 = v1;
                                let a10 = 54u64;
                                nat_eq(a9, a10)
                            }, v2, v7, v4, v6)?))
                        } else {
                            let a11 = v1;
                            let a12 = 78u64;
                            if nat_eq(a11, a12) {
                                Ok(Adt9::C0(f3(v2, v7, v4, v5, v6)?))
                            } else {
                                let a13 = v1;
                                let a14 = 23u64;
                                if if if nat_eq(a13, a14) {
                                    true
                                } else {
                                    let a15 = v1;
                                    let a16 = 26u64;
                                    nat_eq(a15, a16)
                                } {
                                    true
                                } else {
                                    let a17 = v1;
                                    let a18 = 25u64;
                                    nat_eq(a17, a18)
                                } {
                                    Ok(Adt9::C0(f4(v1, v2, v7, v4, v6)?))
                                } else {
                                    let a19 = v1;
                                    let a20 = 24u64;
                                    if nat_eq(a19, a20) {
                                        Ok(Adt9::C0(f5(v2, v7, v6)?))
                                    } else {
                                        let a21 = v1;
                                        let a22 = 27u64;
                                        if nat_eq(a21, a22) {
                                            Ok(Adt9::C0(f6(v2, v7, v6)?))
                                        } else {
                                            let a23 = v1;
                                            let a24 = 3u64;
                                            if nat_eq(a23, a24) {
                                                Ok(Adt9::C0(f7(v7, v4, v5, v6)?))
                                            } else {
                                                f8(v1, v2, v7, v4, v6)
                                            }
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
            f10(v13, v14, v15, v16, v17)
        }
        Adt0::C2(v28) => {
            f12(v28)
        }
        Adt0::C3(v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41) => {
            Ok(f13(v29, v30, v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            f9(v8, v9, v10, v11)
        }
        Adt0::C5(v12) => {
            Ok(Adt9::C9({
                let a25 = 3240099840u64;
                let a26 = v12;
                if if nat_le(a25, a26) {
                    let a27 = v12;
                    let a28 = 4286578688u64;
                    nat_le(a27, a28)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a29 = 1092616192u64;
                    let a30 = v12;
                    if if nat_le(a29, a30) {
                        let a31 = v12;
                        let a32 = 2139095040u64;
                        nat_le(a31, a32)
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
            f11(v18, v19, v20, v21, v22, true, v23, v24, v25, v26, v27)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f14(v1.clone(), 1u64)?;
    let a34 = match v1.clone() {
        Adt2::C0(h33, _, _, _, _, _, _, _, _, _, _) => {
            h33
        }
    };
    let a35 = 0u64;
    if if if if if if if if nat_eq(a34, a35) {
        let a37 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h36, _, _, _, _, _) => {
                h36
            }
        };
        let a38 = 4u64;
        nat_eq(a37, a38)
    } else {
        false
    } {
        f15(match v0.clone() {
            Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        })
    } else {
        false
    } {
        f15(match v0.clone() {
            Adt1::C0(_, h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        })
    } else {
        false
    } {
        let a42 = 0u64;
        let a43 = match v0.clone() {
            Adt1::C0(_, _, h41, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        };
        nat_lt(a42, a43)
    } else {
        false
    } {
        let a45 = match v0.clone() {
            Adt1::C0(_, _, h44, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        };
        let a46 = 2139095040u64;
        nat_lt(a45, a46)
    } else {
        false
    } {
        let a47 = v2;
        let a48 = 9223372036854775808u64;
        nat_lt(a47, a48)
    } else {
        false
    } {
        let a50 = f16(v2)?;
        let a51 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h49, _) => {
                h49
            }
        };
        nat_le(a50, a51)
    } else {
        false
    } {
        Ok(Adt10::C4(f17(v2)?, 4u64, f18(v1.clone(), 1u64), f18(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h52, _, _, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        }, match v0.clone() {
            Adt1::C0(_, h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        }, match v0 {
            Adt1::C0(_, _, h54, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m55 = v3;
    match m55 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v1.clone() {
                Adt1::C0(h57, _, _, _, _, _, _, _, _, _, _, _) => {
                    h57
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h58, _, _, _, _, _, _, _, _) => {
                        h58
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h59, _, _, _, _, _, _, _, _, _) => {
                        h59
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h60, _, _, _, _, _) => {
                        h60
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h61, _, _, _, _, _, _, _) => {
                        h61
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
                    h62
                }
            }, match v1.clone() {
                Adt1::C0(_, h63, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h64, _, _, _, _, _, _, _) => {
                        h64
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h65, _, _, _, _, _, _, _, _) => {
                        h65
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h66, _, _, _, _) => {
                        h66
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h67, _, _, _, _, _, _) => {
                        h67
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                        h68
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h69, _, _, _, _, _, _, _, _, _) => {
                        h69
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h70, _, _, _, _, _, _) => {
                        h70
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h71, _, _, _) => {
                        h71
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f19(v6.clone(), match v5.clone() {
                Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                    h72
                }
            })?;
            let v10: u64 = f19(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                    h73
                }
            })?;
            let v11: u64 = if v0 {
                f19(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h74, _, _, _, _, _, _, _) => {
                        h74
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h75, _, _) => {
                        h75
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                        h76
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h77, _) => {
                        h77
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                        h78
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h79) => {
                        h79
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                        h80
                    }
                }
            };
            let v15: u64 = {
                let a85 = {
                    let a81 = v9;
                    let a82 = v10;
                    nat_mul(a81, a82)?
                };
                let a86 = {
                    let a83 = v11;
                    let a84 = v13;
                    nat_mul(a83, a84)?
                };
                nat_mul(a85, a86)?
            };
            let v16: u64 = {
                let a94 = {
                    let a89 = match v6.clone() {
                        Adt6::C0(h87, _, _, _) => {
                            h87
                        }
                    };
                    let a90 = match v7.clone() {
                        Adt6::C0(h88, _, _, _) => {
                            h88
                        }
                    };
                    nat_mul(a89, a90)?
                };
                let a95 = {
                    let a92 = match v8.clone() {
                        Adt6::C0(h91, _, _, _) => {
                            h91
                        }
                    };
                    let a93 = v12;
                    nat_mul(a92, a93)?
                };
                nat_mul(a94, a95)?
            };
            let v17: u64 = {
                let a96 = v15;
                let a97 = 32u64;
                if nat_lt(a96, a97) {
                    v15
                } else {
                    32u64
                }
            };
            let a99 = match v2.clone() {
                Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                    h98
                }
            };
            let a100 = 0u64;
            if if if if if if if if if if if nat_eq(a99, a100) {
                true
            } else {
                let a102 = match v2.clone() {
                    Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                };
                let a103 = 1u64;
                nat_eq(a102, a103)
            } {
                let a105 = match v5.clone() {
                    Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                let a106 = 0u64;
                nat_eq(a105, a106)
            } else {
                false
            } {
                let a108 = match v4.clone() {
                    Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                };
                let a109 = 0u64;
                nat_eq(a108, a109)
            } else {
                false
            } {
                let a111 = match v4.clone() {
                    Adt2::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                        h110
                    }
                };
                let a112 = v9;
                nat_eq(a111, a112)
            } else {
                false
            } {
                let a114 = match v4.clone() {
                    Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                        h113
                    }
                };
                let a115 = v10;
                nat_eq(a114, a115)
            } else {
                false
            } {
                if v0 {
                    let a116 = 0u64;
                    let a117 = v12;
                    if if if if if if if nat_lt(a116, a117) {
                        let a118 = v12;
                        let a119 = 2147483648u64;
                        nat_lt(a118, a119)
                    } else {
                        false
                    } {
                        let a120 = v14;
                        let a121 = 2147483648u64;
                        nat_lt(a120, a121)
                    } else {
                        false
                    } {
                        let a122 = v13;
                        let a123 = 2147483648u64;
                        nat_lt(a122, a123)
                    } else {
                        false
                    } {
                        let a127 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                                h124
                            }
                        };
                        let a128 = {
                            let a125 = v12;
                            let a126 = v14;
                            nat_mul(a125, a126)?
                        };
                        nat_eq(a127, a128)
                    } else {
                        false
                    } {
                        let a132 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h129, _, _, _, _, _, _) => {
                                h129
                            }
                        };
                        let a133 = {
                            let a130 = v12;
                            let a131 = v13;
                            nat_mul(a130, a131)?
                        };
                        nat_eq(a132, a133)
                    } else {
                        false
                    } {
                        let a135 = match v4.clone() {
                            Adt2::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                                h134
                            }
                        };
                        let a136 = v11;
                        nat_eq(a135, a136)
                    } else {
                        false
                    } {
                        let a140 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h137, _, _, _, _, _, _) => {
                                h137
                            }
                        };
                        let a141 = {
                            let a138 = v14;
                            let a139 = v13;
                            nat_mul(a138, a139)?
                        };
                        nat_eq(a140, a141)
                    } else {
                        false
                    }
                } else {
                    let a143 = match v2.clone() {
                        Adt2::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                            h142
                        }
                    };
                    let a144 = v12;
                    if if if nat_eq(a143, a144) {
                        let a146 = match v4.clone() {
                            Adt2::C0(_, _, _, h145, _, _, _, _, _, _, _) => {
                                h145
                            }
                        };
                        let a147 = v14;
                        nat_eq(a146, a147)
                    } else {
                        false
                    } {
                        let a149 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                                h148
                            }
                        };
                        let a150 = v13;
                        nat_eq(a149, a150)
                    } else {
                        false
                    } {
                        let a151 = 0u64;
                        let a152 = v16;
                        nat_lt(a151, a152)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a153 = 0u64;
                let a154 = v14;
                nat_lt(a153, a154)
            } else {
                false
            } {
                let a155 = 0u64;
                let a156 = v15;
                nat_lt(a155, a156)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f20(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a157 = v16;
                    let a158 = v14;
                    nat_mul(a157, a158)?
                }, v17, {
                    let a159 = v17;
                    let a160 = v16;
                    nat_mul(a159, a160)?
                }, {
                    let a161 = v17;
                    let a162 = v14;
                    nat_mul(a161, a162)?
                }, match v2.clone() {
                    Adt2::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                }, f18(v2, 1u64), f18(v5, 1u64), f18(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f3(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m164 = v2;
    match m164 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m165 = v3;
            match m165 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a167 = match v1.clone() {
                        Adt2::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    };
                    let a168 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a167, a168) {
                        let a170 = match v5.clone() {
                            Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        let a171 = 0u64;
                        nat_eq(a170, a171)
                    } else {
                        false
                    } {
                        let a173 = match v6.clone() {
                            Adt2::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
                                h172
                            }
                        };
                        let a174 = 0u64;
                        nat_eq(a173, a174)
                    } else {
                        false
                    } {
                        let a176 = match v4.clone() {
                            Adt2::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                                h175
                            }
                        };
                        let a177 = 0u64;
                        nat_eq(a176, a177)
                    } else {
                        false
                    } {
                        f21(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f21(v1.clone(), v4.clone())
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
                        let a183 = match v1.clone() {
                            Adt2::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        let a184 = {
                            let a181 = match v5.clone() {
                                Adt2::C0(_, h179, _, _, _, _, _, _, _, _, _) => {
                                    h179
                                }
                            };
                            let a182 = match v5.clone() {
                                Adt2::C0(_, h180, _, _, _, _, _, _, _, _, _) => {
                                    h180
                                }
                            };
                            nat_mul(a181, a182)?
                        };
                        nat_eq(a183, a184)
                    } else {
                        false
                    } {
                        let a190 = match v1.clone() {
                            Adt2::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                                h185
                            }
                        };
                        let a191 = {
                            let a188 = match v5.clone() {
                                Adt2::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                                    h186
                                }
                            };
                            let a189 = match v5.clone() {
                                Adt2::C0(_, _, _, h187, _, _, _, _, _, _, _) => {
                                    h187
                                }
                            };
                            nat_mul(a188, a189)?
                        };
                        nat_eq(a190, a191)
                    } else {
                        false
                    } {
                        let a194 = match v1.clone() {
                            Adt2::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                                h192
                            }
                        };
                        let a195 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                                h193
                            }
                        };
                        nat_eq(a194, a195)
                    } else {
                        false
                    } {
                        let a197 = match v0.clone() {
                            Adt1::C0(h196, _, _, _, _, _, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        let a198 = 4294967296u64;
                        nat_lt(a197, a198)
                    } else {
                        false
                    } {
                        let a200 = match v0.clone() {
                            Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        let a201 = 0u64;
                        if nat_eq(a200, a201) {
                            if f20(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a204 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h202) => {
                                        h202
                                    }
                                };
                                let a205 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h203) => {
                                        h203
                                    }
                                };
                                nat_eq(a204, a205)
                            }
                        } else {
                            let a208 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h206) => {
                                    h206
                                }
                            };
                            let a209 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h207) => {
                                    h207
                                }
                            };
                            nat_eq(a208, a209)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a211 = match v4.clone() {
                                Adt2::C0(_, h210, _, _, _, _, _, _, _, _, _) => {
                                    h210
                                }
                            };
                            let a212 = 0u64;
                            if if if nat_eq(a211, a212) {
                                true
                            } else {
                                let a214 = match v4.clone() {
                                    Adt2::C0(_, _, h213, _, _, _, _, _, _, _, _) => {
                                        h213
                                    }
                                };
                                let a215 = 0u64;
                                nat_eq(a214, a215)
                            } {
                                true
                            } else {
                                let a217 = match v4.clone() {
                                    Adt2::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                                        h216
                                    }
                                };
                                let a218 = 0u64;
                                nat_eq(a217, a218)
                            } {
                                true
                            } else {
                                let a220 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h219, _, _, _, _, _, _) => {
                                        h219
                                    }
                                };
                                let a221 = 0u64;
                                nat_eq(a220, a221)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                                h222
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a224 = match v0 {
                                Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            let a225 = 0u64;
                            if nat_eq(a224, a225) {
                                f14(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f23(v8, match v5.clone() {
                                Adt2::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                                    h226
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h227, _, _, _, _, _, _, _) => {
                                    h227
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h228, _, _, _, _, _, _) => {
                                    h228
                                }
                            })?
                        };
                        Ok(Adt10::C4(f24(v9, v8, v10)?, 4u64, f18(v1, 1u64), f18(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f25(v2.clone());
    let v6: Adt14 = f25(v4.clone());
    let v8: Adt15 = {
        let m229 = v3;
        match m229 {
            None => {
                f26(v0, match v1.clone() {
                    Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _, _) => {
                        h230
                    }
                }, match v1 {
                    Adt1::C0(_, h231, _, _, _, _, _, _, _, _, _, _) => {
                        h231
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f26(v0, match v1.clone() {
                    Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _, _) => {
                        h232
                    }
                }, match v1 {
                    Adt1::C0(_, h233, _, _, _, _, _, _, _, _, _, _) => {
                        h233
                    }
                }, true, v5, f25(v7), v6)?
            }
        }
    };
    let m234 = v8;
    match m234 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f18(v2, 1u64), f18(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
            h235
        }
    };
    let v4: u64 = f27(v3);
    let a237 = match v1.clone() {
        Adt2::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
            h236
        }
    };
    let a238 = 0u64;
    if if if if if if if if nat_eq(a237, a238) {
        true
    } else {
        let a240 = match v1.clone() {
            Adt2::C0(h239, _, _, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        let a241 = 30u64;
        nat_eq(a240, a241)
    } {
        let a244 = match v1.clone() {
            Adt2::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = match v2.clone() {
            Adt2::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        };
        nat_eq(a244, a245)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f28(match v0.clone() {
            Adt1::C0(h246, _, _, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        })
    } else {
        false
    } {
        f29(v1.clone(), v4)
    } else {
        false
    } {
        f29(v2.clone(), v4)
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f30(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f31(match v1.clone() {
            Adt2::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h248, _, _, _, _, _, _, _, _) => {
                h248
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h249, _, _, _, _, _, _, _) => {
                h249
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h250, _, _, _, _, _, _) => {
                h250
            }
        }) {
            0u64
        } else {
            let a256 = match v1.clone() {
                Adt2::C0(_, _, h251, _, _, _, _, _, _, _, _) => {
                    h251
                }
            };
            let a257 = {
                let a254 = match v1.clone() {
                    Adt2::C0(_, _, _, h252, _, _, _, _, _, _, _) => {
                        h252
                    }
                };
                let a255 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                        h253
                    }
                };
                nat_mul(a254, a255)?
            };
            nat_mul(a256, a257)?
        };
        let v6: u64 = {
            let a258 = v5;
            let a259 = 0u64;
            if if nat_eq(a258, a259) {
                true
            } else {
                f32(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a261 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h260, _) => {
                        h260
                    }
                };
                let a262 = v4;
                let a263 = 0u64;
                nat_quot(a261, a262, a263)
            }
        };
        Ok(Adt10::C4(f33(v3, match v1.clone() {
            Adt2::C0(_, h264, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        }, v5, v6)?, v4, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h265, _, _, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        }, v5, f34(match v1 {
            Adt2::C0(_, h266, _, _, _, _, _, _, _, _, _) => {
                h266
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a268 = match v1.clone() {
        Adt2::C0(h267, _, _, _, _, _, _, _, _, _, _) => {
            h267
        }
    };
    let a269 = 0u64;
    if if if if if nat_eq(a268, a269) {
        let a271 = match v2.clone() {
            Adt2::C0(h270, _, _, _, _, _, _, _, _, _, _) => {
                h270
            }
        };
        let a272 = 0u64;
        nat_eq(a271, a272)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f35(match v0.clone() {
            Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _, _) => {
                h273
            }
        })
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f30(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f18(v1, 1u64);
        let v4: u64 = {
            let a279 = match v3.clone() {
                Adt3::C0(_, h274, _, _, _, _, _, _) => {
                    h274
                }
            };
            let a280 = {
                let a277 = match v3.clone() {
                    Adt3::C0(_, _, h275, _, _, _, _, _) => {
                        h275
                    }
                };
                let a278 = match v3.clone() {
                    Adt3::C0(_, _, _, h276, _, _, _, _) => {
                        h276
                    }
                };
                nat_mul(a277, a278)?
            };
            nat_mul(a279, a280)?
        };
        Ok(Adt10::C4(f36(match v3.clone() {
            Adt3::C0(h281, _, _, _, _, _, _, _) => {
                h281
            }
        }, v4)?, 4u64, v3, f18(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m283 = v1;
    match m283 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m284 = v2;
            match m284 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a286 = match v0.clone() {
                        Adt2::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                            h285
                        }
                    };
                    let a287 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a286, a287) {
                        let a289 = match v4.clone() {
                            Adt2::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                                h288
                            }
                        };
                        let a290 = 0u64;
                        nat_eq(a289, a290)
                    } else {
                        false
                    } {
                        let a292 = match v5.clone() {
                            Adt2::C0(h291, _, _, _, _, _, _, _, _, _, _) => {
                                h291
                            }
                        };
                        let a293 = 26u64;
                        nat_eq(a292, a293)
                    } else {
                        false
                    } {
                        let a295 = match v3.clone() {
                            Adt2::C0(h294, _, _, _, _, _, _, _, _, _, _) => {
                                h294
                            }
                        };
                        let a296 = 0u64;
                        nat_eq(a295, a296)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a299 = match v0.clone() {
                            Adt2::C0(_, h297, _, _, _, _, _, _, _, _, _) => {
                                h297
                            }
                        };
                        let a300 = match v4.clone() {
                            Adt2::C0(_, h298, _, _, _, _, _, _, _, _, _) => {
                                h298
                            }
                        };
                        nat_eq(a299, a300)
                    } else {
                        false
                    } {
                        let a303 = match v0.clone() {
                            Adt2::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        let a304 = match v5.clone() {
                            Adt2::C0(_, h302, _, _, _, _, _, _, _, _, _) => {
                                h302
                            }
                        };
                        nat_eq(a303, a304)
                    } else {
                        false
                    } {
                        let a307 = match v0.clone() {
                            Adt2::C0(_, _, _, h305, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a308 = match v5.clone() {
                            Adt2::C0(_, _, h306, _, _, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        nat_eq(a307, a308)
                    } else {
                        false
                    } {
                        let a310 = 0u64;
                        let a311 = match v5.clone() {
                            Adt2::C0(_, _, _, h309, _, _, _, _, _, _, _) => {
                                h309
                            }
                        };
                        nat_lt(a310, a311)
                    } else {
                        false
                    } {
                        let a313 = 0u64;
                        let a314 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h312, _, _, _, _, _, _) => {
                                h312
                            }
                        };
                        nat_lt(a313, a314)
                    } else {
                        false
                    } {
                        let a316 = 4u64;
                        let a317 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h315, _, _, _, _, _) => {
                                h315
                            }
                        };
                        nat_le(a316, a317)
                    } else {
                        false
                    } {
                        let a319 = 4u64;
                        let a320 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h318, _, _, _, _, _) => {
                                h318
                            }
                        };
                        nat_le(a319, a320)
                    } else {
                        false
                    } {
                        let a322 = 4u64;
                        let a323 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h321, _, _, _, _, _) => {
                                h321
                            }
                        };
                        nat_le(a322, a323)
                    } else {
                        false
                    } {
                        let a325 = 4u64;
                        let a326 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h324, _, _, _, _, _) => {
                                h324
                            }
                        };
                        nat_le(a325, a326)
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
                        Ok(Adt10::C4(f37(match v0.clone() {
                            Adt2::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                                h327
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h328, _, _, _, _, _, _, _, _) => {
                                h328
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h329, _, _, _, _, _, _, _) => {
                                h329
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h330, _, _, _, _, _, _) => {
                                h330
                            }
                        })?, 4u64, f18(v0, 1u64), f18(v3, 1u64), Adt4::C22(f18(v4, 1u64), f18(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h331, _, _, _, _, _, _, _, _, _, _, _) => {
            h331
        }
    };
    Ok(Adt9::C0({
        let a332 = v0;
        let a333 = 52u64;
        if nat_eq(a332, a333) {
            f38(v1, v2, v4)?
        } else {
            let a334 = v0;
            let a335 = 73u64;
            if nat_eq(a334, a335) {
                f39(v2, v3, v4)?
            } else {
                let a336 = v0;
                let a337 = 57u64;
                if if nat_eq(a336, a337) {
                    true
                } else {
                    let a338 = v0;
                    let a339 = 58u64;
                    nat_eq(a338, a339)
                } {
                    let m340 = v3;
                    match m340 {
                        None => {
                            f40({
                                let a341 = v0;
                                let a342 = 58u64;
                                nat_eq(a341, a342)
                            }, v1, v2, v4)?
                        }
                        Some(_) => {
                            Adt10::C1
                        }
                    }
                } else {
                    let a343 = v0;
                    let a344 = 49u64;
                    if nat_eq(a343, a344) {
                        f41(v1, v2, v3, v4)?
                    } else {
                        let a345 = v0;
                        let a346 = 51u64;
                        if nat_eq(a345, a346) {
                            f42(v1, v2, v3, v4)?
                        } else {
                            let a347 = v0;
                            let a348 = 39u64;
                            if nat_eq(a347, a348) {
                                f43(v2, v3, v4)?
                            } else {
                                let a349 = v0;
                                let a350 = 77u64;
                                if nat_eq(a349, a350) {
                                    f44(v2, v4)?
                                } else {
                                    let a351 = v0;
                                    let a352 = 40u64;
                                    if nat_eq(a351, a352) {
                                        f45(v2, v3, v4)?
                                    } else {
                                        if f46(v0) {
                                            let a353 = v0;
                                            let a354 = 33u64;
                                            if if if nat_eq(a353, a354) {
                                                let a356 = match v4.clone() {
                                                    Adt2::C0(h355, _, _, _, _, _, _, _, _, _, _) => {
                                                        h355
                                                    }
                                                };
                                                let a357 = 0u64;
                                                nat_eq(a356, a357)
                                            } else {
                                                false
                                            } {
                                                f47(match v2.clone() {
                                                    Adt2::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                                                        h358
                                                    }
                                                })
                                            } else {
                                                false
                                            } {
                                                f48(v2, v3, v4)?
                                            } else {
                                                f49(v0, v2, {
                                                    let m359 = v3;
                                                    match m359 {
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
                                            let a360 = v0;
                                            let a361 = 19u64;
                                            if nat_eq(a360, a361) {
                                                f50(v2, v4)?
                                            } else {
                                                let a362 = v0;
                                                let a363 = 21u64;
                                                if nat_eq(a362, a363) {
                                                    f51(v5, v2, v3, v4)?
                                                } else {
                                                    let a364 = v0;
                                                    let a365 = 61u64;
                                                    if if if nat_eq(a364, a365) {
                                                        true
                                                    } else {
                                                        let a366 = v0;
                                                        let a367 = 62u64;
                                                        nat_eq(a366, a367)
                                                    } {
                                                        true
                                                    } else {
                                                        let a368 = v0;
                                                        let a369 = 63u64;
                                                        nat_eq(a368, a369)
                                                    } {
                                                        f52(v0, v1, v2, v4)?
                                                    } else {
                                                        let a370 = v0;
                                                        let a371 = 4u64;
                                                        if if nat_eq(a370, a371) {
                                                            true
                                                        } else {
                                                            let a372 = v0;
                                                            let a373 = 32u64;
                                                            nat_eq(a372, a373)
                                                        } {
                                                            f53(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a374 = v0;
                                                            let a375 = 20u64;
                                                            if nat_eq(a374, a375) {
                                                                f54(v2, v4)?
                                                            } else {
                                                                let a376 = v0;
                                                                let a377 = 42u64;
                                                                if nat_eq(a376, a377) {
                                                                    f55(4u64, v2, v4)?
                                                                } else {
                                                                    let a378 = v0;
                                                                    let a379 = 69u64;
                                                                    if nat_eq(a378, a379) {
                                                                        let a380 = v5;
                                                                        let a381 = 4u64;
                                                                        if nat_lt(a380, a381) {
                                                                            f55(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a382 = v0;
                                                                        let a383 = 70u64;
                                                                        if nat_eq(a382, a383) {
                                                                            f56(v5, v2, v4)?
                                                                        } else {
                                                                            let a384 = v0;
                                                                            let a385 = 75u64;
                                                                            if if nat_eq(a384, a385) {
                                                                                true
                                                                            } else {
                                                                                let a386 = v0;
                                                                                let a387 = 76u64;
                                                                                nat_eq(a386, a387)
                                                                            } {
                                                                                f57({
                                                                                    let a388 = v0;
                                                                                    let a389 = 76u64;
                                                                                    nat_eq(a388, a389)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a390 = v0;
                                                                                let a391 = 31u64;
                                                                                if if if nat_eq(a390, a391) {
                                                                                    true
                                                                                } else {
                                                                                    let a392 = v0;
                                                                                    let a393 = 47u64;
                                                                                    nat_eq(a392, a393)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a394 = v0;
                                                                                    let a395 = 68u64;
                                                                                    nat_eq(a394, a395)
                                                                                } {
                                                                                    f58(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a396 = v0;
                                                                                    let a397 = 15u64;
                                                                                    if nat_eq(a396, a397) {
                                                                                        f59(v2, v4)?
                                                                                    } else {
                                                                                        let a398 = v0;
                                                                                        let a399 = 17u64;
                                                                                        if nat_eq(a398, a399) {
                                                                                            f60(v2, v4)?
                                                                                        } else {
                                                                                            let a400 = v0;
                                                                                            let a401 = 18u64;
                                                                                            if nat_eq(a400, a401) {
                                                                                                f61(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a402 = v0;
                                                                                                let a403 = 13u64;
                                                                                                if if if nat_eq(a402, a403) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a404 = v0;
                                                                                                    let a405 = 14u64;
                                                                                                    nat_eq(a404, a405)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a406 = v0;
                                                                                                    let a407 = 16u64;
                                                                                                    nat_eq(a406, a407)
                                                                                                } {
                                                                                                    f62(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m408 = f63(v0, v5)?;
                                                                                                    match m408 {
                                                                                                        None => {
                                                                                                            f64(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f65(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f9(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a409 = v1;
    let a410 = 0u64;
    if nat_eq(a409, a410) {
        let a412 = v3;
        let a413 = match v0.clone() {
            Adt8::C0(_, _, _, h411, _, _, _, _, _, _, _, _, _, _, _) => {
                h411
            }
        };
        if nat_lt(a412, a413) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h414, _, _, _, _, _, _) => {
                    h414
                }
            };
            let v5: u64 = {
                let a416 = v3;
                let a417 = match v0.clone() {
                    Adt8::C0(_, _, h415, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h415
                    }
                };
                let a418 = 0u64;
                nat_quot(a416, a417, a418)
            };
            Ok(Adt9::C8(Some(f66(v4.clone(), f67(v5, v4.clone(), 0u64), f67(v5, v4.clone(), 1u64), f67(v5, v4, 2u64), f68(v0.clone(), v5, {
                let a420 = v3;
                let a421 = match v0 {
                    Adt8::C0(_, _, h419, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h419
                    }
                };
                let a422 = 0u64;
                nat_rem(a420, a421, a422)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h423, _, _, _, _) => {
                h423
            }
        };
        let v7: u64 = {
            let a424 = v1;
            let a425 = 1u64;
            if nat_eq(a424, a425) {
                match v0.clone() {
                    Adt8::C0(_, h426, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h426
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h427, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h427
                    }
                }
            }
        };
        let v8: u64 = {
            let a431 = v2;
            let a432 = {
                let a428 = v3;
                let a429 = v7;
                let a430 = 0u64;
                nat_quot(a428, a429, a430)
            };
            nat_add(a431, a432)?
        };
        let v9: u64 = {
            let a433 = v3;
            let a434 = v7;
            let a435 = 0u64;
            nat_rem(a433, a434, a435)
        };
        let v10: u64 = {
            let a437 = v8;
            let a438 = match v6.clone() {
                Adt3::C0(h436, _, _, _, _, _, _, _) => {
                    h436
                }
            };
            let a439 = 0u64;
            nat_rem(a437, a438, a439)
        };
        let v11: u64 = {
            let a441 = v8;
            let a442 = match v6.clone() {
                Adt3::C0(h440, _, _, _, _, _, _, _) => {
                    h440
                }
            };
            let a443 = 0u64;
            nat_quot(a441, a442, a443)
        };
        let v12: u64 = {
            let a445 = v11;
            let a446 = match v6.clone() {
                Adt3::C0(_, h444, _, _, _, _, _, _) => {
                    h444
                }
            };
            let a447 = 0u64;
            nat_rem(a445, a446, a447)
        };
        let v13: u64 = {
            let a449 = v11;
            let a450 = match v6.clone() {
                Adt3::C0(_, h448, _, _, _, _, _, _) => {
                    h448
                }
            };
            let a451 = 0u64;
            nat_quot(a449, a450, a451)
        };
        let a453 = v8;
        let a454 = match v0.clone() {
            Adt8::C0(h452, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h452
            }
        };
        if nat_lt(a453, a454) {
            let a455 = v1;
            let a456 = 1u64;
            if nat_eq(a455, a456) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h457, _, _, _, _, _, _) => {
                        h457
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h458, _, _, _, _, _) => {
                        h458
                    }
                };
                let m461 = f69(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h459, _, _, _) => {
                        h459
                    }
                }, match v15.clone() {
                    Adt3::C0(h460, _, _, _, _, _, _, _) => {
                        h460
                    }
                }, v10, f67(v9, v14.clone(), 0u64))?;
                match m461 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m464 = f69(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h462, _, _) => {
                                h462
                            }
                        }, match v15 {
                            Adt3::C0(_, h463, _, _, _, _, _, _) => {
                                h463
                            }
                        }, v12, f67(v9, v14, 1u64))?;
                        match m464 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f70(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a465 = v1;
                let a466 = 2u64;
                if nat_eq(a465, a466) {
                    Ok(Adt9::C8(Some({
                        let a468 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h467) => {
                                h467
                            }
                        };
                        let a469 = 0u64;
                        if nat_eq(a468, a469) {
                            f66(v6, v10, v12, v9, v13)?
                        } else {
                            f66(v6.clone(), v10, v12, {
                                let a471 = v13;
                                let a472 = match v6.clone() {
                                    Adt3::C0(_, _, h470, _, _, _, _, _) => {
                                        h470
                                    }
                                };
                                let a473 = 0u64;
                                nat_rem(a471, a472, a473)
                            }, {
                                let a481 = {
                                    let a479 = {
                                        let a475 = v13;
                                        let a476 = match v6 {
                                            Adt3::C0(_, _, h474, _, _, _, _, _) => {
                                                h474
                                            }
                                        };
                                        let a477 = 0u64;
                                        nat_quot(a475, a476, a477)
                                    };
                                    let a480 = match v0 {
                                        Adt8::C0(_, _, h478, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h478
                                        }
                                    };
                                    nat_mul(a479, a480)?
                                };
                                let a482 = v9;
                                nat_add(a481, a482)?
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

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f11(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f11(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f71(v3.clone(), {
        let m483 = v3.clone();
        match m483 {
            Adt4::C0 => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f74(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C19(v51, v52) => {
                f75(v51, v0, v1, v2, v52, v4)?
            }
            Adt4::C20(v53, v54) => {
                f76(v53, v0, v1, v2, v54, v4)?
            }
            Adt4::C21 => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C22(v55, v56) => {
                f77(v0, v1, v2, v55, v56, v4)?
            }
            Adt4::C23(v70, v71, v72, v73) => {
                f84(v0, v73, v70, v71, v72, v4)?
            }
            Adt4::C24(v69) => {
                if v5 {
                    f82(Adt16::C0(v69), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f72(v0, v1, v2, v3)?
            }
            Adt4::C26(v65, v66, v67, v68) => {
                let a484 = v65;
                let a485 = 64u64;
                if if if nat_eq(a484, a485) {
                    true
                } else {
                    let a486 = v65;
                    let a487 = 65u64;
                    nat_eq(a486, a487)
                } {
                    true
                } else {
                    let a488 = v65;
                    let a489 = 66u64;
                    nat_eq(a488, a489)
                } {
                    if v5 {
                        f82(Adt16::C1(v65, v66, v67, v68), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a490 = v65;
                    let a491 = 0u64;
                    if if nat_eq(a490, a491) {
                        true
                    } else {
                        let a492 = v65;
                        let a493 = 30u64;
                        nat_eq(a492, a493)
                    } {
                        f83(v0, v1, v2, v65, v66, v67, v68, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v64) => {
                f81(v0, v1, v2, v64)?
            }
            Adt4::C28(v60) => {
                f78(v0, v1, v2, v60, v4)?
            }
            Adt4::C29(v61) => {
                f79(v0, v1, v2, v61)?
            }
            Adt4::C30(v62, _) => {
                f80(v0, v1, v2, v62)?
            }
            Adt4::C31(v33, v34) => {
                f73(v0, v1, v2, v33, v34)?
            }
        }
    }))
}

pub fn f12(v0: Adt4) -> R<Adt9> {
    let m494 = v0;
    match m494 {
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
                let a499 = {
                    let a495 = 0u64;
                    let a496 = v47;
                    if nat_lt(a495, a496) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a500 = {
                    let a497 = 0u64;
                    let a498 = v49;
                    if nat_lt(a497, a498) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a499, a500)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v55, _, _, _) => {
            Ok(Adt9::C6({
                let a501 = v55;
                let a502 = 66u64;
                if nat_eq(a501, a502) {
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

pub fn f13(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a503 = v1;
    let a504 = 5u64;
    if if if if nat_lt(a503, a504) {
        let a505 = v5;
        let a506 = 5u64;
        nat_lt(a505, a506)
    } else {
        false
    } {
        let a507 = v7;
        let a508 = 5u64;
        nat_lt(a507, a508)
    } else {
        false
    } {
        let a509 = v9;
        let a510 = 2u64;
        if nat_lt(a509, a510) {
            true
        } else {
            let a511 = v9;
            let a512 = 3u64;
            nat_eq(a511, a512)
        }
    } else {
        false
    } {
        Adt9::C7(f85(v0, v1, v2, v3, v4), f86(v5, v6), f86(v7, v8), f87({
            let a513 = v9;
            let a514 = 0u64;
            if nat_eq(a513, a514) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> R<u64> {
    let a526 = {
        let a520 = {
            let a516 = match v0.clone() {
                Adt2::C0(_, h515, _, _, _, _, _, _, _, _, _) => {
                    h515
                }
            };
            let a517 = v1;
            let a518 = 0u64;
            nat_quot(a516, a517, a518)
        };
        let a521 = match v0.clone() {
            Adt2::C0(_, _, h519, _, _, _, _, _, _, _, _) => {
                h519
            }
        };
        nat_mul(a520, a521)?
    };
    let a527 = {
        let a524 = match v0.clone() {
            Adt2::C0(_, _, _, h522, _, _, _, _, _, _, _) => {
                h522
            }
        };
        let a525 = match v0 {
            Adt2::C0(_, _, _, _, h523, _, _, _, _, _, _) => {
                h523
            }
        };
        nat_mul(a524, a525)?
    };
    nat_mul(a526, a527)
}

pub fn f15(v0: u64) -> bool {
    let a528 = v0;
    let a529 = 4294967296u64;
    if nat_lt(a528, a529) {
        let a533 = {
            let a530 = v0;
            let a531 = 2147483648u64;
            let a532 = 0u64;
            nat_rem(a530, a531, a532)
        };
        let a534 = 2139095040u64;
        nat_lt(a533, a534)
    } else {
        false
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a535 = 4u64;
    let a536 = v0;
    nat_mul(a535, a536)
}

pub fn f17(v0: u64) -> R<u64> {
    let a537 = v0;
    let a538 = 0u64;
    if nat_eq(a537, a538) {
        Ok(0u64)
    } else {
        let a541 = 4u64;
        let a542 = {
            let a539 = 3u64;
            let a540 = v0;
            nat_mul(a539, a540)?
        };
        nat_add(a541, a542)
    }
}

pub fn f18(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a544 = match v0.clone() {
            Adt2::C0(_, h543, _, _, _, _, _, _, _, _, _) => {
                h543
            }
        };
        let a545 = v1;
        let a546 = 0u64;
        nat_quot(a544, a545, a546)
    }, match v0.clone() {
        Adt2::C0(_, _, h547, _, _, _, _, _, _, _, _) => {
            h547
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h548, _, _, _, _, _, _, _) => {
            h548
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h549, _, _, _, _, _, _) => {
            h549
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h550, _, _, _, _, _) => {
            h550
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h551, _, _, _, _) => {
            h551
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h552, _, _, _) => {
            h552
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h553, _, _) => {
            h553
        }
    })
}

pub fn f19(v0: Adt6, v1: u64) -> R<u64> {
    let a555 = match v0.clone() {
        Adt6::C0(h554, _, _, _) => {
            h554
        }
    };
    let a556 = 0u64;
    if nat_eq(a555, a556) {
        f88(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h557, _, _) => {
                h557
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h558, _) => {
                h558
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h559) => {
                h559
            }
        }), {
            let a561 = v1;
            let a562 = match v0 {
                Adt6::C0(_, _, _, h560) => {
                    h560
                }
            };
            nat_add(a561, a562)?
        })
    } else {
        f88(v0, v1)
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> R<bool> {
    let a564 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h563, _) => {
            h563
        }
    };
    let a565 = 0u64;
    if nat_eq(a564, a565) {
        Ok(true)
    } else {
        let a567 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h566, _) => {
                h566
            }
        };
        let a568 = 0u64;
        if nat_eq(a567, a568) {
            Ok(true)
        } else {
            let a570 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h569) => {
                    h569
                }
            };
            let a571 = 0u64;
            if nat_eq(a570, a571) {
                Ok(true)
            } else {
                let a573 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h572) => {
                        h572
                    }
                };
                let a574 = 0u64;
                if nat_eq(a573, a574) {
                    Ok(true)
                } else {
                    let a580 = {
                        let a577 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h575) => {
                                h575
                            }
                        };
                        let a578 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h576, _) => {
                                h576
                            }
                        };
                        nat_add(a577, a578)?
                    };
                    let a581 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h579) => {
                            h579
                        }
                    };
                    if nat_le(a580, a581) {
                        Ok(true)
                    } else {
                        let a587 = {
                            let a584 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h582) => {
                                    h582
                                }
                            };
                            let a585 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h583, _) => {
                                    h583
                                }
                            };
                            nat_add(a584, a585)?
                        };
                        let a588 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h586) => {
                                h586
                            }
                        };
                        Ok(nat_le(a587, a588))
                    }
                }
            }
        }
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> bool {
    let a591 = match v0.clone() {
        Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    };
    let a592 = match v1.clone() {
        Adt2::C0(_, h590, _, _, _, _, _, _, _, _, _) => {
            h590
        }
    };
    if if if nat_eq(a591, a592) {
        let a595 = match v0.clone() {
            Adt2::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
                h593
            }
        };
        let a596 = match v1.clone() {
            Adt2::C0(_, _, h594, _, _, _, _, _, _, _, _) => {
                h594
            }
        };
        nat_eq(a595, a596)
    } else {
        false
    } {
        let a599 = match v0.clone() {
            Adt2::C0(_, _, _, h597, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a600 = match v1.clone() {
            Adt2::C0(_, _, _, h598, _, _, _, _, _, _, _) => {
                h598
            }
        };
        nat_eq(a599, a600)
    } else {
        false
    } {
        let a603 = match v0 {
            Adt2::C0(_, _, _, _, h601, _, _, _, _, _, _) => {
                h601
            }
        };
        let a604 = match v1 {
            Adt2::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                h602
            }
        };
        nat_eq(a603, a604)
    } else {
        false
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a606 = match v0.clone() {
        Adt2::C0(_, h605, _, _, _, _, _, _, _, _, _) => {
            h605
        }
    };
    let a607 = 1u64;
    if if if if nat_eq(a606, a607) {
        true
    } else {
        let a609 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h608, _, _, _, _, _) => {
                h608
            }
        };
        let a610 = 4u64;
        nat_eq(a609, a610)
    } {
        let a612 = match v0.clone() {
            Adt2::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                h611
            }
        };
        let a613 = 1u64;
        if nat_eq(a612, a613) {
            true
        } else {
            let a618 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h614, _, _, _, _) => {
                    h614
                }
            };
            let a619 = {
                let a616 = match v0.clone() {
                    Adt2::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                        h615
                    }
                };
                let a617 = 4u64;
                nat_mul(a616, a617)?
            };
            nat_eq(a618, a619)
        }
    } else {
        false
    } {
        let a621 = match v0.clone() {
            Adt2::C0(_, _, _, h620, _, _, _, _, _, _, _) => {
                h620
            }
        };
        let a622 = 1u64;
        if nat_eq(a621, a622) {
            true
        } else {
            let a630 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h623, _, _, _) => {
                    h623
                }
            };
            let a631 = {
                let a628 = {
                    let a626 = match v0.clone() {
                        Adt2::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                            h624
                        }
                    };
                    let a627 = match v0.clone() {
                        Adt2::C0(_, _, h625, _, _, _, _, _, _, _, _) => {
                            h625
                        }
                    };
                    nat_mul(a626, a627)?
                };
                let a629 = 4u64;
                nat_mul(a628, a629)?
            };
            nat_eq(a630, a631)
        }
    } else {
        false
    } {
        let a633 = match v0.clone() {
            Adt2::C0(_, _, _, _, h632, _, _, _, _, _, _) => {
                h632
            }
        };
        let a634 = 1u64;
        if nat_eq(a633, a634) {
            Ok(true)
        } else {
            let a645 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h635, _, _) => {
                    h635
                }
            };
            let a646 = {
                let a643 = {
                    let a641 = {
                        let a638 = match v0.clone() {
                            Adt2::C0(_, h636, _, _, _, _, _, _, _, _, _) => {
                                h636
                            }
                        };
                        let a639 = match v0.clone() {
                            Adt2::C0(_, _, h637, _, _, _, _, _, _, _, _) => {
                                h637
                            }
                        };
                        nat_mul(a638, a639)?
                    };
                    let a642 = match v0 {
                        Adt2::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                            h640
                        }
                    };
                    nat_mul(a641, a642)?
                };
                let a644 = 4u64;
                nat_mul(a643, a644)?
            };
            Ok(nat_eq(a645, a646))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a651 = {
        let a647 = v0;
        let a648 = v1;
        nat_mul(a647, a648)?
    };
    let a652 = {
        let a649 = v2;
        let a650 = f89(v3);
        nat_mul(a649, a650)?
    };
    nat_mul(a651, a652)
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a655 = v0;
    let a656 = {
        let a653 = f90(v1)?;
        let a654 = v2;
        nat_mul(a653, a654)?
    };
    nat_add(a655, a656)
}

pub fn f25(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h657, _, _, _, _, _, _, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt2::C0(_, h658, _, _, _, _, _, _, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
            h660
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h661, _, _, _, _, _, _) => {
            h661
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h662, _, _, _, _, _) => {
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
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h665, _, _) => {
            h665
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h666, _) => {
            h666
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h667) => {
            h667
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a668 = v0;
    let a669 = 23u64;
    if nat_eq(a668, a669) {
        f91(v1, v4, v6)
    } else {
        let a670 = v0;
        let a671 = 26u64;
        if nat_eq(a670, a671) {
            f92(v1, v2, v4, v6)
        } else {
            let a672 = v0;
            let a673 = 25u64;
            if nat_eq(a672, a673) {
                f93(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    let a674 = v0;
    let a675 = 30u64;
    if nat_eq(a674, a675) {
        2u64
    } else {
        4u64
    }
}

pub fn f28(v0: u64) -> bool {
    let a676 = v0;
    let a677 = 2139095040u64;
    if nat_le(a676, a677) {
        true
    } else {
        let a678 = v0;
        let a679 = 2147483648u64;
        nat_eq(a678, a679)
    }
}

pub fn f29(v0: Adt2, v1: u64) -> bool {
    let a681 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h680, _, _, _, _, _) => {
            h680
        }
    };
    let a682 = v1;
    if if nat_eq(a681, a682) {
        let a687 = {
            let a684 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h683) => {
                    h683
                }
            };
            let a685 = v1;
            let a686 = 0u64;
            nat_rem(a684, a685, a686)
        };
        let a688 = 0u64;
        nat_eq(a687, a688)
    } else {
        false
    } {
        f94(match v0.clone() {
            Adt2::C0(_, _, h689, _, _, _, _, _, _, _, _) => {
                h689
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h690, _, _, _, _, _, _, _) => {
                h690
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h691, _, _, _, _, _, _) => {
                h691
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h692, _, _, _, _) => {
                h692
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h693, _, _, _) => {
                h693
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h694, _, _) => {
                h694
            }
        }, v1)
    } else {
        false
    }
}

pub fn f30(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a697 = match v0.clone() {
            Adt2::C0(h695, _, _, _, _, _, _, _, _, _, _) => {
                h695
            }
        };
        let a698 = match v1.clone() {
            Adt2::C0(h696, _, _, _, _, _, _, _, _, _, _) => {
                h696
            }
        };
        nat_eq(a697, a698)
    } else {
        false
    } {
        let a701 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h699) => {
                h699
            }
        };
        let a702 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h700) => {
                h700
            }
        };
        nat_eq(a701, a702)
    } else {
        false
    } {
        let a705 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h703, _, _, _, _, _) => {
                h703
            }
        };
        let a706 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h704, _, _, _, _, _) => {
                h704
            }
        };
        nat_eq(a705, a706)
    } else {
        false
    } {
        let a709 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h707, _, _, _, _) => {
                h707
            }
        };
        let a710 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h708, _, _, _, _) => {
                h708
            }
        };
        nat_eq(a709, a710)
    } else {
        false
    } {
        let a713 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h711, _, _, _) => {
                h711
            }
        };
        let a714 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h712, _, _, _) => {
                h712
            }
        };
        nat_eq(a713, a714)
    } else {
        false
    } {
        let a717 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h715, _, _) => {
                h715
            }
        };
        let a718 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h716, _, _) => {
                h716
            }
        };
        nat_eq(a717, a718)
    } else {
        false
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a719 = v0;
    let a720 = 0u64;
    if if if nat_eq(a719, a720) {
        true
    } else {
        let a721 = v1;
        let a722 = 0u64;
        nat_eq(a721, a722)
    } {
        true
    } else {
        let a723 = v2;
        let a724 = 0u64;
        nat_eq(a723, a724)
    } {
        true
    } else {
        let a725 = v3;
        let a726 = 0u64;
        nat_eq(a725, a726)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> R<bool> {
    f95({
        let a728 = match v0.clone() {
            Adt2::C0(_, h727, _, _, _, _, _, _, _, _, _) => {
                h727
            }
        };
        let a729 = v1;
        nat_mul(a728, a729)?
    }, match v0.clone() {
        Adt2::C0(_, _, h730, _, _, _, _, _, _, _, _) => {
            h730
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h731, _, _, _, _, _, _, _) => {
            h731
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h732, _, _, _, _, _, _) => {
            h732
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h733, _, _, _, _) => {
            h733
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h734, _, _, _) => {
            h734
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h735, _, _) => {
            h735
        }
    })
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a736 = v1;
    let a737 = 0u64;
    if if nat_eq(a736, a737) {
        true
    } else {
        let a738 = v2;
        let a739 = 0u64;
        nat_eq(a738, a739)
    } {
        Ok(0u64)
    } else {
        let a746 = {
            let a740 = f96(v1, v2, v3)?;
            let a741 = f97(v0, v1, v2)?;
            nat_add(a740, a741)?
        };
        let a747 = {
            let a744 = f98(v0);
            let a745 = {
                let a742 = v1;
                let a743 = v2;
                nat_mul(a742, a743)?
            };
            nat_mul(a744, a745)?
        };
        nat_add(a746, a747)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a748 = v0;
    let a749 = 0u64;
    if if nat_eq(a748, a749) {
        true
    } else {
        let a750 = v1;
        let a751 = 0u64;
        nat_eq(a750, a751)
    } {
        Ok(3u64)
    } else {
        let a754 = {
            let a752 = 3u64;
            let a753 = v1;
            nat_add(a752, a753)?
        };
        let a755 = v2;
        nat_add(a754, a755)
    }
}

pub fn f35(v0: u64) -> bool {
    let a756 = v0;
    let a757 = 2139095040u64;
    if nat_le(a756, a757) {
        true
    } else {
        let a758 = v0;
        let a759 = 2147483648u64;
        nat_eq(a758, a759)
    }
}

pub fn f36(v0: u64, v1: u64) -> R<u64> {
    let a760 = v0;
    let a761 = 0u64;
    if nat_eq(a760, a761) {
        Ok(0u64)
    } else {
        let a762 = v1;
        let a763 = f99(v0)?;
        nat_mul(a762, a763)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a764 = v0;
    let a765 = 0u64;
    if if if if nat_eq(a764, a765) {
        true
    } else {
        let a766 = v1;
        let a767 = 0u64;
        nat_eq(a766, a767)
    } {
        true
    } else {
        let a768 = v2;
        let a769 = 0u64;
        nat_eq(a768, a769)
    } {
        true
    } else {
        let a770 = v3;
        let a771 = 0u64;
        nat_eq(a770, a771)
    } {
        Ok(0u64)
    } else {
        let a784 = {
            let a774 = 2u64;
            let a775 = {
                let a772 = v1;
                let a773 = v2;
                nat_mul(a772, a773)?
            };
            nat_mul(a774, a775)?
        };
        let a785 = {
            let a782 = {
                let a778 = {
                    let a776 = v1;
                    let a777 = v2;
                    nat_mul(a776, a777)?
                };
                let a779 = v3;
                nat_mul(a778, a779)?
            };
            let a783 = {
                let a780 = 1u64;
                let a781 = v0;
                nat_add(a780, a781)?
            };
            nat_mul(a782, a783)?
        };
        nat_add(a784, a785)
    }
}

pub fn f38(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h786, _, _, _, _, _, _, _, _, _, _, _) => {
            h786
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h787, _, _, _, _, _, _, _, _, _, _) => {
            h787
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h788, _, _, _, _, _, _, _, _, _) => {
            h788
        }
    };
    let v6: u64 = {
        let a790 = match v1.clone() {
            Adt2::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                h789
            }
        };
        let a791 = v4;
        let a792 = 0u64;
        nat_quot(a790, a791, a792)
    };
    let v7: u64 = {
        let a794 = match v1.clone() {
            Adt2::C0(h793, _, _, _, _, _, _, _, _, _, _) => {
                h793
            }
        };
        let a795 = 0u64;
        if nat_eq(a794, a795) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f100(match v1.clone() {
        Adt2::C0(h796, _, _, _, _, _, _, _, _, _, _) => {
            h796
        }
    }) {
        let a799 = match v1.clone() {
            Adt2::C0(h797, _, _, _, _, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = match v2.clone() {
            Adt2::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                h798
            }
        };
        nat_eq(a799, a800)
    } else {
        false
    } {
        let a801 = 0u64;
        let a802 = v3;
        nat_lt(a801, a802)
    } else {
        false
    } {
        let a803 = v3;
        let a804 = 2147483648u64;
        nat_lt(a803, a804)
    } else {
        false
    } {
        let a805 = 0u64;
        let a806 = v4;
        nat_lt(a805, a806)
    } else {
        false
    } {
        let a807 = v4;
        let a808 = 2147483648u64;
        nat_lt(a807, a808)
    } else {
        false
    } {
        let a809 = v5;
        let a810 = 2147483648u64;
        nat_lt(a809, a810)
    } else {
        false
    } {
        let a811 = 0u64;
        let a812 = v6;
        nat_lt(a811, a812)
    } else {
        false
    } {
        let a814 = 0u64;
        let a815 = match v2.clone() {
            Adt2::C0(_, h813, _, _, _, _, _, _, _, _, _) => {
                h813
            }
        };
        nat_lt(a814, a815)
    } else {
        false
    } {
        let a819 = match v1.clone() {
            Adt2::C0(_, h816, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        let a820 = {
            let a817 = v6;
            let a818 = v4;
            nat_mul(a817, a818)?
        };
        nat_eq(a819, a820)
    } else {
        false
    } {
        let a822 = match v2.clone() {
            Adt2::C0(_, _, h821, _, _, _, _, _, _, _, _) => {
                h821
            }
        };
        let a823 = v4;
        nat_eq(a822, a823)
    } else {
        false
    } {
        let a836 = {
            let a829 = {
                let a827 = match v2.clone() {
                    Adt2::C0(_, h824, _, _, _, _, _, _, _, _, _) => {
                        h824
                    }
                };
                let a828 = {
                    let a825 = v5;
                    let a826 = 2u64;
                    nat_mul(a825, a826)?
                };
                nat_add(a827, a828)?
            };
            let a830 = v3;
            nat_add(a829, a830)?
        };
        let a837 = {
            let a834 = {
                let a832 = match v1.clone() {
                    Adt2::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                        h831
                    }
                };
                let a833 = v3;
                nat_mul(a832, a833)?
            };
            let a835 = v6;
            nat_add(a834, a835)?
        };
        nat_eq(a836, a837)
    } else {
        false
    } {
        f101(v1.clone(), v7)?
    } else {
        false
    } {
        f101(v2.clone(), v7)?
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a838 = f14(v2.clone(), 1u64)?;
            let a839 = f102(v8.clone())?;
            nat_mul(a838, a839)?
        }, v7, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h840, _, _, _, _, _, _, _, _, _, _) => {
                h840
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f39(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m841 = v1;
    match m841 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a843 = match v0.clone() {
                Adt2::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
                    h842
                }
            };
            let a844 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a843, a844) {
                let a846 = match v3.clone() {
                    Adt2::C0(h845, _, _, _, _, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a847 = 0u64;
                nat_eq(a846, a847)
            } else {
                false
            } {
                let a849 = match v2.clone() {
                    Adt2::C0(h848, _, _, _, _, _, _, _, _, _, _) => {
                        h848
                    }
                };
                let a850 = 0u64;
                nat_eq(a849, a850)
            } else {
                false
            } {
                let a852 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h851, _, _, _, _, _, _) => {
                        h851
                    }
                };
                let a853 = 1u64;
                nat_eq(a852, a853)
            } else {
                false
            } {
                let a855 = match v3.clone() {
                    Adt2::C0(_, _, _, h854, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a856 = 1u64;
                nat_eq(a855, a856)
            } else {
                false
            } {
                let a858 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h857, _, _, _, _, _, _) => {
                        h857
                    }
                };
                let a859 = 1u64;
                nat_eq(a858, a859)
            } else {
                false
            } {
                let a862 = match v0.clone() {
                    Adt2::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                        h860
                    }
                };
                let a863 = match v3.clone() {
                    Adt2::C0(_, _, h861, _, _, _, _, _, _, _, _) => {
                        h861
                    }
                };
                nat_eq(a862, a863)
            } else {
                false
            } {
                let a868 = match v3.clone() {
                    Adt2::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
                        h864
                    }
                };
                let a869 = {
                    let a866 = match v0.clone() {
                        Adt2::C0(_, h865, _, _, _, _, _, _, _, _, _) => {
                            h865
                        }
                    };
                    let a867 = 1u64;
                    nat_add(a866, a867)?
                };
                nat_le(a868, a869)
            } else {
                false
            } {
                let a872 = match v2.clone() {
                    Adt2::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                        h870
                    }
                };
                let a873 = match v0.clone() {
                    Adt2::C0(_, _, h871, _, _, _, _, _, _, _, _) => {
                        h871
                    }
                };
                nat_eq(a872, a873)
            } else {
                false
            } {
                let a881 = match v2.clone() {
                    Adt2::C0(_, _, h874, _, _, _, _, _, _, _, _) => {
                        h874
                    }
                };
                let a882 = {
                    let a879 = {
                        let a876 = match v0.clone() {
                            Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                                h875
                            }
                        };
                        let a877 = 1u64;
                        nat_add(a876, a877)?
                    };
                    let a880 = match v3.clone() {
                        Adt2::C0(_, h878, _, _, _, _, _, _, _, _, _) => {
                            h878
                        }
                    };
                    nat_sub(a879, a880)
                };
                nat_eq(a881, a882)
            } else {
                false
            } {
                let a885 = match v2.clone() {
                    Adt2::C0(_, _, _, h883, _, _, _, _, _, _, _) => {
                        h883
                    }
                };
                let a886 = match v0.clone() {
                    Adt2::C0(_, _, _, h884, _, _, _, _, _, _, _) => {
                        h884
                    }
                };
                nat_eq(a885, a886)
            } else {
                false
            } {
                let a888 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h887, _, _, _, _, _, _) => {
                        h887
                    }
                };
                let a889 = 1u64;
                nat_eq(a888, a889)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a891 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h890, _, _, _, _, _) => {
                        h890
                    }
                };
                let a892 = 4u64;
                nat_eq(a891, a892)
            } else {
                false
            } {
                let a894 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h893, _, _, _, _, _) => {
                        h893
                    }
                };
                let a895 = 4u64;
                nat_eq(a894, a895)
            } else {
                false
            } {
                let a900 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h896, _, _, _, _) => {
                        h896
                    }
                };
                let a901 = {
                    let a898 = match v0.clone() {
                        Adt2::C0(_, h897, _, _, _, _, _, _, _, _, _) => {
                            h897
                        }
                    };
                    let a899 = 4u64;
                    nat_mul(a898, a899)?
                };
                nat_eq(a900, a901)
            } else {
                false
            } {
                let a906 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h902, _, _, _, _) => {
                        h902
                    }
                };
                let a907 = {
                    let a904 = match v3.clone() {
                        Adt2::C0(_, h903, _, _, _, _, _, _, _, _, _) => {
                            h903
                        }
                    };
                    let a905 = 4u64;
                    nat_mul(a904, a905)?
                };
                nat_eq(a906, a907)
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
                Ok(Adt10::C4({
                    let a909 = f14(v2.clone(), 1u64)?;
                    let a910 = f103(match v3.clone() {
                        Adt2::C0(_, h908, _, _, _, _, _, _, _, _, _) => {
                            h908
                        }
                    })?;
                    nat_mul(a909, a910)?
                }, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C29(f18(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f40(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h911, _, _, _, _, _, _, _, _, _, _) => {
            h911
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h912, _, _, _, _, _, _, _, _) => {
                h912
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h913, _, _, _, _, _, _, _, _, _) => {
                h913
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h914, _, _, _, _, _, _) => {
                h914
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h915, _, _, _, _, _, _, _, _) => {
                h915
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h916, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h917, _, _, _, _, _, _, _) => {
                h917
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h918, _, _, _, _, _) => {
                h918
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a920 = match v1.clone() {
        Adt1::C0(h919, _, _, _, _, _, _, _, _, _, _, _) => {
            h919
        }
    };
    let a921 = 1u64;
    if if if if if if if if if if if nat_le(a920, a921) {
        let a923 = match v2.clone() {
            Adt2::C0(h922, _, _, _, _, _, _, _, _, _, _) => {
                h922
            }
        };
        let a924 = 1u64;
        nat_le(a923, a924)
    } else {
        false
    } {
        let a926 = match v3.clone() {
            Adt2::C0(h925, _, _, _, _, _, _, _, _, _, _) => {
                h925
            }
        };
        let a927 = 0u64;
        nat_eq(a926, a927)
    } else {
        false
    } {
        let a932 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h928, _, _, _, _, _) => {
                h928
            }
        };
        let a933 = {
            let a930 = match v2.clone() {
                Adt2::C0(h929, _, _, _, _, _, _, _, _, _, _) => {
                    h929
                }
            };
            let a931 = 0u64;
            if nat_eq(a930, a931) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a932, a933)
    } else {
        false
    } {
        f22(v3.clone())?
    } else {
        false
    } {
        f20(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f104(v4.clone(), match v2.clone() {
            Adt2::C0(_, h934, _, _, _, _, _, _, _, _, _) => {
                h934
            }
        }, match v3.clone() {
            Adt2::C0(_, h935, _, _, _, _, _, _, _, _, _) => {
                h935
            }
        })?
    } else {
        false
    } {
        if v0 {
            f104(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h936, _, _, _, _, _, _, _, _) => {
                    h936
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                    h937
                }
            })?
        } else {
            let a940 = match v2.clone() {
                Adt2::C0(_, _, h938, _, _, _, _, _, _, _, _) => {
                    h938
                }
            };
            let a941 = match v3.clone() {
                Adt2::C0(_, _, h939, _, _, _, _, _, _, _, _) => {
                    h939
                }
            };
            nat_eq(a940, a941)
        }
    } else {
        false
    } {
        let a944 = match v2.clone() {
            Adt2::C0(_, _, _, h942, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a945 = match v3.clone() {
            Adt2::C0(_, _, _, h943, _, _, _, _, _, _, _) => {
                h943
            }
        };
        nat_eq(a944, a945)
    } else {
        false
    } {
        let a948 = match v2.clone() {
            Adt2::C0(_, _, _, _, h946, _, _, _, _, _, _) => {
                h946
            }
        };
        let a949 = match v3.clone() {
            Adt2::C0(_, _, _, _, h947, _, _, _, _, _, _) => {
                h947
            }
        };
        nat_eq(a948, a949)
    } else {
        false
    } {
        let a954 = {
            let a952 = match v4.clone() {
                Adt6::C0(h950, _, _, _) => {
                    h950
                }
            };
            let a953 = match v5.clone() {
                Adt6::C0(h951, _, _, _) => {
                    h951
                }
            };
            nat_mul(a952, a953)?
        };
        let a955 = 2147483648u64;
        nat_lt(a954, a955)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a957 = match v1 {
                Adt1::C0(h956, _, _, _, _, _, _, _, _, _, _, _) => {
                    h956
                }
            };
            let a958 = 0u64;
            nat_eq(a957, a958)
        }, match v2.clone() {
            Adt2::C0(h959, _, _, _, _, _, _, _, _, _, _) => {
                h959
            }
        });
        Ok(Adt10::C4({
            let a960 = f14(v3.clone(), 1u64)?;
            let a961 = f105(v6.clone())?;
            nat_mul(a960, a961)?
        }, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m962 = v2;
    match m962 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a964 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h963, _, _, _, _, _) => {
                        h963
                    }
                };
                let a965 = 1u64;
                nat_eq(a964, a965)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
                    h966
                }
            }, match v0.clone() {
                Adt1::C0(h967, _, _, _, _, _, _, _, _, _, _, _) => {
                    h967
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h968, _, _, _, _, _, _, _, _, _) => {
                    h968
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h969, _, _, _, _, _, _, _) => {
                    h969
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h970, _, _, _, _, _, _, _, _) => {
                        h970
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h971, _, _, _, _, _, _, _, _, _, _) => {
                        h971
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h972, _, _, _, _, _, _, _, _) => {
                        h972
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h973, _, _, _, _, _, _) => {
                        h973
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h974, _, _, _, _, _, _, _) => {
                        h974
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h975, _, _, _, _, _, _, _, _) => {
                        h975
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h976, _, _, _, _, _, _) => {
                        h976
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h977, _, _, _, _, _, _, _) => {
                        h977
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h978, _, _, _, _, _, _, _, _) => {
                        h978
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h979, _, _, _, _, _, _, _, _, _, _) => {
                    h979
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let v14: u64 = {
                let a981 = v12;
                let a982 = 0u64;
                if nat_eq(a981, a982) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a984 = match v4.clone() {
                    Adt2::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                        h983
                    }
                };
                let a985 = v14;
                nat_mul(a984, a985)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h986, _, _, _) => {
                        h986
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h987, _, _, _, _) => {
                        h987
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h988, _, _) => {
                        h988
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h989, _, _, _) => {
                        h989
                    }
                }
            };
            let a991 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h990, _, _, _, _, _) => {
                    h990
                }
            };
            let a992 = 1u64;
            if if if if if if if if if if nat_le(a991, a992) {
                let a993 = v12;
                let a994 = 0u64;
                if nat_eq(a993, a994) {
                    true
                } else {
                    let a995 = v12;
                    let a996 = 1u64;
                    if nat_eq(a995, a996) {
                        let a997 = v13;
                        let a998 = 1u64;
                        nat_eq(a997, a998)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a999 = v13;
                let a1000 = 0u64;
                if nat_eq(a999, a1000) {
                    true
                } else {
                    let a1001 = v13;
                    let a1002 = 1u64;
                    nat_eq(a1001, a1002)
                }
            } else {
                false
            } {
                let a1004 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1003, _, _, _, _, _) => {
                        h1003
                    }
                };
                let a1005 = v14;
                nat_eq(a1004, a1005)
            } else {
                false
            } {
                let a1013 = match v3.clone() {
                    Adt2::C0(_, h1006, _, _, _, _, _, _, _, _, _) => {
                        h1006
                    }
                };
                let a1014 = {
                    let a1011 = v9;
                    let a1012 = {
                        let a1009 = match v6.clone() {
                            Adt6::C0(h1007, _, _, _) => {
                                h1007
                            }
                        };
                        let a1010 = match v7.clone() {
                            Adt6::C0(h1008, _, _, _) => {
                                h1008
                            }
                        };
                        nat_mul(a1009, a1010)?
                    };
                    nat_mul(a1011, a1012)?
                };
                nat_eq(a1013, a1014)
            } else {
                false
            } {
                let a1017 = match v3.clone() {
                    Adt2::C0(_, _, h1015, _, _, _, _, _, _, _, _) => {
                        h1015
                    }
                };
                let a1018 = f88(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1016, _, _, _, _, _, _, _, _, _) => {
                        h1016
                    }
                })?;
                nat_eq(a1017, a1018)
            } else {
                false
            } {
                let a1020 = 0u64;
                let a1021 = match v3.clone() {
                    Adt2::C0(_, _, h1019, _, _, _, _, _, _, _, _) => {
                        h1019
                    }
                };
                nat_lt(a1020, a1021)
            } else {
                false
            } {
                if v5 {
                    let a1023 = match v1 {
                        Adt2::C0(_, _, _, h1022, _, _, _, _, _, _, _) => {
                            h1022
                        }
                    };
                    let a1024 = v9;
                    if if if nat_eq(a1023, a1024) {
                        let a1026 = match v3.clone() {
                            Adt2::C0(_, _, _, h1025, _, _, _, _, _, _, _) => {
                                h1025
                            }
                        };
                        let a1027 = f88(v7.clone(), v11)?;
                        nat_eq(a1026, a1027)
                    } else {
                        false
                    } {
                        let a1029 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1028, _, _, _, _, _, _) => {
                                h1028
                            }
                        };
                        let a1030 = v10;
                        nat_eq(a1029, a1030)
                    } else {
                        false
                    } {
                        let a1032 = 0u64;
                        let a1033 = match v3.clone() {
                            Adt2::C0(_, _, _, h1031, _, _, _, _, _, _, _) => {
                                h1031
                            }
                        };
                        nat_lt(a1032, a1033)
                    } else {
                        false
                    }
                } else {
                    let a1035 = match v1 {
                        Adt2::C0(_, _, h1034, _, _, _, _, _, _, _, _) => {
                            h1034
                        }
                    };
                    let a1036 = v9;
                    if if if nat_eq(a1035, a1036) {
                        let a1038 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1037, _, _, _, _, _, _) => {
                                h1037
                            }
                        };
                        let a1039 = 1u64;
                        nat_eq(a1038, a1039)
                    } else {
                        false
                    } {
                        let a1041 = match v3.clone() {
                            Adt2::C0(_, _, _, h1040, _, _, _, _, _, _, _) => {
                                h1040
                            }
                        };
                        let a1042 = v10;
                        nat_eq(a1041, a1042)
                    } else {
                        false
                    } {
                        let a1044 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1043, _, _, _, _, _, _) => {
                                h1043
                            }
                        };
                        let a1045 = 1u64;
                        nat_eq(a1044, a1045)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f31(match v4.clone() {
                    Adt2::C0(_, h1046, _, _, _, _, _, _, _, _, _) => {
                        h1046
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1066 = {
                        let a1063 = {
                            let a1061 = {
                                let a1055 = {
                                    let a1049 = {
                                        let a1047 = v10;
                                        let a1048 = 1u64;
                                        nat_sub(a1047, a1048)
                                    };
                                    let a1050 = v17;
                                    nat_mul(a1049, a1050)?
                                };
                                let a1056 = {
                                    let a1053 = {
                                        let a1051 = v9;
                                        let a1052 = 1u64;
                                        nat_sub(a1051, a1052)
                                    };
                                    let a1054 = v16;
                                    nat_mul(a1053, a1054)?
                                };
                                nat_add(a1055, a1056)?
                            };
                            let a1062 = {
                                let a1059 = {
                                    let a1057 = v11;
                                    let a1058 = 1u64;
                                    nat_sub(a1057, a1058)
                                };
                                let a1060 = v15;
                                nat_mul(a1059, a1060)?
                            };
                            nat_add(a1061, a1062)?
                        };
                        let a1064 = v15;
                        nat_add(a1063, a1064)?
                    };
                    let a1067 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1065, _) => {
                            h1065
                        }
                    };
                    nat_le(a1066, a1067)
                }
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
                    let a1068 = v13;
                    let a1069 = 0u64;
                    if nat_eq(a1068, a1069) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1070, _, _, _, _, _, _, _, _, _) => {
                        h1070
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f18(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1071, _, _, _, _, _, _, _, _, _) => {
                            h1071
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1072, _, _, _, _, _, _, _, _) => {
                            h1072
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1073, _, _, _, _, _, _, _) => {
                            h1073
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1074, _, _, _, _, _) => {
                            h1074
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1075, _, _, _, _) => {
                            h1075
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1076, _, _, _) => {
                            h1076
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f42(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1077 = v2;
    match m1077 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1078, _, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            }, match v0.clone() {
                Adt1::C0(h1079, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1079
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1080, _, _, _, _, _, _, _, _) => {
                    h1080
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1081, _, _, _, _, _) => {
                    h1081
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1082, _, _, _, _, _, _, _, _) => {
                    h1082
                }
            }, match v0.clone() {
                Adt1::C0(_, h1083, _, _, _, _, _, _, _, _, _, _) => {
                    h1083
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1084, _, _, _, _, _, _, _) => {
                    h1084
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1085, _, _, _, _) => {
                    h1085
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1086, _, _, _, _, _, _, _) => {
                    h1086
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1087, _, _, _, _, _, _, _, _, _) => {
                    h1087
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1088, _, _, _, _, _, _) => {
                    h1088
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1089, _, _, _) => {
                    h1089
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1090, _, _) => {
                    h1090
                }
            };
            let v9: u64 = f88(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1091, _, _, _, _, _, _, _) => {
                    h1091
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1092, _, _, _, _, _, _, _, _, _, _) => {
                    h1092
                }
            };
            let v11: u64 = {
                let a1100 = v8;
                let a1101 = {
                    let a1098 = match v5.clone() {
                        Adt6::C0(h1093, _, _, _) => {
                            h1093
                        }
                    };
                    let a1099 = {
                        let a1096 = match v6.clone() {
                            Adt6::C0(h1094, _, _, _) => {
                                h1094
                            }
                        };
                        let a1097 = match v7.clone() {
                            Adt6::C0(h1095, _, _, _) => {
                                h1095
                            }
                        };
                        nat_mul(a1096, a1097)?
                    };
                    nat_mul(a1098, a1099)?
                };
                nat_mul(a1100, a1101)?
            };
            let a1103 = match v4.clone() {
                Adt2::C0(h1102, _, _, _, _, _, _, _, _, _, _) => {
                    h1102
                }
            };
            let a1104 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1103, a1104) {
                let a1105 = v10;
                let a1106 = 0u64;
                if nat_eq(a1105, a1106) {
                    true
                } else {
                    let a1107 = v10;
                    let a1108 = 1u64;
                    nat_eq(a1107, a1108)
                }
            } else {
                false
            } {
                let a1109 = 0u64;
                let a1110 = v8;
                nat_lt(a1109, a1110)
            } else {
                false
            } {
                let a1111 = v8;
                let a1112 = 2147483648u64;
                nat_lt(a1111, a1112)
            } else {
                false
            } {
                let a1114 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1113, _, _, _, _, _) => {
                        h1113
                    }
                };
                let a1115 = 4u64;
                nat_eq(a1114, a1115)
            } else {
                false
            } {
                let a1116 = 0u64;
                let a1117 = v9;
                nat_lt(a1116, a1117)
            } else {
                false
            } {
                let a1119 = 0u64;
                let a1120 = match v3.clone() {
                    Adt2::C0(_, _, h1118, _, _, _, _, _, _, _, _) => {
                        h1118
                    }
                };
                nat_lt(a1119, a1120)
            } else {
                false
            } {
                let a1122 = 0u64;
                let a1123 = match v3.clone() {
                    Adt2::C0(_, _, _, h1121, _, _, _, _, _, _, _) => {
                        h1121
                    }
                };
                nat_lt(a1122, a1123)
            } else {
                false
            } {
                let a1125 = match v3.clone() {
                    Adt2::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                };
                let a1126 = v11;
                nat_eq(a1125, a1126)
            } else {
                false
            } {
                let a1129 = match v3.clone() {
                    Adt2::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                        h1127
                    }
                };
                let a1130 = f88(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1128, _, _, _, _, _, _, _, _, _) => {
                        h1128
                    }
                })?;
                nat_eq(a1129, a1130)
            } else {
                false
            } {
                let a1133 = match v3.clone() {
                    Adt2::C0(_, _, _, h1131, _, _, _, _, _, _, _) => {
                        h1131
                    }
                };
                let a1134 = f88(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1132, _, _, _, _, _, _, _, _) => {
                        h1132
                    }
                })?;
                nat_eq(a1133, a1134)
            } else {
                false
            } {
                let a1142 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1135, _, _, _, _, _, _) => {
                        h1135
                    }
                };
                let a1143 = {
                    let a1140 = v9;
                    let a1141 = {
                        let a1137 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1136, _, _, _, _, _, _) => {
                                h1136
                            }
                        };
                        let a1138 = v8;
                        let a1139 = 0u64;
                        nat_quot(a1137, a1138, a1139)
                    };
                    nat_mul(a1140, a1141)?
                };
                nat_eq(a1142, a1143)
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
                    let a1144 = v10;
                    let a1145 = 0u64;
                    if nat_eq(a1144, a1145) {
                        4u64
                    } else {
                        2u64
                    }
                }, f18(v4.clone(), 1u64), f18(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1146, _, _) => {
                        h1146
                    }
                }, {
                    let a1148 = v8;
                    let a1149 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1147, _, _) => {
                            h1147
                        }
                    };
                    nat_mul(a1148, a1149)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f43(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1150 = v1;
    match m1150 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f106(v0, v3, v2)
        }
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1152 = match v0.clone() {
        Adt2::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
            h1151
        }
    };
    let a1153 = 1u64;
    if if nat_eq(a1152, a1153) {
        true
    } else {
        let a1155 = match v0.clone() {
            Adt2::C0(h1154, _, _, _, _, _, _, _, _, _, _) => {
                h1154
            }
        };
        let a1156 = 30u64;
        nat_eq(a1155, a1156)
    } {
        let a1158 = match v1.clone() {
            Adt2::C0(h1157, _, _, _, _, _, _, _, _, _, _) => {
                h1157
            }
        };
        let a1159 = 1u64;
        if if if if if if if if if if if nat_eq(a1158, a1159) {
            let a1162 = match v0.clone() {
                Adt2::C0(_, h1160, _, _, _, _, _, _, _, _, _) => {
                    h1160
                }
            };
            let a1163 = match v1.clone() {
                Adt2::C0(_, h1161, _, _, _, _, _, _, _, _, _) => {
                    h1161
                }
            };
            nat_eq(a1162, a1163)
        } else {
            false
        } {
            let a1165 = 0u64;
            let a1166 = match v1.clone() {
                Adt2::C0(_, _, h1164, _, _, _, _, _, _, _, _) => {
                    h1164
                }
            };
            nat_lt(a1165, a1166)
        } else {
            false
        } {
            let a1169 = match v1.clone() {
                Adt2::C0(_, _, h1167, _, _, _, _, _, _, _, _) => {
                    h1167
                }
            };
            let a1170 = match v1.clone() {
                Adt2::C0(_, _, _, h1168, _, _, _, _, _, _, _) => {
                    h1168
                }
            };
            nat_eq(a1169, a1170)
        } else {
            false
        } {
            let a1172 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1171, _, _, _, _, _, _) => {
                    h1171
                }
            };
            let a1173 = 1u64;
            nat_eq(a1172, a1173)
        } else {
            false
        } {
            let a1181 = match v0.clone() {
                Adt2::C0(_, _, h1174, _, _, _, _, _, _, _, _) => {
                    h1174
                }
            };
            let a1182 = {
                let a1179 = {
                    let a1177 = match v1.clone() {
                        Adt2::C0(_, _, h1175, _, _, _, _, _, _, _, _) => {
                            h1175
                        }
                    };
                    let a1178 = match v1.clone() {
                        Adt2::C0(_, _, h1176, _, _, _, _, _, _, _, _) => {
                            h1176
                        }
                    };
                    nat_add(a1177, a1178)?
                };
                let a1180 = 1u64;
                nat_sub(a1179, a1180)
            };
            nat_eq(a1181, a1182)
        } else {
            false
        } {
            let a1184 = 0u64;
            let a1185 = match v0.clone() {
                Adt2::C0(_, _, _, h1183, _, _, _, _, _, _, _) => {
                    h1183
                }
            };
            nat_lt(a1184, a1185)
        } else {
            false
        } {
            let a1187 = 0u64;
            let a1188 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1186, _, _, _, _, _, _) => {
                    h1186
                }
            };
            nat_lt(a1187, a1188)
        } else {
            false
        } {
            let a1190 = 2u64;
            let a1191 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1189, _, _, _, _, _) => {
                    h1189
                }
            };
            nat_le(a1190, a1191)
        } else {
            false
        } {
            let a1193 = 2u64;
            let a1194 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1192, _, _, _, _, _) => {
                    h1192
                }
            };
            nat_le(a1193, a1194)
        } else {
            false
        } {
            f20(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f14(v1.clone(), 1u64)?, 2u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f45(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1195 = v1;
    match m1195 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f107(v0, v3, v2)
        }
    }
}

pub fn f46(v0: u64) -> bool {
    let a1196 = v0;
    let a1197 = 1u64;
    if if nat_eq(a1196, a1197) {
        true
    } else {
        let a1198 = v0;
        let a1199 = 33u64;
        nat_eq(a1198, a1199)
    } {
        true
    } else {
        let a1200 = v0;
        let a1201 = 34u64;
        nat_eq(a1200, a1201)
    }
}

pub fn f47(v0: u64) -> bool {
    if f108(v0) {
        !f109(v0)
    } else {
        false
    }
}

pub fn f48(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1202 = v1;
    match m1202 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1204 = f110(match v0.clone() {
                Adt2::C0(h1203, _, _, _, _, _, _, _, _, _, _) => {
                    h1203
                }
            });
            match m1204 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1205, _) => {
                            h1205
                        }
                    };
                    if if if if if if f30(v3, v2.clone()) {
                        f20(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1206 = f14(v0.clone(), 1u64)?;
                        let a1207 = f14(v2.clone(), 1u64)?;
                        nat_eq(a1206, a1207)
                    } else {
                        false
                    } {
                        let a1212 = {
                            let a1209 = match v0.clone() {
                                Adt2::C0(_, h1208, _, _, _, _, _, _, _, _, _) => {
                                    h1208
                                }
                            };
                            let a1210 = v5;
                            let a1211 = 1u64;
                            nat_rem(a1209, a1210, a1211)
                        };
                        let a1213 = 0u64;
                        if nat_eq(a1212, a1213) {
                            let a1216 = match v4 {
                                Adt18::C0(_, _, h1214) => {
                                    h1214
                                }
                            };
                            let a1217 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1215, _, _, _, _, _) => {
                                    h1215
                                }
                            };
                            nat_le(a1216, a1217)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1218 = v5;
                        let a1219 = 1u64;
                        if nat_eq(a1218, a1219) {
                            let a1221 = 4u64;
                            let a1222 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1220, _, _, _, _, _) => {
                                    h1220
                                }
                            };
                            nat_le(a1221, a1222)
                        } else {
                            let a1224 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1223, _, _, _, _, _) => {
                                    h1223
                                }
                            };
                            let a1225 = 4u64;
                            nat_eq(a1224, a1225)
                        }
                    } else {
                        false
                    } {
                        let a1230 = {
                            let a1227 = match v2.clone() {
                                Adt2::C0(_, h1226, _, _, _, _, _, _, _, _, _) => {
                                    h1226
                                }
                            };
                            let a1228 = v5;
                            let a1229 = 1u64;
                            nat_rem(a1227, a1228, a1229)
                        };
                        let a1231 = 0u64;
                        if nat_eq(a1230, a1231) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f14(v0.clone(), v5)?, 4u64, f18(v0.clone(), v5), f18(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1232, _, _, _, _, _, _, _, _, _, _) => {
                                h1232
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

pub fn f49(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1235 = match v1.clone() {
        Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
            h1233
        }
    };
    let a1236 = match v3.clone() {
        Adt2::C0(h1234, _, _, _, _, _, _, _, _, _, _) => {
            h1234
        }
    };
    if nat_eq(a1235, a1236) {
        f111(v0, v1, v2, v3)
    } else {
        if if if if f109(match v1.clone() {
            Adt2::C0(h1237, _, _, _, _, _, _, _, _, _, _) => {
                h1237
            }
        }) {
            f109(match v3.clone() {
                Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                    h1238
                }
            })
        } else {
            false
        } {
            let a1239 = v0;
            let a1240 = 33u64;
            if nat_eq(a1239, a1240) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1241 = f14(v1.clone(), 1u64)?;
            let a1242 = f14(v3.clone(), 1u64)?;
            nat_eq(a1241, a1242)
        } else {
            false
        } {
            let m1244 = f110(match v3.clone() {
                Adt2::C0(h1243, _, _, _, _, _, _, _, _, _, _) => {
                    h1243
                }
            });
            match m1244 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                            h1245
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1246, _, _, _, _, _, _, _, _, _, _) => {
                            h1246
                        }
                    };
                    let v7: bool = {
                        let a1247 = v5;
                        let a1248 = 43u64;
                        if nat_eq(a1247, a1248) {
                            true
                        } else {
                            let a1249 = v6;
                            let a1250 = 43u64;
                            nat_eq(a1249, a1250)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1251 = f14(v3.clone(), 1u64)?;
                        let a1252 = if v7 {
                            f112(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1251, a1252)?
                    }, match v4 {
                        Adt18::C0(_, _, h1253) => {
                            h1253
                        }
                    }, f18(v1, 1u64), f18(v3, 1u64), if v7 {
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

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1255 = match v0.clone() {
        Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
            h1254
        }
    };
    let a1256 = 0u64;
    if if if if nat_eq(a1255, a1256) {
        true
    } else {
        let a1258 = match v0.clone() {
            Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
                h1257
            }
        };
        let a1259 = 1u64;
        nat_eq(a1258, a1259)
    } {
        true
    } else {
        let a1261 = match v0.clone() {
            Adt2::C0(h1260, _, _, _, _, _, _, _, _, _, _) => {
                h1260
            }
        };
        let a1262 = 25u64;
        if nat_eq(a1261, a1262) {
            true
        } else {
            let a1264 = match v0.clone() {
                Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
                    h1263
                }
            };
            let a1265 = 26u64;
            nat_eq(a1264, a1265)
        }
    } {
        true
    } else {
        let a1267 = match v0.clone() {
            Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                h1266
            }
        };
        let a1268 = 30u64;
        nat_eq(a1267, a1268)
    } {
        let a1271 = match v0.clone() {
            Adt2::C0(h1269, _, _, _, _, _, _, _, _, _, _) => {
                h1269
            }
        };
        let a1272 = match v1.clone() {
            Adt2::C0(h1270, _, _, _, _, _, _, _, _, _, _) => {
                h1270
            }
        };
        if if nat_eq(a1271, a1272) {
            f113(match v1.clone() {
                Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                    h1273
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1274, _, _, _, _, _, _, _, _) => {
                    h1274
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1275, _, _, _, _, _, _, _) => {
                    h1275
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1276, _, _, _, _, _, _) => {
                    h1276
                }
            }, match v0.clone() {
                Adt2::C0(_, h1277, _, _, _, _, _, _, _, _, _) => {
                    h1277
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1278, _, _, _, _, _, _, _, _) => {
                    h1278
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1279, _, _, _, _, _, _, _) => {
                    h1279
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1280, _, _, _, _, _, _) => {
                    h1280
                }
            })
        } else {
            false
        } {
            let m1282 = f110(match v0.clone() {
                Adt2::C0(h1281, _, _, _, _, _, _, _, _, _, _) => {
                    h1281
                }
            });
            match m1282 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f14(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1283) => {
                            h1283
                        }
                    }, f18(v0, 1u64), f18(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f51(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1284 = v2;
    match m1284 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1285 = v0;
            let a1286 = 4u64;
            if if if nat_lt(a1285, a1286) {
                let a1289 = match v1.clone() {
                    Adt2::C0(h1287, _, _, _, _, _, _, _, _, _, _) => {
                        h1287
                    }
                };
                let a1290 = match v4.clone() {
                    Adt2::C0(h1288, _, _, _, _, _, _, _, _, _, _) => {
                        h1288
                    }
                };
                if nat_eq(a1289, a1290) {
                    let a1293 = match v1.clone() {
                        Adt2::C0(h1291, _, _, _, _, _, _, _, _, _, _) => {
                            h1291
                        }
                    };
                    let a1294 = match v3.clone() {
                        Adt2::C0(h1292, _, _, _, _, _, _, _, _, _, _) => {
                            h1292
                        }
                    };
                    nat_eq(a1293, a1294)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f114(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1295, _, _, _, _, _, _, _, _, _) => {
                        h1295
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1296, _, _, _, _, _, _, _, _, _) => {
                        h1296
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                        h1297
                    }
                })? {
                    f114(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1298, _, _, _, _, _, _, _, _) => {
                            h1298
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1299, _, _, _, _, _, _, _, _) => {
                            h1299
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1300, _, _, _, _, _, _, _, _) => {
                            h1300
                        }
                    })?
                } else {
                    false
                } {
                    f114(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1301, _, _, _, _, _, _, _) => {
                            h1301
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1302, _, _, _, _, _, _, _) => {
                            h1302
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1303, _, _, _, _, _, _, _) => {
                            h1303
                        }
                    })?
                } else {
                    false
                } {
                    f114(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1304, _, _, _, _, _, _) => {
                            h1304
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1305, _, _, _, _, _, _) => {
                            h1305
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1306, _, _, _, _, _, _) => {
                            h1306
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1308 = f110(match v1.clone() {
                    Adt2::C0(h1307, _, _, _, _, _, _, _, _, _, _) => {
                        h1307
                    }
                });
                match m1308 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1309, _) => {
                                h1309
                            }
                        };
                        let a1314 = {
                            let a1311 = match v1.clone() {
                                Adt2::C0(_, h1310, _, _, _, _, _, _, _, _, _) => {
                                    h1310
                                }
                            };
                            let a1312 = v6;
                            let a1313 = 1u64;
                            nat_rem(a1311, a1312, a1313)
                        };
                        let a1315 = 0u64;
                        if if if nat_eq(a1314, a1315) {
                            let a1320 = {
                                let a1317 = match v4.clone() {
                                    Adt2::C0(_, h1316, _, _, _, _, _, _, _, _, _) => {
                                        h1316
                                    }
                                };
                                let a1318 = v6;
                                let a1319 = 1u64;
                                nat_rem(a1317, a1318, a1319)
                            };
                            let a1321 = 0u64;
                            nat_eq(a1320, a1321)
                        } else {
                            false
                        } {
                            let a1326 = {
                                let a1323 = match v3.clone() {
                                    Adt2::C0(_, h1322, _, _, _, _, _, _, _, _, _) => {
                                        h1322
                                    }
                                };
                                let a1324 = v6;
                                let a1325 = 1u64;
                                nat_rem(a1323, a1324, a1325)
                            };
                            let a1327 = 0u64;
                            nat_eq(a1326, a1327)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f14(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1328) => {
                                    h1328
                                }
                            }, f18(v1, v6), f18(v3, v6), Adt4::C2(v0, f18(v4, v6))))
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

pub fn f52(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1330 = match v2.clone() {
        Adt2::C0(h1329, _, _, _, _, _, _, _, _, _, _) => {
            h1329
        }
    };
    let a1331 = 0u64;
    if if nat_eq(a1330, a1331) {
        let a1333 = match v3.clone() {
            Adt2::C0(h1332, _, _, _, _, _, _, _, _, _, _) => {
                h1332
            }
        };
        let a1334 = 0u64;
        nat_eq(a1333, a1334)
    } else {
        false
    } {
        let a1335 = v0;
        let a1336 = 61u64;
        if nat_eq(a1335, a1336) {
            let v4: bool = {
                let a1338 = 0u64;
                let a1339 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1337, _, _, _) => {
                        h1337
                    }
                };
                nat_lt(a1338, a1339)
            };
            if if if if if if f115(match v2.clone() {
                Adt2::C0(_, h1340, _, _, _, _, _, _, _, _, _) => {
                    h1340
                }
            }, match v1.clone() {
                Adt1::C0(h1341, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1341
                }
            }, match v1.clone() {
                Adt1::C0(_, h1342, _, _, _, _, _, _, _, _, _, _) => {
                    h1342
                }
            }, match v3.clone() {
                Adt2::C0(_, h1343, _, _, _, _, _, _, _, _, _) => {
                    h1343
                }
            })? {
                f115(match v2.clone() {
                    Adt2::C0(_, _, h1344, _, _, _, _, _, _, _, _) => {
                        h1344
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1345, _, _, _, _, _, _, _, _, _) => {
                        h1345
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1346, _, _, _, _, _, _, _, _) => {
                        h1346
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1347, _, _, _, _, _, _, _, _) => {
                        h1347
                    }
                })?
            } else {
                false
            } {
                f115(match v2.clone() {
                    Adt2::C0(_, _, _, h1348, _, _, _, _, _, _, _) => {
                        h1348
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1349, _, _, _, _, _, _, _) => {
                        h1349
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1350, _, _, _, _, _, _) => {
                        h1350
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1351, _, _, _, _, _, _, _) => {
                        h1351
                    }
                })?
            } else {
                false
            } {
                f115(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1352, _, _, _, _, _, _) => {
                        h1352
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1353, _, _, _, _, _) => {
                        h1353
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1354, _, _, _, _) => {
                        h1354
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1355, _, _, _, _, _, _) => {
                        h1355
                    }
                })?
            } else {
                false
            } {
                let a1357 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1356, _, _, _) => {
                        h1356
                    }
                };
                let a1358 = 4294967295u64;
                nat_le(a1357, a1358)
            } else {
                false
            } {
                if v4 {
                    !f31(match v2.clone() {
                        Adt2::C0(_, h1359, _, _, _, _, _, _, _, _, _) => {
                            h1359
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1360, _, _, _, _, _, _, _, _) => {
                            h1360
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1361, _, _, _, _, _, _, _) => {
                            h1361
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1362, _, _, _, _, _, _) => {
                            h1362
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1363, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1363
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1364, _, _, _, _, _, _, _, _, _) => {
                        h1364
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1365, _, _, _, _, _, _, _) => {
                        h1365
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1366, _, _, _, _, _) => {
                        h1366
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1367 = v0;
            let a1368 = 62u64;
            if nat_eq(a1367, a1368) {
                let a1370 = match v1.clone() {
                    Adt1::C0(h1369, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1369
                    }
                };
                let a1371 = 2147483648u64;
                if if if if if nat_lt(a1370, a1371) {
                    let a1373 = match v1.clone() {
                        Adt1::C0(_, h1372, _, _, _, _, _, _, _, _, _, _) => {
                            h1372
                        }
                    };
                    let a1374 = 2147483648u64;
                    nat_lt(a1373, a1374)
                } else {
                    false
                } {
                    let a1377 = match v1.clone() {
                        Adt1::C0(h1375, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1375
                        }
                    };
                    let a1378 = match v2.clone() {
                        Adt2::C0(_, h1376, _, _, _, _, _, _, _, _, _) => {
                            h1376
                        }
                    };
                    nat_lt(a1377, a1378)
                } else {
                    false
                } {
                    let a1381 = match v1.clone() {
                        Adt1::C0(_, h1379, _, _, _, _, _, _, _, _, _, _) => {
                            h1379
                        }
                    };
                    let a1382 = match v2.clone() {
                        Adt2::C0(_, h1380, _, _, _, _, _, _, _, _, _) => {
                            h1380
                        }
                    };
                    nat_lt(a1381, a1382)
                } else {
                    false
                } {
                    let a1385 = match v2.clone() {
                        Adt2::C0(_, _, h1383, _, _, _, _, _, _, _, _) => {
                            h1383
                        }
                    };
                    let a1386 = match v3.clone() {
                        Adt2::C0(_, _, h1384, _, _, _, _, _, _, _, _) => {
                            h1384
                        }
                    };
                    if if if nat_eq(a1385, a1386) {
                        let a1389 = match v2.clone() {
                            Adt2::C0(_, _, _, h1387, _, _, _, _, _, _, _) => {
                                h1387
                            }
                        };
                        let a1390 = match v3.clone() {
                            Adt2::C0(_, _, _, h1388, _, _, _, _, _, _, _) => {
                                h1388
                            }
                        };
                        nat_eq(a1389, a1390)
                    } else {
                        false
                    } {
                        let a1393 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1391, _, _, _, _, _, _) => {
                                h1391
                            }
                        };
                        let a1394 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1392, _, _, _, _, _, _) => {
                                h1392
                            }
                        };
                        nat_eq(a1393, a1394)
                    } else {
                        false
                    } {
                        let a1403 = match v3.clone() {
                            Adt2::C0(_, h1395, _, _, _, _, _, _, _, _, _) => {
                                h1395
                            }
                        };
                        let a1404 = {
                            let a1401 = {
                                let a1398 = match v2.clone() {
                                    Adt2::C0(_, h1396, _, _, _, _, _, _, _, _, _) => {
                                        h1396
                                    }
                                };
                                let a1399 = match v1.clone() {
                                    Adt1::C0(h1397, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1397
                                    }
                                };
                                nat_add(a1398, a1399)?
                            };
                            let a1402 = match v1.clone() {
                                Adt1::C0(_, h1400, _, _, _, _, _, _, _, _, _, _) => {
                                    h1400
                                }
                            };
                            nat_add(a1401, a1402)?
                        };
                        nat_eq(a1403, a1404)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1405, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1405
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1406 = v0;
                let a1407 = 63u64;
                if nat_eq(a1406, a1407) {
                    if if if if if f116(match v2.clone() {
                        Adt2::C0(_, h1408, _, _, _, _, _, _, _, _, _) => {
                            h1408
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1409, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1409
                        }
                    })? {
                        f116(match v2.clone() {
                            Adt2::C0(_, _, h1410, _, _, _, _, _, _, _, _) => {
                                h1410
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1411, _, _, _, _, _, _, _, _, _, _) => {
                                h1411
                            }
                        })?
                    } else {
                        false
                    } {
                        f116(match v2.clone() {
                            Adt2::C0(_, _, _, h1412, _, _, _, _, _, _, _) => {
                                h1412
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1413, _, _, _, _, _, _, _, _, _) => {
                                h1413
                            }
                        })?
                    } else {
                        false
                    } {
                        f116(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1414, _, _, _, _, _, _) => {
                                h1414
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1415, _, _, _, _, _, _, _, _) => {
                                h1415
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1418 = match v2.clone() {
                            Adt2::C0(_, h1416, _, _, _, _, _, _, _, _, _) => {
                                h1416
                            }
                        };
                        let a1419 = match v3.clone() {
                            Adt2::C0(_, h1417, _, _, _, _, _, _, _, _, _) => {
                                h1417
                            }
                        };
                        if if if nat_eq(a1418, a1419) {
                            let a1422 = match v2.clone() {
                                Adt2::C0(_, _, h1420, _, _, _, _, _, _, _, _) => {
                                    h1420
                                }
                            };
                            let a1423 = match v3.clone() {
                                Adt2::C0(_, _, h1421, _, _, _, _, _, _, _, _) => {
                                    h1421
                                }
                            };
                            nat_eq(a1422, a1423)
                        } else {
                            false
                        } {
                            let a1426 = match v2.clone() {
                                Adt2::C0(_, _, _, h1424, _, _, _, _, _, _, _) => {
                                    h1424
                                }
                            };
                            let a1427 = match v3.clone() {
                                Adt2::C0(_, _, _, h1425, _, _, _, _, _, _, _) => {
                                    h1425
                                }
                            };
                            nat_eq(a1426, a1427)
                        } else {
                            false
                        } {
                            let a1430 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1428, _, _, _, _, _, _) => {
                                    h1428
                                }
                            };
                            let a1431 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1429, _, _, _, _, _, _) => {
                                    h1429
                                }
                            };
                            nat_eq(a1430, a1431)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1432, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1432
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1433, _, _, _, _, _, _, _, _, _, _) => {
                                h1433
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1434, _, _, _, _, _, _, _, _, _) => {
                                h1434
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1435, _, _, _, _, _, _, _, _) => {
                                h1435
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

pub fn f53(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1436 = v3;
    match m1436 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1437 = v0;
                let a1438 = 4u64;
                nat_eq(a1437, a1438)
            };
            let v7: bool = {
                let a1440 = 0u64;
                let a1441 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1439, _, _, _, _, _, _, _) => {
                        h1439
                    }
                };
                nat_lt(a1440, a1441)
            };
            let v8: u64 = f14(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1442, _, _, _, _, _, _, _, _, _) => {
                    h1442
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1443, _, _, _, _, _, _, _, _) => {
                    h1443
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1444, _, _, _, _, _, _, _) => {
                    h1444
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1445, _, _, _, _, _, _) => {
                    h1445
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1446, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1446
                }
            }, match v1.clone() {
                Adt1::C0(_, h1447, _, _, _, _, _, _, _, _, _, _) => {
                    h1447
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1448, _, _, _, _, _, _, _, _, _) => {
                    h1448
                }
            });
            if if if if if if if if if if if if v6 {
                let a1450 = match v2.clone() {
                    Adt2::C0(h1449, _, _, _, _, _, _, _, _, _, _) => {
                        h1449
                    }
                };
                let a1451 = 0u64;
                if nat_eq(a1450, a1451) {
                    let a1453 = match v5.clone() {
                        Adt2::C0(h1452, _, _, _, _, _, _, _, _, _, _) => {
                            h1452
                        }
                    };
                    let a1454 = 0u64;
                    nat_eq(a1453, a1454)
                } else {
                    false
                }
            } else {
                let a1456 = match v2.clone() {
                    Adt2::C0(h1455, _, _, _, _, _, _, _, _, _, _) => {
                        h1455
                    }
                };
                let a1457 = 0u64;
                if if nat_eq(a1456, a1457) {
                    true
                } else {
                    let a1459 = match v2.clone() {
                        Adt2::C0(h1458, _, _, _, _, _, _, _, _, _, _) => {
                            h1458
                        }
                    };
                    let a1460 = 26u64;
                    nat_eq(a1459, a1460)
                } {
                    let a1462 = match v5.clone() {
                        Adt2::C0(h1461, _, _, _, _, _, _, _, _, _, _) => {
                            h1461
                        }
                    };
                    let a1463 = 0u64;
                    if nat_eq(a1462, a1463) {
                        true
                    } else {
                        let a1465 = match v5.clone() {
                            Adt2::C0(h1464, _, _, _, _, _, _, _, _, _, _) => {
                                h1464
                            }
                        };
                        let a1466 = 26u64;
                        nat_eq(a1465, a1466)
                    }
                } else {
                    false
                }
            } {
                let a1469 = match v2.clone() {
                    Adt2::C0(h1467, _, _, _, _, _, _, _, _, _, _) => {
                        h1467
                    }
                };
                let a1470 = match v4.clone() {
                    Adt2::C0(h1468, _, _, _, _, _, _, _, _, _, _) => {
                        h1468
                    }
                };
                nat_eq(a1469, a1470)
            } else {
                false
            } {
                f21(v2.clone(), v4.clone())
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
                let a1472 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1471, _, _, _, _, _) => {
                        h1471
                    }
                };
                let a1473 = 4u64;
                nat_eq(a1472, a1473)
            } else {
                false
            } {
                let a1474 = v8;
                let a1475 = f14(v2.clone(), 1u64)?;
                nat_le(a1474, a1475)
            } else {
                false
            } {
                let a1477 = match v1.clone() {
                    Adt1::C0(_, _, _, h1476, _, _, _, _, _, _, _, _) => {
                        h1476
                    }
                };
                let a1478 = 1073741824u64;
                nat_lt(a1477, a1478)
            } else {
                false
            } {
                let a1480 = match v1.clone() {
                    Adt1::C0(h1479, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1479
                    }
                };
                let a1481 = 4294967295u64;
                if if if if nat_le(a1480, a1481) {
                    let a1483 = match v1.clone() {
                        Adt1::C0(_, h1482, _, _, _, _, _, _, _, _, _, _) => {
                            h1482
                        }
                    };
                    let a1484 = 4294967295u64;
                    nat_le(a1483, a1484)
                } else {
                    false
                } {
                    let a1486 = match v1.clone() {
                        Adt1::C0(_, _, h1485, _, _, _, _, _, _, _, _, _) => {
                            h1485
                        }
                    };
                    let a1487 = 4294967295u64;
                    nat_le(a1486, a1487)
                } else {
                    false
                } {
                    let a1489 = match v1.clone() {
                        Adt1::C0(_, _, _, h1488, _, _, _, _, _, _, _, _) => {
                            h1488
                        }
                    };
                    let a1490 = 4294967295u64;
                    nat_le(a1489, a1490)
                } else {
                    false
                } {
                    let a1492 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1491, _, _, _, _, _, _, _) => {
                            h1491
                        }
                    };
                    let a1493 = 4294967295u64;
                    nat_le(a1492, a1493)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1496 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1494) => {
                            h1494
                        }
                    };
                    let a1497 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1495) => {
                            h1495
                        }
                    };
                    nat_eq(a1496, a1497)
                } else {
                    true
                }
            } else {
                false
            } {
                f117(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1498, _, _, _, _, _, _, _, _) => {
                        h1498
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1499, _) => {
                        h1499
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
                Ok(Adt10::C4({
                    let a1500 = v10;
                    let a1501 = v8;
                    nat_add(a1500, a1501)?
                }, 4u64, f18(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1502, _, _, _, _, _, _, _, _) => {
                        h1502
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f54(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1504 = match v0.clone() {
        Adt2::C0(h1503, _, _, _, _, _, _, _, _, _, _) => {
            h1503
        }
    };
    let a1505 = 0u64;
    if if if nat_eq(a1504, a1505) {
        let a1507 = match v1.clone() {
            Adt2::C0(h1506, _, _, _, _, _, _, _, _, _, _) => {
                h1506
            }
        };
        let a1508 = 0u64;
        nat_eq(a1507, a1508)
    } else {
        false
    } {
        f113(match v0.clone() {
            Adt2::C0(_, h1509, _, _, _, _, _, _, _, _, _) => {
                h1509
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1510, _, _, _, _, _, _, _, _) => {
                h1510
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1511, _, _, _, _, _, _, _) => {
                h1511
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1512, _, _, _, _, _, _) => {
                h1512
            }
        }, match v1.clone() {
            Adt2::C0(_, h1513, _, _, _, _, _, _, _, _, _) => {
                h1513
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1514, _, _, _, _, _, _, _, _) => {
                h1514
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1515, _, _, _, _, _, _, _) => {
                h1515
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1516, _, _, _, _, _, _) => {
                h1516
            }
        })
    } else {
        false
    } {
        let v2: u64 = f14(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1517 = v2;
            let a1518 = f14(v0.clone(), 1u64)?;
            nat_add(a1517, a1518)?
        }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f55(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1520 = match v1.clone() {
        Adt2::C0(h1519, _, _, _, _, _, _, _, _, _, _) => {
            h1519
        }
    };
    let a1521 = 0u64;
    if if if nat_eq(a1520, a1521) {
        true
    } else {
        let a1523 = match v1.clone() {
            Adt2::C0(h1522, _, _, _, _, _, _, _, _, _, _) => {
                h1522
            }
        };
        let a1524 = 1u64;
        nat_eq(a1523, a1524)
    } {
        true
    } else {
        let a1526 = match v1.clone() {
            Adt2::C0(h1525, _, _, _, _, _, _, _, _, _, _) => {
                h1525
            }
        };
        let a1527 = 30u64;
        nat_eq(a1526, a1527)
    } {
        let a1530 = match v1.clone() {
            Adt2::C0(h1528, _, _, _, _, _, _, _, _, _, _) => {
                h1528
            }
        };
        let a1531 = match v2.clone() {
            Adt2::C0(h1529, _, _, _, _, _, _, _, _, _, _) => {
                h1529
            }
        };
        if if if if if nat_eq(a1530, a1531) {
            let a1534 = match v1.clone() {
                Adt2::C0(_, h1532, _, _, _, _, _, _, _, _, _) => {
                    h1532
                }
            };
            let a1535 = match v2.clone() {
                Adt2::C0(_, h1533, _, _, _, _, _, _, _, _, _) => {
                    h1533
                }
            };
            nat_eq(a1534, a1535)
        } else {
            false
        } {
            let a1538 = match v1.clone() {
                Adt2::C0(_, _, _, h1536, _, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1539 = match v2.clone() {
                Adt2::C0(_, _, _, h1537, _, _, _, _, _, _, _) => {
                    h1537
                }
            };
            nat_eq(a1538, a1539)
        } else {
            false
        } {
            let a1542 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1540, _, _, _, _, _, _) => {
                    h1540
                }
            };
            let a1543 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1541, _, _, _, _, _, _) => {
                    h1541
                }
            };
            nat_eq(a1542, a1543)
        } else {
            false
        } {
            let a1544 = v0;
            let a1545 = 4u64;
            if nat_eq(a1544, a1545) {
                let a1547 = match v1.clone() {
                    Adt2::C0(_, _, h1546, _, _, _, _, _, _, _, _) => {
                        h1546
                    }
                };
                let a1548 = 1u64;
                if nat_eq(a1547, a1548) {
                    let a1551 = match v1.clone() {
                        Adt2::C0(_, h1549, _, _, _, _, _, _, _, _, _) => {
                            h1549
                        }
                    };
                    let a1552 = match v2.clone() {
                        Adt2::C0(_, _, h1550, _, _, _, _, _, _, _, _) => {
                            h1550
                        }
                    };
                    nat_eq(a1551, a1552)
                } else {
                    false
                }
            } else {
                let a1553 = v0;
                let a1554 = 4u64;
                if if nat_lt(a1553, a1554) {
                    let a1557 = match v1.clone() {
                        Adt2::C0(_, h1555, _, _, _, _, _, _, _, _, _) => {
                            h1555
                        }
                    };
                    let a1558 = match v1.clone() {
                        Adt2::C0(_, _, h1556, _, _, _, _, _, _, _, _) => {
                            h1556
                        }
                    };
                    nat_eq(a1557, a1558)
                } else {
                    false
                } {
                    let a1561 = match v1.clone() {
                        Adt2::C0(_, _, h1559, _, _, _, _, _, _, _, _) => {
                            h1559
                        }
                    };
                    let a1562 = match v2.clone() {
                        Adt2::C0(_, _, h1560, _, _, _, _, _, _, _, _) => {
                            h1560
                        }
                    };
                    nat_eq(a1561, a1562)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1564 = f110(match v1.clone() {
                Adt2::C0(h1563, _, _, _, _, _, _, _, _, _, _) => {
                    h1563
                }
            });
            match m1564 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1565) => {
                            h1565
                        }
                    }, f18(v1, 1u64), f18(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f56(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f118(match v1.clone() {
        Adt2::C0(h1566, _, _, _, _, _, _, _, _, _, _) => {
            h1566
        }
    }, v0) {
        let a1569 = match v1.clone() {
            Adt2::C0(h1567, _, _, _, _, _, _, _, _, _, _) => {
                h1567
            }
        };
        let a1570 = match v2.clone() {
            Adt2::C0(h1568, _, _, _, _, _, _, _, _, _, _) => {
                h1568
            }
        };
        nat_eq(a1569, a1570)
    } else {
        false
    } {
        let a1573 = match v1.clone() {
            Adt2::C0(_, h1571, _, _, _, _, _, _, _, _, _) => {
                h1571
            }
        };
        let a1574 = match v2.clone() {
            Adt2::C0(_, h1572, _, _, _, _, _, _, _, _, _) => {
                h1572
            }
        };
        if if if nat_eq(a1573, a1574) {
            let a1577 = match v1.clone() {
                Adt2::C0(_, _, h1575, _, _, _, _, _, _, _, _) => {
                    h1575
                }
            };
            let a1578 = match v2.clone() {
                Adt2::C0(_, _, h1576, _, _, _, _, _, _, _, _) => {
                    h1576
                }
            };
            nat_eq(a1577, a1578)
        } else {
            false
        } {
            let a1581 = match v1.clone() {
                Adt2::C0(_, _, _, h1579, _, _, _, _, _, _, _) => {
                    h1579
                }
            };
            let a1582 = match v2.clone() {
                Adt2::C0(_, _, _, h1580, _, _, _, _, _, _, _) => {
                    h1580
                }
            };
            nat_eq(a1581, a1582)
        } else {
            false
        } {
            let a1585 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1583, _, _, _, _, _, _) => {
                    h1583
                }
            };
            let a1586 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1584, _, _, _, _, _, _) => {
                    h1584
                }
            };
            nat_eq(a1585, a1586)
        } else {
            false
        }
    } else {
        false
    } {
        let m1588 = f110(match v1.clone() {
            Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
                h1587
            }
        });
        match m1588 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1589) => {
                        h1589
                    }
                }, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1590, _, _, _, _, _, _, _, _, _, _) => {
                        h1590
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f57(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1591, _, _, _, _, _, _, _, _, _, _, _) => {
                h1591
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1592, _, _, _, _, _, _, _, _, _) => {
                h1592
            }
        }
    };
    let a1593 = 0u64;
    let a1594 = v4;
    if if nat_lt(a1593, a1594) {
        let a1595 = v4;
        let a1596 = 2147483648u64;
        nat_lt(a1595, a1596)
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
        let v7: u64 = f119(match v5.clone() {
            Adt2::C0(_, _, h1597, _, _, _, _, _, _, _, _) => {
                h1597
            }
        }, v4)?;
        let v8: u64 = f119(match v5.clone() {
            Adt2::C0(_, _, _, h1598, _, _, _, _, _, _, _) => {
                h1598
            }
        }, v4)?;
        let a1600 = match v5.clone() {
            Adt2::C0(h1599, _, _, _, _, _, _, _, _, _, _) => {
                h1599
            }
        };
        let a1601 = 0u64;
        if if if if if nat_eq(a1600, a1601) {
            let a1603 = match v6.clone() {
                Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
                    h1602
                }
            };
            let a1604 = 0u64;
            nat_eq(a1603, a1604)
        } else {
            false
        } {
            let a1606 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1605, _, _, _, _, _, _) => {
                    h1605
                }
            };
            let a1607 = 1u64;
            nat_eq(a1606, a1607)
        } else {
            false
        } {
            let a1610 = match v5.clone() {
                Adt2::C0(_, h1608, _, _, _, _, _, _, _, _, _) => {
                    h1608
                }
            };
            let a1611 = match v6.clone() {
                Adt2::C0(_, h1609, _, _, _, _, _, _, _, _, _) => {
                    h1609
                }
            };
            nat_eq(a1610, a1611)
        } else {
            false
        } {
            if v0 {
                let a1613 = match v5.clone() {
                    Adt2::C0(_, _, h1612, _, _, _, _, _, _, _, _) => {
                        h1612
                    }
                };
                let a1614 = 2147483648u64;
                if if if nat_lt(a1613, a1614) {
                    let a1616 = match v5.clone() {
                        Adt2::C0(_, _, _, h1615, _, _, _, _, _, _, _) => {
                            h1615
                        }
                    };
                    let a1617 = 2147483648u64;
                    nat_lt(a1616, a1617)
                } else {
                    false
                } {
                    let a1620 = {
                        let a1618 = v7;
                        let a1619 = v8;
                        nat_mul(a1618, a1619)?
                    };
                    let a1621 = 2147483648u64;
                    nat_le(a1620, a1621)
                } else {
                    false
                } {
                    f120(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1623 = match v1.clone() {
                    Adt1::C0(h1622, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1622
                    }
                };
                let a1624 = v7;
                if if if if nat_eq(a1623, a1624) {
                    let a1626 = match v1 {
                        Adt1::C0(_, h1625, _, _, _, _, _, _, _, _, _, _) => {
                            h1625
                        }
                    };
                    let a1627 = v8;
                    nat_eq(a1626, a1627)
                } else {
                    false
                } {
                    f121(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1629 = match v6.clone() {
                        Adt2::C0(_, _, h1628, _, _, _, _, _, _, _, _) => {
                            h1628
                        }
                    };
                    let a1630 = v4;
                    if nat_eq(a1629, a1630) {
                        let a1632 = match v6.clone() {
                            Adt2::C0(_, _, _, h1631, _, _, _, _, _, _, _) => {
                                h1631
                            }
                        };
                        let a1633 = v4;
                        nat_eq(a1632, a1633)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1637 = match v6 {
                        Adt2::C0(_, _, _, _, h1634, _, _, _, _, _, _) => {
                            h1634
                        }
                    };
                    let a1638 = {
                        let a1635 = v7;
                        let a1636 = v8;
                        nat_mul(a1635, a1636)?
                    };
                    nat_eq(a1637, a1638)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f58(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1640 = match v2.clone() {
        Adt2::C0(h1639, _, _, _, _, _, _, _, _, _, _) => {
            h1639
        }
    };
    let a1641 = 0u64;
    if if if if if nat_eq(a1640, a1641) {
        true
    } else {
        let a1643 = match v2.clone() {
            Adt2::C0(h1642, _, _, _, _, _, _, _, _, _, _) => {
                h1642
            }
        };
        let a1644 = 1u64;
        nat_eq(a1643, a1644)
    } {
        true
    } else {
        let a1646 = match v2.clone() {
            Adt2::C0(h1645, _, _, _, _, _, _, _, _, _, _) => {
                h1645
            }
        };
        let a1647 = 30u64;
        nat_eq(a1646, a1647)
    } {
        let a1650 = match v2.clone() {
            Adt2::C0(h1648, _, _, _, _, _, _, _, _, _, _) => {
                h1648
            }
        };
        let a1651 = match v3.clone() {
            Adt2::C0(h1649, _, _, _, _, _, _, _, _, _, _) => {
                h1649
            }
        };
        nat_eq(a1650, a1651)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a1653 = match v1.clone() {
                Adt1::C0(h1652, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1652
                }
            };
            let a1654 = 4294967296u64;
            nat_lt(a1653, a1654)
        } else {
            false
        } {
            let a1656 = match v1.clone() {
                Adt1::C0(_, h1655, _, _, _, _, _, _, _, _, _, _) => {
                    h1655
                }
            };
            let a1657 = 4294967296u64;
            nat_lt(a1656, a1657)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
            let a1659 = match v2.clone() {
                Adt2::C0(h1658, _, _, _, _, _, _, _, _, _, _) => {
                    h1658
                }
            };
            let a1660 = 0u64;
            if nat_eq(a1659, a1660) {
                4u64
            } else {
                2u64
            }
        }, f18(v2.clone(), 1u64), f18(v3, 1u64), Adt4::C13({
            let a1661 = v0;
            let a1662 = 31u64;
            if nat_eq(a1661, a1662) {
                0u64
            } else {
                let a1663 = v0;
                let a1664 = 47u64;
                if nat_eq(a1663, a1664) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1665, _, _, _, _, _, _, _, _, _, _) => {
                h1665
            }
        }, match v1.clone() {
            Adt1::C0(h1666, _, _, _, _, _, _, _, _, _, _, _) => {
                h1666
            }
        }, match v1 {
            Adt1::C0(_, h1667, _, _, _, _, _, _, _, _, _, _) => {
                h1667
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f59(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1669 = match v0.clone() {
        Adt2::C0(h1668, _, _, _, _, _, _, _, _, _, _) => {
            h1668
        }
    };
    let a1670 = 0u64;
    if if if nat_eq(a1669, a1670) {
        let a1672 = match v1.clone() {
            Adt2::C0(h1671, _, _, _, _, _, _, _, _, _, _) => {
                h1671
            }
        };
        let a1673 = 0u64;
        nat_eq(a1672, a1673)
    } else {
        false
    } {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v1.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f60(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1675 = match v0.clone() {
        Adt2::C0(h1674, _, _, _, _, _, _, _, _, _, _) => {
            h1674
        }
    };
    let a1676 = 0u64;
    if if nat_eq(a1675, a1676) {
        let a1678 = match v1.clone() {
            Adt2::C0(h1677, _, _, _, _, _, _, _, _, _, _) => {
                h1677
            }
        };
        let a1679 = 26u64;
        nat_eq(a1678, a1679)
    } else {
        false
    } {
        let a1681 = match v0.clone() {
            Adt2::C0(_, _, _, h1680, _, _, _, _, _, _, _) => {
                h1680
            }
        };
        let a1682 = 1u64;
        if if if if if if if nat_eq(a1681, a1682) {
            let a1684 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1683, _, _, _, _, _, _) => {
                    h1683
                }
            };
            let a1685 = 1u64;
            nat_eq(a1684, a1685)
        } else {
            false
        } {
            let a1687 = match v0.clone() {
                Adt2::C0(_, h1686, _, _, _, _, _, _, _, _, _) => {
                    h1686
                }
            };
            let a1688 = 2147483647u64;
            nat_le(a1687, a1688)
        } else {
            false
        } {
            let a1691 = match v1.clone() {
                Adt2::C0(_, h1689, _, _, _, _, _, _, _, _, _) => {
                    h1689
                }
            };
            let a1692 = match v0.clone() {
                Adt2::C0(_, _, h1690, _, _, _, _, _, _, _, _) => {
                    h1690
                }
            };
            nat_eq(a1691, a1692)
        } else {
            false
        } {
            let a1694 = match v1.clone() {
                Adt2::C0(_, _, h1693, _, _, _, _, _, _, _, _) => {
                    h1693
                }
            };
            let a1695 = 1u64;
            nat_eq(a1694, a1695)
        } else {
            false
        } {
            let a1697 = match v1.clone() {
                Adt2::C0(_, _, _, h1696, _, _, _, _, _, _, _) => {
                    h1696
                }
            };
            let a1698 = 1u64;
            nat_eq(a1697, a1698)
        } else {
            false
        } {
            let a1700 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1699, _, _, _, _, _, _) => {
                    h1699
                }
            };
            let a1701 = 1u64;
            nat_eq(a1700, a1701)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1704 = match v0.clone() {
                    Adt2::C0(_, _, h1702, _, _, _, _, _, _, _, _) => {
                        h1702
                    }
                };
                let a1705 = f122(match v0.clone() {
                    Adt2::C0(_, h1703, _, _, _, _, _, _, _, _, _) => {
                        h1703
                    }
                })?;
                nat_mul(a1704, a1705)?
            }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f61(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1706 = v1;
    match m1706 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f123(f18(v0.clone(), 1u64))?;
            let a1708 = match v0.clone() {
                Adt2::C0(h1707, _, _, _, _, _, _, _, _, _, _) => {
                    h1707
                }
            };
            let a1709 = 26u64;
            if if if nat_eq(a1708, a1709) {
                let a1711 = match v3.clone() {
                    Adt2::C0(h1710, _, _, _, _, _, _, _, _, _, _) => {
                        h1710
                    }
                };
                let a1712 = 26u64;
                nat_eq(a1711, a1712)
            } else {
                false
            } {
                let a1714 = match v2.clone() {
                    Adt2::C0(h1713, _, _, _, _, _, _, _, _, _, _) => {
                        h1713
                    }
                };
                let a1715 = 27u64;
                nat_eq(a1714, a1715)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a1717 = match v2.clone() {
                        Adt2::C0(_, h1716, _, _, _, _, _, _, _, _, _) => {
                            h1716
                        }
                    };
                    let a1718 = 1u64;
                    nat_eq(a1717, a1718)
                } else {
                    false
                } {
                    let a1720 = match v2.clone() {
                        Adt2::C0(_, _, h1719, _, _, _, _, _, _, _, _) => {
                            h1719
                        }
                    };
                    let a1721 = 1u64;
                    nat_eq(a1720, a1721)
                } else {
                    false
                } {
                    let a1723 = match v2.clone() {
                        Adt2::C0(_, _, _, h1722, _, _, _, _, _, _, _) => {
                            h1722
                        }
                    };
                    let a1724 = 1u64;
                    nat_eq(a1723, a1724)
                } else {
                    false
                } {
                    let a1726 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1725, _, _, _, _, _, _) => {
                            h1725
                        }
                    };
                    let a1727 = 1u64;
                    nat_eq(a1726, a1727)
                } else {
                    false
                } {
                    let a1728 = v4;
                    let a1729 = 9223372036854775808u64;
                    nat_lt(a1728, a1729)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1732 = 2u64;
                        let a1733 = {
                            let a1730 = v4;
                            let a1731 = 2u64;
                            nat_mul(a1730, a1731)?
                        };
                        nat_add(a1732, a1733)?
                    }, 8u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C18(v4, f18(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f62(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1734, _, _, _, _, _, _, _, _, _, _) => {
            h1734
        }
    };
    let a1735 = v3;
    let a1736 = 0u64;
    if if nat_eq(a1735, a1736) {
        true
    } else {
        let a1737 = v0;
        let a1738 = 13u64;
        if nat_eq(a1737, a1738) {
            let a1739 = v3;
            let a1740 = 1u64;
            if nat_eq(a1739, a1740) {
                true
            } else {
                let a1741 = v3;
                let a1742 = 30u64;
                nat_eq(a1741, a1742)
            }
        } else {
            false
        }
    } {
        let a1744 = v3;
        let a1745 = match v2.clone() {
            Adt2::C0(h1743, _, _, _, _, _, _, _, _, _, _) => {
                h1743
            }
        };
        if if nat_eq(a1744, a1745) {
            let a1746 = v0;
            let a1747 = 13u64;
            if nat_eq(a1746, a1747) {
                let a1749 = match v2.clone() {
                    Adt2::C0(_, h1748, _, _, _, _, _, _, _, _, _) => {
                        h1748
                    }
                };
                let a1750 = 1u64;
                if if if nat_eq(a1749, a1750) {
                    let a1752 = match v2.clone() {
                        Adt2::C0(_, _, h1751, _, _, _, _, _, _, _, _) => {
                            h1751
                        }
                    };
                    let a1753 = 1u64;
                    nat_eq(a1752, a1753)
                } else {
                    false
                } {
                    let a1755 = match v2.clone() {
                        Adt2::C0(_, _, _, h1754, _, _, _, _, _, _, _) => {
                            h1754
                        }
                    };
                    let a1756 = 1u64;
                    nat_eq(a1755, a1756)
                } else {
                    false
                } {
                    let a1758 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1757, _, _, _, _, _, _) => {
                            h1757
                        }
                    };
                    let a1759 = 1u64;
                    nat_eq(a1758, a1759)
                } else {
                    false
                }
            } else {
                let a1761 = match v2.clone() {
                    Adt2::C0(_, h1760, _, _, _, _, _, _, _, _, _) => {
                        h1760
                    }
                };
                let a1762 = 1u64;
                if if if nat_eq(a1761, a1762) {
                    let a1765 = match v1.clone() {
                        Adt2::C0(_, _, h1763, _, _, _, _, _, _, _, _) => {
                            h1763
                        }
                    };
                    let a1766 = match v2.clone() {
                        Adt2::C0(_, _, h1764, _, _, _, _, _, _, _, _) => {
                            h1764
                        }
                    };
                    nat_eq(a1765, a1766)
                } else {
                    false
                } {
                    let a1769 = match v1.clone() {
                        Adt2::C0(_, _, _, h1767, _, _, _, _, _, _, _) => {
                            h1767
                        }
                    };
                    let a1770 = match v2.clone() {
                        Adt2::C0(_, _, _, h1768, _, _, _, _, _, _, _) => {
                            h1768
                        }
                    };
                    nat_eq(a1769, a1770)
                } else {
                    false
                } {
                    let a1773 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1771, _, _, _, _, _, _) => {
                            h1771
                        }
                    };
                    let a1774 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1772, _, _, _, _, _, _) => {
                            h1772
                        }
                    };
                    nat_eq(a1773, a1774)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f18(v1, 1u64);
            Ok(Adt10::C4(f124(v4.clone(), v0, v3)?, {
                let a1775 = v3;
                let a1776 = 0u64;
                if nat_eq(a1775, a1776) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f18(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f63(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1777 = v0;
    let a1778 = 89u64;
    if if nat_eq(a1777, a1778) {
        let a1779 = v1;
        let a1780 = 15u64;
        nat_eq(a1779, a1780)
    } else {
        false
    } {
        Ok(Some(22u64))
    } else {
        let a1781 = v0;
        let a1782 = 89u64;
        if if nat_eq(a1781, a1782) {
            let a1783 = 8u64;
            let a1784 = v1;
            if nat_le(a1783, a1784) {
                let a1785 = v1;
                let a1786 = 9u64;
                nat_le(a1785, a1786)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Some({
                let a1787 = v1;
                let a1788 = 12u64;
                nat_add(a1787, a1788)?
            }))
        } else {
            let a1789 = v0;
            let a1790 = 89u64;
            if if nat_eq(a1789, a1790) {
                let a1791 = v1;
                let a1792 = 4u64;
                nat_eq(a1791, a1792)
            } else {
                false
            } {
                Ok(Some(19u64))
            } else {
                let a1793 = 10u64;
                let a1794 = v0;
                if if nat_le(a1793, a1794) {
                    let a1795 = v0;
                    let a1796 = 12u64;
                    nat_le(a1795, a1796)
                } else {
                    false
                } {
                    Ok(Some({
                        let a1797 = v0;
                        let a1798 = 3u64;
                        nat_add(a1797, a1798)?
                    }))
                } else {
                    let a1799 = v0;
                    let a1800 = 89u64;
                    if if nat_eq(a1799, a1800) {
                        let a1801 = v1;
                        let a1802 = 7u64;
                        nat_eq(a1801, a1802)
                    } else {
                        false
                    } {
                        Ok(Some(17u64))
                    } else {
                        let a1803 = v0;
                        let a1804 = 89u64;
                        if if nat_eq(a1803, a1804) {
                            let a1805 = v1;
                            let a1806 = 10u64;
                            nat_eq(a1805, a1806)
                        } else {
                            false
                        } {
                            Ok(Some(18u64))
                        } else {
                            let a1807 = v0;
                            let a1808 = 89u64;
                            if if nat_eq(a1807, a1808) {
                                let a1809 = v1;
                                let a1810 = 13u64;
                                nat_eq(a1809, a1810)
                            } else {
                                false
                            } {
                                Ok(Some(16u64))
                            } else {
                                let a1811 = v0;
                                let a1812 = 8u64;
                                if nat_eq(a1811, a1812) {
                                    Ok(Some(5u64))
                                } else {
                                    let a1813 = v0;
                                    let a1814 = 9u64;
                                    if nat_eq(a1813, a1814) {
                                        Ok(Some(6u64))
                                    } else {
                                        let a1815 = v0;
                                        let a1816 = 89u64;
                                        if nat_eq(a1815, a1816) {
                                            let a1817 = v1;
                                            let a1818 = 0u64;
                                            if nat_eq(a1817, a1818) {
                                                Ok(Some(0u64))
                                            } else {
                                                let a1819 = v1;
                                                let a1820 = 1u64;
                                                if nat_eq(a1819, a1820) {
                                                    Ok(Some(1u64))
                                                } else {
                                                    let a1821 = v1;
                                                    let a1822 = 2u64;
                                                    if nat_eq(a1821, a1822) {
                                                        Ok(Some(2u64))
                                                    } else {
                                                        let a1823 = v1;
                                                        let a1824 = 3u64;
                                                        if nat_eq(a1823, a1824) {
                                                            Ok(Some(3u64))
                                                        } else {
                                                            let a1825 = v1;
                                                            let a1826 = 6u64;
                                                            if nat_eq(a1825, a1826) {
                                                                Ok(Some(4u64))
                                                            } else {
                                                                let a1827 = v1;
                                                                let a1828 = 12u64;
                                                                if nat_eq(a1827, a1828) {
                                                                    Ok(Some(7u64))
                                                                } else {
                                                                    let a1829 = v1;
                                                                    let a1830 = 11u64;
                                                                    if nat_eq(a1829, a1830) {
                                                                        Ok(Some(8u64))
                                                                    } else {
                                                                        let a1831 = v1;
                                                                        let a1832 = 18u64;
                                                                        if nat_eq(a1831, a1832) {
                                                                            Ok(Some(9u64))
                                                                        } else {
                                                                            let a1833 = v1;
                                                                            let a1834 = 19u64;
                                                                            if nat_eq(a1833, a1834) {
                                                                                Ok(Some(10u64))
                                                                            } else {
                                                                                let a1835 = v1;
                                                                                let a1836 = 20u64;
                                                                                if nat_eq(a1835, a1836) {
                                                                                    Ok(Some(11u64))
                                                                                } else {
                                                                                    let a1837 = v1;
                                                                                    let a1838 = 21u64;
                                                                                    if nat_eq(a1837, a1838) {
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

pub fn f64(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1839 = f125(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1839 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1840 = v0;
            let a1841 = 2u64;
            if if if if nat_eq(a1840, a1841) {
                true
            } else {
                let a1842 = v0;
                let a1843 = 5u64;
                nat_eq(a1842, a1843)
            } {
                true
            } else {
                let a1844 = v0;
                let a1845 = 6u64;
                nat_eq(a1844, a1845)
            } {
                true
            } else {
                let a1846 = v0;
                let a1847 = 7u64;
                nat_eq(a1846, a1847)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f18(v2.clone(), 1u64), f18(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1848 = v3;
                    match m1848 {
                        None => {
                            f18(v2, 1u64)
                        }
                        Some(v14) => {
                            f18(v14, 1u64)
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

pub fn f65(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1850 = match v1.clone() {
        Adt2::C0(h1849, _, _, _, _, _, _, _, _, _, _) => {
            h1849
        }
    };
    let a1851 = 0u64;
    if if if if if nat_eq(a1850, a1851) {
        true
    } else {
        let a1853 = match v1.clone() {
            Adt2::C0(h1852, _, _, _, _, _, _, _, _, _, _) => {
                h1852
            }
        };
        let a1854 = 1u64;
        nat_eq(a1853, a1854)
    } {
        true
    } else {
        let a1856 = match v1.clone() {
            Adt2::C0(h1855, _, _, _, _, _, _, _, _, _, _) => {
                h1855
            }
        };
        let a1857 = 30u64;
        nat_eq(a1856, a1857)
    } {
        let a1860 = match v1.clone() {
            Adt2::C0(h1858, _, _, _, _, _, _, _, _, _, _) => {
                h1858
            }
        };
        let a1861 = match v2.clone() {
            Adt2::C0(h1859, _, _, _, _, _, _, _, _, _, _) => {
                h1859
            }
        };
        if nat_eq(a1860, a1861) {
            true
        } else {
            let a1863 = match v2.clone() {
                Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                    h1862
                }
            };
            let a1864 = 0u64;
            nat_eq(a1863, a1864)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v2.clone(), 1u64)?, {
            let a1866 = match v2.clone() {
                Adt2::C0(h1865, _, _, _, _, _, _, _, _, _, _) => {
                    h1865
                }
            };
            let a1867 = 0u64;
            if nat_eq(a1866, a1867) {
                4u64
            } else {
                2u64
            }
        }, f18(v1.clone(), 1u64), f18(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1868, _, _, _, _, _, _, _, _, _, _) => {
                h1868
            }
        }, match v2 {
            Adt2::C0(h1869, _, _, _, _, _, _, _, _, _, _) => {
                h1869
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f66(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f126(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1870, _, _, _) => {
            h1870
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1871, _, _) => {
            h1871
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1872, _) => {
            h1872
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1873) => {
            h1873
        }
    })
}

pub fn f67(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1875 = v0;
        let a1876 = match v1.clone() {
            Adt3::C0(h1874, _, _, _, _, _, _, _) => {
                h1874
            }
        };
        let a1877 = 0u64;
        nat_quot(a1875, a1876, a1877)
    };
    let v4: u64 = {
        let a1879 = v3;
        let a1880 = match v1.clone() {
            Adt3::C0(_, h1878, _, _, _, _, _, _) => {
                h1878
            }
        };
        let a1881 = 0u64;
        nat_quot(a1879, a1880, a1881)
    };
    let a1882 = v2;
    let a1883 = 0u64;
    if nat_eq(a1882, a1883) {
        let a1885 = v0;
        let a1886 = match v1 {
            Adt3::C0(h1884, _, _, _, _, _, _, _) => {
                h1884
            }
        };
        let a1887 = 0u64;
        nat_rem(a1885, a1886, a1887)
    } else {
        let a1888 = v2;
        let a1889 = 1u64;
        if nat_eq(a1888, a1889) {
            let a1891 = v3;
            let a1892 = match v1 {
                Adt3::C0(_, h1890, _, _, _, _, _, _) => {
                    h1890
                }
            };
            let a1893 = 0u64;
            nat_rem(a1891, a1892, a1893)
        } else {
            let a1894 = v2;
            let a1895 = 2u64;
            if nat_eq(a1894, a1895) {
                let a1897 = v4;
                let a1898 = match v1 {
                    Adt3::C0(_, _, h1896, _, _, _, _, _) => {
                        h1896
                    }
                };
                let a1899 = 0u64;
                nat_rem(a1897, a1898, a1899)
            } else {
                let a1901 = v4;
                let a1902 = match v1 {
                    Adt3::C0(_, _, h1900, _, _, _, _, _) => {
                        h1900
                    }
                };
                let a1903 = 0u64;
                nat_quot(a1901, a1902, a1903)
            }
        }
    }
}

pub fn f68(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1905 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1904) => {
            h1904
        }
    };
    let a1906 = 0u64;
    if nat_eq(a1905, a1906) {
        Ok(v2)
    } else {
        let a1911 = {
            let a1908 = v2;
            let a1909 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1907) => {
                    h1907
                }
            };
            nat_mul(a1908, a1909)?
        };
        let a1912 = f67(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1910, _, _, _, _, _, _) => {
                h1910
            }
        }, 3u64);
        nat_add(a1911, a1912)
    }
}

pub fn f69(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1922 = {
            let a1919 = {
                let a1914 = v2;
                let a1915 = match v0.clone() {
                    Adt6::C0(_, h1913, _, _) => {
                        h1913
                    }
                };
                nat_mul(a1914, a1915)?
            };
            let a1920 = {
                let a1917 = v3;
                let a1918 = match v0.clone() {
                    Adt6::C0(_, _, _, h1916) => {
                        h1916
                    }
                };
                nat_mul(a1917, a1918)?
            };
            nat_add(a1919, a1920)?
        };
        let a1923 = f127(match v0.clone() {
            Adt6::C0(_, _, h1921, _) => {
                h1921
            }
        });
        nat_add(a1922, a1923)?
    };
    let v5: u64 = f128(match v0 {
        Adt6::C0(_, _, h1924, _) => {
            h1924
        }
    });
    let a1925 = v5;
    let a1926 = v4;
    if if nat_le(a1925, a1926) {
        let a1929 = {
            let a1927 = v4;
            let a1928 = v5;
            nat_sub(a1927, a1928)
        };
        let a1930 = v1;
        nat_lt(a1929, a1930)
    } else {
        false
    } {
        Ok(Some({
            let a1931 = v4;
            let a1932 = v5;
            nat_sub(a1931, a1932)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f70(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1933, _, _, _, _, _) => {
            h1933
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1934, _, _, _, _, _, _) => {
            h1934
        }
    };
    let v7: u64 = f67(v1, v6.clone(), 2u64);
    let a1936 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1935) => {
            h1935
        }
    };
    let a1937 = 0u64;
    if nat_eq(a1936, a1937) {
        Ok(Adt9::C8(Some(f66(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1940 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1939, _, _, _, _) => {
                    h1939
                }
            };
            match m1940 {
                Adt3::C0(_, _, h1938, _, _, _, _, _) => {
                    h1938
                }
            }
        };
        let m1946 = f69(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1941, _) => {
                h1941
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1942, _, _, _, _, _) => {
                h1942
            }
        }, {
            let a1943 = v2;
            let a1944 = v8;
            let a1945 = 0u64;
            nat_rem(a1943, a1944, a1945)
        }, v7)?;
        match m1946 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1953 = {
                        let a1951 = {
                            let a1947 = v2;
                            let a1948 = v8;
                            let a1949 = 0u64;
                            nat_quot(a1947, a1948, a1949)
                        };
                        let a1952 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1950) => {
                                h1950
                            }
                        };
                        nat_mul(a1951, a1952)?
                    };
                    let a1954 = f67(v1, v6, 3u64);
                    nat_add(a1953, a1954)?
                };
                Ok(Adt9::C8(Some(f66(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f71(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1955 = v1;
    match m1955 {
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
            f129(f130(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f131(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f132(v0.clone(), v24);
            f129(f133(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1956 = v3;
    match m1956 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f134(v0, v1)?, f134(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f136(v0, v1, v2.clone())?, f134(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f137(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f138(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f139(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f138(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f140(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f141(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f142(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f134(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f143(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v30, v31) => {
            f144(v0, v1, v2, v30, v31)
        }
        Adt4::C12(v32, v33, v34) => {
            f145(v0, v1, v2, v32, v33, v34)
        }
        Adt4::C13(_, v36, _, _) => {
            Ok(Adt9::C3(v36, v36, f134(v0, v1)?, f134(v0, v2)?))
        }
        Adt4::C14(_, v40, v41) => {
            Ok(Adt9::C3(v40, v41, f134(v0, v1)?, f134(v0, v2)?))
        }
        Adt4::C15 => {
            f146(v0, v1, v2)
        }
        Adt4::C16(v42, v43) => {
            f147(v0, v1, v2, v42, v43)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v44, v45) => {
            f148(v0, v1, v45, v44)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f134(f135(v0, match v2.clone() {
                Adt3::C0(h1957, _, _, _, _, _, _, _) => {
                    h1957
                }
            }, match v2.clone() {
                Adt3::C0(_, h1958, _, _, _, _, _, _) => {
                    h1958
                }
            })?, v1)?, f134(v0, v2)?))
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
            f149(v0, v50, v51, v52)
        }
        Adt4::C26(v58, _, _, _) => {
            let a1959 = v58;
            let a1960 = 64u64;
            if if if nat_eq(a1959, a1960) {
                true
            } else {
                let a1961 = v58;
                let a1962 = 65u64;
                nat_eq(a1961, a1962)
            } {
                true
            } else {
                let a1963 = v58;
                let a1964 = 66u64;
                nat_eq(a1963, a1964)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v57) => {
            f81(v0, v1, v2, v57)
        }
        Adt4::C28(v53) => {
            f78(v0, v1, v2, v53, 0u64)
        }
        Adt4::C29(v54) => {
            f79(v0, v1, v2, v54)
        }
        Adt4::C30(v55, _) => {
            f80(v0, v1, v2, v55)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f112(v3, v4);
    let v6: u64 = {
        let a1965 = v0;
        let a1966 = v5;
        let a1967 = 0u64;
        nat_quot(a1965, a1966, a1967)
    };
    let a1968 = v5;
    let a1969 = 1u64;
    if nat_eq(a1968, a1969) {
        Ok(f129(f150(v3, v4), v3, v4, 0u64, f134(v6, v1)?, 2u64, 0u64, 3u64, f134(v6, v2)?))
    } else {
        let a1973 = {
            let a1970 = v0;
            let a1971 = 2u64;
            let a1972 = 0u64;
            nat_rem(a1970, a1971, a1972)
        };
        let a1974 = 0u64;
        if nat_eq(a1973, a1974) {
            Ok(f129(f150(v3, 0u64), v3, 0u64, 0u64, f134(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f129(f150(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f134(v6, v2)?))
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f122(match v1.clone() {
        Adt3::C0(h1975, _, _, _, _, _, _, _) => {
            h1975
        }
    })?;
    let v5: u64 = {
        let a1976 = v0;
        let a1977 = v4;
        let a1978 = 0u64;
        nat_quot(a1976, a1977, a1978)
    };
    let v6: u64 = {
        let a1979 = v0;
        let a1980 = v4;
        let a1981 = 0u64;
        nat_rem(a1979, a1980, a1981)
    };
    let a1982 = v6;
    let a1983 = 0u64;
    if nat_eq(a1982, a1983) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1984 = v6;
        let a1985 = 1u64;
        if nat_eq(a1984, a1985) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1988 = {
                let a1986 = v6;
                let a1987 = 1u64;
                nat_add(a1986, a1987)?
            };
            let a1989 = v4;
            if nat_eq(a1988, a1989) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f134(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1990 = v6;
                    let a1991 = 2u64;
                    nat_sub(a1990, a1991)
                };
                let v8: u64 = {
                    let a1992 = v7;
                    let a1993 = 3u64;
                    let a1994 = 0u64;
                    nat_quot(a1992, a1993, a1994)
                };
                let v9: u64 = {
                    let a1995 = v7;
                    let a1996 = 3u64;
                    let a1997 = 0u64;
                    nat_rem(a1995, a1996, a1997)
                };
                let v10: u64 = f134({
                    let a2001 = {
                        let a1999 = v5;
                        let a2000 = match v1.clone() {
                            Adt3::C0(h1998, _, _, _, _, _, _, _) => {
                                h1998
                            }
                        };
                        nat_mul(a1999, a2000)?
                    };
                    let a2002 = v8;
                    nat_add(a2001, a2002)?
                }, v1)?;
                let a2003 = v9;
                let a2004 = 0u64;
                if nat_eq(a2003, a2004) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2005 = v9;
                    let a2006 = 1u64;
                    if nat_eq(a2005, a2006) {
                        let a2007 = v3;
                        let a2008 = 2u64;
                        if nat_eq(a2007, a2008) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2009 = v3;
                        let a2010 = 2u64;
                        if if nat_eq(a2009, a2010) {
                            true
                        } else {
                            let a2011 = v3;
                            let a2012 = 3u64;
                            nat_eq(a2011, a2012)
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

pub fn f75(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2014 = 2u64;
        let a2015 = match v2.clone() {
            Adt3::C0(_, h2013, _, _, _, _, _, _) => {
                h2013
            }
        };
        nat_mul(a2014, a2015)?
    };
    let a2016 = v1;
    let a2017 = v6;
    if nat_lt(a2016, a2017) {
        let a2021 = {
            let a2018 = v1;
            let a2019 = 2u64;
            let a2020 = 0u64;
            nat_rem(a2018, a2019, a2020)
        };
        let a2022 = 0u64;
        if nat_eq(a2021, a2022) {
            Ok(Adt9::C4(19u64, 4u64, f134({
                let a2023 = v1;
                let a2024 = 2u64;
                let a2025 = 0u64;
                nat_quot(a2023, a2024, a2025)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2026 = v5;
            let a2027 = 2147483648u64;
            if if nat_lt(a2026, a2027) {
                let a2029 = v5;
                let a2030 = match v3 {
                    Adt3::C0(_, h2028, _, _, _, _, _, _) => {
                        h2028
                    }
                };
                nat_lt(a2029, a2030)
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
            let a2033 = match v3.clone() {
                Adt3::C0(h2031, _, _, _, _, _, _, _) => {
                    h2031
                }
            };
            let a2034 = match v3.clone() {
                Adt3::C0(_, h2032, _, _, _, _, _, _) => {
                    h2032
                }
            };
            nat_mul(a2033, a2034)?
        };
        let a2037 = {
            let a2035 = v1;
            let a2036 = v6;
            nat_sub(a2035, a2036)
        };
        let a2038 = v7;
        if nat_lt(a2037, a2038) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f134({
                let a2039 = v1;
                let a2040 = v6;
                nat_sub(a2039, a2040)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2043 = {
                    let a2041 = v1;
                    let a2042 = v6;
                    nat_sub(a2041, a2042)
                };
                let a2044 = v7;
                nat_sub(a2043, a2044)
            };
            let v9: u64 = {
                let a2046 = 1u64;
                let a2047 = f151(v0, match v2.clone() {
                    Adt3::C0(h2045, _, _, _, _, _, _, _) => {
                        h2045
                    }
                })?;
                nat_add(a2046, a2047)?
            };
            let v10: u64 = {
                let a2048 = v8;
                let a2049 = v9;
                let a2050 = 0u64;
                nat_quot(a2048, a2049, a2050)
            };
            let v11: u64 = {
                let a2051 = v8;
                let a2052 = v9;
                let a2053 = 0u64;
                nat_rem(a2051, a2052, a2053)
            };
            let a2054 = v11;
            let a2055 = 0u64;
            if nat_eq(a2054, a2055) {
                Ok(Adt9::C4(19u64, 4u64, f134(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f100(v0) {
                    let a2059 = {
                        let a2056 = v11;
                        let a2057 = 2u64;
                        let a2058 = 0u64;
                        nat_rem(a2056, a2057, a2058)
                    };
                    let a2060 = 1u64;
                    if nat_eq(a2059, a2060) {
                        Ok(Adt9::C4(6u64, 0u64, f134({
                            let a2069 = {
                                let a2062 = v10;
                                let a2063 = match v2.clone() {
                                    Adt3::C0(h2061, _, _, _, _, _, _, _) => {
                                        h2061
                                    }
                                };
                                nat_mul(a2062, a2063)?
                            };
                            let a2070 = {
                                let a2066 = {
                                    let a2064 = v11;
                                    let a2065 = 1u64;
                                    nat_sub(a2064, a2065)
                                };
                                let a2067 = 2u64;
                                let a2068 = 0u64;
                                nat_quot(a2066, a2067, a2068)
                            };
                            nat_add(a2069, a2070)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f134({
                            let a2079 = {
                                let a2072 = v5;
                                let a2073 = match v2.clone() {
                                    Adt3::C0(h2071, _, _, _, _, _, _, _) => {
                                        h2071
                                    }
                                };
                                nat_mul(a2072, a2073)?
                            };
                            let a2080 = {
                                let a2076 = {
                                    let a2074 = v11;
                                    let a2075 = 1u64;
                                    nat_sub(a2074, a2075)
                                };
                                let a2077 = 2u64;
                                let a2078 = 0u64;
                                nat_quot(a2076, a2077, a2078)
                            };
                            nat_add(a2079, a2080)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f134({
                            let a2089 = {
                                let a2082 = v5;
                                let a2083 = match v2 {
                                    Adt3::C0(h2081, _, _, _, _, _, _, _) => {
                                        h2081
                                    }
                                };
                                nat_mul(a2082, a2083)?
                            };
                            let a2090 = {
                                let a2086 = {
                                    let a2084 = v11;
                                    let a2085 = 1u64;
                                    nat_sub(a2084, a2085)
                                };
                                let a2087 = 2u64;
                                let a2088 = 0u64;
                                nat_quot(a2086, a2087, a2088)
                            };
                            nat_add(a2089, a2090)?
                        }, v3)?))
                    }
                } else {
                    f152(v0, v10, {
                        let a2091 = v11;
                        let a2092 = 1u64;
                        nat_sub(a2091, a2092)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2093 = v1;
    let a2094 = f153(v2.clone(), v4.clone())?;
    if nat_lt(a2093, a2094) {
        let a2097 = v1;
        let a2098 = {
            let a2095 = 2u64;
            let a2096 = f123(v4.clone())?;
            nat_mul(a2095, a2096)?
        };
        if nat_lt(a2097, a2098) {
            let a2102 = {
                let a2099 = v1;
                let a2100 = 2u64;
                let a2101 = 0u64;
                nat_rem(a2099, a2100, a2101)
            };
            let a2103 = 0u64;
            if nat_eq(a2102, a2103) {
                f154({
                    let a2104 = v1;
                    let a2105 = 2u64;
                    let a2106 = 0u64;
                    nat_quot(a2104, a2105, a2106)
                }, v4)
            } else {
                if f155(match v2 {
                    Adt3::C0(_, h2107, _, _, _, _, _, _) => {
                        h2107
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2110 = v1;
                let a2111 = {
                    let a2108 = 2u64;
                    let a2109 = f123(v4.clone())?;
                    nat_mul(a2108, a2109)?
                };
                nat_sub(a2110, a2111)
            };
            let v7: u64 = {
                let a2115 = v6;
                let a2116 = {
                    let a2113 = 1u64;
                    let a2114 = match v2.clone() {
                        Adt3::C0(h2112, _, _, _, _, _, _, _) => {
                            h2112
                        }
                    };
                    nat_add(a2113, a2114)?
                };
                let a2117 = 0u64;
                nat_quot(a2115, a2116, a2117)
            };
            let v8: u64 = {
                let a2121 = v6;
                let a2122 = {
                    let a2119 = 1u64;
                    let a2120 = match v2.clone() {
                        Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                            h2118
                        }
                    };
                    nat_add(a2119, a2120)?
                };
                let a2123 = 0u64;
                nat_rem(a2121, a2122, a2123)
            };
            let a2124 = v8;
            let a2125 = 0u64;
            if nat_eq(a2124, a2125) {
                f154(v7, v4)
            } else {
                f156(v0, v7, {
                    let a2126 = v8;
                    let a2127 = 1u64;
                    nat_sub(a2126, a2127)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2130 = match v4.clone() {
            Adt3::C0(h2128, _, _, _, _, _, _, _) => {
                h2128
            }
        };
        let a2131 = match v4.clone() {
            Adt3::C0(_, h2129, _, _, _, _, _, _) => {
                h2129
            }
        };
        nat_mul(a2130, a2131)?
    };
    let v7: u64 = {
        let a2132 = 2u64;
        let a2133 = v6;
        nat_mul(a2132, a2133)?
    };
    let v8: u64 = f157(match v3.clone() {
        Adt3::C0(_, h2134, _, _, _, _, _, _) => {
            h2134
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2135, _, _, _, _, _) => {
            h2135
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2136, _, _, _, _) => {
            h2136
        }
    });
    let a2137 = v0;
    let a2138 = v7;
    if nat_lt(a2137, a2138) {
        let a2142 = {
            let a2139 = v0;
            let a2140 = 2u64;
            let a2141 = 0u64;
            nat_rem(a2139, a2140, a2141)
        };
        let a2143 = 0u64;
        if nat_eq(a2142, a2143) {
            Ok(Adt9::C4(19u64, 5u64, f134({
                let a2144 = v0;
                let a2145 = 2u64;
                let a2146 = 0u64;
                nat_quot(a2144, a2145, a2146)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f158(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2147 = v0;
            let a2148 = v7;
            nat_sub(a2147, a2148)
        };
        let v10: u64 = {
            let a2152 = v9;
            let a2153 = {
                let a2150 = 1u64;
                let a2151 = match v1.clone() {
                    Adt3::C0(h2149, _, _, _, _, _, _, _) => {
                        h2149
                    }
                };
                nat_add(a2150, a2151)?
            };
            let a2154 = 0u64;
            nat_quot(a2152, a2153, a2154)
        };
        let v11: u64 = {
            let a2158 = v9;
            let a2159 = {
                let a2156 = 1u64;
                let a2157 = match v1.clone() {
                    Adt3::C0(h2155, _, _, _, _, _, _, _) => {
                        h2155
                    }
                };
                nat_add(a2156, a2157)?
            };
            let a2160 = 0u64;
            nat_rem(a2158, a2159, a2160)
        };
        let a2161 = v11;
        let a2162 = 0u64;
        if nat_eq(a2161, a2162) {
            Ok(Adt9::C4(19u64, 5u64, f134({
                let a2163 = v10;
                let a2164 = v6;
                let a2165 = 0u64;
                nat_rem(a2163, a2164, a2165)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f158(v5, v8) {
                let v12: u64 = {
                    let a2171 = {
                        let a2167 = v10;
                        let a2168 = match v1.clone() {
                            Adt3::C0(h2166, _, _, _, _, _, _, _) => {
                                h2166
                            }
                        };
                        nat_mul(a2167, a2168)?
                    };
                    let a2172 = {
                        let a2169 = v11;
                        let a2170 = 1u64;
                        nat_sub(a2169, a2170)
                    };
                    nat_add(a2171, a2172)?
                };
                let a2173 = v5;
                let a2174 = 4294967295u64;
                if nat_eq(a2173, a2174) {
                    Ok(Adt9::C1(false, f134(v12, v1)?, f134(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f134(v12, v1)?, 4u64, f134({
                        let a2180 = {
                            let a2176 = v5;
                            let a2177 = match v3.clone() {
                                Adt3::C0(h2175, _, _, _, _, _, _, _) => {
                                    h2175
                                }
                            };
                            nat_mul(a2176, a2177)?
                        };
                        let a2181 = {
                            let a2178 = v11;
                            let a2179 = 1u64;
                            nat_sub(a2178, a2179)
                        };
                        nat_add(a2180, a2181)?
                    }, v3)?, 3u64, f134(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f105(v3.clone())?;
    let v6: u64 = {
        let a2182 = v0;
        let a2183 = v5;
        let a2184 = 0u64;
        nat_quot(a2182, a2183, a2184)
    };
    let v7: u64 = {
        let a2185 = v0;
        let a2186 = v5;
        let a2187 = 0u64;
        nat_rem(a2185, a2186, a2187)
    };
    let v8: u64 = f159(v3.clone())?;
    let a2188 = v7;
    let a2189 = 0u64;
    if nat_eq(a2188, a2189) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2190, _) => {
                h2190
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2194 = v5;
        let a2195 = {
            let a2192 = v7;
            let a2193 = if match v3.clone() {
                Adt7::C0(_, _, _, h2191, _) => {
                    h2191
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2192, a2193)?
        };
        if nat_le(a2194, a2195) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2196, _) => {
                    h2196
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f134(v6, v2)?))
            } else {
                let a2199 = {
                    let a2197 = v7;
                    let a2198 = 2u64;
                    nat_add(a2197, a2198)?
                };
                let a2200 = v5;
                if nat_eq(a2199, a2200) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2201, _, _) => {
                            h2201
                        }
                    } {
                        let a2208 = {
                            let m2204 = match v3.clone() {
                                Adt7::C0(h2203, _, _, _, _) => {
                                    h2203
                                }
                            };
                            match m2204 {
                                Adt6::C0(h2202, _, _, _) => {
                                    h2202
                                }
                            }
                        };
                        let a2209 = {
                            let m2207 = match v3 {
                                Adt7::C0(_, h2206, _, _, _) => {
                                    h2206
                                }
                            };
                            match m2207 {
                                Adt6::C0(h2205, _, _, _) => {
                                    h2205
                                }
                            }
                        };
                        nat_mul(a2208, a2209)?
                    } else {
                        f160(match v3 {
                            Adt7::C0(h2210, _, _, _, _) => {
                                h2210
                            }
                        }, match v1 {
                            Adt3::C0(h2211, _, _, _, _, _, _, _) => {
                                h2211
                            }
                        }, f67(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2212 = v9;
                        let a2213 = 0u64;
                        if nat_eq(a2212, a2213) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f134(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2216 = {
                    let a2214 = v7;
                    let a2215 = 1u64;
                    nat_sub(a2214, a2215)
                };
                let a2217 = v8;
                let a2218 = 0u64;
                nat_rem(a2216, a2217, a2218)
            };
            let m2224 = f161(v3.clone(), v1, v2, v6, {
                let a2221 = {
                    let a2219 = v7;
                    let a2220 = 1u64;
                    nat_sub(a2219, a2220)
                };
                let a2222 = v8;
                let a2223 = 0u64;
                nat_quot(a2221, a2222, a2223)
            })?;
            match m2224 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2226 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2225) => {
                            h2225
                        }
                    };
                    let a2227 = 1u64;
                    if if nat_eq(a2226, a2227) {
                        let a2228 = v10;
                        let a2229 = 0u64;
                        nat_eq(a2228, a2229)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2230, _) => {
                                h2230
                            }
                        } {
                            let a2233 = {
                                let a2231 = v10;
                                let a2232 = 1u64;
                                nat_add(a2231, a2232)?
                            };
                            let a2234 = v8;
                            if nat_eq(a2233, a2234) {
                                let a2235 = v4;
                                let a2236 = 2u64;
                                if nat_eq(a2235, a2236) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2238 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2237) => {
                                                h2237
                                            }
                                        };
                                        let a2239 = 1u64;
                                        if nat_eq(a2238, a2239) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2241 = match v3 {
                                            Adt7::C0(_, _, _, _, h2240) => {
                                                h2240
                                            }
                                        };
                                        let a2242 = 1u64;
                                        if nat_eq(a2241, a2242) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2244 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2243) => {
                                            h2243
                                        }
                                    };
                                    let a2245 = 1u64;
                                    if nat_eq(a2244, a2245) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2247 = match v3 {
                                        Adt7::C0(_, _, _, _, h2246) => {
                                            h2246
                                        }
                                    };
                                    let a2248 = 1u64;
                                    if nat_eq(a2247, a2248) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2250 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2249) => {
                                        h2249
                                    }
                                };
                                let a2251 = 1u64;
                                if nat_eq(a2250, a2251) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2253 = match v3 {
                                    Adt7::C0(_, _, _, _, h2252) => {
                                        h2252
                                    }
                                };
                                let a2254 = 1u64;
                                if nat_eq(a2253, a2254) {
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

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f103(match v3.clone() {
        Adt3::C0(h2255, _, _, _, _, _, _, _) => {
            h2255
        }
    })?;
    let v5: u64 = {
        let a2256 = v0;
        let a2257 = v4;
        let a2258 = 0u64;
        nat_rem(a2256, a2257, a2258)
    };
    let v6: u64 = {
        let a2259 = v0;
        let a2260 = v4;
        let a2261 = 0u64;
        nat_quot(a2259, a2260, a2261)
    };
    let a2262 = v5;
    let a2263 = 0u64;
    if nat_eq(a2262, a2263) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2266 = {
            let a2264 = v5;
            let a2265 = 1u64;
            nat_add(a2264, a2265)?
        };
        let a2267 = v4;
        if nat_eq(a2266, a2267) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f134(v6, v2)?))
        } else {
            let a2271 = {
                let a2268 = v5;
                let a2269 = 2u64;
                let a2270 = 0u64;
                nat_rem(a2268, a2269, a2270)
            };
            let a2272 = 0u64;
            if nat_eq(a2271, a2272) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2275 = {
                        let a2273 = v5;
                        let a2274 = 1u64;
                        nat_sub(a2273, a2274)
                    };
                    let a2276 = 2u64;
                    let a2277 = 0u64;
                    nat_quot(a2275, a2276, a2277)
                };
                let v8: u64 = f67(v6, v2.clone(), 0u64);
                let v9: u64 = f67(v6, v2.clone(), 1u64);
                let v10: u64 = f67(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f66(v1, {
                    let a2278 = v9;
                    let a2279 = v7;
                    nat_add(a2278, a2279)?
                }, v8, v10, 0u64)?, 4u64, f66(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f102(v3.clone())?;
    let v5: u64 = {
        let a2280 = v0;
        let a2281 = v4;
        let a2282 = 0u64;
        nat_rem(a2280, a2281, a2282)
    };
    let v6: u64 = {
        let a2283 = v0;
        let a2284 = v4;
        let a2285 = 0u64;
        nat_quot(a2283, a2284, a2285)
    };
    let a2286 = v5;
    let a2287 = 0u64;
    if nat_eq(a2286, a2287) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2290 = {
            let a2288 = v5;
            let a2289 = 1u64;
            nat_add(a2288, a2289)?
        };
        let a2291 = v4;
        if nat_eq(a2290, a2291) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f134(v6, v2)?))
        } else {
            let a2295 = {
                let a2292 = v5;
                let a2293 = 2u64;
                let a2294 = 0u64;
                nat_rem(a2292, a2293, a2294)
            };
            let a2296 = 0u64;
            if nat_eq(a2295, a2296) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2298 = f67(v6, v2.clone(), 0u64);
                    let a2299 = match v3.clone() {
                        Adt6::C0(_, _, h2297, _) => {
                            h2297
                        }
                    };
                    nat_add(a2298, a2299)?
                };
                let v8: u64 = {
                    let a2307 = {
                        let a2304 = {
                            let a2301 = v7;
                            let a2302 = match v3.clone() {
                                Adt6::C0(_, h2300, _, _) => {
                                    h2300
                                }
                            };
                            nat_add(a2301, a2302)?
                        };
                        let a2305 = match v3.clone() {
                            Adt6::C0(h2303, _, _, _) => {
                                h2303
                            }
                        };
                        nat_sub(a2304, a2305)
                    };
                    let a2308 = match v3.clone() {
                        Adt6::C0(_, h2306, _, _) => {
                            h2306
                        }
                    };
                    let a2309 = 0u64;
                    nat_quot(a2307, a2308, a2309)
                };
                let v9: u64 = {
                    let a2315 = v8;
                    let a2316 = {
                        let a2312 = {
                            let a2310 = v5;
                            let a2311 = 1u64;
                            nat_sub(a2310, a2311)
                        };
                        let a2313 = 2u64;
                        let a2314 = 0u64;
                        nat_quot(a2312, a2313, a2314)
                    };
                    nat_add(a2315, a2316)?
                };
                let v10: u64 = {
                    let a2318 = v9;
                    let a2319 = match v3.clone() {
                        Adt6::C0(_, h2317, _, _) => {
                            h2317
                        }
                    };
                    nat_mul(a2318, a2319)?
                };
                let v11: u64 = {
                    let a2320 = v7;
                    let a2321 = v10;
                    nat_sub(a2320, a2321)
                };
                let a2323 = v9;
                let a2324 = match v1.clone() {
                    Adt3::C0(_, h2322, _, _, _, _, _, _) => {
                        h2322
                    }
                };
                if if if nat_lt(a2323, a2324) {
                    let a2325 = v10;
                    let a2326 = v7;
                    nat_le(a2325, a2326)
                } else {
                    false
                } {
                    let a2328 = v11;
                    let a2329 = match v3.clone() {
                        Adt6::C0(h2327, _, _, _) => {
                            h2327
                        }
                    };
                    nat_lt(a2328, a2329)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f66(v1, {
                        let a2333 = {
                            let a2331 = f67(v6, v2, 1u64);
                            let a2332 = match v3 {
                                Adt6::C0(h2330, _, _, _) => {
                                    h2330
                                }
                            };
                            nat_mul(a2331, a2332)?
                        };
                        let a2334 = v11;
                        nat_add(a2333, a2334)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f67(v0, v2.clone(), 0u64);
    let v5: u64 = f67(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2336 = v5;
        let a2337 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2335, _) => {
                h2335
            }
        };
        let a2338 = 0u64;
        nat_rem(a2336, a2337, a2338)
    };
    let v7: u64 = {
        let a2340 = v5;
        let a2341 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2339, _) => {
                h2339
            }
        };
        let a2342 = 0u64;
        nat_quot(a2340, a2341, a2342)
    };
    let v8: Option<u64> = f69(match v3.clone() {
        Adt5::C0(h2343, _, _, _, _, _, _, _) => {
            h2343
        }
    }, match v1.clone() {
        Adt3::C0(h2344, _, _, _, _, _, _, _) => {
            h2344
        }
    }, f67(v0, v2.clone(), 1u64), {
        let a2348 = v4;
        let a2349 = {
            let m2347 = match v3.clone() {
                Adt5::C0(h2346, _, _, _, _, _, _, _) => {
                    h2346
                }
            };
            match m2347 {
                Adt6::C0(h2345, _, _, _) => {
                    h2345
                }
            }
        };
        let a2350 = 0u64;
        nat_rem(a2348, a2349, a2350)
    })?;
    let v9: u64 = {
        let a2354 = v4;
        let a2355 = {
            let m2353 = match v3.clone() {
                Adt5::C0(h2352, _, _, _, _, _, _, _) => {
                    h2352
                }
            };
            match m2353 {
                Adt6::C0(h2351, _, _, _) => {
                    h2351
                }
            }
        };
        let a2356 = 0u64;
        nat_quot(a2354, a2355, a2356)
    };
    let v10: Option<u64> = f69(match v3.clone() {
        Adt5::C0(_, h2357, _, _, _, _, _, _) => {
            h2357
        }
    }, match v1.clone() {
        Adt3::C0(_, h2358, _, _, _, _, _, _) => {
            h2358
        }
    }, f67(v0, v2.clone(), 2u64), {
        let a2362 = v9;
        let a2363 = {
            let m2361 = match v3.clone() {
                Adt5::C0(_, h2360, _, _, _, _, _, _) => {
                    h2360
                }
            };
            match m2361 {
                Adt6::C0(h2359, _, _, _) => {
                    h2359
                }
            }
        };
        let a2364 = 0u64;
        nat_rem(a2362, a2363, a2364)
    })?;
    let v11: u64 = {
        let a2368 = v9;
        let a2369 = {
            let m2367 = match v3.clone() {
                Adt5::C0(_, h2366, _, _, _, _, _, _) => {
                    h2366
                }
            };
            match m2367 {
                Adt6::C0(h2365, _, _, _) => {
                    h2365
                }
            }
        };
        let a2370 = 0u64;
        nat_quot(a2368, a2369, a2370)
    };
    let v12: Option<u64> = f69(match v3.clone() {
        Adt5::C0(_, _, h2371, _, _, _, _, _) => {
            h2371
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2372, _, _, _, _, _) => {
            h2372
        }
    }, v6, {
        let a2376 = v11;
        let a2377 = {
            let m2375 = match v3.clone() {
                Adt5::C0(_, _, h2374, _, _, _, _, _) => {
                    h2374
                }
            };
            match m2375 {
                Adt6::C0(h2373, _, _, _) => {
                    h2373
                }
            }
        };
        let a2378 = 0u64;
        nat_rem(a2376, a2377, a2378)
    })?;
    let v13: u64 = {
        let a2382 = v11;
        let a2383 = {
            let m2381 = match v3.clone() {
                Adt5::C0(_, _, h2380, _, _, _, _, _) => {
                    h2380
                }
            };
            match m2381 {
                Adt6::C0(h2379, _, _, _) => {
                    h2379
                }
            }
        };
        let a2384 = 0u64;
        nat_quot(a2382, a2383, a2384)
    };
    let v17: Adt12 = {
        let m2385 = v8;
        match m2385 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2386 = v10;
                match m2386 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2387 = v12;
                        match m2387 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2396 = {
                                        let a2391 = f66(v1, v14, v15, v16, 0u64)?;
                                        let a2392 = {
                                            let a2389 = v7;
                                            let a2390 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2388, _, _, _) => {
                                                    h2388
                                                }
                                            };
                                            nat_mul(a2389, a2390)?
                                        };
                                        nat_add(a2391, a2392)?
                                    };
                                    let a2397 = {
                                        let a2394 = v13;
                                        let a2395 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2393, _, _, _, _) => {
                                                h2393
                                            }
                                        };
                                        nat_mul(a2394, a2395)?
                                    };
                                    nat_add(a2396, a2397)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2398, _, _) => {
            h2398
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2399) => {
            h2399
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f134(v0, v2)?), {
        let a2401 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2400, _, _) => {
                h2400
            }
        };
        let a2402 = 0u64;
        if nat_eq(a2401, a2402) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2404 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2403) => {
                h2403
            }
        };
        let a2405 = 0u64;
        if nat_eq(a2404, a2405) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f82(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2418 = f162(v0, v1, match v2.clone() {
        Adt3::C0(h2406, _, _, _, _, _, _, _) => {
            h2406
        }
    }, match v2.clone() {
        Adt3::C0(_, h2407, _, _, _, _, _, _) => {
            h2407
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2408, _, _, _, _, _) => {
            h2408
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2409, _, _, _, _) => {
            h2409
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2410, _, _, _) => {
            h2410
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2411, _, _) => {
            h2411
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2412, _) => {
            h2412
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2413) => {
            h2413
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2414, _, _, _) => {
            h2414
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2415, _, _) => {
            h2415
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2416, _) => {
            h2416
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2417) => {
            h2417
        }
    }, v4)?;
    match m2418 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f13(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2421 = v6;
        let a2422 = {
            let a2419 = 3u64;
            let a2420 = v5;
            nat_add(a2419, a2420)?
        };
        nat_sub(a2421, a2422)
    };
    let a2424 = v0;
    let a2425 = f33(v3, match v1.clone() {
        Adt3::C0(h2423, _, _, _, _, _, _, _) => {
            h2423
        }
    }, v5, v8)?;
    if nat_lt(a2424, a2425) {
        let v9: u64 = f96(match v1.clone() {
            Adt3::C0(h2426, _, _, _, _, _, _, _) => {
                h2426
            }
        }, v5, v8)?;
        let v10: u64 = f97(v3, match v1.clone() {
            Adt3::C0(h2427, _, _, _, _, _, _, _) => {
                h2427
            }
        }, v5)?;
        let a2428 = v0;
        let a2429 = v9;
        if nat_lt(a2428, a2429) {
            f163(v0, v2, f27(v3), {
                let a2430 = 3u64;
                let a2431 = v5;
                nat_add(a2430, a2431)?
            }, v7)
        } else {
            let a2434 = v0;
            let a2435 = {
                let a2432 = v9;
                let a2433 = v10;
                nat_add(a2432, a2433)?
            };
            if nat_lt(a2434, a2435) {
                let v11: u64 = {
                    let a2436 = v0;
                    let a2437 = v9;
                    nat_sub(a2436, a2437)
                };
                let v12: u64 = f164(v3, match v1.clone() {
                    Adt3::C0(h2438, _, _, _, _, _, _, _) => {
                        h2438
                    }
                })?;
                f165({
                    let a2439 = v11;
                    let a2440 = v12;
                    let a2441 = 0u64;
                    nat_rem(a2439, a2440, a2441)
                }, {
                    let a2442 = v11;
                    let a2443 = v12;
                    let a2444 = 0u64;
                    nat_quot(a2442, a2443, a2444)
                }, v1, v3, v4, v7)
            } else {
                f166({
                    let a2447 = v0;
                    let a2448 = {
                        let a2445 = v9;
                        let a2446 = v10;
                        nat_add(a2445, a2446)?
                    };
                    nat_sub(a2447, a2448)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2449 = v0;
    let a2450 = f17(v1)?;
    if nat_lt(a2449, a2450) {
        let a2451 = v0;
        let a2452 = 4u64;
        if nat_lt(a2451, a2452) {
            let a2453 = v0;
            let a2454 = 0u64;
            if nat_eq(a2453, a2454) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2455 = v0;
                let a2456 = 1u64;
                if nat_eq(a2455, a2456) {
                    let a2457 = v5;
                    let a2458 = 0u64;
                    if nat_eq(a2457, a2458) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2459 = v0;
                    let a2460 = 2u64;
                    if nat_eq(a2459, a2460) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2461 = v0;
            let a2462 = 4u64;
            if if nat_eq(a2461, a2462) {
                let a2463 = f167(v5)?;
                let a2464 = v1;
                !nat_eq(a2463, a2464)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2467 = {
                        let a2465 = v0;
                        let a2466 = 4u64;
                        nat_sub(a2465, a2466)
                    };
                    let a2468 = 3u64;
                    let a2469 = 0u64;
                    nat_quot(a2467, a2468, a2469)
                };
                let v7: u64 = {
                    let a2472 = {
                        let a2470 = v0;
                        let a2471 = 4u64;
                        nat_sub(a2470, a2471)
                    };
                    let a2473 = 3u64;
                    let a2474 = 0u64;
                    nat_rem(a2472, a2473, a2474)
                };
                let a2475 = v7;
                let a2476 = 0u64;
                if nat_eq(a2475, a2476) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2477 = v7;
                    let a2478 = 1u64;
                    if nat_eq(a2477, a2478) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f16(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f85(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2479 = v1;
    let a2480 = 0u64;
    if nat_eq(a2479, a2480) {
        if v0 {
            let a2481 = v3;
            let a2482 = 0u64;
            if if nat_eq(a2481, a2482) {
                let a2483 = v4;
                let a2484 = 43u64;
                nat_eq(a2483, a2484)
            } else {
                false
            } {
                f150(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2485 = v1;
        let a2486 = 1u64;
        if nat_eq(a2485, a2486) {
            Adt11::C1(v2)
        } else {
            let a2487 = v1;
            let a2488 = 2u64;
            if nat_eq(a2487, a2488) {
                Adt11::C5(v2)
            } else {
                let a2489 = v1;
                let a2490 = 3u64;
                if nat_eq(a2489, a2490) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2491 = v1;
                    let a2492 = 4u64;
                    if nat_eq(a2491, a2492) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f86(v0: u64, v1: u64) -> Adt12 {
    let a2493 = v0;
    let a2494 = 0u64;
    if nat_eq(a2493, a2494) {
        Adt12::C0(0u64, v1)
    } else {
        let a2495 = v0;
        let a2496 = 1u64;
        if nat_eq(a2495, a2496) {
            Adt12::C1(v1)
        } else {
            let a2497 = v0;
            let a2498 = 2u64;
            if nat_eq(a2497, a2498) {
                Adt12::C2(v1)
            } else {
                let a2499 = v0;
                let a2500 = 3u64;
                if nat_eq(a2499, a2500) {
                    Adt12::C3(v1)
                } else {
                    let a2501 = v0;
                    let a2502 = 13u64;
                    if nat_lt(a2501, a2502) {
                        Adt12::C0({
                            let a2503 = v0;
                            let a2504 = 3u64;
                            nat_sub(a2503, a2504)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64) -> Adt13 {
    let a2505 = v0;
    let a2506 = 1u64;
    if nat_eq(a2505, a2506) {
        Adt13::C0(v1)
    } else {
        let a2507 = v0;
        let a2508 = 3u64;
        if if nat_eq(a2507, a2508) {
            true
        } else {
            let a2509 = v0;
            let a2510 = 6u64;
            nat_eq(a2509, a2510)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f88(v0: Adt6, v1: u64) -> R<u64> {
    let a2512 = 0u64;
    let a2513 = match v0.clone() {
        Adt6::C0(h2511, _, _, _) => {
            h2511
        }
    };
    if if if if if nat_lt(a2512, a2513) {
        let a2515 = 0u64;
        let a2516 = match v0.clone() {
            Adt6::C0(_, h2514, _, _) => {
                h2514
            }
        };
        nat_lt(a2515, a2516)
    } else {
        false
    } {
        let a2518 = 0u64;
        let a2519 = match v0.clone() {
            Adt6::C0(_, _, _, h2517) => {
                h2517
            }
        };
        nat_lt(a2518, a2519)
    } else {
        false
    } {
        let a2521 = match v0.clone() {
            Adt6::C0(_, h2520, _, _) => {
                h2520
            }
        };
        let a2522 = 2147483648u64;
        nat_lt(a2521, a2522)
    } else {
        false
    } {
        let a2524 = match v0.clone() {
            Adt6::C0(_, _, _, h2523) => {
                h2523
            }
        };
        let a2525 = 2147483648u64;
        nat_lt(a2524, a2525)
    } else {
        false
    } {
        let v2: u64 = {
            let a2529 = v1;
            let a2530 = {
                let a2527 = 2u64;
                let a2528 = f128(match v0.clone() {
                    Adt6::C0(_, _, h2526, _) => {
                        h2526
                    }
                });
                nat_mul(a2527, a2528)?
            };
            nat_add(a2529, a2530)?
        };
        let v3: u64 = {
            let a2542 = {
                let a2537 = 1u64;
                let a2538 = {
                    let a2535 = match v0.clone() {
                        Adt6::C0(_, _, _, h2531) => {
                            h2531
                        }
                    };
                    let a2536 = {
                        let a2533 = match v0.clone() {
                            Adt6::C0(h2532, _, _, _) => {
                                h2532
                            }
                        };
                        let a2534 = 1u64;
                        nat_sub(a2533, a2534)
                    };
                    nat_mul(a2535, a2536)?
                };
                nat_add(a2537, a2538)?
            };
            let a2543 = {
                let a2540 = 2u64;
                let a2541 = f127(match v0.clone() {
                    Adt6::C0(_, _, h2539, _) => {
                        h2539
                    }
                });
                nat_mul(a2540, a2541)?
            };
            nat_add(a2542, a2543)?
        };
        let a2544 = v3;
        let a2545 = v2;
        if nat_le(a2544, a2545) {
            let a2552 = 1u64;
            let a2553 = {
                let a2549 = {
                    let a2546 = v2;
                    let a2547 = v3;
                    nat_sub(a2546, a2547)
                };
                let a2550 = match v0 {
                    Adt6::C0(_, h2548, _, _) => {
                        h2548
                    }
                };
                let a2551 = 0u64;
                nat_quot(a2549, a2550, a2551)
            };
            nat_add(a2552, a2553)
        } else {
            let a2557 = {
                let a2554 = v3;
                let a2555 = v2;
                nat_sub(a2554, a2555)
            };
            let a2558 = match v0 {
                Adt6::C0(_, h2556, _, _) => {
                    h2556
                }
            };
            if nat_lt(a2557, a2558) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f89(v0: u64) -> u64 {
    let v1: u64 = {
        let a2559 = v0;
        let a2560 = 4294967296u64;
        let a2561 = 0u64;
        nat_rem(a2559, a2560, a2561)
    };
    let a2562 = v1;
    let a2563 = 2147483648u64;
    if nat_lt(a2562, a2563) {
        v1
    } else {
        0u64
    }
}

pub fn f90(v0: u64) -> R<u64> {
    let a2566 = 2u64;
    let a2567 = {
        let a2564 = 2u64;
        let a2565 = v0;
        nat_mul(a2564, a2565)?
    };
    nat_add(a2566, a2567)
}

pub fn f91(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2569 = match v1.clone() {
        Adt14::C0(h2568, _, _, _, _, _, _, _, _, _, _) => {
            h2568
        }
    };
    let a2570 = 0u64;
    if if if if nat_eq(a2569, a2570) {
        let a2572 = match v2.clone() {
            Adt14::C0(h2571, _, _, _, _, _, _, _, _, _, _) => {
                h2571
            }
        };
        let a2573 = 0u64;
        nat_eq(a2572, a2573)
    } else {
        false
    } {
        f168(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f169(match v1.clone() {
            Adt14::C0(_, h2574, _, _, _, _, _, _, _, _, _) => {
                h2574
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2575, _, _, _, _, _, _, _, _) => {
                h2575
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2576, _, _, _, _, _, _, _) => {
                h2576
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2577, _, _, _, _, _, _) => {
                h2577
            }
        }) {
            true
        } else {
            if if if if f28(v0) {
                f170(v1.clone(), v2.clone())
            } else {
                false
            } {
                f171(v1.clone())
            } else {
                false
            } {
                f171(v2.clone())
            } else {
                false
            } {
                if f172(v1.clone(), v2.clone())? {
                    true
                } else {
                    f173(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f169(match v1.clone() {
            Adt14::C0(_, h2578, _, _, _, _, _, _, _, _, _) => {
                h2578
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2579, _, _, _, _, _, _, _, _) => {
                h2579
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2580, _, _, _, _, _, _, _) => {
                h2580
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2581, _, _, _, _, _, _) => {
                h2581
            }
        }) {
            0u64
        } else {
            let a2587 = match v1.clone() {
                Adt14::C0(_, _, h2582, _, _, _, _, _, _, _, _) => {
                    h2582
                }
            };
            let a2588 = {
                let a2585 = match v1.clone() {
                    Adt14::C0(_, _, _, h2583, _, _, _, _, _, _, _) => {
                        h2583
                    }
                };
                let a2586 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2584, _, _, _, _, _, _) => {
                        h2584
                    }
                };
                nat_mul(a2585, a2586)?
            };
            nat_mul(a2587, a2588)?
        };
        let v4: u64 = {
            let a2589 = v3;
            let a2590 = 0u64;
            if if nat_eq(a2589, a2590) {
                true
            } else {
                f174(v2.clone())?
            } {
                0u64
            } else {
                let a2592 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2591, _) => {
                        h2591
                    }
                };
                let a2593 = 4u64;
                let a2594 = 0u64;
                nat_quot(a2592, a2593, a2594)
            }
        };
        Ok(Adt15::C1(f175(f176(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2595, _, _, _, _, _) => {
                h2595
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2596, _, _, _, _, _) => {
                h2596
            }
        }), match v1 {
            Adt14::C0(_, h2597, _, _, _, _, _, _, _, _, _) => {
                h2597
            }
        }, v3, v4)?, 4u64, 64u64, v3, f177(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f92(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f178(v3.clone());
    let a2599 = match v2.clone() {
        Adt14::C0(h2598, _, _, _, _, _, _, _, _, _, _) => {
            h2598
        }
    };
    let a2600 = 0u64;
    if if if if nat_eq(a2599, a2600) {
        let a2602 = match v3.clone() {
            Adt14::C0(h2601, _, _, _, _, _, _, _, _, _, _) => {
                h2601
            }
        };
        let a2603 = 0u64;
        nat_eq(a2602, a2603)
    } else {
        false
    } {
        f168(v2.clone(), v3)
    } else {
        false
    } {
        if f169(match v2.clone() {
            Adt14::C0(_, h2604, _, _, _, _, _, _, _, _, _) => {
                h2604
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2605, _, _, _, _, _, _, _, _) => {
                h2605
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2606, _, _, _, _, _, _, _) => {
                h2606
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2607, _, _, _, _, _, _) => {
                h2607
            }
        }) {
            true
        } else {
            let a2609 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2608, _, _, _, _, _) => {
                    h2608
                }
            };
            let a2610 = 4u64;
            if if if if if nat_eq(a2609, a2610) {
                f179(match v2.clone() {
                    Adt14::C0(_, h2611, _, _, _, _, _, _, _, _, _) => {
                        h2611
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2612, _, _, _, _, _, _, _, _) => {
                        h2612
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2613, _, _, _, _, _, _, _) => {
                        h2613
                    }
                }, v0)?
            } else {
                false
            } {
                f171(v2.clone())
            } else {
                false
            } {
                f171(v4.clone())
            } else {
                false
            } {
                let a2615 = f180(v4.clone())?;
                let a2616 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2614, _) => {
                        h2614
                    }
                };
                nat_le(a2615, a2616)
            } else {
                false
            } {
                if f172(v2.clone(), v4.clone())? {
                    true
                } else {
                    f173(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f169(match v2.clone() {
            Adt14::C0(_, h2617, _, _, _, _, _, _, _, _, _) => {
                h2617
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2618, _, _, _, _, _, _, _, _) => {
                h2618
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2619, _, _, _, _, _, _, _) => {
                h2619
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2620, _, _, _, _, _, _) => {
                h2620
            }
        }) {
            true
        } else {
            f174(v4.clone())?
        } {
            0u64
        } else {
            let a2622 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2621, _) => {
                    h2621
                }
            };
            let a2623 = 4u64;
            let a2624 = 0u64;
            nat_quot(a2622, a2623, a2624)
        };
        Ok(Adt15::C1(f181(match v2.clone() {
            Adt14::C0(_, h2625, _, _, _, _, _, _, _, _, _) => {
                h2625
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2626, _, _, _, _, _, _, _, _) => {
                h2626
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2627, _, _, _, _, _, _, _) => {
                h2627
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2628, _, _, _, _, _, _) => {
                h2628
            }
        }, v0, v5)?, 4u64, 65u64, v0, f177(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f93(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f178(v4.clone());
    if if if if if if if v1 {
        let a2630 = match v2.clone() {
            Adt14::C0(h2629, _, _, _, _, _, _, _, _, _, _) => {
                h2629
            }
        };
        let a2631 = 0u64;
        nat_eq(a2630, a2631)
    } else {
        false
    } {
        let a2633 = match v3.clone() {
            Adt14::C0(h2632, _, _, _, _, _, _, _, _, _, _) => {
                h2632
            }
        };
        let a2634 = 0u64;
        nat_eq(a2633, a2634)
    } else {
        false
    } {
        let a2636 = match v4.clone() {
            Adt14::C0(h2635, _, _, _, _, _, _, _, _, _, _) => {
                h2635
            }
        };
        let a2637 = 0u64;
        nat_eq(a2636, a2637)
    } else {
        false
    } {
        f168(v2.clone(), v3.clone())
    } else {
        false
    } {
        f168(v2.clone(), v4)
    } else {
        false
    } {
        if f169(match v2.clone() {
            Adt14::C0(_, h2638, _, _, _, _, _, _, _, _, _) => {
                h2638
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2639, _, _, _, _, _, _, _, _) => {
                h2639
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2640, _, _, _, _, _, _, _) => {
                h2640
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2641, _, _, _, _, _, _) => {
                h2641
            }
        }) {
            true
        } else {
            let a2643 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2642, _, _, _, _, _) => {
                    h2642
                }
            };
            let a2644 = 4u64;
            if if if if if if if nat_eq(a2643, a2644) {
                let a2646 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2645, _, _, _, _, _) => {
                        h2645
                    }
                };
                let a2647 = 4u64;
                nat_eq(a2646, a2647)
            } else {
                false
            } {
                f171(v2.clone())
            } else {
                false
            } {
                f171(v3.clone())
            } else {
                false
            } {
                f171(v5.clone())
            } else {
                false
            } {
                let a2649 = f180(v5.clone())?;
                let a2650 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2648, _) => {
                        h2648
                    }
                };
                nat_le(a2649, a2650)
            } else {
                false
            } {
                if f172(v2.clone(), v5.clone())? {
                    true
                } else {
                    f173(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f172(v3.clone(), v5.clone())? {
                    true
                } else {
                    f173(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f169(match v2.clone() {
            Adt14::C0(_, h2651, _, _, _, _, _, _, _, _, _) => {
                h2651
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2652, _, _, _, _, _, _, _, _) => {
                h2652
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2653, _, _, _, _, _, _, _) => {
                h2653
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2654, _, _, _, _, _, _) => {
                h2654
            }
        }) {
            0u64
        } else {
            let a2660 = match v2.clone() {
                Adt14::C0(_, _, h2655, _, _, _, _, _, _, _, _) => {
                    h2655
                }
            };
            let a2661 = {
                let a2658 = match v2.clone() {
                    Adt14::C0(_, _, _, h2656, _, _, _, _, _, _, _) => {
                        h2656
                    }
                };
                let a2659 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2657, _, _, _, _, _, _) => {
                        h2657
                    }
                };
                nat_mul(a2658, a2659)?
            };
            nat_mul(a2660, a2661)?
        };
        let v7: u64 = {
            let a2662 = v6;
            let a2663 = 0u64;
            if if nat_eq(a2662, a2663) {
                true
            } else {
                f174(v5.clone())?
            } {
                0u64
            } else {
                let a2665 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2664, _) => {
                        h2664
                    }
                };
                let a2666 = 4u64;
                let a2667 = 0u64;
                nat_quot(a2665, a2666, a2667)
            }
        };
        Ok(Adt15::C1(f182(match v2 {
            Adt14::C0(_, h2668, _, _, _, _, _, _, _, _, _) => {
                h2668
            }
        }, v6, v7)?, 4u64, 66u64, v6, f177(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f183(v0, v3, v6) {
        f183(v1, v4, v6)
    } else {
        false
    } {
        f183(v2, v5, v6)
    } else {
        false
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f184(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f184(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f184(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f184(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f184(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f184(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2669 = v2;
    let a2670 = 0u64;
    if nat_eq(a2669, a2670) {
        Ok(0u64)
    } else {
        let a2673 = 2u64;
        let a2674 = {
            let a2671 = v0;
            let a2672 = v1;
            nat_mul(a2671, a2672)?
        };
        nat_mul(a2673, a2674)
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2675 = v2;
    let a2676 = f164(v0, v1)?;
    nat_mul(a2675, a2676)
}

pub fn f98(v0: u64) -> u64 {
    let a2677 = v0;
    let a2678 = 30u64;
    if nat_eq(a2677, a2678) {
        3u64
    } else {
        1u64
    }
}

pub fn f99(v0: u64) -> R<u64> {
    let a2681 = 5u64;
    let a2682 = {
        let a2679 = 4u64;
        let a2680 = v0;
        nat_mul(a2679, a2680)?
    };
    nat_add(a2681, a2682)
}

pub fn f100(v0: u64) -> bool {
    let a2683 = v0;
    let a2684 = 0u64;
    if if nat_eq(a2683, a2684) {
        true
    } else {
        let a2685 = v0;
        let a2686 = 1u64;
        nat_eq(a2685, a2686)
    } {
        true
    } else {
        let a2687 = v0;
        let a2688 = 30u64;
        nat_eq(a2687, a2688)
    }
}

pub fn f101(v0: Adt2, v1: u64) -> R<bool> {
    let a2690 = match v0.clone() {
        Adt2::C0(_, _, _, h2689, _, _, _, _, _, _, _) => {
            h2689
        }
    };
    let a2691 = 1u64;
    if if if nat_eq(a2690, a2691) {
        let a2693 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2692, _, _, _, _, _, _) => {
                h2692
            }
        };
        let a2694 = 1u64;
        nat_eq(a2693, a2694)
    } else {
        false
    } {
        let a2696 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2695, _, _, _, _, _) => {
                h2695
            }
        };
        let a2697 = v1;
        nat_eq(a2696, a2697)
    } else {
        false
    } {
        let a2699 = match v0.clone() {
            Adt2::C0(_, _, h2698, _, _, _, _, _, _, _, _) => {
                h2698
            }
        };
        let a2700 = 1u64;
        if nat_eq(a2699, a2700) {
            Ok(true)
        } else {
            let a2705 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2701, _, _, _, _) => {
                    h2701
                }
            };
            let a2706 = {
                let a2703 = match v0 {
                    Adt2::C0(_, h2702, _, _, _, _, _, _, _, _, _) => {
                        h2702
                    }
                };
                let a2704 = v1;
                nat_mul(a2703, a2704)?
            };
            Ok(nat_eq(a2705, a2706))
        }
    } else {
        Ok(false)
    }
}

pub fn f102(v0: Adt6) -> R<u64> {
    let a2719 = 2u64;
    let a2720 = {
        let a2717 = 2u64;
        let a2718 = {
            let a2714 = {
                let a2711 = match v0.clone() {
                    Adt6::C0(h2707, _, _, _) => {
                        h2707
                    }
                };
                let a2712 = {
                    let a2709 = match v0.clone() {
                        Adt6::C0(_, h2708, _, _) => {
                            h2708
                        }
                    };
                    let a2710 = 1u64;
                    nat_sub(a2709, a2710)
                };
                nat_add(a2711, a2712)?
            };
            let a2715 = match v0 {
                Adt6::C0(_, h2713, _, _) => {
                    h2713
                }
            };
            let a2716 = 0u64;
            nat_quot(a2714, a2715, a2716)
        };
        nat_mul(a2717, a2718)?
    };
    nat_add(a2719, a2720)
}

pub fn f103(v0: u64) -> R<u64> {
    let a2723 = 2u64;
    let a2724 = {
        let a2721 = v0;
        let a2722 = 2u64;
        nat_mul(a2721, a2722)?
    };
    nat_add(a2723, a2724)
}

pub fn f104(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2728 = {
            let a2725 = v2;
            let a2726 = 1u64;
            nat_sub(a2725, a2726)
        };
        let a2729 = match v0.clone() {
            Adt6::C0(_, h2727, _, _) => {
                h2727
            }
        };
        nat_mul(a2728, a2729)?
    };
    let a2731 = 0u64;
    let a2732 = match v0.clone() {
        Adt6::C0(h2730, _, _, _) => {
            h2730
        }
    };
    if if if if if if if if nat_lt(a2731, a2732) {
        let a2734 = match v0.clone() {
            Adt6::C0(h2733, _, _, _) => {
                h2733
            }
        };
        let a2735 = 2147483648u64;
        nat_lt(a2734, a2735)
    } else {
        false
    } {
        let a2737 = 0u64;
        let a2738 = match v0.clone() {
            Adt6::C0(_, h2736, _, _) => {
                h2736
            }
        };
        nat_lt(a2737, a2738)
    } else {
        false
    } {
        let a2740 = match v0.clone() {
            Adt6::C0(_, h2739, _, _) => {
                h2739
            }
        };
        let a2741 = 2147483648u64;
        nat_lt(a2740, a2741)
    } else {
        false
    } {
        let a2742 = v1;
        let a2743 = 2147483648u64;
        nat_lt(a2742, a2743)
    } else {
        false
    } {
        let a2744 = 0u64;
        let a2745 = v2;
        nat_lt(a2744, a2745)
    } else {
        false
    } {
        let a2746 = v3;
        let a2747 = 2147483648u64;
        nat_lt(a2746, a2747)
    } else {
        false
    } {
        let a2759 = {
            let a2754 = {
                let a2751 = v3;
                let a2752 = {
                    let a2749 = match v0.clone() {
                        Adt6::C0(h2748, _, _, _) => {
                            h2748
                        }
                    };
                    let a2750 = 1u64;
                    nat_sub(a2749, a2750)
                };
                nat_add(a2751, a2752)?
            };
            let a2755 = f127(match v0.clone() {
                Adt6::C0(_, _, h2753, _) => {
                    h2753
                }
            });
            nat_add(a2754, a2755)?
        };
        let a2760 = {
            let a2757 = 2147483647u64;
            let a2758 = f128(match v0.clone() {
                Adt6::C0(_, _, h2756, _) => {
                    h2756
                }
            });
            nat_add(a2757, a2758)?
        };
        nat_le(a2759, a2760)
    } else {
        false
    } {
        let a2762 = match v0 {
            Adt6::C0(_, _, h2761, _) => {
                h2761
            }
        };
        let a2763 = 2147483648u64;
        Ok(!nat_eq(a2762, a2763))
    } else {
        Ok(false)
    }
}

pub fn f105(v0: Adt7) -> R<u64> {
    let a2775 = if match v0.clone() {
        Adt7::C0(_, _, _, h2764, _) => {
            h2764
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2776 = {
        let a2773 = {
            let a2771 = {
                let m2767 = match v0.clone() {
                    Adt7::C0(h2766, _, _, _, _) => {
                        h2766
                    }
                };
                match m2767 {
                    Adt6::C0(h2765, _, _, _) => {
                        h2765
                    }
                }
            };
            let a2772 = {
                let m2770 = match v0.clone() {
                    Adt7::C0(_, h2769, _, _, _) => {
                        h2769
                    }
                };
                match m2770 {
                    Adt6::C0(h2768, _, _, _) => {
                        h2768
                    }
                }
            };
            nat_mul(a2771, a2772)?
        };
        let a2774 = f159(v0)?;
        nat_mul(a2773, a2774)?
    };
    nat_add(a2775, a2776)
}

pub fn f106(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f185(match v0.clone() {
        Adt2::C0(h2777, _, _, _, _, _, _, _, _, _, _) => {
            h2777
        }
    }, match v0.clone() {
        Adt2::C0(_, h2778, _, _, _, _, _, _, _, _, _) => {
            h2778
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2779, _, _, _, _, _, _, _, _) => {
            h2779
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2780, _, _, _, _, _, _, _) => {
            h2780
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2781, _, _, _, _, _, _) => {
            h2781
        }
    }, match v1.clone() {
        Adt2::C0(h2782, _, _, _, _, _, _, _, _, _, _) => {
            h2782
        }
    }, match v1.clone() {
        Adt2::C0(_, h2783, _, _, _, _, _, _, _, _, _) => {
            h2783
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2784, _, _, _, _, _, _, _, _) => {
            h2784
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2785, _, _, _, _, _, _, _) => {
            h2785
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2786, _, _, _, _, _, _) => {
            h2786
        }
    }, match v2.clone() {
        Adt2::C0(h2787, _, _, _, _, _, _, _, _, _, _) => {
            h2787
        }
    }, match v2.clone() {
        Adt2::C0(_, h2788, _, _, _, _, _, _, _, _, _) => {
            h2788
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2789, _, _, _, _, _, _, _, _) => {
            h2789
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2790, _, _, _, _, _, _, _) => {
            h2790
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2791, _, _, _, _, _, _) => {
            h2791
        }
    }) {
        let m2793 = f110(match v0.clone() {
            Adt2::C0(h2792, _, _, _, _, _, _, _, _, _, _) => {
                h2792
            }
        });
        match m2793 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a2799 = {
                    let a2796 = match v0.clone() {
                        Adt2::C0(_, h2794, _, _, _, _, _, _, _, _, _) => {
                            h2794
                        }
                    };
                    let a2797 = match v3.clone() {
                        Adt18::C0(_, h2795, _) => {
                            h2795
                        }
                    };
                    let a2798 = 0u64;
                    nat_rem(a2796, a2797, a2798)
                };
                let a2800 = 0u64;
                if if nat_eq(a2799, a2800) {
                    if f186(match v0.clone() {
                        Adt2::C0(_, h2801, _, _, _, _, _, _, _, _, _) => {
                            h2801
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2802, _, _, _, _, _, _, _, _, _) => {
                            h2802
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2803, _, _, _, _, _, _, _, _) => {
                            h2803
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2804, _, _, _, _, _, _, _) => {
                            h2804
                        }
                    }) {
                        true
                    } else {
                        let a2806 = 4u64;
                        let a2807 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2805, _, _, _, _, _) => {
                                h2805
                            }
                        };
                        if if nat_le(a2806, a2807) {
                            let a2809 = match v3.clone() {
                                Adt18::C0(_, h2808, _) => {
                                    h2808
                                }
                            };
                            let a2810 = 1u64;
                            if nat_eq(a2809, a2810) {
                                true
                            } else {
                                let a2813 = match v3.clone() {
                                    Adt18::C0(_, _, h2811) => {
                                        h2811
                                    }
                                };
                                let a2814 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2812, _, _, _, _, _) => {
                                        h2812
                                    }
                                };
                                nat_le(a2813, a2814)
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
                    let v4: Adt3 = f18(v0.clone(), match v3 {
                        Adt18::C0(_, h2815, _) => {
                            h2815
                        }
                    });
                    Ok(Adt10::C4(if f186(match v0.clone() {
                        Adt2::C0(_, h2816, _, _, _, _, _, _, _, _, _) => {
                            h2816
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2817, _, _, _, _, _, _, _, _, _) => {
                            h2817
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2818, _, _, _, _, _, _, _, _) => {
                            h2818
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2819, _, _, _, _, _, _, _) => {
                            h2819
                        }
                    }) {
                        0u64
                    } else {
                        f153(v4.clone(), f18(v1.clone(), 1u64))?
                    }, 4u64, v4, f18(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2820, _, _, _, _, _, _, _, _, _, _) => {
                            h2820
                        }
                    }, f18(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f108(match v0.clone() {
        Adt2::C0(h2821, _, _, _, _, _, _, _, _, _, _) => {
            h2821
        }
    }) {
        let a2827 = {
            let a2824 = match v0.clone() {
                Adt2::C0(_, h2822, _, _, _, _, _, _, _, _, _) => {
                    h2822
                }
            };
            let a2825 = f187(match v0.clone() {
                Adt2::C0(h2823, _, _, _, _, _, _, _, _, _, _) => {
                    h2823
                }
            });
            let a2826 = 1u64;
            nat_rem(a2824, a2825, a2826)
        };
        let a2828 = 0u64;
        nat_eq(a2827, a2828)
    } else {
        false
    } {
        let a2830 = match v1.clone() {
            Adt2::C0(h2829, _, _, _, _, _, _, _, _, _, _) => {
                h2829
            }
        };
        let a2831 = 26u64;
        nat_eq(a2830, a2831)
    } else {
        false
    } {
        let a2833 = match v2.clone() {
            Adt2::C0(h2832, _, _, _, _, _, _, _, _, _, _) => {
                h2832
            }
        };
        let a2834 = 0u64;
        nat_eq(a2833, a2834)
    } else {
        false
    } {
        let a2837 = match v0.clone() {
            Adt2::C0(_, h2835, _, _, _, _, _, _, _, _, _) => {
                h2835
            }
        };
        let a2838 = match v2.clone() {
            Adt2::C0(_, h2836, _, _, _, _, _, _, _, _, _) => {
                h2836
            }
        };
        nat_eq(a2837, a2838)
    } else {
        false
    } {
        let a2841 = match v1.clone() {
            Adt2::C0(_, h2839, _, _, _, _, _, _, _, _, _) => {
                h2839
            }
        };
        let a2842 = match v0.clone() {
            Adt2::C0(_, _, h2840, _, _, _, _, _, _, _, _) => {
                h2840
            }
        };
        nat_le(a2841, a2842)
    } else {
        false
    } {
        let a2844 = match v0.clone() {
            Adt2::C0(_, _, _, h2843, _, _, _, _, _, _, _) => {
                h2843
            }
        };
        let a2845 = 1u64;
        nat_eq(a2844, a2845)
    } else {
        false
    } {
        let a2847 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2846, _, _, _, _, _, _) => {
                h2846
            }
        };
        let a2848 = 1u64;
        nat_eq(a2847, a2848)
    } else {
        false
    } {
        let a2850 = match v2.clone() {
            Adt2::C0(_, _, _, h2849, _, _, _, _, _, _, _) => {
                h2849
            }
        };
        let a2851 = 1u64;
        nat_eq(a2850, a2851)
    } else {
        false
    } {
        let a2853 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2852, _, _, _, _, _, _) => {
                h2852
            }
        };
        let a2854 = 1u64;
        nat_eq(a2853, a2854)
    } else {
        false
    } {
        let a2856 = match v1.clone() {
            Adt2::C0(_, _, h2855, _, _, _, _, _, _, _, _) => {
                h2855
            }
        };
        let a2857 = 1u64;
        nat_eq(a2856, a2857)
    } else {
        false
    } {
        let a2859 = match v1.clone() {
            Adt2::C0(_, _, _, h2858, _, _, _, _, _, _, _) => {
                h2858
            }
        };
        let a2860 = 1u64;
        nat_eq(a2859, a2860)
    } else {
        false
    } {
        let a2862 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2861, _, _, _, _, _, _) => {
                h2861
            }
        };
        let a2863 = 1u64;
        nat_eq(a2862, a2863)
    } else {
        false
    } {
        if f31(match v2.clone() {
            Adt2::C0(_, h2864, _, _, _, _, _, _, _, _, _) => {
                h2864
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2865, _, _, _, _, _, _, _, _) => {
                h2865
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2866, _, _, _, _, _, _, _) => {
                h2866
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2867, _, _, _, _, _, _) => {
                h2867
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2869 = match v1.clone() {
                Adt2::C0(_, h2868, _, _, _, _, _, _, _, _, _) => {
                    h2868
                }
            };
            let a2870 = 0u64;
            if nat_eq(a2869, a2870) {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2872 = match v0.clone() {
                    Adt2::C0(_, h2871, _, _, _, _, _, _, _, _, _) => {
                        h2871
                    }
                };
                let a2873 = 2147483647u64;
                if if if if if if nat_le(a2872, a2873) {
                    let a2875 = match v1.clone() {
                        Adt2::C0(_, h2874, _, _, _, _, _, _, _, _, _) => {
                            h2874
                        }
                    };
                    let a2876 = 2147483647u64;
                    nat_le(a2875, a2876)
                } else {
                    false
                } {
                    let a2878 = 4u64;
                    let a2879 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2877, _, _, _, _, _) => {
                            h2877
                        }
                    };
                    nat_le(a2878, a2879)
                } else {
                    false
                } {
                    let a2885 = {
                        let a2882 = match v2.clone() {
                            Adt2::C0(_, h2880, _, _, _, _, _, _, _, _, _) => {
                                h2880
                            }
                        };
                        let a2883 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2881, _, _, _, _, _) => {
                                h2881
                            }
                        };
                        nat_mul(a2882, a2883)?
                    };
                    let a2886 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2884, _, _, _, _) => {
                            h2884
                        }
                    };
                    nat_le(a2885, a2886)
                } else {
                    false
                } {
                    f20(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f20(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2899 = {
                            let a2890 = {
                                let a2888 = 2u64;
                                let a2889 = match v1.clone() {
                                    Adt2::C0(_, h2887, _, _, _, _, _, _, _, _, _) => {
                                        h2887
                                    }
                                };
                                nat_mul(a2888, a2889)?
                            };
                            let a2891 = f14(v2.clone(), 1u64)?;
                            nat_add(a2890, a2891)?
                        };
                        let a2900 = {
                            let a2897 = match v1.clone() {
                                Adt2::C0(_, h2892, _, _, _, _, _, _, _, _, _) => {
                                    h2892
                                }
                            };
                            let a2898 = {
                                let a2895 = 1u64;
                                let a2896 = f151(match v0.clone() {
                                    Adt2::C0(h2893, _, _, _, _, _, _, _, _, _, _) => {
                                        h2893
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h2894, _, _, _, _, _, _, _, _, _) => {
                                        h2894
                                    }
                                })?;
                                nat_add(a2895, a2896)?
                            };
                            nat_mul(a2897, a2898)?
                        };
                        nat_add(a2899, a2900)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2901, _, _, _, _, _, _, _, _, _) => {
                            h2901
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2902, _, _, _, _, _, _, _, _, _) => {
                            h2902
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2903, _, _, _, _, _) => {
                            h2903
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2904, _, _, _, _) => {
                            h2904
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2905, _, _, _) => {
                            h2905
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2906, _, _) => {
                            h2906
                        }
                    }), f18(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2907, _, _, _, _, _, _, _, _, _, _) => {
                            h2907
                        }
                    }, f18(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: u64) -> bool {
    let a2908 = v0;
    let a2909 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2908, a2909) {
        true
    } else {
        let a2910 = v0;
        let a2911 = 1u64;
        nat_eq(a2910, a2911)
    } {
        true
    } else {
        let a2912 = v0;
        let a2913 = 2u64;
        nat_eq(a2912, a2913)
    } {
        true
    } else {
        let a2914 = v0;
        let a2915 = 3u64;
        nat_eq(a2914, a2915)
    } {
        true
    } else {
        let a2916 = v0;
        let a2917 = 6u64;
        nat_eq(a2916, a2917)
    } {
        true
    } else {
        let a2918 = v0;
        let a2919 = 7u64;
        nat_eq(a2918, a2919)
    } {
        true
    } else {
        let a2920 = v0;
        let a2921 = 8u64;
        nat_eq(a2920, a2921)
    } {
        true
    } else {
        let a2922 = v0;
        let a2923 = 10u64;
        nat_eq(a2922, a2923)
    } {
        true
    } else {
        let a2924 = v0;
        let a2925 = 11u64;
        nat_eq(a2924, a2925)
    } {
        true
    } else {
        let a2926 = v0;
        let a2927 = 12u64;
        nat_eq(a2926, a2927)
    } {
        true
    } else {
        let a2928 = v0;
        let a2929 = 13u64;
        nat_eq(a2928, a2929)
    } {
        true
    } else {
        let a2930 = v0;
        let a2931 = 14u64;
        nat_eq(a2930, a2931)
    } {
        true
    } else {
        let a2932 = v0;
        let a2933 = 16u64;
        nat_eq(a2932, a2933)
    } {
        true
    } else {
        let a2934 = v0;
        let a2935 = 17u64;
        nat_eq(a2934, a2935)
    } {
        true
    } else {
        let a2936 = v0;
        let a2937 = 18u64;
        nat_eq(a2936, a2937)
    } {
        true
    } else {
        let a2938 = v0;
        let a2939 = 19u64;
        nat_eq(a2938, a2939)
    } {
        true
    } else {
        let a2940 = v0;
        let a2941 = 20u64;
        nat_eq(a2940, a2941)
    } {
        true
    } else {
        let a2942 = v0;
        let a2943 = 21u64;
        nat_eq(a2942, a2943)
    } {
        true
    } else {
        let a2944 = v0;
        let a2945 = 22u64;
        nat_eq(a2944, a2945)
    } {
        true
    } else {
        let a2946 = v0;
        let a2947 = 23u64;
        nat_eq(a2946, a2947)
    } {
        true
    } else {
        let a2948 = v0;
        let a2949 = 26u64;
        nat_eq(a2948, a2949)
    } {
        true
    } else {
        let a2950 = v0;
        let a2951 = 29u64;
        nat_eq(a2950, a2951)
    } {
        true
    } else {
        let a2952 = v0;
        let a2953 = 30u64;
        nat_eq(a2952, a2953)
    } {
        true
    } else {
        let a2954 = v0;
        let a2955 = 34u64;
        nat_eq(a2954, a2955)
    } {
        true
    } else {
        let a2956 = v0;
        let a2957 = 35u64;
        nat_eq(a2956, a2957)
    } {
        true
    } else {
        let a2958 = v0;
        let a2959 = 39u64;
        nat_eq(a2958, a2959)
    } {
        true
    } else {
        let a2960 = v0;
        let a2961 = 40u64;
        nat_eq(a2960, a2961)
    } {
        true
    } else {
        let a2962 = v0;
        let a2963 = 41u64;
        nat_eq(a2962, a2963)
    } {
        true
    } else {
        let a2964 = v0;
        let a2965 = 42u64;
        nat_eq(a2964, a2965)
    } {
        true
    } else {
        let a2966 = v0;
        let a2967 = 43u64;
        nat_eq(a2966, a2967)
    }
}

pub fn f109(v0: u64) -> bool {
    if f100(v0) {
        true
    } else {
        let a2968 = v0;
        let a2969 = 43u64;
        nat_eq(a2968, a2969)
    }
}

pub fn f110(v0: u64) -> Adt17 {
    let a2970 = v0;
    let a2971 = 0u64;
    if nat_eq(a2970, a2971) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a2972 = v0;
        let a2973 = 1u64;
        if nat_eq(a2972, a2973) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a2974 = v0;
            let a2975 = 2u64;
            if nat_eq(a2974, a2975) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a2976 = v0;
                let a2977 = 3u64;
                if nat_eq(a2976, a2977) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a2978 = v0;
                    let a2979 = 6u64;
                    if nat_eq(a2978, a2979) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2980 = v0;
                        let a2981 = 7u64;
                        if nat_eq(a2980, a2981) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2982 = v0;
                            let a2983 = 8u64;
                            if nat_eq(a2982, a2983) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2984 = v0;
                                let a2985 = 9u64;
                                if nat_eq(a2984, a2985) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2986 = v0;
                                    let a2987 = 10u64;
                                    if nat_eq(a2986, a2987) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2988 = v0;
                                        let a2989 = 11u64;
                                        if nat_eq(a2988, a2989) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2990 = v0;
                                            let a2991 = 12u64;
                                            if nat_eq(a2990, a2991) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2992 = v0;
                                                let a2993 = 13u64;
                                                if nat_eq(a2992, a2993) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2994 = v0;
                                                    let a2995 = 14u64;
                                                    if nat_eq(a2994, a2995) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2996 = v0;
                                                        let a2997 = 15u64;
                                                        if nat_eq(a2996, a2997) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2998 = v0;
                                                            let a2999 = 16u64;
                                                            if nat_eq(a2998, a2999) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3000 = v0;
                                                                let a3001 = 17u64;
                                                                if nat_eq(a3000, a3001) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3002 = v0;
                                                                    let a3003 = 18u64;
                                                                    if nat_eq(a3002, a3003) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3004 = v0;
                                                                        let a3005 = 19u64;
                                                                        if nat_eq(a3004, a3005) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3006 = v0;
                                                                            let a3007 = 20u64;
                                                                            if nat_eq(a3006, a3007) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3008 = v0;
                                                                                let a3009 = 21u64;
                                                                                if nat_eq(a3008, a3009) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3010 = v0;
                                                                                    let a3011 = 22u64;
                                                                                    if nat_eq(a3010, a3011) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3012 = v0;
                                                                                        let a3013 = 23u64;
                                                                                        if nat_eq(a3012, a3013) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3014 = v0;
                                                                                            let a3015 = 24u64;
                                                                                            if nat_eq(a3014, a3015) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3016 = v0;
                                                                                                let a3017 = 25u64;
                                                                                                if nat_eq(a3016, a3017) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3018 = v0;
                                                                                                    let a3019 = 26u64;
                                                                                                    if nat_eq(a3018, a3019) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3020 = v0;
                                                                                                        let a3021 = 27u64;
                                                                                                        if nat_eq(a3020, a3021) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3022 = v0;
                                                                                                            let a3023 = 28u64;
                                                                                                            if nat_eq(a3022, a3023) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3024 = v0;
                                                                                                                let a3025 = 29u64;
                                                                                                                if nat_eq(a3024, a3025) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3026 = v0;
                                                                                                                    let a3027 = 30u64;
                                                                                                                    if nat_eq(a3026, a3027) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3028 = v0;
                                                                                                                        let a3029 = 34u64;
                                                                                                                        if nat_eq(a3028, a3029) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3030 = v0;
                                                                                                                            let a3031 = 35u64;
                                                                                                                            if nat_eq(a3030, a3031) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3032 = v0;
                                                                                                                                let a3033 = 39u64;
                                                                                                                                if nat_eq(a3032, a3033) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3034 = v0;
                                                                                                                                    let a3035 = 40u64;
                                                                                                                                    if nat_eq(a3034, a3035) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3036 = v0;
                                                                                                                                        let a3037 = 41u64;
                                                                                                                                        if nat_eq(a3036, a3037) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3038 = v0;
                                                                                                                                            let a3039 = 42u64;
                                                                                                                                            if nat_eq(a3038, a3039) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3040 = v0;
                                                                                                                                                let a3041 = 43u64;
                                                                                                                                                if nat_eq(a3040, a3041) {
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

pub fn f111(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3044 = match v1.clone() {
        Adt2::C0(h3042, _, _, _, _, _, _, _, _, _, _) => {
            h3042
        }
    };
    let a3045 = match v3.clone() {
        Adt2::C0(h3043, _, _, _, _, _, _, _, _, _, _) => {
            h3043
        }
    };
    if if nat_eq(a3044, a3045) {
        let a3046 = v0;
        let a3047 = 33u64;
        if nat_eq(a3046, a3047) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3049 = f110(match v1.clone() {
            Adt2::C0(h3048, _, _, _, _, _, _, _, _, _, _) => {
                h3048
            }
        });
        match m3049 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3050, _) => {
                        h3050
                    }
                };
                let v6: u64 = f14(v1.clone(), v5)?;
                let a3055 = {
                    let a3052 = match v1.clone() {
                        Adt2::C0(_, h3051, _, _, _, _, _, _, _, _, _) => {
                            h3051
                        }
                    };
                    let a3053 = v5;
                    let a3054 = 1u64;
                    nat_rem(a3052, a3053, a3054)
                };
                let a3056 = 0u64;
                if if if nat_eq(a3055, a3056) {
                    let a3061 = {
                        let a3058 = match v3.clone() {
                            Adt2::C0(_, h3057, _, _, _, _, _, _, _, _, _) => {
                                h3057
                            }
                        };
                        let a3059 = v5;
                        let a3060 = 1u64;
                        nat_rem(a3058, a3059, a3060)
                    };
                    let a3062 = 0u64;
                    nat_eq(a3061, a3062)
                } else {
                    false
                } {
                    let a3063 = v6;
                    let a3064 = f14(v3.clone(), v5)?;
                    nat_eq(a3063, a3064)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3065) => {
                            h3065
                        }
                    }, f18(v1, v5), f18(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: u64, v1: u64) -> u64 {
    let a3066 = v0;
    let a3067 = 0u64;
    if if nat_eq(a3066, a3067) {
        true
    } else {
        let a3068 = v1;
        let a3069 = 0u64;
        nat_eq(a3068, a3069)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f31(v4, v5, v6, v7) {
        f31(v0, v1, v2, v3)
    } else {
        let a3073 = {
            let a3070 = v0;
            let a3071 = v4;
            let a3072 = 1u64;
            nat_rem(a3070, a3071, a3072)
        };
        let a3074 = 0u64;
        if if if nat_eq(a3073, a3074) {
            let a3078 = {
                let a3075 = v1;
                let a3076 = v5;
                let a3077 = 1u64;
                nat_rem(a3075, a3076, a3077)
            };
            let a3079 = 0u64;
            nat_eq(a3078, a3079)
        } else {
            false
        } {
            let a3083 = {
                let a3080 = v2;
                let a3081 = v6;
                let a3082 = 1u64;
                nat_rem(a3080, a3081, a3082)
            };
            let a3084 = 0u64;
            nat_eq(a3083, a3084)
        } else {
            false
        } {
            let a3088 = {
                let a3085 = v3;
                let a3086 = v7;
                let a3087 = 1u64;
                nat_rem(a3085, a3086, a3087)
            };
            let a3089 = 0u64;
            nat_eq(a3088, a3089)
        } else {
            false
        }
    }
}

pub fn f114(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3090 = v0;
    let a3091 = v1;
    if nat_eq(a3090, a3091) {
        let a3094 = v4;
        let a3095 = {
            let a3092 = v2;
            let a3093 = v3;
            nat_add(a3092, a3093)?
        };
        Ok(nat_eq(a3094, a3095))
    } else {
        let a3096 = v2;
        let a3097 = v3;
        if nat_eq(a3096, a3097) {
            let a3098 = v4;
            let a3099 = v2;
            Ok(nat_eq(a3098, a3099))
        } else {
            Ok(false)
        }
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3102 = {
            let a3100 = v0;
            let a3101 = f128(v1);
            nat_add(a3100, a3101)?
        };
        let a3103 = f128(v2);
        nat_add(a3102, a3103)?
    };
    let v5: u64 = {
        let a3104 = f127(v1);
        let a3105 = f127(v2);
        nat_add(a3104, a3105)?
    };
    let a3106 = v1;
    let a3107 = 4294967295u64;
    if if nat_le(a3106, a3107) {
        let a3108 = v2;
        let a3109 = 4294967295u64;
        nat_le(a3108, a3109)
    } else {
        false
    } {
        let a3110 = v5;
        let a3111 = v4;
        if nat_le(a3110, a3111) {
            let a3114 = v3;
            let a3115 = {
                let a3112 = v4;
                let a3113 = v5;
                nat_sub(a3112, a3113)
            };
            Ok(nat_eq(a3114, a3115))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f116(v0: u64, v1: u64) -> R<bool> {
    let a3116 = v1;
    let a3117 = 4294967295u64;
    if nat_le(a3116, a3117) {
        let a3120 = {
            let a3118 = f128(v1);
            let a3119 = f127(v1);
            nat_add(a3118, a3119)?
        };
        let a3121 = v0;
        Ok(nat_lt(a3120, a3121))
    } else {
        Ok(false)
    }
}

pub fn f117(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f188(v0.clone(), false)?;
    let v6: u64 = f188(v0, true)?;
    let a3122 = v6;
    let a3123 = v1;
    if if nat_le(a3122, a3123) {
        let a3130 = {
            let a3128 = {
                let a3124 = v1;
                let a3125 = v5;
                nat_add(a3124, a3125)?
            };
            let a3129 = {
                let a3126 = v3;
                let a3127 = 0u64;
                if nat_eq(a3126, a3127) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3128, a3129)?
        };
        let a3131 = v2;
        nat_le(a3130, a3131)
    } else {
        false
    } {
        if v4 {
            let a3136 = {
                let a3134 = {
                    let a3132 = v1;
                    let a3133 = v5;
                    nat_add(a3132, a3133)?
                };
                let a3135 = v6;
                nat_sub(a3134, a3135)
            };
            let a3137 = v2;
            Ok(nat_lt(a3136, a3137))
        } else {
            let a3142 = {
                let a3140 = {
                    let a3138 = v1;
                    let a3139 = v5;
                    nat_add(a3138, a3139)?
                };
                let a3141 = v6;
                nat_sub(a3140, a3141)
            };
            let a3143 = v2;
            Ok(nat_le(a3142, a3143))
        }
    } else {
        Ok(false)
    }
}

pub fn f118(v0: u64, v1: u64) -> bool {
    let a3144 = v0;
    let a3145 = 0u64;
    if if nat_eq(a3144, a3145) {
        true
    } else {
        let a3146 = v0;
        let a3147 = 1u64;
        nat_eq(a3146, a3147)
    } {
        let a3148 = v1;
        let a3149 = 4294967296u64;
        nat_lt(a3148, a3149)
    } else {
        false
    }
}

pub fn f119(v0: u64, v1: u64) -> R<u64> {
    let a3154 = {
        let a3152 = v0;
        let a3153 = {
            let a3150 = v1;
            let a3151 = 1u64;
            nat_sub(a3150, a3151)
        };
        nat_add(a3152, a3153)?
    };
    let a3155 = v1;
    let a3156 = 0u64;
    Ok(nat_quot(a3154, a3155, a3156))
}

pub fn f120(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f31(match v0.clone() {
        Adt2::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
            h3157
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3158, _, _, _, _, _, _, _, _) => {
            h3158
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3159, _, _, _, _, _, _, _) => {
            h3159
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3160, _, _, _, _, _, _) => {
            h3160
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3163 = match v1.clone() {
                Adt2::C0(_, h3161, _, _, _, _, _, _, _, _, _) => {
                    h3161
                }
            };
            let a3164 = match v1.clone() {
                Adt2::C0(_, _, h3162, _, _, _, _, _, _, _, _) => {
                    h3162
                }
            };
            nat_mul(a3163, a3164)?
        };
        let v5: u64 = {
            let a3166 = v4;
            let a3167 = match v1.clone() {
                Adt2::C0(_, _, _, h3165, _, _, _, _, _, _, _) => {
                    h3165
                }
            };
            nat_mul(a3166, a3167)?
        };
        let a3180 = {
            let a3178 = {
                let a3169 = match v0.clone() {
                    Adt2::C0(_, h3168, _, _, _, _, _, _, _, _, _) => {
                        h3168
                    }
                };
                let a3170 = 1u64;
                nat_sub(a3169, a3170)
            };
            let a3179 = {
                let a3176 = f189(match v0.clone() {
                    Adt2::C0(_, _, h3171, _, _, _, _, _, _, _, _) => {
                        h3171
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3172, _, _, _, _, _, _, _, _, _) => {
                        h3172
                    }
                })?;
                let a3177 = f189(match v0 {
                    Adt2::C0(_, _, _, h3173, _, _, _, _, _, _, _) => {
                        h3173
                    }
                }, v2, {
                    let a3174 = v3;
                    let a3175 = v5;
                    nat_mul(a3174, a3175)?
                }, v4)?;
                nat_add(a3176, a3177)?
            };
            nat_add(a3178, a3179)?
        };
        let a3181 = f14(v1, 1u64)?;
        Ok(nat_lt(a3180, a3181))
    }
}

pub fn f121(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3182 = v0;
    let a3183 = 2147483648u64;
    if if if if nat_lt(a3182, a3183) {
        let a3184 = v1;
        let a3185 = 2147483648u64;
        nat_lt(a3184, a3185)
    } else {
        false
    } {
        let a3188 = {
            let a3186 = v0;
            let a3187 = v1;
            nat_mul(a3186, a3187)?
        };
        let a3189 = 2147483648u64;
        nat_lt(a3188, a3189)
    } else {
        false
    } {
        let a3194 = {
            let a3192 = {
                let a3190 = v0;
                let a3191 = 1u64;
                nat_sub(a3190, a3191)
            };
            let a3193 = v2;
            nat_mul(a3192, a3193)?
        };
        let a3195 = 2147483648u64;
        nat_lt(a3194, a3195)
    } else {
        false
    } {
        let a3200 = {
            let a3198 = {
                let a3196 = v1;
                let a3197 = 1u64;
                nat_sub(a3196, a3197)
            };
            let a3199 = v2;
            nat_mul(a3198, a3199)?
        };
        let a3201 = 2147483648u64;
        Ok(nat_lt(a3200, a3201))
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64) -> R<u64> {
    let a3204 = 3u64;
    let a3205 = {
        let a3202 = v0;
        let a3203 = 3u64;
        nat_mul(a3202, a3203)?
    };
    nat_add(a3204, a3205)
}

pub fn f123(v0: Adt3) -> R<u64> {
    if f31(match v0.clone() {
        Adt3::C0(h3206, _, _, _, _, _, _, _) => {
            h3206
        }
    }, match v0.clone() {
        Adt3::C0(_, h3207, _, _, _, _, _, _) => {
            h3207
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3208, _, _, _, _, _) => {
            h3208
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3209, _, _, _, _) => {
            h3209
        }
    }) {
        Ok(0u64)
    } else {
        let a3218 = match v0.clone() {
            Adt3::C0(h3210, _, _, _, _, _, _, _) => {
                h3210
            }
        };
        let a3219 = {
            let a3216 = match v0.clone() {
                Adt3::C0(_, h3211, _, _, _, _, _, _) => {
                    h3211
                }
            };
            let a3217 = {
                let a3214 = match v0.clone() {
                    Adt3::C0(_, _, h3212, _, _, _, _, _) => {
                        h3212
                    }
                };
                let a3215 = match v0 {
                    Adt3::C0(_, _, _, h3213, _, _, _, _) => {
                        h3213
                    }
                };
                nat_mul(a3214, a3215)?
            };
            nat_mul(a3216, a3217)?
        };
        nat_mul(a3218, a3219)
    }
}

pub fn f124(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3220 = v1;
    let a3221 = 13u64;
    if if nat_eq(a3220, a3221) {
        f31(match v0.clone() {
            Adt3::C0(h3222, _, _, _, _, _, _, _) => {
                h3222
            }
        }, match v0.clone() {
            Adt3::C0(_, h3223, _, _, _, _, _, _) => {
                h3223
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3224, _, _, _, _, _) => {
                h3224
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3225, _, _, _, _) => {
                h3225
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3231 = match v0.clone() {
                Adt3::C0(_, h3226, _, _, _, _, _, _) => {
                    h3226
                }
            };
            let a3232 = {
                let a3229 = match v0.clone() {
                    Adt3::C0(_, _, h3227, _, _, _, _, _) => {
                        h3227
                    }
                };
                let a3230 = match v0.clone() {
                    Adt3::C0(_, _, _, h3228, _, _, _, _) => {
                        h3228
                    }
                };
                nat_mul(a3229, a3230)?
            };
            nat_mul(a3231, a3232)?
        };
        let a3233 = v1;
        let a3234 = 13u64;
        if nat_eq(a3233, a3234) {
            let a3243 = 2u64;
            let a3244 = if f190(v0.clone(), v2) {
                let a3236 = v3;
                let a3237 = f191(match v0 {
                    Adt3::C0(h3235, _, _, _, _, _, _, _) => {
                        h3235
                    }
                }, false)?;
                nat_mul(a3236, a3237)?
            } else {
                let a3241 = v3;
                let a3242 = {
                    let a3239 = match v0 {
                        Adt3::C0(h3238, _, _, _, _, _, _, _) => {
                            h3238
                        }
                    };
                    let a3240 = 2u64;
                    nat_mul(a3239, a3240)?
                };
                nat_mul(a3241, a3242)?
            };
            nat_add(a3243, a3244)
        } else {
            let a3248 = v3;
            let a3249 = f191(match v0 {
                Adt3::C0(h3245, _, _, _, _, _, _, _) => {
                    h3245
                }
            }, {
                let a3246 = v1;
                let a3247 = 16u64;
                nat_eq(a3246, a3247)
            })?;
            nat_mul(a3248, a3249)
        }
    }
}

pub fn f125(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3250 = v0;
    let a3251 = 30u64;
    if if nat_eq(a3250, a3251) {
        let a3254 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3252, _, _, _, _) => {
                h3252
            }
        };
        let a3255 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3253, _, _, _, _, _) => {
                h3253
            }
        };
        nat_lt(a3254, a3255)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3256 = v3;
        match m3256 {
            None => {
                f192(v0, v1, match v2.clone() {
                    Adt2::C0(h3257, _, _, _, _, _, _, _, _, _, _) => {
                        h3257
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3258, _, _, _, _, _, _, _, _, _, _) => {
                        h3258
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3259, _, _, _, _, _, _, _, _, _) => {
                        h3259
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3260, _, _, _, _, _, _, _, _) => {
                        h3260
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3261, _, _, _, _, _, _, _) => {
                        h3261
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3262, _, _, _, _, _, _) => {
                        h3262
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3263, _, _, _, _, _, _, _, _, _) => {
                        h3263
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3264, _, _, _, _, _, _, _, _) => {
                        h3264
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3265, _, _, _, _, _, _, _) => {
                        h3265
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3266, _, _, _, _, _, _) => {
                        h3266
                    }
                })
            }
            Some(v5) => {
                f192(v0, v1, match v2.clone() {
                    Adt2::C0(h3267, _, _, _, _, _, _, _, _, _, _) => {
                        h3267
                    }
                }, match v5.clone() {
                    Adt2::C0(h3268, _, _, _, _, _, _, _, _, _, _) => {
                        h3268
                    }
                }, match v4.clone() {
                    Adt2::C0(h3269, _, _, _, _, _, _, _, _, _, _) => {
                        h3269
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3270, _, _, _, _, _, _, _, _, _) => {
                        h3270
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3271, _, _, _, _, _, _, _, _) => {
                        h3271
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3272, _, _, _, _, _, _, _) => {
                        h3272
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3273, _, _, _, _, _, _) => {
                        h3273
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3274, _, _, _, _, _, _, _, _, _) => {
                        h3274
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3275, _, _, _, _, _, _, _, _) => {
                        h3275
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3276, _, _, _, _, _, _, _) => {
                        h3276
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3277, _, _, _, _, _, _) => {
                        h3277
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3278, _, _, _, _, _, _, _, _, _) => {
                        h3278
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3279, _, _, _, _, _, _, _, _) => {
                        h3279
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3280, _, _, _, _, _, _, _) => {
                        h3280
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3281, _, _, _, _, _, _) => {
                        h3281
                    }
                })
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3294 = {
        let a3290 = {
            let a3286 = {
                let a3282 = v0;
                let a3283 = v4;
                nat_mul(a3282, a3283)?
            };
            let a3287 = {
                let a3284 = v1;
                let a3285 = v5;
                nat_mul(a3284, a3285)?
            };
            nat_add(a3286, a3287)?
        };
        let a3291 = {
            let a3288 = v2;
            let a3289 = v6;
            nat_mul(a3288, a3289)?
        };
        nat_add(a3290, a3291)?
    };
    let a3295 = {
        let a3292 = v3;
        let a3293 = v7;
        nat_mul(a3292, a3293)?
    };
    nat_add(a3294, a3295)
}

pub fn f127(v0: u64) -> u64 {
    let a3296 = v0;
    let a3297 = 2147483648u64;
    if nat_lt(a3296, a3297) {
        0u64
    } else {
        let a3298 = 4294967296u64;
        let a3299 = v0;
        nat_sub(a3298, a3299)
    }
}

pub fn f128(v0: u64) -> u64 {
    let a3300 = v0;
    let a3301 = 2147483648u64;
    if nat_lt(a3300, a3301) {
        v0
    } else {
        0u64
    }
}

pub fn f129(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3302 = v7;
        let a3303 = 6u64;
        if nat_eq(a3302, a3303) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f86(v3, v4), f86(v5, v6), f87(v7, v8), f193(v1), f193(v9))
}

pub fn f130(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3304 = v0;
    match m3304 {
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
            let a3305 = 13u64;
            let a3306 = v36;
            if nat_le(a3305, a3306) {
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
                Adt5::C0(_, _, _, _, _, h3307, _, _) => {
                    h3307
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3308) => {
                    h3308
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

pub fn f131(v0: Adt4) -> u64 {
    let m3309 = v0;
    match m3309 {
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

pub fn f132(v0: Adt4, v1: u64) -> u64 {
    let m3310 = v0;
    match m3310 {
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
            let a3311 = v1;
            let a3312 = 6u64;
            if nat_eq(a3311, a3312) {
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
            let a3321 = v1;
            let a3322 = 6u64;
            if if if nat_eq(a3321, a3322) {
                true
            } else {
                let a3323 = v1;
                let a3324 = 7u64;
                nat_eq(a3323, a3324)
            } {
                let a3325 = v56;
                let a3326 = 30u64;
                nat_eq(a3325, a3326)
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
                Adt5::C0(_, _, _, _, _, _, _, h3320) => {
                    h3320
                }
            }
        }
        Adt4::C28(v51) => {
            let a3313 = v1;
            let a3314 = 6u64;
            if nat_eq(a3313, a3314) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3315) => {
                        h3315
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
            let a3316 = v1;
            let a3317 = 6u64;
            if if nat_eq(a3316, a3317) {
                true
            } else {
                let a3318 = v1;
                let a3319 = 7u64;
                nat_eq(a3318, a3319)
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

pub fn f133(v0: Adt4, v1: u64) -> Adt11 {
    let a3327 = v1;
    let a3328 = 4u64;
    if nat_lt(a3327, a3328) {
        Adt11::C0(v1)
    } else {
        let a3329 = v1;
        let a3330 = 4u64;
        if nat_eq(a3329, a3330) {
            Adt11::C1(0u64)
        } else {
            let a3331 = v1;
            let a3332 = 5u64;
            if nat_eq(a3331, a3332) {
                Adt11::C1(3u64)
            } else {
                let a3333 = v1;
                let a3334 = 6u64;
                if nat_eq(a3333, a3334) {
                    Adt11::C2(f132(v0, v1), 0u64, false)
                } else {
                    let a3335 = v1;
                    let a3336 = 7u64;
                    if nat_eq(a3335, a3336) {
                        Adt11::C2(0u64, f132(v0, v1), false)
                    } else {
                        let a3337 = v1;
                        let a3338 = 16u64;
                        if nat_lt(a3337, a3338) {
                            Adt11::C1({
                                let a3339 = v1;
                                let a3340 = 8u64;
                                nat_sub(a3339, a3340)
                            })
                        } else {
                            let a3341 = v1;
                            let a3342 = 20u64;
                            if nat_lt(a3341, a3342) {
                                Adt11::C5({
                                    let a3343 = v1;
                                    let a3344 = 16u64;
                                    nat_sub(a3343, a3344)
                                })
                            } else {
                                let a3345 = v1;
                                let a3346 = 20u64;
                                if nat_eq(a3345, a3346) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3347 = v1;
                                    let a3348 = 21u64;
                                    if nat_eq(a3347, a3348) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3349 = v1;
                                        let a3350 = 22u64;
                                        if nat_eq(a3349, a3350) {
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

pub fn f134(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3352 = v0;
        let a3353 = match v1.clone() {
            Adt3::C0(h3351, _, _, _, _, _, _, _) => {
                h3351
            }
        };
        let a3354 = 0u64;
        nat_quot(a3352, a3353, a3354)
    };
    let v3: u64 = {
        let a3356 = v2;
        let a3357 = match v1.clone() {
            Adt3::C0(_, h3355, _, _, _, _, _, _) => {
                h3355
            }
        };
        let a3358 = 0u64;
        nat_quot(a3356, a3357, a3358)
    };
    f126({
        let a3360 = v0;
        let a3361 = match v1.clone() {
            Adt3::C0(h3359, _, _, _, _, _, _, _) => {
                h3359
            }
        };
        let a3362 = 0u64;
        nat_rem(a3360, a3361, a3362)
    }, {
        let a3364 = v2;
        let a3365 = match v1.clone() {
            Adt3::C0(_, h3363, _, _, _, _, _, _) => {
                h3363
            }
        };
        let a3366 = 0u64;
        nat_rem(a3364, a3365, a3366)
    }, {
        let a3368 = v3;
        let a3369 = match v1.clone() {
            Adt3::C0(_, _, h3367, _, _, _, _, _) => {
                h3367
            }
        };
        let a3370 = 0u64;
        nat_rem(a3368, a3369, a3370)
    }, {
        let a3372 = v3;
        let a3373 = match v1.clone() {
            Adt3::C0(_, _, h3371, _, _, _, _, _) => {
                h3371
            }
        };
        let a3374 = 0u64;
        nat_quot(a3372, a3373, a3374)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3375, _, _, _) => {
            h3375
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3376, _, _) => {
            h3376
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3377, _) => {
            h3377
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3378) => {
            h3378
        }
    })
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3396 = {
        let a3391 = f194(v2, {
            let a3382 = {
                let a3379 = v0;
                let a3380 = v1;
                let a3381 = 0u64;
                nat_quot(a3379, a3380, a3381)
            };
            let a3383 = v2;
            let a3384 = 0u64;
            nat_rem(a3382, a3383, a3384)
        }, {
            let a3388 = {
                let a3385 = v0;
                let a3386 = v1;
                let a3387 = 0u64;
                nat_quot(a3385, a3386, a3387)
            };
            let a3389 = v2;
            let a3390 = 0u64;
            nat_quot(a3388, a3389, a3390)
        })?;
        let a3392 = v1;
        nat_mul(a3391, a3392)?
    };
    let a3397 = {
        let a3393 = v0;
        let a3394 = v1;
        let a3395 = 0u64;
        nat_rem(a3393, a3394, a3395)
    };
    nat_add(a3396, a3397)
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3399 = v0;
        let a3400 = match v2.clone() {
            Adt3::C0(h3398, _, _, _, _, _, _, _) => {
                h3398
            }
        };
        let a3401 = 0u64;
        nat_quot(a3399, a3400, a3401)
    };
    let v4: u64 = {
        let a3403 = v3;
        let a3404 = match v2.clone() {
            Adt3::C0(_, h3402, _, _, _, _, _, _) => {
                h3402
            }
        };
        let a3405 = 0u64;
        nat_quot(a3403, a3404, a3405)
    };
    f195({
        let a3407 = v0;
        let a3408 = match v2.clone() {
            Adt3::C0(h3406, _, _, _, _, _, _, _) => {
                h3406
            }
        };
        let a3409 = 0u64;
        nat_rem(a3407, a3408, a3409)
    }, {
        let a3411 = v3;
        let a3412 = match v2.clone() {
            Adt3::C0(_, h3410, _, _, _, _, _, _) => {
                h3410
            }
        };
        let a3413 = 0u64;
        nat_rem(a3411, a3412, a3413)
    }, {
        let a3415 = v4;
        let a3416 = match v2.clone() {
            Adt3::C0(_, _, h3414, _, _, _, _, _) => {
                h3414
            }
        };
        let a3417 = 0u64;
        nat_rem(a3415, a3416, a3417)
    }, {
        let a3419 = v4;
        let a3420 = match v2 {
            Adt3::C0(_, _, h3418, _, _, _, _, _) => {
                h3418
            }
        };
        let a3421 = 0u64;
        nat_quot(a3419, a3420, a3421)
    }, match v1.clone() {
        Adt3::C0(h3422, _, _, _, _, _, _, _) => {
            h3422
        }
    }, match v1.clone() {
        Adt3::C0(_, h3423, _, _, _, _, _, _) => {
            h3423
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3424, _, _, _, _, _) => {
            h3424
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3425, _, _, _, _) => {
            h3425
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3426, _, _, _) => {
            h3426
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3427, _, _) => {
            h3427
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3428, _) => {
            h3428
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3429) => {
            h3429
        }
    })
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3431 = v0;
        let a3432 = match v3.clone() {
            Adt3::C0(h3430, _, _, _, _, _, _, _) => {
                h3430
            }
        };
        let a3433 = 0u64;
        nat_quot(a3431, a3432, a3433)
    };
    let v6: u64 = {
        let a3435 = v5;
        let a3436 = match v3.clone() {
            Adt3::C0(_, h3434, _, _, _, _, _, _) => {
                h3434
            }
        };
        let a3437 = 0u64;
        nat_quot(a3435, a3436, a3437)
    };
    let v7: u64 = {
        let a3439 = v0;
        let a3440 = match v3.clone() {
            Adt3::C0(h3438, _, _, _, _, _, _, _) => {
                h3438
            }
        };
        let a3441 = 0u64;
        nat_rem(a3439, a3440, a3441)
    };
    let v8: u64 = {
        let a3443 = v5;
        let a3444 = match v3.clone() {
            Adt3::C0(_, h3442, _, _, _, _, _, _) => {
                h3442
            }
        };
        let a3445 = 0u64;
        nat_rem(a3443, a3444, a3445)
    };
    let v9: u64 = {
        let a3447 = v6;
        let a3448 = match v3.clone() {
            Adt3::C0(_, _, h3446, _, _, _, _, _) => {
                h3446
            }
        };
        let a3449 = 0u64;
        nat_rem(a3447, a3448, a3449)
    };
    let v10: u64 = {
        let a3451 = v6;
        let a3452 = match v3.clone() {
            Adt3::C0(_, _, h3450, _, _, _, _, _) => {
                h3450
            }
        };
        let a3453 = 0u64;
        nat_quot(a3451, a3452, a3453)
    };
    let v11: u64 = {
        let a3454 = v4;
        let a3455 = 0u64;
        if nat_eq(a3454, a3455) {
            match v1.clone() {
                Adt3::C0(h3456, _, _, _, _, _, _, _) => {
                    h3456
                }
            }
        } else {
            let a3457 = v4;
            let a3458 = 1u64;
            if nat_eq(a3457, a3458) {
                match v1.clone() {
                    Adt3::C0(_, h3459, _, _, _, _, _, _) => {
                        h3459
                    }
                }
            } else {
                let a3460 = v4;
                let a3461 = 2u64;
                if nat_eq(a3460, a3461) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3462, _, _, _, _, _) => {
                            h3462
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3463, _, _, _, _) => {
                            h3463
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3470 = v11;
        let a3471 = {
            let a3464 = v4;
            let a3465 = 0u64;
            if nat_eq(a3464, a3465) {
                v7
            } else {
                let a3466 = v4;
                let a3467 = 1u64;
                if nat_eq(a3466, a3467) {
                    v8
                } else {
                    let a3468 = v4;
                    let a3469 = 2u64;
                    if nat_eq(a3468, a3469) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3470, a3471)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f126(if if v12 {
        let a3472 = v4;
        let a3473 = 0u64;
        nat_eq(a3472, a3473)
    } else {
        false
    } {
        let a3474 = v7;
        let a3475 = v11;
        nat_sub(a3474, a3475)
    } else {
        v7
    }, if if v12 {
        let a3476 = v4;
        let a3477 = 1u64;
        nat_eq(a3476, a3477)
    } else {
        false
    } {
        let a3478 = v8;
        let a3479 = v11;
        nat_sub(a3478, a3479)
    } else {
        v8
    }, if if v12 {
        let a3480 = v4;
        let a3481 = 2u64;
        nat_eq(a3480, a3481)
    } else {
        false
    } {
        let a3482 = v9;
        let a3483 = v11;
        nat_sub(a3482, a3483)
    } else {
        v9
    }, if if v12 {
        let a3484 = v4;
        let a3485 = 3u64;
        nat_eq(a3484, a3485)
    } else {
        false
    } {
        let a3486 = v10;
        let a3487 = v11;
        nat_sub(a3486, a3487)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3488, _, _, _) => {
            h3488
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3489, _, _) => {
            h3489
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3490, _) => {
            h3490
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3491) => {
            h3491
        }
    })?, f134(v0, v3)?))
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f67(v0, v2.clone(), 0u64);
    let v9: u64 = f67(v0, v2.clone(), 1u64);
    let v10: u64 = f67(v0, v2.clone(), 2u64);
    let v11: u64 = f67(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f66(v1.clone(), f196(v8, v3, match v1.clone() {
            Adt3::C0(h3492, _, _, _, _, _, _, _) => {
                h3492
            }
        })?, f196(v9, v4, match v1.clone() {
            Adt3::C0(_, h3493, _, _, _, _, _, _) => {
                h3493
            }
        })?, f196(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3494, _, _, _, _, _) => {
                h3494
            }
        })?, f196(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3495, _, _, _, _) => {
                h3495
            }
        })?)?, f134(v0, v2)?))
    } else {
        if if if if f197(v8, v3, match v1.clone() {
            Adt3::C0(h3496, _, _, _, _, _, _, _) => {
                h3496
            }
        })? {
            f197(v9, v4, match v1.clone() {
                Adt3::C0(_, h3497, _, _, _, _, _, _) => {
                    h3497
                }
            })?
        } else {
            false
        } {
            f197(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3498, _, _, _, _, _) => {
                    h3498
                }
            })?
        } else {
            false
        } {
            f197(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3499, _, _, _, _) => {
                    h3499
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f66(v1, f198(v8, v3)?, f198(v9, v4)?, f198(v10, v5)?, f198(v11, v6)?)?, f134(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f134(v0, v2)?))
        }
    }
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f67(v0, v2.clone(), 0u64);
    let v5: u64 = f67(v0, v2.clone(), 1u64);
    let v6: u64 = f67(v0, v2.clone(), 2u64);
    let v7: u64 = f67(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f66(v1.clone(), f199(v4, v3, match v1 {
        Adt3::C0(h3500, _, _, _, _, _, _, _) => {
            h3500
        }
    })?, v5, v6, v7)?, f134(v0, v2)?))
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3501 = v0;
    let a3502 = v3;
    if nat_lt(a3501, a3502) {
        Ok(Adt9::C1(false, {
            let a3503 = v0;
            let a3504 = 4u64;
            nat_mul(a3503, a3504)?
        }, {
            let a3505 = v0;
            let a3506 = 4u64;
            nat_mul(a3505, a3506)?
        }))
    } else {
        let v6: u64 = {
            let a3507 = v0;
            let a3508 = v3;
            nat_sub(a3507, a3508)
        };
        let v7: u64 = f134(v6, v1)?;
        let v8: u64 = f200(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3509 = v0;
    let a3510 = v3;
    if nat_lt(a3509, a3510) {
        Ok(Adt9::C1(true, 0u64, f134(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3511 = v0;
            let a3512 = v3;
            nat_sub(a3511, a3512)
        };
        let v5: u64 = f136(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f134(v4, v1)?, 3u64, v5))
    }
}

pub fn f142(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3514 = v1;
        let a3515 = match v3.clone() {
            Adt3::C0(h3513, _, _, _, _, _, _, _) => {
                h3513
            }
        };
        let a3516 = 0u64;
        nat_rem(a3514, a3515, a3516)
    };
    let v5: u64 = {
        let a3522 = {
            let a3518 = v1;
            let a3519 = match v3.clone() {
                Adt3::C0(h3517, _, _, _, _, _, _, _) => {
                    h3517
                }
            };
            let a3520 = 0u64;
            nat_quot(a3518, a3519, a3520)
        };
        let a3523 = match v3.clone() {
            Adt3::C0(_, h3521, _, _, _, _, _, _) => {
                h3521
            }
        };
        let a3524 = 0u64;
        nat_rem(a3522, a3523, a3524)
    };
    let v6: bool = f201(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f136(v1, v2, v3.clone())?
    } else {
        0u64
    }, f134(v1, v3)?))
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3525 = v3;
        let a3526 = 8u64;
        nat_add(a3525, a3526)?
    }, 0u64, f134(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f136(v0, v5, v2.clone())?
    }, 3u64, f134(v0, v2)?))
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3527 = v4;
    let a3528 = 0u64;
    if if nat_eq(a3527, a3528) {
        f47(v3)
    } else {
        false
    } {
        let m3529 = f110(v3);
        match m3529 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3530, _) => {
                        h3530
                    }
                };
                Ok(Adt9::C5(v3, f134(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3531) => {
                        h3531
                    }
                }, f134({
                    let a3532 = v0;
                    let a3533 = v6;
                    nat_mul(a3532, a3533)?
                }, v2.clone())?, {
                    let a3539 = {
                        let a3537 = {
                            let a3534 = v6;
                            let a3535 = 1u64;
                            nat_sub(a3534, a3535)
                        };
                        let a3538 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3536, _, _, _) => {
                                h3536
                            }
                        };
                        nat_mul(a3537, a3538)?
                    };
                    let a3540 = 4u64;
                    nat_add(a3539, a3540)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3541, _, _, _) => {
                        h3541
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f134(v0, v1)?, f134(v0, v2)?))
    }
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3543 = v0;
        let a3544 = match v2.clone() {
            Adt3::C0(h3542, _, _, _, _, _, _, _) => {
                h3542
            }
        };
        let a3545 = 0u64;
        nat_rem(a3543, a3544, a3545)
    };
    let v7: u64 = {
        let a3547 = v0;
        let a3548 = match v2.clone() {
            Adt3::C0(h3546, _, _, _, _, _, _, _) => {
                h3546
            }
        };
        let a3549 = 0u64;
        nat_quot(a3547, a3548, a3549)
    };
    let v8: u64 = {
        let a3551 = v7;
        let a3552 = match v2.clone() {
            Adt3::C0(_, h3550, _, _, _, _, _, _) => {
                h3550
            }
        };
        let a3553 = 0u64;
        nat_rem(a3551, a3552, a3553)
    };
    let v9: u64 = {
        let a3555 = v7;
        let a3556 = match v2.clone() {
            Adt3::C0(_, h3554, _, _, _, _, _, _) => {
                h3554
            }
        };
        let a3557 = 0u64;
        nat_quot(a3555, a3556, a3557)
    };
    let v10: u64 = {
        let a3559 = v9;
        let a3560 = match v2.clone() {
            Adt3::C0(_, _, h3558, _, _, _, _, _) => {
                h3558
            }
        };
        let a3561 = 0u64;
        nat_rem(a3559, a3560, a3561)
    };
    let v11: u64 = {
        let a3563 = v9;
        let a3564 = match v2.clone() {
            Adt3::C0(_, _, h3562, _, _, _, _, _) => {
                h3562
            }
        };
        let a3565 = 0u64;
        nat_quot(a3563, a3564, a3565)
    };
    let v12: u64 = f134(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3574 = {
                let a3569 = {
                    let a3566 = v10;
                    let a3567 = v4;
                    let a3568 = 0u64;
                    nat_quot(a3566, a3567, a3568)
                };
                let a3570 = v5;
                nat_mul(a3569, a3570)?
            };
            let a3575 = {
                let a3571 = v8;
                let a3572 = v4;
                let a3573 = 0u64;
                nat_quot(a3571, a3572, a3573)
            };
            nat_add(a3574, a3575)?
        };
        let v14: u64 = {
            let a3595 = v6;
            let a3596 = {
                let a3593 = match v1.clone() {
                    Adt3::C0(h3576, _, _, _, _, _, _, _) => {
                        h3576
                    }
                };
                let a3594 = {
                    let a3591 = {
                        let a3577 = v8;
                        let a3578 = v4;
                        let a3579 = 0u64;
                        nat_rem(a3577, a3578, a3579)
                    };
                    let a3592 = {
                        let a3589 = match v1.clone() {
                            Adt3::C0(_, h3580, _, _, _, _, _, _) => {
                                h3580
                            }
                        };
                        let a3590 = {
                            let a3587 = {
                                let a3581 = v10;
                                let a3582 = v4;
                                let a3583 = 0u64;
                                nat_rem(a3581, a3582, a3583)
                            };
                            let a3588 = {
                                let a3585 = match v1.clone() {
                                    Adt3::C0(_, _, h3584, _, _, _, _, _) => {
                                        h3584
                                    }
                                };
                                let a3586 = v13;
                                nat_mul(a3585, a3586)?
                            };
                            nat_add(a3587, a3588)?
                        };
                        nat_mul(a3589, a3590)?
                    };
                    nat_add(a3591, a3592)?
                };
                nat_mul(a3593, a3594)?
            };
            nat_add(a3595, a3596)?
        };
        Ok(Adt9::C1(false, f134(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3602 = {
                let a3600 = {
                    let a3597 = v11;
                    let a3598 = v5;
                    let a3599 = 0u64;
                    nat_rem(a3597, a3598, a3599)
                };
                let a3601 = v4;
                nat_mul(a3600, a3601)?
            };
            let a3603 = v8;
            nat_add(a3602, a3603)?
        };
        let v16: u64 = {
            let a3609 = {
                let a3607 = {
                    let a3604 = v11;
                    let a3605 = v5;
                    let a3606 = 0u64;
                    nat_quot(a3604, a3605, a3606)
                };
                let a3608 = v4;
                nat_mul(a3607, a3608)?
            };
            let a3610 = v10;
            nat_add(a3609, a3610)?
        };
        let a3612 = v15;
        let a3613 = match v1.clone() {
            Adt3::C0(_, h3611, _, _, _, _, _, _) => {
                h3611
            }
        };
        if if nat_lt(a3612, a3613) {
            let a3615 = v16;
            let a3616 = match v1.clone() {
                Adt3::C0(_, _, h3614, _, _, _, _, _) => {
                    h3614
                }
            };
            nat_lt(a3615, a3616)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f134({
                let a3625 = v6;
                let a3626 = {
                    let a3623 = match v1.clone() {
                        Adt3::C0(h3617, _, _, _, _, _, _, _) => {
                            h3617
                        }
                    };
                    let a3624 = {
                        let a3621 = v15;
                        let a3622 = {
                            let a3619 = match v1.clone() {
                                Adt3::C0(_, h3618, _, _, _, _, _, _) => {
                                    h3618
                                }
                            };
                            let a3620 = v16;
                            nat_mul(a3619, a3620)?
                        };
                        nat_add(a3621, a3622)?
                    };
                    nat_mul(a3623, a3624)?
                };
                nat_add(a3625, a3626)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f134(v0, v2.clone())?;
    let v4: u64 = f134(v0, v1.clone())?;
    let a3631 = {
        let a3628 = v0;
        let a3629 = match v1 {
            Adt3::C0(h3627, _, _, _, _, _, _, _) => {
                h3627
            }
        };
        let a3630 = 0u64;
        nat_rem(a3628, a3629, a3630)
    };
    let a3632 = 0u64;
    if nat_eq(a3631, a3632) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f134({
            let a3633 = v0;
            let a3634 = 1u64;
            nat_sub(a3633, a3634)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3635 = v3;
    let a3636 = 13u64;
    if nat_eq(a3635, a3636) {
        f202(v0, v1, v4)
    } else {
        let v5: u64 = f191(match v1.clone() {
            Adt3::C0(h3637, _, _, _, _, _, _, _) => {
                h3637
            }
        }, {
            let a3638 = v3;
            let a3639 = 16u64;
            nat_eq(a3638, a3639)
        })?;
        let v6: u64 = {
            let a3640 = v0;
            let a3641 = v5;
            let a3642 = 0u64;
            nat_quot(a3640, a3641, a3642)
        };
        let v7: u64 = {
            let a3643 = v0;
            let a3644 = v5;
            let a3645 = 0u64;
            nat_rem(a3643, a3644, a3645)
        };
        let v8: u64 = {
            let a3649 = 1u64;
            let a3650 = {
                let a3647 = match v1.clone() {
                    Adt3::C0(h3646, _, _, _, _, _, _, _) => {
                        h3646
                    }
                };
                let a3648 = 2u64;
                nat_mul(a3647, a3648)?
            };
            nat_add(a3649, a3650)?
        };
        let a3651 = v7;
        let a3652 = v8;
        if nat_lt(a3651, a3652) {
            f203(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f134(v6, v2)?;
            let a3653 = v3;
            let a3654 = 14u64;
            if nat_eq(a3653, a3654) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3655 = v7;
                let a3656 = v8;
                if nat_eq(a3655, a3656) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3659 = v7;
                    let a3660 = {
                        let a3657 = v8;
                        let a3658 = 1u64;
                        nat_add(a3657, a3658)?
                    };
                    if nat_eq(a3659, a3660) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3661, _, _, _, _, _, _, _) => {
                                h3661
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

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3662 = v0;
    let a3663 = 0u64;
    if nat_eq(a3662, a3663) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3670 = {
            let a3664 = v0;
            let a3665 = 1u64;
            nat_add(a3664, a3665)?
        };
        let a3671 = {
            let a3668 = 2u64;
            let a3669 = {
                let a3666 = v3;
                let a3667 = 2u64;
                nat_mul(a3666, a3667)?
            };
            nat_add(a3668, a3669)?
        };
        if nat_eq(a3670, a3671) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3672 = v0;
                let a3673 = 1u64;
                nat_sub(a3672, a3673)
            };
            let v5: u64 = {
                let a3674 = v4;
                let a3675 = 2u64;
                let a3676 = 0u64;
                nat_quot(a3674, a3675, a3676)
            };
            let a3680 = {
                let a3677 = v4;
                let a3678 = 2u64;
                let a3679 = 0u64;
                nat_rem(a3677, a3678, a3679)
            };
            let a3681 = 0u64;
            if nat_eq(a3680, a3681) {
                Ok(Adt9::C4(17u64, 0u64, f134(v5, v1)?, 4u64, f134(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f149(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3682 = v0;
    let a3683 = f24(v1, v2, v3)?;
    if nat_lt(a3682, a3683) {
        let a3684 = v0;
        let a3685 = v1;
        if nat_lt(a3684, a3685) {
            Ok(Adt9::C1(false, {
                let a3686 = 4u64;
                let a3687 = v0;
                nat_mul(a3686, a3687)?
            }, {
                let a3688 = 4u64;
                let a3689 = v0;
                nat_mul(a3688, a3689)?
            }))
        } else {
            let v4: u64 = {
                let a3690 = v0;
                let a3691 = v1;
                nat_sub(a3690, a3691)
            };
            let v5: u64 = f204(v4, v2)?;
            let a3692 = v5;
            let a3693 = 2u64;
            if nat_lt(a3692, a3693) {
                Ok(Adt9::C4(6u64, {
                    let a3694 = v5;
                    let a3695 = 0u64;
                    if nat_eq(a3694, a3695) {
                        4u64
                    } else {
                        5u64
                    }
                }, f205(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f206(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f207(v4, v2)? {
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

pub fn f150(v0: u64, v1: u64) -> Adt11 {
    let a3696 = v0;
    let a3697 = 43u64;
    if nat_eq(a3696, a3697) {
        Adt11::C8(3u64)
    } else {
        let a3698 = v1;
        let a3699 = 43u64;
        if nat_eq(a3698, a3699) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f151(v0: u64, v1: u64) -> R<u64> {
    if f100(v0) {
        let a3700 = 2u64;
        let a3701 = v1;
        nat_mul(a3700, a3701)
    } else {
        let a3702 = v1;
        let a3703 = f187(v0);
        let a3704 = 0u64;
        Ok(nat_quot(a3702, a3703, a3704))
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m3705 = f110(v0);
    match m3705 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f126(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h3706, _, _, _) => {
                    h3706
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h3707, _, _) => {
                    h3707
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h3708, _) => {
                    h3708
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h3709) => {
                    h3709
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h3710) => {
                    h3710
                }
            }, f126({
                let a3712 = v2;
                let a3713 = match v6.clone() {
                    Adt18::C0(_, h3711, _) => {
                        h3711
                    }
                };
                nat_mul(a3712, a3713)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h3714, _, _, _) => {
                    h3714
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h3715, _, _) => {
                    h3715
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h3716, _) => {
                    h3716
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h3717) => {
                    h3717
                }
            })?, match v6 {
                Adt18::C0(_, h3718, _) => {
                    h3718
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h3719, _, _, _) => {
                    h3719
                }
            }))
        }
    }
}

pub fn f153(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3727 = {
        let a3720 = 2u64;
        let a3721 = f123(v1.clone())?;
        nat_mul(a3720, a3721)?
    };
    let a3728 = {
        let a3725 = f123(v1)?;
        let a3726 = {
            let a3723 = 1u64;
            let a3724 = match v0 {
                Adt3::C0(h3722, _, _, _, _, _, _, _) => {
                    h3722
                }
            };
            nat_add(a3723, a3724)?
        };
        nat_mul(a3725, a3726)?
    };
    nat_add(a3727, a3728)
}

pub fn f154(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f134(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f155(v0: u64, v1: u64) -> bool {
    let a3729 = v1;
    let a3730 = 2147483647u64;
    if nat_le(a3729, a3730) {
        let a3731 = v1;
        let a3732 = v0;
        nat_lt(a3731, a3732)
    } else {
        false
    }
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3733 = f110(v0);
    match m3733 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a3735 = v2;
                let a3736 = match v7.clone() {
                    Adt18::C0(_, h3734, _) => {
                        h3734
                    }
                };
                nat_mul(a3735, a3736)?
            };
            let m3749 = f208(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3737, _, _, _, _, _, _, _) => {
                    h3737
                }
            }, match v5.clone() {
                Adt3::C0(_, h3738, _, _, _, _, _, _) => {
                    h3738
                }
            }, match v5 {
                Adt3::C0(_, _, h3739, _, _, _, _, _) => {
                    h3739
                }
            }, {
                let a3742 = match v3.clone() {
                    Adt3::C0(h3740, _, _, _, _, _, _, _) => {
                        h3740
                    }
                };
                let a3743 = match v7.clone() {
                    Adt18::C0(_, h3741, _) => {
                        h3741
                    }
                };
                nat_mul(a3742, a3743)?
            }, match v3.clone() {
                Adt3::C0(_, h3744, _, _, _, _, _, _) => {
                    h3744
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3745, _, _, _) => {
                    h3745
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3746, _, _) => {
                    h3746
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3747, _) => {
                    h3747
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3748) => {
                    h3748
                }
            })?;
            match m3749 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3750 = v0;
                    let a3751 = 0u64;
                    if if nat_eq(a3750, a3751) {
                        true
                    } else {
                        let a3752 = v0;
                        let a3753 = 26u64;
                        nat_eq(a3752, a3753)
                    } {
                        Ok(Adt9::C1(false, v9, f134({
                            let a3757 = {
                                let a3755 = v1;
                                let a3756 = match v4.clone() {
                                    Adt3::C0(h3754, _, _, _, _, _, _, _) => {
                                        h3754
                                    }
                                };
                                nat_mul(a3755, a3756)?
                            };
                            let a3758 = v8;
                            nat_add(a3757, a3758)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h3759) => {
                                h3759
                            }
                        }, f134({
                            let a3763 = {
                                let a3761 = v1;
                                let a3762 = match v4.clone() {
                                    Adt3::C0(h3760, _, _, _, _, _, _, _) => {
                                        h3760
                                    }
                                };
                                nat_mul(a3761, a3762)?
                            };
                            let a3764 = v8;
                            nat_add(a3763, a3764)?
                        }, v4.clone())?, {
                            let a3771 = {
                                let a3769 = {
                                    let a3766 = match v7 {
                                        Adt18::C0(_, h3765, _) => {
                                            h3765
                                        }
                                    };
                                    let a3767 = 1u64;
                                    nat_sub(a3766, a3767)
                                };
                                let a3770 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3768, _, _, _) => {
                                        h3768
                                    }
                                };
                                nat_mul(a3769, a3770)?
                            };
                            let a3772 = 4u64;
                            nat_add(a3771, a3772)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3773, _, _, _) => {
                                h3773
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f157(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3774 = v1;
    let a3775 = 0u64;
    if if nat_eq(a3774, a3775) {
        true
    } else {
        let a3776 = v2;
        let a3777 = 0u64;
        nat_eq(a3776, a3777)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f158(v0: u64, v1: u64) -> bool {
    let a3778 = v0;
    let a3779 = 4294967295u64;
    if nat_eq(a3778, a3779) {
        true
    } else {
        let a3780 = v0;
        let a3781 = 2147483648u64;
        if nat_lt(a3780, a3781) {
            let a3782 = v0;
            let a3783 = v1;
            nat_lt(a3782, a3783)
        } else {
            false
        }
    }
}

pub fn f159(v0: Adt7) -> R<u64> {
    let a3788 = if match v0.clone() {
        Adt7::C0(_, _, _, h3784, _) => {
            h3784
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3789 = {
        let a3786 = match v0 {
            Adt7::C0(_, _, _, _, h3785) => {
                h3785
            }
        };
        let a3787 = 1u64;
        if nat_eq(a3786, a3787) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3788, a3789)
}

pub fn f160(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3794 = {
            let a3791 = v2;
            let a3792 = match v0.clone() {
                Adt6::C0(_, h3790, _, _) => {
                    h3790
                }
            };
            nat_mul(a3791, a3792)?
        };
        let a3795 = f127(match v0.clone() {
            Adt6::C0(_, _, h3793, _) => {
                h3793
            }
        });
        nat_add(a3794, a3795)?
    };
    let v4: u64 = f128(match v0.clone() {
        Adt6::C0(_, _, h3796, _) => {
            h3796
        }
    });
    let v5: u64 = {
        let a3800 = {
            let a3798 = v3;
            let a3799 = match v0 {
                Adt6::C0(h3797, _, _, _) => {
                    h3797
                }
            };
            nat_add(a3798, a3799)?
        };
        let a3801 = v4;
        nat_sub(a3800, a3801)
    };
    let a3806 = {
        let a3802 = v5;
        let a3803 = v1;
        if nat_lt(a3802, a3803) {
            v5
        } else {
            v1
        }
    };
    let a3807 = {
        let a3804 = v3;
        let a3805 = v4;
        nat_sub(a3804, a3805)
    };
    Ok(nat_sub(a3806, a3807))
}

pub fn f161(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3816 = f69(match v0.clone() {
        Adt7::C0(h3808, _, _, _, _) => {
            h3808
        }
    }, match v1.clone() {
        Adt3::C0(h3809, _, _, _, _, _, _, _) => {
            h3809
        }
    }, f67(v3, v2.clone(), 0u64), {
        let a3813 = v4;
        let a3814 = {
            let m3812 = match v0.clone() {
                Adt7::C0(h3811, _, _, _, _) => {
                    h3811
                }
            };
            match m3812 {
                Adt6::C0(h3810, _, _, _) => {
                    h3810
                }
            }
        };
        let a3815 = 0u64;
        nat_rem(a3813, a3814, a3815)
    })?;
    match m3816 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3825 = f69(match v0.clone() {
                Adt7::C0(_, h3817, _, _, _) => {
                    h3817
                }
            }, match v1.clone() {
                Adt3::C0(_, h3818, _, _, _, _, _, _) => {
                    h3818
                }
            }, f67(v3, v2.clone(), 1u64), {
                let a3822 = v4;
                let a3823 = {
                    let m3821 = match v0 {
                        Adt7::C0(h3820, _, _, _, _) => {
                            h3820
                        }
                    };
                    match m3821 {
                        Adt6::C0(h3819, _, _, _) => {
                            h3819
                        }
                    }
                };
                let a3824 = 0u64;
                nat_quot(a3822, a3823, a3824)
            })?;
            match m3825 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f66(v1, v5, v6, f67(v3, v2.clone(), 2u64), f67(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f162(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m3826 = v0;
    match m3826 {
        Adt16::C0(v19) => {
            f209(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a3827 = v20;
            let a3828 = 66u64;
            if nat_eq(a3827, a3828) {
                f210(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f211(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3835 = v3;
        let a3836 = {
            let a3832 = f134({
                let a3829 = v0;
                let a3830 = 2u64;
                let a3831 = 0u64;
                nat_quot(a3829, a3830, a3831)
            }, v1)?;
            let a3833 = v2;
            let a3834 = 0u64;
            nat_quot(a3832, a3833, a3834)
        };
        nat_add(a3835, a3836)?
    };
    let a3840 = {
        let a3837 = v0;
        let a3838 = 2u64;
        let a3839 = 0u64;
        nat_rem(a3837, a3838, a3839)
    };
    let a3841 = 0u64;
    if nat_eq(a3840, a3841) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3842 = v4;
        let a3843 = 0u64;
        if nat_eq(a3842, a3843) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f164(v0: u64, v1: u64) -> R<u64> {
    let a3846 = 8u64;
    let a3847 = {
        let a3844 = f212(v0);
        let a3845 = v1;
        nat_mul(a3844, a3845)?
    };
    nat_add(a3846, a3847)
}

pub fn f165(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3848, _, _, _, _, _, _, _) => {
            h3848
        }
    };
    let v7: u64 = f212(v3);
    let v8: u64 = {
        let a3851 = 1u64;
        let a3852 = {
            let a3849 = v6;
            let a3850 = v7;
            nat_mul(a3849, a3850)?
        };
        nat_add(a3851, a3852)?
    };
    let a3853 = v0;
    let a3854 = 0u64;
    if nat_eq(a3853, a3854) {
        Ok(f213(0u64))
    } else {
        let a3855 = v0;
        let a3856 = v8;
        if nat_lt(a3855, a3856) {
            let v9: u64 = {
                let a3857 = v0;
                let a3858 = 1u64;
                nat_sub(a3857, a3858)
            };
            let v10: u64 = {
                let a3859 = v9;
                let a3860 = v7;
                let a3861 = 0u64;
                nat_rem(a3859, a3860, a3861)
            };
            let v11: u64 = f134({
                let a3867 = {
                    let a3862 = v1;
                    let a3863 = v6;
                    nat_mul(a3862, a3863)?
                };
                let a3868 = {
                    let a3864 = v9;
                    let a3865 = v7;
                    let a3866 = 0u64;
                    nat_quot(a3864, a3865, a3866)
                };
                nat_add(a3867, a3868)?
            }, v2)?;
            let a3869 = v3;
            let a3870 = 30u64;
            if nat_eq(a3869, a3870) {
                let a3871 = v10;
                let a3872 = 0u64;
                if nat_eq(a3871, a3872) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3873 = v10;
                    let a3874 = 1u64;
                    if nat_eq(a3873, a3874) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3875 = v10;
                        let a3876 = 2u64;
                        if nat_eq(a3875, a3876) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3877 = v10;
                let a3878 = 0u64;
                if nat_eq(a3877, a3878) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3879 = v10;
                    let a3880 = 1u64;
                    if nat_eq(a3879, a3880) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3881 = v0;
                let a3882 = v8;
                nat_sub(a3881, a3882)
            };
            let a3883 = v12;
            let a3884 = 0u64;
            if nat_eq(a3883, a3884) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3885 = v12;
                let a3886 = 1u64;
                if nat_eq(a3885, a3886) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3887 = v12;
                    let a3888 = 2u64;
                    if nat_eq(a3887, a3888) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3889 = v12;
                        let a3890 = 3u64;
                        if nat_eq(a3889, a3890) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3891 = v12;
                            let a3892 = 4u64;
                            if nat_eq(a3891, a3892) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3893 = v12;
                                let a3894 = 5u64;
                                if nat_eq(a3893, a3894) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3895 = v12;
                                    let a3896 = 6u64;
                                    if nat_eq(a3895, a3896) {
                                        if f214(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3897 = 3u64;
                                                let a3898 = v1;
                                                nat_add(a3897, a3898)?
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

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3899 = v0;
        let a3900 = f98(v3);
        let a3901 = 0u64;
        nat_quot(a3899, a3900, a3901)
    };
    let v5: u64 = {
        let a3906 = 3u64;
        let a3907 = {
            let a3903 = v4;
            let a3904 = match v1.clone() {
                Adt3::C0(h3902, _, _, _, _, _, _, _) => {
                    h3902
                }
            };
            let a3905 = 0u64;
            nat_quot(a3903, a3904, a3905)
        };
        nat_add(a3906, a3907)?
    };
    let v6: u64 = f134(v4, v1)?;
    let v7: u64 = f134(v4, v2)?;
    let a3908 = v3;
    let a3909 = 30u64;
    if nat_eq(a3908, a3909) {
        let a3913 = {
            let a3910 = v0;
            let a3911 = 3u64;
            let a3912 = 0u64;
            nat_rem(a3910, a3911, a3912)
        };
        let a3914 = 0u64;
        if nat_eq(a3913, a3914) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3918 = {
                let a3915 = v0;
                let a3916 = 3u64;
                let a3917 = 0u64;
                nat_rem(a3915, a3916, a3917)
            };
            let a3919 = 1u64;
            if nat_eq(a3918, a3919) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f167(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3920 = v0;
        let a3921 = 8388608u64;
        let a3922 = 0u64;
        nat_quot(a3920, a3921, a3922)
    };
    let a3923 = v0;
    let a3924 = 2147483648u64;
    if if if nat_lt(a3923, a3924) {
        let a3925 = 127u64;
        let a3926 = v1;
        nat_le(a3925, a3926)
    } else {
        false
    } {
        let a3927 = v1;
        let a3928 = 190u64;
        nat_lt(a3927, a3928)
    } else {
        false
    } {
        let v2: u64 = {
            let a3932 = 8388608u64;
            let a3933 = {
                let a3929 = v0;
                let a3930 = 8388608u64;
                let a3931 = 0u64;
                nat_rem(a3929, a3930, a3931)
            };
            nat_add(a3932, a3933)?
        };
        let a3934 = v1;
        let a3935 = 150u64;
        if nat_lt(a3934, a3935) {
            let v3: u64 = f215({
                let a3936 = 150u64;
                let a3937 = v1;
                nat_sub(a3936, a3937)
            });
            let a3941 = {
                let a3938 = v2;
                let a3939 = v3;
                let a3940 = 0u64;
                nat_rem(a3938, a3939, a3940)
            };
            let a3942 = 0u64;
            if nat_eq(a3941, a3942) {
                let a3943 = v2;
                let a3944 = v3;
                let a3945 = 0u64;
                Ok(nat_quot(a3943, a3944, a3945))
            } else {
                Ok(0u64)
            }
        } else {
            let a3948 = f215({
                let a3946 = v1;
                let a3947 = 150u64;
                nat_sub(a3946, a3947)
            });
            let a3949 = v2;
            nat_mul(a3948, a3949)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f168(v0: Adt14, v1: Adt14) -> bool {
    let a3952 = match v0.clone() {
        Adt14::C0(_, h3950, _, _, _, _, _, _, _, _, _) => {
            h3950
        }
    };
    let a3953 = match v1.clone() {
        Adt14::C0(_, h3951, _, _, _, _, _, _, _, _, _) => {
            h3951
        }
    };
    if if if nat_eq(a3952, a3953) {
        let a3956 = match v0.clone() {
            Adt14::C0(_, _, h3954, _, _, _, _, _, _, _, _) => {
                h3954
            }
        };
        let a3957 = match v1.clone() {
            Adt14::C0(_, _, h3955, _, _, _, _, _, _, _, _) => {
                h3955
            }
        };
        nat_eq(a3956, a3957)
    } else {
        false
    } {
        let a3960 = match v0.clone() {
            Adt14::C0(_, _, _, h3958, _, _, _, _, _, _, _) => {
                h3958
            }
        };
        let a3961 = match v1.clone() {
            Adt14::C0(_, _, _, h3959, _, _, _, _, _, _, _) => {
                h3959
            }
        };
        nat_eq(a3960, a3961)
    } else {
        false
    } {
        let a3964 = match v0 {
            Adt14::C0(_, _, _, _, h3962, _, _, _, _, _, _) => {
                h3962
            }
        };
        let a3965 = match v1 {
            Adt14::C0(_, _, _, _, h3963, _, _, _, _, _, _) => {
                h3963
            }
        };
        nat_eq(a3964, a3965)
    } else {
        false
    }
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3966 = v0;
    let a3967 = 0u64;
    if if if nat_eq(a3966, a3967) {
        true
    } else {
        let a3968 = v1;
        let a3969 = 0u64;
        nat_eq(a3968, a3969)
    } {
        true
    } else {
        let a3970 = v2;
        let a3971 = 0u64;
        nat_eq(a3970, a3971)
    } {
        true
    } else {
        let a3972 = v3;
        let a3973 = 0u64;
        nat_eq(a3972, a3973)
    }
}

pub fn f170(v0: Adt14, v1: Adt14) -> bool {
    if f176(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3974, _, _, _, _, _) => {
            h3974
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h3975, _, _, _, _, _) => {
            h3975
        }
    }) {
        let a3977 = match v0 {
            Adt14::C0(_, h3976, _, _, _, _, _, _, _, _, _) => {
                h3976
            }
        };
        let a3978 = 2147483647u64;
        nat_le(a3977, a3978)
    } else {
        true
    }
}

pub fn f171(v0: Adt14) -> bool {
    let a3983 = {
        let a3980 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3979) => {
                h3979
            }
        };
        let a3981 = 4u64;
        let a3982 = 0u64;
        nat_rem(a3980, a3981, a3982)
    };
    let a3984 = 0u64;
    if if nat_eq(a3983, a3984) {
        f216(match v0.clone() {
            Adt14::C0(_, h3985, _, _, _, _, _, _, _, _, _) => {
                h3985
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3986, _, _, _, _, _) => {
                h3986
            }
        })
    } else {
        false
    } {
        f94(match v0.clone() {
            Adt14::C0(_, _, h3987, _, _, _, _, _, _, _, _) => {
                h3987
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3988, _, _, _, _, _, _, _) => {
                h3988
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3989, _, _, _, _, _, _) => {
                h3989
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3990, _, _, _, _) => {
                h3990
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3991, _, _, _) => {
                h3991
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3992, _, _) => {
                h3992
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f172(v0: Adt14, v1: Adt14) -> R<bool> {
    let a3994 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3993, _) => {
            h3993
        }
    };
    let a3995 = 0u64;
    if nat_eq(a3994, a3995) {
        Ok(true)
    } else {
        let a3997 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h3996, _) => {
                h3996
            }
        };
        let a3998 = 0u64;
        if nat_eq(a3997, a3998) {
            Ok(true)
        } else {
            let a4000 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3999) => {
                    h3999
                }
            };
            let a4001 = 0u64;
            if nat_eq(a4000, a4001) {
                Ok(true)
            } else {
                let a4003 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4002) => {
                        h4002
                    }
                };
                let a4004 = 0u64;
                if nat_eq(a4003, a4004) {
                    Ok(true)
                } else {
                    let a4010 = {
                        let a4007 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4005) => {
                                h4005
                            }
                        };
                        let a4008 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4006, _) => {
                                h4006
                            }
                        };
                        nat_add(a4007, a4008)?
                    };
                    let a4011 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4009) => {
                            h4009
                        }
                    };
                    if nat_le(a4010, a4011) {
                        Ok(true)
                    } else {
                        let a4017 = {
                            let a4014 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4012) => {
                                    h4012
                                }
                            };
                            let a4015 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4013, _) => {
                                    h4013
                                }
                            };
                            nat_add(a4014, a4015)?
                        };
                        let a4018 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4016) => {
                                h4016
                            }
                        };
                        Ok(nat_le(a4017, a4018))
                    }
                }
            }
        }
    }
}

pub fn f173(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f168(v0.clone(), v1.clone()) {
        let a4021 = match v0.clone() {
            Adt14::C0(h4019, _, _, _, _, _, _, _, _, _, _) => {
                h4019
            }
        };
        let a4022 = match v1.clone() {
            Adt14::C0(h4020, _, _, _, _, _, _, _, _, _, _) => {
                h4020
            }
        };
        nat_eq(a4021, a4022)
    } else {
        false
    } {
        let a4025 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4023) => {
                h4023
            }
        };
        let a4026 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4024) => {
                h4024
            }
        };
        nat_eq(a4025, a4026)
    } else {
        false
    } {
        let a4029 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4027, _, _, _, _, _) => {
                h4027
            }
        };
        let a4030 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4028, _, _, _, _, _) => {
                h4028
            }
        };
        nat_eq(a4029, a4030)
    } else {
        false
    } {
        let a4033 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4031, _, _, _, _) => {
                h4031
            }
        };
        let a4034 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4032, _, _, _, _) => {
                h4032
            }
        };
        nat_eq(a4033, a4034)
    } else {
        false
    } {
        let a4037 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4035, _, _, _) => {
                h4035
            }
        };
        let a4038 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4036, _, _, _) => {
                h4036
            }
        };
        nat_eq(a4037, a4038)
    } else {
        false
    } {
        let a4041 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4039, _, _) => {
                h4039
            }
        };
        let a4042 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4040, _, _) => {
                h4040
            }
        };
        nat_eq(a4041, a4042)
    } else {
        false
    }
}

pub fn f174(v0: Adt14) -> R<bool> {
    if f217(match v0.clone() {
        Adt14::C0(_, h4043, _, _, _, _, _, _, _, _, _) => {
            h4043
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4044, _, _, _, _, _) => {
            h4044
        }
    }) {
        f95(f218(match v0.clone() {
            Adt14::C0(_, h4045, _, _, _, _, _, _, _, _, _) => {
                h4045
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4046, _, _, _, _, _) => {
                h4046
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4047, _, _, _, _, _, _, _, _) => {
                h4047
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4048, _, _, _, _, _, _, _) => {
                h4048
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4049, _, _, _, _, _, _) => {
                h4049
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4050, _, _, _, _) => {
                h4050
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4051, _, _, _) => {
                h4051
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4052, _, _) => {
                h4052
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f175(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4053 = v1;
    let a4054 = 0u64;
    if if nat_eq(a4053, a4054) {
        true
    } else {
        let a4055 = v2;
        let a4056 = 0u64;
        nat_eq(a4055, a4056)
    } {
        Ok(0u64)
    } else {
        let a4059 = f96(v1, v2, v3)?;
        let a4060 = {
            let a4057 = v2;
            let a4058 = f219(v0, v1)?;
            nat_mul(a4057, a4058)?
        };
        nat_add(a4059, a4060)
    }
}

pub fn f176(v0: u64, v1: u64) -> bool {
    let a4061 = v0;
    let a4062 = 4u64;
    if nat_eq(a4061, a4062) {
        let a4063 = v1;
        let a4064 = 4u64;
        nat_eq(a4063, a4064)
    } else {
        false
    }
}

pub fn f177(v0: u64) -> R<u64> {
    let a4065 = 6u64;
    let a4066 = v0;
    nat_add(a4065, a4066)
}

pub fn f178(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4067, _, _, _, _, _, _, _, _, _, _) => {
            h4067
        }
    }, match v0.clone() {
        Adt14::C0(_, h4068, _, _, _, _, _, _, _, _, _) => {
            h4068
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4069, _, _, _, _, _, _, _, _) => {
            h4069
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4070, _, _, _, _, _, _, _) => {
            h4070
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4071, _, _, _, _, _, _) => {
            h4071
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4072, _, _, _, _) => {
            h4072
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4073, _, _, _) => {
            h4073
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4074, _, _) => {
            h4074
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4075, _) => {
            h4075
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4076) => {
            h4076
        }
    })
}

pub fn f179(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4077 = v0;
    let a4078 = 2147483647u64;
    if if if nat_le(a4077, a4078) {
        let a4079 = 0u64;
        let a4080 = v3;
        if nat_lt(a4079, a4080) {
            let a4081 = v3;
            let a4082 = 2147483647u64;
            nat_le(a4081, a4082)
        } else {
            false
        }
    } else {
        false
    } {
        let a4087 = {
            let a4085 = {
                let a4083 = v2;
                let a4084 = v3;
                nat_add(a4083, a4084)?
            };
            let a4086 = 1u64;
            nat_sub(a4085, a4086)
        };
        let a4088 = 2147483647u64;
        nat_le(a4087, a4088)
    } else {
        false
    } {
        let a4093 = {
            let a4091 = {
                let a4089 = v0;
                let a4090 = v1;
                nat_mul(a4089, a4090)?
            };
            let a4092 = f220(v2, v3)?;
            nat_mul(a4091, a4092)?
        };
        let a4094 = 9223372036854775807u64;
        Ok(nat_le(a4093, a4094))
    } else {
        Ok(false)
    }
}

pub fn f180(v0: Adt14) -> R<u64> {
    let a4125 = {
        let a4117 = {
            let a4109 = {
                let a4101 = 4u64;
                let a4102 = {
                    let a4099 = {
                        let a4096 = match v0.clone() {
                            Adt14::C0(_, h4095, _, _, _, _, _, _, _, _, _) => {
                                h4095
                            }
                        };
                        let a4097 = 1u64;
                        nat_sub(a4096, a4097)
                    };
                    let a4100 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4098, _, _, _, _, _) => {
                            h4098
                        }
                    };
                    nat_mul(a4099, a4100)?
                };
                nat_add(a4101, a4102)?
            };
            let a4110 = {
                let a4107 = {
                    let a4104 = match v0.clone() {
                        Adt14::C0(_, _, h4103, _, _, _, _, _, _, _, _) => {
                            h4103
                        }
                    };
                    let a4105 = 1u64;
                    nat_sub(a4104, a4105)
                };
                let a4108 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4106, _, _, _, _) => {
                        h4106
                    }
                };
                nat_mul(a4107, a4108)?
            };
            nat_add(a4109, a4110)?
        };
        let a4118 = {
            let a4115 = {
                let a4112 = match v0.clone() {
                    Adt14::C0(_, _, _, h4111, _, _, _, _, _, _, _) => {
                        h4111
                    }
                };
                let a4113 = 1u64;
                nat_sub(a4112, a4113)
            };
            let a4116 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4114, _, _, _) => {
                    h4114
                }
            };
            nat_mul(a4115, a4116)?
        };
        nat_add(a4117, a4118)?
    };
    let a4126 = {
        let a4123 = {
            let a4120 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4119, _, _, _, _, _, _) => {
                    h4119
                }
            };
            let a4121 = 1u64;
            nat_sub(a4120, a4121)
        };
        let a4124 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4122, _, _) => {
                h4122
            }
        };
        nat_mul(a4123, a4124)?
    };
    nat_add(a4125, a4126)
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4127 = v0;
    let a4128 = 0u64;
    if if if if nat_eq(a4127, a4128) {
        true
    } else {
        let a4129 = v1;
        let a4130 = 0u64;
        nat_eq(a4129, a4130)
    } {
        true
    } else {
        let a4131 = v2;
        let a4132 = 0u64;
        nat_eq(a4131, a4132)
    } {
        true
    } else {
        let a4133 = v3;
        let a4134 = 0u64;
        nat_eq(a4133, a4134)
    } {
        Ok(0u64)
    } else {
        let a4135 = f221(v0, v1, v2, v3, v5)?;
        let a4136 = f222(v0, v1, v2, v3, v4)?;
        nat_add(a4135, a4136)
    }
}

pub fn f182(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4137 = v0;
    let a4138 = 0u64;
    if if nat_eq(a4137, a4138) {
        true
    } else {
        let a4139 = v1;
        let a4140 = 0u64;
        nat_eq(a4139, a4140)
    } {
        Ok(0u64)
    } else {
        let a4143 = f96(v0, v1, v2)?;
        let a4144 = {
            let a4141 = v1;
            let a4142 = f223(v0)?;
            nat_mul(a4141, a4142)?
        };
        nat_add(a4143, a4144)
    }
}

pub fn f183(v0: u64, v1: u64, v2: u64) -> bool {
    let a4145 = v0;
    let a4146 = 1u64;
    if nat_le(a4145, a4146) {
        true
    } else {
        let a4150 = {
            let a4147 = v1;
            let a4148 = v2;
            let a4149 = 1u64;
            nat_rem(a4147, a4148, a4149)
        };
        let a4151 = 0u64;
        nat_eq(a4150, a4151)
    }
}

pub fn f184(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4152 = v1;
    let a4153 = 1u64;
    if if if nat_le(a4152, a4153) {
        true
    } else {
        let a4154 = v0;
        let a4155 = v2;
        nat_le(a4154, a4155)
    } {
        let a4156 = v3;
        let a4157 = 1u64;
        if nat_le(a4156, a4157) {
            true
        } else {
            let a4164 = {
                let a4162 = v0;
                let a4163 = {
                    let a4160 = {
                        let a4158 = v1;
                        let a4159 = 1u64;
                        nat_sub(a4158, a4159)
                    };
                    let a4161 = v2;
                    nat_mul(a4160, a4161)?
                };
                nat_add(a4162, a4163)?
            };
            let a4165 = v4;
            nat_le(a4164, a4165)
        }
    } else {
        false
    } {
        let a4166 = v5;
        let a4167 = 1u64;
        if nat_le(a4166, a4167) {
            Ok(true)
        } else {
            let a4180 = {
                let a4178 = {
                    let a4172 = v0;
                    let a4173 = {
                        let a4170 = {
                            let a4168 = v1;
                            let a4169 = 1u64;
                            nat_sub(a4168, a4169)
                        };
                        let a4171 = v2;
                        nat_mul(a4170, a4171)?
                    };
                    nat_add(a4172, a4173)?
                };
                let a4179 = {
                    let a4176 = {
                        let a4174 = v3;
                        let a4175 = 1u64;
                        nat_sub(a4174, a4175)
                    };
                    let a4177 = v4;
                    nat_mul(a4176, a4177)?
                };
                nat_add(a4178, a4179)?
            };
            let a4181 = v6;
            Ok(nat_le(a4180, a4181))
        }
    } else {
        Ok(false)
    }
}

pub fn f185(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f224(v0, v1, v6, v7, v8) {
        let a4182 = v5;
        let a4183 = 26u64;
        nat_eq(a4182, a4183)
    } else {
        false
    } {
        let a4184 = v3;
        let a4185 = v7;
        nat_eq(a4184, a4185)
    } else {
        false
    } {
        let a4186 = v4;
        let a4187 = v8;
        nat_eq(a4186, a4187)
    } else {
        false
    } {
        let a4188 = v9;
        let a4189 = 1u64;
        nat_eq(a4188, a4189)
    } else {
        false
    } {
        let a4190 = v10;
        let a4191 = f225(v0);
        nat_eq(a4190, a4191)
    } else {
        false
    } {
        let a4192 = v11;
        let a4193 = v1;
        nat_eq(a4192, a4193)
    } else {
        false
    } {
        let a4194 = v12;
        let a4195 = v6;
        nat_eq(a4194, a4195)
    } else {
        false
    } {
        let a4196 = v13;
        let a4197 = v7;
        nat_eq(a4196, a4197)
    } else {
        false
    } {
        let a4198 = v14;
        let a4199 = v8;
        nat_eq(a4198, a4199)
    } else {
        false
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4200 = v0;
    let a4201 = 0u64;
    if if if nat_eq(a4200, a4201) {
        true
    } else {
        let a4202 = v1;
        let a4203 = 0u64;
        nat_eq(a4202, a4203)
    } {
        true
    } else {
        let a4204 = v2;
        let a4205 = 0u64;
        nat_eq(a4204, a4205)
    } {
        true
    } else {
        let a4206 = v3;
        let a4207 = 0u64;
        nat_eq(a4206, a4207)
    }
}

pub fn f187(v0: u64) -> u64 {
    let m4208 = f110(v0);
    match m4208 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4209, _) => {
                    h4209
                }
            }
        }
    }
}

pub fn f188(v0: Adt3, v1: bool) -> R<u64> {
    f226(v0.clone(), {
        let a4211 = match v0.clone() {
            Adt3::C0(h4210, _, _, _, _, _, _, _) => {
                h4210
            }
        };
        let a4212 = 1u64;
        nat_sub(a4211, a4212)
    }, {
        let a4214 = match v0.clone() {
            Adt3::C0(_, h4213, _, _, _, _, _, _) => {
                h4213
            }
        };
        let a4215 = 1u64;
        nat_sub(a4214, a4215)
    }, {
        let a4217 = match v0.clone() {
            Adt3::C0(_, _, h4216, _, _, _, _, _) => {
                h4216
            }
        };
        let a4218 = 1u64;
        nat_sub(a4217, a4218)
    }, {
        let a4220 = match v0 {
            Adt3::C0(_, _, _, h4219, _, _, _, _) => {
                h4219
            }
        };
        let a4221 = 1u64;
        nat_sub(a4220, a4221)
    }, v1)
}

pub fn f189(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4222 = v0;
    let a4223 = 0u64;
    if nat_eq(a4222, a4223) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4224 = v0;
            let a4225 = 1u64;
            nat_sub(a4224, a4225)
        };
        let v5: u64 = {
            let a4226 = v4;
            let a4227 = v1;
            let a4228 = 0u64;
            nat_quot(a4226, a4227, a4228)
        };
        let v6: u64 = {
            let a4236 = {
                let a4229 = v5;
                let a4230 = v2;
                nat_mul(a4229, a4230)?
            };
            let a4237 = {
                let a4234 = {
                    let a4231 = v4;
                    let a4232 = v1;
                    let a4233 = 0u64;
                    nat_rem(a4231, a4232, a4233)
                };
                let a4235 = v3;
                nat_mul(a4234, a4235)?
            };
            nat_add(a4236, a4237)?
        };
        let a4238 = v5;
        let a4239 = 0u64;
        if nat_eq(a4238, a4239) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4248 = {
                    let a4242 = {
                        let a4240 = v5;
                        let a4241 = 1u64;
                        nat_sub(a4240, a4241)
                    };
                    let a4243 = v2;
                    nat_mul(a4242, a4243)?
                };
                let a4249 = {
                    let a4246 = {
                        let a4244 = v1;
                        let a4245 = 1u64;
                        nat_sub(a4244, a4245)
                    };
                    let a4247 = v3;
                    nat_mul(a4246, a4247)?
                };
                nat_add(a4248, a4249)?
            };
            let a4250 = v6;
            let a4251 = v7;
            if nat_lt(a4250, a4251) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f190(v0: Adt3, v1: u64) -> bool {
    let a4252 = v1;
    let a4253 = 0u64;
    if !nat_eq(a4252, a4253) {
        true
    } else {
        let a4255 = match v0 {
            Adt3::C0(_, _, _, _, h4254, _, _, _) => {
                h4254
            }
        };
        let a4256 = 4u64;
        nat_eq(a4255, a4256)
    }
}

pub fn f191(v0: u64, v1: bool) -> R<u64> {
    let a4259 = {
        let a4257 = v0;
        let a4258 = 2u64;
        nat_mul(a4257, a4258)?
    };
    let a4260 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4259, a4260)
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4261 = f227(v0, v1);
    match m4261 {
        None => {
            if v5 {
                let m4262 = f228(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4262 {
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
                let a4263 = 4u64;
                let a4264 = v28;
                nat_le(a4263, a4264)
            };
            let a4265 = v2;
            let a4266 = 0u64;
            if if if if if nat_eq(a4265, a4266) {
                let a4267 = v4;
                let a4268 = 0u64;
                nat_eq(a4267, a4268)
            } else {
                false
            } {
                let a4269 = v6;
                let a4270 = v14;
                if nat_eq(a4269, a4270) {
                    let a4271 = v7;
                    let a4272 = v15;
                    nat_eq(a4271, a4272)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4273 = v8;
                let a4274 = v16;
                if nat_eq(a4273, a4274) {
                    let a4275 = v9;
                    let a4276 = v17;
                    nat_eq(a4275, a4276)
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
                        let a4277 = v3;
                        let a4278 = 0u64;
                        nat_eq(a4277, a4278)
                    } else {
                        false
                    } {
                        f113(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4283 = {
                        let a4279 = v6;
                        let a4280 = v7;
                        nat_mul(a4279, a4280)?
                    };
                    let a4284 = {
                        let a4281 = v8;
                        let a4282 = v9;
                        nat_mul(a4281, a4282)?
                    };
                    nat_mul(a4283, a4284)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f193(v0: u64) -> u64 {
    let m4285 = f110(v0);
    match m4285 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4287 = match v1.clone() {
                Adt18::C0(_, h4286, _) => {
                    h4286
                }
            };
            let a4288 = 1u64;
            if if nat_eq(a4287, a4288) {
                let a4290 = match v1.clone() {
                    Adt18::C0(_, _, h4289) => {
                        h4289
                    }
                };
                let a4291 = 1u64;
                if if if nat_eq(a4290, a4291) {
                    true
                } else {
                    let a4293 = match v1.clone() {
                        Adt18::C0(_, _, h4292) => {
                            h4292
                        }
                    };
                    let a4294 = 2u64;
                    nat_eq(a4293, a4294)
                } {
                    true
                } else {
                    let a4296 = match v1.clone() {
                        Adt18::C0(_, _, h4295) => {
                            h4295
                        }
                    };
                    let a4297 = 4u64;
                    nat_eq(a4296, a4297)
                } {
                    true
                } else {
                    let a4299 = match v1.clone() {
                        Adt18::C0(_, _, h4298) => {
                            h4298
                        }
                    };
                    let a4300 = 8u64;
                    nat_eq(a4299, a4300)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4301) => {
                        h4301
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4306 = {
        let a4304 = {
            let a4302 = v0;
            let a4303 = v1;
            nat_sub(a4302, a4303)
        };
        let a4305 = 1u64;
        nat_sub(a4304, a4305)
    };
    let a4307 = v2;
    nat_add(a4306, a4307)
}

pub fn f195(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f126({
        let a4308 = v0;
        let a4309 = v4;
        let a4310 = 0u64;
        nat_rem(a4308, a4309, a4310)
    }, {
        let a4311 = v1;
        let a4312 = v5;
        let a4313 = 0u64;
        nat_rem(a4311, a4312, a4313)
    }, {
        let a4314 = v2;
        let a4315 = v6;
        let a4316 = 0u64;
        nat_rem(a4314, a4315, a4316)
    }, {
        let a4317 = v3;
        let a4318 = v7;
        let a4319 = 0u64;
        nat_rem(a4317, a4318, a4319)
    }, v8, v9, v10, v11)
}

pub fn f196(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4320 = v1;
    let a4321 = 2147483648u64;
    if nat_lt(a4320, a4321) {
        let a4332 = {
            let a4330 = {
                let a4325 = {
                    let a4322 = v0;
                    let a4323 = v2;
                    let a4324 = 0u64;
                    nat_rem(a4322, a4323, a4324)
                };
                let a4326 = v2;
                nat_add(a4325, a4326)?
            };
            let a4331 = {
                let a4327 = v1;
                let a4328 = v2;
                let a4329 = 0u64;
                nat_rem(a4327, a4328, a4329)
            };
            nat_sub(a4330, a4331)
        };
        let a4333 = v2;
        let a4334 = 0u64;
        Ok(nat_rem(a4332, a4333, a4334))
    } else {
        let a4337 = {
            let a4335 = v0;
            let a4336 = f127(v1);
            nat_add(a4335, a4336)?
        };
        let a4338 = v2;
        let a4339 = 0u64;
        Ok(nat_rem(a4337, a4338, a4339))
    }
}

pub fn f197(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4342 = f128(v1);
    let a4343 = {
        let a4340 = v0;
        let a4341 = f127(v1);
        nat_add(a4340, a4341)?
    };
    if nat_le(a4342, a4343) {
        let a4344 = f198(v0, v1)?;
        let a4345 = v2;
        Ok(nat_lt(a4344, a4345))
    } else {
        Ok(false)
    }
}

pub fn f198(v0: u64, v1: u64) -> R<u64> {
    let a4348 = {
        let a4346 = v0;
        let a4347 = f127(v1);
        nat_add(a4346, a4347)?
    };
    let a4349 = f128(v1);
    Ok(nat_sub(a4348, a4349))
}

pub fn f199(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4350 = v0;
    let a4351 = v1;
    if nat_lt(a4350, a4351) {
        let a4352 = v1;
        let a4353 = v0;
        Ok(nat_sub(a4352, a4353))
    } else {
        let a4356 = v0;
        let a4357 = {
            let a4354 = v1;
            let a4355 = v2;
            nat_add(a4354, a4355)?
        };
        if nat_lt(a4356, a4357) {
            let a4358 = v0;
            let a4359 = v1;
            Ok(nat_sub(a4358, a4359))
        } else {
            let a4366 = {
                let a4362 = 2u64;
                let a4363 = {
                    let a4360 = v2;
                    let a4361 = 1u64;
                    nat_sub(a4360, a4361)
                };
                nat_mul(a4362, a4363)?
            };
            let a4367 = {
                let a4364 = v0;
                let a4365 = v1;
                nat_sub(a4364, a4365)
            };
            Ok(nat_sub(a4366, a4367))
        }
    }
}

pub fn f200(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4370 = {
        let a4368 = v2;
        let a4369 = f226(v1.clone(), f67(v0, v1.clone(), 0u64), f67(v0, v1.clone(), 1u64), f67(v0, v1.clone(), 2u64), f67(v0, v1.clone(), 3u64), false)?;
        nat_add(a4368, a4369)?
    };
    let a4371 = f226(v1.clone(), f67(v0, v1.clone(), 0u64), f67(v0, v1.clone(), 1u64), f67(v0, v1.clone(), 2u64), f67(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4370, a4371))
}

pub fn f201(v0: u64, v1: u64, v2: u64) -> bool {
    let a4372 = v0;
    let a4373 = 0u64;
    if nat_eq(a4372, a4373) {
        let a4374 = v2;
        let a4375 = v1;
        nat_le(a4374, a4375)
    } else {
        let a4376 = v0;
        let a4377 = 1u64;
        if nat_eq(a4376, a4377) {
            let a4378 = v2;
            let a4379 = v1;
            nat_lt(a4378, a4379)
        } else {
            let a4380 = v0;
            let a4381 = 2u64;
            if nat_eq(a4380, a4381) {
                let a4382 = v1;
                let a4383 = v2;
                nat_le(a4382, a4383)
            } else {
                let a4384 = v0;
                let a4385 = 3u64;
                if nat_eq(a4384, a4385) {
                    let a4386 = v1;
                    let a4387 = v2;
                    nat_lt(a4386, a4387)
                } else {
                    let a4388 = v1;
                    let a4389 = v2;
                    nat_eq(a4388, a4389)
                }
            }
        }
    }
}

pub fn f202(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4390 = v0;
    let a4391 = 0u64;
    if nat_eq(a4390, a4391) {
        Ok(f213(0u64))
    } else {
        let a4394 = {
            let a4392 = v0;
            let a4393 = 1u64;
            nat_add(a4392, a4393)?
        };
        let a4395 = f124(v1.clone(), 13u64, v2)?;
        if nat_eq(a4394, a4395) {
            Ok(Adt9::C4({
                let a4396 = v2;
                let a4397 = 0u64;
                if nat_eq(a4396, a4397) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4398 = v0;
                let a4399 = 1u64;
                nat_sub(a4398, a4399)
            };
            if f190(v1.clone(), v2) {
                let v4: u64 = f191(match v1.clone() {
                    Adt3::C0(h4400, _, _, _, _, _, _, _) => {
                        h4400
                    }
                }, false)?;
                let v5: u64 = {
                    let a4401 = v3;
                    let a4402 = v4;
                    let a4403 = 0u64;
                    nat_quot(a4401, a4402, a4403)
                };
                let v6: u64 = {
                    let a4404 = v3;
                    let a4405 = v4;
                    let a4406 = 0u64;
                    nat_rem(a4404, a4405, a4406)
                };
                let a4409 = {
                    let a4407 = v6;
                    let a4408 = 1u64;
                    nat_add(a4407, a4408)?
                };
                let a4410 = v4;
                if nat_eq(a4409, a4410) {
                    Ok(Adt9::C4({
                        let a4411 = v2;
                        let a4412 = 0u64;
                        if nat_eq(a4411, a4412) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f203(v6, v5, v1, v2)
                }
            } else {
                f229({
                    let a4413 = v3;
                    let a4414 = 2u64;
                    let a4415 = 0u64;
                    nat_quot(a4413, a4414, a4415)
                }, {
                    let a4416 = v3;
                    let a4417 = 2u64;
                    let a4418 = 0u64;
                    nat_rem(a4416, a4417, a4418)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f203(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4419 = v0;
    let a4420 = 0u64;
    if nat_eq(a4419, a4420) {
        Ok(f213(1u64))
    } else {
        f229({
            let a4429 = {
                let a4422 = v1;
                let a4423 = match v2.clone() {
                    Adt3::C0(h4421, _, _, _, _, _, _, _) => {
                        h4421
                    }
                };
                nat_mul(a4422, a4423)?
            };
            let a4430 = {
                let a4426 = {
                    let a4424 = v0;
                    let a4425 = 1u64;
                    nat_sub(a4424, a4425)
                };
                let a4427 = 2u64;
                let a4428 = 0u64;
                nat_quot(a4426, a4427, a4428)
            };
            nat_add(a4429, a4430)?
        }, {
            let a4433 = {
                let a4431 = v0;
                let a4432 = 1u64;
                nat_sub(a4431, a4432)
            };
            let a4434 = 2u64;
            let a4435 = 0u64;
            nat_rem(a4433, a4434, a4435)
        }, v2, v3, 1u64)
    }
}

pub fn f204(v0: u64, v1: u64) -> R<u64> {
    let a4436 = v0;
    let a4437 = f90(v1)?;
    let a4438 = 0u64;
    Ok(nat_rem(a4436, a4437, a4438))
}

pub fn f205(v0: u64, v1: u64) -> R<u64> {
    let a4439 = 4u64;
    let a4440 = f230(v0, v1)?;
    nat_mul(a4439, a4440)
}

pub fn f206(v0: u64, v1: u64) -> R<u64> {
    let a4441 = 4u64;
    let a4442 = f231(v0, v1)?;
    nat_mul(a4441, a4442)
}

pub fn f207(v0: u64, v1: u64) -> R<bool> {
    let a4448 = {
        let a4445 = {
            let a4443 = f204(v0, v1)?;
            let a4444 = 2u64;
            nat_sub(a4443, a4444)
        };
        let a4446 = 2u64;
        let a4447 = 0u64;
        nat_rem(a4445, a4446, a4447)
    };
    let a4449 = 0u64;
    Ok(nat_eq(a4448, a4449))
}

pub fn f208(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4450 = f110(v0);
    match m4450 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f108(v0) {
                let a4451 = v1;
                let a4452 = v7;
                nat_lt(a4451, a4452)
            } else {
                false
            } {
                f155(v8, v2)
            } else {
                false
            } {
                let a4453 = v3;
                let a4454 = f232(v4, v5, v6)?;
                nat_lt(a4453, a4454)
            } else {
                false
            } {
                let a4459 = {
                    let a4456 = v7;
                    let a4457 = match v13.clone() {
                        Adt18::C0(_, h4455, _) => {
                            h4455
                        }
                    };
                    let a4458 = 0u64;
                    nat_rem(a4456, a4457, a4458)
                };
                let a4460 = 0u64;
                nat_eq(a4459, a4460)
            } else {
                false
            } {
                Ok(Some({
                    let a4489 = {
                        let a4479 = {
                            let a4469 = {
                                let a4465 = {
                                    let a4462 = v1;
                                    let a4463 = match v13 {
                                        Adt18::C0(_, h4461, _) => {
                                            h4461
                                        }
                                    };
                                    let a4464 = 0u64;
                                    nat_quot(a4462, a4463, a4464)
                                };
                                let a4466 = v9;
                                nat_mul(a4465, a4466)?
                            };
                            let a4470 = {
                                let a4467 = v2;
                                let a4468 = v10;
                                nat_mul(a4467, a4468)?
                            };
                            nat_add(a4469, a4470)?
                        };
                        let a4480 = {
                            let a4477 = {
                                let a4474 = {
                                    let a4471 = v3;
                                    let a4472 = v4;
                                    let a4473 = 0u64;
                                    nat_quot(a4471, a4472, a4473)
                                };
                                let a4475 = v5;
                                let a4476 = 0u64;
                                nat_rem(a4474, a4475, a4476)
                            };
                            let a4478 = v11;
                            nat_mul(a4477, a4478)?
                        };
                        nat_add(a4479, a4480)?
                    };
                    let a4490 = {
                        let a4487 = {
                            let a4484 = {
                                let a4481 = v3;
                                let a4482 = v4;
                                let a4483 = 0u64;
                                nat_quot(a4481, a4482, a4483)
                            };
                            let a4485 = v5;
                            let a4486 = 0u64;
                            nat_quot(a4484, a4485, a4486)
                        };
                        let a4488 = v12;
                        nat_mul(a4487, a4488)?
                    };
                    nat_add(a4489, a4490)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f233(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f210(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f234(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f211(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4491 = v0;
    let a4492 = 64u64;
    if nat_eq(a4491, a4492) {
        f235(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4493 = v0;
        let a4494 = 65u64;
        if nat_eq(a4493, a4494) {
            f236(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f212(v0: u64) -> u64 {
    let a4495 = v0;
    let a4496 = 30u64;
    if nat_eq(a4495, a4496) {
        4u64
    } else {
        3u64
    }
}

pub fn f213(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f214(v0: u64) -> bool {
    let a4497 = 0u64;
    let a4498 = v0;
    if nat_lt(a4497, a4498) {
        let a4499 = v0;
        let a4500 = 2139095040u64;
        nat_le(a4499, a4500)
    } else {
        false
    }
}

pub fn f215(v0: u64) -> u64 {
    let a4501 = v0;
    let a4502 = 40u64;
    if nat_lt(a4501, a4502) {
        let a4503 = v0;
        let a4504 = 20u64;
        if nat_lt(a4503, a4504) {
            let a4505 = v0;
            let a4506 = 10u64;
            if nat_lt(a4505, a4506) {
                let a4507 = v0;
                let a4508 = 5u64;
                if nat_lt(a4507, a4508) {
                    let a4509 = v0;
                    let a4510 = 2u64;
                    if nat_lt(a4509, a4510) {
                        let a4511 = v0;
                        let a4512 = 1u64;
                        if nat_lt(a4511, a4512) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4513 = v0;
                        let a4514 = 3u64;
                        if nat_lt(a4513, a4514) {
                            4u64
                        } else {
                            let a4515 = v0;
                            let a4516 = 4u64;
                            if nat_lt(a4515, a4516) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4517 = v0;
                    let a4518 = 7u64;
                    if nat_lt(a4517, a4518) {
                        let a4519 = v0;
                        let a4520 = 6u64;
                        if nat_lt(a4519, a4520) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4521 = v0;
                        let a4522 = 8u64;
                        if nat_lt(a4521, a4522) {
                            128u64
                        } else {
                            let a4523 = v0;
                            let a4524 = 9u64;
                            if nat_lt(a4523, a4524) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4525 = v0;
                let a4526 = 15u64;
                if nat_lt(a4525, a4526) {
                    let a4527 = v0;
                    let a4528 = 12u64;
                    if nat_lt(a4527, a4528) {
                        let a4529 = v0;
                        let a4530 = 11u64;
                        if nat_lt(a4529, a4530) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4531 = v0;
                        let a4532 = 13u64;
                        if nat_lt(a4531, a4532) {
                            4096u64
                        } else {
                            let a4533 = v0;
                            let a4534 = 14u64;
                            if nat_lt(a4533, a4534) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4535 = v0;
                    let a4536 = 17u64;
                    if nat_lt(a4535, a4536) {
                        let a4537 = v0;
                        let a4538 = 16u64;
                        if nat_lt(a4537, a4538) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4539 = v0;
                        let a4540 = 18u64;
                        if nat_lt(a4539, a4540) {
                            131072u64
                        } else {
                            let a4541 = v0;
                            let a4542 = 19u64;
                            if nat_lt(a4541, a4542) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4543 = v0;
            let a4544 = 30u64;
            if nat_lt(a4543, a4544) {
                let a4545 = v0;
                let a4546 = 25u64;
                if nat_lt(a4545, a4546) {
                    let a4547 = v0;
                    let a4548 = 22u64;
                    if nat_lt(a4547, a4548) {
                        let a4549 = v0;
                        let a4550 = 21u64;
                        if nat_lt(a4549, a4550) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4551 = v0;
                        let a4552 = 23u64;
                        if nat_lt(a4551, a4552) {
                            4194304u64
                        } else {
                            let a4553 = v0;
                            let a4554 = 24u64;
                            if nat_lt(a4553, a4554) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4555 = v0;
                    let a4556 = 27u64;
                    if nat_lt(a4555, a4556) {
                        let a4557 = v0;
                        let a4558 = 26u64;
                        if nat_lt(a4557, a4558) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4559 = v0;
                        let a4560 = 28u64;
                        if nat_lt(a4559, a4560) {
                            134217728u64
                        } else {
                            let a4561 = v0;
                            let a4562 = 29u64;
                            if nat_lt(a4561, a4562) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4563 = v0;
                let a4564 = 35u64;
                if nat_lt(a4563, a4564) {
                    let a4565 = v0;
                    let a4566 = 32u64;
                    if nat_lt(a4565, a4566) {
                        let a4567 = v0;
                        let a4568 = 31u64;
                        if nat_lt(a4567, a4568) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4569 = v0;
                        let a4570 = 33u64;
                        if nat_lt(a4569, a4570) {
                            4294967296u64
                        } else {
                            let a4571 = v0;
                            let a4572 = 34u64;
                            if nat_lt(a4571, a4572) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4573 = v0;
                    let a4574 = 37u64;
                    if nat_lt(a4573, a4574) {
                        let a4575 = v0;
                        let a4576 = 36u64;
                        if nat_lt(a4575, a4576) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4577 = v0;
                        let a4578 = 38u64;
                        if nat_lt(a4577, a4578) {
                            137438953472u64
                        } else {
                            let a4579 = v0;
                            let a4580 = 39u64;
                            if nat_lt(a4579, a4580) {
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

pub fn f216(v0: u64, v1: u64) -> bool {
    let a4581 = v0;
    let a4582 = 1u64;
    if nat_le(a4581, a4582) {
        true
    } else {
        let a4586 = {
            let a4583 = v1;
            let a4584 = 4u64;
            let a4585 = 0u64;
            nat_rem(a4583, a4584, a4585)
        };
        let a4587 = 0u64;
        nat_eq(a4586, a4587)
    }
}

pub fn f217(v0: u64, v1: u64) -> bool {
    let a4588 = v0;
    let a4589 = 1u64;
    if nat_le(a4588, a4589) {
        true
    } else {
        let a4590 = 4u64;
        let a4591 = v1;
        nat_le(a4590, a4591)
    }
}

pub fn f218(v0: u64, v1: u64) -> R<u64> {
    let a4592 = v0;
    let a4593 = 0u64;
    if nat_eq(a4592, a4593) {
        Ok(0u64)
    } else {
        let a4598 = 4u64;
        let a4599 = {
            let a4596 = {
                let a4594 = v0;
                let a4595 = 1u64;
                nat_sub(a4594, a4595)
            };
            let a4597 = v1;
            nat_mul(a4596, a4597)?
        };
        nat_add(a4598, a4599)
    }
}

pub fn f219(v0: bool, v1: u64) -> R<u64> {
    let a4604 = {
        let a4602 = {
            let a4600 = f237(v0, v1)?;
            let a4601 = f238(v0, v1)?;
            nat_add(a4600, a4601)?
        };
        let a4603 = f239(v0);
        nat_add(a4602, a4603)?
    };
    let a4605 = v1;
    nat_add(a4604, a4605)
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f240(v0, v1)?;
    let v3: u64 = {
        let a4610 = {
            let a4608 = {
                let a4606 = v1;
                let a4607 = 1u64;
                nat_sub(a4606, a4607)
            };
            let a4609 = v2;
            nat_mul(a4608, a4609)?
        };
        let a4611 = v0;
        nat_sub(a4610, a4611)
    };
    let a4612 = v2;
    let a4613 = v3;
    if nat_lt(a4612, a4613) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f221(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4614 = v4;
    let a4615 = 0u64;
    if nat_eq(a4614, a4615) {
        Ok(0u64)
    } else {
        let a4622 = 2u64;
        let a4623 = {
            let a4620 = {
                let a4616 = v0;
                let a4617 = v1;
                nat_mul(a4616, a4617)?
            };
            let a4621 = {
                let a4618 = v2;
                let a4619 = v3;
                nat_mul(a4618, a4619)?
            };
            nat_mul(a4620, a4621)?
        };
        nat_mul(a4622, a4623)
    }
}

pub fn f222(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4628 = v3;
    let a4629 = {
        let a4626 = {
            let a4624 = f241(v2, v4)?;
            let a4625 = f242(v0, v1, f240(v2, v4)?)?;
            nat_mul(a4624, a4625)?
        };
        let a4627 = f242(v0, v1, f243(v2, v4)?)?;
        nat_add(a4626, a4627)?
    };
    nat_mul(a4628, a4629)
}

pub fn f223(v0: u64) -> R<u64> {
    let a4632 = 13u64;
    let a4633 = {
        let a4630 = 9u64;
        let a4631 = v0;
        nat_mul(a4630, a4631)?
    };
    nat_add(a4632, a4633)
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f186(v1, v2, v3, v4) {
        f244(v0)
    } else {
        f108(v0)
    }
}

pub fn f225(v0: u64) -> u64 {
    let a4634 = v0;
    let a4635 = 26u64;
    if nat_eq(a4634, a4635) {
        26u64
    } else {
        0u64
    }
}

pub fn f226(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4656 = {
        let a4650 = {
            let a4644 = {
                let a4638 = v1;
                let a4639 = if v5 {
                    f127(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4636, _, _, _) => {
                            h4636
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4637, _, _, _) => {
                            h4637
                        }
                    })
                };
                nat_mul(a4638, a4639)?
            };
            let a4645 = {
                let a4642 = v2;
                let a4643 = if v5 {
                    f127(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4640, _, _) => {
                            h4640
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4641, _, _) => {
                            h4641
                        }
                    })
                };
                nat_mul(a4642, a4643)?
            };
            nat_add(a4644, a4645)?
        };
        let a4651 = {
            let a4648 = v3;
            let a4649 = if v5 {
                f127(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4646, _) => {
                        h4646
                    }
                })
            } else {
                f128(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4647, _) => {
                        h4647
                    }
                })
            };
            nat_mul(a4648, a4649)?
        };
        nat_add(a4650, a4651)?
    };
    let a4657 = {
        let a4654 = v4;
        let a4655 = if v5 {
            f127(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4652) => {
                    h4652
                }
            })
        } else {
            f128(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4653) => {
                    h4653
                }
            })
        };
        nat_mul(a4654, a4655)?
    };
    nat_add(a4656, a4657)
}

pub fn f227(v0: u64, v1: u64) -> Option<u64> {
    let a4658 = v0;
    let a4659 = 2u64;
    if nat_eq(a4658, a4659) {
        Some(0u64)
    } else {
        let a4660 = v0;
        let a4661 = 5u64;
        if nat_eq(a4660, a4661) {
            Some(1u64)
        } else {
            let a4662 = v0;
            let a4663 = 6u64;
            if nat_eq(a4662, a4663) {
                Some(2u64)
            } else {
                let a4664 = v0;
                let a4665 = 7u64;
                if nat_eq(a4664, a4665) {
                    Some(3u64)
                } else {
                    let a4666 = v0;
                    let a4667 = 9u64;
                    if nat_eq(a4666, a4667) {
                        Some(4u64)
                    } else {
                        let a4668 = v0;
                        let a4669 = 8u64;
                        if nat_eq(a4668, a4669) {
                            Some(5u64)
                        } else {
                            let a4670 = v0;
                            let a4671 = 89u64;
                            if nat_eq(a4670, a4671) {
                                let a4672 = v1;
                                let a4673 = 2u64;
                                if nat_eq(a4672, a4673) {
                                    Some(6u64)
                                } else {
                                    let a4674 = v1;
                                    let a4675 = 0u64;
                                    if nat_eq(a4674, a4675) {
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

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4676 = v0;
    let a4677 = 30u64;
    if nat_eq(a4676, a4677) {
        let a4678 = v2;
        let a4679 = 0u64;
        if if if nat_eq(a4678, a4679) {
            true
        } else {
            let a4680 = v2;
            let a4681 = 1u64;
            nat_eq(a4680, a4681)
        } {
            let a4682 = v1;
            let a4683 = 0u64;
            if nat_eq(a4682, a4683) {
                true
            } else {
                f245(v1, v4)
            }
        } else {
            false
        } {
            f246(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f247(v1) {
            f247(v2)
        } else {
            false
        } {
            let a4684 = v1;
            let a4685 = 0u64;
            if if nat_eq(a4684, a4685) {
                true
            } else {
                let a4686 = v2;
                let a4687 = 0u64;
                nat_eq(a4686, a4687)
            } {
                true
            } else {
                let a4688 = v1;
                let a4689 = v2;
                nat_eq(a4688, a4689)
            }
        } else {
            false
        } {
            true
        } else {
            let a4690 = v1;
            let a4691 = 43u64;
            if nat_eq(a4690, a4691) {
                f247(v2)
            } else {
                false
            }
        } {
            f246(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f229(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4692 = v1;
    let a4693 = 0u64;
    if nat_eq(a4692, a4693) {
        Ok(Adt9::C4({
            let a4694 = v3;
            let a4695 = 0u64;
            if nat_eq(a4694, a4695) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f134(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4696 = v3;
            let a4697 = 0u64;
            if nat_eq(a4696, a4697) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f230(v0: u64, v1: u64) -> R<u64> {
    let a4698 = v0;
    let a4699 = f90(v1)?;
    let a4700 = 0u64;
    Ok(nat_quot(a4698, a4699, a4700))
}

pub fn f231(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f230(v0, v1)?;
    let v3: u64 = {
        let a4703 = {
            let a4701 = f204(v0, v1)?;
            let a4702 = 2u64;
            nat_sub(a4701, a4702)
        };
        let a4704 = 2u64;
        let a4705 = 0u64;
        nat_quot(a4703, a4704, a4705)
    };
    let v4: u64 = {
        let a4706 = v2;
        let a4707 = v1;
        nat_mul(a4706, a4707)?
    };
    if f207(v0, v1)? {
        let a4708 = v4;
        let a4709 = v3;
        nat_add(a4708, a4709)
    } else {
        let a4721 = {
            let a4717 = v4;
            let a4718 = {
                let a4715 = {
                    let a4710 = v1;
                    let a4711 = 1u64;
                    nat_sub(a4710, a4711)
                };
                let a4716 = {
                    let a4712 = v2;
                    let a4713 = v1;
                    let a4714 = 0u64;
                    nat_rem(a4712, a4713, a4714)
                };
                nat_mul(a4715, a4716)?
            };
            nat_sub(a4717, a4718)
        };
        let a4722 = {
            let a4719 = v3;
            let a4720 = v1;
            nat_mul(a4719, a4720)?
        };
        nat_add(a4721, a4722)
    }
}

pub fn f232(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4723 = v0;
    let a4724 = 0u64;
    if if if nat_eq(a4723, a4724) {
        true
    } else {
        let a4725 = v1;
        let a4726 = 0u64;
        nat_eq(a4725, a4726)
    } {
        true
    } else {
        let a4727 = v2;
        let a4728 = 0u64;
        nat_eq(a4727, a4728)
    } {
        Ok(0u64)
    } else {
        let a4731 = {
            let a4729 = v0;
            let a4730 = v1;
            nat_mul(a4729, a4730)?
        };
        let a4732 = v2;
        nat_mul(a4731, a4732)
    }
}

pub fn f233(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f99(match v1.clone() {
        Adt20::C0(h4733, _, _, _, _, _, _, _) => {
            h4733
        }
    })?;
    let v6: u64 = {
        let a4739 = match v1.clone() {
            Adt20::C0(_, h4734, _, _, _, _, _, _) => {
                h4734
            }
        };
        let a4740 = {
            let a4737 = match v1.clone() {
                Adt20::C0(_, _, h4735, _, _, _, _, _) => {
                    h4735
                }
            };
            let a4738 = match v1.clone() {
                Adt20::C0(_, _, _, h4736, _, _, _, _) => {
                    h4736
                }
            };
            nat_mul(a4737, a4738)?
        };
        nat_mul(a4739, a4740)?
    };
    let a4742 = v0;
    let a4743 = f36(match v1.clone() {
        Adt20::C0(h4741, _, _, _, _, _, _, _) => {
            h4741
        }
    }, v6)?;
    if nat_lt(a4742, a4743) {
        f248({
            let a4744 = v0;
            let a4745 = v5;
            let a4746 = 0u64;
            nat_rem(a4744, a4745, a4746)
        }, {
            let a4747 = v0;
            let a4748 = v5;
            let a4749 = 0u64;
            nat_quot(a4747, a4748, a4749)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f234(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a4750 = v6;
        let a4751 = 6u64;
        nat_sub(a4750, a4751)
    };
    let v9: Adt20 = f249(v3);
    let a4753 = v0;
    let a4754 = f182(match v1.clone() {
        Adt20::C0(h4752, _, _, _, _, _, _, _) => {
            h4752
        }
    }, v5, v8)?;
    if nat_lt(a4753, a4754) {
        let v10: u64 = f96(match v1.clone() {
            Adt20::C0(h4755, _, _, _, _, _, _, _) => {
                h4755
            }
        }, v5, v8)?;
        let a4756 = v0;
        let a4757 = v10;
        if nat_lt(a4756, a4757) {
            f250(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a4758 = v0;
                let a4759 = v10;
                nat_sub(a4758, a4759)
            };
            let v12: u64 = f223(match v1.clone() {
                Adt20::C0(h4760, _, _, _, _, _, _, _) => {
                    h4760
                }
            })?;
            f251({
                let a4761 = v11;
                let a4762 = v12;
                let a4763 = 0u64;
                nat_rem(a4761, a4762, a4763)
            }, {
                let a4764 = v11;
                let a4765 = v12;
                let a4766 = 0u64;
                nat_quot(a4764, a4765, a4766)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f235(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4767 = v5;
        let a4768 = 6u64;
        nat_sub(a4767, a4768)
    };
    let v8: bool = f176(match v1.clone() {
        Adt20::C0(_, _, _, _, h4769, _, _, _) => {
            h4769
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h4770, _, _, _) => {
            h4770
        }
    });
    let a4772 = v0;
    let a4773 = f175(v8, match v1.clone() {
        Adt20::C0(h4771, _, _, _, _, _, _, _) => {
            h4771
        }
    }, v4, v7)?;
    if nat_lt(a4772, a4773) {
        let v9: u64 = f96(match v1.clone() {
            Adt20::C0(h4774, _, _, _, _, _, _, _) => {
                h4774
            }
        }, v4, v7)?;
        let a4775 = v0;
        let a4776 = v9;
        if nat_lt(a4775, a4776) {
            f250(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a4777 = v0;
                let a4778 = v9;
                nat_sub(a4777, a4778)
            };
            let v11: u64 = f219(v8, match v1.clone() {
                Adt20::C0(h4779, _, _, _, _, _, _, _) => {
                    h4779
                }
            })?;
            f252({
                let a4780 = v10;
                let a4781 = v11;
                let a4782 = 0u64;
                nat_rem(a4780, a4781, a4782)
            }, {
                let a4783 = v10;
                let a4784 = v11;
                let a4785 = 0u64;
                nat_quot(a4783, a4784, a4785)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f236(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4786 = v5;
        let a4787 = 6u64;
        nat_sub(a4786, a4787)
    };
    let v8: Adt20 = f249(v2);
    let v9: u64 = f221(match v1.clone() {
        Adt20::C0(h4788, _, _, _, _, _, _, _) => {
            h4788
        }
    }, match v1.clone() {
        Adt20::C0(_, h4789, _, _, _, _, _, _) => {
            h4789
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4790, _, _, _, _, _) => {
            h4790
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4791, _, _, _, _) => {
            h4791
        }
    }, v7)?;
    let a4796 = v0;
    let a4797 = f181(match v1.clone() {
        Adt20::C0(h4792, _, _, _, _, _, _, _) => {
            h4792
        }
    }, match v1.clone() {
        Adt20::C0(_, h4793, _, _, _, _, _, _) => {
            h4793
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4794, _, _, _, _, _) => {
            h4794
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4795, _, _, _, _) => {
            h4795
        }
    }, v4, v7)?;
    if nat_lt(a4796, a4797) {
        let a4798 = v0;
        let a4799 = v9;
        if nat_lt(a4798, a4799) {
            f250(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f253({
                let a4800 = v0;
                let a4801 = v9;
                nat_sub(a4800, a4801)
            }, match v1.clone() {
                Adt20::C0(h4802, _, _, _, _, _, _, _) => {
                    h4802
                }
            }, match v1.clone() {
                Adt20::C0(_, h4803, _, _, _, _, _, _) => {
                    h4803
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h4804, _, _, _, _, _) => {
                    h4804
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h4805, _, _, _, _) => {
                    h4805
                }
            }, v4)?;
            f254(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f237(v0: bool, v1: u64) -> R<u64> {
    let a4808 = if v0 {
        5u64
    } else {
        4u64
    };
    let a4809 = {
        let a4806 = f255(v0);
        let a4807 = v1;
        nat_mul(a4806, a4807)?
    };
    nat_add(a4808, a4809)
}

pub fn f238(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4817 = {
            let a4810 = 13u64;
            let a4811 = f256(v1);
            nat_mul(a4810, a4811)?
        };
        let a4818 = {
            let a4815 = 4u64;
            let a4816 = {
                let a4812 = v1;
                let a4813 = 4u64;
                let a4814 = 0u64;
                nat_rem(a4812, a4813, a4814)
            };
            nat_mul(a4815, a4816)?
        };
        nat_add(a4817, a4818)
    } else {
        let a4819 = 3u64;
        let a4820 = v1;
        nat_mul(a4819, a4820)
    }
}

pub fn f239(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a4825 = {
        let a4823 = {
            let a4821 = v0;
            let a4822 = v1;
            nat_add(a4821, a4822)?
        };
        let a4824 = 1u64;
        nat_sub(a4823, a4824)
    };
    let a4826 = v1;
    let a4827 = 0u64;
    Ok(nat_quot(a4825, a4826, a4827))
}

pub fn f241(v0: u64, v1: u64) -> R<u64> {
    let a4828 = v0;
    let a4829 = f240(v0, v1)?;
    let a4830 = 0u64;
    Ok(nat_quot(a4828, a4829, a4830))
}

pub fn f242(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4831 = v2;
    let a4832 = 0u64;
    if nat_eq(a4831, a4832) {
        Ok(0u64)
    } else {
        let a4837 = 11u64;
        let a4838 = {
            let a4835 = {
                let a4833 = v1;
                let a4834 = v2;
                nat_mul(a4833, a4834)?
            };
            let a4836 = f257(v0)?;
            nat_mul(a4835, a4836)?
        };
        nat_add(a4837, a4838)
    }
}

pub fn f243(v0: u64, v1: u64) -> R<u64> {
    let a4839 = v0;
    let a4840 = f240(v0, v1)?;
    let a4841 = 0u64;
    Ok(nat_rem(a4839, a4840, a4841))
}

pub fn f244(v0: u64) -> bool {
    let m4842 = f110(v0);
    match m4842 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f245(v0: u64, v1: u64) -> bool {
    let a4843 = v0;
    let a4844 = 1u64;
    if nat_eq(a4843, a4844) {
        true
    } else {
        let v2: u64 = f258(v0);
        let a4845 = 0u64;
        let a4846 = v2;
        if nat_lt(a4845, a4846) {
            let a4850 = {
                let a4847 = v1;
                let a4848 = v2;
                let a4849 = 1u64;
                nat_rem(a4847, a4848, a4849)
            };
            let a4851 = 0u64;
            nat_eq(a4850, a4851)
        } else {
            false
        }
    }
}

pub fn f246(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4852 = v0;
    let a4853 = 28u64;
    if if if if nat_eq(a4852, a4853) {
        let a4854 = v1;
        let a4855 = 0u64;
        nat_eq(a4854, a4855)
    } else {
        false
    } {
        let a4856 = v2;
        let a4857 = 0u64;
        nat_eq(a4856, a4857)
    } else {
        false
    } {
        let a4858 = v3;
        let a4859 = 0u64;
        nat_eq(a4858, a4859)
    } else {
        false
    } {
        let a4860 = v4;
        let a4861 = v8;
        if if if if if if if if nat_eq(a4860, a4861) {
            let a4862 = v5;
            let a4863 = v12;
            nat_eq(a4862, a4863)
        } else {
            false
        } {
            let a4864 = v9;
            let a4865 = v13;
            nat_eq(a4864, a4865)
        } else {
            false
        } {
            let a4866 = v10;
            let a4867 = v14;
            nat_eq(a4866, a4867)
        } else {
            false
        } {
            let a4868 = v11;
            let a4869 = v15;
            nat_eq(a4868, a4869)
        } else {
            false
        } {
            let a4870 = 0u64;
            let a4871 = v6;
            nat_lt(a4870, a4871)
        } else {
            false
        } {
            let a4872 = 0u64;
            let a4873 = v7;
            nat_lt(a4872, a4873)
        } else {
            false
        } {
            let a4877 = {
                let a4874 = v10;
                let a4875 = v6;
                let a4876 = 1u64;
                nat_rem(a4874, a4875, a4876)
            };
            let a4878 = 0u64;
            if nat_eq(a4877, a4878) {
                let a4882 = {
                    let a4879 = v11;
                    let a4880 = v7;
                    let a4881 = 1u64;
                    nat_rem(a4879, a4880, a4881)
                };
                let a4883 = 0u64;
                nat_eq(a4882, a4883)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a4884 = v10;
                let a4885 = v6;
                let a4886 = 0u64;
                nat_quot(a4884, a4885, a4886)
            }, {
                let a4887 = v11;
                let a4888 = v7;
                let a4889 = 0u64;
                nat_quot(a4887, a4888, a4889)
            }, {
                let a4890 = v5;
                let a4891 = v4;
                nat_mul(a4890, a4891)?
            }, {
                let a4892 = v4;
                let a4893 = v9;
                nat_mul(a4892, a4893)?
            }, {
                let a4894 = v5;
                let a4895 = v9;
                nat_mul(a4894, a4895)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f247(v0: u64) -> bool {
    let a4896 = v0;
    let a4897 = 0u64;
    if if nat_eq(a4896, a4897) {
        true
    } else {
        let a4898 = v0;
        let a4899 = 1u64;
        nat_eq(a4898, a4899)
    } {
        true
    } else {
        let a4900 = v0;
        let a4901 = 30u64;
        nat_eq(a4900, a4901)
    }
}

pub fn f248(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h4902, _, _, _, _, _, _, _) => {
            h4902
        }
    };
    let v7: u64 = {
        let a4905 = 1u64;
        let a4906 = {
            let a4903 = 3u64;
            let a4904 = v6;
            nat_mul(a4903, a4904)?
        };
        nat_add(a4905, a4906)?
    };
    let a4907 = v0;
    let a4908 = 0u64;
    if nat_eq(a4907, a4908) {
        Ok(f259(0u64))
    } else {
        let a4909 = v0;
        let a4910 = v7;
        if nat_lt(a4909, a4910) {
            let v8: u64 = {
                let a4911 = v0;
                let a4912 = 1u64;
                nat_sub(a4911, a4912)
            };
            let v9: u64 = {
                let a4918 = {
                    let a4913 = v1;
                    let a4914 = v6;
                    nat_mul(a4913, a4914)?
                };
                let a4919 = {
                    let a4915 = v8;
                    let a4916 = 3u64;
                    let a4917 = 0u64;
                    nat_quot(a4915, a4916, a4917)
                };
                nat_add(a4918, a4919)?
            };
            let v10: u64 = f260(v9, v2)?;
            let v11: u64 = {
                let a4920 = v8;
                let a4921 = 3u64;
                let a4922 = 0u64;
                nat_rem(a4920, a4921, a4922)
            };
            let a4923 = v11;
            let a4924 = 0u64;
            if nat_eq(a4923, a4924) {
                Ok(f261(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a4925 = v11;
                let a4926 = 1u64;
                if nat_eq(a4925, a4926) {
                    Ok(f261(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f261(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a4927 = v0;
            let a4928 = v7;
            if nat_eq(a4927, a4928) {
                Ok(f261(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4931 = v0;
                let a4932 = {
                    let a4929 = v7;
                    let a4930 = 1u64;
                    nat_add(a4929, a4930)?
                };
                if nat_eq(a4931, a4932) {
                    Ok(f261(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4935 = v0;
                    let a4936 = {
                        let a4933 = v7;
                        let a4934 = 2u64;
                        nat_add(a4933, a4934)?
                    };
                    if nat_eq(a4935, a4936) {
                        Ok(f261(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a4939 = v0;
                        let a4940 = {
                            let a4937 = v7;
                            let a4938 = 3u64;
                            nat_add(a4937, a4938)?
                        };
                        if nat_eq(a4939, a4940) {
                            let a4941 = v5;
                            let a4942 = 1u64;
                            if if nat_eq(a4941, a4942) {
                                true
                            } else {
                                let a4943 = v5;
                                let a4944 = 2u64;
                                nat_eq(a4943, a4944)
                            } {
                                Ok(f261(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f261(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a4951 = {
                                    let a4945 = v1;
                                    let a4946 = v6;
                                    nat_mul(a4945, a4946)?
                                };
                                let a4952 = {
                                    let a4949 = v0;
                                    let a4950 = {
                                        let a4947 = v7;
                                        let a4948 = 4u64;
                                        nat_add(a4947, a4948)?
                                    };
                                    nat_sub(a4949, a4950)
                                };
                                nat_add(a4951, a4952)?
                            };
                            Ok(f261(10u64, 0u64, f260(v12, v2)?, 1u64, 2u64, 3u64, f260(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f249(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h4953, _, _, _, _, _, _, _) => {
            h4953
        }
    }, match v0.clone() {
        Adt20::C0(_, h4954, _, _, _, _, _, _) => {
            h4954
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h4955, _, _, _, _, _) => {
            h4955
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h4956, _, _, _, _) => {
            h4956
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h4957, _, _) => {
            h4957
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h4958, _) => {
            h4958
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h4959) => {
            h4959
        }
    })
}

pub fn f250(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a4966 = v3;
        let a4967 = {
            let a4963 = f260({
                let a4960 = v0;
                let a4961 = 2u64;
                let a4962 = 0u64;
                nat_quot(a4960, a4961, a4962)
            }, v1)?;
            let a4964 = v2;
            let a4965 = 0u64;
            nat_quot(a4963, a4964, a4965)
        };
        nat_add(a4966, a4967)?
    };
    let a4971 = {
        let a4968 = v0;
        let a4969 = 2u64;
        let a4970 = 0u64;
        nat_rem(a4968, a4969, a4970)
    };
    let a4972 = 0u64;
    if nat_eq(a4971, a4972) {
        Ok(f261(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4973 = v4;
        let a4974 = 0u64;
        if nat_eq(a4973, a4974) {
            Ok(f261(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f251(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f262(match v2.clone() {
        Adt20::C0(h4975, _, _, _, _, _, _, _) => {
            h4975
        }
    })?;
    let v7: u64 = f263(match v2.clone() {
        Adt20::C0(h4976, _, _, _, _, _, _, _) => {
            h4976
        }
    })?;
    let a4977 = v0;
    let a4978 = 2u64;
    if nat_lt(a4977, a4978) {
        Ok(f259(v0))
    } else {
        let a4979 = v0;
        let a4980 = v6;
        if nat_lt(a4979, a4980) {
            f264({
                let a4981 = v0;
                let a4982 = 2u64;
                nat_sub(a4981, a4982)
            }, v1, v2, v3)
        } else {
            let a4983 = v0;
            let a4984 = v7;
            if nat_lt(a4983, a4984) {
                Ok(f265({
                    let a4985 = v0;
                    let a4986 = v6;
                    nat_sub(a4985, a4986)
                }, match v2 {
                    Adt20::C0(h4987, _, _, _, _, _, _, _) => {
                        h4987
                    }
                }, v5))
            } else {
                let a4989 = v0;
                let a4990 = f223(match v2.clone() {
                    Adt20::C0(h4988, _, _, _, _, _, _, _) => {
                        h4988
                    }
                })?;
                if nat_lt(a4989, a4990) {
                    f266({
                        let a4991 = v0;
                        let a4992 = v7;
                        nat_sub(a4991, a4992)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f252(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f237(v5, match v2.clone() {
        Adt20::C0(h4993, _, _, _, _, _, _, _) => {
            h4993
        }
    })?;
    let v7: u64 = {
        let a4995 = v6;
        let a4996 = f238(v5, match v2.clone() {
            Adt20::C0(h4994, _, _, _, _, _, _, _) => {
                h4994
            }
        })?;
        nat_add(a4995, a4996)?
    };
    let v8: u64 = {
        let a4997 = v7;
        let a4998 = f239(v5);
        nat_add(a4997, a4998)?
    };
    let a4999 = v0;
    let a5000 = v6;
    if nat_lt(a4999, a5000) {
        f267(v0, v1, v2, v5)
    } else {
        let a5001 = v0;
        let a5002 = v7;
        if nat_lt(a5001, a5002) {
            f268({
                let a5003 = v0;
                let a5004 = v6;
                nat_sub(a5003, a5004)
            }, v1, v2, v3, v5)
        } else {
            let a5005 = v0;
            let a5006 = v8;
            if nat_lt(a5005, a5006) {
                Ok(f269({
                    let a5007 = v0;
                    let a5008 = v7;
                    nat_sub(a5007, a5008)
                }, match v2 {
                    Adt20::C0(h5009, _, _, _, _, _, _, _) => {
                        h5009
                    }
                }, v4, v5))
            } else {
                let a5013 = v0;
                let a5014 = {
                    let a5011 = v8;
                    let a5012 = match v2.clone() {
                        Adt20::C0(h5010, _, _, _, _, _, _, _) => {
                            h5010
                        }
                    };
                    nat_add(a5011, a5012)?
                };
                if nat_lt(a5013, a5014) {
                    let v9: u64 = f260({
                        let a5020 = {
                            let a5016 = v1;
                            let a5017 = match v2 {
                                Adt20::C0(h5015, _, _, _, _, _, _, _) => {
                                    h5015
                                }
                            };
                            nat_mul(a5016, a5017)?
                        };
                        let a5021 = {
                            let a5018 = v0;
                            let a5019 = v8;
                            nat_sub(a5018, a5019)
                        };
                        nat_add(a5020, a5021)?
                    }, v3)?;
                    Ok(f261(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f253(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f240(v3, v5)?;
    let v7: u64 = f242(v1, v2, v6)?;
    let v8: u64 = {
        let a5024 = {
            let a5022 = f241(v3, v5)?;
            let a5023 = v4;
            nat_mul(a5022, a5023)?
        };
        let a5025 = v7;
        nat_mul(a5024, a5025)?
    };
    let a5026 = v0;
    let a5027 = v8;
    if nat_lt(a5026, a5027) {
        let v9: u64 = {
            let a5028 = v0;
            let a5029 = v7;
            let a5030 = 0u64;
            nat_quot(a5028, a5029, a5030)
        };
        Ok(Adt22::C0({
            let a5034 = {
                let a5031 = v9;
                let a5032 = v4;
                let a5033 = 0u64;
                nat_quot(a5031, a5032, a5033)
            };
            let a5035 = v6;
            nat_mul(a5034, a5035)?
        }, v6, {
            let a5036 = v9;
            let a5037 = v4;
            let a5038 = 0u64;
            nat_rem(a5036, a5037, a5038)
        }, {
            let a5039 = v0;
            let a5040 = v7;
            let a5041 = 0u64;
            nat_rem(a5039, a5040, a5041)
        }))
    } else {
        let v10: u64 = {
            let a5042 = v0;
            let a5043 = v8;
            nat_sub(a5042, a5043)
        };
        let v11: u64 = f242(v1, v2, f243(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5044 = f241(v3, v5)?;
            let a5045 = v6;
            nat_mul(a5044, a5045)?
        }, f243(v3, v5)?, {
            let a5046 = v10;
            let a5047 = v11;
            let a5048 = 0u64;
            nat_quot(a5046, a5047, a5048)
        }, {
            let a5049 = v10;
            let a5050 = v11;
            let a5051 = 0u64;
            nat_rem(a5049, a5050, a5051)
        }))
    }
}

pub fn f254(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5052) => {
            h5052
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5053, _, _, _, _, _, _, _) => {
            h5053
        }
    };
    let v6: u64 = {
        let a5056 = match v0.clone() {
            Adt22::C0(_, h5054, _, _) => {
                h5054
            }
        };
        let a5057 = match v1.clone() {
            Adt20::C0(_, h5055, _, _, _, _, _, _) => {
                h5055
            }
        };
        nat_mul(a5056, a5057)?
    };
    let v7: u64 = {
        let a5058 = v6;
        let a5059 = v5;
        nat_mul(a5058, a5059)?
    };
    let v8: u64 = {
        let a5062 = {
            let a5060 = 2u64;
            let a5061 = v5;
            nat_mul(a5060, a5061)?
        };
        let a5063 = 2u64;
        nat_add(a5062, a5063)?
    };
    let v9: u64 = {
        let a5066 = {
            let a5064 = 4u64;
            let a5065 = v5;
            nat_mul(a5064, a5065)?
        };
        let a5067 = 2u64;
        nat_add(a5066, a5067)?
    };
    let v10: u64 = {
        let a5070 = 1u64;
        let a5071 = {
            let a5068 = v6;
            let a5069 = v8;
            nat_mul(a5068, a5069)?
        };
        nat_add(a5070, a5071)?
    };
    let v11: u64 = {
        let a5072 = v10;
        let a5073 = 4u64;
        nat_add(a5072, a5073)?
    };
    let v12: u64 = {
        let a5076 = v11;
        let a5077 = {
            let a5074 = v6;
            let a5075 = v9;
            nat_mul(a5074, a5075)?
        };
        nat_add(a5076, a5077)?
    };
    let v13: u64 = {
        let a5078 = v12;
        let a5079 = 6u64;
        nat_add(a5078, a5079)?
    };
    let a5080 = v4;
    let a5081 = 0u64;
    if nat_eq(a5080, a5081) {
        Ok(f259(0u64))
    } else {
        let a5082 = v4;
        let a5083 = v10;
        if nat_lt(a5082, a5083) {
            let v14: u64 = {
                let a5084 = v4;
                let a5085 = 1u64;
                nat_sub(a5084, a5085)
            };
            f270(false, {
                let a5086 = v14;
                let a5087 = v8;
                let a5088 = 0u64;
                nat_rem(a5086, a5087, a5088)
            }, {
                let a5089 = v14;
                let a5090 = v8;
                let a5091 = 0u64;
                nat_quot(a5089, a5090, a5091)
            }, v0, v1, v2)
        } else {
            let a5092 = v4;
            let a5093 = v11;
            if nat_lt(a5092, a5093) {
                Ok(f271({
                    let a5094 = v4;
                    let a5095 = v10;
                    nat_sub(a5094, a5095)
                }, v7))
            } else {
                let a5096 = v4;
                let a5097 = v12;
                if nat_lt(a5096, a5097) {
                    let v15: u64 = {
                        let a5098 = v4;
                        let a5099 = v11;
                        nat_sub(a5098, a5099)
                    };
                    f270(true, {
                        let a5100 = v15;
                        let a5101 = v9;
                        let a5102 = 0u64;
                        nat_rem(a5100, a5101, a5102)
                    }, {
                        let a5103 = v15;
                        let a5104 = v9;
                        let a5105 = 0u64;
                        nat_quot(a5103, a5104, a5105)
                    }, v0, v1, v2)
                } else {
                    let a5106 = v4;
                    let a5107 = v13;
                    if nat_lt(a5106, a5107) {
                        Ok(f272({
                            let a5108 = v4;
                            let a5109 = v12;
                            nat_sub(a5108, a5109)
                        }, v7, v3))
                    } else {
                        let a5112 = v4;
                        let a5113 = {
                            let a5110 = v13;
                            let a5111 = v7;
                            nat_add(a5110, a5111)?
                        };
                        if nat_lt(a5112, a5113) {
                            let v16: u64 = {
                                let a5114 = v4;
                                let a5115 = v13;
                                nat_sub(a5114, a5115)
                            };
                            let v17: u64 = {
                                let a5116 = v16;
                                let a5117 = v5;
                                let a5118 = 0u64;
                                nat_quot(a5116, a5117, a5118)
                            };
                            let v18: u64 = {
                                let a5124 = {
                                    let a5119 = f273(v17, v0, v1)?;
                                    let a5120 = v5;
                                    nat_mul(a5119, a5120)?
                                };
                                let a5125 = {
                                    let a5121 = v16;
                                    let a5122 = v5;
                                    let a5123 = 0u64;
                                    nat_rem(a5121, a5122, a5123)
                                };
                                nat_add(a5124, a5125)?
                            };
                            let v19: u64 = f260(v18, v2)?;
                            Ok(f261(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f255(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f256(v0: u64) -> u64 {
    let a5126 = v0;
    let a5127 = 4u64;
    let a5128 = 0u64;
    nat_quot(a5126, a5127, a5128)
}

pub fn f257(v0: u64) -> R<u64> {
    let a5131 = {
        let a5129 = 7u64;
        let a5130 = v0;
        nat_mul(a5129, a5130)?
    };
    let a5132 = 4u64;
    nat_add(a5131, a5132)
}

pub fn f258(v0: u64) -> u64 {
    if f108(v0) {
        let m5133 = f110(v0);
        match m5133 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5135 = 1u64;
                let a5136 = match v1.clone() {
                    Adt18::C0(_, h5134, _) => {
                        h5134
                    }
                };
                if if nat_lt(a5135, a5136) {
                    true
                } else {
                    let a5137 = v0;
                    let a5138 = 43u64;
                    nat_eq(a5137, a5138)
                } {
                    match v1 {
                        Adt18::C0(_, h5139, _) => {
                            h5139
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

pub fn f259(v0: u64) -> Adt19 {
    f261(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f260(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5141 = v0;
        let a5142 = match v1.clone() {
            Adt20::C0(h5140, _, _, _, _, _, _, _) => {
                h5140
            }
        };
        let a5143 = 0u64;
        nat_quot(a5141, a5142, a5143)
    };
    let v3: u64 = {
        let a5145 = v2;
        let a5146 = match v1.clone() {
            Adt20::C0(_, h5144, _, _, _, _, _, _) => {
                h5144
            }
        };
        let a5147 = 0u64;
        nat_quot(a5145, a5146, a5147)
    };
    f126({
        let a5149 = v0;
        let a5150 = match v1.clone() {
            Adt20::C0(h5148, _, _, _, _, _, _, _) => {
                h5148
            }
        };
        let a5151 = 0u64;
        nat_rem(a5149, a5150, a5151)
    }, {
        let a5153 = v2;
        let a5154 = match v1.clone() {
            Adt20::C0(_, h5152, _, _, _, _, _, _) => {
                h5152
            }
        };
        let a5155 = 0u64;
        nat_rem(a5153, a5154, a5155)
    }, {
        let a5157 = v3;
        let a5158 = match v1.clone() {
            Adt20::C0(_, _, h5156, _, _, _, _, _) => {
                h5156
            }
        };
        let a5159 = 0u64;
        nat_rem(a5157, a5158, a5159)
    }, {
        let a5161 = v3;
        let a5162 = match v1.clone() {
            Adt20::C0(_, _, h5160, _, _, _, _, _) => {
                h5160
            }
        };
        let a5163 = 0u64;
        nat_quot(a5161, a5162, a5163)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5164, _, _, _) => {
            h5164
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5165, _, _) => {
            h5165
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5166, _) => {
            h5166
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5167) => {
            h5167
        }
    })
}

pub fn f261(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5168 = v0;
    let a5169 = 4u64;
    if if if nat_le(a5168, a5169) {
        true
    } else {
        let a5170 = 9u64;
        let a5171 = v0;
        if nat_le(a5170, a5171) {
            let a5172 = v0;
            let a5173 = 12u64;
            nat_le(a5172, a5173)
        } else {
            false
        }
    } {
        true
    } else {
        let a5174 = v0;
        let a5175 = 16u64;
        if if nat_eq(a5174, a5175) {
            true
        } else {
            let a5176 = v0;
            let a5177 = 19u64;
            nat_eq(a5176, a5177)
        } {
            true
        } else {
            let a5178 = v0;
            let a5179 = 21u64;
            if nat_eq(a5178, a5179) {
                true
            } else {
                let a5180 = v0;
                let a5181 = 22u64;
                nat_eq(a5180, a5181)
            }
        }
    } {
        Adt19::C1({
            let a5182 = v0;
            let a5183 = 4u64;
            if if nat_lt(a5182, a5183) {
                true
            } else {
                let a5184 = 21u64;
                let a5185 = v0;
                nat_le(a5184, a5185)
            } {
                0u64
            } else {
                let a5186 = v0;
                let a5187 = 16u64;
                if if nat_eq(a5186, a5187) {
                    true
                } else {
                    let a5188 = v0;
                    let a5189 = 19u64;
                    nat_eq(a5188, a5189)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5190 = v0;
            let a5191 = 4u64;
            if nat_lt(a5190, a5191) {
                v0
            } else {
                let a5192 = v0;
                let a5193 = 4u64;
                if if nat_eq(a5192, a5193) {
                    true
                } else {
                    let a5194 = v0;
                    let a5195 = 16u64;
                    nat_eq(a5194, a5195)
                } {
                    0u64
                } else {
                    let a5196 = v0;
                    let a5197 = 19u64;
                    if nat_eq(a5196, a5197) {
                        3u64
                    } else {
                        let a5198 = 21u64;
                        let a5199 = v0;
                        if nat_le(a5198, a5199) {
                            let a5200 = v0;
                            let a5201 = 17u64;
                            nat_sub(a5200, a5201)
                        } else {
                            let a5202 = v0;
                            let a5203 = 8u64;
                            nat_sub(a5202, a5203)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f262(v0: u64) -> R<u64> {
    let a5206 = 2u64;
    let a5207 = {
        let a5204 = 6u64;
        let a5205 = v0;
        nat_mul(a5204, a5205)?
    };
    nat_add(a5206, a5207)
}

pub fn f263(v0: u64) -> R<u64> {
    let a5208 = f262(v0)?;
    let a5209 = 11u64;
    nat_add(a5208, a5209)
}

pub fn f264(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5210 = v0;
        let a5211 = 6u64;
        let a5212 = 0u64;
        nat_rem(a5210, a5211, a5212)
    };
    let v5: u64 = {
        let a5219 = {
            let a5214 = v1;
            let a5215 = match v2.clone() {
                Adt20::C0(h5213, _, _, _, _, _, _, _) => {
                    h5213
                }
            };
            nat_mul(a5214, a5215)?
        };
        let a5220 = {
            let a5216 = v0;
            let a5217 = 6u64;
            let a5218 = 0u64;
            nat_quot(a5216, a5217, a5218)
        };
        nat_add(a5219, a5220)?
    };
    let v6: u64 = f260(v5, v3)?;
    let v7: u64 = f260(v5, v2)?;
    let a5221 = v4;
    let a5222 = 0u64;
    if nat_eq(a5221, a5222) {
        Ok(f261(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5223 = v4;
        let a5224 = 1u64;
        if nat_eq(a5223, a5224) {
            Ok(f261(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5225 = v4;
            let a5226 = 2u64;
            if nat_eq(a5225, a5226) {
                Ok(f261(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5227 = v4;
                let a5228 = 3u64;
                if nat_eq(a5227, a5228) {
                    Ok(f261(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5229 = v4;
                    let a5230 = 4u64;
                    if nat_eq(a5229, a5230) {
                        Ok(f261(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f261(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5231 = v0;
    let a5232 = 0u64;
    if nat_eq(a5231, a5232) {
        f261(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5233 = v0;
        let a5234 = 1u64;
        if nat_eq(a5233, a5234) {
            f261(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5235 = v0;
            let a5236 = 2u64;
            if nat_eq(a5235, a5236) {
                f261(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5237 = v0;
                let a5238 = 3u64;
                if nat_eq(a5237, a5238) {
                    f261(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5239 = v0;
                    let a5240 = 4u64;
                    if nat_eq(a5239, a5240) {
                        f261(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5241 = v0;
                        let a5242 = 5u64;
                        if nat_eq(a5241, a5242) {
                            f261(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5243 = v0;
                            let a5244 = 6u64;
                            if nat_eq(a5243, a5244) {
                                f261(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5245 = v0;
                                let a5246 = 7u64;
                                if nat_eq(a5245, a5246) {
                                    f261(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5247 = v0;
                                    let a5248 = 8u64;
                                    if nat_eq(a5247, a5248) {
                                        f261(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5249 = v0;
                                        let a5250 = 9u64;
                                        if nat_eq(a5249, a5250) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5251 = v0;
                                            let a5252 = 10u64;
                                            if nat_eq(a5251, a5252) {
                                                f261(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f266(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5253 = v0;
        let a5254 = 3u64;
        let a5255 = 0u64;
        nat_rem(a5253, a5254, a5255)
    };
    let v6: u64 = {
        let a5262 = {
            let a5257 = v1;
            let a5258 = match v2.clone() {
                Adt20::C0(h5256, _, _, _, _, _, _, _) => {
                    h5256
                }
            };
            nat_mul(a5257, a5258)?
        };
        let a5263 = {
            let a5259 = v0;
            let a5260 = 3u64;
            let a5261 = 0u64;
            nat_quot(a5259, a5260, a5261)
        };
        nat_add(a5262, a5263)?
    };
    let v7: u64 = f260(v6, v3)?;
    let v8: u64 = f260(v6, v2)?;
    let v9: u64 = f260(v6, v4)?;
    let a5264 = v5;
    let a5265 = 0u64;
    if nat_eq(a5264, a5265) {
        Ok(f261(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5266 = v5;
        let a5267 = 1u64;
        if nat_eq(a5266, a5267) {
            Ok(f261(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f261(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f267(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5268, _, _, _, _, _, _, _) => {
            h5268
        }
    };
    let v5: u64 = {
        let a5271 = 1u64;
        let a5272 = {
            let a5269 = v4;
            let a5270 = f255(v3);
            nat_mul(a5269, a5270)?
        };
        nat_add(a5271, a5272)?
    };
    let a5273 = v0;
    let a5274 = 0u64;
    if nat_eq(a5273, a5274) {
        Ok(f259(0u64))
    } else {
        let a5275 = v0;
        let a5276 = v5;
        if nat_lt(a5275, a5276) {
            let v6: u64 = {
                let a5277 = v0;
                let a5278 = 1u64;
                nat_sub(a5277, a5278)
            };
            let v7: u64 = f260({
                let a5284 = {
                    let a5279 = v1;
                    let a5280 = v4;
                    nat_mul(a5279, a5280)?
                };
                let a5285 = {
                    let a5281 = v6;
                    let a5282 = f255(v3);
                    let a5283 = 0u64;
                    nat_quot(a5281, a5282, a5283)
                };
                nat_add(a5284, a5285)?
            }, v2)?;
            if v3 {
                let a5289 = {
                    let a5286 = v6;
                    let a5287 = 2u64;
                    let a5288 = 0u64;
                    nat_rem(a5286, a5287, a5288)
                };
                let a5290 = 0u64;
                if nat_eq(a5289, a5290) {
                    Ok(f261(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f261(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f261(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5291 = v0;
                let a5292 = v5;
                nat_sub(a5291, a5292)
            };
            let a5293 = v8;
            let a5294 = 0u64;
            if nat_eq(a5293, a5294) {
                Ok(f261(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5295 = v8;
                    let a5296 = 1u64;
                    nat_eq(a5295, a5296)
                } else {
                    false
                } {
                    Ok(f261(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5297 = v8;
                    let a5298 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5297, a5298) {
                        Ok(f261(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5299 = v8;
                        let a5300 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5299, a5300) {
                            Ok(f259(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f268(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5302 = 13u64;
        let a5303 = f256(match v2.clone() {
            Adt20::C0(h5301, _, _, _, _, _, _, _) => {
                h5301
            }
        });
        nat_mul(a5302, a5303)?
    } else {
        0u64
    };
    let a5304 = v0;
    let a5305 = v5;
    if nat_lt(a5304, a5305) {
        f274({
            let a5306 = v0;
            let a5307 = 13u64;
            let a5308 = 0u64;
            nat_rem(a5306, a5307, a5308)
        }, v1, {
            let a5309 = v0;
            let a5310 = 13u64;
            let a5311 = 0u64;
            nat_quot(a5309, a5310, a5311)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5312 = v0;
            let a5313 = v5;
            nat_sub(a5312, a5313)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f275({
            let a5314 = v6;
            let a5315 = v7;
            let a5316 = 0u64;
            nat_rem(a5314, a5315, a5316)
        }, {
            let a5326 = {
                let a5321 = {
                    let a5318 = v1;
                    let a5319 = match v2.clone() {
                        Adt20::C0(h5317, _, _, _, _, _, _, _) => {
                            h5317
                        }
                    };
                    nat_mul(a5318, a5319)?
                };
                let a5322 = if v4 {
                    f276(match v2.clone() {
                        Adt20::C0(h5320, _, _, _, _, _, _, _) => {
                            h5320
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5321, a5322)?
            };
            let a5327 = {
                let a5323 = v6;
                let a5324 = v7;
                let a5325 = 0u64;
                nat_quot(a5323, a5324, a5325)
            };
            nat_add(a5326, a5327)?
        }, v2, v3, v4)
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5328 = v0;
    let a5329 = 0u64;
    if nat_eq(a5328, a5329) {
        f261(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5330 = v0;
        let a5331 = 1u64;
        if nat_eq(a5330, a5331) {
            f261(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5332 = v0;
                let a5333 = 2u64;
                nat_eq(a5332, a5333)
            } else {
                false
            } {
                f261(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5334 = v0;
                let a5335 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5334, a5335) {
                    f261(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5336 = v0;
                    let a5337 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5336, a5337) {
                        f261(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5338 = v0;
                        let a5339 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5338, a5339) {
                            f261(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5343 = 1u64;
        let a5344 = {
            let a5341 = match v4.clone() {
                Adt20::C0(h5340, _, _, _, _, _, _, _) => {
                    h5340
                }
            };
            let a5342 = v6;
            nat_mul(a5341, a5342)?
        };
        nat_add(a5343, a5344)?
    };
    let a5345 = v1;
    let a5346 = 0u64;
    if nat_eq(a5345, a5346) {
        Ok(f259(1u64))
    } else {
        let a5347 = v1;
        let a5348 = v7;
        if nat_lt(a5347, a5348) {
            let v8: u64 = {
                let a5349 = v1;
                let a5350 = 1u64;
                nat_sub(a5349, a5350)
            };
            let v9: u64 = {
                let a5357 = {
                    let a5352 = f273(v2, v3, v4.clone())?;
                    let a5353 = match v4.clone() {
                        Adt20::C0(h5351, _, _, _, _, _, _, _) => {
                            h5351
                        }
                    };
                    nat_mul(a5352, a5353)?
                };
                let a5358 = {
                    let a5354 = v8;
                    let a5355 = v6;
                    let a5356 = 0u64;
                    nat_quot(a5354, a5355, a5356)
                };
                nat_add(a5357, a5358)?
            };
            let v10: u64 = f260(v9, v4)?;
            let v11: u64 = f260(v9, v5)?;
            let v12: u64 = {
                let a5359 = v8;
                let a5360 = v6;
                let a5361 = 0u64;
                nat_rem(a5359, a5360, a5361)
            };
            if v0 {
                let a5362 = v12;
                let a5363 = 0u64;
                if nat_eq(a5362, a5363) {
                    Ok(f261(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5364 = v12;
                    let a5365 = 1u64;
                    if nat_eq(a5364, a5365) {
                        Ok(f261(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5366 = v12;
                        let a5367 = 2u64;
                        if nat_eq(a5366, a5367) {
                            Ok(f261(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f261(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5368 = v12;
                let a5369 = 0u64;
                if nat_eq(a5368, a5369) {
                    Ok(f261(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f261(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5370 = v1;
            let a5371 = v7;
            if nat_eq(a5370, a5371) {
                Ok(f261(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f271(v0: u64, v1: u64) -> Adt19 {
    let a5372 = v0;
    let a5373 = 0u64;
    if nat_eq(a5372, a5373) {
        f261(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5374 = v0;
        let a5375 = 1u64;
        if nat_eq(a5374, a5375) {
            f261(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5376 = v0;
            let a5377 = 2u64;
            if nat_eq(a5376, a5377) {
                f261(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5378 = v0;
                let a5379 = 3u64;
                if nat_eq(a5378, a5379) {
                    f259(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f272(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5380 = v0;
    let a5381 = 0u64;
    if nat_eq(a5380, a5381) {
        f261(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5382 = v0;
        let a5383 = 1u64;
        if nat_eq(a5382, a5383) {
            f261(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5384 = v0;
            let a5385 = 2u64;
            if nat_eq(a5384, a5385) {
                f261(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5386 = v0;
                let a5387 = 3u64;
                if nat_eq(a5386, a5387) {
                    f261(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5388 = v0;
                    let a5389 = 4u64;
                    if nat_eq(a5388, a5389) {
                        f261(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5390 = v0;
                        let a5391 = 5u64;
                        if nat_eq(a5390, a5391) {
                            f261(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f273(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5402 = {
        let a5400 = {
            let a5397 = {
                let a5394 = match v1.clone() {
                    Adt22::C0(_, _, h5392, _) => {
                        h5392
                    }
                };
                let a5395 = match v2.clone() {
                    Adt20::C0(_, _, h5393, _, _, _, _, _) => {
                        h5393
                    }
                };
                nat_mul(a5394, a5395)?
            };
            let a5398 = match v1 {
                Adt22::C0(h5396, _, _, _) => {
                    h5396
                }
            };
            nat_add(a5397, a5398)?
        };
        let a5401 = match v2 {
            Adt20::C0(_, h5399, _, _, _, _, _, _) => {
                h5399
            }
        };
        nat_mul(a5400, a5401)?
    };
    let a5403 = v0;
    nat_add(a5402, a5403)
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5411 = {
            let a5409 = {
                let a5405 = v1;
                let a5406 = match v3.clone() {
                    Adt20::C0(h5404, _, _, _, _, _, _, _) => {
                        h5404
                    }
                };
                nat_mul(a5405, a5406)?
            };
            let a5410 = {
                let a5407 = v2;
                let a5408 = 4u64;
                nat_mul(a5407, a5408)?
            };
            nat_add(a5409, a5410)?
        };
        let a5412 = f277(v0);
        nat_add(a5411, a5412)?
    };
    let v6: u64 = f260(v5, v3)?;
    let v7: u64 = f260(v5, v4)?;
    let a5413 = v0;
    let a5414 = 0u64;
    if if if if nat_eq(a5413, a5414) {
        true
    } else {
        let a5415 = v0;
        let a5416 = 2u64;
        nat_eq(a5415, a5416)
    } {
        true
    } else {
        let a5417 = v0;
        let a5418 = 5u64;
        nat_eq(a5417, a5418)
    } {
        true
    } else {
        let a5419 = v0;
        let a5420 = 7u64;
        nat_eq(a5419, a5420)
    } {
        Ok(f261(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5421 = v0;
        let a5422 = 1u64;
        if if if if nat_eq(a5421, a5422) {
            true
        } else {
            let a5423 = v0;
            let a5424 = 3u64;
            nat_eq(a5423, a5424)
        } {
            true
        } else {
            let a5425 = v0;
            let a5426 = 6u64;
            nat_eq(a5425, a5426)
        } {
            true
        } else {
            let a5427 = v0;
            let a5428 = 8u64;
            nat_eq(a5427, a5428)
        } {
            Ok(f261(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5429 = v0;
                let a5430 = 1u64;
                if nat_eq(a5429, a5430) {
                    4u64
                } else {
                    let a5431 = v0;
                    let a5432 = 6u64;
                    if nat_eq(a5431, a5432) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5433 = v0;
            let a5434 = 4u64;
            if nat_eq(a5433, a5434) {
                Ok(f261(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5435 = v0;
                let a5436 = 9u64;
                if nat_eq(a5435, a5436) {
                    Ok(f261(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5437 = v0;
                    let a5438 = 10u64;
                    if nat_eq(a5437, a5438) {
                        Ok(f261(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5439 = v0;
                        let a5440 = 11u64;
                        if nat_eq(a5439, a5440) {
                            Ok(f261(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5441 = v0;
                            let a5442 = 12u64;
                            if nat_eq(a5441, a5442) {
                                Ok(f261(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f275(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f260(v1, v2)?;
    let v6: u64 = f260(v1, v3)?;
    let a5443 = v0;
    let a5444 = 0u64;
    if nat_eq(a5443, a5444) {
        Ok(f261(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5445 = v0;
        let a5446 = 1u64;
        if nat_eq(a5445, a5446) {
            Ok(f261(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5447 = v0;
                let a5448 = 2u64;
                if nat_eq(a5447, a5448) {
                    Ok(f261(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5449 = v0;
                    let a5450 = 3u64;
                    if nat_eq(a5449, a5450) {
                        Ok(f261(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5451 = v0;
                let a5452 = 2u64;
                if nat_eq(a5451, a5452) {
                    Ok(f261(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f276(v0: u64) -> R<u64> {
    let a5453 = 4u64;
    let a5454 = f256(v0);
    nat_mul(a5453, a5454)
}

pub fn f277(v0: u64) -> u64 {
    let a5455 = v0;
    let a5456 = 5u64;
    if nat_lt(a5455, a5456) {
        let a5457 = v0;
        let a5458 = 2u64;
        if nat_lt(a5457, a5458) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5459 = v0;
        let a5460 = 7u64;
        if nat_lt(a5459, a5460) {
            2u64
        } else {
            3u64
        }
    }
}
