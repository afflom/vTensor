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
                                            f47(v0, v2, {
                                                let m353 = v3;
                                                match m353 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(_) => {
                                                        true
                                                    }
                                                }
                                            }, v4)?
                                        } else {
                                            let a354 = v0;
                                            let a355 = 19u64;
                                            if nat_eq(a354, a355) {
                                                f48(v2, v4)?
                                            } else {
                                                let a356 = v0;
                                                let a357 = 21u64;
                                                if nat_eq(a356, a357) {
                                                    f49(v5, v2, v3, v4)?
                                                } else {
                                                    let a358 = v0;
                                                    let a359 = 61u64;
                                                    if if if nat_eq(a358, a359) {
                                                        true
                                                    } else {
                                                        let a360 = v0;
                                                        let a361 = 62u64;
                                                        nat_eq(a360, a361)
                                                    } {
                                                        true
                                                    } else {
                                                        let a362 = v0;
                                                        let a363 = 63u64;
                                                        nat_eq(a362, a363)
                                                    } {
                                                        f50(v0, v1, v2, v4)?
                                                    } else {
                                                        let a364 = v0;
                                                        let a365 = 4u64;
                                                        if if nat_eq(a364, a365) {
                                                            true
                                                        } else {
                                                            let a366 = v0;
                                                            let a367 = 32u64;
                                                            nat_eq(a366, a367)
                                                        } {
                                                            f51(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a368 = v0;
                                                            let a369 = 20u64;
                                                            if nat_eq(a368, a369) {
                                                                f52(v2, v4)?
                                                            } else {
                                                                let a370 = v0;
                                                                let a371 = 42u64;
                                                                if nat_eq(a370, a371) {
                                                                    f53(4u64, v2, v4)?
                                                                } else {
                                                                    let a372 = v0;
                                                                    let a373 = 69u64;
                                                                    if nat_eq(a372, a373) {
                                                                        let a374 = v5;
                                                                        let a375 = 4u64;
                                                                        if nat_lt(a374, a375) {
                                                                            f53(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a376 = v0;
                                                                        let a377 = 70u64;
                                                                        if nat_eq(a376, a377) {
                                                                            f54(v5, v2, v4)?
                                                                        } else {
                                                                            let a378 = v0;
                                                                            let a379 = 75u64;
                                                                            if if nat_eq(a378, a379) {
                                                                                true
                                                                            } else {
                                                                                let a380 = v0;
                                                                                let a381 = 76u64;
                                                                                nat_eq(a380, a381)
                                                                            } {
                                                                                f55({
                                                                                    let a382 = v0;
                                                                                    let a383 = 76u64;
                                                                                    nat_eq(a382, a383)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a384 = v0;
                                                                                let a385 = 31u64;
                                                                                if if if nat_eq(a384, a385) {
                                                                                    true
                                                                                } else {
                                                                                    let a386 = v0;
                                                                                    let a387 = 47u64;
                                                                                    nat_eq(a386, a387)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a388 = v0;
                                                                                    let a389 = 68u64;
                                                                                    nat_eq(a388, a389)
                                                                                } {
                                                                                    f56(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a390 = v0;
                                                                                    let a391 = 15u64;
                                                                                    if nat_eq(a390, a391) {
                                                                                        f57(v2, v4)?
                                                                                    } else {
                                                                                        let a392 = v0;
                                                                                        let a393 = 17u64;
                                                                                        if nat_eq(a392, a393) {
                                                                                            f58(v2, v4)?
                                                                                        } else {
                                                                                            let a394 = v0;
                                                                                            let a395 = 18u64;
                                                                                            if nat_eq(a394, a395) {
                                                                                                f59(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a396 = v0;
                                                                                                let a397 = 13u64;
                                                                                                if if if nat_eq(a396, a397) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a398 = v0;
                                                                                                    let a399 = 14u64;
                                                                                                    nat_eq(a398, a399)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a400 = v0;
                                                                                                    let a401 = 16u64;
                                                                                                    nat_eq(a400, a401)
                                                                                                } {
                                                                                                    f60(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m402 = f61(v0, v5)?;
                                                                                                    match m402 {
                                                                                                        None => {
                                                                                                            f62(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f63(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    let a403 = v1;
    let a404 = 0u64;
    if nat_eq(a403, a404) {
        let a406 = v3;
        let a407 = match v0.clone() {
            Adt8::C0(_, _, _, h405, _, _, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        };
        if nat_lt(a406, a407) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h408, _, _, _, _, _, _) => {
                    h408
                }
            };
            let v5: u64 = {
                let a410 = v3;
                let a411 = match v0.clone() {
                    Adt8::C0(_, _, h409, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h409
                    }
                };
                let a412 = 0u64;
                nat_quot(a410, a411, a412)
            };
            Ok(Adt9::C8(Some(f64(v4.clone(), f65(v5, v4.clone(), 0u64), f65(v5, v4.clone(), 1u64), f65(v5, v4, 2u64), f66(v0.clone(), v5, {
                let a414 = v3;
                let a415 = match v0 {
                    Adt8::C0(_, _, h413, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a416 = 0u64;
                nat_rem(a414, a415, a416)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h417, _, _, _, _) => {
                h417
            }
        };
        let v7: u64 = {
            let a418 = v1;
            let a419 = 1u64;
            if nat_eq(a418, a419) {
                match v0.clone() {
                    Adt8::C0(_, h420, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h420
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h421, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h421
                    }
                }
            }
        };
        let v8: u64 = {
            let a425 = v2;
            let a426 = {
                let a422 = v3;
                let a423 = v7;
                let a424 = 0u64;
                nat_quot(a422, a423, a424)
            };
            nat_add(a425, a426)?
        };
        let v9: u64 = {
            let a427 = v3;
            let a428 = v7;
            let a429 = 0u64;
            nat_rem(a427, a428, a429)
        };
        let v10: u64 = {
            let a431 = v8;
            let a432 = match v6.clone() {
                Adt3::C0(h430, _, _, _, _, _, _, _) => {
                    h430
                }
            };
            let a433 = 0u64;
            nat_rem(a431, a432, a433)
        };
        let v11: u64 = {
            let a435 = v8;
            let a436 = match v6.clone() {
                Adt3::C0(h434, _, _, _, _, _, _, _) => {
                    h434
                }
            };
            let a437 = 0u64;
            nat_quot(a435, a436, a437)
        };
        let v12: u64 = {
            let a439 = v11;
            let a440 = match v6.clone() {
                Adt3::C0(_, h438, _, _, _, _, _, _) => {
                    h438
                }
            };
            let a441 = 0u64;
            nat_rem(a439, a440, a441)
        };
        let v13: u64 = {
            let a443 = v11;
            let a444 = match v6.clone() {
                Adt3::C0(_, h442, _, _, _, _, _, _) => {
                    h442
                }
            };
            let a445 = 0u64;
            nat_quot(a443, a444, a445)
        };
        let a447 = v8;
        let a448 = match v0.clone() {
            Adt8::C0(h446, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h446
            }
        };
        if nat_lt(a447, a448) {
            let a449 = v1;
            let a450 = 1u64;
            if nat_eq(a449, a450) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h451, _, _, _, _, _, _) => {
                        h451
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h452, _, _, _, _, _) => {
                        h452
                    }
                };
                let m455 = f67(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h453, _, _, _) => {
                        h453
                    }
                }, match v15.clone() {
                    Adt3::C0(h454, _, _, _, _, _, _, _) => {
                        h454
                    }
                }, v10, f65(v9, v14.clone(), 0u64))?;
                match m455 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m458 = f67(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h456, _, _) => {
                                h456
                            }
                        }, match v15 {
                            Adt3::C0(_, h457, _, _, _, _, _, _) => {
                                h457
                            }
                        }, v12, f65(v9, v14, 1u64))?;
                        match m458 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f68(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a459 = v1;
                let a460 = 2u64;
                if nat_eq(a459, a460) {
                    Ok(Adt9::C8(Some({
                        let a462 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h461) => {
                                h461
                            }
                        };
                        let a463 = 0u64;
                        if nat_eq(a462, a463) {
                            f64(v6, v10, v12, v9, v13)?
                        } else {
                            f64(v6.clone(), v10, v12, {
                                let a465 = v13;
                                let a466 = match v6.clone() {
                                    Adt3::C0(_, _, h464, _, _, _, _, _) => {
                                        h464
                                    }
                                };
                                let a467 = 0u64;
                                nat_rem(a465, a466, a467)
                            }, {
                                let a475 = {
                                    let a473 = {
                                        let a469 = v13;
                                        let a470 = match v6 {
                                            Adt3::C0(_, _, h468, _, _, _, _, _) => {
                                                h468
                                            }
                                        };
                                        let a471 = 0u64;
                                        nat_quot(a469, a470, a471)
                                    };
                                    let a474 = match v0 {
                                        Adt8::C0(_, _, h472, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h472
                                        }
                                    };
                                    nat_mul(a473, a474)?
                                };
                                let a476 = v9;
                                nat_add(a475, a476)?
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
    Ok(f69(v3.clone(), {
        let m477 = v3.clone();
        match m477 {
            Adt4::C0 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f72(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C19(v51, v52) => {
                f73(v51, v0, v1, v2, v52, v4)?
            }
            Adt4::C20(v53, v54) => {
                f74(v53, v0, v1, v2, v54, v4)?
            }
            Adt4::C21 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C22(v55, v56) => {
                f75(v0, v1, v2, v55, v56, v4)?
            }
            Adt4::C23(v70, v71, v72, v73) => {
                f82(v0, v73, v70, v71, v72, v4)?
            }
            Adt4::C24(v69) => {
                if v5 {
                    f80(Adt16::C0(v69), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C26(v65, v66, v67, v68) => {
                let a478 = v65;
                let a479 = 64u64;
                if if if nat_eq(a478, a479) {
                    true
                } else {
                    let a480 = v65;
                    let a481 = 65u64;
                    nat_eq(a480, a481)
                } {
                    true
                } else {
                    let a482 = v65;
                    let a483 = 66u64;
                    nat_eq(a482, a483)
                } {
                    if v5 {
                        f80(Adt16::C1(v65, v66, v67, v68), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a484 = v65;
                    let a485 = 0u64;
                    if if nat_eq(a484, a485) {
                        true
                    } else {
                        let a486 = v65;
                        let a487 = 30u64;
                        nat_eq(a486, a487)
                    } {
                        f81(v0, v1, v2, v65, v66, v67, v68, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v64) => {
                f79(v0, v1, v2, v64)?
            }
            Adt4::C28(v60) => {
                f76(v0, v1, v2, v60, v4)?
            }
            Adt4::C29(v61) => {
                f77(v0, v1, v2, v61)?
            }
            Adt4::C30(v62, _) => {
                f78(v0, v1, v2, v62)?
            }
            Adt4::C31(v33, v34) => {
                f71(v0, v1, v2, v33, v34)?
            }
        }
    }))
}

pub fn f12(v0: Adt4) -> R<Adt9> {
    let m488 = v0;
    match m488 {
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
                let a493 = {
                    let a489 = 0u64;
                    let a490 = v47;
                    if nat_lt(a489, a490) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a494 = {
                    let a491 = 0u64;
                    let a492 = v49;
                    if nat_lt(a491, a492) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a493, a494)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v55, _, _, _) => {
            Ok(Adt9::C6({
                let a495 = v55;
                let a496 = 66u64;
                if nat_eq(a495, a496) {
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
    let a497 = v1;
    let a498 = 5u64;
    if if if if nat_lt(a497, a498) {
        let a499 = v5;
        let a500 = 5u64;
        nat_lt(a499, a500)
    } else {
        false
    } {
        let a501 = v7;
        let a502 = 5u64;
        nat_lt(a501, a502)
    } else {
        false
    } {
        let a503 = v9;
        let a504 = 2u64;
        if nat_lt(a503, a504) {
            true
        } else {
            let a505 = v9;
            let a506 = 3u64;
            nat_eq(a505, a506)
        }
    } else {
        false
    } {
        Adt9::C7(f83(v0, v1, v2, v3, v4), f84(v5, v6), f84(v7, v8), f85({
            let a507 = v9;
            let a508 = 0u64;
            if nat_eq(a507, a508) {
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
    let a520 = {
        let a514 = {
            let a510 = match v0.clone() {
                Adt2::C0(_, h509, _, _, _, _, _, _, _, _, _) => {
                    h509
                }
            };
            let a511 = v1;
            let a512 = 0u64;
            nat_quot(a510, a511, a512)
        };
        let a515 = match v0.clone() {
            Adt2::C0(_, _, h513, _, _, _, _, _, _, _, _) => {
                h513
            }
        };
        nat_mul(a514, a515)?
    };
    let a521 = {
        let a518 = match v0.clone() {
            Adt2::C0(_, _, _, h516, _, _, _, _, _, _, _) => {
                h516
            }
        };
        let a519 = match v0 {
            Adt2::C0(_, _, _, _, h517, _, _, _, _, _, _) => {
                h517
            }
        };
        nat_mul(a518, a519)?
    };
    nat_mul(a520, a521)
}

pub fn f15(v0: u64) -> bool {
    let a522 = v0;
    let a523 = 4294967296u64;
    if nat_lt(a522, a523) {
        let a527 = {
            let a524 = v0;
            let a525 = 2147483648u64;
            let a526 = 0u64;
            nat_rem(a524, a525, a526)
        };
        let a528 = 2139095040u64;
        nat_lt(a527, a528)
    } else {
        false
    }
}

pub fn f16(v0: u64) -> R<u64> {
    let a529 = 4u64;
    let a530 = v0;
    nat_mul(a529, a530)
}

pub fn f17(v0: u64) -> R<u64> {
    let a531 = v0;
    let a532 = 0u64;
    if nat_eq(a531, a532) {
        Ok(0u64)
    } else {
        let a535 = 4u64;
        let a536 = {
            let a533 = 3u64;
            let a534 = v0;
            nat_mul(a533, a534)?
        };
        nat_add(a535, a536)
    }
}

pub fn f18(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a538 = match v0.clone() {
            Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                h537
            }
        };
        let a539 = v1;
        let a540 = 0u64;
        nat_quot(a538, a539, a540)
    }, match v0.clone() {
        Adt2::C0(_, _, h541, _, _, _, _, _, _, _, _) => {
            h541
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h542, _, _, _, _, _, _, _) => {
            h542
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
            h543
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h544, _, _, _, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h545, _, _, _, _) => {
            h545
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h546, _, _, _) => {
            h546
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h547, _, _) => {
            h547
        }
    })
}

pub fn f19(v0: Adt6, v1: u64) -> R<u64> {
    let a549 = match v0.clone() {
        Adt6::C0(h548, _, _, _) => {
            h548
        }
    };
    let a550 = 0u64;
    if nat_eq(a549, a550) {
        f86(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h551, _, _) => {
                h551
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h552, _) => {
                h552
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h553) => {
                h553
            }
        }), {
            let a555 = v1;
            let a556 = match v0 {
                Adt6::C0(_, _, _, h554) => {
                    h554
                }
            };
            nat_add(a555, a556)?
        })
    } else {
        f86(v0, v1)
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> R<bool> {
    let a558 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h557, _) => {
            h557
        }
    };
    let a559 = 0u64;
    if nat_eq(a558, a559) {
        Ok(true)
    } else {
        let a561 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h560, _) => {
                h560
            }
        };
        let a562 = 0u64;
        if nat_eq(a561, a562) {
            Ok(true)
        } else {
            let a564 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h563) => {
                    h563
                }
            };
            let a565 = 0u64;
            if nat_eq(a564, a565) {
                Ok(true)
            } else {
                let a567 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h566) => {
                        h566
                    }
                };
                let a568 = 0u64;
                if nat_eq(a567, a568) {
                    Ok(true)
                } else {
                    let a574 = {
                        let a571 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h569) => {
                                h569
                            }
                        };
                        let a572 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h570, _) => {
                                h570
                            }
                        };
                        nat_add(a571, a572)?
                    };
                    let a575 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h573) => {
                            h573
                        }
                    };
                    if nat_le(a574, a575) {
                        Ok(true)
                    } else {
                        let a581 = {
                            let a578 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h576) => {
                                    h576
                                }
                            };
                            let a579 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h577, _) => {
                                    h577
                                }
                            };
                            nat_add(a578, a579)?
                        };
                        let a582 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h580) => {
                                h580
                            }
                        };
                        Ok(nat_le(a581, a582))
                    }
                }
            }
        }
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> bool {
    let a585 = match v0.clone() {
        Adt2::C0(_, h583, _, _, _, _, _, _, _, _, _) => {
            h583
        }
    };
    let a586 = match v1.clone() {
        Adt2::C0(_, h584, _, _, _, _, _, _, _, _, _) => {
            h584
        }
    };
    if if if nat_eq(a585, a586) {
        let a589 = match v0.clone() {
            Adt2::C0(_, _, h587, _, _, _, _, _, _, _, _) => {
                h587
            }
        };
        let a590 = match v1.clone() {
            Adt2::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                h588
            }
        };
        nat_eq(a589, a590)
    } else {
        false
    } {
        let a593 = match v0.clone() {
            Adt2::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
                h591
            }
        };
        let a594 = match v1.clone() {
            Adt2::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
                h592
            }
        };
        nat_eq(a593, a594)
    } else {
        false
    } {
        let a597 = match v0 {
            Adt2::C0(_, _, _, _, h595, _, _, _, _, _, _) => {
                h595
            }
        };
        let a598 = match v1 {
            Adt2::C0(_, _, _, _, h596, _, _, _, _, _, _) => {
                h596
            }
        };
        nat_eq(a597, a598)
    } else {
        false
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a600 = match v0.clone() {
        Adt2::C0(_, h599, _, _, _, _, _, _, _, _, _) => {
            h599
        }
    };
    let a601 = 1u64;
    if if if if nat_eq(a600, a601) {
        true
    } else {
        let a603 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h602, _, _, _, _, _) => {
                h602
            }
        };
        let a604 = 4u64;
        nat_eq(a603, a604)
    } {
        let a606 = match v0.clone() {
            Adt2::C0(_, _, h605, _, _, _, _, _, _, _, _) => {
                h605
            }
        };
        let a607 = 1u64;
        if nat_eq(a606, a607) {
            true
        } else {
            let a612 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h608, _, _, _, _) => {
                    h608
                }
            };
            let a613 = {
                let a610 = match v0.clone() {
                    Adt2::C0(_, h609, _, _, _, _, _, _, _, _, _) => {
                        h609
                    }
                };
                let a611 = 4u64;
                nat_mul(a610, a611)?
            };
            nat_eq(a612, a613)
        }
    } else {
        false
    } {
        let a615 = match v0.clone() {
            Adt2::C0(_, _, _, h614, _, _, _, _, _, _, _) => {
                h614
            }
        };
        let a616 = 1u64;
        if nat_eq(a615, a616) {
            true
        } else {
            let a624 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h617, _, _, _) => {
                    h617
                }
            };
            let a625 = {
                let a622 = {
                    let a620 = match v0.clone() {
                        Adt2::C0(_, h618, _, _, _, _, _, _, _, _, _) => {
                            h618
                        }
                    };
                    let a621 = match v0.clone() {
                        Adt2::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                            h619
                        }
                    };
                    nat_mul(a620, a621)?
                };
                let a623 = 4u64;
                nat_mul(a622, a623)?
            };
            nat_eq(a624, a625)
        }
    } else {
        false
    } {
        let a627 = match v0.clone() {
            Adt2::C0(_, _, _, _, h626, _, _, _, _, _, _) => {
                h626
            }
        };
        let a628 = 1u64;
        if nat_eq(a627, a628) {
            Ok(true)
        } else {
            let a639 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h629, _, _) => {
                    h629
                }
            };
            let a640 = {
                let a637 = {
                    let a635 = {
                        let a632 = match v0.clone() {
                            Adt2::C0(_, h630, _, _, _, _, _, _, _, _, _) => {
                                h630
                            }
                        };
                        let a633 = match v0.clone() {
                            Adt2::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                                h631
                            }
                        };
                        nat_mul(a632, a633)?
                    };
                    let a636 = match v0 {
                        Adt2::C0(_, _, _, h634, _, _, _, _, _, _, _) => {
                            h634
                        }
                    };
                    nat_mul(a635, a636)?
                };
                let a638 = 4u64;
                nat_mul(a637, a638)?
            };
            Ok(nat_eq(a639, a640))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a645 = {
        let a641 = v0;
        let a642 = v1;
        nat_mul(a641, a642)?
    };
    let a646 = {
        let a643 = v2;
        let a644 = f87(v3);
        nat_mul(a643, a644)?
    };
    nat_mul(a645, a646)
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a649 = v0;
    let a650 = {
        let a647 = f88(v1)?;
        let a648 = v2;
        nat_mul(a647, a648)?
    };
    nat_add(a649, a650)
}

pub fn f25(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h651, _, _, _, _, _, _, _, _, _, _) => {
            h651
        }
    }, match v0.clone() {
        Adt2::C0(_, h652, _, _, _, _, _, _, _, _, _) => {
            h652
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h653, _, _, _, _, _, _, _, _) => {
            h653
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h654, _, _, _, _, _, _, _) => {
            h654
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h655, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h656, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h657, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h658, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h659, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h660, _) => {
            h660
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h661) => {
            h661
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a662 = v0;
    let a663 = 23u64;
    if nat_eq(a662, a663) {
        f89(v1, v4, v6)
    } else {
        let a664 = v0;
        let a665 = 26u64;
        if nat_eq(a664, a665) {
            f90(v1, v2, v4, v6)
        } else {
            let a666 = v0;
            let a667 = 25u64;
            if nat_eq(a666, a667) {
                f91(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f27(v0: u64) -> u64 {
    let a668 = v0;
    let a669 = 30u64;
    if nat_eq(a668, a669) {
        2u64
    } else {
        4u64
    }
}

pub fn f28(v0: u64) -> bool {
    let a670 = v0;
    let a671 = 2139095040u64;
    if nat_le(a670, a671) {
        true
    } else {
        let a672 = v0;
        let a673 = 2147483648u64;
        nat_eq(a672, a673)
    }
}

pub fn f29(v0: Adt2, v1: u64) -> bool {
    let a675 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
            h674
        }
    };
    let a676 = v1;
    if if nat_eq(a675, a676) {
        let a681 = {
            let a678 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h677) => {
                    h677
                }
            };
            let a679 = v1;
            let a680 = 0u64;
            nat_rem(a678, a679, a680)
        };
        let a682 = 0u64;
        nat_eq(a681, a682)
    } else {
        false
    } {
        f92(match v0.clone() {
            Adt2::C0(_, _, h683, _, _, _, _, _, _, _, _) => {
                h683
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h684, _, _, _, _, _, _, _) => {
                h684
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h685, _, _, _, _, _, _) => {
                h685
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h686, _, _, _, _) => {
                h686
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h687, _, _, _) => {
                h687
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h688, _, _) => {
                h688
            }
        }, v1)
    } else {
        false
    }
}

pub fn f30(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a691 = match v0.clone() {
            Adt2::C0(h689, _, _, _, _, _, _, _, _, _, _) => {
                h689
            }
        };
        let a692 = match v1.clone() {
            Adt2::C0(h690, _, _, _, _, _, _, _, _, _, _) => {
                h690
            }
        };
        nat_eq(a691, a692)
    } else {
        false
    } {
        let a695 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h693) => {
                h693
            }
        };
        let a696 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h694) => {
                h694
            }
        };
        nat_eq(a695, a696)
    } else {
        false
    } {
        let a699 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h697, _, _, _, _, _) => {
                h697
            }
        };
        let a700 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h698, _, _, _, _, _) => {
                h698
            }
        };
        nat_eq(a699, a700)
    } else {
        false
    } {
        let a703 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h701, _, _, _, _) => {
                h701
            }
        };
        let a704 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h702, _, _, _, _) => {
                h702
            }
        };
        nat_eq(a703, a704)
    } else {
        false
    } {
        let a707 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h705, _, _, _) => {
                h705
            }
        };
        let a708 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h706, _, _, _) => {
                h706
            }
        };
        nat_eq(a707, a708)
    } else {
        false
    } {
        let a711 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h709, _, _) => {
                h709
            }
        };
        let a712 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h710, _, _) => {
                h710
            }
        };
        nat_eq(a711, a712)
    } else {
        false
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a713 = v0;
    let a714 = 0u64;
    if if if nat_eq(a713, a714) {
        true
    } else {
        let a715 = v1;
        let a716 = 0u64;
        nat_eq(a715, a716)
    } {
        true
    } else {
        let a717 = v2;
        let a718 = 0u64;
        nat_eq(a717, a718)
    } {
        true
    } else {
        let a719 = v3;
        let a720 = 0u64;
        nat_eq(a719, a720)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> R<bool> {
    f93({
        let a722 = match v0.clone() {
            Adt2::C0(_, h721, _, _, _, _, _, _, _, _, _) => {
                h721
            }
        };
        let a723 = v1;
        nat_mul(a722, a723)?
    }, match v0.clone() {
        Adt2::C0(_, _, h724, _, _, _, _, _, _, _, _) => {
            h724
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
            h725
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h726, _, _, _, _, _, _) => {
            h726
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h727, _, _, _, _) => {
            h727
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h728, _, _, _) => {
            h728
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h729, _, _) => {
            h729
        }
    })
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a730 = v1;
    let a731 = 0u64;
    if if nat_eq(a730, a731) {
        true
    } else {
        let a732 = v2;
        let a733 = 0u64;
        nat_eq(a732, a733)
    } {
        Ok(0u64)
    } else {
        let a740 = {
            let a734 = f94(v1, v2, v3)?;
            let a735 = f95(v0, v1, v2)?;
            nat_add(a734, a735)?
        };
        let a741 = {
            let a738 = f96(v0);
            let a739 = {
                let a736 = v1;
                let a737 = v2;
                nat_mul(a736, a737)?
            };
            nat_mul(a738, a739)?
        };
        nat_add(a740, a741)
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a742 = v0;
    let a743 = 0u64;
    if if nat_eq(a742, a743) {
        true
    } else {
        let a744 = v1;
        let a745 = 0u64;
        nat_eq(a744, a745)
    } {
        Ok(3u64)
    } else {
        let a748 = {
            let a746 = 3u64;
            let a747 = v1;
            nat_add(a746, a747)?
        };
        let a749 = v2;
        nat_add(a748, a749)
    }
}

pub fn f35(v0: u64) -> bool {
    let a750 = v0;
    let a751 = 2139095040u64;
    if nat_le(a750, a751) {
        true
    } else {
        let a752 = v0;
        let a753 = 2147483648u64;
        nat_eq(a752, a753)
    }
}

pub fn f36(v0: u64, v1: u64) -> R<u64> {
    let a754 = v0;
    let a755 = 0u64;
    if nat_eq(a754, a755) {
        Ok(0u64)
    } else {
        let a756 = v1;
        let a757 = f97(v0)?;
        nat_mul(a756, a757)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a758 = v0;
    let a759 = 0u64;
    if if if if nat_eq(a758, a759) {
        true
    } else {
        let a760 = v1;
        let a761 = 0u64;
        nat_eq(a760, a761)
    } {
        true
    } else {
        let a762 = v2;
        let a763 = 0u64;
        nat_eq(a762, a763)
    } {
        true
    } else {
        let a764 = v3;
        let a765 = 0u64;
        nat_eq(a764, a765)
    } {
        Ok(0u64)
    } else {
        let a778 = {
            let a768 = 2u64;
            let a769 = {
                let a766 = v1;
                let a767 = v2;
                nat_mul(a766, a767)?
            };
            nat_mul(a768, a769)?
        };
        let a779 = {
            let a776 = {
                let a772 = {
                    let a770 = v1;
                    let a771 = v2;
                    nat_mul(a770, a771)?
                };
                let a773 = v3;
                nat_mul(a772, a773)?
            };
            let a777 = {
                let a774 = 1u64;
                let a775 = v0;
                nat_add(a774, a775)?
            };
            nat_mul(a776, a777)?
        };
        nat_add(a778, a779)
    }
}

pub fn f38(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h780, _, _, _, _, _, _, _, _, _, _, _) => {
            h780
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h781, _, _, _, _, _, _, _, _, _, _) => {
            h781
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h782, _, _, _, _, _, _, _, _, _) => {
            h782
        }
    };
    let v6: u64 = {
        let a784 = match v1.clone() {
            Adt2::C0(_, h783, _, _, _, _, _, _, _, _, _) => {
                h783
            }
        };
        let a785 = v4;
        let a786 = 0u64;
        nat_quot(a784, a785, a786)
    };
    let v7: u64 = {
        let a788 = match v1.clone() {
            Adt2::C0(h787, _, _, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a789 = 0u64;
        if nat_eq(a788, a789) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f98(match v1.clone() {
        Adt2::C0(h790, _, _, _, _, _, _, _, _, _, _) => {
            h790
        }
    }) {
        let a793 = match v1.clone() {
            Adt2::C0(h791, _, _, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v2.clone() {
            Adt2::C0(h792, _, _, _, _, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_eq(a793, a794)
    } else {
        false
    } {
        let a795 = 0u64;
        let a796 = v3;
        nat_lt(a795, a796)
    } else {
        false
    } {
        let a797 = v3;
        let a798 = 2147483648u64;
        nat_lt(a797, a798)
    } else {
        false
    } {
        let a799 = 0u64;
        let a800 = v4;
        nat_lt(a799, a800)
    } else {
        false
    } {
        let a801 = v4;
        let a802 = 2147483648u64;
        nat_lt(a801, a802)
    } else {
        false
    } {
        let a803 = v5;
        let a804 = 2147483648u64;
        nat_lt(a803, a804)
    } else {
        false
    } {
        let a805 = 0u64;
        let a806 = v6;
        nat_lt(a805, a806)
    } else {
        false
    } {
        let a808 = 0u64;
        let a809 = match v2.clone() {
            Adt2::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        nat_lt(a808, a809)
    } else {
        false
    } {
        let a813 = match v1.clone() {
            Adt2::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
                h810
            }
        };
        let a814 = {
            let a811 = v6;
            let a812 = v4;
            nat_mul(a811, a812)?
        };
        nat_eq(a813, a814)
    } else {
        false
    } {
        let a816 = match v2.clone() {
            Adt2::C0(_, _, h815, _, _, _, _, _, _, _, _) => {
                h815
            }
        };
        let a817 = v4;
        nat_eq(a816, a817)
    } else {
        false
    } {
        let a830 = {
            let a823 = {
                let a821 = match v2.clone() {
                    Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                        h818
                    }
                };
                let a822 = {
                    let a819 = v5;
                    let a820 = 2u64;
                    nat_mul(a819, a820)?
                };
                nat_add(a821, a822)?
            };
            let a824 = v3;
            nat_add(a823, a824)?
        };
        let a831 = {
            let a828 = {
                let a826 = match v1.clone() {
                    Adt2::C0(_, _, h825, _, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = v3;
                nat_mul(a826, a827)?
            };
            let a829 = v6;
            nat_add(a828, a829)?
        };
        nat_eq(a830, a831)
    } else {
        false
    } {
        f99(v1.clone(), v7)?
    } else {
        false
    } {
        f99(v2.clone(), v7)?
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a832 = f14(v2.clone(), 1u64)?;
            let a833 = f100(v8.clone())?;
            nat_mul(a832, a833)?
        }, v7, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h834, _, _, _, _, _, _, _, _, _, _) => {
                h834
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f39(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m835 = v1;
    match m835 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a837 = match v0.clone() {
                Adt2::C0(h836, _, _, _, _, _, _, _, _, _, _) => {
                    h836
                }
            };
            let a838 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a837, a838) {
                let a840 = match v3.clone() {
                    Adt2::C0(h839, _, _, _, _, _, _, _, _, _, _) => {
                        h839
                    }
                };
                let a841 = 0u64;
                nat_eq(a840, a841)
            } else {
                false
            } {
                let a843 = match v2.clone() {
                    Adt2::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
                        h842
                    }
                };
                let a844 = 0u64;
                nat_eq(a843, a844)
            } else {
                false
            } {
                let a846 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h845, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a847 = 1u64;
                nat_eq(a846, a847)
            } else {
                false
            } {
                let a849 = match v3.clone() {
                    Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                        h848
                    }
                };
                let a850 = 1u64;
                nat_eq(a849, a850)
            } else {
                false
            } {
                let a852 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h851, _, _, _, _, _, _) => {
                        h851
                    }
                };
                let a853 = 1u64;
                nat_eq(a852, a853)
            } else {
                false
            } {
                let a856 = match v0.clone() {
                    Adt2::C0(_, _, h854, _, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a857 = match v3.clone() {
                    Adt2::C0(_, _, h855, _, _, _, _, _, _, _, _) => {
                        h855
                    }
                };
                nat_eq(a856, a857)
            } else {
                false
            } {
                let a862 = match v3.clone() {
                    Adt2::C0(_, h858, _, _, _, _, _, _, _, _, _) => {
                        h858
                    }
                };
                let a863 = {
                    let a860 = match v0.clone() {
                        Adt2::C0(_, h859, _, _, _, _, _, _, _, _, _) => {
                            h859
                        }
                    };
                    let a861 = 1u64;
                    nat_add(a860, a861)?
                };
                nat_le(a862, a863)
            } else {
                false
            } {
                let a866 = match v2.clone() {
                    Adt2::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
                        h864
                    }
                };
                let a867 = match v0.clone() {
                    Adt2::C0(_, _, h865, _, _, _, _, _, _, _, _) => {
                        h865
                    }
                };
                nat_eq(a866, a867)
            } else {
                false
            } {
                let a875 = match v2.clone() {
                    Adt2::C0(_, _, h868, _, _, _, _, _, _, _, _) => {
                        h868
                    }
                };
                let a876 = {
                    let a873 = {
                        let a870 = match v0.clone() {
                            Adt2::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                                h869
                            }
                        };
                        let a871 = 1u64;
                        nat_add(a870, a871)?
                    };
                    let a874 = match v3.clone() {
                        Adt2::C0(_, h872, _, _, _, _, _, _, _, _, _) => {
                            h872
                        }
                    };
                    nat_sub(a873, a874)
                };
                nat_eq(a875, a876)
            } else {
                false
            } {
                let a879 = match v2.clone() {
                    Adt2::C0(_, _, _, h877, _, _, _, _, _, _, _) => {
                        h877
                    }
                };
                let a880 = match v0.clone() {
                    Adt2::C0(_, _, _, h878, _, _, _, _, _, _, _) => {
                        h878
                    }
                };
                nat_eq(a879, a880)
            } else {
                false
            } {
                let a882 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h881, _, _, _, _, _, _) => {
                        h881
                    }
                };
                let a883 = 1u64;
                nat_eq(a882, a883)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a885 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h884, _, _, _, _, _) => {
                        h884
                    }
                };
                let a886 = 4u64;
                nat_eq(a885, a886)
            } else {
                false
            } {
                let a888 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h887, _, _, _, _, _) => {
                        h887
                    }
                };
                let a889 = 4u64;
                nat_eq(a888, a889)
            } else {
                false
            } {
                let a894 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h890, _, _, _, _) => {
                        h890
                    }
                };
                let a895 = {
                    let a892 = match v0.clone() {
                        Adt2::C0(_, h891, _, _, _, _, _, _, _, _, _) => {
                            h891
                        }
                    };
                    let a893 = 4u64;
                    nat_mul(a892, a893)?
                };
                nat_eq(a894, a895)
            } else {
                false
            } {
                let a900 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h896, _, _, _, _) => {
                        h896
                    }
                };
                let a901 = {
                    let a898 = match v3.clone() {
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
                f20(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f20(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a903 = f14(v2.clone(), 1u64)?;
                    let a904 = f101(match v3.clone() {
                        Adt2::C0(_, h902, _, _, _, _, _, _, _, _, _) => {
                            h902
                        }
                    })?;
                    nat_mul(a903, a904)?
                }, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C29(f18(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f40(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h905, _, _, _, _, _, _, _, _, _, _) => {
            h905
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h906, _, _, _, _, _, _, _, _) => {
                h906
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h907, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h908, _, _, _, _, _, _) => {
                h908
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h909, _, _, _, _, _, _, _, _) => {
                h909
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h910, _, _, _, _, _, _, _, _, _) => {
                h910
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h911, _, _, _, _, _, _, _) => {
                h911
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h912, _, _, _, _, _) => {
                h912
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a914 = match v1.clone() {
        Adt1::C0(h913, _, _, _, _, _, _, _, _, _, _, _) => {
            h913
        }
    };
    let a915 = 1u64;
    if if if if if if if if if if if nat_le(a914, a915) {
        let a917 = match v2.clone() {
            Adt2::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        nat_le(a917, a918)
    } else {
        false
    } {
        let a920 = match v3.clone() {
            Adt2::C0(h919, _, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 0u64;
        nat_eq(a920, a921)
    } else {
        false
    } {
        let a926 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h922, _, _, _, _, _) => {
                h922
            }
        };
        let a927 = {
            let a924 = match v2.clone() {
                Adt2::C0(h923, _, _, _, _, _, _, _, _, _, _) => {
                    h923
                }
            };
            let a925 = 0u64;
            if nat_eq(a924, a925) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a926, a927)
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
        f102(v4.clone(), match v2.clone() {
            Adt2::C0(_, h928, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        }, match v3.clone() {
            Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        })?
    } else {
        false
    } {
        if v0 {
            f102(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h930, _, _, _, _, _, _, _, _) => {
                    h930
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h931, _, _, _, _, _, _, _, _) => {
                    h931
                }
            })?
        } else {
            let a934 = match v2.clone() {
                Adt2::C0(_, _, h932, _, _, _, _, _, _, _, _) => {
                    h932
                }
            };
            let a935 = match v3.clone() {
                Adt2::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                    h933
                }
            };
            nat_eq(a934, a935)
        }
    } else {
        false
    } {
        let a938 = match v2.clone() {
            Adt2::C0(_, _, _, h936, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a939 = match v3.clone() {
            Adt2::C0(_, _, _, h937, _, _, _, _, _, _, _) => {
                h937
            }
        };
        nat_eq(a938, a939)
    } else {
        false
    } {
        let a942 = match v2.clone() {
            Adt2::C0(_, _, _, _, h940, _, _, _, _, _, _) => {
                h940
            }
        };
        let a943 = match v3.clone() {
            Adt2::C0(_, _, _, _, h941, _, _, _, _, _, _) => {
                h941
            }
        };
        nat_eq(a942, a943)
    } else {
        false
    } {
        let a948 = {
            let a946 = match v4.clone() {
                Adt6::C0(h944, _, _, _) => {
                    h944
                }
            };
            let a947 = match v5.clone() {
                Adt6::C0(h945, _, _, _) => {
                    h945
                }
            };
            nat_mul(a946, a947)?
        };
        let a949 = 2147483648u64;
        nat_lt(a948, a949)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a951 = match v1 {
                Adt1::C0(h950, _, _, _, _, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            let a952 = 0u64;
            nat_eq(a951, a952)
        }, match v2.clone() {
            Adt2::C0(h953, _, _, _, _, _, _, _, _, _, _) => {
                h953
            }
        });
        Ok(Adt10::C4({
            let a954 = f14(v3.clone(), 1u64)?;
            let a955 = f103(v6.clone())?;
            nat_mul(a954, a955)?
        }, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m956 = v2;
    match m956 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a958 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h957, _, _, _, _, _) => {
                        h957
                    }
                };
                let a959 = 1u64;
                nat_eq(a958, a959)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h960, _, _, _, _, _, _, _, _, _) => {
                    h960
                }
            }, match v0.clone() {
                Adt1::C0(h961, _, _, _, _, _, _, _, _, _, _, _) => {
                    h961
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h962, _, _, _, _, _, _, _, _, _) => {
                    h962
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h963, _, _, _, _, _, _, _) => {
                    h963
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h964, _, _, _, _, _, _, _, _) => {
                        h964
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h965, _, _, _, _, _, _, _, _, _, _) => {
                        h965
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h966, _, _, _, _, _, _, _, _) => {
                        h966
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h967, _, _, _, _, _, _) => {
                        h967
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                        h968
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h969, _, _, _, _, _, _, _, _) => {
                        h969
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h970, _, _, _, _, _, _) => {
                        h970
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h971, _, _, _, _, _, _, _) => {
                        h971
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h972, _, _, _, _, _, _, _, _) => {
                        h972
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                    h973
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                    h974
                }
            };
            let v14: u64 = {
                let a975 = v12;
                let a976 = 0u64;
                if nat_eq(a975, a976) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a978 = match v4.clone() {
                    Adt2::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a979 = v14;
                nat_mul(a978, a979)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h980, _, _, _) => {
                        h980
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h981, _, _, _, _) => {
                        h981
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h982, _, _) => {
                        h982
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h983, _, _, _) => {
                        h983
                    }
                }
            };
            let a985 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h984, _, _, _, _, _) => {
                    h984
                }
            };
            let a986 = 1u64;
            if if if if if if if if if if nat_le(a985, a986) {
                let a987 = v12;
                let a988 = 0u64;
                if nat_eq(a987, a988) {
                    true
                } else {
                    let a989 = v12;
                    let a990 = 1u64;
                    if nat_eq(a989, a990) {
                        let a991 = v13;
                        let a992 = 1u64;
                        nat_eq(a991, a992)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a993 = v13;
                let a994 = 0u64;
                if nat_eq(a993, a994) {
                    true
                } else {
                    let a995 = v13;
                    let a996 = 1u64;
                    nat_eq(a995, a996)
                }
            } else {
                false
            } {
                let a998 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h997, _, _, _, _, _) => {
                        h997
                    }
                };
                let a999 = v14;
                nat_eq(a998, a999)
            } else {
                false
            } {
                let a1007 = match v3.clone() {
                    Adt2::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                        h1000
                    }
                };
                let a1008 = {
                    let a1005 = v9;
                    let a1006 = {
                        let a1003 = match v6.clone() {
                            Adt6::C0(h1001, _, _, _) => {
                                h1001
                            }
                        };
                        let a1004 = match v7.clone() {
                            Adt6::C0(h1002, _, _, _) => {
                                h1002
                            }
                        };
                        nat_mul(a1003, a1004)?
                    };
                    nat_mul(a1005, a1006)?
                };
                nat_eq(a1007, a1008)
            } else {
                false
            } {
                let a1011 = match v3.clone() {
                    Adt2::C0(_, _, h1009, _, _, _, _, _, _, _, _) => {
                        h1009
                    }
                };
                let a1012 = f86(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                        h1010
                    }
                })?;
                nat_eq(a1011, a1012)
            } else {
                false
            } {
                let a1014 = 0u64;
                let a1015 = match v3.clone() {
                    Adt2::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                };
                nat_lt(a1014, a1015)
            } else {
                false
            } {
                if v5 {
                    let a1017 = match v1 {
                        Adt2::C0(_, _, _, h1016, _, _, _, _, _, _, _) => {
                            h1016
                        }
                    };
                    let a1018 = v9;
                    if if if nat_eq(a1017, a1018) {
                        let a1020 = match v3.clone() {
                            Adt2::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                                h1019
                            }
                        };
                        let a1021 = f86(v7.clone(), v11)?;
                        nat_eq(a1020, a1021)
                    } else {
                        false
                    } {
                        let a1023 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
                                h1022
                            }
                        };
                        let a1024 = v10;
                        nat_eq(a1023, a1024)
                    } else {
                        false
                    } {
                        let a1026 = 0u64;
                        let a1027 = match v3.clone() {
                            Adt2::C0(_, _, _, h1025, _, _, _, _, _, _, _) => {
                                h1025
                            }
                        };
                        nat_lt(a1026, a1027)
                    } else {
                        false
                    }
                } else {
                    let a1029 = match v1 {
                        Adt2::C0(_, _, h1028, _, _, _, _, _, _, _, _) => {
                            h1028
                        }
                    };
                    let a1030 = v9;
                    if if if nat_eq(a1029, a1030) {
                        let a1032 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1031, _, _, _, _, _, _) => {
                                h1031
                            }
                        };
                        let a1033 = 1u64;
                        nat_eq(a1032, a1033)
                    } else {
                        false
                    } {
                        let a1035 = match v3.clone() {
                            Adt2::C0(_, _, _, h1034, _, _, _, _, _, _, _) => {
                                h1034
                            }
                        };
                        let a1036 = v10;
                        nat_eq(a1035, a1036)
                    } else {
                        false
                    } {
                        let a1038 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1037, _, _, _, _, _, _) => {
                                h1037
                            }
                        };
                        let a1039 = 1u64;
                        nat_eq(a1038, a1039)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f31(match v4.clone() {
                    Adt2::C0(_, h1040, _, _, _, _, _, _, _, _, _) => {
                        h1040
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1060 = {
                        let a1057 = {
                            let a1055 = {
                                let a1049 = {
                                    let a1043 = {
                                        let a1041 = v10;
                                        let a1042 = 1u64;
                                        nat_sub(a1041, a1042)
                                    };
                                    let a1044 = v17;
                                    nat_mul(a1043, a1044)?
                                };
                                let a1050 = {
                                    let a1047 = {
                                        let a1045 = v9;
                                        let a1046 = 1u64;
                                        nat_sub(a1045, a1046)
                                    };
                                    let a1048 = v16;
                                    nat_mul(a1047, a1048)?
                                };
                                nat_add(a1049, a1050)?
                            };
                            let a1056 = {
                                let a1053 = {
                                    let a1051 = v11;
                                    let a1052 = 1u64;
                                    nat_sub(a1051, a1052)
                                };
                                let a1054 = v15;
                                nat_mul(a1053, a1054)?
                            };
                            nat_add(a1055, a1056)?
                        };
                        let a1058 = v15;
                        nat_add(a1057, a1058)?
                    };
                    let a1061 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1059, _) => {
                            h1059
                        }
                    };
                    nat_le(a1060, a1061)
                }
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
                    let a1062 = v13;
                    let a1063 = 0u64;
                    if nat_eq(a1062, a1063) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1064, _, _, _, _, _, _, _, _, _) => {
                        h1064
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f18(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1065, _, _, _, _, _, _, _, _, _) => {
                            h1065
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
                            h1066
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1067, _, _, _, _, _, _, _) => {
                            h1067
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1068, _, _, _, _, _) => {
                            h1068
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1069, _, _, _, _) => {
                            h1069
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1070, _, _, _) => {
                            h1070
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
    let m1071 = v2;
    match m1071 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1072, _, _, _, _, _, _, _, _, _) => {
                    h1072
                }
            }, match v0.clone() {
                Adt1::C0(h1073, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1073
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1074, _, _, _, _, _, _, _, _) => {
                    h1074
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1075, _, _, _, _, _) => {
                    h1075
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1076, _, _, _, _, _, _, _, _) => {
                    h1076
                }
            }, match v0.clone() {
                Adt1::C0(_, h1077, _, _, _, _, _, _, _, _, _, _) => {
                    h1077
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1078, _, _, _, _, _, _, _) => {
                    h1078
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1079, _, _, _, _) => {
                    h1079
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1080, _, _, _, _, _, _, _) => {
                    h1080
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1081, _, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1082, _, _, _, _, _, _) => {
                    h1082
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1083, _, _, _) => {
                    h1083
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1084, _, _) => {
                    h1084
                }
            };
            let v9: u64 = f86(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1085, _, _, _, _, _, _, _) => {
                    h1085
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                    h1086
                }
            };
            let v11: u64 = {
                let a1094 = v8;
                let a1095 = {
                    let a1092 = match v5.clone() {
                        Adt6::C0(h1087, _, _, _) => {
                            h1087
                        }
                    };
                    let a1093 = {
                        let a1090 = match v6.clone() {
                            Adt6::C0(h1088, _, _, _) => {
                                h1088
                            }
                        };
                        let a1091 = match v7.clone() {
                            Adt6::C0(h1089, _, _, _) => {
                                h1089
                            }
                        };
                        nat_mul(a1090, a1091)?
                    };
                    nat_mul(a1092, a1093)?
                };
                nat_mul(a1094, a1095)?
            };
            let a1097 = match v4.clone() {
                Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
                    h1096
                }
            };
            let a1098 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1097, a1098) {
                let a1099 = v10;
                let a1100 = 0u64;
                if nat_eq(a1099, a1100) {
                    true
                } else {
                    let a1101 = v10;
                    let a1102 = 1u64;
                    nat_eq(a1101, a1102)
                }
            } else {
                false
            } {
                let a1103 = 0u64;
                let a1104 = v8;
                nat_lt(a1103, a1104)
            } else {
                false
            } {
                let a1105 = v8;
                let a1106 = 2147483648u64;
                nat_lt(a1105, a1106)
            } else {
                false
            } {
                let a1108 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1107, _, _, _, _, _) => {
                        h1107
                    }
                };
                let a1109 = 4u64;
                nat_eq(a1108, a1109)
            } else {
                false
            } {
                let a1110 = 0u64;
                let a1111 = v9;
                nat_lt(a1110, a1111)
            } else {
                false
            } {
                let a1113 = 0u64;
                let a1114 = match v3.clone() {
                    Adt2::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
                        h1112
                    }
                };
                nat_lt(a1113, a1114)
            } else {
                false
            } {
                let a1116 = 0u64;
                let a1117 = match v3.clone() {
                    Adt2::C0(_, _, _, h1115, _, _, _, _, _, _, _) => {
                        h1115
                    }
                };
                nat_lt(a1116, a1117)
            } else {
                false
            } {
                let a1119 = match v3.clone() {
                    Adt2::C0(_, h1118, _, _, _, _, _, _, _, _, _) => {
                        h1118
                    }
                };
                let a1120 = v11;
                nat_eq(a1119, a1120)
            } else {
                false
            } {
                let a1123 = match v3.clone() {
                    Adt2::C0(_, _, h1121, _, _, _, _, _, _, _, _) => {
                        h1121
                    }
                };
                let a1124 = f86(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1122, _, _, _, _, _, _, _, _, _) => {
                        h1122
                    }
                })?;
                nat_eq(a1123, a1124)
            } else {
                false
            } {
                let a1127 = match v3.clone() {
                    Adt2::C0(_, _, _, h1125, _, _, _, _, _, _, _) => {
                        h1125
                    }
                };
                let a1128 = f86(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1126, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                })?;
                nat_eq(a1127, a1128)
            } else {
                false
            } {
                let a1136 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1129, _, _, _, _, _, _) => {
                        h1129
                    }
                };
                let a1137 = {
                    let a1134 = v9;
                    let a1135 = {
                        let a1131 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1130, _, _, _, _, _, _) => {
                                h1130
                            }
                        };
                        let a1132 = v8;
                        let a1133 = 0u64;
                        nat_quot(a1131, a1132, a1133)
                    };
                    nat_mul(a1134, a1135)?
                };
                nat_eq(a1136, a1137)
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
                    let a1138 = v10;
                    let a1139 = 0u64;
                    if nat_eq(a1138, a1139) {
                        4u64
                    } else {
                        2u64
                    }
                }, f18(v4.clone(), 1u64), f18(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1140, _, _) => {
                        h1140
                    }
                }, {
                    let a1142 = v8;
                    let a1143 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1141, _, _) => {
                            h1141
                        }
                    };
                    nat_mul(a1142, a1143)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f43(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1144 = v1;
    match m1144 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f104(v0, v3, v2)
        }
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1146 = match v0.clone() {
        Adt2::C0(h1145, _, _, _, _, _, _, _, _, _, _) => {
            h1145
        }
    };
    let a1147 = 1u64;
    if if nat_eq(a1146, a1147) {
        true
    } else {
        let a1149 = match v0.clone() {
            Adt2::C0(h1148, _, _, _, _, _, _, _, _, _, _) => {
                h1148
            }
        };
        let a1150 = 30u64;
        nat_eq(a1149, a1150)
    } {
        let a1152 = match v1.clone() {
            Adt2::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
                h1151
            }
        };
        let a1153 = 1u64;
        if if if if if if if if if if if nat_eq(a1152, a1153) {
            let a1156 = match v0.clone() {
                Adt2::C0(_, h1154, _, _, _, _, _, _, _, _, _) => {
                    h1154
                }
            };
            let a1157 = match v1.clone() {
                Adt2::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
                    h1155
                }
            };
            nat_eq(a1156, a1157)
        } else {
            false
        } {
            let a1159 = 0u64;
            let a1160 = match v1.clone() {
                Adt2::C0(_, _, h1158, _, _, _, _, _, _, _, _) => {
                    h1158
                }
            };
            nat_lt(a1159, a1160)
        } else {
            false
        } {
            let a1163 = match v1.clone() {
                Adt2::C0(_, _, h1161, _, _, _, _, _, _, _, _) => {
                    h1161
                }
            };
            let a1164 = match v1.clone() {
                Adt2::C0(_, _, _, h1162, _, _, _, _, _, _, _) => {
                    h1162
                }
            };
            nat_eq(a1163, a1164)
        } else {
            false
        } {
            let a1166 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1165, _, _, _, _, _, _) => {
                    h1165
                }
            };
            let a1167 = 1u64;
            nat_eq(a1166, a1167)
        } else {
            false
        } {
            let a1175 = match v0.clone() {
                Adt2::C0(_, _, h1168, _, _, _, _, _, _, _, _) => {
                    h1168
                }
            };
            let a1176 = {
                let a1173 = {
                    let a1171 = match v1.clone() {
                        Adt2::C0(_, _, h1169, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1172 = match v1.clone() {
                        Adt2::C0(_, _, h1170, _, _, _, _, _, _, _, _) => {
                            h1170
                        }
                    };
                    nat_add(a1171, a1172)?
                };
                let a1174 = 1u64;
                nat_sub(a1173, a1174)
            };
            nat_eq(a1175, a1176)
        } else {
            false
        } {
            let a1178 = 0u64;
            let a1179 = match v0.clone() {
                Adt2::C0(_, _, _, h1177, _, _, _, _, _, _, _) => {
                    h1177
                }
            };
            nat_lt(a1178, a1179)
        } else {
            false
        } {
            let a1181 = 0u64;
            let a1182 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1180, _, _, _, _, _, _) => {
                    h1180
                }
            };
            nat_lt(a1181, a1182)
        } else {
            false
        } {
            let a1184 = 2u64;
            let a1185 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1183, _, _, _, _, _) => {
                    h1183
                }
            };
            nat_le(a1184, a1185)
        } else {
            false
        } {
            let a1187 = 2u64;
            let a1188 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1186, _, _, _, _, _) => {
                    h1186
                }
            };
            nat_le(a1187, a1188)
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
    let m1189 = v1;
    match m1189 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f105(v0, v3, v2)
        }
    }
}

pub fn f46(v0: u64) -> bool {
    let a1190 = v0;
    let a1191 = 1u64;
    if if nat_eq(a1190, a1191) {
        true
    } else {
        let a1192 = v0;
        let a1193 = 33u64;
        nat_eq(a1192, a1193)
    } {
        true
    } else {
        let a1194 = v0;
        let a1195 = 34u64;
        nat_eq(a1194, a1195)
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1198 = match v1.clone() {
        Adt2::C0(h1196, _, _, _, _, _, _, _, _, _, _) => {
            h1196
        }
    };
    let a1199 = match v3.clone() {
        Adt2::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
            h1197
        }
    };
    if nat_eq(a1198, a1199) {
        f106(v0, v1, v2, v3)
    } else {
        if if if if f107(match v1.clone() {
            Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
                h1200
            }
        }) {
            f107(match v3.clone() {
                Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                    h1201
                }
            })
        } else {
            false
        } {
            let a1202 = v0;
            let a1203 = 33u64;
            if nat_eq(a1202, a1203) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1204 = f14(v1.clone(), 1u64)?;
            let a1205 = f14(v3.clone(), 1u64)?;
            nat_eq(a1204, a1205)
        } else {
            false
        } {
            let m1207 = f108(match v3.clone() {
                Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                    h1206
                }
            });
            match m1207 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1208, _, _, _, _, _, _, _, _, _, _) => {
                            h1208
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                            h1209
                        }
                    };
                    let v7: bool = {
                        let a1210 = v5;
                        let a1211 = 43u64;
                        if nat_eq(a1210, a1211) {
                            true
                        } else {
                            let a1212 = v6;
                            let a1213 = 43u64;
                            nat_eq(a1212, a1213)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1214 = f14(v3.clone(), 1u64)?;
                        let a1215 = if v7 {
                            f109(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1214, a1215)?
                    }, match v4 {
                        Adt18::C0(_, _, h1216) => {
                            h1216
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

pub fn f48(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1218 = match v0.clone() {
        Adt2::C0(h1217, _, _, _, _, _, _, _, _, _, _) => {
            h1217
        }
    };
    let a1219 = 0u64;
    if if if if nat_eq(a1218, a1219) {
        true
    } else {
        let a1221 = match v0.clone() {
            Adt2::C0(h1220, _, _, _, _, _, _, _, _, _, _) => {
                h1220
            }
        };
        let a1222 = 1u64;
        nat_eq(a1221, a1222)
    } {
        true
    } else {
        let a1224 = match v0.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 25u64;
        if nat_eq(a1224, a1225) {
            true
        } else {
            let a1227 = match v0.clone() {
                Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                    h1226
                }
            };
            let a1228 = 26u64;
            nat_eq(a1227, a1228)
        }
    } {
        true
    } else {
        let a1230 = match v0.clone() {
            Adt2::C0(h1229, _, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1231 = 30u64;
        nat_eq(a1230, a1231)
    } {
        let a1234 = match v0.clone() {
            Adt2::C0(h1232, _, _, _, _, _, _, _, _, _, _) => {
                h1232
            }
        };
        let a1235 = match v1.clone() {
            Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                h1233
            }
        };
        if if nat_eq(a1234, a1235) {
            f110(match v1.clone() {
                Adt2::C0(_, h1236, _, _, _, _, _, _, _, _, _) => {
                    h1236
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1237, _, _, _, _, _, _, _, _) => {
                    h1237
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1238, _, _, _, _, _, _, _) => {
                    h1238
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1239, _, _, _, _, _, _) => {
                    h1239
                }
            }, match v0.clone() {
                Adt2::C0(_, h1240, _, _, _, _, _, _, _, _, _) => {
                    h1240
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1241, _, _, _, _, _, _, _, _) => {
                    h1241
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1242, _, _, _, _, _, _, _) => {
                    h1242
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1243, _, _, _, _, _, _) => {
                    h1243
                }
            })
        } else {
            false
        } {
            let m1245 = f108(match v0.clone() {
                Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                    h1244
                }
            });
            match m1245 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f14(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1246) => {
                            h1246
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

pub fn f49(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1247 = v2;
    match m1247 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1248 = v0;
            let a1249 = 4u64;
            if if if nat_lt(a1248, a1249) {
                let a1252 = match v1.clone() {
                    Adt2::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                        h1250
                    }
                };
                let a1253 = match v4.clone() {
                    Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                        h1251
                    }
                };
                if nat_eq(a1252, a1253) {
                    let a1256 = match v1.clone() {
                        Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                            h1254
                        }
                    };
                    let a1257 = match v3.clone() {
                        Adt2::C0(h1255, _, _, _, _, _, _, _, _, _, _) => {
                            h1255
                        }
                    };
                    nat_eq(a1256, a1257)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f111(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1258, _, _, _, _, _, _, _, _, _) => {
                        h1258
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1259, _, _, _, _, _, _, _, _, _) => {
                        h1259
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1260, _, _, _, _, _, _, _, _, _) => {
                        h1260
                    }
                })? {
                    f111(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1261, _, _, _, _, _, _, _, _) => {
                            h1261
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1262, _, _, _, _, _, _, _, _) => {
                            h1262
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1263, _, _, _, _, _, _, _, _) => {
                            h1263
                        }
                    })?
                } else {
                    false
                } {
                    f111(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1264, _, _, _, _, _, _, _) => {
                            h1264
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1265, _, _, _, _, _, _, _) => {
                            h1265
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1266, _, _, _, _, _, _, _) => {
                            h1266
                        }
                    })?
                } else {
                    false
                } {
                    f111(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1267, _, _, _, _, _, _) => {
                            h1267
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1268, _, _, _, _, _, _) => {
                            h1268
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1269, _, _, _, _, _, _) => {
                            h1269
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1271 = f108(match v1.clone() {
                    Adt2::C0(h1270, _, _, _, _, _, _, _, _, _, _) => {
                        h1270
                    }
                });
                match m1271 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1272, _) => {
                                h1272
                            }
                        };
                        let a1277 = {
                            let a1274 = match v1.clone() {
                                Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                                    h1273
                                }
                            };
                            let a1275 = v6;
                            let a1276 = 1u64;
                            nat_rem(a1274, a1275, a1276)
                        };
                        let a1278 = 0u64;
                        if if if nat_eq(a1277, a1278) {
                            let a1283 = {
                                let a1280 = match v4.clone() {
                                    Adt2::C0(_, h1279, _, _, _, _, _, _, _, _, _) => {
                                        h1279
                                    }
                                };
                                let a1281 = v6;
                                let a1282 = 1u64;
                                nat_rem(a1280, a1281, a1282)
                            };
                            let a1284 = 0u64;
                            nat_eq(a1283, a1284)
                        } else {
                            false
                        } {
                            let a1289 = {
                                let a1286 = match v3.clone() {
                                    Adt2::C0(_, h1285, _, _, _, _, _, _, _, _, _) => {
                                        h1285
                                    }
                                };
                                let a1287 = v6;
                                let a1288 = 1u64;
                                nat_rem(a1286, a1287, a1288)
                            };
                            let a1290 = 0u64;
                            nat_eq(a1289, a1290)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f14(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1291) => {
                                    h1291
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

pub fn f50(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1293 = match v2.clone() {
        Adt2::C0(h1292, _, _, _, _, _, _, _, _, _, _) => {
            h1292
        }
    };
    let a1294 = 0u64;
    if if nat_eq(a1293, a1294) {
        let a1296 = match v3.clone() {
            Adt2::C0(h1295, _, _, _, _, _, _, _, _, _, _) => {
                h1295
            }
        };
        let a1297 = 0u64;
        nat_eq(a1296, a1297)
    } else {
        false
    } {
        let a1298 = v0;
        let a1299 = 61u64;
        if nat_eq(a1298, a1299) {
            let v4: bool = {
                let a1301 = 0u64;
                let a1302 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1300, _, _, _) => {
                        h1300
                    }
                };
                nat_lt(a1301, a1302)
            };
            if if if if if if f112(match v2.clone() {
                Adt2::C0(_, h1303, _, _, _, _, _, _, _, _, _) => {
                    h1303
                }
            }, match v1.clone() {
                Adt1::C0(h1304, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1304
                }
            }, match v1.clone() {
                Adt1::C0(_, h1305, _, _, _, _, _, _, _, _, _, _) => {
                    h1305
                }
            }, match v3.clone() {
                Adt2::C0(_, h1306, _, _, _, _, _, _, _, _, _) => {
                    h1306
                }
            })? {
                f112(match v2.clone() {
                    Adt2::C0(_, _, h1307, _, _, _, _, _, _, _, _) => {
                        h1307
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1308, _, _, _, _, _, _, _, _, _) => {
                        h1308
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1309, _, _, _, _, _, _, _, _) => {
                        h1309
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1310, _, _, _, _, _, _, _, _) => {
                        h1310
                    }
                })?
            } else {
                false
            } {
                f112(match v2.clone() {
                    Adt2::C0(_, _, _, h1311, _, _, _, _, _, _, _) => {
                        h1311
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1312, _, _, _, _, _, _, _) => {
                        h1312
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1313, _, _, _, _, _, _) => {
                        h1313
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1314, _, _, _, _, _, _, _) => {
                        h1314
                    }
                })?
            } else {
                false
            } {
                f112(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1315, _, _, _, _, _, _) => {
                        h1315
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1316, _, _, _, _, _) => {
                        h1316
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1317, _, _, _, _) => {
                        h1317
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1318, _, _, _, _, _, _) => {
                        h1318
                    }
                })?
            } else {
                false
            } {
                let a1320 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1319, _, _, _) => {
                        h1319
                    }
                };
                let a1321 = 4294967295u64;
                nat_le(a1320, a1321)
            } else {
                false
            } {
                if v4 {
                    !f31(match v2.clone() {
                        Adt2::C0(_, h1322, _, _, _, _, _, _, _, _, _) => {
                            h1322
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1323, _, _, _, _, _, _, _, _) => {
                            h1323
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1324, _, _, _, _, _, _, _) => {
                            h1324
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1325, _, _, _, _, _, _) => {
                            h1325
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1326, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1326
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1327, _, _, _, _, _, _, _, _, _) => {
                        h1327
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1328, _, _, _, _, _, _, _) => {
                        h1328
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1329, _, _, _, _, _) => {
                        h1329
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1330 = v0;
            let a1331 = 62u64;
            if nat_eq(a1330, a1331) {
                let a1333 = match v1.clone() {
                    Adt1::C0(h1332, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1332
                    }
                };
                let a1334 = 2147483648u64;
                if if if if if nat_lt(a1333, a1334) {
                    let a1336 = match v1.clone() {
                        Adt1::C0(_, h1335, _, _, _, _, _, _, _, _, _, _) => {
                            h1335
                        }
                    };
                    let a1337 = 2147483648u64;
                    nat_lt(a1336, a1337)
                } else {
                    false
                } {
                    let a1340 = match v1.clone() {
                        Adt1::C0(h1338, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1338
                        }
                    };
                    let a1341 = match v2.clone() {
                        Adt2::C0(_, h1339, _, _, _, _, _, _, _, _, _) => {
                            h1339
                        }
                    };
                    nat_lt(a1340, a1341)
                } else {
                    false
                } {
                    let a1344 = match v1.clone() {
                        Adt1::C0(_, h1342, _, _, _, _, _, _, _, _, _, _) => {
                            h1342
                        }
                    };
                    let a1345 = match v2.clone() {
                        Adt2::C0(_, h1343, _, _, _, _, _, _, _, _, _) => {
                            h1343
                        }
                    };
                    nat_lt(a1344, a1345)
                } else {
                    false
                } {
                    let a1348 = match v2.clone() {
                        Adt2::C0(_, _, h1346, _, _, _, _, _, _, _, _) => {
                            h1346
                        }
                    };
                    let a1349 = match v3.clone() {
                        Adt2::C0(_, _, h1347, _, _, _, _, _, _, _, _) => {
                            h1347
                        }
                    };
                    if if if nat_eq(a1348, a1349) {
                        let a1352 = match v2.clone() {
                            Adt2::C0(_, _, _, h1350, _, _, _, _, _, _, _) => {
                                h1350
                            }
                        };
                        let a1353 = match v3.clone() {
                            Adt2::C0(_, _, _, h1351, _, _, _, _, _, _, _) => {
                                h1351
                            }
                        };
                        nat_eq(a1352, a1353)
                    } else {
                        false
                    } {
                        let a1356 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1354, _, _, _, _, _, _) => {
                                h1354
                            }
                        };
                        let a1357 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1355, _, _, _, _, _, _) => {
                                h1355
                            }
                        };
                        nat_eq(a1356, a1357)
                    } else {
                        false
                    } {
                        let a1366 = match v3.clone() {
                            Adt2::C0(_, h1358, _, _, _, _, _, _, _, _, _) => {
                                h1358
                            }
                        };
                        let a1367 = {
                            let a1364 = {
                                let a1361 = match v2.clone() {
                                    Adt2::C0(_, h1359, _, _, _, _, _, _, _, _, _) => {
                                        h1359
                                    }
                                };
                                let a1362 = match v1.clone() {
                                    Adt1::C0(h1360, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1360
                                    }
                                };
                                nat_add(a1361, a1362)?
                            };
                            let a1365 = match v1.clone() {
                                Adt1::C0(_, h1363, _, _, _, _, _, _, _, _, _, _) => {
                                    h1363
                                }
                            };
                            nat_add(a1364, a1365)?
                        };
                        nat_eq(a1366, a1367)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1368, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1368
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1369 = v0;
                let a1370 = 63u64;
                if nat_eq(a1369, a1370) {
                    if if if if if f113(match v2.clone() {
                        Adt2::C0(_, h1371, _, _, _, _, _, _, _, _, _) => {
                            h1371
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1372, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1372
                        }
                    })? {
                        f113(match v2.clone() {
                            Adt2::C0(_, _, h1373, _, _, _, _, _, _, _, _) => {
                                h1373
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1374, _, _, _, _, _, _, _, _, _, _) => {
                                h1374
                            }
                        })?
                    } else {
                        false
                    } {
                        f113(match v2.clone() {
                            Adt2::C0(_, _, _, h1375, _, _, _, _, _, _, _) => {
                                h1375
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1376, _, _, _, _, _, _, _, _, _) => {
                                h1376
                            }
                        })?
                    } else {
                        false
                    } {
                        f113(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1377, _, _, _, _, _, _) => {
                                h1377
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1378, _, _, _, _, _, _, _, _) => {
                                h1378
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1381 = match v2.clone() {
                            Adt2::C0(_, h1379, _, _, _, _, _, _, _, _, _) => {
                                h1379
                            }
                        };
                        let a1382 = match v3.clone() {
                            Adt2::C0(_, h1380, _, _, _, _, _, _, _, _, _) => {
                                h1380
                            }
                        };
                        if if if nat_eq(a1381, a1382) {
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
                            nat_eq(a1385, a1386)
                        } else {
                            false
                        } {
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
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1395, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1395
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1396, _, _, _, _, _, _, _, _, _, _) => {
                                h1396
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1397, _, _, _, _, _, _, _, _, _) => {
                                h1397
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1398, _, _, _, _, _, _, _, _) => {
                                h1398
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

pub fn f51(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1399 = v3;
    match m1399 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1400 = v0;
                let a1401 = 4u64;
                nat_eq(a1400, a1401)
            };
            let v7: bool = {
                let a1403 = 0u64;
                let a1404 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1402, _, _, _, _, _, _, _) => {
                        h1402
                    }
                };
                nat_lt(a1403, a1404)
            };
            let v8: u64 = f14(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1405, _, _, _, _, _, _, _, _, _) => {
                    h1405
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1406, _, _, _, _, _, _, _, _) => {
                    h1406
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1407, _, _, _, _, _, _, _) => {
                    h1407
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1408, _, _, _, _, _, _) => {
                    h1408
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1409, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1409
                }
            }, match v1.clone() {
                Adt1::C0(_, h1410, _, _, _, _, _, _, _, _, _, _) => {
                    h1410
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1411, _, _, _, _, _, _, _, _, _) => {
                    h1411
                }
            });
            if if if if if if if if if if if if v6 {
                let a1413 = match v2.clone() {
                    Adt2::C0(h1412, _, _, _, _, _, _, _, _, _, _) => {
                        h1412
                    }
                };
                let a1414 = 0u64;
                if nat_eq(a1413, a1414) {
                    let a1416 = match v5.clone() {
                        Adt2::C0(h1415, _, _, _, _, _, _, _, _, _, _) => {
                            h1415
                        }
                    };
                    let a1417 = 0u64;
                    nat_eq(a1416, a1417)
                } else {
                    false
                }
            } else {
                let a1419 = match v2.clone() {
                    Adt2::C0(h1418, _, _, _, _, _, _, _, _, _, _) => {
                        h1418
                    }
                };
                let a1420 = 0u64;
                if if nat_eq(a1419, a1420) {
                    true
                } else {
                    let a1422 = match v2.clone() {
                        Adt2::C0(h1421, _, _, _, _, _, _, _, _, _, _) => {
                            h1421
                        }
                    };
                    let a1423 = 26u64;
                    nat_eq(a1422, a1423)
                } {
                    let a1425 = match v5.clone() {
                        Adt2::C0(h1424, _, _, _, _, _, _, _, _, _, _) => {
                            h1424
                        }
                    };
                    let a1426 = 0u64;
                    if nat_eq(a1425, a1426) {
                        true
                    } else {
                        let a1428 = match v5.clone() {
                            Adt2::C0(h1427, _, _, _, _, _, _, _, _, _, _) => {
                                h1427
                            }
                        };
                        let a1429 = 26u64;
                        nat_eq(a1428, a1429)
                    }
                } else {
                    false
                }
            } {
                let a1432 = match v2.clone() {
                    Adt2::C0(h1430, _, _, _, _, _, _, _, _, _, _) => {
                        h1430
                    }
                };
                let a1433 = match v4.clone() {
                    Adt2::C0(h1431, _, _, _, _, _, _, _, _, _, _) => {
                        h1431
                    }
                };
                nat_eq(a1432, a1433)
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
                let a1435 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1434, _, _, _, _, _) => {
                        h1434
                    }
                };
                let a1436 = 4u64;
                nat_eq(a1435, a1436)
            } else {
                false
            } {
                let a1437 = v8;
                let a1438 = f14(v2.clone(), 1u64)?;
                nat_le(a1437, a1438)
            } else {
                false
            } {
                let a1440 = match v1.clone() {
                    Adt1::C0(_, _, _, h1439, _, _, _, _, _, _, _, _) => {
                        h1439
                    }
                };
                let a1441 = 1073741824u64;
                nat_lt(a1440, a1441)
            } else {
                false
            } {
                let a1443 = match v1.clone() {
                    Adt1::C0(h1442, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1442
                    }
                };
                let a1444 = 4294967295u64;
                if if if if nat_le(a1443, a1444) {
                    let a1446 = match v1.clone() {
                        Adt1::C0(_, h1445, _, _, _, _, _, _, _, _, _, _) => {
                            h1445
                        }
                    };
                    let a1447 = 4294967295u64;
                    nat_le(a1446, a1447)
                } else {
                    false
                } {
                    let a1449 = match v1.clone() {
                        Adt1::C0(_, _, h1448, _, _, _, _, _, _, _, _, _) => {
                            h1448
                        }
                    };
                    let a1450 = 4294967295u64;
                    nat_le(a1449, a1450)
                } else {
                    false
                } {
                    let a1452 = match v1.clone() {
                        Adt1::C0(_, _, _, h1451, _, _, _, _, _, _, _, _) => {
                            h1451
                        }
                    };
                    let a1453 = 4294967295u64;
                    nat_le(a1452, a1453)
                } else {
                    false
                } {
                    let a1455 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1454, _, _, _, _, _, _, _) => {
                            h1454
                        }
                    };
                    let a1456 = 4294967295u64;
                    nat_le(a1455, a1456)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1459 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1457) => {
                            h1457
                        }
                    };
                    let a1460 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1458) => {
                            h1458
                        }
                    };
                    nat_eq(a1459, a1460)
                } else {
                    true
                }
            } else {
                false
            } {
                f114(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1461, _, _, _, _, _, _, _, _) => {
                        h1461
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1462, _) => {
                        h1462
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
                    let a1463 = v10;
                    let a1464 = v8;
                    nat_add(a1463, a1464)?
                }, 4u64, f18(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1465, _, _, _, _, _, _, _, _) => {
                        h1465
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f52(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1467 = match v0.clone() {
        Adt2::C0(h1466, _, _, _, _, _, _, _, _, _, _) => {
            h1466
        }
    };
    let a1468 = 0u64;
    if if if nat_eq(a1467, a1468) {
        let a1470 = match v1.clone() {
            Adt2::C0(h1469, _, _, _, _, _, _, _, _, _, _) => {
                h1469
            }
        };
        let a1471 = 0u64;
        nat_eq(a1470, a1471)
    } else {
        false
    } {
        f110(match v0.clone() {
            Adt2::C0(_, h1472, _, _, _, _, _, _, _, _, _) => {
                h1472
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1473, _, _, _, _, _, _, _, _) => {
                h1473
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1474, _, _, _, _, _, _, _) => {
                h1474
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1475, _, _, _, _, _, _) => {
                h1475
            }
        }, match v1.clone() {
            Adt2::C0(_, h1476, _, _, _, _, _, _, _, _, _) => {
                h1476
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1477, _, _, _, _, _, _, _, _) => {
                h1477
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1478, _, _, _, _, _, _, _) => {
                h1478
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1479, _, _, _, _, _, _) => {
                h1479
            }
        })
    } else {
        false
    } {
        let v2: u64 = f14(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1480 = v2;
            let a1481 = f14(v0.clone(), 1u64)?;
            nat_add(a1480, a1481)?
        }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1483 = match v1.clone() {
        Adt2::C0(h1482, _, _, _, _, _, _, _, _, _, _) => {
            h1482
        }
    };
    let a1484 = 0u64;
    if if if nat_eq(a1483, a1484) {
        true
    } else {
        let a1486 = match v1.clone() {
            Adt2::C0(h1485, _, _, _, _, _, _, _, _, _, _) => {
                h1485
            }
        };
        let a1487 = 1u64;
        nat_eq(a1486, a1487)
    } {
        true
    } else {
        let a1489 = match v1.clone() {
            Adt2::C0(h1488, _, _, _, _, _, _, _, _, _, _) => {
                h1488
            }
        };
        let a1490 = 30u64;
        nat_eq(a1489, a1490)
    } {
        let a1493 = match v1.clone() {
            Adt2::C0(h1491, _, _, _, _, _, _, _, _, _, _) => {
                h1491
            }
        };
        let a1494 = match v2.clone() {
            Adt2::C0(h1492, _, _, _, _, _, _, _, _, _, _) => {
                h1492
            }
        };
        if if if if if nat_eq(a1493, a1494) {
            let a1497 = match v1.clone() {
                Adt2::C0(_, h1495, _, _, _, _, _, _, _, _, _) => {
                    h1495
                }
            };
            let a1498 = match v2.clone() {
                Adt2::C0(_, h1496, _, _, _, _, _, _, _, _, _) => {
                    h1496
                }
            };
            nat_eq(a1497, a1498)
        } else {
            false
        } {
            let a1501 = match v1.clone() {
                Adt2::C0(_, _, _, h1499, _, _, _, _, _, _, _) => {
                    h1499
                }
            };
            let a1502 = match v2.clone() {
                Adt2::C0(_, _, _, h1500, _, _, _, _, _, _, _) => {
                    h1500
                }
            };
            nat_eq(a1501, a1502)
        } else {
            false
        } {
            let a1505 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1503, _, _, _, _, _, _) => {
                    h1503
                }
            };
            let a1506 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1504, _, _, _, _, _, _) => {
                    h1504
                }
            };
            nat_eq(a1505, a1506)
        } else {
            false
        } {
            let a1507 = v0;
            let a1508 = 4u64;
            if nat_eq(a1507, a1508) {
                let a1510 = match v1.clone() {
                    Adt2::C0(_, _, h1509, _, _, _, _, _, _, _, _) => {
                        h1509
                    }
                };
                let a1511 = 1u64;
                if nat_eq(a1510, a1511) {
                    let a1514 = match v1.clone() {
                        Adt2::C0(_, h1512, _, _, _, _, _, _, _, _, _) => {
                            h1512
                        }
                    };
                    let a1515 = match v2.clone() {
                        Adt2::C0(_, _, h1513, _, _, _, _, _, _, _, _) => {
                            h1513
                        }
                    };
                    nat_eq(a1514, a1515)
                } else {
                    false
                }
            } else {
                let a1516 = v0;
                let a1517 = 4u64;
                if if nat_lt(a1516, a1517) {
                    let a1520 = match v1.clone() {
                        Adt2::C0(_, h1518, _, _, _, _, _, _, _, _, _) => {
                            h1518
                        }
                    };
                    let a1521 = match v1.clone() {
                        Adt2::C0(_, _, h1519, _, _, _, _, _, _, _, _) => {
                            h1519
                        }
                    };
                    nat_eq(a1520, a1521)
                } else {
                    false
                } {
                    let a1524 = match v1.clone() {
                        Adt2::C0(_, _, h1522, _, _, _, _, _, _, _, _) => {
                            h1522
                        }
                    };
                    let a1525 = match v2.clone() {
                        Adt2::C0(_, _, h1523, _, _, _, _, _, _, _, _) => {
                            h1523
                        }
                    };
                    nat_eq(a1524, a1525)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1527 = f108(match v1.clone() {
                Adt2::C0(h1526, _, _, _, _, _, _, _, _, _, _) => {
                    h1526
                }
            });
            match m1527 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1528) => {
                            h1528
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

pub fn f54(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f115(match v1.clone() {
        Adt2::C0(h1529, _, _, _, _, _, _, _, _, _, _) => {
            h1529
        }
    }, v0) {
        let a1532 = match v1.clone() {
            Adt2::C0(h1530, _, _, _, _, _, _, _, _, _, _) => {
                h1530
            }
        };
        let a1533 = match v2.clone() {
            Adt2::C0(h1531, _, _, _, _, _, _, _, _, _, _) => {
                h1531
            }
        };
        nat_eq(a1532, a1533)
    } else {
        false
    } {
        let a1536 = match v1.clone() {
            Adt2::C0(_, h1534, _, _, _, _, _, _, _, _, _) => {
                h1534
            }
        };
        let a1537 = match v2.clone() {
            Adt2::C0(_, h1535, _, _, _, _, _, _, _, _, _) => {
                h1535
            }
        };
        if if if nat_eq(a1536, a1537) {
            let a1540 = match v1.clone() {
                Adt2::C0(_, _, h1538, _, _, _, _, _, _, _, _) => {
                    h1538
                }
            };
            let a1541 = match v2.clone() {
                Adt2::C0(_, _, h1539, _, _, _, _, _, _, _, _) => {
                    h1539
                }
            };
            nat_eq(a1540, a1541)
        } else {
            false
        } {
            let a1544 = match v1.clone() {
                Adt2::C0(_, _, _, h1542, _, _, _, _, _, _, _) => {
                    h1542
                }
            };
            let a1545 = match v2.clone() {
                Adt2::C0(_, _, _, h1543, _, _, _, _, _, _, _) => {
                    h1543
                }
            };
            nat_eq(a1544, a1545)
        } else {
            false
        } {
            let a1548 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1546, _, _, _, _, _, _) => {
                    h1546
                }
            };
            let a1549 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1547, _, _, _, _, _, _) => {
                    h1547
                }
            };
            nat_eq(a1548, a1549)
        } else {
            false
        }
    } else {
        false
    } {
        let m1551 = f108(match v1.clone() {
            Adt2::C0(h1550, _, _, _, _, _, _, _, _, _, _) => {
                h1550
            }
        });
        match m1551 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1552) => {
                        h1552
                    }
                }, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1553, _, _, _, _, _, _, _, _, _, _) => {
                        h1553
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f55(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1554, _, _, _, _, _, _, _, _, _, _, _) => {
                h1554
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1555, _, _, _, _, _, _, _, _, _) => {
                h1555
            }
        }
    };
    let a1556 = 0u64;
    let a1557 = v4;
    if if nat_lt(a1556, a1557) {
        let a1558 = v4;
        let a1559 = 2147483648u64;
        nat_lt(a1558, a1559)
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
        let v7: u64 = f116(match v5.clone() {
            Adt2::C0(_, _, h1560, _, _, _, _, _, _, _, _) => {
                h1560
            }
        }, v4)?;
        let v8: u64 = f116(match v5.clone() {
            Adt2::C0(_, _, _, h1561, _, _, _, _, _, _, _) => {
                h1561
            }
        }, v4)?;
        let a1563 = match v5.clone() {
            Adt2::C0(h1562, _, _, _, _, _, _, _, _, _, _) => {
                h1562
            }
        };
        let a1564 = 0u64;
        if if if if if nat_eq(a1563, a1564) {
            let a1566 = match v6.clone() {
                Adt2::C0(h1565, _, _, _, _, _, _, _, _, _, _) => {
                    h1565
                }
            };
            let a1567 = 0u64;
            nat_eq(a1566, a1567)
        } else {
            false
        } {
            let a1569 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1568, _, _, _, _, _, _) => {
                    h1568
                }
            };
            let a1570 = 1u64;
            nat_eq(a1569, a1570)
        } else {
            false
        } {
            let a1573 = match v5.clone() {
                Adt2::C0(_, h1571, _, _, _, _, _, _, _, _, _) => {
                    h1571
                }
            };
            let a1574 = match v6.clone() {
                Adt2::C0(_, h1572, _, _, _, _, _, _, _, _, _) => {
                    h1572
                }
            };
            nat_eq(a1573, a1574)
        } else {
            false
        } {
            if v0 {
                let a1576 = match v5.clone() {
                    Adt2::C0(_, _, h1575, _, _, _, _, _, _, _, _) => {
                        h1575
                    }
                };
                let a1577 = 2147483648u64;
                if if if nat_lt(a1576, a1577) {
                    let a1579 = match v5.clone() {
                        Adt2::C0(_, _, _, h1578, _, _, _, _, _, _, _) => {
                            h1578
                        }
                    };
                    let a1580 = 2147483648u64;
                    nat_lt(a1579, a1580)
                } else {
                    false
                } {
                    let a1583 = {
                        let a1581 = v7;
                        let a1582 = v8;
                        nat_mul(a1581, a1582)?
                    };
                    let a1584 = 2147483648u64;
                    nat_le(a1583, a1584)
                } else {
                    false
                } {
                    f117(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1586 = match v1.clone() {
                    Adt1::C0(h1585, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1585
                    }
                };
                let a1587 = v7;
                if if if if nat_eq(a1586, a1587) {
                    let a1589 = match v1 {
                        Adt1::C0(_, h1588, _, _, _, _, _, _, _, _, _, _) => {
                            h1588
                        }
                    };
                    let a1590 = v8;
                    nat_eq(a1589, a1590)
                } else {
                    false
                } {
                    f118(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1592 = match v6.clone() {
                        Adt2::C0(_, _, h1591, _, _, _, _, _, _, _, _) => {
                            h1591
                        }
                    };
                    let a1593 = v4;
                    if nat_eq(a1592, a1593) {
                        let a1595 = match v6.clone() {
                            Adt2::C0(_, _, _, h1594, _, _, _, _, _, _, _) => {
                                h1594
                            }
                        };
                        let a1596 = v4;
                        nat_eq(a1595, a1596)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1600 = match v6 {
                        Adt2::C0(_, _, _, _, h1597, _, _, _, _, _, _) => {
                            h1597
                        }
                    };
                    let a1601 = {
                        let a1598 = v7;
                        let a1599 = v8;
                        nat_mul(a1598, a1599)?
                    };
                    nat_eq(a1600, a1601)
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

pub fn f56(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1603 = match v2.clone() {
        Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
            h1602
        }
    };
    let a1604 = 0u64;
    if if if if if nat_eq(a1603, a1604) {
        true
    } else {
        let a1606 = match v2.clone() {
            Adt2::C0(h1605, _, _, _, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1607 = 1u64;
        nat_eq(a1606, a1607)
    } {
        true
    } else {
        let a1609 = match v2.clone() {
            Adt2::C0(h1608, _, _, _, _, _, _, _, _, _, _) => {
                h1608
            }
        };
        let a1610 = 30u64;
        nat_eq(a1609, a1610)
    } {
        let a1613 = match v2.clone() {
            Adt2::C0(h1611, _, _, _, _, _, _, _, _, _, _) => {
                h1611
            }
        };
        let a1614 = match v3.clone() {
            Adt2::C0(h1612, _, _, _, _, _, _, _, _, _, _) => {
                h1612
            }
        };
        nat_eq(a1613, a1614)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a1616 = match v1.clone() {
                Adt1::C0(h1615, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1615
                }
            };
            let a1617 = 4294967296u64;
            nat_lt(a1616, a1617)
        } else {
            false
        } {
            let a1619 = match v1.clone() {
                Adt1::C0(_, h1618, _, _, _, _, _, _, _, _, _, _) => {
                    h1618
                }
            };
            let a1620 = 4294967296u64;
            nat_lt(a1619, a1620)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
            let a1622 = match v2.clone() {
                Adt2::C0(h1621, _, _, _, _, _, _, _, _, _, _) => {
                    h1621
                }
            };
            let a1623 = 0u64;
            if nat_eq(a1622, a1623) {
                4u64
            } else {
                2u64
            }
        }, f18(v2.clone(), 1u64), f18(v3, 1u64), Adt4::C13({
            let a1624 = v0;
            let a1625 = 31u64;
            if nat_eq(a1624, a1625) {
                0u64
            } else {
                let a1626 = v0;
                let a1627 = 47u64;
                if nat_eq(a1626, a1627) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
                h1628
            }
        }, match v1.clone() {
            Adt1::C0(h1629, _, _, _, _, _, _, _, _, _, _, _) => {
                h1629
            }
        }, match v1 {
            Adt1::C0(_, h1630, _, _, _, _, _, _, _, _, _, _) => {
                h1630
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f57(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1632 = match v0.clone() {
        Adt2::C0(h1631, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    };
    let a1633 = 0u64;
    if if if nat_eq(a1632, a1633) {
        let a1635 = match v1.clone() {
            Adt2::C0(h1634, _, _, _, _, _, _, _, _, _, _) => {
                h1634
            }
        };
        let a1636 = 0u64;
        nat_eq(a1635, a1636)
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

pub fn f58(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1638 = match v0.clone() {
        Adt2::C0(h1637, _, _, _, _, _, _, _, _, _, _) => {
            h1637
        }
    };
    let a1639 = 0u64;
    if if nat_eq(a1638, a1639) {
        let a1641 = match v1.clone() {
            Adt2::C0(h1640, _, _, _, _, _, _, _, _, _, _) => {
                h1640
            }
        };
        let a1642 = 26u64;
        nat_eq(a1641, a1642)
    } else {
        false
    } {
        let a1644 = match v0.clone() {
            Adt2::C0(_, _, _, h1643, _, _, _, _, _, _, _) => {
                h1643
            }
        };
        let a1645 = 1u64;
        if if if if if if if nat_eq(a1644, a1645) {
            let a1647 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1646, _, _, _, _, _, _) => {
                    h1646
                }
            };
            let a1648 = 1u64;
            nat_eq(a1647, a1648)
        } else {
            false
        } {
            let a1650 = match v0.clone() {
                Adt2::C0(_, h1649, _, _, _, _, _, _, _, _, _) => {
                    h1649
                }
            };
            let a1651 = 2147483647u64;
            nat_le(a1650, a1651)
        } else {
            false
        } {
            let a1654 = match v1.clone() {
                Adt2::C0(_, h1652, _, _, _, _, _, _, _, _, _) => {
                    h1652
                }
            };
            let a1655 = match v0.clone() {
                Adt2::C0(_, _, h1653, _, _, _, _, _, _, _, _) => {
                    h1653
                }
            };
            nat_eq(a1654, a1655)
        } else {
            false
        } {
            let a1657 = match v1.clone() {
                Adt2::C0(_, _, h1656, _, _, _, _, _, _, _, _) => {
                    h1656
                }
            };
            let a1658 = 1u64;
            nat_eq(a1657, a1658)
        } else {
            false
        } {
            let a1660 = match v1.clone() {
                Adt2::C0(_, _, _, h1659, _, _, _, _, _, _, _) => {
                    h1659
                }
            };
            let a1661 = 1u64;
            nat_eq(a1660, a1661)
        } else {
            false
        } {
            let a1663 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1662, _, _, _, _, _, _) => {
                    h1662
                }
            };
            let a1664 = 1u64;
            nat_eq(a1663, a1664)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1667 = match v0.clone() {
                    Adt2::C0(_, _, h1665, _, _, _, _, _, _, _, _) => {
                        h1665
                    }
                };
                let a1668 = f119(match v0.clone() {
                    Adt2::C0(_, h1666, _, _, _, _, _, _, _, _, _) => {
                        h1666
                    }
                })?;
                nat_mul(a1667, a1668)?
            }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f59(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1669 = v1;
    match m1669 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f120(f18(v0.clone(), 1u64))?;
            let a1671 = match v0.clone() {
                Adt2::C0(h1670, _, _, _, _, _, _, _, _, _, _) => {
                    h1670
                }
            };
            let a1672 = 26u64;
            if if if nat_eq(a1671, a1672) {
                let a1674 = match v3.clone() {
                    Adt2::C0(h1673, _, _, _, _, _, _, _, _, _, _) => {
                        h1673
                    }
                };
                let a1675 = 26u64;
                nat_eq(a1674, a1675)
            } else {
                false
            } {
                let a1677 = match v2.clone() {
                    Adt2::C0(h1676, _, _, _, _, _, _, _, _, _, _) => {
                        h1676
                    }
                };
                let a1678 = 27u64;
                nat_eq(a1677, a1678)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a1680 = match v2.clone() {
                        Adt2::C0(_, h1679, _, _, _, _, _, _, _, _, _) => {
                            h1679
                        }
                    };
                    let a1681 = 1u64;
                    nat_eq(a1680, a1681)
                } else {
                    false
                } {
                    let a1683 = match v2.clone() {
                        Adt2::C0(_, _, h1682, _, _, _, _, _, _, _, _) => {
                            h1682
                        }
                    };
                    let a1684 = 1u64;
                    nat_eq(a1683, a1684)
                } else {
                    false
                } {
                    let a1686 = match v2.clone() {
                        Adt2::C0(_, _, _, h1685, _, _, _, _, _, _, _) => {
                            h1685
                        }
                    };
                    let a1687 = 1u64;
                    nat_eq(a1686, a1687)
                } else {
                    false
                } {
                    let a1689 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1688, _, _, _, _, _, _) => {
                            h1688
                        }
                    };
                    let a1690 = 1u64;
                    nat_eq(a1689, a1690)
                } else {
                    false
                } {
                    let a1691 = v4;
                    let a1692 = 9223372036854775808u64;
                    nat_lt(a1691, a1692)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1695 = 2u64;
                        let a1696 = {
                            let a1693 = v4;
                            let a1694 = 2u64;
                            nat_mul(a1693, a1694)?
                        };
                        nat_add(a1695, a1696)?
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

pub fn f60(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1697, _, _, _, _, _, _, _, _, _, _) => {
            h1697
        }
    };
    let a1698 = v3;
    let a1699 = 0u64;
    if if nat_eq(a1698, a1699) {
        true
    } else {
        let a1700 = v0;
        let a1701 = 13u64;
        if nat_eq(a1700, a1701) {
            let a1702 = v3;
            let a1703 = 1u64;
            if nat_eq(a1702, a1703) {
                true
            } else {
                let a1704 = v3;
                let a1705 = 30u64;
                nat_eq(a1704, a1705)
            }
        } else {
            false
        }
    } {
        let a1707 = v3;
        let a1708 = match v2.clone() {
            Adt2::C0(h1706, _, _, _, _, _, _, _, _, _, _) => {
                h1706
            }
        };
        if if nat_eq(a1707, a1708) {
            let a1709 = v0;
            let a1710 = 13u64;
            if nat_eq(a1709, a1710) {
                let a1712 = match v2.clone() {
                    Adt2::C0(_, h1711, _, _, _, _, _, _, _, _, _) => {
                        h1711
                    }
                };
                let a1713 = 1u64;
                if if if nat_eq(a1712, a1713) {
                    let a1715 = match v2.clone() {
                        Adt2::C0(_, _, h1714, _, _, _, _, _, _, _, _) => {
                            h1714
                        }
                    };
                    let a1716 = 1u64;
                    nat_eq(a1715, a1716)
                } else {
                    false
                } {
                    let a1718 = match v2.clone() {
                        Adt2::C0(_, _, _, h1717, _, _, _, _, _, _, _) => {
                            h1717
                        }
                    };
                    let a1719 = 1u64;
                    nat_eq(a1718, a1719)
                } else {
                    false
                } {
                    let a1721 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1720, _, _, _, _, _, _) => {
                            h1720
                        }
                    };
                    let a1722 = 1u64;
                    nat_eq(a1721, a1722)
                } else {
                    false
                }
            } else {
                let a1724 = match v2.clone() {
                    Adt2::C0(_, h1723, _, _, _, _, _, _, _, _, _) => {
                        h1723
                    }
                };
                let a1725 = 1u64;
                if if if nat_eq(a1724, a1725) {
                    let a1728 = match v1.clone() {
                        Adt2::C0(_, _, h1726, _, _, _, _, _, _, _, _) => {
                            h1726
                        }
                    };
                    let a1729 = match v2.clone() {
                        Adt2::C0(_, _, h1727, _, _, _, _, _, _, _, _) => {
                            h1727
                        }
                    };
                    nat_eq(a1728, a1729)
                } else {
                    false
                } {
                    let a1732 = match v1.clone() {
                        Adt2::C0(_, _, _, h1730, _, _, _, _, _, _, _) => {
                            h1730
                        }
                    };
                    let a1733 = match v2.clone() {
                        Adt2::C0(_, _, _, h1731, _, _, _, _, _, _, _) => {
                            h1731
                        }
                    };
                    nat_eq(a1732, a1733)
                } else {
                    false
                } {
                    let a1736 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1734, _, _, _, _, _, _) => {
                            h1734
                        }
                    };
                    let a1737 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1735, _, _, _, _, _, _) => {
                            h1735
                        }
                    };
                    nat_eq(a1736, a1737)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f18(v1, 1u64);
            Ok(Adt10::C4(f121(v4.clone(), v0, v3)?, {
                let a1738 = v3;
                let a1739 = 0u64;
                if nat_eq(a1738, a1739) {
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

pub fn f61(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1740 = v0;
    let a1741 = 89u64;
    if if nat_eq(a1740, a1741) {
        let a1742 = v1;
        let a1743 = 15u64;
        nat_eq(a1742, a1743)
    } else {
        false
    } {
        Ok(Some(22u64))
    } else {
        let a1744 = v0;
        let a1745 = 89u64;
        if if nat_eq(a1744, a1745) {
            let a1746 = 8u64;
            let a1747 = v1;
            if nat_le(a1746, a1747) {
                let a1748 = v1;
                let a1749 = 9u64;
                nat_le(a1748, a1749)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Some({
                let a1750 = v1;
                let a1751 = 12u64;
                nat_add(a1750, a1751)?
            }))
        } else {
            let a1752 = v0;
            let a1753 = 89u64;
            if if nat_eq(a1752, a1753) {
                let a1754 = v1;
                let a1755 = 4u64;
                nat_eq(a1754, a1755)
            } else {
                false
            } {
                Ok(Some(19u64))
            } else {
                let a1756 = 10u64;
                let a1757 = v0;
                if if nat_le(a1756, a1757) {
                    let a1758 = v0;
                    let a1759 = 12u64;
                    nat_le(a1758, a1759)
                } else {
                    false
                } {
                    Ok(Some({
                        let a1760 = v0;
                        let a1761 = 3u64;
                        nat_add(a1760, a1761)?
                    }))
                } else {
                    let a1762 = v0;
                    let a1763 = 89u64;
                    if if nat_eq(a1762, a1763) {
                        let a1764 = v1;
                        let a1765 = 7u64;
                        nat_eq(a1764, a1765)
                    } else {
                        false
                    } {
                        Ok(Some(17u64))
                    } else {
                        let a1766 = v0;
                        let a1767 = 89u64;
                        if if nat_eq(a1766, a1767) {
                            let a1768 = v1;
                            let a1769 = 10u64;
                            nat_eq(a1768, a1769)
                        } else {
                            false
                        } {
                            Ok(Some(18u64))
                        } else {
                            let a1770 = v0;
                            let a1771 = 89u64;
                            if if nat_eq(a1770, a1771) {
                                let a1772 = v1;
                                let a1773 = 13u64;
                                nat_eq(a1772, a1773)
                            } else {
                                false
                            } {
                                Ok(Some(16u64))
                            } else {
                                let a1774 = v0;
                                let a1775 = 8u64;
                                if nat_eq(a1774, a1775) {
                                    Ok(Some(5u64))
                                } else {
                                    let a1776 = v0;
                                    let a1777 = 9u64;
                                    if nat_eq(a1776, a1777) {
                                        Ok(Some(6u64))
                                    } else {
                                        let a1778 = v0;
                                        let a1779 = 89u64;
                                        if nat_eq(a1778, a1779) {
                                            let a1780 = v1;
                                            let a1781 = 0u64;
                                            if nat_eq(a1780, a1781) {
                                                Ok(Some(0u64))
                                            } else {
                                                let a1782 = v1;
                                                let a1783 = 1u64;
                                                if nat_eq(a1782, a1783) {
                                                    Ok(Some(1u64))
                                                } else {
                                                    let a1784 = v1;
                                                    let a1785 = 2u64;
                                                    if nat_eq(a1784, a1785) {
                                                        Ok(Some(2u64))
                                                    } else {
                                                        let a1786 = v1;
                                                        let a1787 = 3u64;
                                                        if nat_eq(a1786, a1787) {
                                                            Ok(Some(3u64))
                                                        } else {
                                                            let a1788 = v1;
                                                            let a1789 = 6u64;
                                                            if nat_eq(a1788, a1789) {
                                                                Ok(Some(4u64))
                                                            } else {
                                                                let a1790 = v1;
                                                                let a1791 = 12u64;
                                                                if nat_eq(a1790, a1791) {
                                                                    Ok(Some(7u64))
                                                                } else {
                                                                    let a1792 = v1;
                                                                    let a1793 = 11u64;
                                                                    if nat_eq(a1792, a1793) {
                                                                        Ok(Some(8u64))
                                                                    } else {
                                                                        let a1794 = v1;
                                                                        let a1795 = 18u64;
                                                                        if nat_eq(a1794, a1795) {
                                                                            Ok(Some(9u64))
                                                                        } else {
                                                                            let a1796 = v1;
                                                                            let a1797 = 19u64;
                                                                            if nat_eq(a1796, a1797) {
                                                                                Ok(Some(10u64))
                                                                            } else {
                                                                                let a1798 = v1;
                                                                                let a1799 = 20u64;
                                                                                if nat_eq(a1798, a1799) {
                                                                                    Ok(Some(11u64))
                                                                                } else {
                                                                                    let a1800 = v1;
                                                                                    let a1801 = 21u64;
                                                                                    if nat_eq(a1800, a1801) {
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

pub fn f62(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1802 = f122(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1802 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1803 = v0;
            let a1804 = 2u64;
            if if if if nat_eq(a1803, a1804) {
                true
            } else {
                let a1805 = v0;
                let a1806 = 5u64;
                nat_eq(a1805, a1806)
            } {
                true
            } else {
                let a1807 = v0;
                let a1808 = 6u64;
                nat_eq(a1807, a1808)
            } {
                true
            } else {
                let a1809 = v0;
                let a1810 = 7u64;
                nat_eq(a1809, a1810)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f18(v2.clone(), 1u64), f18(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1811 = v3;
                    match m1811 {
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

pub fn f63(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1813 = match v1.clone() {
        Adt2::C0(h1812, _, _, _, _, _, _, _, _, _, _) => {
            h1812
        }
    };
    let a1814 = 0u64;
    if if if if if nat_eq(a1813, a1814) {
        true
    } else {
        let a1816 = match v1.clone() {
            Adt2::C0(h1815, _, _, _, _, _, _, _, _, _, _) => {
                h1815
            }
        };
        let a1817 = 1u64;
        nat_eq(a1816, a1817)
    } {
        true
    } else {
        let a1819 = match v1.clone() {
            Adt2::C0(h1818, _, _, _, _, _, _, _, _, _, _) => {
                h1818
            }
        };
        let a1820 = 30u64;
        nat_eq(a1819, a1820)
    } {
        let a1823 = match v1.clone() {
            Adt2::C0(h1821, _, _, _, _, _, _, _, _, _, _) => {
                h1821
            }
        };
        let a1824 = match v2.clone() {
            Adt2::C0(h1822, _, _, _, _, _, _, _, _, _, _) => {
                h1822
            }
        };
        if nat_eq(a1823, a1824) {
            true
        } else {
            let a1826 = match v2.clone() {
                Adt2::C0(h1825, _, _, _, _, _, _, _, _, _, _) => {
                    h1825
                }
            };
            let a1827 = 0u64;
            nat_eq(a1826, a1827)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v2.clone(), 1u64)?, {
            let a1829 = match v2.clone() {
                Adt2::C0(h1828, _, _, _, _, _, _, _, _, _, _) => {
                    h1828
                }
            };
            let a1830 = 0u64;
            if nat_eq(a1829, a1830) {
                4u64
            } else {
                2u64
            }
        }, f18(v1.clone(), 1u64), f18(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1831, _, _, _, _, _, _, _, _, _, _) => {
                h1831
            }
        }, match v2 {
            Adt2::C0(h1832, _, _, _, _, _, _, _, _, _, _) => {
                h1832
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f64(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f123(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1833, _, _, _) => {
            h1833
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1834, _, _) => {
            h1834
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1835, _) => {
            h1835
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1836) => {
            h1836
        }
    })
}

pub fn f65(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1838 = v0;
        let a1839 = match v1.clone() {
            Adt3::C0(h1837, _, _, _, _, _, _, _) => {
                h1837
            }
        };
        let a1840 = 0u64;
        nat_quot(a1838, a1839, a1840)
    };
    let v4: u64 = {
        let a1842 = v3;
        let a1843 = match v1.clone() {
            Adt3::C0(_, h1841, _, _, _, _, _, _) => {
                h1841
            }
        };
        let a1844 = 0u64;
        nat_quot(a1842, a1843, a1844)
    };
    let a1845 = v2;
    let a1846 = 0u64;
    if nat_eq(a1845, a1846) {
        let a1848 = v0;
        let a1849 = match v1 {
            Adt3::C0(h1847, _, _, _, _, _, _, _) => {
                h1847
            }
        };
        let a1850 = 0u64;
        nat_rem(a1848, a1849, a1850)
    } else {
        let a1851 = v2;
        let a1852 = 1u64;
        if nat_eq(a1851, a1852) {
            let a1854 = v3;
            let a1855 = match v1 {
                Adt3::C0(_, h1853, _, _, _, _, _, _) => {
                    h1853
                }
            };
            let a1856 = 0u64;
            nat_rem(a1854, a1855, a1856)
        } else {
            let a1857 = v2;
            let a1858 = 2u64;
            if nat_eq(a1857, a1858) {
                let a1860 = v4;
                let a1861 = match v1 {
                    Adt3::C0(_, _, h1859, _, _, _, _, _) => {
                        h1859
                    }
                };
                let a1862 = 0u64;
                nat_rem(a1860, a1861, a1862)
            } else {
                let a1864 = v4;
                let a1865 = match v1 {
                    Adt3::C0(_, _, h1863, _, _, _, _, _) => {
                        h1863
                    }
                };
                let a1866 = 0u64;
                nat_quot(a1864, a1865, a1866)
            }
        }
    }
}

pub fn f66(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1868 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1867) => {
            h1867
        }
    };
    let a1869 = 0u64;
    if nat_eq(a1868, a1869) {
        Ok(v2)
    } else {
        let a1874 = {
            let a1871 = v2;
            let a1872 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1870) => {
                    h1870
                }
            };
            nat_mul(a1871, a1872)?
        };
        let a1875 = f65(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1873, _, _, _, _, _, _) => {
                h1873
            }
        }, 3u64);
        nat_add(a1874, a1875)
    }
}

pub fn f67(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1885 = {
            let a1882 = {
                let a1877 = v2;
                let a1878 = match v0.clone() {
                    Adt6::C0(_, h1876, _, _) => {
                        h1876
                    }
                };
                nat_mul(a1877, a1878)?
            };
            let a1883 = {
                let a1880 = v3;
                let a1881 = match v0.clone() {
                    Adt6::C0(_, _, _, h1879) => {
                        h1879
                    }
                };
                nat_mul(a1880, a1881)?
            };
            nat_add(a1882, a1883)?
        };
        let a1886 = f124(match v0.clone() {
            Adt6::C0(_, _, h1884, _) => {
                h1884
            }
        });
        nat_add(a1885, a1886)?
    };
    let v5: u64 = f125(match v0 {
        Adt6::C0(_, _, h1887, _) => {
            h1887
        }
    });
    let a1888 = v5;
    let a1889 = v4;
    if if nat_le(a1888, a1889) {
        let a1892 = {
            let a1890 = v4;
            let a1891 = v5;
            nat_sub(a1890, a1891)
        };
        let a1893 = v1;
        nat_lt(a1892, a1893)
    } else {
        false
    } {
        Ok(Some({
            let a1894 = v4;
            let a1895 = v5;
            nat_sub(a1894, a1895)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f68(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1896, _, _, _, _, _) => {
            h1896
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1897, _, _, _, _, _, _) => {
            h1897
        }
    };
    let v7: u64 = f65(v1, v6.clone(), 2u64);
    let a1899 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1898) => {
            h1898
        }
    };
    let a1900 = 0u64;
    if nat_eq(a1899, a1900) {
        Ok(Adt9::C8(Some(f64(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1903 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1902, _, _, _, _) => {
                    h1902
                }
            };
            match m1903 {
                Adt3::C0(_, _, h1901, _, _, _, _, _) => {
                    h1901
                }
            }
        };
        let m1909 = f67(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1904, _) => {
                h1904
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1905, _, _, _, _, _) => {
                h1905
            }
        }, {
            let a1906 = v2;
            let a1907 = v8;
            let a1908 = 0u64;
            nat_rem(a1906, a1907, a1908)
        }, v7)?;
        match m1909 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1916 = {
                        let a1914 = {
                            let a1910 = v2;
                            let a1911 = v8;
                            let a1912 = 0u64;
                            nat_quot(a1910, a1911, a1912)
                        };
                        let a1915 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1913) => {
                                h1913
                            }
                        };
                        nat_mul(a1914, a1915)?
                    };
                    let a1917 = f65(v1, v6, 3u64);
                    nat_add(a1916, a1917)?
                };
                Ok(Adt9::C8(Some(f64(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f69(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1918 = v1;
    match m1918 {
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
            f126(f127(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f128(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f129(v0.clone(), v24);
            f126(f130(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1919 = v3;
    match m1919 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f131(v0, v1)?, f131(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f133(v0, v1, v2.clone())?, f131(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f134(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f135(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f136(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f135(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f137(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f138(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f139(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f131(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f140(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v30, v31) => {
            Ok(Adt9::C3(v30, v31, f131(v0, v1)?, f131(v0, v2)?))
        }
        Adt4::C12(v32, v33, v34) => {
            f141(v0, v1, v2, v32, v33, v34)
        }
        Adt4::C13(_, v36, _, _) => {
            Ok(Adt9::C3(v36, v36, f131(v0, v1)?, f131(v0, v2)?))
        }
        Adt4::C14(_, v40, v41) => {
            Ok(Adt9::C3(v40, v41, f131(v0, v1)?, f131(v0, v2)?))
        }
        Adt4::C15 => {
            f142(v0, v1, v2)
        }
        Adt4::C16(v42, v43) => {
            f143(v0, v1, v2, v42, v43)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v44, v45) => {
            f144(v0, v1, v45, v44)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f131(f132(v0, match v2.clone() {
                Adt3::C0(h1920, _, _, _, _, _, _, _) => {
                    h1920
                }
            }, match v2.clone() {
                Adt3::C0(_, h1921, _, _, _, _, _, _) => {
                    h1921
                }
            })?, v1)?, f131(v0, v2)?))
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
            f145(v0, v50, v51, v52)
        }
        Adt4::C26(v58, _, _, _) => {
            let a1922 = v58;
            let a1923 = 64u64;
            if if if nat_eq(a1922, a1923) {
                true
            } else {
                let a1924 = v58;
                let a1925 = 65u64;
                nat_eq(a1924, a1925)
            } {
                true
            } else {
                let a1926 = v58;
                let a1927 = 66u64;
                nat_eq(a1926, a1927)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v57) => {
            f79(v0, v1, v2, v57)
        }
        Adt4::C28(v53) => {
            f76(v0, v1, v2, v53, 0u64)
        }
        Adt4::C29(v54) => {
            f77(v0, v1, v2, v54)
        }
        Adt4::C30(v55, _) => {
            f78(v0, v1, v2, v55)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f109(v3, v4);
    let v6: u64 = {
        let a1928 = v0;
        let a1929 = v5;
        let a1930 = 0u64;
        nat_quot(a1928, a1929, a1930)
    };
    let a1931 = v5;
    let a1932 = 1u64;
    if nat_eq(a1931, a1932) {
        Ok(f126(f146(v3, v4), v3, v4, 0u64, f131(v6, v1)?, 2u64, 0u64, 3u64, f131(v6, v2)?))
    } else {
        let a1936 = {
            let a1933 = v0;
            let a1934 = 2u64;
            let a1935 = 0u64;
            nat_rem(a1933, a1934, a1935)
        };
        let a1937 = 0u64;
        if nat_eq(a1936, a1937) {
            Ok(f126(f146(v3, 0u64), v3, 0u64, 0u64, f131(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f126(f146(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f131(v6, v2)?))
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f119(match v1.clone() {
        Adt3::C0(h1938, _, _, _, _, _, _, _) => {
            h1938
        }
    })?;
    let v5: u64 = {
        let a1939 = v0;
        let a1940 = v4;
        let a1941 = 0u64;
        nat_quot(a1939, a1940, a1941)
    };
    let v6: u64 = {
        let a1942 = v0;
        let a1943 = v4;
        let a1944 = 0u64;
        nat_rem(a1942, a1943, a1944)
    };
    let a1945 = v6;
    let a1946 = 0u64;
    if nat_eq(a1945, a1946) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1947 = v6;
        let a1948 = 1u64;
        if nat_eq(a1947, a1948) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1951 = {
                let a1949 = v6;
                let a1950 = 1u64;
                nat_add(a1949, a1950)?
            };
            let a1952 = v4;
            if nat_eq(a1951, a1952) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f131(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1953 = v6;
                    let a1954 = 2u64;
                    nat_sub(a1953, a1954)
                };
                let v8: u64 = {
                    let a1955 = v7;
                    let a1956 = 3u64;
                    let a1957 = 0u64;
                    nat_quot(a1955, a1956, a1957)
                };
                let v9: u64 = {
                    let a1958 = v7;
                    let a1959 = 3u64;
                    let a1960 = 0u64;
                    nat_rem(a1958, a1959, a1960)
                };
                let v10: u64 = f131({
                    let a1964 = {
                        let a1962 = v5;
                        let a1963 = match v1.clone() {
                            Adt3::C0(h1961, _, _, _, _, _, _, _) => {
                                h1961
                            }
                        };
                        nat_mul(a1962, a1963)?
                    };
                    let a1965 = v8;
                    nat_add(a1964, a1965)?
                }, v1)?;
                let a1966 = v9;
                let a1967 = 0u64;
                if nat_eq(a1966, a1967) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1968 = v9;
                    let a1969 = 1u64;
                    if nat_eq(a1968, a1969) {
                        let a1970 = v3;
                        let a1971 = 2u64;
                        if nat_eq(a1970, a1971) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1972 = v3;
                        let a1973 = 2u64;
                        if if nat_eq(a1972, a1973) {
                            true
                        } else {
                            let a1974 = v3;
                            let a1975 = 3u64;
                            nat_eq(a1974, a1975)
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

pub fn f73(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1977 = 2u64;
        let a1978 = match v2.clone() {
            Adt3::C0(_, h1976, _, _, _, _, _, _) => {
                h1976
            }
        };
        nat_mul(a1977, a1978)?
    };
    let a1979 = v1;
    let a1980 = v6;
    if nat_lt(a1979, a1980) {
        let a1984 = {
            let a1981 = v1;
            let a1982 = 2u64;
            let a1983 = 0u64;
            nat_rem(a1981, a1982, a1983)
        };
        let a1985 = 0u64;
        if nat_eq(a1984, a1985) {
            Ok(Adt9::C4(19u64, 4u64, f131({
                let a1986 = v1;
                let a1987 = 2u64;
                let a1988 = 0u64;
                nat_quot(a1986, a1987, a1988)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1989 = v5;
            let a1990 = 2147483648u64;
            if if nat_lt(a1989, a1990) {
                let a1992 = v5;
                let a1993 = match v3 {
                    Adt3::C0(_, h1991, _, _, _, _, _, _) => {
                        h1991
                    }
                };
                nat_lt(a1992, a1993)
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
            let a1996 = match v3.clone() {
                Adt3::C0(h1994, _, _, _, _, _, _, _) => {
                    h1994
                }
            };
            let a1997 = match v3.clone() {
                Adt3::C0(_, h1995, _, _, _, _, _, _) => {
                    h1995
                }
            };
            nat_mul(a1996, a1997)?
        };
        let a2000 = {
            let a1998 = v1;
            let a1999 = v6;
            nat_sub(a1998, a1999)
        };
        let a2001 = v7;
        if nat_lt(a2000, a2001) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f131({
                let a2002 = v1;
                let a2003 = v6;
                nat_sub(a2002, a2003)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2006 = {
                    let a2004 = v1;
                    let a2005 = v6;
                    nat_sub(a2004, a2005)
                };
                let a2007 = v7;
                nat_sub(a2006, a2007)
            };
            let v9: u64 = {
                let a2009 = 1u64;
                let a2010 = f147(v0, match v2.clone() {
                    Adt3::C0(h2008, _, _, _, _, _, _, _) => {
                        h2008
                    }
                })?;
                nat_add(a2009, a2010)?
            };
            let v10: u64 = {
                let a2011 = v8;
                let a2012 = v9;
                let a2013 = 0u64;
                nat_quot(a2011, a2012, a2013)
            };
            let v11: u64 = {
                let a2014 = v8;
                let a2015 = v9;
                let a2016 = 0u64;
                nat_rem(a2014, a2015, a2016)
            };
            let a2017 = v11;
            let a2018 = 0u64;
            if nat_eq(a2017, a2018) {
                Ok(Adt9::C4(19u64, 4u64, f131(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f98(v0) {
                    let a2022 = {
                        let a2019 = v11;
                        let a2020 = 2u64;
                        let a2021 = 0u64;
                        nat_rem(a2019, a2020, a2021)
                    };
                    let a2023 = 1u64;
                    if nat_eq(a2022, a2023) {
                        Ok(Adt9::C4(6u64, 0u64, f131({
                            let a2032 = {
                                let a2025 = v10;
                                let a2026 = match v2.clone() {
                                    Adt3::C0(h2024, _, _, _, _, _, _, _) => {
                                        h2024
                                    }
                                };
                                nat_mul(a2025, a2026)?
                            };
                            let a2033 = {
                                let a2029 = {
                                    let a2027 = v11;
                                    let a2028 = 1u64;
                                    nat_sub(a2027, a2028)
                                };
                                let a2030 = 2u64;
                                let a2031 = 0u64;
                                nat_quot(a2029, a2030, a2031)
                            };
                            nat_add(a2032, a2033)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f131({
                            let a2042 = {
                                let a2035 = v5;
                                let a2036 = match v2.clone() {
                                    Adt3::C0(h2034, _, _, _, _, _, _, _) => {
                                        h2034
                                    }
                                };
                                nat_mul(a2035, a2036)?
                            };
                            let a2043 = {
                                let a2039 = {
                                    let a2037 = v11;
                                    let a2038 = 1u64;
                                    nat_sub(a2037, a2038)
                                };
                                let a2040 = 2u64;
                                let a2041 = 0u64;
                                nat_quot(a2039, a2040, a2041)
                            };
                            nat_add(a2042, a2043)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f131({
                            let a2052 = {
                                let a2045 = v5;
                                let a2046 = match v2 {
                                    Adt3::C0(h2044, _, _, _, _, _, _, _) => {
                                        h2044
                                    }
                                };
                                nat_mul(a2045, a2046)?
                            };
                            let a2053 = {
                                let a2049 = {
                                    let a2047 = v11;
                                    let a2048 = 1u64;
                                    nat_sub(a2047, a2048)
                                };
                                let a2050 = 2u64;
                                let a2051 = 0u64;
                                nat_quot(a2049, a2050, a2051)
                            };
                            nat_add(a2052, a2053)?
                        }, v3)?))
                    }
                } else {
                    f148(v0, v10, {
                        let a2054 = v11;
                        let a2055 = 1u64;
                        nat_sub(a2054, a2055)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2056 = v1;
    let a2057 = f149(v2.clone(), v4.clone())?;
    if nat_lt(a2056, a2057) {
        let a2060 = v1;
        let a2061 = {
            let a2058 = 2u64;
            let a2059 = f120(v4.clone())?;
            nat_mul(a2058, a2059)?
        };
        if nat_lt(a2060, a2061) {
            let a2065 = {
                let a2062 = v1;
                let a2063 = 2u64;
                let a2064 = 0u64;
                nat_rem(a2062, a2063, a2064)
            };
            let a2066 = 0u64;
            if nat_eq(a2065, a2066) {
                f150({
                    let a2067 = v1;
                    let a2068 = 2u64;
                    let a2069 = 0u64;
                    nat_quot(a2067, a2068, a2069)
                }, v4)
            } else {
                if f151(match v2 {
                    Adt3::C0(_, h2070, _, _, _, _, _, _) => {
                        h2070
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2073 = v1;
                let a2074 = {
                    let a2071 = 2u64;
                    let a2072 = f120(v4.clone())?;
                    nat_mul(a2071, a2072)?
                };
                nat_sub(a2073, a2074)
            };
            let v7: u64 = {
                let a2078 = v6;
                let a2079 = {
                    let a2076 = 1u64;
                    let a2077 = match v2.clone() {
                        Adt3::C0(h2075, _, _, _, _, _, _, _) => {
                            h2075
                        }
                    };
                    nat_add(a2076, a2077)?
                };
                let a2080 = 0u64;
                nat_quot(a2078, a2079, a2080)
            };
            let v8: u64 = {
                let a2084 = v6;
                let a2085 = {
                    let a2082 = 1u64;
                    let a2083 = match v2.clone() {
                        Adt3::C0(h2081, _, _, _, _, _, _, _) => {
                            h2081
                        }
                    };
                    nat_add(a2082, a2083)?
                };
                let a2086 = 0u64;
                nat_rem(a2084, a2085, a2086)
            };
            let a2087 = v8;
            let a2088 = 0u64;
            if nat_eq(a2087, a2088) {
                f150(v7, v4)
            } else {
                f152(v0, v7, {
                    let a2089 = v8;
                    let a2090 = 1u64;
                    nat_sub(a2089, a2090)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2093 = match v4.clone() {
            Adt3::C0(h2091, _, _, _, _, _, _, _) => {
                h2091
            }
        };
        let a2094 = match v4.clone() {
            Adt3::C0(_, h2092, _, _, _, _, _, _) => {
                h2092
            }
        };
        nat_mul(a2093, a2094)?
    };
    let v7: u64 = {
        let a2095 = 2u64;
        let a2096 = v6;
        nat_mul(a2095, a2096)?
    };
    let v8: u64 = f153(match v3.clone() {
        Adt3::C0(_, h2097, _, _, _, _, _, _) => {
            h2097
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2098, _, _, _, _, _) => {
            h2098
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2099, _, _, _, _) => {
            h2099
        }
    });
    let a2100 = v0;
    let a2101 = v7;
    if nat_lt(a2100, a2101) {
        let a2105 = {
            let a2102 = v0;
            let a2103 = 2u64;
            let a2104 = 0u64;
            nat_rem(a2102, a2103, a2104)
        };
        let a2106 = 0u64;
        if nat_eq(a2105, a2106) {
            Ok(Adt9::C4(19u64, 5u64, f131({
                let a2107 = v0;
                let a2108 = 2u64;
                let a2109 = 0u64;
                nat_quot(a2107, a2108, a2109)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f154(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2110 = v0;
            let a2111 = v7;
            nat_sub(a2110, a2111)
        };
        let v10: u64 = {
            let a2115 = v9;
            let a2116 = {
                let a2113 = 1u64;
                let a2114 = match v1.clone() {
                    Adt3::C0(h2112, _, _, _, _, _, _, _) => {
                        h2112
                    }
                };
                nat_add(a2113, a2114)?
            };
            let a2117 = 0u64;
            nat_quot(a2115, a2116, a2117)
        };
        let v11: u64 = {
            let a2121 = v9;
            let a2122 = {
                let a2119 = 1u64;
                let a2120 = match v1.clone() {
                    Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                        h2118
                    }
                };
                nat_add(a2119, a2120)?
            };
            let a2123 = 0u64;
            nat_rem(a2121, a2122, a2123)
        };
        let a2124 = v11;
        let a2125 = 0u64;
        if nat_eq(a2124, a2125) {
            Ok(Adt9::C4(19u64, 5u64, f131({
                let a2126 = v10;
                let a2127 = v6;
                let a2128 = 0u64;
                nat_rem(a2126, a2127, a2128)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f154(v5, v8) {
                let v12: u64 = {
                    let a2134 = {
                        let a2130 = v10;
                        let a2131 = match v1.clone() {
                            Adt3::C0(h2129, _, _, _, _, _, _, _) => {
                                h2129
                            }
                        };
                        nat_mul(a2130, a2131)?
                    };
                    let a2135 = {
                        let a2132 = v11;
                        let a2133 = 1u64;
                        nat_sub(a2132, a2133)
                    };
                    nat_add(a2134, a2135)?
                };
                let a2136 = v5;
                let a2137 = 4294967295u64;
                if nat_eq(a2136, a2137) {
                    Ok(Adt9::C1(false, f131(v12, v1)?, f131(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f131(v12, v1)?, 4u64, f131({
                        let a2143 = {
                            let a2139 = v5;
                            let a2140 = match v3.clone() {
                                Adt3::C0(h2138, _, _, _, _, _, _, _) => {
                                    h2138
                                }
                            };
                            nat_mul(a2139, a2140)?
                        };
                        let a2144 = {
                            let a2141 = v11;
                            let a2142 = 1u64;
                            nat_sub(a2141, a2142)
                        };
                        nat_add(a2143, a2144)?
                    }, v3)?, 3u64, f131(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f103(v3.clone())?;
    let v6: u64 = {
        let a2145 = v0;
        let a2146 = v5;
        let a2147 = 0u64;
        nat_quot(a2145, a2146, a2147)
    };
    let v7: u64 = {
        let a2148 = v0;
        let a2149 = v5;
        let a2150 = 0u64;
        nat_rem(a2148, a2149, a2150)
    };
    let v8: u64 = f155(v3.clone())?;
    let a2151 = v7;
    let a2152 = 0u64;
    if nat_eq(a2151, a2152) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2153, _) => {
                h2153
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2157 = v5;
        let a2158 = {
            let a2155 = v7;
            let a2156 = if match v3.clone() {
                Adt7::C0(_, _, _, h2154, _) => {
                    h2154
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2155, a2156)?
        };
        if nat_le(a2157, a2158) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2159, _) => {
                    h2159
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f131(v6, v2)?))
            } else {
                let a2162 = {
                    let a2160 = v7;
                    let a2161 = 2u64;
                    nat_add(a2160, a2161)?
                };
                let a2163 = v5;
                if nat_eq(a2162, a2163) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2164, _, _) => {
                            h2164
                        }
                    } {
                        let a2171 = {
                            let m2167 = match v3.clone() {
                                Adt7::C0(h2166, _, _, _, _) => {
                                    h2166
                                }
                            };
                            match m2167 {
                                Adt6::C0(h2165, _, _, _) => {
                                    h2165
                                }
                            }
                        };
                        let a2172 = {
                            let m2170 = match v3 {
                                Adt7::C0(_, h2169, _, _, _) => {
                                    h2169
                                }
                            };
                            match m2170 {
                                Adt6::C0(h2168, _, _, _) => {
                                    h2168
                                }
                            }
                        };
                        nat_mul(a2171, a2172)?
                    } else {
                        f156(match v3 {
                            Adt7::C0(h2173, _, _, _, _) => {
                                h2173
                            }
                        }, match v1 {
                            Adt3::C0(h2174, _, _, _, _, _, _, _) => {
                                h2174
                            }
                        }, f65(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2175 = v9;
                        let a2176 = 0u64;
                        if nat_eq(a2175, a2176) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f131(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2179 = {
                    let a2177 = v7;
                    let a2178 = 1u64;
                    nat_sub(a2177, a2178)
                };
                let a2180 = v8;
                let a2181 = 0u64;
                nat_rem(a2179, a2180, a2181)
            };
            let m2187 = f157(v3.clone(), v1, v2, v6, {
                let a2184 = {
                    let a2182 = v7;
                    let a2183 = 1u64;
                    nat_sub(a2182, a2183)
                };
                let a2185 = v8;
                let a2186 = 0u64;
                nat_quot(a2184, a2185, a2186)
            })?;
            match m2187 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2189 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2188) => {
                            h2188
                        }
                    };
                    let a2190 = 1u64;
                    if if nat_eq(a2189, a2190) {
                        let a2191 = v10;
                        let a2192 = 0u64;
                        nat_eq(a2191, a2192)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2193, _) => {
                                h2193
                            }
                        } {
                            let a2196 = {
                                let a2194 = v10;
                                let a2195 = 1u64;
                                nat_add(a2194, a2195)?
                            };
                            let a2197 = v8;
                            if nat_eq(a2196, a2197) {
                                let a2198 = v4;
                                let a2199 = 2u64;
                                if nat_eq(a2198, a2199) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2201 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2200) => {
                                                h2200
                                            }
                                        };
                                        let a2202 = 1u64;
                                        if nat_eq(a2201, a2202) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2204 = match v3 {
                                            Adt7::C0(_, _, _, _, h2203) => {
                                                h2203
                                            }
                                        };
                                        let a2205 = 1u64;
                                        if nat_eq(a2204, a2205) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2207 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2206) => {
                                            h2206
                                        }
                                    };
                                    let a2208 = 1u64;
                                    if nat_eq(a2207, a2208) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2210 = match v3 {
                                        Adt7::C0(_, _, _, _, h2209) => {
                                            h2209
                                        }
                                    };
                                    let a2211 = 1u64;
                                    if nat_eq(a2210, a2211) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2213 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2212) => {
                                        h2212
                                    }
                                };
                                let a2214 = 1u64;
                                if nat_eq(a2213, a2214) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2216 = match v3 {
                                    Adt7::C0(_, _, _, _, h2215) => {
                                        h2215
                                    }
                                };
                                let a2217 = 1u64;
                                if nat_eq(a2216, a2217) {
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

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f101(match v3.clone() {
        Adt3::C0(h2218, _, _, _, _, _, _, _) => {
            h2218
        }
    })?;
    let v5: u64 = {
        let a2219 = v0;
        let a2220 = v4;
        let a2221 = 0u64;
        nat_rem(a2219, a2220, a2221)
    };
    let v6: u64 = {
        let a2222 = v0;
        let a2223 = v4;
        let a2224 = 0u64;
        nat_quot(a2222, a2223, a2224)
    };
    let a2225 = v5;
    let a2226 = 0u64;
    if nat_eq(a2225, a2226) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2229 = {
            let a2227 = v5;
            let a2228 = 1u64;
            nat_add(a2227, a2228)?
        };
        let a2230 = v4;
        if nat_eq(a2229, a2230) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f131(v6, v2)?))
        } else {
            let a2234 = {
                let a2231 = v5;
                let a2232 = 2u64;
                let a2233 = 0u64;
                nat_rem(a2231, a2232, a2233)
            };
            let a2235 = 0u64;
            if nat_eq(a2234, a2235) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2238 = {
                        let a2236 = v5;
                        let a2237 = 1u64;
                        nat_sub(a2236, a2237)
                    };
                    let a2239 = 2u64;
                    let a2240 = 0u64;
                    nat_quot(a2238, a2239, a2240)
                };
                let v8: u64 = f65(v6, v2.clone(), 0u64);
                let v9: u64 = f65(v6, v2.clone(), 1u64);
                let v10: u64 = f65(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f64(v1, {
                    let a2241 = v9;
                    let a2242 = v7;
                    nat_add(a2241, a2242)?
                }, v8, v10, 0u64)?, 4u64, f64(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f100(v3.clone())?;
    let v5: u64 = {
        let a2243 = v0;
        let a2244 = v4;
        let a2245 = 0u64;
        nat_rem(a2243, a2244, a2245)
    };
    let v6: u64 = {
        let a2246 = v0;
        let a2247 = v4;
        let a2248 = 0u64;
        nat_quot(a2246, a2247, a2248)
    };
    let a2249 = v5;
    let a2250 = 0u64;
    if nat_eq(a2249, a2250) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2253 = {
            let a2251 = v5;
            let a2252 = 1u64;
            nat_add(a2251, a2252)?
        };
        let a2254 = v4;
        if nat_eq(a2253, a2254) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f131(v6, v2)?))
        } else {
            let a2258 = {
                let a2255 = v5;
                let a2256 = 2u64;
                let a2257 = 0u64;
                nat_rem(a2255, a2256, a2257)
            };
            let a2259 = 0u64;
            if nat_eq(a2258, a2259) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2261 = f65(v6, v2.clone(), 0u64);
                    let a2262 = match v3.clone() {
                        Adt6::C0(_, _, h2260, _) => {
                            h2260
                        }
                    };
                    nat_add(a2261, a2262)?
                };
                let v8: u64 = {
                    let a2270 = {
                        let a2267 = {
                            let a2264 = v7;
                            let a2265 = match v3.clone() {
                                Adt6::C0(_, h2263, _, _) => {
                                    h2263
                                }
                            };
                            nat_add(a2264, a2265)?
                        };
                        let a2268 = match v3.clone() {
                            Adt6::C0(h2266, _, _, _) => {
                                h2266
                            }
                        };
                        nat_sub(a2267, a2268)
                    };
                    let a2271 = match v3.clone() {
                        Adt6::C0(_, h2269, _, _) => {
                            h2269
                        }
                    };
                    let a2272 = 0u64;
                    nat_quot(a2270, a2271, a2272)
                };
                let v9: u64 = {
                    let a2278 = v8;
                    let a2279 = {
                        let a2275 = {
                            let a2273 = v5;
                            let a2274 = 1u64;
                            nat_sub(a2273, a2274)
                        };
                        let a2276 = 2u64;
                        let a2277 = 0u64;
                        nat_quot(a2275, a2276, a2277)
                    };
                    nat_add(a2278, a2279)?
                };
                let v10: u64 = {
                    let a2281 = v9;
                    let a2282 = match v3.clone() {
                        Adt6::C0(_, h2280, _, _) => {
                            h2280
                        }
                    };
                    nat_mul(a2281, a2282)?
                };
                let v11: u64 = {
                    let a2283 = v7;
                    let a2284 = v10;
                    nat_sub(a2283, a2284)
                };
                let a2286 = v9;
                let a2287 = match v1.clone() {
                    Adt3::C0(_, h2285, _, _, _, _, _, _) => {
                        h2285
                    }
                };
                if if if nat_lt(a2286, a2287) {
                    let a2288 = v10;
                    let a2289 = v7;
                    nat_le(a2288, a2289)
                } else {
                    false
                } {
                    let a2291 = v11;
                    let a2292 = match v3.clone() {
                        Adt6::C0(h2290, _, _, _) => {
                            h2290
                        }
                    };
                    nat_lt(a2291, a2292)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f64(v1, {
                        let a2296 = {
                            let a2294 = f65(v6, v2, 1u64);
                            let a2295 = match v3 {
                                Adt6::C0(h2293, _, _, _) => {
                                    h2293
                                }
                            };
                            nat_mul(a2294, a2295)?
                        };
                        let a2297 = v11;
                        nat_add(a2296, a2297)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2299 = v5;
        let a2300 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2298, _) => {
                h2298
            }
        };
        let a2301 = 0u64;
        nat_rem(a2299, a2300, a2301)
    };
    let v7: u64 = {
        let a2303 = v5;
        let a2304 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2302, _) => {
                h2302
            }
        };
        let a2305 = 0u64;
        nat_quot(a2303, a2304, a2305)
    };
    let v8: Option<u64> = f67(match v3.clone() {
        Adt5::C0(h2306, _, _, _, _, _, _, _) => {
            h2306
        }
    }, match v1.clone() {
        Adt3::C0(h2307, _, _, _, _, _, _, _) => {
            h2307
        }
    }, f65(v0, v2.clone(), 1u64), {
        let a2311 = v4;
        let a2312 = {
            let m2310 = match v3.clone() {
                Adt5::C0(h2309, _, _, _, _, _, _, _) => {
                    h2309
                }
            };
            match m2310 {
                Adt6::C0(h2308, _, _, _) => {
                    h2308
                }
            }
        };
        let a2313 = 0u64;
        nat_rem(a2311, a2312, a2313)
    })?;
    let v9: u64 = {
        let a2317 = v4;
        let a2318 = {
            let m2316 = match v3.clone() {
                Adt5::C0(h2315, _, _, _, _, _, _, _) => {
                    h2315
                }
            };
            match m2316 {
                Adt6::C0(h2314, _, _, _) => {
                    h2314
                }
            }
        };
        let a2319 = 0u64;
        nat_quot(a2317, a2318, a2319)
    };
    let v10: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, h2320, _, _, _, _, _, _) => {
            h2320
        }
    }, match v1.clone() {
        Adt3::C0(_, h2321, _, _, _, _, _, _) => {
            h2321
        }
    }, f65(v0, v2.clone(), 2u64), {
        let a2325 = v9;
        let a2326 = {
            let m2324 = match v3.clone() {
                Adt5::C0(_, h2323, _, _, _, _, _, _) => {
                    h2323
                }
            };
            match m2324 {
                Adt6::C0(h2322, _, _, _) => {
                    h2322
                }
            }
        };
        let a2327 = 0u64;
        nat_rem(a2325, a2326, a2327)
    })?;
    let v11: u64 = {
        let a2331 = v9;
        let a2332 = {
            let m2330 = match v3.clone() {
                Adt5::C0(_, h2329, _, _, _, _, _, _) => {
                    h2329
                }
            };
            match m2330 {
                Adt6::C0(h2328, _, _, _) => {
                    h2328
                }
            }
        };
        let a2333 = 0u64;
        nat_quot(a2331, a2332, a2333)
    };
    let v12: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, _, h2334, _, _, _, _, _) => {
            h2334
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2335, _, _, _, _, _) => {
            h2335
        }
    }, v6, {
        let a2339 = v11;
        let a2340 = {
            let m2338 = match v3.clone() {
                Adt5::C0(_, _, h2337, _, _, _, _, _) => {
                    h2337
                }
            };
            match m2338 {
                Adt6::C0(h2336, _, _, _) => {
                    h2336
                }
            }
        };
        let a2341 = 0u64;
        nat_rem(a2339, a2340, a2341)
    })?;
    let v13: u64 = {
        let a2345 = v11;
        let a2346 = {
            let m2344 = match v3.clone() {
                Adt5::C0(_, _, h2343, _, _, _, _, _) => {
                    h2343
                }
            };
            match m2344 {
                Adt6::C0(h2342, _, _, _) => {
                    h2342
                }
            }
        };
        let a2347 = 0u64;
        nat_quot(a2345, a2346, a2347)
    };
    let v17: Adt12 = {
        let m2348 = v8;
        match m2348 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2349 = v10;
                match m2349 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2350 = v12;
                        match m2350 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2359 = {
                                        let a2354 = f64(v1, v14, v15, v16, 0u64)?;
                                        let a2355 = {
                                            let a2352 = v7;
                                            let a2353 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2351, _, _, _) => {
                                                    h2351
                                                }
                                            };
                                            nat_mul(a2352, a2353)?
                                        };
                                        nat_add(a2354, a2355)?
                                    };
                                    let a2360 = {
                                        let a2357 = v13;
                                        let a2358 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2356, _, _, _, _) => {
                                                h2356
                                            }
                                        };
                                        nat_mul(a2357, a2358)?
                                    };
                                    nat_add(a2359, a2360)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2361, _, _) => {
            h2361
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2362) => {
            h2362
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f131(v0, v2)?), {
        let a2364 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2363, _, _) => {
                h2363
            }
        };
        let a2365 = 0u64;
        if nat_eq(a2364, a2365) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2367 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2366) => {
                h2366
            }
        };
        let a2368 = 0u64;
        if nat_eq(a2367, a2368) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f80(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2381 = f158(v0, v1, match v2.clone() {
        Adt3::C0(h2369, _, _, _, _, _, _, _) => {
            h2369
        }
    }, match v2.clone() {
        Adt3::C0(_, h2370, _, _, _, _, _, _) => {
            h2370
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2371, _, _, _, _, _) => {
            h2371
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2372, _, _, _, _) => {
            h2372
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2373, _, _, _) => {
            h2373
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2374, _, _) => {
            h2374
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2375, _) => {
            h2375
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2376) => {
            h2376
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2377, _, _, _) => {
            h2377
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2378, _, _) => {
            h2378
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2379, _) => {
            h2379
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2380) => {
            h2380
        }
    }, v4)?;
    match m2381 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f13(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2384 = v6;
        let a2385 = {
            let a2382 = 3u64;
            let a2383 = v5;
            nat_add(a2382, a2383)?
        };
        nat_sub(a2384, a2385)
    };
    let a2387 = v0;
    let a2388 = f33(v3, match v1.clone() {
        Adt3::C0(h2386, _, _, _, _, _, _, _) => {
            h2386
        }
    }, v5, v8)?;
    if nat_lt(a2387, a2388) {
        let v9: u64 = f94(match v1.clone() {
            Adt3::C0(h2389, _, _, _, _, _, _, _) => {
                h2389
            }
        }, v5, v8)?;
        let v10: u64 = f95(v3, match v1.clone() {
            Adt3::C0(h2390, _, _, _, _, _, _, _) => {
                h2390
            }
        }, v5)?;
        let a2391 = v0;
        let a2392 = v9;
        if nat_lt(a2391, a2392) {
            f159(v0, v2, f27(v3), {
                let a2393 = 3u64;
                let a2394 = v5;
                nat_add(a2393, a2394)?
            }, v7)
        } else {
            let a2397 = v0;
            let a2398 = {
                let a2395 = v9;
                let a2396 = v10;
                nat_add(a2395, a2396)?
            };
            if nat_lt(a2397, a2398) {
                let v11: u64 = {
                    let a2399 = v0;
                    let a2400 = v9;
                    nat_sub(a2399, a2400)
                };
                let v12: u64 = f160(v3, match v1.clone() {
                    Adt3::C0(h2401, _, _, _, _, _, _, _) => {
                        h2401
                    }
                })?;
                f161({
                    let a2402 = v11;
                    let a2403 = v12;
                    let a2404 = 0u64;
                    nat_rem(a2402, a2403, a2404)
                }, {
                    let a2405 = v11;
                    let a2406 = v12;
                    let a2407 = 0u64;
                    nat_quot(a2405, a2406, a2407)
                }, v1, v3, v4, v7)
            } else {
                f162({
                    let a2410 = v0;
                    let a2411 = {
                        let a2408 = v9;
                        let a2409 = v10;
                        nat_add(a2408, a2409)?
                    };
                    nat_sub(a2410, a2411)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2412 = v0;
    let a2413 = f17(v1)?;
    if nat_lt(a2412, a2413) {
        let a2414 = v0;
        let a2415 = 4u64;
        if nat_lt(a2414, a2415) {
            let a2416 = v0;
            let a2417 = 0u64;
            if nat_eq(a2416, a2417) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2418 = v0;
                let a2419 = 1u64;
                if nat_eq(a2418, a2419) {
                    let a2420 = v5;
                    let a2421 = 0u64;
                    if nat_eq(a2420, a2421) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2422 = v0;
                    let a2423 = 2u64;
                    if nat_eq(a2422, a2423) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2424 = v0;
            let a2425 = 4u64;
            if if nat_eq(a2424, a2425) {
                let a2426 = f163(v5)?;
                let a2427 = v1;
                !nat_eq(a2426, a2427)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2430 = {
                        let a2428 = v0;
                        let a2429 = 4u64;
                        nat_sub(a2428, a2429)
                    };
                    let a2431 = 3u64;
                    let a2432 = 0u64;
                    nat_quot(a2430, a2431, a2432)
                };
                let v7: u64 = {
                    let a2435 = {
                        let a2433 = v0;
                        let a2434 = 4u64;
                        nat_sub(a2433, a2434)
                    };
                    let a2436 = 3u64;
                    let a2437 = 0u64;
                    nat_rem(a2435, a2436, a2437)
                };
                let a2438 = v7;
                let a2439 = 0u64;
                if nat_eq(a2438, a2439) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2440 = v7;
                    let a2441 = 1u64;
                    if nat_eq(a2440, a2441) {
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

pub fn f83(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2442 = v1;
    let a2443 = 0u64;
    if nat_eq(a2442, a2443) {
        if v0 {
            let a2444 = v3;
            let a2445 = 0u64;
            if if nat_eq(a2444, a2445) {
                let a2446 = v4;
                let a2447 = 43u64;
                nat_eq(a2446, a2447)
            } else {
                false
            } {
                f146(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2448 = v1;
        let a2449 = 1u64;
        if nat_eq(a2448, a2449) {
            Adt11::C1(v2)
        } else {
            let a2450 = v1;
            let a2451 = 2u64;
            if nat_eq(a2450, a2451) {
                Adt11::C5(v2)
            } else {
                let a2452 = v1;
                let a2453 = 3u64;
                if nat_eq(a2452, a2453) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2454 = v1;
                    let a2455 = 4u64;
                    if nat_eq(a2454, a2455) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: u64) -> Adt12 {
    let a2456 = v0;
    let a2457 = 0u64;
    if nat_eq(a2456, a2457) {
        Adt12::C0(0u64, v1)
    } else {
        let a2458 = v0;
        let a2459 = 1u64;
        if nat_eq(a2458, a2459) {
            Adt12::C1(v1)
        } else {
            let a2460 = v0;
            let a2461 = 2u64;
            if nat_eq(a2460, a2461) {
                Adt12::C2(v1)
            } else {
                let a2462 = v0;
                let a2463 = 3u64;
                if nat_eq(a2462, a2463) {
                    Adt12::C3(v1)
                } else {
                    let a2464 = v0;
                    let a2465 = 13u64;
                    if nat_lt(a2464, a2465) {
                        Adt12::C0({
                            let a2466 = v0;
                            let a2467 = 3u64;
                            nat_sub(a2466, a2467)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64) -> Adt13 {
    let a2468 = v0;
    let a2469 = 1u64;
    if nat_eq(a2468, a2469) {
        Adt13::C0(v1)
    } else {
        let a2470 = v0;
        let a2471 = 3u64;
        if if nat_eq(a2470, a2471) {
            true
        } else {
            let a2472 = v0;
            let a2473 = 6u64;
            nat_eq(a2472, a2473)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f86(v0: Adt6, v1: u64) -> R<u64> {
    let a2475 = 0u64;
    let a2476 = match v0.clone() {
        Adt6::C0(h2474, _, _, _) => {
            h2474
        }
    };
    if if if if if nat_lt(a2475, a2476) {
        let a2478 = 0u64;
        let a2479 = match v0.clone() {
            Adt6::C0(_, h2477, _, _) => {
                h2477
            }
        };
        nat_lt(a2478, a2479)
    } else {
        false
    } {
        let a2481 = 0u64;
        let a2482 = match v0.clone() {
            Adt6::C0(_, _, _, h2480) => {
                h2480
            }
        };
        nat_lt(a2481, a2482)
    } else {
        false
    } {
        let a2484 = match v0.clone() {
            Adt6::C0(_, h2483, _, _) => {
                h2483
            }
        };
        let a2485 = 2147483648u64;
        nat_lt(a2484, a2485)
    } else {
        false
    } {
        let a2487 = match v0.clone() {
            Adt6::C0(_, _, _, h2486) => {
                h2486
            }
        };
        let a2488 = 2147483648u64;
        nat_lt(a2487, a2488)
    } else {
        false
    } {
        let v2: u64 = {
            let a2492 = v1;
            let a2493 = {
                let a2490 = 2u64;
                let a2491 = f125(match v0.clone() {
                    Adt6::C0(_, _, h2489, _) => {
                        h2489
                    }
                });
                nat_mul(a2490, a2491)?
            };
            nat_add(a2492, a2493)?
        };
        let v3: u64 = {
            let a2505 = {
                let a2500 = 1u64;
                let a2501 = {
                    let a2498 = match v0.clone() {
                        Adt6::C0(_, _, _, h2494) => {
                            h2494
                        }
                    };
                    let a2499 = {
                        let a2496 = match v0.clone() {
                            Adt6::C0(h2495, _, _, _) => {
                                h2495
                            }
                        };
                        let a2497 = 1u64;
                        nat_sub(a2496, a2497)
                    };
                    nat_mul(a2498, a2499)?
                };
                nat_add(a2500, a2501)?
            };
            let a2506 = {
                let a2503 = 2u64;
                let a2504 = f124(match v0.clone() {
                    Adt6::C0(_, _, h2502, _) => {
                        h2502
                    }
                });
                nat_mul(a2503, a2504)?
            };
            nat_add(a2505, a2506)?
        };
        let a2507 = v3;
        let a2508 = v2;
        if nat_le(a2507, a2508) {
            let a2515 = 1u64;
            let a2516 = {
                let a2512 = {
                    let a2509 = v2;
                    let a2510 = v3;
                    nat_sub(a2509, a2510)
                };
                let a2513 = match v0 {
                    Adt6::C0(_, h2511, _, _) => {
                        h2511
                    }
                };
                let a2514 = 0u64;
                nat_quot(a2512, a2513, a2514)
            };
            nat_add(a2515, a2516)
        } else {
            let a2520 = {
                let a2517 = v3;
                let a2518 = v2;
                nat_sub(a2517, a2518)
            };
            let a2521 = match v0 {
                Adt6::C0(_, h2519, _, _) => {
                    h2519
                }
            };
            if nat_lt(a2520, a2521) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f87(v0: u64) -> u64 {
    let v1: u64 = {
        let a2522 = v0;
        let a2523 = 4294967296u64;
        let a2524 = 0u64;
        nat_rem(a2522, a2523, a2524)
    };
    let a2525 = v1;
    let a2526 = 2147483648u64;
    if nat_lt(a2525, a2526) {
        v1
    } else {
        0u64
    }
}

pub fn f88(v0: u64) -> R<u64> {
    let a2529 = 2u64;
    let a2530 = {
        let a2527 = 2u64;
        let a2528 = v0;
        nat_mul(a2527, a2528)?
    };
    nat_add(a2529, a2530)
}

pub fn f89(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2532 = match v1.clone() {
        Adt14::C0(h2531, _, _, _, _, _, _, _, _, _, _) => {
            h2531
        }
    };
    let a2533 = 0u64;
    if if if if nat_eq(a2532, a2533) {
        let a2535 = match v2.clone() {
            Adt14::C0(h2534, _, _, _, _, _, _, _, _, _, _) => {
                h2534
            }
        };
        let a2536 = 0u64;
        nat_eq(a2535, a2536)
    } else {
        false
    } {
        f164(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f165(match v1.clone() {
            Adt14::C0(_, h2537, _, _, _, _, _, _, _, _, _) => {
                h2537
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2538, _, _, _, _, _, _, _, _) => {
                h2538
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2539, _, _, _, _, _, _, _) => {
                h2539
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2540, _, _, _, _, _, _) => {
                h2540
            }
        }) {
            true
        } else {
            if if if if f28(v0) {
                f166(v1.clone(), v2.clone())
            } else {
                false
            } {
                f167(v1.clone())
            } else {
                false
            } {
                f167(v2.clone())
            } else {
                false
            } {
                if f168(v1.clone(), v2.clone())? {
                    true
                } else {
                    f169(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f165(match v1.clone() {
            Adt14::C0(_, h2541, _, _, _, _, _, _, _, _, _) => {
                h2541
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2542, _, _, _, _, _, _, _, _) => {
                h2542
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2543, _, _, _, _, _, _, _) => {
                h2543
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2544, _, _, _, _, _, _) => {
                h2544
            }
        }) {
            0u64
        } else {
            let a2550 = match v1.clone() {
                Adt14::C0(_, _, h2545, _, _, _, _, _, _, _, _) => {
                    h2545
                }
            };
            let a2551 = {
                let a2548 = match v1.clone() {
                    Adt14::C0(_, _, _, h2546, _, _, _, _, _, _, _) => {
                        h2546
                    }
                };
                let a2549 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2547, _, _, _, _, _, _) => {
                        h2547
                    }
                };
                nat_mul(a2548, a2549)?
            };
            nat_mul(a2550, a2551)?
        };
        let v4: u64 = {
            let a2552 = v3;
            let a2553 = 0u64;
            if if nat_eq(a2552, a2553) {
                true
            } else {
                f170(v2.clone())?
            } {
                0u64
            } else {
                let a2555 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2554, _) => {
                        h2554
                    }
                };
                let a2556 = 4u64;
                let a2557 = 0u64;
                nat_quot(a2555, a2556, a2557)
            }
        };
        Ok(Adt15::C1(f171(f172(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2558, _, _, _, _, _) => {
                h2558
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2559, _, _, _, _, _) => {
                h2559
            }
        }), match v1 {
            Adt14::C0(_, h2560, _, _, _, _, _, _, _, _, _) => {
                h2560
            }
        }, v3, v4)?, 4u64, 64u64, v3, f173(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f90(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f174(v3.clone());
    let a2562 = match v2.clone() {
        Adt14::C0(h2561, _, _, _, _, _, _, _, _, _, _) => {
            h2561
        }
    };
    let a2563 = 0u64;
    if if if if nat_eq(a2562, a2563) {
        let a2565 = match v3.clone() {
            Adt14::C0(h2564, _, _, _, _, _, _, _, _, _, _) => {
                h2564
            }
        };
        let a2566 = 0u64;
        nat_eq(a2565, a2566)
    } else {
        false
    } {
        f164(v2.clone(), v3)
    } else {
        false
    } {
        if f165(match v2.clone() {
            Adt14::C0(_, h2567, _, _, _, _, _, _, _, _, _) => {
                h2567
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2568, _, _, _, _, _, _, _, _) => {
                h2568
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2569, _, _, _, _, _, _, _) => {
                h2569
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2570, _, _, _, _, _, _) => {
                h2570
            }
        }) {
            true
        } else {
            let a2572 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2571, _, _, _, _, _) => {
                    h2571
                }
            };
            let a2573 = 4u64;
            if if if if if nat_eq(a2572, a2573) {
                f175(match v2.clone() {
                    Adt14::C0(_, h2574, _, _, _, _, _, _, _, _, _) => {
                        h2574
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2575, _, _, _, _, _, _, _, _) => {
                        h2575
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2576, _, _, _, _, _, _, _) => {
                        h2576
                    }
                }, v0)?
            } else {
                false
            } {
                f167(v2.clone())
            } else {
                false
            } {
                f167(v4.clone())
            } else {
                false
            } {
                let a2578 = f176(v4.clone())?;
                let a2579 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2577, _) => {
                        h2577
                    }
                };
                nat_le(a2578, a2579)
            } else {
                false
            } {
                if f168(v2.clone(), v4.clone())? {
                    true
                } else {
                    f169(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f165(match v2.clone() {
            Adt14::C0(_, h2580, _, _, _, _, _, _, _, _, _) => {
                h2580
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2581, _, _, _, _, _, _, _, _) => {
                h2581
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2582, _, _, _, _, _, _, _) => {
                h2582
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2583, _, _, _, _, _, _) => {
                h2583
            }
        }) {
            true
        } else {
            f170(v4.clone())?
        } {
            0u64
        } else {
            let a2585 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2584, _) => {
                    h2584
                }
            };
            let a2586 = 4u64;
            let a2587 = 0u64;
            nat_quot(a2585, a2586, a2587)
        };
        Ok(Adt15::C1(f177(match v2.clone() {
            Adt14::C0(_, h2588, _, _, _, _, _, _, _, _, _) => {
                h2588
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2589, _, _, _, _, _, _, _, _) => {
                h2589
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2590, _, _, _, _, _, _, _) => {
                h2590
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2591, _, _, _, _, _, _) => {
                h2591
            }
        }, v0, v5)?, 4u64, 65u64, v0, f173(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f91(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f174(v4.clone());
    if if if if if if if v1 {
        let a2593 = match v2.clone() {
            Adt14::C0(h2592, _, _, _, _, _, _, _, _, _, _) => {
                h2592
            }
        };
        let a2594 = 0u64;
        nat_eq(a2593, a2594)
    } else {
        false
    } {
        let a2596 = match v3.clone() {
            Adt14::C0(h2595, _, _, _, _, _, _, _, _, _, _) => {
                h2595
            }
        };
        let a2597 = 0u64;
        nat_eq(a2596, a2597)
    } else {
        false
    } {
        let a2599 = match v4.clone() {
            Adt14::C0(h2598, _, _, _, _, _, _, _, _, _, _) => {
                h2598
            }
        };
        let a2600 = 0u64;
        nat_eq(a2599, a2600)
    } else {
        false
    } {
        f164(v2.clone(), v3.clone())
    } else {
        false
    } {
        f164(v2.clone(), v4)
    } else {
        false
    } {
        if f165(match v2.clone() {
            Adt14::C0(_, h2601, _, _, _, _, _, _, _, _, _) => {
                h2601
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2602, _, _, _, _, _, _, _, _) => {
                h2602
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2603, _, _, _, _, _, _, _) => {
                h2603
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2604, _, _, _, _, _, _) => {
                h2604
            }
        }) {
            true
        } else {
            let a2606 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2605, _, _, _, _, _) => {
                    h2605
                }
            };
            let a2607 = 4u64;
            if if if if if if if nat_eq(a2606, a2607) {
                let a2609 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2608, _, _, _, _, _) => {
                        h2608
                    }
                };
                let a2610 = 4u64;
                nat_eq(a2609, a2610)
            } else {
                false
            } {
                f167(v2.clone())
            } else {
                false
            } {
                f167(v3.clone())
            } else {
                false
            } {
                f167(v5.clone())
            } else {
                false
            } {
                let a2612 = f176(v5.clone())?;
                let a2613 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2611, _) => {
                        h2611
                    }
                };
                nat_le(a2612, a2613)
            } else {
                false
            } {
                if f168(v2.clone(), v5.clone())? {
                    true
                } else {
                    f169(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f168(v3.clone(), v5.clone())? {
                    true
                } else {
                    f169(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f165(match v2.clone() {
            Adt14::C0(_, h2614, _, _, _, _, _, _, _, _, _) => {
                h2614
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2615, _, _, _, _, _, _, _, _) => {
                h2615
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2616, _, _, _, _, _, _, _) => {
                h2616
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2617, _, _, _, _, _, _) => {
                h2617
            }
        }) {
            0u64
        } else {
            let a2623 = match v2.clone() {
                Adt14::C0(_, _, h2618, _, _, _, _, _, _, _, _) => {
                    h2618
                }
            };
            let a2624 = {
                let a2621 = match v2.clone() {
                    Adt14::C0(_, _, _, h2619, _, _, _, _, _, _, _) => {
                        h2619
                    }
                };
                let a2622 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2620, _, _, _, _, _, _) => {
                        h2620
                    }
                };
                nat_mul(a2621, a2622)?
            };
            nat_mul(a2623, a2624)?
        };
        let v7: u64 = {
            let a2625 = v6;
            let a2626 = 0u64;
            if if nat_eq(a2625, a2626) {
                true
            } else {
                f170(v5.clone())?
            } {
                0u64
            } else {
                let a2628 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2627, _) => {
                        h2627
                    }
                };
                let a2629 = 4u64;
                let a2630 = 0u64;
                nat_quot(a2628, a2629, a2630)
            }
        };
        Ok(Adt15::C1(f178(match v2 {
            Adt14::C0(_, h2631, _, _, _, _, _, _, _, _, _) => {
                h2631
            }
        }, v6, v7)?, 4u64, 66u64, v6, f173(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f92(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f179(v0, v3, v6) {
        f179(v1, v4, v6)
    } else {
        false
    } {
        f179(v2, v5, v6)
    } else {
        false
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f180(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f180(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f180(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f180(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f180(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f180(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2632 = v2;
    let a2633 = 0u64;
    if nat_eq(a2632, a2633) {
        Ok(0u64)
    } else {
        let a2636 = 2u64;
        let a2637 = {
            let a2634 = v0;
            let a2635 = v1;
            nat_mul(a2634, a2635)?
        };
        nat_mul(a2636, a2637)
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2638 = v2;
    let a2639 = f160(v0, v1)?;
    nat_mul(a2638, a2639)
}

pub fn f96(v0: u64) -> u64 {
    let a2640 = v0;
    let a2641 = 30u64;
    if nat_eq(a2640, a2641) {
        3u64
    } else {
        1u64
    }
}

pub fn f97(v0: u64) -> R<u64> {
    let a2644 = 5u64;
    let a2645 = {
        let a2642 = 4u64;
        let a2643 = v0;
        nat_mul(a2642, a2643)?
    };
    nat_add(a2644, a2645)
}

pub fn f98(v0: u64) -> bool {
    let a2646 = v0;
    let a2647 = 0u64;
    if if nat_eq(a2646, a2647) {
        true
    } else {
        let a2648 = v0;
        let a2649 = 1u64;
        nat_eq(a2648, a2649)
    } {
        true
    } else {
        let a2650 = v0;
        let a2651 = 30u64;
        nat_eq(a2650, a2651)
    }
}

pub fn f99(v0: Adt2, v1: u64) -> R<bool> {
    let a2653 = match v0.clone() {
        Adt2::C0(_, _, _, h2652, _, _, _, _, _, _, _) => {
            h2652
        }
    };
    let a2654 = 1u64;
    if if if nat_eq(a2653, a2654) {
        let a2656 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2655, _, _, _, _, _, _) => {
                h2655
            }
        };
        let a2657 = 1u64;
        nat_eq(a2656, a2657)
    } else {
        false
    } {
        let a2659 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2658, _, _, _, _, _) => {
                h2658
            }
        };
        let a2660 = v1;
        nat_eq(a2659, a2660)
    } else {
        false
    } {
        let a2662 = match v0.clone() {
            Adt2::C0(_, _, h2661, _, _, _, _, _, _, _, _) => {
                h2661
            }
        };
        let a2663 = 1u64;
        if nat_eq(a2662, a2663) {
            Ok(true)
        } else {
            let a2668 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2664, _, _, _, _) => {
                    h2664
                }
            };
            let a2669 = {
                let a2666 = match v0 {
                    Adt2::C0(_, h2665, _, _, _, _, _, _, _, _, _) => {
                        h2665
                    }
                };
                let a2667 = v1;
                nat_mul(a2666, a2667)?
            };
            Ok(nat_eq(a2668, a2669))
        }
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Adt6) -> R<u64> {
    let a2682 = 2u64;
    let a2683 = {
        let a2680 = 2u64;
        let a2681 = {
            let a2677 = {
                let a2674 = match v0.clone() {
                    Adt6::C0(h2670, _, _, _) => {
                        h2670
                    }
                };
                let a2675 = {
                    let a2672 = match v0.clone() {
                        Adt6::C0(_, h2671, _, _) => {
                            h2671
                        }
                    };
                    let a2673 = 1u64;
                    nat_sub(a2672, a2673)
                };
                nat_add(a2674, a2675)?
            };
            let a2678 = match v0 {
                Adt6::C0(_, h2676, _, _) => {
                    h2676
                }
            };
            let a2679 = 0u64;
            nat_quot(a2677, a2678, a2679)
        };
        nat_mul(a2680, a2681)?
    };
    nat_add(a2682, a2683)
}

pub fn f101(v0: u64) -> R<u64> {
    let a2686 = 2u64;
    let a2687 = {
        let a2684 = v0;
        let a2685 = 2u64;
        nat_mul(a2684, a2685)?
    };
    nat_add(a2686, a2687)
}

pub fn f102(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2691 = {
            let a2688 = v2;
            let a2689 = 1u64;
            nat_sub(a2688, a2689)
        };
        let a2692 = match v0.clone() {
            Adt6::C0(_, h2690, _, _) => {
                h2690
            }
        };
        nat_mul(a2691, a2692)?
    };
    let a2694 = 0u64;
    let a2695 = match v0.clone() {
        Adt6::C0(h2693, _, _, _) => {
            h2693
        }
    };
    if if if if if if if if nat_lt(a2694, a2695) {
        let a2697 = match v0.clone() {
            Adt6::C0(h2696, _, _, _) => {
                h2696
            }
        };
        let a2698 = 2147483648u64;
        nat_lt(a2697, a2698)
    } else {
        false
    } {
        let a2700 = 0u64;
        let a2701 = match v0.clone() {
            Adt6::C0(_, h2699, _, _) => {
                h2699
            }
        };
        nat_lt(a2700, a2701)
    } else {
        false
    } {
        let a2703 = match v0.clone() {
            Adt6::C0(_, h2702, _, _) => {
                h2702
            }
        };
        let a2704 = 2147483648u64;
        nat_lt(a2703, a2704)
    } else {
        false
    } {
        let a2705 = v1;
        let a2706 = 2147483648u64;
        nat_lt(a2705, a2706)
    } else {
        false
    } {
        let a2707 = 0u64;
        let a2708 = v2;
        nat_lt(a2707, a2708)
    } else {
        false
    } {
        let a2709 = v3;
        let a2710 = 2147483648u64;
        nat_lt(a2709, a2710)
    } else {
        false
    } {
        let a2722 = {
            let a2717 = {
                let a2714 = v3;
                let a2715 = {
                    let a2712 = match v0.clone() {
                        Adt6::C0(h2711, _, _, _) => {
                            h2711
                        }
                    };
                    let a2713 = 1u64;
                    nat_sub(a2712, a2713)
                };
                nat_add(a2714, a2715)?
            };
            let a2718 = f124(match v0.clone() {
                Adt6::C0(_, _, h2716, _) => {
                    h2716
                }
            });
            nat_add(a2717, a2718)?
        };
        let a2723 = {
            let a2720 = 2147483647u64;
            let a2721 = f125(match v0.clone() {
                Adt6::C0(_, _, h2719, _) => {
                    h2719
                }
            });
            nat_add(a2720, a2721)?
        };
        nat_le(a2722, a2723)
    } else {
        false
    } {
        let a2725 = match v0 {
            Adt6::C0(_, _, h2724, _) => {
                h2724
            }
        };
        let a2726 = 2147483648u64;
        Ok(!nat_eq(a2725, a2726))
    } else {
        Ok(false)
    }
}

pub fn f103(v0: Adt7) -> R<u64> {
    let a2738 = if match v0.clone() {
        Adt7::C0(_, _, _, h2727, _) => {
            h2727
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2739 = {
        let a2736 = {
            let a2734 = {
                let m2730 = match v0.clone() {
                    Adt7::C0(h2729, _, _, _, _) => {
                        h2729
                    }
                };
                match m2730 {
                    Adt6::C0(h2728, _, _, _) => {
                        h2728
                    }
                }
            };
            let a2735 = {
                let m2733 = match v0.clone() {
                    Adt7::C0(_, h2732, _, _, _) => {
                        h2732
                    }
                };
                match m2733 {
                    Adt6::C0(h2731, _, _, _) => {
                        h2731
                    }
                }
            };
            nat_mul(a2734, a2735)?
        };
        let a2737 = f155(v0)?;
        nat_mul(a2736, a2737)?
    };
    nat_add(a2738, a2739)
}

pub fn f104(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f181(match v0.clone() {
        Adt2::C0(h2740, _, _, _, _, _, _, _, _, _, _) => {
            h2740
        }
    }, match v0.clone() {
        Adt2::C0(_, h2741, _, _, _, _, _, _, _, _, _) => {
            h2741
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2742, _, _, _, _, _, _, _, _) => {
            h2742
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2743, _, _, _, _, _, _, _) => {
            h2743
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2744, _, _, _, _, _, _) => {
            h2744
        }
    }, match v1.clone() {
        Adt2::C0(h2745, _, _, _, _, _, _, _, _, _, _) => {
            h2745
        }
    }, match v1.clone() {
        Adt2::C0(_, h2746, _, _, _, _, _, _, _, _, _) => {
            h2746
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2747, _, _, _, _, _, _, _, _) => {
            h2747
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2748, _, _, _, _, _, _, _) => {
            h2748
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2749, _, _, _, _, _, _) => {
            h2749
        }
    }, match v2.clone() {
        Adt2::C0(h2750, _, _, _, _, _, _, _, _, _, _) => {
            h2750
        }
    }, match v2.clone() {
        Adt2::C0(_, h2751, _, _, _, _, _, _, _, _, _) => {
            h2751
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2752, _, _, _, _, _, _, _, _) => {
            h2752
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2753, _, _, _, _, _, _, _) => {
            h2753
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2754, _, _, _, _, _, _) => {
            h2754
        }
    }) {
        let m2756 = f108(match v0.clone() {
            Adt2::C0(h2755, _, _, _, _, _, _, _, _, _, _) => {
                h2755
            }
        });
        match m2756 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a2762 = {
                    let a2759 = match v0.clone() {
                        Adt2::C0(_, h2757, _, _, _, _, _, _, _, _, _) => {
                            h2757
                        }
                    };
                    let a2760 = match v3.clone() {
                        Adt18::C0(_, h2758, _) => {
                            h2758
                        }
                    };
                    let a2761 = 0u64;
                    nat_rem(a2759, a2760, a2761)
                };
                let a2763 = 0u64;
                if if nat_eq(a2762, a2763) {
                    if f182(match v0.clone() {
                        Adt2::C0(_, h2764, _, _, _, _, _, _, _, _, _) => {
                            h2764
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2765, _, _, _, _, _, _, _, _, _) => {
                            h2765
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2766, _, _, _, _, _, _, _, _) => {
                            h2766
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2767, _, _, _, _, _, _, _) => {
                            h2767
                        }
                    }) {
                        true
                    } else {
                        let a2769 = 4u64;
                        let a2770 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2768, _, _, _, _, _) => {
                                h2768
                            }
                        };
                        if if nat_le(a2769, a2770) {
                            let a2772 = match v3.clone() {
                                Adt18::C0(_, h2771, _) => {
                                    h2771
                                }
                            };
                            let a2773 = 1u64;
                            if nat_eq(a2772, a2773) {
                                true
                            } else {
                                let a2776 = match v3.clone() {
                                    Adt18::C0(_, _, h2774) => {
                                        h2774
                                    }
                                };
                                let a2777 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2775, _, _, _, _, _) => {
                                        h2775
                                    }
                                };
                                nat_le(a2776, a2777)
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
                        Adt18::C0(_, h2778, _) => {
                            h2778
                        }
                    });
                    Ok(Adt10::C4(if f182(match v0.clone() {
                        Adt2::C0(_, h2779, _, _, _, _, _, _, _, _, _) => {
                            h2779
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2780, _, _, _, _, _, _, _, _, _) => {
                            h2780
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2781, _, _, _, _, _, _, _, _) => {
                            h2781
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2782, _, _, _, _, _, _, _) => {
                            h2782
                        }
                    }) {
                        0u64
                    } else {
                        f149(v4.clone(), f18(v1.clone(), 1u64))?
                    }, 4u64, v4, f18(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2783, _, _, _, _, _, _, _, _, _, _) => {
                            h2783
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

pub fn f105(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f183(match v0.clone() {
        Adt2::C0(h2784, _, _, _, _, _, _, _, _, _, _) => {
            h2784
        }
    }) {
        let a2790 = {
            let a2787 = match v0.clone() {
                Adt2::C0(_, h2785, _, _, _, _, _, _, _, _, _) => {
                    h2785
                }
            };
            let a2788 = f184(match v0.clone() {
                Adt2::C0(h2786, _, _, _, _, _, _, _, _, _, _) => {
                    h2786
                }
            });
            let a2789 = 1u64;
            nat_rem(a2787, a2788, a2789)
        };
        let a2791 = 0u64;
        nat_eq(a2790, a2791)
    } else {
        false
    } {
        let a2793 = match v1.clone() {
            Adt2::C0(h2792, _, _, _, _, _, _, _, _, _, _) => {
                h2792
            }
        };
        let a2794 = 26u64;
        nat_eq(a2793, a2794)
    } else {
        false
    } {
        let a2796 = match v2.clone() {
            Adt2::C0(h2795, _, _, _, _, _, _, _, _, _, _) => {
                h2795
            }
        };
        let a2797 = 0u64;
        nat_eq(a2796, a2797)
    } else {
        false
    } {
        let a2800 = match v0.clone() {
            Adt2::C0(_, h2798, _, _, _, _, _, _, _, _, _) => {
                h2798
            }
        };
        let a2801 = match v2.clone() {
            Adt2::C0(_, h2799, _, _, _, _, _, _, _, _, _) => {
                h2799
            }
        };
        nat_eq(a2800, a2801)
    } else {
        false
    } {
        let a2804 = match v1.clone() {
            Adt2::C0(_, h2802, _, _, _, _, _, _, _, _, _) => {
                h2802
            }
        };
        let a2805 = match v0.clone() {
            Adt2::C0(_, _, h2803, _, _, _, _, _, _, _, _) => {
                h2803
            }
        };
        nat_le(a2804, a2805)
    } else {
        false
    } {
        let a2807 = match v0.clone() {
            Adt2::C0(_, _, _, h2806, _, _, _, _, _, _, _) => {
                h2806
            }
        };
        let a2808 = 1u64;
        nat_eq(a2807, a2808)
    } else {
        false
    } {
        let a2810 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2809, _, _, _, _, _, _) => {
                h2809
            }
        };
        let a2811 = 1u64;
        nat_eq(a2810, a2811)
    } else {
        false
    } {
        let a2813 = match v2.clone() {
            Adt2::C0(_, _, _, h2812, _, _, _, _, _, _, _) => {
                h2812
            }
        };
        let a2814 = 1u64;
        nat_eq(a2813, a2814)
    } else {
        false
    } {
        let a2816 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2815, _, _, _, _, _, _) => {
                h2815
            }
        };
        let a2817 = 1u64;
        nat_eq(a2816, a2817)
    } else {
        false
    } {
        let a2819 = match v1.clone() {
            Adt2::C0(_, _, h2818, _, _, _, _, _, _, _, _) => {
                h2818
            }
        };
        let a2820 = 1u64;
        nat_eq(a2819, a2820)
    } else {
        false
    } {
        let a2822 = match v1.clone() {
            Adt2::C0(_, _, _, h2821, _, _, _, _, _, _, _) => {
                h2821
            }
        };
        let a2823 = 1u64;
        nat_eq(a2822, a2823)
    } else {
        false
    } {
        let a2825 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2824, _, _, _, _, _, _) => {
                h2824
            }
        };
        let a2826 = 1u64;
        nat_eq(a2825, a2826)
    } else {
        false
    } {
        if f31(match v2.clone() {
            Adt2::C0(_, h2827, _, _, _, _, _, _, _, _, _) => {
                h2827
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2828, _, _, _, _, _, _, _, _) => {
                h2828
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2829, _, _, _, _, _, _, _) => {
                h2829
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2830, _, _, _, _, _, _) => {
                h2830
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2832 = match v1.clone() {
                Adt2::C0(_, h2831, _, _, _, _, _, _, _, _, _) => {
                    h2831
                }
            };
            let a2833 = 0u64;
            if nat_eq(a2832, a2833) {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2835 = match v0.clone() {
                    Adt2::C0(_, h2834, _, _, _, _, _, _, _, _, _) => {
                        h2834
                    }
                };
                let a2836 = 2147483647u64;
                if if if if if if nat_le(a2835, a2836) {
                    let a2838 = match v1.clone() {
                        Adt2::C0(_, h2837, _, _, _, _, _, _, _, _, _) => {
                            h2837
                        }
                    };
                    let a2839 = 2147483647u64;
                    nat_le(a2838, a2839)
                } else {
                    false
                } {
                    let a2841 = 4u64;
                    let a2842 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2840, _, _, _, _, _) => {
                            h2840
                        }
                    };
                    nat_le(a2841, a2842)
                } else {
                    false
                } {
                    let a2848 = {
                        let a2845 = match v2.clone() {
                            Adt2::C0(_, h2843, _, _, _, _, _, _, _, _, _) => {
                                h2843
                            }
                        };
                        let a2846 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2844, _, _, _, _, _) => {
                                h2844
                            }
                        };
                        nat_mul(a2845, a2846)?
                    };
                    let a2849 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2847, _, _, _, _) => {
                            h2847
                        }
                    };
                    nat_le(a2848, a2849)
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
                        let a2862 = {
                            let a2853 = {
                                let a2851 = 2u64;
                                let a2852 = match v1.clone() {
                                    Adt2::C0(_, h2850, _, _, _, _, _, _, _, _, _) => {
                                        h2850
                                    }
                                };
                                nat_mul(a2851, a2852)?
                            };
                            let a2854 = f14(v2.clone(), 1u64)?;
                            nat_add(a2853, a2854)?
                        };
                        let a2863 = {
                            let a2860 = match v1.clone() {
                                Adt2::C0(_, h2855, _, _, _, _, _, _, _, _, _) => {
                                    h2855
                                }
                            };
                            let a2861 = {
                                let a2858 = 1u64;
                                let a2859 = f147(match v0.clone() {
                                    Adt2::C0(h2856, _, _, _, _, _, _, _, _, _, _) => {
                                        h2856
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h2857, _, _, _, _, _, _, _, _, _) => {
                                        h2857
                                    }
                                })?;
                                nat_add(a2858, a2859)?
                            };
                            nat_mul(a2860, a2861)?
                        };
                        nat_add(a2862, a2863)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2864, _, _, _, _, _, _, _, _, _) => {
                            h2864
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2865, _, _, _, _, _, _, _, _, _) => {
                            h2865
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2866, _, _, _, _, _) => {
                            h2866
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2867, _, _, _, _) => {
                            h2867
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2868, _, _, _) => {
                            h2868
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2869, _, _) => {
                            h2869
                        }
                    }), f18(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2870, _, _, _, _, _, _, _, _, _, _) => {
                            h2870
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

pub fn f106(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2873 = match v1.clone() {
        Adt2::C0(h2871, _, _, _, _, _, _, _, _, _, _) => {
            h2871
        }
    };
    let a2874 = match v3.clone() {
        Adt2::C0(h2872, _, _, _, _, _, _, _, _, _, _) => {
            h2872
        }
    };
    if if nat_eq(a2873, a2874) {
        let a2875 = v0;
        let a2876 = 33u64;
        if nat_eq(a2875, a2876) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2878 = f108(match v1.clone() {
            Adt2::C0(h2877, _, _, _, _, _, _, _, _, _, _) => {
                h2877
            }
        });
        match m2878 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h2879, _) => {
                        h2879
                    }
                };
                let v6: u64 = f14(v1.clone(), v5)?;
                let a2884 = {
                    let a2881 = match v1.clone() {
                        Adt2::C0(_, h2880, _, _, _, _, _, _, _, _, _) => {
                            h2880
                        }
                    };
                    let a2882 = v5;
                    let a2883 = 1u64;
                    nat_rem(a2881, a2882, a2883)
                };
                let a2885 = 0u64;
                if if if nat_eq(a2884, a2885) {
                    let a2890 = {
                        let a2887 = match v3.clone() {
                            Adt2::C0(_, h2886, _, _, _, _, _, _, _, _, _) => {
                                h2886
                            }
                        };
                        let a2888 = v5;
                        let a2889 = 1u64;
                        nat_rem(a2887, a2888, a2889)
                    };
                    let a2891 = 0u64;
                    nat_eq(a2890, a2891)
                } else {
                    false
                } {
                    let a2892 = v6;
                    let a2893 = f14(v3.clone(), v5)?;
                    nat_eq(a2892, a2893)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h2894) => {
                            h2894
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

pub fn f107(v0: u64) -> bool {
    if f98(v0) {
        true
    } else {
        let a2895 = v0;
        let a2896 = 43u64;
        nat_eq(a2895, a2896)
    }
}

pub fn f108(v0: u64) -> Adt17 {
    let a2897 = v0;
    let a2898 = 0u64;
    if nat_eq(a2897, a2898) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a2899 = v0;
        let a2900 = 1u64;
        if nat_eq(a2899, a2900) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a2901 = v0;
            let a2902 = 2u64;
            if nat_eq(a2901, a2902) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a2903 = v0;
                let a2904 = 3u64;
                if nat_eq(a2903, a2904) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a2905 = v0;
                    let a2906 = 6u64;
                    if nat_eq(a2905, a2906) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2907 = v0;
                        let a2908 = 7u64;
                        if nat_eq(a2907, a2908) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2909 = v0;
                            let a2910 = 8u64;
                            if nat_eq(a2909, a2910) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2911 = v0;
                                let a2912 = 9u64;
                                if nat_eq(a2911, a2912) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2913 = v0;
                                    let a2914 = 10u64;
                                    if nat_eq(a2913, a2914) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2915 = v0;
                                        let a2916 = 11u64;
                                        if nat_eq(a2915, a2916) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2917 = v0;
                                            let a2918 = 12u64;
                                            if nat_eq(a2917, a2918) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2919 = v0;
                                                let a2920 = 13u64;
                                                if nat_eq(a2919, a2920) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2921 = v0;
                                                    let a2922 = 14u64;
                                                    if nat_eq(a2921, a2922) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2923 = v0;
                                                        let a2924 = 15u64;
                                                        if nat_eq(a2923, a2924) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2925 = v0;
                                                            let a2926 = 16u64;
                                                            if nat_eq(a2925, a2926) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2927 = v0;
                                                                let a2928 = 17u64;
                                                                if nat_eq(a2927, a2928) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2929 = v0;
                                                                    let a2930 = 18u64;
                                                                    if nat_eq(a2929, a2930) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2931 = v0;
                                                                        let a2932 = 19u64;
                                                                        if nat_eq(a2931, a2932) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2933 = v0;
                                                                            let a2934 = 20u64;
                                                                            if nat_eq(a2933, a2934) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2935 = v0;
                                                                                let a2936 = 21u64;
                                                                                if nat_eq(a2935, a2936) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2937 = v0;
                                                                                    let a2938 = 22u64;
                                                                                    if nat_eq(a2937, a2938) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2939 = v0;
                                                                                        let a2940 = 23u64;
                                                                                        if nat_eq(a2939, a2940) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2941 = v0;
                                                                                            let a2942 = 24u64;
                                                                                            if nat_eq(a2941, a2942) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2943 = v0;
                                                                                                let a2944 = 25u64;
                                                                                                if nat_eq(a2943, a2944) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2945 = v0;
                                                                                                    let a2946 = 26u64;
                                                                                                    if nat_eq(a2945, a2946) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2947 = v0;
                                                                                                        let a2948 = 27u64;
                                                                                                        if nat_eq(a2947, a2948) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2949 = v0;
                                                                                                            let a2950 = 28u64;
                                                                                                            if nat_eq(a2949, a2950) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2951 = v0;
                                                                                                                let a2952 = 29u64;
                                                                                                                if nat_eq(a2951, a2952) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2953 = v0;
                                                                                                                    let a2954 = 30u64;
                                                                                                                    if nat_eq(a2953, a2954) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2955 = v0;
                                                                                                                        let a2956 = 34u64;
                                                                                                                        if nat_eq(a2955, a2956) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2957 = v0;
                                                                                                                            let a2958 = 35u64;
                                                                                                                            if nat_eq(a2957, a2958) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2959 = v0;
                                                                                                                                let a2960 = 39u64;
                                                                                                                                if nat_eq(a2959, a2960) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2961 = v0;
                                                                                                                                    let a2962 = 40u64;
                                                                                                                                    if nat_eq(a2961, a2962) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2963 = v0;
                                                                                                                                        let a2964 = 41u64;
                                                                                                                                        if nat_eq(a2963, a2964) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2965 = v0;
                                                                                                                                            let a2966 = 42u64;
                                                                                                                                            if nat_eq(a2965, a2966) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2967 = v0;
                                                                                                                                                let a2968 = 43u64;
                                                                                                                                                if nat_eq(a2967, a2968) {
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

pub fn f109(v0: u64, v1: u64) -> u64 {
    let a2969 = v0;
    let a2970 = 0u64;
    if if nat_eq(a2969, a2970) {
        true
    } else {
        let a2971 = v1;
        let a2972 = 0u64;
        nat_eq(a2971, a2972)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f110(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f31(v4, v5, v6, v7) {
        f31(v0, v1, v2, v3)
    } else {
        let a2976 = {
            let a2973 = v0;
            let a2974 = v4;
            let a2975 = 1u64;
            nat_rem(a2973, a2974, a2975)
        };
        let a2977 = 0u64;
        if if if nat_eq(a2976, a2977) {
            let a2981 = {
                let a2978 = v1;
                let a2979 = v5;
                let a2980 = 1u64;
                nat_rem(a2978, a2979, a2980)
            };
            let a2982 = 0u64;
            nat_eq(a2981, a2982)
        } else {
            false
        } {
            let a2986 = {
                let a2983 = v2;
                let a2984 = v6;
                let a2985 = 1u64;
                nat_rem(a2983, a2984, a2985)
            };
            let a2987 = 0u64;
            nat_eq(a2986, a2987)
        } else {
            false
        } {
            let a2991 = {
                let a2988 = v3;
                let a2989 = v7;
                let a2990 = 1u64;
                nat_rem(a2988, a2989, a2990)
            };
            let a2992 = 0u64;
            nat_eq(a2991, a2992)
        } else {
            false
        }
    }
}

pub fn f111(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2993 = v0;
    let a2994 = v1;
    if nat_eq(a2993, a2994) {
        let a2997 = v4;
        let a2998 = {
            let a2995 = v2;
            let a2996 = v3;
            nat_add(a2995, a2996)?
        };
        Ok(nat_eq(a2997, a2998))
    } else {
        let a2999 = v2;
        let a3000 = v3;
        if nat_eq(a2999, a3000) {
            let a3001 = v4;
            let a3002 = v2;
            Ok(nat_eq(a3001, a3002))
        } else {
            Ok(false)
        }
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3005 = {
            let a3003 = v0;
            let a3004 = f125(v1);
            nat_add(a3003, a3004)?
        };
        let a3006 = f125(v2);
        nat_add(a3005, a3006)?
    };
    let v5: u64 = {
        let a3007 = f124(v1);
        let a3008 = f124(v2);
        nat_add(a3007, a3008)?
    };
    let a3009 = v1;
    let a3010 = 4294967295u64;
    if if nat_le(a3009, a3010) {
        let a3011 = v2;
        let a3012 = 4294967295u64;
        nat_le(a3011, a3012)
    } else {
        false
    } {
        let a3013 = v5;
        let a3014 = v4;
        if nat_le(a3013, a3014) {
            let a3017 = v3;
            let a3018 = {
                let a3015 = v4;
                let a3016 = v5;
                nat_sub(a3015, a3016)
            };
            Ok(nat_eq(a3017, a3018))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f113(v0: u64, v1: u64) -> R<bool> {
    let a3019 = v1;
    let a3020 = 4294967295u64;
    if nat_le(a3019, a3020) {
        let a3023 = {
            let a3021 = f125(v1);
            let a3022 = f124(v1);
            nat_add(a3021, a3022)?
        };
        let a3024 = v0;
        Ok(nat_lt(a3023, a3024))
    } else {
        Ok(false)
    }
}

pub fn f114(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f185(v0.clone(), false)?;
    let v6: u64 = f185(v0, true)?;
    let a3025 = v6;
    let a3026 = v1;
    if if nat_le(a3025, a3026) {
        let a3033 = {
            let a3031 = {
                let a3027 = v1;
                let a3028 = v5;
                nat_add(a3027, a3028)?
            };
            let a3032 = {
                let a3029 = v3;
                let a3030 = 0u64;
                if nat_eq(a3029, a3030) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3031, a3032)?
        };
        let a3034 = v2;
        nat_le(a3033, a3034)
    } else {
        false
    } {
        if v4 {
            let a3039 = {
                let a3037 = {
                    let a3035 = v1;
                    let a3036 = v5;
                    nat_add(a3035, a3036)?
                };
                let a3038 = v6;
                nat_sub(a3037, a3038)
            };
            let a3040 = v2;
            Ok(nat_lt(a3039, a3040))
        } else {
            let a3045 = {
                let a3043 = {
                    let a3041 = v1;
                    let a3042 = v5;
                    nat_add(a3041, a3042)?
                };
                let a3044 = v6;
                nat_sub(a3043, a3044)
            };
            let a3046 = v2;
            Ok(nat_le(a3045, a3046))
        }
    } else {
        Ok(false)
    }
}

pub fn f115(v0: u64, v1: u64) -> bool {
    let a3047 = v0;
    let a3048 = 0u64;
    if if nat_eq(a3047, a3048) {
        true
    } else {
        let a3049 = v0;
        let a3050 = 1u64;
        nat_eq(a3049, a3050)
    } {
        let a3051 = v1;
        let a3052 = 4294967296u64;
        nat_lt(a3051, a3052)
    } else {
        false
    }
}

pub fn f116(v0: u64, v1: u64) -> R<u64> {
    let a3057 = {
        let a3055 = v0;
        let a3056 = {
            let a3053 = v1;
            let a3054 = 1u64;
            nat_sub(a3053, a3054)
        };
        nat_add(a3055, a3056)?
    };
    let a3058 = v1;
    let a3059 = 0u64;
    Ok(nat_quot(a3057, a3058, a3059))
}

pub fn f117(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f31(match v0.clone() {
        Adt2::C0(_, h3060, _, _, _, _, _, _, _, _, _) => {
            h3060
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3061, _, _, _, _, _, _, _, _) => {
            h3061
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3062, _, _, _, _, _, _, _) => {
            h3062
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3063, _, _, _, _, _, _) => {
            h3063
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3066 = match v1.clone() {
                Adt2::C0(_, h3064, _, _, _, _, _, _, _, _, _) => {
                    h3064
                }
            };
            let a3067 = match v1.clone() {
                Adt2::C0(_, _, h3065, _, _, _, _, _, _, _, _) => {
                    h3065
                }
            };
            nat_mul(a3066, a3067)?
        };
        let v5: u64 = {
            let a3069 = v4;
            let a3070 = match v1.clone() {
                Adt2::C0(_, _, _, h3068, _, _, _, _, _, _, _) => {
                    h3068
                }
            };
            nat_mul(a3069, a3070)?
        };
        let a3083 = {
            let a3081 = {
                let a3072 = match v0.clone() {
                    Adt2::C0(_, h3071, _, _, _, _, _, _, _, _, _) => {
                        h3071
                    }
                };
                let a3073 = 1u64;
                nat_sub(a3072, a3073)
            };
            let a3082 = {
                let a3079 = f186(match v0.clone() {
                    Adt2::C0(_, _, h3074, _, _, _, _, _, _, _, _) => {
                        h3074
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3075, _, _, _, _, _, _, _, _, _) => {
                        h3075
                    }
                })?;
                let a3080 = f186(match v0 {
                    Adt2::C0(_, _, _, h3076, _, _, _, _, _, _, _) => {
                        h3076
                    }
                }, v2, {
                    let a3077 = v3;
                    let a3078 = v5;
                    nat_mul(a3077, a3078)?
                }, v4)?;
                nat_add(a3079, a3080)?
            };
            nat_add(a3081, a3082)?
        };
        let a3084 = f14(v1, 1u64)?;
        Ok(nat_lt(a3083, a3084))
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3085 = v0;
    let a3086 = 2147483648u64;
    if if if if nat_lt(a3085, a3086) {
        let a3087 = v1;
        let a3088 = 2147483648u64;
        nat_lt(a3087, a3088)
    } else {
        false
    } {
        let a3091 = {
            let a3089 = v0;
            let a3090 = v1;
            nat_mul(a3089, a3090)?
        };
        let a3092 = 2147483648u64;
        nat_lt(a3091, a3092)
    } else {
        false
    } {
        let a3097 = {
            let a3095 = {
                let a3093 = v0;
                let a3094 = 1u64;
                nat_sub(a3093, a3094)
            };
            let a3096 = v2;
            nat_mul(a3095, a3096)?
        };
        let a3098 = 2147483648u64;
        nat_lt(a3097, a3098)
    } else {
        false
    } {
        let a3103 = {
            let a3101 = {
                let a3099 = v1;
                let a3100 = 1u64;
                nat_sub(a3099, a3100)
            };
            let a3102 = v2;
            nat_mul(a3101, a3102)?
        };
        let a3104 = 2147483648u64;
        Ok(nat_lt(a3103, a3104))
    } else {
        Ok(false)
    }
}

pub fn f119(v0: u64) -> R<u64> {
    let a3107 = 3u64;
    let a3108 = {
        let a3105 = v0;
        let a3106 = 3u64;
        nat_mul(a3105, a3106)?
    };
    nat_add(a3107, a3108)
}

pub fn f120(v0: Adt3) -> R<u64> {
    if f31(match v0.clone() {
        Adt3::C0(h3109, _, _, _, _, _, _, _) => {
            h3109
        }
    }, match v0.clone() {
        Adt3::C0(_, h3110, _, _, _, _, _, _) => {
            h3110
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3111, _, _, _, _, _) => {
            h3111
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3112, _, _, _, _) => {
            h3112
        }
    }) {
        Ok(0u64)
    } else {
        let a3121 = match v0.clone() {
            Adt3::C0(h3113, _, _, _, _, _, _, _) => {
                h3113
            }
        };
        let a3122 = {
            let a3119 = match v0.clone() {
                Adt3::C0(_, h3114, _, _, _, _, _, _) => {
                    h3114
                }
            };
            let a3120 = {
                let a3117 = match v0.clone() {
                    Adt3::C0(_, _, h3115, _, _, _, _, _) => {
                        h3115
                    }
                };
                let a3118 = match v0 {
                    Adt3::C0(_, _, _, h3116, _, _, _, _) => {
                        h3116
                    }
                };
                nat_mul(a3117, a3118)?
            };
            nat_mul(a3119, a3120)?
        };
        nat_mul(a3121, a3122)
    }
}

pub fn f121(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3123 = v1;
    let a3124 = 13u64;
    if if nat_eq(a3123, a3124) {
        f31(match v0.clone() {
            Adt3::C0(h3125, _, _, _, _, _, _, _) => {
                h3125
            }
        }, match v0.clone() {
            Adt3::C0(_, h3126, _, _, _, _, _, _) => {
                h3126
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3127, _, _, _, _, _) => {
                h3127
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3128, _, _, _, _) => {
                h3128
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3134 = match v0.clone() {
                Adt3::C0(_, h3129, _, _, _, _, _, _) => {
                    h3129
                }
            };
            let a3135 = {
                let a3132 = match v0.clone() {
                    Adt3::C0(_, _, h3130, _, _, _, _, _) => {
                        h3130
                    }
                };
                let a3133 = match v0.clone() {
                    Adt3::C0(_, _, _, h3131, _, _, _, _) => {
                        h3131
                    }
                };
                nat_mul(a3132, a3133)?
            };
            nat_mul(a3134, a3135)?
        };
        let a3136 = v1;
        let a3137 = 13u64;
        if nat_eq(a3136, a3137) {
            let a3146 = 2u64;
            let a3147 = if f187(v0.clone(), v2) {
                let a3139 = v3;
                let a3140 = f188(match v0 {
                    Adt3::C0(h3138, _, _, _, _, _, _, _) => {
                        h3138
                    }
                }, false)?;
                nat_mul(a3139, a3140)?
            } else {
                let a3144 = v3;
                let a3145 = {
                    let a3142 = match v0 {
                        Adt3::C0(h3141, _, _, _, _, _, _, _) => {
                            h3141
                        }
                    };
                    let a3143 = 2u64;
                    nat_mul(a3142, a3143)?
                };
                nat_mul(a3144, a3145)?
            };
            nat_add(a3146, a3147)
        } else {
            let a3151 = v3;
            let a3152 = f188(match v0 {
                Adt3::C0(h3148, _, _, _, _, _, _, _) => {
                    h3148
                }
            }, {
                let a3149 = v1;
                let a3150 = 16u64;
                nat_eq(a3149, a3150)
            })?;
            nat_mul(a3151, a3152)
        }
    }
}

pub fn f122(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3153 = v0;
    let a3154 = 30u64;
    if if nat_eq(a3153, a3154) {
        let a3157 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3155, _, _, _, _) => {
                h3155
            }
        };
        let a3158 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3156, _, _, _, _, _) => {
                h3156
            }
        };
        nat_lt(a3157, a3158)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3159 = v3;
        match m3159 {
            None => {
                f189(v0, v1, match v2.clone() {
                    Adt2::C0(h3160, _, _, _, _, _, _, _, _, _, _) => {
                        h3160
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3161, _, _, _, _, _, _, _, _, _, _) => {
                        h3161
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3162, _, _, _, _, _, _, _, _, _) => {
                        h3162
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3163, _, _, _, _, _, _, _, _) => {
                        h3163
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3164, _, _, _, _, _, _, _) => {
                        h3164
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3165, _, _, _, _, _, _) => {
                        h3165
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3166, _, _, _, _, _, _, _, _, _) => {
                        h3166
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3167, _, _, _, _, _, _, _, _) => {
                        h3167
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3168, _, _, _, _, _, _, _) => {
                        h3168
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3169, _, _, _, _, _, _) => {
                        h3169
                    }
                })
            }
            Some(v5) => {
                f189(v0, v1, match v2.clone() {
                    Adt2::C0(h3170, _, _, _, _, _, _, _, _, _, _) => {
                        h3170
                    }
                }, match v5.clone() {
                    Adt2::C0(h3171, _, _, _, _, _, _, _, _, _, _) => {
                        h3171
                    }
                }, match v4.clone() {
                    Adt2::C0(h3172, _, _, _, _, _, _, _, _, _, _) => {
                        h3172
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3173, _, _, _, _, _, _, _, _, _) => {
                        h3173
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3174, _, _, _, _, _, _, _, _) => {
                        h3174
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3175, _, _, _, _, _, _, _) => {
                        h3175
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3176, _, _, _, _, _, _) => {
                        h3176
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3177, _, _, _, _, _, _, _, _, _) => {
                        h3177
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3178, _, _, _, _, _, _, _, _) => {
                        h3178
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3179, _, _, _, _, _, _, _) => {
                        h3179
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3180, _, _, _, _, _, _) => {
                        h3180
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3181, _, _, _, _, _, _, _, _, _) => {
                        h3181
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3182, _, _, _, _, _, _, _, _) => {
                        h3182
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3183, _, _, _, _, _, _, _) => {
                        h3183
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3184, _, _, _, _, _, _) => {
                        h3184
                    }
                })
            }
        }
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3197 = {
        let a3193 = {
            let a3189 = {
                let a3185 = v0;
                let a3186 = v4;
                nat_mul(a3185, a3186)?
            };
            let a3190 = {
                let a3187 = v1;
                let a3188 = v5;
                nat_mul(a3187, a3188)?
            };
            nat_add(a3189, a3190)?
        };
        let a3194 = {
            let a3191 = v2;
            let a3192 = v6;
            nat_mul(a3191, a3192)?
        };
        nat_add(a3193, a3194)?
    };
    let a3198 = {
        let a3195 = v3;
        let a3196 = v7;
        nat_mul(a3195, a3196)?
    };
    nat_add(a3197, a3198)
}

pub fn f124(v0: u64) -> u64 {
    let a3199 = v0;
    let a3200 = 2147483648u64;
    if nat_lt(a3199, a3200) {
        0u64
    } else {
        let a3201 = 4294967296u64;
        let a3202 = v0;
        nat_sub(a3201, a3202)
    }
}

pub fn f125(v0: u64) -> u64 {
    let a3203 = v0;
    let a3204 = 2147483648u64;
    if nat_lt(a3203, a3204) {
        v0
    } else {
        0u64
    }
}

pub fn f126(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3205 = v7;
        let a3206 = 6u64;
        if nat_eq(a3205, a3206) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f84(v3, v4), f84(v5, v6), f85(v7, v8), f190(v1), f190(v9))
}

pub fn f127(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3207 = v0;
    match m3207 {
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
            let a3208 = 13u64;
            let a3209 = v36;
            if nat_le(a3208, a3209) {
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
                Adt5::C0(_, _, _, _, _, h3210, _, _) => {
                    h3210
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3211) => {
                    h3211
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

pub fn f128(v0: Adt4) -> u64 {
    let m3212 = v0;
    match m3212 {
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

pub fn f129(v0: Adt4, v1: u64) -> u64 {
    let m3213 = v0;
    match m3213 {
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
            let a3214 = v1;
            let a3215 = 6u64;
            if nat_eq(a3214, a3215) {
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
            let a3224 = v1;
            let a3225 = 6u64;
            if if if nat_eq(a3224, a3225) {
                true
            } else {
                let a3226 = v1;
                let a3227 = 7u64;
                nat_eq(a3226, a3227)
            } {
                let a3228 = v56;
                let a3229 = 30u64;
                nat_eq(a3228, a3229)
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
                Adt5::C0(_, _, _, _, _, _, _, h3223) => {
                    h3223
                }
            }
        }
        Adt4::C28(v51) => {
            let a3216 = v1;
            let a3217 = 6u64;
            if nat_eq(a3216, a3217) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3218) => {
                        h3218
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
            let a3219 = v1;
            let a3220 = 6u64;
            if if nat_eq(a3219, a3220) {
                true
            } else {
                let a3221 = v1;
                let a3222 = 7u64;
                nat_eq(a3221, a3222)
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

pub fn f130(v0: Adt4, v1: u64) -> Adt11 {
    let a3230 = v1;
    let a3231 = 4u64;
    if nat_lt(a3230, a3231) {
        Adt11::C0(v1)
    } else {
        let a3232 = v1;
        let a3233 = 4u64;
        if nat_eq(a3232, a3233) {
            Adt11::C1(0u64)
        } else {
            let a3234 = v1;
            let a3235 = 5u64;
            if nat_eq(a3234, a3235) {
                Adt11::C1(3u64)
            } else {
                let a3236 = v1;
                let a3237 = 6u64;
                if nat_eq(a3236, a3237) {
                    Adt11::C2(f129(v0, v1), 0u64, false)
                } else {
                    let a3238 = v1;
                    let a3239 = 7u64;
                    if nat_eq(a3238, a3239) {
                        Adt11::C2(0u64, f129(v0, v1), false)
                    } else {
                        let a3240 = v1;
                        let a3241 = 16u64;
                        if nat_lt(a3240, a3241) {
                            Adt11::C1({
                                let a3242 = v1;
                                let a3243 = 8u64;
                                nat_sub(a3242, a3243)
                            })
                        } else {
                            let a3244 = v1;
                            let a3245 = 20u64;
                            if nat_lt(a3244, a3245) {
                                Adt11::C5({
                                    let a3246 = v1;
                                    let a3247 = 16u64;
                                    nat_sub(a3246, a3247)
                                })
                            } else {
                                let a3248 = v1;
                                let a3249 = 20u64;
                                if nat_eq(a3248, a3249) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3250 = v1;
                                    let a3251 = 21u64;
                                    if nat_eq(a3250, a3251) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3252 = v1;
                                        let a3253 = 22u64;
                                        if nat_eq(a3252, a3253) {
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

pub fn f131(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3255 = v0;
        let a3256 = match v1.clone() {
            Adt3::C0(h3254, _, _, _, _, _, _, _) => {
                h3254
            }
        };
        let a3257 = 0u64;
        nat_quot(a3255, a3256, a3257)
    };
    let v3: u64 = {
        let a3259 = v2;
        let a3260 = match v1.clone() {
            Adt3::C0(_, h3258, _, _, _, _, _, _) => {
                h3258
            }
        };
        let a3261 = 0u64;
        nat_quot(a3259, a3260, a3261)
    };
    f123({
        let a3263 = v0;
        let a3264 = match v1.clone() {
            Adt3::C0(h3262, _, _, _, _, _, _, _) => {
                h3262
            }
        };
        let a3265 = 0u64;
        nat_rem(a3263, a3264, a3265)
    }, {
        let a3267 = v2;
        let a3268 = match v1.clone() {
            Adt3::C0(_, h3266, _, _, _, _, _, _) => {
                h3266
            }
        };
        let a3269 = 0u64;
        nat_rem(a3267, a3268, a3269)
    }, {
        let a3271 = v3;
        let a3272 = match v1.clone() {
            Adt3::C0(_, _, h3270, _, _, _, _, _) => {
                h3270
            }
        };
        let a3273 = 0u64;
        nat_rem(a3271, a3272, a3273)
    }, {
        let a3275 = v3;
        let a3276 = match v1.clone() {
            Adt3::C0(_, _, h3274, _, _, _, _, _) => {
                h3274
            }
        };
        let a3277 = 0u64;
        nat_quot(a3275, a3276, a3277)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3278, _, _, _) => {
            h3278
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3279, _, _) => {
            h3279
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3280, _) => {
            h3280
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3281) => {
            h3281
        }
    })
}

pub fn f132(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3299 = {
        let a3294 = f191(v2, {
            let a3285 = {
                let a3282 = v0;
                let a3283 = v1;
                let a3284 = 0u64;
                nat_quot(a3282, a3283, a3284)
            };
            let a3286 = v2;
            let a3287 = 0u64;
            nat_rem(a3285, a3286, a3287)
        }, {
            let a3291 = {
                let a3288 = v0;
                let a3289 = v1;
                let a3290 = 0u64;
                nat_quot(a3288, a3289, a3290)
            };
            let a3292 = v2;
            let a3293 = 0u64;
            nat_quot(a3291, a3292, a3293)
        })?;
        let a3295 = v1;
        nat_mul(a3294, a3295)?
    };
    let a3300 = {
        let a3296 = v0;
        let a3297 = v1;
        let a3298 = 0u64;
        nat_rem(a3296, a3297, a3298)
    };
    nat_add(a3299, a3300)
}

pub fn f133(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3302 = v0;
        let a3303 = match v2.clone() {
            Adt3::C0(h3301, _, _, _, _, _, _, _) => {
                h3301
            }
        };
        let a3304 = 0u64;
        nat_quot(a3302, a3303, a3304)
    };
    let v4: u64 = {
        let a3306 = v3;
        let a3307 = match v2.clone() {
            Adt3::C0(_, h3305, _, _, _, _, _, _) => {
                h3305
            }
        };
        let a3308 = 0u64;
        nat_quot(a3306, a3307, a3308)
    };
    f192({
        let a3310 = v0;
        let a3311 = match v2.clone() {
            Adt3::C0(h3309, _, _, _, _, _, _, _) => {
                h3309
            }
        };
        let a3312 = 0u64;
        nat_rem(a3310, a3311, a3312)
    }, {
        let a3314 = v3;
        let a3315 = match v2.clone() {
            Adt3::C0(_, h3313, _, _, _, _, _, _) => {
                h3313
            }
        };
        let a3316 = 0u64;
        nat_rem(a3314, a3315, a3316)
    }, {
        let a3318 = v4;
        let a3319 = match v2.clone() {
            Adt3::C0(_, _, h3317, _, _, _, _, _) => {
                h3317
            }
        };
        let a3320 = 0u64;
        nat_rem(a3318, a3319, a3320)
    }, {
        let a3322 = v4;
        let a3323 = match v2 {
            Adt3::C0(_, _, h3321, _, _, _, _, _) => {
                h3321
            }
        };
        let a3324 = 0u64;
        nat_quot(a3322, a3323, a3324)
    }, match v1.clone() {
        Adt3::C0(h3325, _, _, _, _, _, _, _) => {
            h3325
        }
    }, match v1.clone() {
        Adt3::C0(_, h3326, _, _, _, _, _, _) => {
            h3326
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3327, _, _, _, _, _) => {
            h3327
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3328, _, _, _, _) => {
            h3328
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3329, _, _, _) => {
            h3329
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3330, _, _) => {
            h3330
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3331, _) => {
            h3331
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3332) => {
            h3332
        }
    })
}

pub fn f134(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3334 = v0;
        let a3335 = match v3.clone() {
            Adt3::C0(h3333, _, _, _, _, _, _, _) => {
                h3333
            }
        };
        let a3336 = 0u64;
        nat_quot(a3334, a3335, a3336)
    };
    let v6: u64 = {
        let a3338 = v5;
        let a3339 = match v3.clone() {
            Adt3::C0(_, h3337, _, _, _, _, _, _) => {
                h3337
            }
        };
        let a3340 = 0u64;
        nat_quot(a3338, a3339, a3340)
    };
    let v7: u64 = {
        let a3342 = v0;
        let a3343 = match v3.clone() {
            Adt3::C0(h3341, _, _, _, _, _, _, _) => {
                h3341
            }
        };
        let a3344 = 0u64;
        nat_rem(a3342, a3343, a3344)
    };
    let v8: u64 = {
        let a3346 = v5;
        let a3347 = match v3.clone() {
            Adt3::C0(_, h3345, _, _, _, _, _, _) => {
                h3345
            }
        };
        let a3348 = 0u64;
        nat_rem(a3346, a3347, a3348)
    };
    let v9: u64 = {
        let a3350 = v6;
        let a3351 = match v3.clone() {
            Adt3::C0(_, _, h3349, _, _, _, _, _) => {
                h3349
            }
        };
        let a3352 = 0u64;
        nat_rem(a3350, a3351, a3352)
    };
    let v10: u64 = {
        let a3354 = v6;
        let a3355 = match v3.clone() {
            Adt3::C0(_, _, h3353, _, _, _, _, _) => {
                h3353
            }
        };
        let a3356 = 0u64;
        nat_quot(a3354, a3355, a3356)
    };
    let v11: u64 = {
        let a3357 = v4;
        let a3358 = 0u64;
        if nat_eq(a3357, a3358) {
            match v1.clone() {
                Adt3::C0(h3359, _, _, _, _, _, _, _) => {
                    h3359
                }
            }
        } else {
            let a3360 = v4;
            let a3361 = 1u64;
            if nat_eq(a3360, a3361) {
                match v1.clone() {
                    Adt3::C0(_, h3362, _, _, _, _, _, _) => {
                        h3362
                    }
                }
            } else {
                let a3363 = v4;
                let a3364 = 2u64;
                if nat_eq(a3363, a3364) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3365, _, _, _, _, _) => {
                            h3365
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3366, _, _, _, _) => {
                            h3366
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3373 = v11;
        let a3374 = {
            let a3367 = v4;
            let a3368 = 0u64;
            if nat_eq(a3367, a3368) {
                v7
            } else {
                let a3369 = v4;
                let a3370 = 1u64;
                if nat_eq(a3369, a3370) {
                    v8
                } else {
                    let a3371 = v4;
                    let a3372 = 2u64;
                    if nat_eq(a3371, a3372) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3373, a3374)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f123(if if v12 {
        let a3375 = v4;
        let a3376 = 0u64;
        nat_eq(a3375, a3376)
    } else {
        false
    } {
        let a3377 = v7;
        let a3378 = v11;
        nat_sub(a3377, a3378)
    } else {
        v7
    }, if if v12 {
        let a3379 = v4;
        let a3380 = 1u64;
        nat_eq(a3379, a3380)
    } else {
        false
    } {
        let a3381 = v8;
        let a3382 = v11;
        nat_sub(a3381, a3382)
    } else {
        v8
    }, if if v12 {
        let a3383 = v4;
        let a3384 = 2u64;
        nat_eq(a3383, a3384)
    } else {
        false
    } {
        let a3385 = v9;
        let a3386 = v11;
        nat_sub(a3385, a3386)
    } else {
        v9
    }, if if v12 {
        let a3387 = v4;
        let a3388 = 3u64;
        nat_eq(a3387, a3388)
    } else {
        false
    } {
        let a3389 = v10;
        let a3390 = v11;
        nat_sub(a3389, a3390)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3391, _, _, _) => {
            h3391
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3392, _, _) => {
            h3392
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3393, _) => {
            h3393
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3394) => {
            h3394
        }
    })?, f131(v0, v3)?))
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f65(v0, v2.clone(), 0u64);
    let v9: u64 = f65(v0, v2.clone(), 1u64);
    let v10: u64 = f65(v0, v2.clone(), 2u64);
    let v11: u64 = f65(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f64(v1.clone(), f193(v8, v3, match v1.clone() {
            Adt3::C0(h3395, _, _, _, _, _, _, _) => {
                h3395
            }
        })?, f193(v9, v4, match v1.clone() {
            Adt3::C0(_, h3396, _, _, _, _, _, _) => {
                h3396
            }
        })?, f193(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3397, _, _, _, _, _) => {
                h3397
            }
        })?, f193(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3398, _, _, _, _) => {
                h3398
            }
        })?)?, f131(v0, v2)?))
    } else {
        if if if if f194(v8, v3, match v1.clone() {
            Adt3::C0(h3399, _, _, _, _, _, _, _) => {
                h3399
            }
        })? {
            f194(v9, v4, match v1.clone() {
                Adt3::C0(_, h3400, _, _, _, _, _, _) => {
                    h3400
                }
            })?
        } else {
            false
        } {
            f194(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3401, _, _, _, _, _) => {
                    h3401
                }
            })?
        } else {
            false
        } {
            f194(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3402, _, _, _, _) => {
                    h3402
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f64(v1, f195(v8, v3)?, f195(v9, v4)?, f195(v10, v5)?, f195(v11, v6)?)?, f131(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f131(v0, v2)?))
        }
    }
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 1u64);
    let v6: u64 = f65(v0, v2.clone(), 2u64);
    let v7: u64 = f65(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f64(v1.clone(), f196(v4, v3, match v1 {
        Adt3::C0(h3403, _, _, _, _, _, _, _) => {
            h3403
        }
    })?, v5, v6, v7)?, f131(v0, v2)?))
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3404 = v0;
    let a3405 = v3;
    if nat_lt(a3404, a3405) {
        Ok(Adt9::C1(false, {
            let a3406 = v0;
            let a3407 = 4u64;
            nat_mul(a3406, a3407)?
        }, {
            let a3408 = v0;
            let a3409 = 4u64;
            nat_mul(a3408, a3409)?
        }))
    } else {
        let v6: u64 = {
            let a3410 = v0;
            let a3411 = v3;
            nat_sub(a3410, a3411)
        };
        let v7: u64 = f131(v6, v1)?;
        let v8: u64 = f197(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3412 = v0;
    let a3413 = v3;
    if nat_lt(a3412, a3413) {
        Ok(Adt9::C1(true, 0u64, f131(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3414 = v0;
            let a3415 = v3;
            nat_sub(a3414, a3415)
        };
        let v5: u64 = f133(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f131(v4, v1)?, 3u64, v5))
    }
}

pub fn f139(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3417 = v1;
        let a3418 = match v3.clone() {
            Adt3::C0(h3416, _, _, _, _, _, _, _) => {
                h3416
            }
        };
        let a3419 = 0u64;
        nat_rem(a3417, a3418, a3419)
    };
    let v5: u64 = {
        let a3425 = {
            let a3421 = v1;
            let a3422 = match v3.clone() {
                Adt3::C0(h3420, _, _, _, _, _, _, _) => {
                    h3420
                }
            };
            let a3423 = 0u64;
            nat_quot(a3421, a3422, a3423)
        };
        let a3426 = match v3.clone() {
            Adt3::C0(_, h3424, _, _, _, _, _, _) => {
                h3424
            }
        };
        let a3427 = 0u64;
        nat_rem(a3425, a3426, a3427)
    };
    let v6: bool = f198(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f133(v1, v2, v3.clone())?
    } else {
        0u64
    }, f131(v1, v3)?))
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3428 = v3;
        let a3429 = 8u64;
        nat_add(a3428, a3429)?
    }, 0u64, f131(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f133(v0, v5, v2.clone())?
    }, 3u64, f131(v0, v2)?))
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3431 = v0;
        let a3432 = match v2.clone() {
            Adt3::C0(h3430, _, _, _, _, _, _, _) => {
                h3430
            }
        };
        let a3433 = 0u64;
        nat_rem(a3431, a3432, a3433)
    };
    let v7: u64 = {
        let a3435 = v0;
        let a3436 = match v2.clone() {
            Adt3::C0(h3434, _, _, _, _, _, _, _) => {
                h3434
            }
        };
        let a3437 = 0u64;
        nat_quot(a3435, a3436, a3437)
    };
    let v8: u64 = {
        let a3439 = v7;
        let a3440 = match v2.clone() {
            Adt3::C0(_, h3438, _, _, _, _, _, _) => {
                h3438
            }
        };
        let a3441 = 0u64;
        nat_rem(a3439, a3440, a3441)
    };
    let v9: u64 = {
        let a3443 = v7;
        let a3444 = match v2.clone() {
            Adt3::C0(_, h3442, _, _, _, _, _, _) => {
                h3442
            }
        };
        let a3445 = 0u64;
        nat_quot(a3443, a3444, a3445)
    };
    let v10: u64 = {
        let a3447 = v9;
        let a3448 = match v2.clone() {
            Adt3::C0(_, _, h3446, _, _, _, _, _) => {
                h3446
            }
        };
        let a3449 = 0u64;
        nat_rem(a3447, a3448, a3449)
    };
    let v11: u64 = {
        let a3451 = v9;
        let a3452 = match v2.clone() {
            Adt3::C0(_, _, h3450, _, _, _, _, _) => {
                h3450
            }
        };
        let a3453 = 0u64;
        nat_quot(a3451, a3452, a3453)
    };
    let v12: u64 = f131(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3462 = {
                let a3457 = {
                    let a3454 = v10;
                    let a3455 = v4;
                    let a3456 = 0u64;
                    nat_quot(a3454, a3455, a3456)
                };
                let a3458 = v5;
                nat_mul(a3457, a3458)?
            };
            let a3463 = {
                let a3459 = v8;
                let a3460 = v4;
                let a3461 = 0u64;
                nat_quot(a3459, a3460, a3461)
            };
            nat_add(a3462, a3463)?
        };
        let v14: u64 = {
            let a3483 = v6;
            let a3484 = {
                let a3481 = match v1.clone() {
                    Adt3::C0(h3464, _, _, _, _, _, _, _) => {
                        h3464
                    }
                };
                let a3482 = {
                    let a3479 = {
                        let a3465 = v8;
                        let a3466 = v4;
                        let a3467 = 0u64;
                        nat_rem(a3465, a3466, a3467)
                    };
                    let a3480 = {
                        let a3477 = match v1.clone() {
                            Adt3::C0(_, h3468, _, _, _, _, _, _) => {
                                h3468
                            }
                        };
                        let a3478 = {
                            let a3475 = {
                                let a3469 = v10;
                                let a3470 = v4;
                                let a3471 = 0u64;
                                nat_rem(a3469, a3470, a3471)
                            };
                            let a3476 = {
                                let a3473 = match v1.clone() {
                                    Adt3::C0(_, _, h3472, _, _, _, _, _) => {
                                        h3472
                                    }
                                };
                                let a3474 = v13;
                                nat_mul(a3473, a3474)?
                            };
                            nat_add(a3475, a3476)?
                        };
                        nat_mul(a3477, a3478)?
                    };
                    nat_add(a3479, a3480)?
                };
                nat_mul(a3481, a3482)?
            };
            nat_add(a3483, a3484)?
        };
        Ok(Adt9::C1(false, f131(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3490 = {
                let a3488 = {
                    let a3485 = v11;
                    let a3486 = v5;
                    let a3487 = 0u64;
                    nat_rem(a3485, a3486, a3487)
                };
                let a3489 = v4;
                nat_mul(a3488, a3489)?
            };
            let a3491 = v8;
            nat_add(a3490, a3491)?
        };
        let v16: u64 = {
            let a3497 = {
                let a3495 = {
                    let a3492 = v11;
                    let a3493 = v5;
                    let a3494 = 0u64;
                    nat_quot(a3492, a3493, a3494)
                };
                let a3496 = v4;
                nat_mul(a3495, a3496)?
            };
            let a3498 = v10;
            nat_add(a3497, a3498)?
        };
        let a3500 = v15;
        let a3501 = match v1.clone() {
            Adt3::C0(_, h3499, _, _, _, _, _, _) => {
                h3499
            }
        };
        if if nat_lt(a3500, a3501) {
            let a3503 = v16;
            let a3504 = match v1.clone() {
                Adt3::C0(_, _, h3502, _, _, _, _, _) => {
                    h3502
                }
            };
            nat_lt(a3503, a3504)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f131({
                let a3513 = v6;
                let a3514 = {
                    let a3511 = match v1.clone() {
                        Adt3::C0(h3505, _, _, _, _, _, _, _) => {
                            h3505
                        }
                    };
                    let a3512 = {
                        let a3509 = v15;
                        let a3510 = {
                            let a3507 = match v1.clone() {
                                Adt3::C0(_, h3506, _, _, _, _, _, _) => {
                                    h3506
                                }
                            };
                            let a3508 = v16;
                            nat_mul(a3507, a3508)?
                        };
                        nat_add(a3509, a3510)?
                    };
                    nat_mul(a3511, a3512)?
                };
                nat_add(a3513, a3514)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f131(v0, v2.clone())?;
    let v4: u64 = f131(v0, v1.clone())?;
    let a3519 = {
        let a3516 = v0;
        let a3517 = match v1 {
            Adt3::C0(h3515, _, _, _, _, _, _, _) => {
                h3515
            }
        };
        let a3518 = 0u64;
        nat_rem(a3516, a3517, a3518)
    };
    let a3520 = 0u64;
    if nat_eq(a3519, a3520) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f131({
            let a3521 = v0;
            let a3522 = 1u64;
            nat_sub(a3521, a3522)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3523 = v3;
    let a3524 = 13u64;
    if nat_eq(a3523, a3524) {
        f199(v0, v1, v4)
    } else {
        let v5: u64 = f188(match v1.clone() {
            Adt3::C0(h3525, _, _, _, _, _, _, _) => {
                h3525
            }
        }, {
            let a3526 = v3;
            let a3527 = 16u64;
            nat_eq(a3526, a3527)
        })?;
        let v6: u64 = {
            let a3528 = v0;
            let a3529 = v5;
            let a3530 = 0u64;
            nat_quot(a3528, a3529, a3530)
        };
        let v7: u64 = {
            let a3531 = v0;
            let a3532 = v5;
            let a3533 = 0u64;
            nat_rem(a3531, a3532, a3533)
        };
        let v8: u64 = {
            let a3537 = 1u64;
            let a3538 = {
                let a3535 = match v1.clone() {
                    Adt3::C0(h3534, _, _, _, _, _, _, _) => {
                        h3534
                    }
                };
                let a3536 = 2u64;
                nat_mul(a3535, a3536)?
            };
            nat_add(a3537, a3538)?
        };
        let a3539 = v7;
        let a3540 = v8;
        if nat_lt(a3539, a3540) {
            f200(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f131(v6, v2)?;
            let a3541 = v3;
            let a3542 = 14u64;
            if nat_eq(a3541, a3542) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3543 = v7;
                let a3544 = v8;
                if nat_eq(a3543, a3544) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3547 = v7;
                    let a3548 = {
                        let a3545 = v8;
                        let a3546 = 1u64;
                        nat_add(a3545, a3546)?
                    };
                    if nat_eq(a3547, a3548) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3549, _, _, _, _, _, _, _) => {
                                h3549
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

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3550 = v0;
    let a3551 = 0u64;
    if nat_eq(a3550, a3551) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3558 = {
            let a3552 = v0;
            let a3553 = 1u64;
            nat_add(a3552, a3553)?
        };
        let a3559 = {
            let a3556 = 2u64;
            let a3557 = {
                let a3554 = v3;
                let a3555 = 2u64;
                nat_mul(a3554, a3555)?
            };
            nat_add(a3556, a3557)?
        };
        if nat_eq(a3558, a3559) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3560 = v0;
                let a3561 = 1u64;
                nat_sub(a3560, a3561)
            };
            let v5: u64 = {
                let a3562 = v4;
                let a3563 = 2u64;
                let a3564 = 0u64;
                nat_quot(a3562, a3563, a3564)
            };
            let a3568 = {
                let a3565 = v4;
                let a3566 = 2u64;
                let a3567 = 0u64;
                nat_rem(a3565, a3566, a3567)
            };
            let a3569 = 0u64;
            if nat_eq(a3568, a3569) {
                Ok(Adt9::C4(17u64, 0u64, f131(v5, v1)?, 4u64, f131(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3570 = v0;
    let a3571 = f24(v1, v2, v3)?;
    if nat_lt(a3570, a3571) {
        let a3572 = v0;
        let a3573 = v1;
        if nat_lt(a3572, a3573) {
            Ok(Adt9::C1(false, {
                let a3574 = 4u64;
                let a3575 = v0;
                nat_mul(a3574, a3575)?
            }, {
                let a3576 = 4u64;
                let a3577 = v0;
                nat_mul(a3576, a3577)?
            }))
        } else {
            let v4: u64 = {
                let a3578 = v0;
                let a3579 = v1;
                nat_sub(a3578, a3579)
            };
            let v5: u64 = f201(v4, v2)?;
            let a3580 = v5;
            let a3581 = 2u64;
            if nat_lt(a3580, a3581) {
                Ok(Adt9::C4(6u64, {
                    let a3582 = v5;
                    let a3583 = 0u64;
                    if nat_eq(a3582, a3583) {
                        4u64
                    } else {
                        5u64
                    }
                }, f202(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f203(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f204(v4, v2)? {
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

pub fn f146(v0: u64, v1: u64) -> Adt11 {
    let a3584 = v0;
    let a3585 = 43u64;
    if nat_eq(a3584, a3585) {
        Adt11::C8(3u64)
    } else {
        let a3586 = v1;
        let a3587 = 43u64;
        if nat_eq(a3586, a3587) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f147(v0: u64, v1: u64) -> R<u64> {
    if f98(v0) {
        let a3588 = 2u64;
        let a3589 = v1;
        nat_mul(a3588, a3589)
    } else {
        let a3590 = v1;
        let a3591 = f184(v0);
        let a3592 = 0u64;
        Ok(nat_quot(a3590, a3591, a3592))
    }
}

pub fn f148(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m3593 = f108(v0);
    match m3593 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f123(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h3594, _, _, _) => {
                    h3594
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h3595, _, _) => {
                    h3595
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h3596, _) => {
                    h3596
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h3597) => {
                    h3597
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h3598) => {
                    h3598
                }
            }, f123({
                let a3600 = v2;
                let a3601 = match v6.clone() {
                    Adt18::C0(_, h3599, _) => {
                        h3599
                    }
                };
                nat_mul(a3600, a3601)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h3602, _, _, _) => {
                    h3602
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h3603, _, _) => {
                    h3603
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h3604, _) => {
                    h3604
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h3605) => {
                    h3605
                }
            })?, match v6 {
                Adt18::C0(_, h3606, _) => {
                    h3606
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h3607, _, _, _) => {
                    h3607
                }
            }))
        }
    }
}

pub fn f149(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3615 = {
        let a3608 = 2u64;
        let a3609 = f120(v1.clone())?;
        nat_mul(a3608, a3609)?
    };
    let a3616 = {
        let a3613 = f120(v1)?;
        let a3614 = {
            let a3611 = 1u64;
            let a3612 = match v0 {
                Adt3::C0(h3610, _, _, _, _, _, _, _) => {
                    h3610
                }
            };
            nat_add(a3611, a3612)?
        };
        nat_mul(a3613, a3614)?
    };
    nat_add(a3615, a3616)
}

pub fn f150(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f131(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f151(v0: u64, v1: u64) -> bool {
    let a3617 = v1;
    let a3618 = 2147483647u64;
    if nat_le(a3617, a3618) {
        let a3619 = v1;
        let a3620 = v0;
        nat_lt(a3619, a3620)
    } else {
        false
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3621 = f108(v0);
    match m3621 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a3623 = v2;
                let a3624 = match v7.clone() {
                    Adt18::C0(_, h3622, _) => {
                        h3622
                    }
                };
                nat_mul(a3623, a3624)?
            };
            let m3637 = f205(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3625, _, _, _, _, _, _, _) => {
                    h3625
                }
            }, match v5.clone() {
                Adt3::C0(_, h3626, _, _, _, _, _, _) => {
                    h3626
                }
            }, match v5 {
                Adt3::C0(_, _, h3627, _, _, _, _, _) => {
                    h3627
                }
            }, {
                let a3630 = match v3.clone() {
                    Adt3::C0(h3628, _, _, _, _, _, _, _) => {
                        h3628
                    }
                };
                let a3631 = match v7.clone() {
                    Adt18::C0(_, h3629, _) => {
                        h3629
                    }
                };
                nat_mul(a3630, a3631)?
            }, match v3.clone() {
                Adt3::C0(_, h3632, _, _, _, _, _, _) => {
                    h3632
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3633, _, _, _) => {
                    h3633
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3634, _, _) => {
                    h3634
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3635, _) => {
                    h3635
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3636) => {
                    h3636
                }
            })?;
            match m3637 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3638 = v0;
                    let a3639 = 0u64;
                    if if nat_eq(a3638, a3639) {
                        true
                    } else {
                        let a3640 = v0;
                        let a3641 = 26u64;
                        nat_eq(a3640, a3641)
                    } {
                        Ok(Adt9::C1(false, v9, f131({
                            let a3645 = {
                                let a3643 = v1;
                                let a3644 = match v4.clone() {
                                    Adt3::C0(h3642, _, _, _, _, _, _, _) => {
                                        h3642
                                    }
                                };
                                nat_mul(a3643, a3644)?
                            };
                            let a3646 = v8;
                            nat_add(a3645, a3646)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h3647) => {
                                h3647
                            }
                        }, f131({
                            let a3651 = {
                                let a3649 = v1;
                                let a3650 = match v4.clone() {
                                    Adt3::C0(h3648, _, _, _, _, _, _, _) => {
                                        h3648
                                    }
                                };
                                nat_mul(a3649, a3650)?
                            };
                            let a3652 = v8;
                            nat_add(a3651, a3652)?
                        }, v4.clone())?, {
                            let a3659 = {
                                let a3657 = {
                                    let a3654 = match v7 {
                                        Adt18::C0(_, h3653, _) => {
                                            h3653
                                        }
                                    };
                                    let a3655 = 1u64;
                                    nat_sub(a3654, a3655)
                                };
                                let a3658 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3656, _, _, _) => {
                                        h3656
                                    }
                                };
                                nat_mul(a3657, a3658)?
                            };
                            let a3660 = 4u64;
                            nat_add(a3659, a3660)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3661, _, _, _) => {
                                h3661
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f153(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3662 = v1;
    let a3663 = 0u64;
    if if nat_eq(a3662, a3663) {
        true
    } else {
        let a3664 = v2;
        let a3665 = 0u64;
        nat_eq(a3664, a3665)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f154(v0: u64, v1: u64) -> bool {
    let a3666 = v0;
    let a3667 = 4294967295u64;
    if nat_eq(a3666, a3667) {
        true
    } else {
        let a3668 = v0;
        let a3669 = 2147483648u64;
        if nat_lt(a3668, a3669) {
            let a3670 = v0;
            let a3671 = v1;
            nat_lt(a3670, a3671)
        } else {
            false
        }
    }
}

pub fn f155(v0: Adt7) -> R<u64> {
    let a3676 = if match v0.clone() {
        Adt7::C0(_, _, _, h3672, _) => {
            h3672
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3677 = {
        let a3674 = match v0 {
            Adt7::C0(_, _, _, _, h3673) => {
                h3673
            }
        };
        let a3675 = 1u64;
        if nat_eq(a3674, a3675) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3676, a3677)
}

pub fn f156(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3682 = {
            let a3679 = v2;
            let a3680 = match v0.clone() {
                Adt6::C0(_, h3678, _, _) => {
                    h3678
                }
            };
            nat_mul(a3679, a3680)?
        };
        let a3683 = f124(match v0.clone() {
            Adt6::C0(_, _, h3681, _) => {
                h3681
            }
        });
        nat_add(a3682, a3683)?
    };
    let v4: u64 = f125(match v0.clone() {
        Adt6::C0(_, _, h3684, _) => {
            h3684
        }
    });
    let v5: u64 = {
        let a3688 = {
            let a3686 = v3;
            let a3687 = match v0 {
                Adt6::C0(h3685, _, _, _) => {
                    h3685
                }
            };
            nat_add(a3686, a3687)?
        };
        let a3689 = v4;
        nat_sub(a3688, a3689)
    };
    let a3694 = {
        let a3690 = v5;
        let a3691 = v1;
        if nat_lt(a3690, a3691) {
            v5
        } else {
            v1
        }
    };
    let a3695 = {
        let a3692 = v3;
        let a3693 = v4;
        nat_sub(a3692, a3693)
    };
    Ok(nat_sub(a3694, a3695))
}

pub fn f157(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3704 = f67(match v0.clone() {
        Adt7::C0(h3696, _, _, _, _) => {
            h3696
        }
    }, match v1.clone() {
        Adt3::C0(h3697, _, _, _, _, _, _, _) => {
            h3697
        }
    }, f65(v3, v2.clone(), 0u64), {
        let a3701 = v4;
        let a3702 = {
            let m3700 = match v0.clone() {
                Adt7::C0(h3699, _, _, _, _) => {
                    h3699
                }
            };
            match m3700 {
                Adt6::C0(h3698, _, _, _) => {
                    h3698
                }
            }
        };
        let a3703 = 0u64;
        nat_rem(a3701, a3702, a3703)
    })?;
    match m3704 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3713 = f67(match v0.clone() {
                Adt7::C0(_, h3705, _, _, _) => {
                    h3705
                }
            }, match v1.clone() {
                Adt3::C0(_, h3706, _, _, _, _, _, _) => {
                    h3706
                }
            }, f65(v3, v2.clone(), 1u64), {
                let a3710 = v4;
                let a3711 = {
                    let m3709 = match v0 {
                        Adt7::C0(h3708, _, _, _, _) => {
                            h3708
                        }
                    };
                    match m3709 {
                        Adt6::C0(h3707, _, _, _) => {
                            h3707
                        }
                    }
                };
                let a3712 = 0u64;
                nat_quot(a3710, a3711, a3712)
            })?;
            match m3713 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f64(v1, v5, v6, f65(v3, v2.clone(), 2u64), f65(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f158(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m3714 = v0;
    match m3714 {
        Adt16::C0(v19) => {
            f206(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a3715 = v20;
            let a3716 = 66u64;
            if nat_eq(a3715, a3716) {
                f207(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f208(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3723 = v3;
        let a3724 = {
            let a3720 = f131({
                let a3717 = v0;
                let a3718 = 2u64;
                let a3719 = 0u64;
                nat_quot(a3717, a3718, a3719)
            }, v1)?;
            let a3721 = v2;
            let a3722 = 0u64;
            nat_quot(a3720, a3721, a3722)
        };
        nat_add(a3723, a3724)?
    };
    let a3728 = {
        let a3725 = v0;
        let a3726 = 2u64;
        let a3727 = 0u64;
        nat_rem(a3725, a3726, a3727)
    };
    let a3729 = 0u64;
    if nat_eq(a3728, a3729) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3730 = v4;
        let a3731 = 0u64;
        if nat_eq(a3730, a3731) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f160(v0: u64, v1: u64) -> R<u64> {
    let a3734 = 8u64;
    let a3735 = {
        let a3732 = f209(v0);
        let a3733 = v1;
        nat_mul(a3732, a3733)?
    };
    nat_add(a3734, a3735)
}

pub fn f161(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3736, _, _, _, _, _, _, _) => {
            h3736
        }
    };
    let v7: u64 = f209(v3);
    let v8: u64 = {
        let a3739 = 1u64;
        let a3740 = {
            let a3737 = v6;
            let a3738 = v7;
            nat_mul(a3737, a3738)?
        };
        nat_add(a3739, a3740)?
    };
    let a3741 = v0;
    let a3742 = 0u64;
    if nat_eq(a3741, a3742) {
        Ok(f210(0u64))
    } else {
        let a3743 = v0;
        let a3744 = v8;
        if nat_lt(a3743, a3744) {
            let v9: u64 = {
                let a3745 = v0;
                let a3746 = 1u64;
                nat_sub(a3745, a3746)
            };
            let v10: u64 = {
                let a3747 = v9;
                let a3748 = v7;
                let a3749 = 0u64;
                nat_rem(a3747, a3748, a3749)
            };
            let v11: u64 = f131({
                let a3755 = {
                    let a3750 = v1;
                    let a3751 = v6;
                    nat_mul(a3750, a3751)?
                };
                let a3756 = {
                    let a3752 = v9;
                    let a3753 = v7;
                    let a3754 = 0u64;
                    nat_quot(a3752, a3753, a3754)
                };
                nat_add(a3755, a3756)?
            }, v2)?;
            let a3757 = v3;
            let a3758 = 30u64;
            if nat_eq(a3757, a3758) {
                let a3759 = v10;
                let a3760 = 0u64;
                if nat_eq(a3759, a3760) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3761 = v10;
                    let a3762 = 1u64;
                    if nat_eq(a3761, a3762) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3763 = v10;
                        let a3764 = 2u64;
                        if nat_eq(a3763, a3764) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3765 = v10;
                let a3766 = 0u64;
                if nat_eq(a3765, a3766) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3767 = v10;
                    let a3768 = 1u64;
                    if nat_eq(a3767, a3768) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3769 = v0;
                let a3770 = v8;
                nat_sub(a3769, a3770)
            };
            let a3771 = v12;
            let a3772 = 0u64;
            if nat_eq(a3771, a3772) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3773 = v12;
                let a3774 = 1u64;
                if nat_eq(a3773, a3774) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3775 = v12;
                    let a3776 = 2u64;
                    if nat_eq(a3775, a3776) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3777 = v12;
                        let a3778 = 3u64;
                        if nat_eq(a3777, a3778) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3779 = v12;
                            let a3780 = 4u64;
                            if nat_eq(a3779, a3780) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3781 = v12;
                                let a3782 = 5u64;
                                if nat_eq(a3781, a3782) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3783 = v12;
                                    let a3784 = 6u64;
                                    if nat_eq(a3783, a3784) {
                                        if f211(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3785 = 3u64;
                                                let a3786 = v1;
                                                nat_add(a3785, a3786)?
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

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3787 = v0;
        let a3788 = f96(v3);
        let a3789 = 0u64;
        nat_quot(a3787, a3788, a3789)
    };
    let v5: u64 = {
        let a3794 = 3u64;
        let a3795 = {
            let a3791 = v4;
            let a3792 = match v1.clone() {
                Adt3::C0(h3790, _, _, _, _, _, _, _) => {
                    h3790
                }
            };
            let a3793 = 0u64;
            nat_quot(a3791, a3792, a3793)
        };
        nat_add(a3794, a3795)?
    };
    let v6: u64 = f131(v4, v1)?;
    let v7: u64 = f131(v4, v2)?;
    let a3796 = v3;
    let a3797 = 30u64;
    if nat_eq(a3796, a3797) {
        let a3801 = {
            let a3798 = v0;
            let a3799 = 3u64;
            let a3800 = 0u64;
            nat_rem(a3798, a3799, a3800)
        };
        let a3802 = 0u64;
        if nat_eq(a3801, a3802) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3806 = {
                let a3803 = v0;
                let a3804 = 3u64;
                let a3805 = 0u64;
                nat_rem(a3803, a3804, a3805)
            };
            let a3807 = 1u64;
            if nat_eq(a3806, a3807) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f163(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3808 = v0;
        let a3809 = 8388608u64;
        let a3810 = 0u64;
        nat_quot(a3808, a3809, a3810)
    };
    let a3811 = v0;
    let a3812 = 2147483648u64;
    if if if nat_lt(a3811, a3812) {
        let a3813 = 127u64;
        let a3814 = v1;
        nat_le(a3813, a3814)
    } else {
        false
    } {
        let a3815 = v1;
        let a3816 = 190u64;
        nat_lt(a3815, a3816)
    } else {
        false
    } {
        let v2: u64 = {
            let a3820 = 8388608u64;
            let a3821 = {
                let a3817 = v0;
                let a3818 = 8388608u64;
                let a3819 = 0u64;
                nat_rem(a3817, a3818, a3819)
            };
            nat_add(a3820, a3821)?
        };
        let a3822 = v1;
        let a3823 = 150u64;
        if nat_lt(a3822, a3823) {
            let v3: u64 = f212({
                let a3824 = 150u64;
                let a3825 = v1;
                nat_sub(a3824, a3825)
            });
            let a3829 = {
                let a3826 = v2;
                let a3827 = v3;
                let a3828 = 0u64;
                nat_rem(a3826, a3827, a3828)
            };
            let a3830 = 0u64;
            if nat_eq(a3829, a3830) {
                let a3831 = v2;
                let a3832 = v3;
                let a3833 = 0u64;
                Ok(nat_quot(a3831, a3832, a3833))
            } else {
                Ok(0u64)
            }
        } else {
            let a3836 = f212({
                let a3834 = v1;
                let a3835 = 150u64;
                nat_sub(a3834, a3835)
            });
            let a3837 = v2;
            nat_mul(a3836, a3837)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f164(v0: Adt14, v1: Adt14) -> bool {
    let a3840 = match v0.clone() {
        Adt14::C0(_, h3838, _, _, _, _, _, _, _, _, _) => {
            h3838
        }
    };
    let a3841 = match v1.clone() {
        Adt14::C0(_, h3839, _, _, _, _, _, _, _, _, _) => {
            h3839
        }
    };
    if if if nat_eq(a3840, a3841) {
        let a3844 = match v0.clone() {
            Adt14::C0(_, _, h3842, _, _, _, _, _, _, _, _) => {
                h3842
            }
        };
        let a3845 = match v1.clone() {
            Adt14::C0(_, _, h3843, _, _, _, _, _, _, _, _) => {
                h3843
            }
        };
        nat_eq(a3844, a3845)
    } else {
        false
    } {
        let a3848 = match v0.clone() {
            Adt14::C0(_, _, _, h3846, _, _, _, _, _, _, _) => {
                h3846
            }
        };
        let a3849 = match v1.clone() {
            Adt14::C0(_, _, _, h3847, _, _, _, _, _, _, _) => {
                h3847
            }
        };
        nat_eq(a3848, a3849)
    } else {
        false
    } {
        let a3852 = match v0 {
            Adt14::C0(_, _, _, _, h3850, _, _, _, _, _, _) => {
                h3850
            }
        };
        let a3853 = match v1 {
            Adt14::C0(_, _, _, _, h3851, _, _, _, _, _, _) => {
                h3851
            }
        };
        nat_eq(a3852, a3853)
    } else {
        false
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3854 = v0;
    let a3855 = 0u64;
    if if if nat_eq(a3854, a3855) {
        true
    } else {
        let a3856 = v1;
        let a3857 = 0u64;
        nat_eq(a3856, a3857)
    } {
        true
    } else {
        let a3858 = v2;
        let a3859 = 0u64;
        nat_eq(a3858, a3859)
    } {
        true
    } else {
        let a3860 = v3;
        let a3861 = 0u64;
        nat_eq(a3860, a3861)
    }
}

pub fn f166(v0: Adt14, v1: Adt14) -> bool {
    if f172(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3862, _, _, _, _, _) => {
            h3862
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h3863, _, _, _, _, _) => {
            h3863
        }
    }) {
        let a3865 = match v0 {
            Adt14::C0(_, h3864, _, _, _, _, _, _, _, _, _) => {
                h3864
            }
        };
        let a3866 = 2147483647u64;
        nat_le(a3865, a3866)
    } else {
        true
    }
}

pub fn f167(v0: Adt14) -> bool {
    let a3871 = {
        let a3868 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3867) => {
                h3867
            }
        };
        let a3869 = 4u64;
        let a3870 = 0u64;
        nat_rem(a3868, a3869, a3870)
    };
    let a3872 = 0u64;
    if if nat_eq(a3871, a3872) {
        f213(match v0.clone() {
            Adt14::C0(_, h3873, _, _, _, _, _, _, _, _, _) => {
                h3873
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3874, _, _, _, _, _) => {
                h3874
            }
        })
    } else {
        false
    } {
        f92(match v0.clone() {
            Adt14::C0(_, _, h3875, _, _, _, _, _, _, _, _) => {
                h3875
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3876, _, _, _, _, _, _, _) => {
                h3876
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3877, _, _, _, _, _, _) => {
                h3877
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3878, _, _, _, _) => {
                h3878
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3879, _, _, _) => {
                h3879
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3880, _, _) => {
                h3880
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f168(v0: Adt14, v1: Adt14) -> R<bool> {
    let a3882 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3881, _) => {
            h3881
        }
    };
    let a3883 = 0u64;
    if nat_eq(a3882, a3883) {
        Ok(true)
    } else {
        let a3885 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h3884, _) => {
                h3884
            }
        };
        let a3886 = 0u64;
        if nat_eq(a3885, a3886) {
            Ok(true)
        } else {
            let a3888 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3887) => {
                    h3887
                }
            };
            let a3889 = 0u64;
            if nat_eq(a3888, a3889) {
                Ok(true)
            } else {
                let a3891 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3890) => {
                        h3890
                    }
                };
                let a3892 = 0u64;
                if nat_eq(a3891, a3892) {
                    Ok(true)
                } else {
                    let a3898 = {
                        let a3895 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3893) => {
                                h3893
                            }
                        };
                        let a3896 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h3894, _) => {
                                h3894
                            }
                        };
                        nat_add(a3895, a3896)?
                    };
                    let a3899 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3897) => {
                            h3897
                        }
                    };
                    if nat_le(a3898, a3899) {
                        Ok(true)
                    } else {
                        let a3905 = {
                            let a3902 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3900) => {
                                    h3900
                                }
                            };
                            let a3903 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h3901, _) => {
                                    h3901
                                }
                            };
                            nat_add(a3902, a3903)?
                        };
                        let a3906 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3904) => {
                                h3904
                            }
                        };
                        Ok(nat_le(a3905, a3906))
                    }
                }
            }
        }
    }
}

pub fn f169(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f164(v0.clone(), v1.clone()) {
        let a3909 = match v0.clone() {
            Adt14::C0(h3907, _, _, _, _, _, _, _, _, _, _) => {
                h3907
            }
        };
        let a3910 = match v1.clone() {
            Adt14::C0(h3908, _, _, _, _, _, _, _, _, _, _) => {
                h3908
            }
        };
        nat_eq(a3909, a3910)
    } else {
        false
    } {
        let a3913 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3911) => {
                h3911
            }
        };
        let a3914 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3912) => {
                h3912
            }
        };
        nat_eq(a3913, a3914)
    } else {
        false
    } {
        let a3917 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3915, _, _, _, _, _) => {
                h3915
            }
        };
        let a3918 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3916, _, _, _, _, _) => {
                h3916
            }
        };
        nat_eq(a3917, a3918)
    } else {
        false
    } {
        let a3921 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3919, _, _, _, _) => {
                h3919
            }
        };
        let a3922 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h3920, _, _, _, _) => {
                h3920
            }
        };
        nat_eq(a3921, a3922)
    } else {
        false
    } {
        let a3925 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3923, _, _, _) => {
                h3923
            }
        };
        let a3926 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3924, _, _, _) => {
                h3924
            }
        };
        nat_eq(a3925, a3926)
    } else {
        false
    } {
        let a3929 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3927, _, _) => {
                h3927
            }
        };
        let a3930 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3928, _, _) => {
                h3928
            }
        };
        nat_eq(a3929, a3930)
    } else {
        false
    }
}

pub fn f170(v0: Adt14) -> R<bool> {
    if f214(match v0.clone() {
        Adt14::C0(_, h3931, _, _, _, _, _, _, _, _, _) => {
            h3931
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3932, _, _, _, _, _) => {
            h3932
        }
    }) {
        f93(f215(match v0.clone() {
            Adt14::C0(_, h3933, _, _, _, _, _, _, _, _, _) => {
                h3933
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3934, _, _, _, _, _) => {
                h3934
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h3935, _, _, _, _, _, _, _, _) => {
                h3935
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3936, _, _, _, _, _, _, _) => {
                h3936
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3937, _, _, _, _, _, _) => {
                h3937
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3938, _, _, _, _) => {
                h3938
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3939, _, _, _) => {
                h3939
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3940, _, _) => {
                h3940
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f171(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3941 = v1;
    let a3942 = 0u64;
    if if nat_eq(a3941, a3942) {
        true
    } else {
        let a3943 = v2;
        let a3944 = 0u64;
        nat_eq(a3943, a3944)
    } {
        Ok(0u64)
    } else {
        let a3947 = f94(v1, v2, v3)?;
        let a3948 = {
            let a3945 = v2;
            let a3946 = f216(v0, v1)?;
            nat_mul(a3945, a3946)?
        };
        nat_add(a3947, a3948)
    }
}

pub fn f172(v0: u64, v1: u64) -> bool {
    let a3949 = v0;
    let a3950 = 4u64;
    if nat_eq(a3949, a3950) {
        let a3951 = v1;
        let a3952 = 4u64;
        nat_eq(a3951, a3952)
    } else {
        false
    }
}

pub fn f173(v0: u64) -> R<u64> {
    let a3953 = 6u64;
    let a3954 = v0;
    nat_add(a3953, a3954)
}

pub fn f174(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h3955, _, _, _, _, _, _, _, _, _, _) => {
            h3955
        }
    }, match v0.clone() {
        Adt14::C0(_, h3956, _, _, _, _, _, _, _, _, _) => {
            h3956
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h3957, _, _, _, _, _, _, _, _) => {
            h3957
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h3958, _, _, _, _, _, _, _) => {
            h3958
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h3959, _, _, _, _, _, _) => {
            h3959
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h3960, _, _, _, _) => {
            h3960
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h3961, _, _, _) => {
            h3961
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h3962, _, _) => {
            h3962
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3963, _) => {
            h3963
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3964) => {
            h3964
        }
    })
}

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a3965 = v0;
    let a3966 = 2147483647u64;
    if if if nat_le(a3965, a3966) {
        let a3967 = 0u64;
        let a3968 = v3;
        if nat_lt(a3967, a3968) {
            let a3969 = v3;
            let a3970 = 2147483647u64;
            nat_le(a3969, a3970)
        } else {
            false
        }
    } else {
        false
    } {
        let a3975 = {
            let a3973 = {
                let a3971 = v2;
                let a3972 = v3;
                nat_add(a3971, a3972)?
            };
            let a3974 = 1u64;
            nat_sub(a3973, a3974)
        };
        let a3976 = 2147483647u64;
        nat_le(a3975, a3976)
    } else {
        false
    } {
        let a3981 = {
            let a3979 = {
                let a3977 = v0;
                let a3978 = v1;
                nat_mul(a3977, a3978)?
            };
            let a3980 = f217(v2, v3)?;
            nat_mul(a3979, a3980)?
        };
        let a3982 = 9223372036854775807u64;
        Ok(nat_le(a3981, a3982))
    } else {
        Ok(false)
    }
}

pub fn f176(v0: Adt14) -> R<u64> {
    let a4013 = {
        let a4005 = {
            let a3997 = {
                let a3989 = 4u64;
                let a3990 = {
                    let a3987 = {
                        let a3984 = match v0.clone() {
                            Adt14::C0(_, h3983, _, _, _, _, _, _, _, _, _) => {
                                h3983
                            }
                        };
                        let a3985 = 1u64;
                        nat_sub(a3984, a3985)
                    };
                    let a3988 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h3986, _, _, _, _, _) => {
                            h3986
                        }
                    };
                    nat_mul(a3987, a3988)?
                };
                nat_add(a3989, a3990)?
            };
            let a3998 = {
                let a3995 = {
                    let a3992 = match v0.clone() {
                        Adt14::C0(_, _, h3991, _, _, _, _, _, _, _, _) => {
                            h3991
                        }
                    };
                    let a3993 = 1u64;
                    nat_sub(a3992, a3993)
                };
                let a3996 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h3994, _, _, _, _) => {
                        h3994
                    }
                };
                nat_mul(a3995, a3996)?
            };
            nat_add(a3997, a3998)?
        };
        let a4006 = {
            let a4003 = {
                let a4000 = match v0.clone() {
                    Adt14::C0(_, _, _, h3999, _, _, _, _, _, _, _) => {
                        h3999
                    }
                };
                let a4001 = 1u64;
                nat_sub(a4000, a4001)
            };
            let a4004 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4002, _, _, _) => {
                    h4002
                }
            };
            nat_mul(a4003, a4004)?
        };
        nat_add(a4005, a4006)?
    };
    let a4014 = {
        let a4011 = {
            let a4008 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4007, _, _, _, _, _, _) => {
                    h4007
                }
            };
            let a4009 = 1u64;
            nat_sub(a4008, a4009)
        };
        let a4012 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4010, _, _) => {
                h4010
            }
        };
        nat_mul(a4011, a4012)?
    };
    nat_add(a4013, a4014)
}

pub fn f177(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4015 = v0;
    let a4016 = 0u64;
    if if if if nat_eq(a4015, a4016) {
        true
    } else {
        let a4017 = v1;
        let a4018 = 0u64;
        nat_eq(a4017, a4018)
    } {
        true
    } else {
        let a4019 = v2;
        let a4020 = 0u64;
        nat_eq(a4019, a4020)
    } {
        true
    } else {
        let a4021 = v3;
        let a4022 = 0u64;
        nat_eq(a4021, a4022)
    } {
        Ok(0u64)
    } else {
        let a4023 = f218(v0, v1, v2, v3, v5)?;
        let a4024 = f219(v0, v1, v2, v3, v4)?;
        nat_add(a4023, a4024)
    }
}

pub fn f178(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4025 = v0;
    let a4026 = 0u64;
    if if nat_eq(a4025, a4026) {
        true
    } else {
        let a4027 = v1;
        let a4028 = 0u64;
        nat_eq(a4027, a4028)
    } {
        Ok(0u64)
    } else {
        let a4031 = f94(v0, v1, v2)?;
        let a4032 = {
            let a4029 = v1;
            let a4030 = f220(v0)?;
            nat_mul(a4029, a4030)?
        };
        nat_add(a4031, a4032)
    }
}

pub fn f179(v0: u64, v1: u64, v2: u64) -> bool {
    let a4033 = v0;
    let a4034 = 1u64;
    if nat_le(a4033, a4034) {
        true
    } else {
        let a4038 = {
            let a4035 = v1;
            let a4036 = v2;
            let a4037 = 1u64;
            nat_rem(a4035, a4036, a4037)
        };
        let a4039 = 0u64;
        nat_eq(a4038, a4039)
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4040 = v1;
    let a4041 = 1u64;
    if if if nat_le(a4040, a4041) {
        true
    } else {
        let a4042 = v0;
        let a4043 = v2;
        nat_le(a4042, a4043)
    } {
        let a4044 = v3;
        let a4045 = 1u64;
        if nat_le(a4044, a4045) {
            true
        } else {
            let a4052 = {
                let a4050 = v0;
                let a4051 = {
                    let a4048 = {
                        let a4046 = v1;
                        let a4047 = 1u64;
                        nat_sub(a4046, a4047)
                    };
                    let a4049 = v2;
                    nat_mul(a4048, a4049)?
                };
                nat_add(a4050, a4051)?
            };
            let a4053 = v4;
            nat_le(a4052, a4053)
        }
    } else {
        false
    } {
        let a4054 = v5;
        let a4055 = 1u64;
        if nat_le(a4054, a4055) {
            Ok(true)
        } else {
            let a4068 = {
                let a4066 = {
                    let a4060 = v0;
                    let a4061 = {
                        let a4058 = {
                            let a4056 = v1;
                            let a4057 = 1u64;
                            nat_sub(a4056, a4057)
                        };
                        let a4059 = v2;
                        nat_mul(a4058, a4059)?
                    };
                    nat_add(a4060, a4061)?
                };
                let a4067 = {
                    let a4064 = {
                        let a4062 = v3;
                        let a4063 = 1u64;
                        nat_sub(a4062, a4063)
                    };
                    let a4065 = v4;
                    nat_mul(a4064, a4065)?
                };
                nat_add(a4066, a4067)?
            };
            let a4069 = v6;
            Ok(nat_le(a4068, a4069))
        }
    } else {
        Ok(false)
    }
}

pub fn f181(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f221(v0, v1, v6, v7, v8) {
        let a4070 = v5;
        let a4071 = 26u64;
        nat_eq(a4070, a4071)
    } else {
        false
    } {
        let a4072 = v3;
        let a4073 = v7;
        nat_eq(a4072, a4073)
    } else {
        false
    } {
        let a4074 = v4;
        let a4075 = v8;
        nat_eq(a4074, a4075)
    } else {
        false
    } {
        let a4076 = v9;
        let a4077 = 1u64;
        nat_eq(a4076, a4077)
    } else {
        false
    } {
        let a4078 = v10;
        let a4079 = f222(v0);
        nat_eq(a4078, a4079)
    } else {
        false
    } {
        let a4080 = v11;
        let a4081 = v1;
        nat_eq(a4080, a4081)
    } else {
        false
    } {
        let a4082 = v12;
        let a4083 = v6;
        nat_eq(a4082, a4083)
    } else {
        false
    } {
        let a4084 = v13;
        let a4085 = v7;
        nat_eq(a4084, a4085)
    } else {
        false
    } {
        let a4086 = v14;
        let a4087 = v8;
        nat_eq(a4086, a4087)
    } else {
        false
    }
}

pub fn f182(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4088 = v0;
    let a4089 = 0u64;
    if if if nat_eq(a4088, a4089) {
        true
    } else {
        let a4090 = v1;
        let a4091 = 0u64;
        nat_eq(a4090, a4091)
    } {
        true
    } else {
        let a4092 = v2;
        let a4093 = 0u64;
        nat_eq(a4092, a4093)
    } {
        true
    } else {
        let a4094 = v3;
        let a4095 = 0u64;
        nat_eq(a4094, a4095)
    }
}

pub fn f183(v0: u64) -> bool {
    let a4096 = v0;
    let a4097 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4096, a4097) {
        true
    } else {
        let a4098 = v0;
        let a4099 = 1u64;
        nat_eq(a4098, a4099)
    } {
        true
    } else {
        let a4100 = v0;
        let a4101 = 2u64;
        nat_eq(a4100, a4101)
    } {
        true
    } else {
        let a4102 = v0;
        let a4103 = 3u64;
        nat_eq(a4102, a4103)
    } {
        true
    } else {
        let a4104 = v0;
        let a4105 = 6u64;
        nat_eq(a4104, a4105)
    } {
        true
    } else {
        let a4106 = v0;
        let a4107 = 7u64;
        nat_eq(a4106, a4107)
    } {
        true
    } else {
        let a4108 = v0;
        let a4109 = 8u64;
        nat_eq(a4108, a4109)
    } {
        true
    } else {
        let a4110 = v0;
        let a4111 = 10u64;
        nat_eq(a4110, a4111)
    } {
        true
    } else {
        let a4112 = v0;
        let a4113 = 11u64;
        nat_eq(a4112, a4113)
    } {
        true
    } else {
        let a4114 = v0;
        let a4115 = 12u64;
        nat_eq(a4114, a4115)
    } {
        true
    } else {
        let a4116 = v0;
        let a4117 = 13u64;
        nat_eq(a4116, a4117)
    } {
        true
    } else {
        let a4118 = v0;
        let a4119 = 14u64;
        nat_eq(a4118, a4119)
    } {
        true
    } else {
        let a4120 = v0;
        let a4121 = 16u64;
        nat_eq(a4120, a4121)
    } {
        true
    } else {
        let a4122 = v0;
        let a4123 = 17u64;
        nat_eq(a4122, a4123)
    } {
        true
    } else {
        let a4124 = v0;
        let a4125 = 18u64;
        nat_eq(a4124, a4125)
    } {
        true
    } else {
        let a4126 = v0;
        let a4127 = 19u64;
        nat_eq(a4126, a4127)
    } {
        true
    } else {
        let a4128 = v0;
        let a4129 = 20u64;
        nat_eq(a4128, a4129)
    } {
        true
    } else {
        let a4130 = v0;
        let a4131 = 21u64;
        nat_eq(a4130, a4131)
    } {
        true
    } else {
        let a4132 = v0;
        let a4133 = 22u64;
        nat_eq(a4132, a4133)
    } {
        true
    } else {
        let a4134 = v0;
        let a4135 = 23u64;
        nat_eq(a4134, a4135)
    } {
        true
    } else {
        let a4136 = v0;
        let a4137 = 26u64;
        nat_eq(a4136, a4137)
    } {
        true
    } else {
        let a4138 = v0;
        let a4139 = 29u64;
        nat_eq(a4138, a4139)
    } {
        true
    } else {
        let a4140 = v0;
        let a4141 = 30u64;
        nat_eq(a4140, a4141)
    } {
        true
    } else {
        let a4142 = v0;
        let a4143 = 34u64;
        nat_eq(a4142, a4143)
    } {
        true
    } else {
        let a4144 = v0;
        let a4145 = 35u64;
        nat_eq(a4144, a4145)
    } {
        true
    } else {
        let a4146 = v0;
        let a4147 = 39u64;
        nat_eq(a4146, a4147)
    } {
        true
    } else {
        let a4148 = v0;
        let a4149 = 40u64;
        nat_eq(a4148, a4149)
    } {
        true
    } else {
        let a4150 = v0;
        let a4151 = 41u64;
        nat_eq(a4150, a4151)
    } {
        true
    } else {
        let a4152 = v0;
        let a4153 = 42u64;
        nat_eq(a4152, a4153)
    } {
        true
    } else {
        let a4154 = v0;
        let a4155 = 43u64;
        nat_eq(a4154, a4155)
    }
}

pub fn f184(v0: u64) -> u64 {
    let m4156 = f108(v0);
    match m4156 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4157, _) => {
                    h4157
                }
            }
        }
    }
}

pub fn f185(v0: Adt3, v1: bool) -> R<u64> {
    f223(v0.clone(), {
        let a4159 = match v0.clone() {
            Adt3::C0(h4158, _, _, _, _, _, _, _) => {
                h4158
            }
        };
        let a4160 = 1u64;
        nat_sub(a4159, a4160)
    }, {
        let a4162 = match v0.clone() {
            Adt3::C0(_, h4161, _, _, _, _, _, _) => {
                h4161
            }
        };
        let a4163 = 1u64;
        nat_sub(a4162, a4163)
    }, {
        let a4165 = match v0.clone() {
            Adt3::C0(_, _, h4164, _, _, _, _, _) => {
                h4164
            }
        };
        let a4166 = 1u64;
        nat_sub(a4165, a4166)
    }, {
        let a4168 = match v0 {
            Adt3::C0(_, _, _, h4167, _, _, _, _) => {
                h4167
            }
        };
        let a4169 = 1u64;
        nat_sub(a4168, a4169)
    }, v1)
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4170 = v0;
    let a4171 = 0u64;
    if nat_eq(a4170, a4171) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4172 = v0;
            let a4173 = 1u64;
            nat_sub(a4172, a4173)
        };
        let v5: u64 = {
            let a4174 = v4;
            let a4175 = v1;
            let a4176 = 0u64;
            nat_quot(a4174, a4175, a4176)
        };
        let v6: u64 = {
            let a4184 = {
                let a4177 = v5;
                let a4178 = v2;
                nat_mul(a4177, a4178)?
            };
            let a4185 = {
                let a4182 = {
                    let a4179 = v4;
                    let a4180 = v1;
                    let a4181 = 0u64;
                    nat_rem(a4179, a4180, a4181)
                };
                let a4183 = v3;
                nat_mul(a4182, a4183)?
            };
            nat_add(a4184, a4185)?
        };
        let a4186 = v5;
        let a4187 = 0u64;
        if nat_eq(a4186, a4187) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4196 = {
                    let a4190 = {
                        let a4188 = v5;
                        let a4189 = 1u64;
                        nat_sub(a4188, a4189)
                    };
                    let a4191 = v2;
                    nat_mul(a4190, a4191)?
                };
                let a4197 = {
                    let a4194 = {
                        let a4192 = v1;
                        let a4193 = 1u64;
                        nat_sub(a4192, a4193)
                    };
                    let a4195 = v3;
                    nat_mul(a4194, a4195)?
                };
                nat_add(a4196, a4197)?
            };
            let a4198 = v6;
            let a4199 = v7;
            if nat_lt(a4198, a4199) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f187(v0: Adt3, v1: u64) -> bool {
    let a4200 = v1;
    let a4201 = 0u64;
    if !nat_eq(a4200, a4201) {
        true
    } else {
        let a4203 = match v0 {
            Adt3::C0(_, _, _, _, h4202, _, _, _) => {
                h4202
            }
        };
        let a4204 = 4u64;
        nat_eq(a4203, a4204)
    }
}

pub fn f188(v0: u64, v1: bool) -> R<u64> {
    let a4207 = {
        let a4205 = v0;
        let a4206 = 2u64;
        nat_mul(a4205, a4206)?
    };
    let a4208 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4207, a4208)
}

pub fn f189(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4209 = f224(v0, v1);
    match m4209 {
        None => {
            if v5 {
                let m4210 = f225(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4210 {
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
                let a4211 = 4u64;
                let a4212 = v28;
                nat_le(a4211, a4212)
            };
            let a4213 = v2;
            let a4214 = 0u64;
            if if if if if nat_eq(a4213, a4214) {
                let a4215 = v4;
                let a4216 = 0u64;
                nat_eq(a4215, a4216)
            } else {
                false
            } {
                let a4217 = v6;
                let a4218 = v14;
                if nat_eq(a4217, a4218) {
                    let a4219 = v7;
                    let a4220 = v15;
                    nat_eq(a4219, a4220)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4221 = v8;
                let a4222 = v16;
                if nat_eq(a4221, a4222) {
                    let a4223 = v9;
                    let a4224 = v17;
                    nat_eq(a4223, a4224)
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
                        let a4225 = v3;
                        let a4226 = 0u64;
                        nat_eq(a4225, a4226)
                    } else {
                        false
                    } {
                        f110(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4231 = {
                        let a4227 = v6;
                        let a4228 = v7;
                        nat_mul(a4227, a4228)?
                    };
                    let a4232 = {
                        let a4229 = v8;
                        let a4230 = v9;
                        nat_mul(a4229, a4230)?
                    };
                    nat_mul(a4231, a4232)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f190(v0: u64) -> u64 {
    let m4233 = f108(v0);
    match m4233 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4235 = match v1.clone() {
                Adt18::C0(_, h4234, _) => {
                    h4234
                }
            };
            let a4236 = 1u64;
            if if nat_eq(a4235, a4236) {
                let a4238 = match v1.clone() {
                    Adt18::C0(_, _, h4237) => {
                        h4237
                    }
                };
                let a4239 = 1u64;
                if if if nat_eq(a4238, a4239) {
                    true
                } else {
                    let a4241 = match v1.clone() {
                        Adt18::C0(_, _, h4240) => {
                            h4240
                        }
                    };
                    let a4242 = 2u64;
                    nat_eq(a4241, a4242)
                } {
                    true
                } else {
                    let a4244 = match v1.clone() {
                        Adt18::C0(_, _, h4243) => {
                            h4243
                        }
                    };
                    let a4245 = 4u64;
                    nat_eq(a4244, a4245)
                } {
                    true
                } else {
                    let a4247 = match v1.clone() {
                        Adt18::C0(_, _, h4246) => {
                            h4246
                        }
                    };
                    let a4248 = 8u64;
                    nat_eq(a4247, a4248)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4249) => {
                        h4249
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f191(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4254 = {
        let a4252 = {
            let a4250 = v0;
            let a4251 = v1;
            nat_sub(a4250, a4251)
        };
        let a4253 = 1u64;
        nat_sub(a4252, a4253)
    };
    let a4255 = v2;
    nat_add(a4254, a4255)
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f123({
        let a4256 = v0;
        let a4257 = v4;
        let a4258 = 0u64;
        nat_rem(a4256, a4257, a4258)
    }, {
        let a4259 = v1;
        let a4260 = v5;
        let a4261 = 0u64;
        nat_rem(a4259, a4260, a4261)
    }, {
        let a4262 = v2;
        let a4263 = v6;
        let a4264 = 0u64;
        nat_rem(a4262, a4263, a4264)
    }, {
        let a4265 = v3;
        let a4266 = v7;
        let a4267 = 0u64;
        nat_rem(a4265, a4266, a4267)
    }, v8, v9, v10, v11)
}

pub fn f193(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4268 = v1;
    let a4269 = 2147483648u64;
    if nat_lt(a4268, a4269) {
        let a4280 = {
            let a4278 = {
                let a4273 = {
                    let a4270 = v0;
                    let a4271 = v2;
                    let a4272 = 0u64;
                    nat_rem(a4270, a4271, a4272)
                };
                let a4274 = v2;
                nat_add(a4273, a4274)?
            };
            let a4279 = {
                let a4275 = v1;
                let a4276 = v2;
                let a4277 = 0u64;
                nat_rem(a4275, a4276, a4277)
            };
            nat_sub(a4278, a4279)
        };
        let a4281 = v2;
        let a4282 = 0u64;
        Ok(nat_rem(a4280, a4281, a4282))
    } else {
        let a4285 = {
            let a4283 = v0;
            let a4284 = f124(v1);
            nat_add(a4283, a4284)?
        };
        let a4286 = v2;
        let a4287 = 0u64;
        Ok(nat_rem(a4285, a4286, a4287))
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4290 = f125(v1);
    let a4291 = {
        let a4288 = v0;
        let a4289 = f124(v1);
        nat_add(a4288, a4289)?
    };
    if nat_le(a4290, a4291) {
        let a4292 = f195(v0, v1)?;
        let a4293 = v2;
        Ok(nat_lt(a4292, a4293))
    } else {
        Ok(false)
    }
}

pub fn f195(v0: u64, v1: u64) -> R<u64> {
    let a4296 = {
        let a4294 = v0;
        let a4295 = f124(v1);
        nat_add(a4294, a4295)?
    };
    let a4297 = f125(v1);
    Ok(nat_sub(a4296, a4297))
}

pub fn f196(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4298 = v0;
    let a4299 = v1;
    if nat_lt(a4298, a4299) {
        let a4300 = v1;
        let a4301 = v0;
        Ok(nat_sub(a4300, a4301))
    } else {
        let a4304 = v0;
        let a4305 = {
            let a4302 = v1;
            let a4303 = v2;
            nat_add(a4302, a4303)?
        };
        if nat_lt(a4304, a4305) {
            let a4306 = v0;
            let a4307 = v1;
            Ok(nat_sub(a4306, a4307))
        } else {
            let a4314 = {
                let a4310 = 2u64;
                let a4311 = {
                    let a4308 = v2;
                    let a4309 = 1u64;
                    nat_sub(a4308, a4309)
                };
                nat_mul(a4310, a4311)?
            };
            let a4315 = {
                let a4312 = v0;
                let a4313 = v1;
                nat_sub(a4312, a4313)
            };
            Ok(nat_sub(a4314, a4315))
        }
    }
}

pub fn f197(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4318 = {
        let a4316 = v2;
        let a4317 = f223(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1.clone(), 3u64), false)?;
        nat_add(a4316, a4317)?
    };
    let a4319 = f223(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4318, a4319))
}

pub fn f198(v0: u64, v1: u64, v2: u64) -> bool {
    let a4320 = v0;
    let a4321 = 0u64;
    if nat_eq(a4320, a4321) {
        let a4322 = v2;
        let a4323 = v1;
        nat_le(a4322, a4323)
    } else {
        let a4324 = v0;
        let a4325 = 1u64;
        if nat_eq(a4324, a4325) {
            let a4326 = v2;
            let a4327 = v1;
            nat_lt(a4326, a4327)
        } else {
            let a4328 = v0;
            let a4329 = 2u64;
            if nat_eq(a4328, a4329) {
                let a4330 = v1;
                let a4331 = v2;
                nat_le(a4330, a4331)
            } else {
                let a4332 = v0;
                let a4333 = 3u64;
                if nat_eq(a4332, a4333) {
                    let a4334 = v1;
                    let a4335 = v2;
                    nat_lt(a4334, a4335)
                } else {
                    let a4336 = v1;
                    let a4337 = v2;
                    nat_eq(a4336, a4337)
                }
            }
        }
    }
}

pub fn f199(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4338 = v0;
    let a4339 = 0u64;
    if nat_eq(a4338, a4339) {
        Ok(f210(0u64))
    } else {
        let a4342 = {
            let a4340 = v0;
            let a4341 = 1u64;
            nat_add(a4340, a4341)?
        };
        let a4343 = f121(v1.clone(), 13u64, v2)?;
        if nat_eq(a4342, a4343) {
            Ok(Adt9::C4({
                let a4344 = v2;
                let a4345 = 0u64;
                if nat_eq(a4344, a4345) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4346 = v0;
                let a4347 = 1u64;
                nat_sub(a4346, a4347)
            };
            if f187(v1.clone(), v2) {
                let v4: u64 = f188(match v1.clone() {
                    Adt3::C0(h4348, _, _, _, _, _, _, _) => {
                        h4348
                    }
                }, false)?;
                let v5: u64 = {
                    let a4349 = v3;
                    let a4350 = v4;
                    let a4351 = 0u64;
                    nat_quot(a4349, a4350, a4351)
                };
                let v6: u64 = {
                    let a4352 = v3;
                    let a4353 = v4;
                    let a4354 = 0u64;
                    nat_rem(a4352, a4353, a4354)
                };
                let a4357 = {
                    let a4355 = v6;
                    let a4356 = 1u64;
                    nat_add(a4355, a4356)?
                };
                let a4358 = v4;
                if nat_eq(a4357, a4358) {
                    Ok(Adt9::C4({
                        let a4359 = v2;
                        let a4360 = 0u64;
                        if nat_eq(a4359, a4360) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f200(v6, v5, v1, v2)
                }
            } else {
                f226({
                    let a4361 = v3;
                    let a4362 = 2u64;
                    let a4363 = 0u64;
                    nat_quot(a4361, a4362, a4363)
                }, {
                    let a4364 = v3;
                    let a4365 = 2u64;
                    let a4366 = 0u64;
                    nat_rem(a4364, a4365, a4366)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f200(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4367 = v0;
    let a4368 = 0u64;
    if nat_eq(a4367, a4368) {
        Ok(f210(1u64))
    } else {
        f226({
            let a4377 = {
                let a4370 = v1;
                let a4371 = match v2.clone() {
                    Adt3::C0(h4369, _, _, _, _, _, _, _) => {
                        h4369
                    }
                };
                nat_mul(a4370, a4371)?
            };
            let a4378 = {
                let a4374 = {
                    let a4372 = v0;
                    let a4373 = 1u64;
                    nat_sub(a4372, a4373)
                };
                let a4375 = 2u64;
                let a4376 = 0u64;
                nat_quot(a4374, a4375, a4376)
            };
            nat_add(a4377, a4378)?
        }, {
            let a4381 = {
                let a4379 = v0;
                let a4380 = 1u64;
                nat_sub(a4379, a4380)
            };
            let a4382 = 2u64;
            let a4383 = 0u64;
            nat_rem(a4381, a4382, a4383)
        }, v2, v3, 1u64)
    }
}

pub fn f201(v0: u64, v1: u64) -> R<u64> {
    let a4384 = v0;
    let a4385 = f88(v1)?;
    let a4386 = 0u64;
    Ok(nat_rem(a4384, a4385, a4386))
}

pub fn f202(v0: u64, v1: u64) -> R<u64> {
    let a4387 = 4u64;
    let a4388 = f227(v0, v1)?;
    nat_mul(a4387, a4388)
}

pub fn f203(v0: u64, v1: u64) -> R<u64> {
    let a4389 = 4u64;
    let a4390 = f228(v0, v1)?;
    nat_mul(a4389, a4390)
}

pub fn f204(v0: u64, v1: u64) -> R<bool> {
    let a4396 = {
        let a4393 = {
            let a4391 = f201(v0, v1)?;
            let a4392 = 2u64;
            nat_sub(a4391, a4392)
        };
        let a4394 = 2u64;
        let a4395 = 0u64;
        nat_rem(a4393, a4394, a4395)
    };
    let a4397 = 0u64;
    Ok(nat_eq(a4396, a4397))
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4398 = f108(v0);
    match m4398 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f183(v0) {
                let a4399 = v1;
                let a4400 = v7;
                nat_lt(a4399, a4400)
            } else {
                false
            } {
                f151(v8, v2)
            } else {
                false
            } {
                let a4401 = v3;
                let a4402 = f229(v4, v5, v6)?;
                nat_lt(a4401, a4402)
            } else {
                false
            } {
                let a4407 = {
                    let a4404 = v7;
                    let a4405 = match v13.clone() {
                        Adt18::C0(_, h4403, _) => {
                            h4403
                        }
                    };
                    let a4406 = 0u64;
                    nat_rem(a4404, a4405, a4406)
                };
                let a4408 = 0u64;
                nat_eq(a4407, a4408)
            } else {
                false
            } {
                Ok(Some({
                    let a4437 = {
                        let a4427 = {
                            let a4417 = {
                                let a4413 = {
                                    let a4410 = v1;
                                    let a4411 = match v13 {
                                        Adt18::C0(_, h4409, _) => {
                                            h4409
                                        }
                                    };
                                    let a4412 = 0u64;
                                    nat_quot(a4410, a4411, a4412)
                                };
                                let a4414 = v9;
                                nat_mul(a4413, a4414)?
                            };
                            let a4418 = {
                                let a4415 = v2;
                                let a4416 = v10;
                                nat_mul(a4415, a4416)?
                            };
                            nat_add(a4417, a4418)?
                        };
                        let a4428 = {
                            let a4425 = {
                                let a4422 = {
                                    let a4419 = v3;
                                    let a4420 = v4;
                                    let a4421 = 0u64;
                                    nat_quot(a4419, a4420, a4421)
                                };
                                let a4423 = v5;
                                let a4424 = 0u64;
                                nat_rem(a4422, a4423, a4424)
                            };
                            let a4426 = v11;
                            nat_mul(a4425, a4426)?
                        };
                        nat_add(a4427, a4428)?
                    };
                    let a4438 = {
                        let a4435 = {
                            let a4432 = {
                                let a4429 = v3;
                                let a4430 = v4;
                                let a4431 = 0u64;
                                nat_quot(a4429, a4430, a4431)
                            };
                            let a4433 = v5;
                            let a4434 = 0u64;
                            nat_quot(a4432, a4433, a4434)
                        };
                        let a4436 = v12;
                        nat_mul(a4435, a4436)?
                    };
                    nat_add(a4437, a4438)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f206(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f230(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f207(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f231(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f208(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4439 = v0;
    let a4440 = 64u64;
    if nat_eq(a4439, a4440) {
        f232(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4441 = v0;
        let a4442 = 65u64;
        if nat_eq(a4441, a4442) {
            f233(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f209(v0: u64) -> u64 {
    let a4443 = v0;
    let a4444 = 30u64;
    if nat_eq(a4443, a4444) {
        4u64
    } else {
        3u64
    }
}

pub fn f210(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f211(v0: u64) -> bool {
    let a4445 = 0u64;
    let a4446 = v0;
    if nat_lt(a4445, a4446) {
        let a4447 = v0;
        let a4448 = 2139095040u64;
        nat_le(a4447, a4448)
    } else {
        false
    }
}

pub fn f212(v0: u64) -> u64 {
    let a4449 = v0;
    let a4450 = 40u64;
    if nat_lt(a4449, a4450) {
        let a4451 = v0;
        let a4452 = 20u64;
        if nat_lt(a4451, a4452) {
            let a4453 = v0;
            let a4454 = 10u64;
            if nat_lt(a4453, a4454) {
                let a4455 = v0;
                let a4456 = 5u64;
                if nat_lt(a4455, a4456) {
                    let a4457 = v0;
                    let a4458 = 2u64;
                    if nat_lt(a4457, a4458) {
                        let a4459 = v0;
                        let a4460 = 1u64;
                        if nat_lt(a4459, a4460) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4461 = v0;
                        let a4462 = 3u64;
                        if nat_lt(a4461, a4462) {
                            4u64
                        } else {
                            let a4463 = v0;
                            let a4464 = 4u64;
                            if nat_lt(a4463, a4464) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4465 = v0;
                    let a4466 = 7u64;
                    if nat_lt(a4465, a4466) {
                        let a4467 = v0;
                        let a4468 = 6u64;
                        if nat_lt(a4467, a4468) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4469 = v0;
                        let a4470 = 8u64;
                        if nat_lt(a4469, a4470) {
                            128u64
                        } else {
                            let a4471 = v0;
                            let a4472 = 9u64;
                            if nat_lt(a4471, a4472) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4473 = v0;
                let a4474 = 15u64;
                if nat_lt(a4473, a4474) {
                    let a4475 = v0;
                    let a4476 = 12u64;
                    if nat_lt(a4475, a4476) {
                        let a4477 = v0;
                        let a4478 = 11u64;
                        if nat_lt(a4477, a4478) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4479 = v0;
                        let a4480 = 13u64;
                        if nat_lt(a4479, a4480) {
                            4096u64
                        } else {
                            let a4481 = v0;
                            let a4482 = 14u64;
                            if nat_lt(a4481, a4482) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4483 = v0;
                    let a4484 = 17u64;
                    if nat_lt(a4483, a4484) {
                        let a4485 = v0;
                        let a4486 = 16u64;
                        if nat_lt(a4485, a4486) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4487 = v0;
                        let a4488 = 18u64;
                        if nat_lt(a4487, a4488) {
                            131072u64
                        } else {
                            let a4489 = v0;
                            let a4490 = 19u64;
                            if nat_lt(a4489, a4490) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4491 = v0;
            let a4492 = 30u64;
            if nat_lt(a4491, a4492) {
                let a4493 = v0;
                let a4494 = 25u64;
                if nat_lt(a4493, a4494) {
                    let a4495 = v0;
                    let a4496 = 22u64;
                    if nat_lt(a4495, a4496) {
                        let a4497 = v0;
                        let a4498 = 21u64;
                        if nat_lt(a4497, a4498) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4499 = v0;
                        let a4500 = 23u64;
                        if nat_lt(a4499, a4500) {
                            4194304u64
                        } else {
                            let a4501 = v0;
                            let a4502 = 24u64;
                            if nat_lt(a4501, a4502) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4503 = v0;
                    let a4504 = 27u64;
                    if nat_lt(a4503, a4504) {
                        let a4505 = v0;
                        let a4506 = 26u64;
                        if nat_lt(a4505, a4506) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4507 = v0;
                        let a4508 = 28u64;
                        if nat_lt(a4507, a4508) {
                            134217728u64
                        } else {
                            let a4509 = v0;
                            let a4510 = 29u64;
                            if nat_lt(a4509, a4510) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4511 = v0;
                let a4512 = 35u64;
                if nat_lt(a4511, a4512) {
                    let a4513 = v0;
                    let a4514 = 32u64;
                    if nat_lt(a4513, a4514) {
                        let a4515 = v0;
                        let a4516 = 31u64;
                        if nat_lt(a4515, a4516) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4517 = v0;
                        let a4518 = 33u64;
                        if nat_lt(a4517, a4518) {
                            4294967296u64
                        } else {
                            let a4519 = v0;
                            let a4520 = 34u64;
                            if nat_lt(a4519, a4520) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4521 = v0;
                    let a4522 = 37u64;
                    if nat_lt(a4521, a4522) {
                        let a4523 = v0;
                        let a4524 = 36u64;
                        if nat_lt(a4523, a4524) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4525 = v0;
                        let a4526 = 38u64;
                        if nat_lt(a4525, a4526) {
                            137438953472u64
                        } else {
                            let a4527 = v0;
                            let a4528 = 39u64;
                            if nat_lt(a4527, a4528) {
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

pub fn f213(v0: u64, v1: u64) -> bool {
    let a4529 = v0;
    let a4530 = 1u64;
    if nat_le(a4529, a4530) {
        true
    } else {
        let a4534 = {
            let a4531 = v1;
            let a4532 = 4u64;
            let a4533 = 0u64;
            nat_rem(a4531, a4532, a4533)
        };
        let a4535 = 0u64;
        nat_eq(a4534, a4535)
    }
}

pub fn f214(v0: u64, v1: u64) -> bool {
    let a4536 = v0;
    let a4537 = 1u64;
    if nat_le(a4536, a4537) {
        true
    } else {
        let a4538 = 4u64;
        let a4539 = v1;
        nat_le(a4538, a4539)
    }
}

pub fn f215(v0: u64, v1: u64) -> R<u64> {
    let a4540 = v0;
    let a4541 = 0u64;
    if nat_eq(a4540, a4541) {
        Ok(0u64)
    } else {
        let a4546 = 4u64;
        let a4547 = {
            let a4544 = {
                let a4542 = v0;
                let a4543 = 1u64;
                nat_sub(a4542, a4543)
            };
            let a4545 = v1;
            nat_mul(a4544, a4545)?
        };
        nat_add(a4546, a4547)
    }
}

pub fn f216(v0: bool, v1: u64) -> R<u64> {
    let a4552 = {
        let a4550 = {
            let a4548 = f234(v0, v1)?;
            let a4549 = f235(v0, v1)?;
            nat_add(a4548, a4549)?
        };
        let a4551 = f236(v0);
        nat_add(a4550, a4551)?
    };
    let a4553 = v1;
    nat_add(a4552, a4553)
}

pub fn f217(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f237(v0, v1)?;
    let v3: u64 = {
        let a4558 = {
            let a4556 = {
                let a4554 = v1;
                let a4555 = 1u64;
                nat_sub(a4554, a4555)
            };
            let a4557 = v2;
            nat_mul(a4556, a4557)?
        };
        let a4559 = v0;
        nat_sub(a4558, a4559)
    };
    let a4560 = v2;
    let a4561 = v3;
    if nat_lt(a4560, a4561) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f218(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4562 = v4;
    let a4563 = 0u64;
    if nat_eq(a4562, a4563) {
        Ok(0u64)
    } else {
        let a4570 = 2u64;
        let a4571 = {
            let a4568 = {
                let a4564 = v0;
                let a4565 = v1;
                nat_mul(a4564, a4565)?
            };
            let a4569 = {
                let a4566 = v2;
                let a4567 = v3;
                nat_mul(a4566, a4567)?
            };
            nat_mul(a4568, a4569)?
        };
        nat_mul(a4570, a4571)
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4576 = v3;
    let a4577 = {
        let a4574 = {
            let a4572 = f238(v2, v4)?;
            let a4573 = f239(v0, v1, f237(v2, v4)?)?;
            nat_mul(a4572, a4573)?
        };
        let a4575 = f239(v0, v1, f240(v2, v4)?)?;
        nat_add(a4574, a4575)?
    };
    nat_mul(a4576, a4577)
}

pub fn f220(v0: u64) -> R<u64> {
    let a4580 = 13u64;
    let a4581 = {
        let a4578 = 9u64;
        let a4579 = v0;
        nat_mul(a4578, a4579)?
    };
    nat_add(a4580, a4581)
}

pub fn f221(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f182(v1, v2, v3, v4) {
        f241(v0)
    } else {
        f183(v0)
    }
}

pub fn f222(v0: u64) -> u64 {
    let a4582 = v0;
    let a4583 = 26u64;
    if nat_eq(a4582, a4583) {
        26u64
    } else {
        0u64
    }
}

pub fn f223(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4604 = {
        let a4598 = {
            let a4592 = {
                let a4586 = v1;
                let a4587 = if v5 {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4584, _, _, _) => {
                            h4584
                        }
                    })
                } else {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4585, _, _, _) => {
                            h4585
                        }
                    })
                };
                nat_mul(a4586, a4587)?
            };
            let a4593 = {
                let a4590 = v2;
                let a4591 = if v5 {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4588, _, _) => {
                            h4588
                        }
                    })
                } else {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4589, _, _) => {
                            h4589
                        }
                    })
                };
                nat_mul(a4590, a4591)?
            };
            nat_add(a4592, a4593)?
        };
        let a4599 = {
            let a4596 = v3;
            let a4597 = if v5 {
                f124(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4594, _) => {
                        h4594
                    }
                })
            } else {
                f125(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4595, _) => {
                        h4595
                    }
                })
            };
            nat_mul(a4596, a4597)?
        };
        nat_add(a4598, a4599)?
    };
    let a4605 = {
        let a4602 = v4;
        let a4603 = if v5 {
            f124(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4600) => {
                    h4600
                }
            })
        } else {
            f125(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4601) => {
                    h4601
                }
            })
        };
        nat_mul(a4602, a4603)?
    };
    nat_add(a4604, a4605)
}

pub fn f224(v0: u64, v1: u64) -> Option<u64> {
    let a4606 = v0;
    let a4607 = 2u64;
    if nat_eq(a4606, a4607) {
        Some(0u64)
    } else {
        let a4608 = v0;
        let a4609 = 5u64;
        if nat_eq(a4608, a4609) {
            Some(1u64)
        } else {
            let a4610 = v0;
            let a4611 = 6u64;
            if nat_eq(a4610, a4611) {
                Some(2u64)
            } else {
                let a4612 = v0;
                let a4613 = 7u64;
                if nat_eq(a4612, a4613) {
                    Some(3u64)
                } else {
                    let a4614 = v0;
                    let a4615 = 9u64;
                    if nat_eq(a4614, a4615) {
                        Some(4u64)
                    } else {
                        let a4616 = v0;
                        let a4617 = 8u64;
                        if nat_eq(a4616, a4617) {
                            Some(5u64)
                        } else {
                            let a4618 = v0;
                            let a4619 = 89u64;
                            if nat_eq(a4618, a4619) {
                                let a4620 = v1;
                                let a4621 = 2u64;
                                if nat_eq(a4620, a4621) {
                                    Some(6u64)
                                } else {
                                    let a4622 = v1;
                                    let a4623 = 0u64;
                                    if nat_eq(a4622, a4623) {
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

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4624 = v0;
    let a4625 = 30u64;
    if nat_eq(a4624, a4625) {
        let a4626 = v2;
        let a4627 = 0u64;
        if if if nat_eq(a4626, a4627) {
            true
        } else {
            let a4628 = v2;
            let a4629 = 1u64;
            nat_eq(a4628, a4629)
        } {
            let a4630 = v1;
            let a4631 = 0u64;
            if nat_eq(a4630, a4631) {
                true
            } else {
                f242(v1, v4)
            }
        } else {
            false
        } {
            f243(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f244(v1) {
            f244(v2)
        } else {
            false
        } {
            let a4632 = v1;
            let a4633 = 0u64;
            if if nat_eq(a4632, a4633) {
                true
            } else {
                let a4634 = v2;
                let a4635 = 0u64;
                nat_eq(a4634, a4635)
            } {
                true
            } else {
                let a4636 = v1;
                let a4637 = v2;
                nat_eq(a4636, a4637)
            }
        } else {
            false
        } {
            true
        } else {
            let a4638 = v1;
            let a4639 = 43u64;
            if nat_eq(a4638, a4639) {
                f244(v2)
            } else {
                false
            }
        } {
            f243(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f226(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4640 = v1;
    let a4641 = 0u64;
    if nat_eq(a4640, a4641) {
        Ok(Adt9::C4({
            let a4642 = v3;
            let a4643 = 0u64;
            if nat_eq(a4642, a4643) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f131(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4644 = v3;
            let a4645 = 0u64;
            if nat_eq(a4644, a4645) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f227(v0: u64, v1: u64) -> R<u64> {
    let a4646 = v0;
    let a4647 = f88(v1)?;
    let a4648 = 0u64;
    Ok(nat_quot(a4646, a4647, a4648))
}

pub fn f228(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f227(v0, v1)?;
    let v3: u64 = {
        let a4651 = {
            let a4649 = f201(v0, v1)?;
            let a4650 = 2u64;
            nat_sub(a4649, a4650)
        };
        let a4652 = 2u64;
        let a4653 = 0u64;
        nat_quot(a4651, a4652, a4653)
    };
    let v4: u64 = {
        let a4654 = v2;
        let a4655 = v1;
        nat_mul(a4654, a4655)?
    };
    if f204(v0, v1)? {
        let a4656 = v4;
        let a4657 = v3;
        nat_add(a4656, a4657)
    } else {
        let a4669 = {
            let a4665 = v4;
            let a4666 = {
                let a4663 = {
                    let a4658 = v1;
                    let a4659 = 1u64;
                    nat_sub(a4658, a4659)
                };
                let a4664 = {
                    let a4660 = v2;
                    let a4661 = v1;
                    let a4662 = 0u64;
                    nat_rem(a4660, a4661, a4662)
                };
                nat_mul(a4663, a4664)?
            };
            nat_sub(a4665, a4666)
        };
        let a4670 = {
            let a4667 = v3;
            let a4668 = v1;
            nat_mul(a4667, a4668)?
        };
        nat_add(a4669, a4670)
    }
}

pub fn f229(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4671 = v0;
    let a4672 = 0u64;
    if if if nat_eq(a4671, a4672) {
        true
    } else {
        let a4673 = v1;
        let a4674 = 0u64;
        nat_eq(a4673, a4674)
    } {
        true
    } else {
        let a4675 = v2;
        let a4676 = 0u64;
        nat_eq(a4675, a4676)
    } {
        Ok(0u64)
    } else {
        let a4679 = {
            let a4677 = v0;
            let a4678 = v1;
            nat_mul(a4677, a4678)?
        };
        let a4680 = v2;
        nat_mul(a4679, a4680)
    }
}

pub fn f230(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f97(match v1.clone() {
        Adt20::C0(h4681, _, _, _, _, _, _, _) => {
            h4681
        }
    })?;
    let v6: u64 = {
        let a4687 = match v1.clone() {
            Adt20::C0(_, h4682, _, _, _, _, _, _) => {
                h4682
            }
        };
        let a4688 = {
            let a4685 = match v1.clone() {
                Adt20::C0(_, _, h4683, _, _, _, _, _) => {
                    h4683
                }
            };
            let a4686 = match v1.clone() {
                Adt20::C0(_, _, _, h4684, _, _, _, _) => {
                    h4684
                }
            };
            nat_mul(a4685, a4686)?
        };
        nat_mul(a4687, a4688)?
    };
    let a4690 = v0;
    let a4691 = f36(match v1.clone() {
        Adt20::C0(h4689, _, _, _, _, _, _, _) => {
            h4689
        }
    }, v6)?;
    if nat_lt(a4690, a4691) {
        f245({
            let a4692 = v0;
            let a4693 = v5;
            let a4694 = 0u64;
            nat_rem(a4692, a4693, a4694)
        }, {
            let a4695 = v0;
            let a4696 = v5;
            let a4697 = 0u64;
            nat_quot(a4695, a4696, a4697)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f231(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a4698 = v6;
        let a4699 = 6u64;
        nat_sub(a4698, a4699)
    };
    let v9: Adt20 = f246(v3);
    let a4701 = v0;
    let a4702 = f178(match v1.clone() {
        Adt20::C0(h4700, _, _, _, _, _, _, _) => {
            h4700
        }
    }, v5, v8)?;
    if nat_lt(a4701, a4702) {
        let v10: u64 = f94(match v1.clone() {
            Adt20::C0(h4703, _, _, _, _, _, _, _) => {
                h4703
            }
        }, v5, v8)?;
        let a4704 = v0;
        let a4705 = v10;
        if nat_lt(a4704, a4705) {
            f247(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a4706 = v0;
                let a4707 = v10;
                nat_sub(a4706, a4707)
            };
            let v12: u64 = f220(match v1.clone() {
                Adt20::C0(h4708, _, _, _, _, _, _, _) => {
                    h4708
                }
            })?;
            f248({
                let a4709 = v11;
                let a4710 = v12;
                let a4711 = 0u64;
                nat_rem(a4709, a4710, a4711)
            }, {
                let a4712 = v11;
                let a4713 = v12;
                let a4714 = 0u64;
                nat_quot(a4712, a4713, a4714)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f232(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4715 = v5;
        let a4716 = 6u64;
        nat_sub(a4715, a4716)
    };
    let v8: bool = f172(match v1.clone() {
        Adt20::C0(_, _, _, _, h4717, _, _, _) => {
            h4717
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h4718, _, _, _) => {
            h4718
        }
    });
    let a4720 = v0;
    let a4721 = f171(v8, match v1.clone() {
        Adt20::C0(h4719, _, _, _, _, _, _, _) => {
            h4719
        }
    }, v4, v7)?;
    if nat_lt(a4720, a4721) {
        let v9: u64 = f94(match v1.clone() {
            Adt20::C0(h4722, _, _, _, _, _, _, _) => {
                h4722
            }
        }, v4, v7)?;
        let a4723 = v0;
        let a4724 = v9;
        if nat_lt(a4723, a4724) {
            f247(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a4725 = v0;
                let a4726 = v9;
                nat_sub(a4725, a4726)
            };
            let v11: u64 = f216(v8, match v1.clone() {
                Adt20::C0(h4727, _, _, _, _, _, _, _) => {
                    h4727
                }
            })?;
            f249({
                let a4728 = v10;
                let a4729 = v11;
                let a4730 = 0u64;
                nat_rem(a4728, a4729, a4730)
            }, {
                let a4731 = v10;
                let a4732 = v11;
                let a4733 = 0u64;
                nat_quot(a4731, a4732, a4733)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f233(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4734 = v5;
        let a4735 = 6u64;
        nat_sub(a4734, a4735)
    };
    let v8: Adt20 = f246(v2);
    let v9: u64 = f218(match v1.clone() {
        Adt20::C0(h4736, _, _, _, _, _, _, _) => {
            h4736
        }
    }, match v1.clone() {
        Adt20::C0(_, h4737, _, _, _, _, _, _) => {
            h4737
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4738, _, _, _, _, _) => {
            h4738
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4739, _, _, _, _) => {
            h4739
        }
    }, v7)?;
    let a4744 = v0;
    let a4745 = f177(match v1.clone() {
        Adt20::C0(h4740, _, _, _, _, _, _, _) => {
            h4740
        }
    }, match v1.clone() {
        Adt20::C0(_, h4741, _, _, _, _, _, _) => {
            h4741
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4742, _, _, _, _, _) => {
            h4742
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4743, _, _, _, _) => {
            h4743
        }
    }, v4, v7)?;
    if nat_lt(a4744, a4745) {
        let a4746 = v0;
        let a4747 = v9;
        if nat_lt(a4746, a4747) {
            f247(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f250({
                let a4748 = v0;
                let a4749 = v9;
                nat_sub(a4748, a4749)
            }, match v1.clone() {
                Adt20::C0(h4750, _, _, _, _, _, _, _) => {
                    h4750
                }
            }, match v1.clone() {
                Adt20::C0(_, h4751, _, _, _, _, _, _) => {
                    h4751
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h4752, _, _, _, _, _) => {
                    h4752
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h4753, _, _, _, _) => {
                    h4753
                }
            }, v4)?;
            f251(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f234(v0: bool, v1: u64) -> R<u64> {
    let a4756 = if v0 {
        5u64
    } else {
        4u64
    };
    let a4757 = {
        let a4754 = f252(v0);
        let a4755 = v1;
        nat_mul(a4754, a4755)?
    };
    nat_add(a4756, a4757)
}

pub fn f235(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4765 = {
            let a4758 = 13u64;
            let a4759 = f253(v1);
            nat_mul(a4758, a4759)?
        };
        let a4766 = {
            let a4763 = 4u64;
            let a4764 = {
                let a4760 = v1;
                let a4761 = 4u64;
                let a4762 = 0u64;
                nat_rem(a4760, a4761, a4762)
            };
            nat_mul(a4763, a4764)?
        };
        nat_add(a4765, a4766)
    } else {
        let a4767 = 3u64;
        let a4768 = v1;
        nat_mul(a4767, a4768)
    }
}

pub fn f236(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let a4773 = {
        let a4771 = {
            let a4769 = v0;
            let a4770 = v1;
            nat_add(a4769, a4770)?
        };
        let a4772 = 1u64;
        nat_sub(a4771, a4772)
    };
    let a4774 = v1;
    let a4775 = 0u64;
    Ok(nat_quot(a4773, a4774, a4775))
}

pub fn f238(v0: u64, v1: u64) -> R<u64> {
    let a4776 = v0;
    let a4777 = f237(v0, v1)?;
    let a4778 = 0u64;
    Ok(nat_quot(a4776, a4777, a4778))
}

pub fn f239(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4779 = v2;
    let a4780 = 0u64;
    if nat_eq(a4779, a4780) {
        Ok(0u64)
    } else {
        let a4785 = 11u64;
        let a4786 = {
            let a4783 = {
                let a4781 = v1;
                let a4782 = v2;
                nat_mul(a4781, a4782)?
            };
            let a4784 = f254(v0)?;
            nat_mul(a4783, a4784)?
        };
        nat_add(a4785, a4786)
    }
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a4787 = v0;
    let a4788 = f237(v0, v1)?;
    let a4789 = 0u64;
    Ok(nat_rem(a4787, a4788, a4789))
}

pub fn f241(v0: u64) -> bool {
    let m4790 = f108(v0);
    match m4790 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f242(v0: u64, v1: u64) -> bool {
    let a4791 = v0;
    let a4792 = 1u64;
    if nat_eq(a4791, a4792) {
        true
    } else {
        let v2: u64 = f255(v0);
        let a4793 = 0u64;
        let a4794 = v2;
        if nat_lt(a4793, a4794) {
            let a4798 = {
                let a4795 = v1;
                let a4796 = v2;
                let a4797 = 1u64;
                nat_rem(a4795, a4796, a4797)
            };
            let a4799 = 0u64;
            nat_eq(a4798, a4799)
        } else {
            false
        }
    }
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4800 = v0;
    let a4801 = 28u64;
    if if if if nat_eq(a4800, a4801) {
        let a4802 = v1;
        let a4803 = 0u64;
        nat_eq(a4802, a4803)
    } else {
        false
    } {
        let a4804 = v2;
        let a4805 = 0u64;
        nat_eq(a4804, a4805)
    } else {
        false
    } {
        let a4806 = v3;
        let a4807 = 0u64;
        nat_eq(a4806, a4807)
    } else {
        false
    } {
        let a4808 = v4;
        let a4809 = v8;
        if if if if if if if if nat_eq(a4808, a4809) {
            let a4810 = v5;
            let a4811 = v12;
            nat_eq(a4810, a4811)
        } else {
            false
        } {
            let a4812 = v9;
            let a4813 = v13;
            nat_eq(a4812, a4813)
        } else {
            false
        } {
            let a4814 = v10;
            let a4815 = v14;
            nat_eq(a4814, a4815)
        } else {
            false
        } {
            let a4816 = v11;
            let a4817 = v15;
            nat_eq(a4816, a4817)
        } else {
            false
        } {
            let a4818 = 0u64;
            let a4819 = v6;
            nat_lt(a4818, a4819)
        } else {
            false
        } {
            let a4820 = 0u64;
            let a4821 = v7;
            nat_lt(a4820, a4821)
        } else {
            false
        } {
            let a4825 = {
                let a4822 = v10;
                let a4823 = v6;
                let a4824 = 1u64;
                nat_rem(a4822, a4823, a4824)
            };
            let a4826 = 0u64;
            if nat_eq(a4825, a4826) {
                let a4830 = {
                    let a4827 = v11;
                    let a4828 = v7;
                    let a4829 = 1u64;
                    nat_rem(a4827, a4828, a4829)
                };
                let a4831 = 0u64;
                nat_eq(a4830, a4831)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a4832 = v10;
                let a4833 = v6;
                let a4834 = 0u64;
                nat_quot(a4832, a4833, a4834)
            }, {
                let a4835 = v11;
                let a4836 = v7;
                let a4837 = 0u64;
                nat_quot(a4835, a4836, a4837)
            }, {
                let a4838 = v5;
                let a4839 = v4;
                nat_mul(a4838, a4839)?
            }, {
                let a4840 = v4;
                let a4841 = v9;
                nat_mul(a4840, a4841)?
            }, {
                let a4842 = v5;
                let a4843 = v9;
                nat_mul(a4842, a4843)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f244(v0: u64) -> bool {
    let a4844 = v0;
    let a4845 = 0u64;
    if if nat_eq(a4844, a4845) {
        true
    } else {
        let a4846 = v0;
        let a4847 = 1u64;
        nat_eq(a4846, a4847)
    } {
        true
    } else {
        let a4848 = v0;
        let a4849 = 30u64;
        nat_eq(a4848, a4849)
    }
}

pub fn f245(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h4850, _, _, _, _, _, _, _) => {
            h4850
        }
    };
    let v7: u64 = {
        let a4853 = 1u64;
        let a4854 = {
            let a4851 = 3u64;
            let a4852 = v6;
            nat_mul(a4851, a4852)?
        };
        nat_add(a4853, a4854)?
    };
    let a4855 = v0;
    let a4856 = 0u64;
    if nat_eq(a4855, a4856) {
        Ok(f256(0u64))
    } else {
        let a4857 = v0;
        let a4858 = v7;
        if nat_lt(a4857, a4858) {
            let v8: u64 = {
                let a4859 = v0;
                let a4860 = 1u64;
                nat_sub(a4859, a4860)
            };
            let v9: u64 = {
                let a4866 = {
                    let a4861 = v1;
                    let a4862 = v6;
                    nat_mul(a4861, a4862)?
                };
                let a4867 = {
                    let a4863 = v8;
                    let a4864 = 3u64;
                    let a4865 = 0u64;
                    nat_quot(a4863, a4864, a4865)
                };
                nat_add(a4866, a4867)?
            };
            let v10: u64 = f257(v9, v2)?;
            let v11: u64 = {
                let a4868 = v8;
                let a4869 = 3u64;
                let a4870 = 0u64;
                nat_rem(a4868, a4869, a4870)
            };
            let a4871 = v11;
            let a4872 = 0u64;
            if nat_eq(a4871, a4872) {
                Ok(f258(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a4873 = v11;
                let a4874 = 1u64;
                if nat_eq(a4873, a4874) {
                    Ok(f258(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f258(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a4875 = v0;
            let a4876 = v7;
            if nat_eq(a4875, a4876) {
                Ok(f258(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4879 = v0;
                let a4880 = {
                    let a4877 = v7;
                    let a4878 = 1u64;
                    nat_add(a4877, a4878)?
                };
                if nat_eq(a4879, a4880) {
                    Ok(f258(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4883 = v0;
                    let a4884 = {
                        let a4881 = v7;
                        let a4882 = 2u64;
                        nat_add(a4881, a4882)?
                    };
                    if nat_eq(a4883, a4884) {
                        Ok(f258(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a4887 = v0;
                        let a4888 = {
                            let a4885 = v7;
                            let a4886 = 3u64;
                            nat_add(a4885, a4886)?
                        };
                        if nat_eq(a4887, a4888) {
                            let a4889 = v5;
                            let a4890 = 1u64;
                            if if nat_eq(a4889, a4890) {
                                true
                            } else {
                                let a4891 = v5;
                                let a4892 = 2u64;
                                nat_eq(a4891, a4892)
                            } {
                                Ok(f258(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f258(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a4899 = {
                                    let a4893 = v1;
                                    let a4894 = v6;
                                    nat_mul(a4893, a4894)?
                                };
                                let a4900 = {
                                    let a4897 = v0;
                                    let a4898 = {
                                        let a4895 = v7;
                                        let a4896 = 4u64;
                                        nat_add(a4895, a4896)?
                                    };
                                    nat_sub(a4897, a4898)
                                };
                                nat_add(a4899, a4900)?
                            };
                            Ok(f258(10u64, 0u64, f257(v12, v2)?, 1u64, 2u64, 3u64, f257(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f246(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h4901, _, _, _, _, _, _, _) => {
            h4901
        }
    }, match v0.clone() {
        Adt20::C0(_, h4902, _, _, _, _, _, _) => {
            h4902
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h4903, _, _, _, _, _) => {
            h4903
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h4904, _, _, _, _) => {
            h4904
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h4905, _, _) => {
            h4905
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h4906, _) => {
            h4906
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h4907) => {
            h4907
        }
    })
}

pub fn f247(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a4914 = v3;
        let a4915 = {
            let a4911 = f257({
                let a4908 = v0;
                let a4909 = 2u64;
                let a4910 = 0u64;
                nat_quot(a4908, a4909, a4910)
            }, v1)?;
            let a4912 = v2;
            let a4913 = 0u64;
            nat_quot(a4911, a4912, a4913)
        };
        nat_add(a4914, a4915)?
    };
    let a4919 = {
        let a4916 = v0;
        let a4917 = 2u64;
        let a4918 = 0u64;
        nat_rem(a4916, a4917, a4918)
    };
    let a4920 = 0u64;
    if nat_eq(a4919, a4920) {
        Ok(f258(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4921 = v4;
        let a4922 = 0u64;
        if nat_eq(a4921, a4922) {
            Ok(f258(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f248(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f259(match v2.clone() {
        Adt20::C0(h4923, _, _, _, _, _, _, _) => {
            h4923
        }
    })?;
    let v7: u64 = f260(match v2.clone() {
        Adt20::C0(h4924, _, _, _, _, _, _, _) => {
            h4924
        }
    })?;
    let a4925 = v0;
    let a4926 = 2u64;
    if nat_lt(a4925, a4926) {
        Ok(f256(v0))
    } else {
        let a4927 = v0;
        let a4928 = v6;
        if nat_lt(a4927, a4928) {
            f261({
                let a4929 = v0;
                let a4930 = 2u64;
                nat_sub(a4929, a4930)
            }, v1, v2, v3)
        } else {
            let a4931 = v0;
            let a4932 = v7;
            if nat_lt(a4931, a4932) {
                Ok(f262({
                    let a4933 = v0;
                    let a4934 = v6;
                    nat_sub(a4933, a4934)
                }, match v2 {
                    Adt20::C0(h4935, _, _, _, _, _, _, _) => {
                        h4935
                    }
                }, v5))
            } else {
                let a4937 = v0;
                let a4938 = f220(match v2.clone() {
                    Adt20::C0(h4936, _, _, _, _, _, _, _) => {
                        h4936
                    }
                })?;
                if nat_lt(a4937, a4938) {
                    f263({
                        let a4939 = v0;
                        let a4940 = v7;
                        nat_sub(a4939, a4940)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f249(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f234(v5, match v2.clone() {
        Adt20::C0(h4941, _, _, _, _, _, _, _) => {
            h4941
        }
    })?;
    let v7: u64 = {
        let a4943 = v6;
        let a4944 = f235(v5, match v2.clone() {
            Adt20::C0(h4942, _, _, _, _, _, _, _) => {
                h4942
            }
        })?;
        nat_add(a4943, a4944)?
    };
    let v8: u64 = {
        let a4945 = v7;
        let a4946 = f236(v5);
        nat_add(a4945, a4946)?
    };
    let a4947 = v0;
    let a4948 = v6;
    if nat_lt(a4947, a4948) {
        f264(v0, v1, v2, v5)
    } else {
        let a4949 = v0;
        let a4950 = v7;
        if nat_lt(a4949, a4950) {
            f265({
                let a4951 = v0;
                let a4952 = v6;
                nat_sub(a4951, a4952)
            }, v1, v2, v3, v5)
        } else {
            let a4953 = v0;
            let a4954 = v8;
            if nat_lt(a4953, a4954) {
                Ok(f266({
                    let a4955 = v0;
                    let a4956 = v7;
                    nat_sub(a4955, a4956)
                }, match v2 {
                    Adt20::C0(h4957, _, _, _, _, _, _, _) => {
                        h4957
                    }
                }, v4, v5))
            } else {
                let a4961 = v0;
                let a4962 = {
                    let a4959 = v8;
                    let a4960 = match v2.clone() {
                        Adt20::C0(h4958, _, _, _, _, _, _, _) => {
                            h4958
                        }
                    };
                    nat_add(a4959, a4960)?
                };
                if nat_lt(a4961, a4962) {
                    let v9: u64 = f257({
                        let a4968 = {
                            let a4964 = v1;
                            let a4965 = match v2 {
                                Adt20::C0(h4963, _, _, _, _, _, _, _) => {
                                    h4963
                                }
                            };
                            nat_mul(a4964, a4965)?
                        };
                        let a4969 = {
                            let a4966 = v0;
                            let a4967 = v8;
                            nat_sub(a4966, a4967)
                        };
                        nat_add(a4968, a4969)?
                    }, v3)?;
                    Ok(f258(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f250(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f237(v3, v5)?;
    let v7: u64 = f239(v1, v2, v6)?;
    let v8: u64 = {
        let a4972 = {
            let a4970 = f238(v3, v5)?;
            let a4971 = v4;
            nat_mul(a4970, a4971)?
        };
        let a4973 = v7;
        nat_mul(a4972, a4973)?
    };
    let a4974 = v0;
    let a4975 = v8;
    if nat_lt(a4974, a4975) {
        let v9: u64 = {
            let a4976 = v0;
            let a4977 = v7;
            let a4978 = 0u64;
            nat_quot(a4976, a4977, a4978)
        };
        Ok(Adt22::C0({
            let a4982 = {
                let a4979 = v9;
                let a4980 = v4;
                let a4981 = 0u64;
                nat_quot(a4979, a4980, a4981)
            };
            let a4983 = v6;
            nat_mul(a4982, a4983)?
        }, v6, {
            let a4984 = v9;
            let a4985 = v4;
            let a4986 = 0u64;
            nat_rem(a4984, a4985, a4986)
        }, {
            let a4987 = v0;
            let a4988 = v7;
            let a4989 = 0u64;
            nat_rem(a4987, a4988, a4989)
        }))
    } else {
        let v10: u64 = {
            let a4990 = v0;
            let a4991 = v8;
            nat_sub(a4990, a4991)
        };
        let v11: u64 = f239(v1, v2, f240(v3, v5)?)?;
        Ok(Adt22::C0({
            let a4992 = f238(v3, v5)?;
            let a4993 = v6;
            nat_mul(a4992, a4993)?
        }, f240(v3, v5)?, {
            let a4994 = v10;
            let a4995 = v11;
            let a4996 = 0u64;
            nat_quot(a4994, a4995, a4996)
        }, {
            let a4997 = v10;
            let a4998 = v11;
            let a4999 = 0u64;
            nat_rem(a4997, a4998, a4999)
        }))
    }
}

pub fn f251(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5000) => {
            h5000
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5001, _, _, _, _, _, _, _) => {
            h5001
        }
    };
    let v6: u64 = {
        let a5004 = match v0.clone() {
            Adt22::C0(_, h5002, _, _) => {
                h5002
            }
        };
        let a5005 = match v1.clone() {
            Adt20::C0(_, h5003, _, _, _, _, _, _) => {
                h5003
            }
        };
        nat_mul(a5004, a5005)?
    };
    let v7: u64 = {
        let a5006 = v6;
        let a5007 = v5;
        nat_mul(a5006, a5007)?
    };
    let v8: u64 = {
        let a5010 = {
            let a5008 = 2u64;
            let a5009 = v5;
            nat_mul(a5008, a5009)?
        };
        let a5011 = 2u64;
        nat_add(a5010, a5011)?
    };
    let v9: u64 = {
        let a5014 = {
            let a5012 = 4u64;
            let a5013 = v5;
            nat_mul(a5012, a5013)?
        };
        let a5015 = 2u64;
        nat_add(a5014, a5015)?
    };
    let v10: u64 = {
        let a5018 = 1u64;
        let a5019 = {
            let a5016 = v6;
            let a5017 = v8;
            nat_mul(a5016, a5017)?
        };
        nat_add(a5018, a5019)?
    };
    let v11: u64 = {
        let a5020 = v10;
        let a5021 = 4u64;
        nat_add(a5020, a5021)?
    };
    let v12: u64 = {
        let a5024 = v11;
        let a5025 = {
            let a5022 = v6;
            let a5023 = v9;
            nat_mul(a5022, a5023)?
        };
        nat_add(a5024, a5025)?
    };
    let v13: u64 = {
        let a5026 = v12;
        let a5027 = 6u64;
        nat_add(a5026, a5027)?
    };
    let a5028 = v4;
    let a5029 = 0u64;
    if nat_eq(a5028, a5029) {
        Ok(f256(0u64))
    } else {
        let a5030 = v4;
        let a5031 = v10;
        if nat_lt(a5030, a5031) {
            let v14: u64 = {
                let a5032 = v4;
                let a5033 = 1u64;
                nat_sub(a5032, a5033)
            };
            f267(false, {
                let a5034 = v14;
                let a5035 = v8;
                let a5036 = 0u64;
                nat_rem(a5034, a5035, a5036)
            }, {
                let a5037 = v14;
                let a5038 = v8;
                let a5039 = 0u64;
                nat_quot(a5037, a5038, a5039)
            }, v0, v1, v2)
        } else {
            let a5040 = v4;
            let a5041 = v11;
            if nat_lt(a5040, a5041) {
                Ok(f268({
                    let a5042 = v4;
                    let a5043 = v10;
                    nat_sub(a5042, a5043)
                }, v7))
            } else {
                let a5044 = v4;
                let a5045 = v12;
                if nat_lt(a5044, a5045) {
                    let v15: u64 = {
                        let a5046 = v4;
                        let a5047 = v11;
                        nat_sub(a5046, a5047)
                    };
                    f267(true, {
                        let a5048 = v15;
                        let a5049 = v9;
                        let a5050 = 0u64;
                        nat_rem(a5048, a5049, a5050)
                    }, {
                        let a5051 = v15;
                        let a5052 = v9;
                        let a5053 = 0u64;
                        nat_quot(a5051, a5052, a5053)
                    }, v0, v1, v2)
                } else {
                    let a5054 = v4;
                    let a5055 = v13;
                    if nat_lt(a5054, a5055) {
                        Ok(f269({
                            let a5056 = v4;
                            let a5057 = v12;
                            nat_sub(a5056, a5057)
                        }, v7, v3))
                    } else {
                        let a5060 = v4;
                        let a5061 = {
                            let a5058 = v13;
                            let a5059 = v7;
                            nat_add(a5058, a5059)?
                        };
                        if nat_lt(a5060, a5061) {
                            let v16: u64 = {
                                let a5062 = v4;
                                let a5063 = v13;
                                nat_sub(a5062, a5063)
                            };
                            let v17: u64 = {
                                let a5064 = v16;
                                let a5065 = v5;
                                let a5066 = 0u64;
                                nat_quot(a5064, a5065, a5066)
                            };
                            let v18: u64 = {
                                let a5072 = {
                                    let a5067 = f270(v17, v0, v1)?;
                                    let a5068 = v5;
                                    nat_mul(a5067, a5068)?
                                };
                                let a5073 = {
                                    let a5069 = v16;
                                    let a5070 = v5;
                                    let a5071 = 0u64;
                                    nat_rem(a5069, a5070, a5071)
                                };
                                nat_add(a5072, a5073)?
                            };
                            let v19: u64 = f257(v18, v2)?;
                            Ok(f258(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f252(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f253(v0: u64) -> u64 {
    let a5074 = v0;
    let a5075 = 4u64;
    let a5076 = 0u64;
    nat_quot(a5074, a5075, a5076)
}

pub fn f254(v0: u64) -> R<u64> {
    let a5079 = {
        let a5077 = 7u64;
        let a5078 = v0;
        nat_mul(a5077, a5078)?
    };
    let a5080 = 4u64;
    nat_add(a5079, a5080)
}

pub fn f255(v0: u64) -> u64 {
    if f183(v0) {
        let m5081 = f108(v0);
        match m5081 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5083 = 1u64;
                let a5084 = match v1.clone() {
                    Adt18::C0(_, h5082, _) => {
                        h5082
                    }
                };
                if if nat_lt(a5083, a5084) {
                    true
                } else {
                    let a5085 = v0;
                    let a5086 = 43u64;
                    nat_eq(a5085, a5086)
                } {
                    match v1 {
                        Adt18::C0(_, h5087, _) => {
                            h5087
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

pub fn f256(v0: u64) -> Adt19 {
    f258(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f257(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5089 = v0;
        let a5090 = match v1.clone() {
            Adt20::C0(h5088, _, _, _, _, _, _, _) => {
                h5088
            }
        };
        let a5091 = 0u64;
        nat_quot(a5089, a5090, a5091)
    };
    let v3: u64 = {
        let a5093 = v2;
        let a5094 = match v1.clone() {
            Adt20::C0(_, h5092, _, _, _, _, _, _) => {
                h5092
            }
        };
        let a5095 = 0u64;
        nat_quot(a5093, a5094, a5095)
    };
    f123({
        let a5097 = v0;
        let a5098 = match v1.clone() {
            Adt20::C0(h5096, _, _, _, _, _, _, _) => {
                h5096
            }
        };
        let a5099 = 0u64;
        nat_rem(a5097, a5098, a5099)
    }, {
        let a5101 = v2;
        let a5102 = match v1.clone() {
            Adt20::C0(_, h5100, _, _, _, _, _, _) => {
                h5100
            }
        };
        let a5103 = 0u64;
        nat_rem(a5101, a5102, a5103)
    }, {
        let a5105 = v3;
        let a5106 = match v1.clone() {
            Adt20::C0(_, _, h5104, _, _, _, _, _) => {
                h5104
            }
        };
        let a5107 = 0u64;
        nat_rem(a5105, a5106, a5107)
    }, {
        let a5109 = v3;
        let a5110 = match v1.clone() {
            Adt20::C0(_, _, h5108, _, _, _, _, _) => {
                h5108
            }
        };
        let a5111 = 0u64;
        nat_quot(a5109, a5110, a5111)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5112, _, _, _) => {
            h5112
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5113, _, _) => {
            h5113
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5114, _) => {
            h5114
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5115) => {
            h5115
        }
    })
}

pub fn f258(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5116 = v0;
    let a5117 = 4u64;
    if if if nat_le(a5116, a5117) {
        true
    } else {
        let a5118 = 9u64;
        let a5119 = v0;
        if nat_le(a5118, a5119) {
            let a5120 = v0;
            let a5121 = 12u64;
            nat_le(a5120, a5121)
        } else {
            false
        }
    } {
        true
    } else {
        let a5122 = v0;
        let a5123 = 16u64;
        if if nat_eq(a5122, a5123) {
            true
        } else {
            let a5124 = v0;
            let a5125 = 19u64;
            nat_eq(a5124, a5125)
        } {
            true
        } else {
            let a5126 = v0;
            let a5127 = 21u64;
            if nat_eq(a5126, a5127) {
                true
            } else {
                let a5128 = v0;
                let a5129 = 22u64;
                nat_eq(a5128, a5129)
            }
        }
    } {
        Adt19::C1({
            let a5130 = v0;
            let a5131 = 4u64;
            if if nat_lt(a5130, a5131) {
                true
            } else {
                let a5132 = 21u64;
                let a5133 = v0;
                nat_le(a5132, a5133)
            } {
                0u64
            } else {
                let a5134 = v0;
                let a5135 = 16u64;
                if if nat_eq(a5134, a5135) {
                    true
                } else {
                    let a5136 = v0;
                    let a5137 = 19u64;
                    nat_eq(a5136, a5137)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5138 = v0;
            let a5139 = 4u64;
            if nat_lt(a5138, a5139) {
                v0
            } else {
                let a5140 = v0;
                let a5141 = 4u64;
                if if nat_eq(a5140, a5141) {
                    true
                } else {
                    let a5142 = v0;
                    let a5143 = 16u64;
                    nat_eq(a5142, a5143)
                } {
                    0u64
                } else {
                    let a5144 = v0;
                    let a5145 = 19u64;
                    if nat_eq(a5144, a5145) {
                        3u64
                    } else {
                        let a5146 = 21u64;
                        let a5147 = v0;
                        if nat_le(a5146, a5147) {
                            let a5148 = v0;
                            let a5149 = 17u64;
                            nat_sub(a5148, a5149)
                        } else {
                            let a5150 = v0;
                            let a5151 = 8u64;
                            nat_sub(a5150, a5151)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f259(v0: u64) -> R<u64> {
    let a5154 = 2u64;
    let a5155 = {
        let a5152 = 6u64;
        let a5153 = v0;
        nat_mul(a5152, a5153)?
    };
    nat_add(a5154, a5155)
}

pub fn f260(v0: u64) -> R<u64> {
    let a5156 = f259(v0)?;
    let a5157 = 11u64;
    nat_add(a5156, a5157)
}

pub fn f261(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5158 = v0;
        let a5159 = 6u64;
        let a5160 = 0u64;
        nat_rem(a5158, a5159, a5160)
    };
    let v5: u64 = {
        let a5167 = {
            let a5162 = v1;
            let a5163 = match v2.clone() {
                Adt20::C0(h5161, _, _, _, _, _, _, _) => {
                    h5161
                }
            };
            nat_mul(a5162, a5163)?
        };
        let a5168 = {
            let a5164 = v0;
            let a5165 = 6u64;
            let a5166 = 0u64;
            nat_quot(a5164, a5165, a5166)
        };
        nat_add(a5167, a5168)?
    };
    let v6: u64 = f257(v5, v3)?;
    let v7: u64 = f257(v5, v2)?;
    let a5169 = v4;
    let a5170 = 0u64;
    if nat_eq(a5169, a5170) {
        Ok(f258(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5171 = v4;
        let a5172 = 1u64;
        if nat_eq(a5171, a5172) {
            Ok(f258(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5173 = v4;
            let a5174 = 2u64;
            if nat_eq(a5173, a5174) {
                Ok(f258(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5175 = v4;
                let a5176 = 3u64;
                if nat_eq(a5175, a5176) {
                    Ok(f258(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5177 = v4;
                    let a5178 = 4u64;
                    if nat_eq(a5177, a5178) {
                        Ok(f258(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f258(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f262(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5179 = v0;
    let a5180 = 0u64;
    if nat_eq(a5179, a5180) {
        f258(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5181 = v0;
        let a5182 = 1u64;
        if nat_eq(a5181, a5182) {
            f258(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5183 = v0;
            let a5184 = 2u64;
            if nat_eq(a5183, a5184) {
                f258(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5185 = v0;
                let a5186 = 3u64;
                if nat_eq(a5185, a5186) {
                    f258(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5187 = v0;
                    let a5188 = 4u64;
                    if nat_eq(a5187, a5188) {
                        f258(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5189 = v0;
                        let a5190 = 5u64;
                        if nat_eq(a5189, a5190) {
                            f258(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5191 = v0;
                            let a5192 = 6u64;
                            if nat_eq(a5191, a5192) {
                                f258(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5193 = v0;
                                let a5194 = 7u64;
                                if nat_eq(a5193, a5194) {
                                    f258(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5195 = v0;
                                    let a5196 = 8u64;
                                    if nat_eq(a5195, a5196) {
                                        f258(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5197 = v0;
                                        let a5198 = 9u64;
                                        if nat_eq(a5197, a5198) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5199 = v0;
                                            let a5200 = 10u64;
                                            if nat_eq(a5199, a5200) {
                                                f258(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f263(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5201 = v0;
        let a5202 = 3u64;
        let a5203 = 0u64;
        nat_rem(a5201, a5202, a5203)
    };
    let v6: u64 = {
        let a5210 = {
            let a5205 = v1;
            let a5206 = match v2.clone() {
                Adt20::C0(h5204, _, _, _, _, _, _, _) => {
                    h5204
                }
            };
            nat_mul(a5205, a5206)?
        };
        let a5211 = {
            let a5207 = v0;
            let a5208 = 3u64;
            let a5209 = 0u64;
            nat_quot(a5207, a5208, a5209)
        };
        nat_add(a5210, a5211)?
    };
    let v7: u64 = f257(v6, v3)?;
    let v8: u64 = f257(v6, v2)?;
    let v9: u64 = f257(v6, v4)?;
    let a5212 = v5;
    let a5213 = 0u64;
    if nat_eq(a5212, a5213) {
        Ok(f258(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5214 = v5;
        let a5215 = 1u64;
        if nat_eq(a5214, a5215) {
            Ok(f258(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f258(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f264(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5216, _, _, _, _, _, _, _) => {
            h5216
        }
    };
    let v5: u64 = {
        let a5219 = 1u64;
        let a5220 = {
            let a5217 = v4;
            let a5218 = f252(v3);
            nat_mul(a5217, a5218)?
        };
        nat_add(a5219, a5220)?
    };
    let a5221 = v0;
    let a5222 = 0u64;
    if nat_eq(a5221, a5222) {
        Ok(f256(0u64))
    } else {
        let a5223 = v0;
        let a5224 = v5;
        if nat_lt(a5223, a5224) {
            let v6: u64 = {
                let a5225 = v0;
                let a5226 = 1u64;
                nat_sub(a5225, a5226)
            };
            let v7: u64 = f257({
                let a5232 = {
                    let a5227 = v1;
                    let a5228 = v4;
                    nat_mul(a5227, a5228)?
                };
                let a5233 = {
                    let a5229 = v6;
                    let a5230 = f252(v3);
                    let a5231 = 0u64;
                    nat_quot(a5229, a5230, a5231)
                };
                nat_add(a5232, a5233)?
            }, v2)?;
            if v3 {
                let a5237 = {
                    let a5234 = v6;
                    let a5235 = 2u64;
                    let a5236 = 0u64;
                    nat_rem(a5234, a5235, a5236)
                };
                let a5238 = 0u64;
                if nat_eq(a5237, a5238) {
                    Ok(f258(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f258(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f258(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5239 = v0;
                let a5240 = v5;
                nat_sub(a5239, a5240)
            };
            let a5241 = v8;
            let a5242 = 0u64;
            if nat_eq(a5241, a5242) {
                Ok(f258(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5243 = v8;
                    let a5244 = 1u64;
                    nat_eq(a5243, a5244)
                } else {
                    false
                } {
                    Ok(f258(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5245 = v8;
                    let a5246 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5245, a5246) {
                        Ok(f258(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5247 = v8;
                        let a5248 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5247, a5248) {
                            Ok(f256(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f265(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5250 = 13u64;
        let a5251 = f253(match v2.clone() {
            Adt20::C0(h5249, _, _, _, _, _, _, _) => {
                h5249
            }
        });
        nat_mul(a5250, a5251)?
    } else {
        0u64
    };
    let a5252 = v0;
    let a5253 = v5;
    if nat_lt(a5252, a5253) {
        f271({
            let a5254 = v0;
            let a5255 = 13u64;
            let a5256 = 0u64;
            nat_rem(a5254, a5255, a5256)
        }, v1, {
            let a5257 = v0;
            let a5258 = 13u64;
            let a5259 = 0u64;
            nat_quot(a5257, a5258, a5259)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5260 = v0;
            let a5261 = v5;
            nat_sub(a5260, a5261)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f272({
            let a5262 = v6;
            let a5263 = v7;
            let a5264 = 0u64;
            nat_rem(a5262, a5263, a5264)
        }, {
            let a5274 = {
                let a5269 = {
                    let a5266 = v1;
                    let a5267 = match v2.clone() {
                        Adt20::C0(h5265, _, _, _, _, _, _, _) => {
                            h5265
                        }
                    };
                    nat_mul(a5266, a5267)?
                };
                let a5270 = if v4 {
                    f273(match v2.clone() {
                        Adt20::C0(h5268, _, _, _, _, _, _, _) => {
                            h5268
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5269, a5270)?
            };
            let a5275 = {
                let a5271 = v6;
                let a5272 = v7;
                let a5273 = 0u64;
                nat_quot(a5271, a5272, a5273)
            };
            nat_add(a5274, a5275)?
        }, v2, v3, v4)
    }
}

pub fn f266(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5276 = v0;
    let a5277 = 0u64;
    if nat_eq(a5276, a5277) {
        f258(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5278 = v0;
        let a5279 = 1u64;
        if nat_eq(a5278, a5279) {
            f258(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5280 = v0;
                let a5281 = 2u64;
                nat_eq(a5280, a5281)
            } else {
                false
            } {
                f258(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5282 = v0;
                let a5283 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5282, a5283) {
                    f258(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5284 = v0;
                    let a5285 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5284, a5285) {
                        f258(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5286 = v0;
                        let a5287 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5286, a5287) {
                            f258(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f267(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5291 = 1u64;
        let a5292 = {
            let a5289 = match v4.clone() {
                Adt20::C0(h5288, _, _, _, _, _, _, _) => {
                    h5288
                }
            };
            let a5290 = v6;
            nat_mul(a5289, a5290)?
        };
        nat_add(a5291, a5292)?
    };
    let a5293 = v1;
    let a5294 = 0u64;
    if nat_eq(a5293, a5294) {
        Ok(f256(1u64))
    } else {
        let a5295 = v1;
        let a5296 = v7;
        if nat_lt(a5295, a5296) {
            let v8: u64 = {
                let a5297 = v1;
                let a5298 = 1u64;
                nat_sub(a5297, a5298)
            };
            let v9: u64 = {
                let a5305 = {
                    let a5300 = f270(v2, v3, v4.clone())?;
                    let a5301 = match v4.clone() {
                        Adt20::C0(h5299, _, _, _, _, _, _, _) => {
                            h5299
                        }
                    };
                    nat_mul(a5300, a5301)?
                };
                let a5306 = {
                    let a5302 = v8;
                    let a5303 = v6;
                    let a5304 = 0u64;
                    nat_quot(a5302, a5303, a5304)
                };
                nat_add(a5305, a5306)?
            };
            let v10: u64 = f257(v9, v4)?;
            let v11: u64 = f257(v9, v5)?;
            let v12: u64 = {
                let a5307 = v8;
                let a5308 = v6;
                let a5309 = 0u64;
                nat_rem(a5307, a5308, a5309)
            };
            if v0 {
                let a5310 = v12;
                let a5311 = 0u64;
                if nat_eq(a5310, a5311) {
                    Ok(f258(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5312 = v12;
                    let a5313 = 1u64;
                    if nat_eq(a5312, a5313) {
                        Ok(f258(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5314 = v12;
                        let a5315 = 2u64;
                        if nat_eq(a5314, a5315) {
                            Ok(f258(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f258(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5316 = v12;
                let a5317 = 0u64;
                if nat_eq(a5316, a5317) {
                    Ok(f258(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f258(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5318 = v1;
            let a5319 = v7;
            if nat_eq(a5318, a5319) {
                Ok(f258(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f268(v0: u64, v1: u64) -> Adt19 {
    let a5320 = v0;
    let a5321 = 0u64;
    if nat_eq(a5320, a5321) {
        f258(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5322 = v0;
        let a5323 = 1u64;
        if nat_eq(a5322, a5323) {
            f258(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5324 = v0;
            let a5325 = 2u64;
            if nat_eq(a5324, a5325) {
                f258(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5326 = v0;
                let a5327 = 3u64;
                if nat_eq(a5326, a5327) {
                    f256(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5328 = v0;
    let a5329 = 0u64;
    if nat_eq(a5328, a5329) {
        f258(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5330 = v0;
        let a5331 = 1u64;
        if nat_eq(a5330, a5331) {
            f258(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5332 = v0;
            let a5333 = 2u64;
            if nat_eq(a5332, a5333) {
                f258(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5334 = v0;
                let a5335 = 3u64;
                if nat_eq(a5334, a5335) {
                    f258(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5336 = v0;
                    let a5337 = 4u64;
                    if nat_eq(a5336, a5337) {
                        f258(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5338 = v0;
                        let a5339 = 5u64;
                        if nat_eq(a5338, a5339) {
                            f258(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5350 = {
        let a5348 = {
            let a5345 = {
                let a5342 = match v1.clone() {
                    Adt22::C0(_, _, h5340, _) => {
                        h5340
                    }
                };
                let a5343 = match v2.clone() {
                    Adt20::C0(_, _, h5341, _, _, _, _, _) => {
                        h5341
                    }
                };
                nat_mul(a5342, a5343)?
            };
            let a5346 = match v1 {
                Adt22::C0(h5344, _, _, _) => {
                    h5344
                }
            };
            nat_add(a5345, a5346)?
        };
        let a5349 = match v2 {
            Adt20::C0(_, h5347, _, _, _, _, _, _) => {
                h5347
            }
        };
        nat_mul(a5348, a5349)?
    };
    let a5351 = v0;
    nat_add(a5350, a5351)
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5359 = {
            let a5357 = {
                let a5353 = v1;
                let a5354 = match v3.clone() {
                    Adt20::C0(h5352, _, _, _, _, _, _, _) => {
                        h5352
                    }
                };
                nat_mul(a5353, a5354)?
            };
            let a5358 = {
                let a5355 = v2;
                let a5356 = 4u64;
                nat_mul(a5355, a5356)?
            };
            nat_add(a5357, a5358)?
        };
        let a5360 = f274(v0);
        nat_add(a5359, a5360)?
    };
    let v6: u64 = f257(v5, v3)?;
    let v7: u64 = f257(v5, v4)?;
    let a5361 = v0;
    let a5362 = 0u64;
    if if if if nat_eq(a5361, a5362) {
        true
    } else {
        let a5363 = v0;
        let a5364 = 2u64;
        nat_eq(a5363, a5364)
    } {
        true
    } else {
        let a5365 = v0;
        let a5366 = 5u64;
        nat_eq(a5365, a5366)
    } {
        true
    } else {
        let a5367 = v0;
        let a5368 = 7u64;
        nat_eq(a5367, a5368)
    } {
        Ok(f258(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5369 = v0;
        let a5370 = 1u64;
        if if if if nat_eq(a5369, a5370) {
            true
        } else {
            let a5371 = v0;
            let a5372 = 3u64;
            nat_eq(a5371, a5372)
        } {
            true
        } else {
            let a5373 = v0;
            let a5374 = 6u64;
            nat_eq(a5373, a5374)
        } {
            true
        } else {
            let a5375 = v0;
            let a5376 = 8u64;
            nat_eq(a5375, a5376)
        } {
            Ok(f258(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5377 = v0;
                let a5378 = 1u64;
                if nat_eq(a5377, a5378) {
                    4u64
                } else {
                    let a5379 = v0;
                    let a5380 = 6u64;
                    if nat_eq(a5379, a5380) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5381 = v0;
            let a5382 = 4u64;
            if nat_eq(a5381, a5382) {
                Ok(f258(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5383 = v0;
                let a5384 = 9u64;
                if nat_eq(a5383, a5384) {
                    Ok(f258(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5385 = v0;
                    let a5386 = 10u64;
                    if nat_eq(a5385, a5386) {
                        Ok(f258(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5387 = v0;
                        let a5388 = 11u64;
                        if nat_eq(a5387, a5388) {
                            Ok(f258(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5389 = v0;
                            let a5390 = 12u64;
                            if nat_eq(a5389, a5390) {
                                Ok(f258(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f272(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f257(v1, v2)?;
    let v6: u64 = f257(v1, v3)?;
    let a5391 = v0;
    let a5392 = 0u64;
    if nat_eq(a5391, a5392) {
        Ok(f258(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5393 = v0;
        let a5394 = 1u64;
        if nat_eq(a5393, a5394) {
            Ok(f258(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5395 = v0;
                let a5396 = 2u64;
                if nat_eq(a5395, a5396) {
                    Ok(f258(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5397 = v0;
                    let a5398 = 3u64;
                    if nat_eq(a5397, a5398) {
                        Ok(f258(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5399 = v0;
                let a5400 = 2u64;
                if nat_eq(a5399, a5400) {
                    Ok(f258(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f273(v0: u64) -> R<u64> {
    let a5401 = 4u64;
    let a5402 = f253(v0);
    nat_mul(a5401, a5402)
}

pub fn f274(v0: u64) -> u64 {
    let a5403 = v0;
    let a5404 = 5u64;
    if nat_lt(a5403, a5404) {
        let a5405 = v0;
        let a5406 = 2u64;
        if nat_lt(a5405, a5406) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5407 = v0;
        let a5408 = 7u64;
        if nat_lt(a5407, a5408) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
