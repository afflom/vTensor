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
                        if nat_eq(a5, a6) {
                            Ok(Adt9::C0(f2(v2, v7, v4, v6)?))
                        } else {
                            let a7 = v1;
                            let a8 = 53u64;
                            if if nat_eq(a7, a8) {
                                true
                            } else {
                                let a9 = v1;
                                let a10 = 54u64;
                                nat_eq(a9, a10)
                            } {
                                Ok(Adt9::C0(f3({
                                    let a11 = v1;
                                    let a12 = 54u64;
                                    nat_eq(a11, a12)
                                }, v2, v7, v4, v6)?))
                            } else {
                                let a13 = v1;
                                let a14 = 78u64;
                                if nat_eq(a13, a14) {
                                    Ok(Adt9::C0(f4(v2, v7, v4, v5, v6)?))
                                } else {
                                    let a15 = v1;
                                    let a16 = 23u64;
                                    if if if nat_eq(a15, a16) {
                                        true
                                    } else {
                                        let a17 = v1;
                                        let a18 = 26u64;
                                        nat_eq(a17, a18)
                                    } {
                                        true
                                    } else {
                                        let a19 = v1;
                                        let a20 = 25u64;
                                        nat_eq(a19, a20)
                                    } {
                                        Ok(Adt9::C0(f5(v1, v2, v7, v4, v6)?))
                                    } else {
                                        let a21 = v1;
                                        let a22 = 24u64;
                                        if nat_eq(a21, a22) {
                                            Ok(Adt9::C0(f6(v2, v7, v6)?))
                                        } else {
                                            let a23 = v1;
                                            let a24 = 27u64;
                                            if nat_eq(a23, a24) {
                                                Ok(Adt9::C0(f7(v2, v7, v6)?))
                                            } else {
                                                let a25 = v1;
                                                let a26 = 3u64;
                                                if nat_eq(a25, a26) {
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
            let a28 = 0u64;
            let a29 = match v8.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h27, _, _, _, _, _, _, _) => {
                    h27
                }
            };
            if nat_lt(a28, a29) {
                f10(v8, v9, v10, v11)
            } else {
                f11(v8, v9, v10, v11)
            }
        }
        Adt0::C5(v12) => {
            Ok(Adt9::C9({
                let a30 = 3240099840u64;
                let a31 = v12;
                if if nat_le(a30, a31) {
                    let a32 = v12;
                    let a33 = 4286578688u64;
                    nat_le(a32, a33)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a34 = 1092616192u64;
                    let a35 = v12;
                    if if nat_le(a34, a35) {
                        let a36 = v12;
                        let a37 = 2139095040u64;
                        nat_le(a36, a37)
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
    let a39 = match v1.clone() {
        Adt2::C0(h38, _, _, _, _, _, _, _, _, _, _) => {
            h38
        }
    };
    let a40 = 0u64;
    if if if if if if if if nat_eq(a39, a40) {
        let a42 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h41, _, _, _, _, _) => {
                h41
            }
        };
        let a43 = 4u64;
        nat_eq(a42, a43)
    } else {
        false
    } {
        f17(match v0.clone() {
            Adt1::C0(h44, _, _, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        })
    } else {
        false
    } {
        f17(match v0.clone() {
            Adt1::C0(_, h45, _, _, _, _, _, _, _, _, _, _) => {
                h45
            }
        })
    } else {
        false
    } {
        let a47 = 0u64;
        let a48 = match v0.clone() {
            Adt1::C0(_, _, h46, _, _, _, _, _, _, _, _, _) => {
                h46
            }
        };
        nat_lt(a47, a48)
    } else {
        false
    } {
        let a50 = match v0.clone() {
            Adt1::C0(_, _, h49, _, _, _, _, _, _, _, _, _) => {
                h49
            }
        };
        let a51 = 2139095040u64;
        nat_lt(a50, a51)
    } else {
        false
    } {
        let a52 = v2;
        let a53 = 9223372036854775808u64;
        nat_lt(a52, a53)
    } else {
        false
    } {
        let a55 = f18(v2)?;
        let a56 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h54, _) => {
                h54
            }
        };
        nat_le(a55, a56)
    } else {
        false
    } {
        Ok(Adt10::C4(f19(v2)?, 4u64, f20(v1.clone(), 1u64), f20(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h57, _, _, _, _, _, _, _, _, _, _, _) => {
                h57
            }
        }, match v0.clone() {
            Adt1::C0(_, h58, _, _, _, _, _, _, _, _, _, _) => {
                h58
            }
        }, match v0 {
            Adt1::C0(_, _, h59, _, _, _, _, _, _, _, _, _) => {
                h59
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m60 = v2;
    match m60 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v0.clone() {
                Adt1::C0(h61, _, _, _, _, _, _, _, _, _, _, _) => {
                    h61
                }
            };
            let v6: u64 = match v1.clone() {
                Adt2::C0(_, h62, _, _, _, _, _, _, _, _, _) => {
                    h62
                }
            };
            let v7: u64 = match v4.clone() {
                Adt2::C0(_, h63, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            };
            let v8: u64 = match v4.clone() {
                Adt2::C0(_, _, h64, _, _, _, _, _, _, _, _) => {
                    h64
                }
            };
            let v9: u64 = match v1.clone() {
                Adt2::C0(_, _, h65, _, _, _, _, _, _, _, _) => {
                    h65
                }
            };
            let v10: u64 = {
                let a70 = v6;
                let a71 = {
                    let a68 = {
                        let a66 = v7;
                        let a67 = 1u64;
                        nat_sub(a66, a67)
                    };
                    let a69 = v5;
                    nat_mul(a68, a69)?
                };
                nat_add(a70, a71)?
            };
            let v11: u64 = {
                let a72 = v6;
                let a73 = v9;
                nat_mul(a72, a73)?
            };
            let v12: u64 = {
                let a74 = v7;
                let a75 = 32u64;
                if nat_lt(a74, a75) {
                    v7
                } else {
                    32u64
                }
            };
            let a77 = match v1.clone() {
                Adt2::C0(h76, _, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a78 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a77, a78) {
                true
            } else {
                let a80 = match v1.clone() {
                    Adt2::C0(h79, _, _, _, _, _, _, _, _, _, _) => {
                        h79
                    }
                };
                let a81 = 1u64;
                nat_eq(a80, a81)
            } {
                let a83 = match v4.clone() {
                    Adt2::C0(h82, _, _, _, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                let a84 = 0u64;
                nat_eq(a83, a84)
            } else {
                false
            } {
                let a86 = match v3.clone() {
                    Adt2::C0(h85, _, _, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a87 = 0u64;
                nat_eq(a86, a87)
            } else {
                false
            } {
                let a89 = match v0.clone() {
                    Adt1::C0(_, h88, _, _, _, _, _, _, _, _, _, _) => {
                        h88
                    }
                };
                let a90 = 0u64;
                nat_eq(a89, a90)
            } else {
                false
            } {
                let a92 = match v0 {
                    Adt1::C0(_, _, h91, _, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a93 = 1u64;
                nat_eq(a92, a93)
            } else {
                false
            } {
                let a94 = v5;
                let a95 = 2147483648u64;
                nat_lt(a94, a95)
            } else {
                false
            } {
                let a96 = 0u64;
                let a97 = v5;
                nat_lt(a96, a97)
            } else {
                false
            } {
                let a98 = 0u64;
                let a99 = v6;
                nat_lt(a98, a99)
            } else {
                false
            } {
                let a100 = 0u64;
                let a101 = v7;
                nat_lt(a100, a101)
            } else {
                false
            } {
                let a102 = 0u64;
                let a103 = v8;
                nat_lt(a102, a103)
            } else {
                false
            } {
                let a104 = 0u64;
                let a105 = v9;
                nat_lt(a104, a105)
            } else {
                false
            } {
                let a107 = match v1.clone() {
                    Adt2::C0(_, _, _, h106, _, _, _, _, _, _, _) => {
                        h106
                    }
                };
                let a108 = v8;
                nat_eq(a107, a108)
            } else {
                false
            } {
                let a110 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h109, _, _, _, _, _, _) => {
                        h109
                    }
                };
                let a111 = 1u64;
                nat_eq(a110, a111)
            } else {
                false
            } {
                let a113 = match v4.clone() {
                    Adt2::C0(_, _, _, h112, _, _, _, _, _, _, _) => {
                        h112
                    }
                };
                let a114 = 1u64;
                nat_eq(a113, a114)
            } else {
                false
            } {
                let a116 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h115, _, _, _, _, _, _) => {
                        h115
                    }
                };
                let a117 = 1u64;
                nat_eq(a116, a117)
            } else {
                false
            } {
                let a119 = match v3.clone() {
                    Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                        h118
                    }
                };
                let a120 = v10;
                nat_eq(a119, a120)
            } else {
                false
            } {
                let a122 = match v3.clone() {
                    Adt2::C0(_, _, h121, _, _, _, _, _, _, _, _) => {
                        h121
                    }
                };
                let a123 = v9;
                nat_eq(a122, a123)
            } else {
                false
            } {
                let a125 = match v3.clone() {
                    Adt2::C0(_, _, _, h124, _, _, _, _, _, _, _) => {
                        h124
                    }
                };
                let a126 = 1u64;
                nat_eq(a125, a126)
            } else {
                false
            } {
                let a128 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h127, _, _, _, _, _, _) => {
                        h127
                    }
                };
                let a129 = 1u64;
                nat_eq(a128, a129)
            } else {
                false
            } {
                f21(v3.clone())?
            } else {
                false
            } {
                let a134 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, h130, _, _, _, _, _) => {
                        h130
                    }
                };
                let a135 = {
                    let a132 = match v1.clone() {
                        Adt2::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                            h131
                        }
                    };
                    let a133 = 1u64;
                    if nat_eq(a132, a133) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a134, a135)
            } else {
                false
            } {
                let a137 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h136, _, _, _, _, _) => {
                        h136
                    }
                };
                let a138 = 4u64;
                nat_eq(a137, a138)
            } else {
                false
            } {
                f22(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f22(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v7, v8, v11, {
                    let a139 = v8;
                    let a140 = v11;
                    nat_mul(a139, a140)?
                }, v12, {
                    let a141 = v12;
                    let a142 = v8;
                    nat_mul(a141, a142)?
                }, {
                    let a143 = v12;
                    let a144 = v11;
                    nat_mul(a143, a144)?
                }, {
                    let a145 = v10;
                    let a146 = v9;
                    nat_mul(a145, a146)?
                }, f20(v1, 1u64), f20(v4, 1u64), f20(v3, 1u64), Adt6::C0(v6, v5, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f3(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m147 = v3;
    match m147 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                    h148
                }
            }, match v1.clone() {
                Adt1::C0(h149, _, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h150, _, _, _, _, _, _, _, _) => {
                        h150
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h151, _, _, _, _, _, _, _, _, _) => {
                        h151
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h152, _, _, _, _, _) => {
                        h152
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h153, _, _, _, _, _, _, _) => {
                        h153
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
                    h154
                }
            }, match v1.clone() {
                Adt1::C0(_, h155, _, _, _, _, _, _, _, _, _, _) => {
                    h155
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h156, _, _, _, _, _, _, _) => {
                        h156
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h157, _, _, _, _, _, _, _, _) => {
                        h157
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h158, _, _, _, _) => {
                        h158
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h159, _, _, _, _, _, _) => {
                        h159
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h160, _, _, _, _, _, _, _) => {
                        h160
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h161, _, _, _, _, _, _, _, _, _) => {
                        h161
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h162, _, _, _, _, _, _) => {
                        h162
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h163, _, _, _) => {
                        h163
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f23(v6.clone(), match v5.clone() {
                Adt2::C0(_, h164, _, _, _, _, _, _, _, _, _) => {
                    h164
                }
            })?;
            let v10: u64 = f23(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h165, _, _, _, _, _, _, _, _) => {
                    h165
                }
            })?;
            let v11: u64 = if v0 {
                f23(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h166, _, _, _, _, _, _, _) => {
                        h166
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h167, _, _) => {
                        h167
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h168, _, _, _, _, _, _, _) => {
                        h168
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h169, _) => {
                        h169
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h170, _, _, _, _, _, _) => {
                        h170
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h171) => {
                        h171
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h172, _, _, _, _, _, _) => {
                        h172
                    }
                }
            };
            let v15: u64 = {
                let a177 = {
                    let a173 = v9;
                    let a174 = v10;
                    nat_mul(a173, a174)?
                };
                let a178 = {
                    let a175 = v11;
                    let a176 = v13;
                    nat_mul(a175, a176)?
                };
                nat_mul(a177, a178)?
            };
            let v16: u64 = {
                let a186 = {
                    let a181 = match v6.clone() {
                        Adt6::C0(h179, _, _, _) => {
                            h179
                        }
                    };
                    let a182 = match v7.clone() {
                        Adt6::C0(h180, _, _, _) => {
                            h180
                        }
                    };
                    nat_mul(a181, a182)?
                };
                let a187 = {
                    let a184 = match v8.clone() {
                        Adt6::C0(h183, _, _, _) => {
                            h183
                        }
                    };
                    let a185 = v12;
                    nat_mul(a184, a185)?
                };
                nat_mul(a186, a187)?
            };
            let v17: u64 = {
                let a188 = v15;
                let a189 = 32u64;
                if nat_lt(a188, a189) {
                    v15
                } else {
                    32u64
                }
            };
            let a191 = match v2.clone() {
                Adt2::C0(h190, _, _, _, _, _, _, _, _, _, _) => {
                    h190
                }
            };
            let a192 = 0u64;
            if if if if if if if if if if if nat_eq(a191, a192) {
                true
            } else {
                let a194 = match v2.clone() {
                    Adt2::C0(h193, _, _, _, _, _, _, _, _, _, _) => {
                        h193
                    }
                };
                let a195 = 1u64;
                nat_eq(a194, a195)
            } {
                let a197 = match v5.clone() {
                    Adt2::C0(h196, _, _, _, _, _, _, _, _, _, _) => {
                        h196
                    }
                };
                let a198 = 0u64;
                nat_eq(a197, a198)
            } else {
                false
            } {
                let a200 = match v4.clone() {
                    Adt2::C0(h199, _, _, _, _, _, _, _, _, _, _) => {
                        h199
                    }
                };
                let a201 = 0u64;
                nat_eq(a200, a201)
            } else {
                false
            } {
                let a203 = match v4.clone() {
                    Adt2::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                        h202
                    }
                };
                let a204 = v9;
                nat_eq(a203, a204)
            } else {
                false
            } {
                let a206 = match v4.clone() {
                    Adt2::C0(_, _, h205, _, _, _, _, _, _, _, _) => {
                        h205
                    }
                };
                let a207 = v10;
                nat_eq(a206, a207)
            } else {
                false
            } {
                if v0 {
                    let a208 = 0u64;
                    let a209 = v12;
                    if if if if if if if nat_lt(a208, a209) {
                        let a210 = v12;
                        let a211 = 2147483648u64;
                        nat_lt(a210, a211)
                    } else {
                        false
                    } {
                        let a212 = v14;
                        let a213 = 2147483648u64;
                        nat_lt(a212, a213)
                    } else {
                        false
                    } {
                        let a214 = v13;
                        let a215 = 2147483648u64;
                        nat_lt(a214, a215)
                    } else {
                        false
                    } {
                        let a219 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h216, _, _, _, _, _, _) => {
                                h216
                            }
                        };
                        let a220 = {
                            let a217 = v12;
                            let a218 = v14;
                            nat_mul(a217, a218)?
                        };
                        nat_eq(a219, a220)
                    } else {
                        false
                    } {
                        let a224 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h221, _, _, _, _, _, _) => {
                                h221
                            }
                        };
                        let a225 = {
                            let a222 = v12;
                            let a223 = v13;
                            nat_mul(a222, a223)?
                        };
                        nat_eq(a224, a225)
                    } else {
                        false
                    } {
                        let a227 = match v4.clone() {
                            Adt2::C0(_, _, _, h226, _, _, _, _, _, _, _) => {
                                h226
                            }
                        };
                        let a228 = v11;
                        nat_eq(a227, a228)
                    } else {
                        false
                    } {
                        let a232 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h229, _, _, _, _, _, _) => {
                                h229
                            }
                        };
                        let a233 = {
                            let a230 = v14;
                            let a231 = v13;
                            nat_mul(a230, a231)?
                        };
                        nat_eq(a232, a233)
                    } else {
                        false
                    }
                } else {
                    let a235 = match v2.clone() {
                        Adt2::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                            h234
                        }
                    };
                    let a236 = v12;
                    if if if nat_eq(a235, a236) {
                        let a238 = match v4.clone() {
                            Adt2::C0(_, _, _, h237, _, _, _, _, _, _, _) => {
                                h237
                            }
                        };
                        let a239 = v14;
                        nat_eq(a238, a239)
                    } else {
                        false
                    } {
                        let a241 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                                h240
                            }
                        };
                        let a242 = v13;
                        nat_eq(a241, a242)
                    } else {
                        false
                    } {
                        let a243 = 0u64;
                        let a244 = v16;
                        nat_lt(a243, a244)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a245 = 0u64;
                let a246 = v14;
                nat_lt(a245, a246)
            } else {
                false
            } {
                let a247 = 0u64;
                let a248 = v15;
                nat_lt(a247, a248)
            } else {
                false
            } {
                f22(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f22(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a249 = v16;
                    let a250 = v14;
                    nat_mul(a249, a250)?
                }, v17, {
                    let a251 = v17;
                    let a252 = v16;
                    nat_mul(a251, a252)?
                }, {
                    let a253 = v17;
                    let a254 = v14;
                    nat_mul(a253, a254)?
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
    let m255 = v2;
    match m255 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m256 = v3;
            match m256 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a258 = match v1.clone() {
                        Adt2::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                            h257
                        }
                    };
                    let a259 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a258, a259) {
                        let a261 = match v5.clone() {
                            Adt2::C0(h260, _, _, _, _, _, _, _, _, _, _) => {
                                h260
                            }
                        };
                        let a262 = 0u64;
                        nat_eq(a261, a262)
                    } else {
                        false
                    } {
                        let a264 = match v6.clone() {
                            Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                                h263
                            }
                        };
                        let a265 = 0u64;
                        nat_eq(a264, a265)
                    } else {
                        false
                    } {
                        let a267 = match v4.clone() {
                            Adt2::C0(h266, _, _, _, _, _, _, _, _, _, _) => {
                                h266
                            }
                        };
                        let a268 = 0u64;
                        nat_eq(a267, a268)
                    } else {
                        false
                    } {
                        f24(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f24(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f21(v1.clone())?
                    } else {
                        false
                    } {
                        f21(v5.clone())?
                    } else {
                        false
                    } {
                        f21(v6)?
                    } else {
                        false
                    } {
                        f21(v4.clone())?
                    } else {
                        false
                    } {
                        let a274 = match v1.clone() {
                            Adt2::C0(_, h269, _, _, _, _, _, _, _, _, _) => {
                                h269
                            }
                        };
                        let a275 = {
                            let a272 = match v5.clone() {
                                Adt2::C0(_, h270, _, _, _, _, _, _, _, _, _) => {
                                    h270
                                }
                            };
                            let a273 = match v5.clone() {
                                Adt2::C0(_, h271, _, _, _, _, _, _, _, _, _) => {
                                    h271
                                }
                            };
                            nat_mul(a272, a273)?
                        };
                        nat_eq(a274, a275)
                    } else {
                        false
                    } {
                        let a281 = match v1.clone() {
                            Adt2::C0(_, _, h276, _, _, _, _, _, _, _, _) => {
                                h276
                            }
                        };
                        let a282 = {
                            let a279 = match v5.clone() {
                                Adt2::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                                    h277
                                }
                            };
                            let a280 = match v5.clone() {
                                Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                                    h278
                                }
                            };
                            nat_mul(a279, a280)?
                        };
                        nat_eq(a281, a282)
                    } else {
                        false
                    } {
                        let a285 = match v1.clone() {
                            Adt2::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                                h283
                            }
                        };
                        let a286 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h284, _, _, _, _, _, _) => {
                                h284
                            }
                        };
                        nat_eq(a285, a286)
                    } else {
                        false
                    } {
                        let a288 = match v0.clone() {
                            Adt1::C0(h287, _, _, _, _, _, _, _, _, _, _, _) => {
                                h287
                            }
                        };
                        let a289 = 4294967296u64;
                        nat_lt(a288, a289)
                    } else {
                        false
                    } {
                        let a291 = match v0.clone() {
                            Adt1::C0(h290, _, _, _, _, _, _, _, _, _, _, _) => {
                                h290
                            }
                        };
                        let a292 = 0u64;
                        if nat_eq(a291, a292) {
                            if f22(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a295 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h293) => {
                                        h293
                                    }
                                };
                                let a296 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h294) => {
                                        h294
                                    }
                                };
                                nat_eq(a295, a296)
                            }
                        } else {
                            let a299 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h297) => {
                                    h297
                                }
                            };
                            let a300 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h298) => {
                                    h298
                                }
                            };
                            nat_eq(a299, a300)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a302 = match v4.clone() {
                                Adt2::C0(_, h301, _, _, _, _, _, _, _, _, _) => {
                                    h301
                                }
                            };
                            let a303 = 0u64;
                            if if if nat_eq(a302, a303) {
                                true
                            } else {
                                let a305 = match v4.clone() {
                                    Adt2::C0(_, _, h304, _, _, _, _, _, _, _, _) => {
                                        h304
                                    }
                                };
                                let a306 = 0u64;
                                nat_eq(a305, a306)
                            } {
                                true
                            } else {
                                let a308 = match v4.clone() {
                                    Adt2::C0(_, _, _, h307, _, _, _, _, _, _, _) => {
                                        h307
                                    }
                                };
                                let a309 = 0u64;
                                nat_eq(a308, a309)
                            } {
                                true
                            } else {
                                let a311 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                                        h310
                                    }
                                };
                                let a312 = 0u64;
                                nat_eq(a311, a312)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
                                h313
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a315 = match v0 {
                                Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h314
                                }
                            };
                            let a316 = 0u64;
                            if nat_eq(a315, a316) {
                                f16(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f25(v8, match v5.clone() {
                                Adt2::C0(_, _, h317, _, _, _, _, _, _, _, _) => {
                                    h317
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h318, _, _, _, _, _, _, _) => {
                                    h318
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h319, _, _, _, _, _, _) => {
                                    h319
                                }
                            })?
                        };
                        Ok(Adt10::C4(f26(v9, v8, v10)?, 4u64, f20(v1, 1u64), f20(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f5(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f27(v2.clone());
    let v6: Adt14 = f27(v4.clone());
    let v8: Adt15 = {
        let m320 = v3;
        match m320 {
            None => {
                f28(v0, match v1.clone() {
                    Adt1::C0(h321, _, _, _, _, _, _, _, _, _, _, _) => {
                        h321
                    }
                }, match v1 {
                    Adt1::C0(_, h322, _, _, _, _, _, _, _, _, _, _) => {
                        h322
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f28(v0, match v1.clone() {
                    Adt1::C0(h323, _, _, _, _, _, _, _, _, _, _, _) => {
                        h323
                    }
                }, match v1 {
                    Adt1::C0(_, h324, _, _, _, _, _, _, _, _, _, _) => {
                        h324
                    }
                }, true, v5, f27(v7), v6)?
            }
        }
    };
    let m325 = v8;
    match m325 {
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
        Adt2::C0(h326, _, _, _, _, _, _, _, _, _, _) => {
            h326
        }
    };
    let v4: u64 = f29(v3);
    let a328 = match v1.clone() {
        Adt2::C0(h327, _, _, _, _, _, _, _, _, _, _) => {
            h327
        }
    };
    let a329 = 0u64;
    if if if if if if if if nat_eq(a328, a329) {
        true
    } else {
        let a331 = match v1.clone() {
            Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a332 = 30u64;
        nat_eq(a331, a332)
    } {
        let a335 = match v1.clone() {
            Adt2::C0(h333, _, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        let a336 = match v2.clone() {
            Adt2::C0(h334, _, _, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        nat_eq(a335, a336)
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        f30(match v0.clone() {
            Adt1::C0(h337, _, _, _, _, _, _, _, _, _, _, _) => {
                h337
            }
        })
    } else {
        false
    } {
        f31(v1.clone(), v4)
    } else {
        false
    } {
        f31(v2.clone(), v4)
    } else {
        false
    } {
        if f22(v1.clone(), v2.clone())? {
            true
        } else {
            f32(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f33(match v1.clone() {
            Adt2::C0(_, h338, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h339, _, _, _, _, _, _, _, _) => {
                h339
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                h340
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h341, _, _, _, _, _, _) => {
                h341
            }
        }) {
            0u64
        } else {
            let a347 = match v1.clone() {
                Adt2::C0(_, _, h342, _, _, _, _, _, _, _, _) => {
                    h342
                }
            };
            let a348 = {
                let a345 = match v1.clone() {
                    Adt2::C0(_, _, _, h343, _, _, _, _, _, _, _) => {
                        h343
                    }
                };
                let a346 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                        h344
                    }
                };
                nat_mul(a345, a346)?
            };
            nat_mul(a347, a348)?
        };
        let v6: u64 = {
            let a349 = v5;
            let a350 = 0u64;
            if if nat_eq(a349, a350) {
                true
            } else {
                f34(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a352 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h351, _) => {
                        h351
                    }
                };
                let a353 = v4;
                let a354 = 0u64;
                nat_quot(a352, a353, a354)
            }
        };
        Ok(Adt10::C4(f35(v3, match v1.clone() {
            Adt2::C0(_, h355, _, _, _, _, _, _, _, _, _) => {
                h355
            }
        }, v5, v6)?, v4, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h356, _, _, _, _, _, _, _, _, _, _, _) => {
                h356
            }
        }, v5, f36(match v1 {
            Adt2::C0(_, h357, _, _, _, _, _, _, _, _, _) => {
                h357
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a359 = match v1.clone() {
        Adt2::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
            h358
        }
    };
    let a360 = 0u64;
    if if if if if nat_eq(a359, a360) {
        let a362 = match v2.clone() {
            Adt2::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                h361
            }
        };
        let a363 = 0u64;
        nat_eq(a362, a363)
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        f37(match v0.clone() {
            Adt1::C0(h364, _, _, _, _, _, _, _, _, _, _, _) => {
                h364
            }
        })
    } else {
        false
    } {
        if f22(v1.clone(), v2.clone())? {
            true
        } else {
            f32(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f20(v1, 1u64);
        let v4: u64 = {
            let a370 = match v3.clone() {
                Adt3::C0(_, h365, _, _, _, _, _, _) => {
                    h365
                }
            };
            let a371 = {
                let a368 = match v3.clone() {
                    Adt3::C0(_, _, h366, _, _, _, _, _) => {
                        h366
                    }
                };
                let a369 = match v3.clone() {
                    Adt3::C0(_, _, _, h367, _, _, _, _) => {
                        h367
                    }
                };
                nat_mul(a368, a369)?
            };
            nat_mul(a370, a371)?
        };
        Ok(Adt10::C4(f38(match v3.clone() {
            Adt3::C0(h372, _, _, _, _, _, _, _) => {
                h372
            }
        }, v4)?, 4u64, v3, f20(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h373, _, _, _, _, _, _, _, _, _, _, _) => {
                h373
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m374 = v1;
    match m374 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m375 = v2;
            match m375 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a377 = match v0.clone() {
                        Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                            h376
                        }
                    };
                    let a378 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a377, a378) {
                        let a380 = match v4.clone() {
                            Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                                h379
                            }
                        };
                        let a381 = 0u64;
                        nat_eq(a380, a381)
                    } else {
                        false
                    } {
                        let a383 = match v5.clone() {
                            Adt2::C0(h382, _, _, _, _, _, _, _, _, _, _) => {
                                h382
                            }
                        };
                        let a384 = 26u64;
                        nat_eq(a383, a384)
                    } else {
                        false
                    } {
                        let a386 = match v3.clone() {
                            Adt2::C0(h385, _, _, _, _, _, _, _, _, _, _) => {
                                h385
                            }
                        };
                        let a387 = 0u64;
                        nat_eq(a386, a387)
                    } else {
                        false
                    } {
                        f24(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a390 = match v0.clone() {
                            Adt2::C0(_, h388, _, _, _, _, _, _, _, _, _) => {
                                h388
                            }
                        };
                        let a391 = match v4.clone() {
                            Adt2::C0(_, h389, _, _, _, _, _, _, _, _, _) => {
                                h389
                            }
                        };
                        nat_eq(a390, a391)
                    } else {
                        false
                    } {
                        let a394 = match v0.clone() {
                            Adt2::C0(_, _, h392, _, _, _, _, _, _, _, _) => {
                                h392
                            }
                        };
                        let a395 = match v5.clone() {
                            Adt2::C0(_, h393, _, _, _, _, _, _, _, _, _) => {
                                h393
                            }
                        };
                        nat_eq(a394, a395)
                    } else {
                        false
                    } {
                        let a398 = match v0.clone() {
                            Adt2::C0(_, _, _, h396, _, _, _, _, _, _, _) => {
                                h396
                            }
                        };
                        let a399 = match v5.clone() {
                            Adt2::C0(_, _, h397, _, _, _, _, _, _, _, _) => {
                                h397
                            }
                        };
                        nat_eq(a398, a399)
                    } else {
                        false
                    } {
                        let a401 = 0u64;
                        let a402 = match v5.clone() {
                            Adt2::C0(_, _, _, h400, _, _, _, _, _, _, _) => {
                                h400
                            }
                        };
                        nat_lt(a401, a402)
                    } else {
                        false
                    } {
                        let a404 = 0u64;
                        let a405 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h403, _, _, _, _, _, _) => {
                                h403
                            }
                        };
                        nat_lt(a404, a405)
                    } else {
                        false
                    } {
                        let a407 = 4u64;
                        let a408 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h406, _, _, _, _, _) => {
                                h406
                            }
                        };
                        nat_le(a407, a408)
                    } else {
                        false
                    } {
                        let a410 = 4u64;
                        let a411 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h409, _, _, _, _, _) => {
                                h409
                            }
                        };
                        nat_le(a410, a411)
                    } else {
                        false
                    } {
                        let a413 = 4u64;
                        let a414 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h412, _, _, _, _, _) => {
                                h412
                            }
                        };
                        nat_le(a413, a414)
                    } else {
                        false
                    } {
                        let a416 = 4u64;
                        let a417 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h415, _, _, _, _, _) => {
                                h415
                            }
                        };
                        nat_le(a416, a417)
                    } else {
                        false
                    } {
                        f22(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f22(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f22(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f39(match v0.clone() {
                            Adt2::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                                h418
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                                h419
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h420, _, _, _, _, _, _, _) => {
                                h420
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                                h421
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
        Adt1::C0(h422, _, _, _, _, _, _, _, _, _, _, _) => {
            h422
        }
    };
    Ok(Adt9::C0({
        let a423 = v0;
        let a424 = 52u64;
        if nat_eq(a423, a424) {
            f40(v1, v2, v4)?
        } else {
            let a425 = v0;
            let a426 = 73u64;
            if nat_eq(a425, a426) {
                f41(v2, v3, v4)?
            } else {
                let a427 = v0;
                let a428 = 57u64;
                if if nat_eq(a427, a428) {
                    true
                } else {
                    let a429 = v0;
                    let a430 = 58u64;
                    nat_eq(a429, a430)
                } {
                    let m431 = v3;
                    match m431 {
                        None => {
                            f42({
                                let a432 = v0;
                                let a433 = 58u64;
                                nat_eq(a432, a433)
                            }, v1, v2, v4)?
                        }
                        Some(_) => {
                            Adt10::C1
                        }
                    }
                } else {
                    let a434 = v0;
                    let a435 = 49u64;
                    if nat_eq(a434, a435) {
                        f43(v1, v2, v3, v4)?
                    } else {
                        let a436 = v0;
                        let a437 = 51u64;
                        if nat_eq(a436, a437) {
                            f44(v1, v2, v3, v4)?
                        } else {
                            let a438 = v0;
                            let a439 = 39u64;
                            if nat_eq(a438, a439) {
                                f45(v2, v3, v4)?
                            } else {
                                let a440 = v0;
                                let a441 = 77u64;
                                if nat_eq(a440, a441) {
                                    f46(v2, v4)?
                                } else {
                                    let a442 = v0;
                                    let a443 = 40u64;
                                    if nat_eq(a442, a443) {
                                        f47(v2, v3, v4)?
                                    } else {
                                        if f48(v0) {
                                            let a444 = v0;
                                            let a445 = 33u64;
                                            if if if nat_eq(a444, a445) {
                                                let a447 = match v4.clone() {
                                                    Adt2::C0(h446, _, _, _, _, _, _, _, _, _, _) => {
                                                        h446
                                                    }
                                                };
                                                let a448 = 0u64;
                                                nat_eq(a447, a448)
                                            } else {
                                                false
                                            } {
                                                f49(match v2.clone() {
                                                    Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                                                        h449
                                                    }
                                                })
                                            } else {
                                                false
                                            } {
                                                f50(v2, v3, v4)?
                                            } else {
                                                f51(v0, v2, {
                                                    let m450 = v3;
                                                    match m450 {
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
                                            let a451 = v0;
                                            let a452 = 19u64;
                                            if nat_eq(a451, a452) {
                                                f52(v2, v4)?
                                            } else {
                                                let a453 = v0;
                                                let a454 = 21u64;
                                                if nat_eq(a453, a454) {
                                                    f53(v5, v2, v3, v4)?
                                                } else {
                                                    let a455 = v0;
                                                    let a456 = 61u64;
                                                    if if if nat_eq(a455, a456) {
                                                        true
                                                    } else {
                                                        let a457 = v0;
                                                        let a458 = 62u64;
                                                        nat_eq(a457, a458)
                                                    } {
                                                        true
                                                    } else {
                                                        let a459 = v0;
                                                        let a460 = 63u64;
                                                        nat_eq(a459, a460)
                                                    } {
                                                        f54(v0, v1, v2, v4)?
                                                    } else {
                                                        let a461 = v0;
                                                        let a462 = 4u64;
                                                        if if nat_eq(a461, a462) {
                                                            true
                                                        } else {
                                                            let a463 = v0;
                                                            let a464 = 32u64;
                                                            nat_eq(a463, a464)
                                                        } {
                                                            f55(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a465 = v0;
                                                            let a466 = 20u64;
                                                            if nat_eq(a465, a466) {
                                                                f56(v2, v4)?
                                                            } else {
                                                                let a467 = v0;
                                                                let a468 = 42u64;
                                                                if nat_eq(a467, a468) {
                                                                    f57(4u64, v2, v4)?
                                                                } else {
                                                                    let a469 = v0;
                                                                    let a470 = 69u64;
                                                                    if nat_eq(a469, a470) {
                                                                        let a471 = v5;
                                                                        let a472 = 4u64;
                                                                        if nat_lt(a471, a472) {
                                                                            f57(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a473 = v0;
                                                                        let a474 = 70u64;
                                                                        if nat_eq(a473, a474) {
                                                                            f58(v5, v2, v4)?
                                                                        } else {
                                                                            let a475 = v0;
                                                                            let a476 = 75u64;
                                                                            if if nat_eq(a475, a476) {
                                                                                true
                                                                            } else {
                                                                                let a477 = v0;
                                                                                let a478 = 76u64;
                                                                                nat_eq(a477, a478)
                                                                            } {
                                                                                f59({
                                                                                    let a479 = v0;
                                                                                    let a480 = 76u64;
                                                                                    nat_eq(a479, a480)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a481 = v0;
                                                                                let a482 = 31u64;
                                                                                if if if nat_eq(a481, a482) {
                                                                                    true
                                                                                } else {
                                                                                    let a483 = v0;
                                                                                    let a484 = 47u64;
                                                                                    nat_eq(a483, a484)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a485 = v0;
                                                                                    let a486 = 68u64;
                                                                                    nat_eq(a485, a486)
                                                                                } {
                                                                                    f60(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a487 = v0;
                                                                                    let a488 = 15u64;
                                                                                    if nat_eq(a487, a488) {
                                                                                        f61(v2, v4)?
                                                                                    } else {
                                                                                        let a489 = v0;
                                                                                        let a490 = 17u64;
                                                                                        if nat_eq(a489, a490) {
                                                                                            f62(v2, v4)?
                                                                                        } else {
                                                                                            let a491 = v0;
                                                                                            let a492 = 18u64;
                                                                                            if nat_eq(a491, a492) {
                                                                                                f63(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a493 = v0;
                                                                                                let a494 = 13u64;
                                                                                                if if if nat_eq(a493, a494) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a495 = v0;
                                                                                                    let a496 = 14u64;
                                                                                                    nat_eq(a495, a496)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a497 = v0;
                                                                                                    let a498 = 16u64;
                                                                                                    nat_eq(a497, a498)
                                                                                                } {
                                                                                                    f64(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m499 = f65(v0, v5)?;
                                                                                                    match m499 {
                                                                                                        None => {
                                                                                                            f66(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f67(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h500, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h500
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h501, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h501
        }
    };
    let v6: u64 = {
        let m504 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h503, _, _, _) => {
                h503
            }
        };
        match m504 {
            Adt6::C0(h502, _, _, _) => {
                h502
            }
        }
    };
    let v7: u64 = {
        let a505 = v3;
        let a506 = v5;
        let a507 = 0u64;
        nat_rem(a505, a506, a507)
    };
    let a508 = v1;
    let a509 = 0u64;
    if nat_eq(a508, a509) {
        let a511 = v3;
        let a512 = match v0.clone() {
            Adt8::C0(_, _, _, h510, _, _, _, _, _, _, _, _, _, _, _) => {
                h510
            }
        };
        if nat_lt(a511, a512) {
            Ok(Adt9::C8(Some(f68(match v0 {
                Adt8::C0(_, _, _, _, _, _, _, _, h513, _, _, _, _, _, _) => {
                    h513
                }
            }, {
                let a514 = v7;
                let a515 = v6;
                let a516 = 0u64;
                nat_rem(a514, a515, a516)
            }, {
                let a517 = v7;
                let a518 = v6;
                let a519 = 0u64;
                nat_quot(a517, a518, a519)
            }, {
                let a520 = v3;
                let a521 = v5;
                let a522 = 0u64;
                nat_quot(a520, a521, a522)
            }, 0u64)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v8: u64 = {
            let a528 = v2;
            let a529 = {
                let a525 = v3;
                let a526 = {
                    let a523 = v1;
                    let a524 = 1u64;
                    if nat_eq(a523, a524) {
                        v4
                    } else {
                        v5
                    }
                };
                let a527 = 0u64;
                nat_quot(a525, a526, a527)
            };
            nat_add(a528, a529)?
        };
        let a531 = v8;
        let a532 = match v0.clone() {
            Adt8::C0(h530, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h530
            }
        };
        if nat_lt(a531, a532) {
            let a533 = v1;
            let a534 = 1u64;
            if nat_eq(a533, a534) {
                Ok(Adt9::C8(Some(f68(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h535, _, _, _, _, _) => {
                        h535
                    }
                }, v8, {
                    let a536 = v3;
                    let a537 = v4;
                    let a538 = 0u64;
                    nat_rem(a536, a537, a538)
                }, 0u64, 0u64)?)))
            } else {
                let a539 = v1;
                let a540 = 2u64;
                if nat_eq(a539, a540) {
                    Ok(Adt9::C8(Some({
                        let a559 = {
                            let a547 = {
                                let a541 = v7;
                                let a542 = v6;
                                let a543 = 0u64;
                                nat_quot(a541, a542, a543)
                            };
                            let a548 = {
                                let m546 = match v0.clone() {
                                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, h545, _, _, _, _) => {
                                        h545
                                    }
                                };
                                match m546 {
                                    Adt3::C0(h544, _, _, _, _, _, _, _) => {
                                        h544
                                    }
                                }
                            };
                            nat_mul(a547, a548)?
                        };
                        let a560 = {
                            let a557 = {
                                let a552 = v8;
                                let a553 = {
                                    let m551 = match v0 {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h550, _, _, _) => {
                                            h550
                                        }
                                    };
                                    match m551 {
                                        Adt6::C0(_, h549, _, _) => {
                                            h549
                                        }
                                    }
                                };
                                nat_mul(a552, a553)?
                            };
                            let a558 = {
                                let a554 = v7;
                                let a555 = v6;
                                let a556 = 0u64;
                                nat_rem(a554, a555, a556)
                            };
                            nat_add(a557, a558)?
                        };
                        nat_add(a559, a560)?
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
    let a561 = v1;
    let a562 = 0u64;
    if nat_eq(a561, a562) {
        let a564 = v3;
        let a565 = match v0.clone() {
            Adt8::C0(_, _, _, h563, _, _, _, _, _, _, _, _, _, _, _) => {
                h563
            }
        };
        if nat_lt(a564, a565) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h566, _, _, _, _, _, _) => {
                    h566
                }
            };
            let v5: u64 = {
                let a568 = v3;
                let a569 = match v0.clone() {
                    Adt8::C0(_, _, h567, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h567
                    }
                };
                let a570 = 0u64;
                nat_quot(a568, a569, a570)
            };
            Ok(Adt9::C8(Some(f68(v4.clone(), f69(v5, v4.clone(), 0u64), f69(v5, v4.clone(), 1u64), f69(v5, v4, 2u64), f70(v0.clone(), v5, {
                let a572 = v3;
                let a573 = match v0 {
                    Adt8::C0(_, _, h571, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h571
                    }
                };
                let a574 = 0u64;
                nat_rem(a572, a573, a574)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h575, _, _, _, _) => {
                h575
            }
        };
        let v7: u64 = {
            let a576 = v1;
            let a577 = 1u64;
            if nat_eq(a576, a577) {
                match v0.clone() {
                    Adt8::C0(_, h578, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h578
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h579, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h579
                    }
                }
            }
        };
        let v8: u64 = {
            let a583 = v2;
            let a584 = {
                let a580 = v3;
                let a581 = v7;
                let a582 = 0u64;
                nat_quot(a580, a581, a582)
            };
            nat_add(a583, a584)?
        };
        let v9: u64 = {
            let a585 = v3;
            let a586 = v7;
            let a587 = 0u64;
            nat_rem(a585, a586, a587)
        };
        let v10: u64 = {
            let a589 = v8;
            let a590 = match v6.clone() {
                Adt3::C0(h588, _, _, _, _, _, _, _) => {
                    h588
                }
            };
            let a591 = 0u64;
            nat_rem(a589, a590, a591)
        };
        let v11: u64 = {
            let a593 = v8;
            let a594 = match v6.clone() {
                Adt3::C0(h592, _, _, _, _, _, _, _) => {
                    h592
                }
            };
            let a595 = 0u64;
            nat_quot(a593, a594, a595)
        };
        let v12: u64 = {
            let a597 = v11;
            let a598 = match v6.clone() {
                Adt3::C0(_, h596, _, _, _, _, _, _) => {
                    h596
                }
            };
            let a599 = 0u64;
            nat_rem(a597, a598, a599)
        };
        let v13: u64 = {
            let a601 = v11;
            let a602 = match v6.clone() {
                Adt3::C0(_, h600, _, _, _, _, _, _) => {
                    h600
                }
            };
            let a603 = 0u64;
            nat_quot(a601, a602, a603)
        };
        let a605 = v8;
        let a606 = match v0.clone() {
            Adt8::C0(h604, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h604
            }
        };
        if nat_lt(a605, a606) {
            let a607 = v1;
            let a608 = 1u64;
            if nat_eq(a607, a608) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h609, _, _, _, _, _, _) => {
                        h609
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h610, _, _, _, _, _) => {
                        h610
                    }
                };
                let m613 = f71(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h611, _, _, _) => {
                        h611
                    }
                }, match v15.clone() {
                    Adt3::C0(h612, _, _, _, _, _, _, _) => {
                        h612
                    }
                }, v10, f69(v9, v14.clone(), 0u64))?;
                match m613 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m616 = f71(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h614, _, _) => {
                                h614
                            }
                        }, match v15 {
                            Adt3::C0(_, h615, _, _, _, _, _, _) => {
                                h615
                            }
                        }, v12, f69(v9, v14, 1u64))?;
                        match m616 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f72(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a617 = v1;
                let a618 = 2u64;
                if nat_eq(a617, a618) {
                    Ok(Adt9::C8(Some({
                        let a620 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h619) => {
                                h619
                            }
                        };
                        let a621 = 0u64;
                        if nat_eq(a620, a621) {
                            f68(v6, v10, v12, v9, v13)?
                        } else {
                            f68(v6.clone(), v10, v12, {
                                let a623 = v13;
                                let a624 = match v6.clone() {
                                    Adt3::C0(_, _, h622, _, _, _, _, _) => {
                                        h622
                                    }
                                };
                                let a625 = 0u64;
                                nat_rem(a623, a624, a625)
                            }, {
                                let a633 = {
                                    let a631 = {
                                        let a627 = v13;
                                        let a628 = match v6 {
                                            Adt3::C0(_, _, h626, _, _, _, _, _) => {
                                                h626
                                            }
                                        };
                                        let a629 = 0u64;
                                        nat_quot(a627, a628, a629)
                                    };
                                    let a632 = match v0 {
                                        Adt8::C0(_, _, h630, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h630
                                        }
                                    };
                                    nat_mul(a631, a632)?
                                };
                                let a634 = v9;
                                nat_add(a633, a634)?
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
    Ok(f73(v3.clone(), {
        let m635 = v3.clone();
        match m635 {
            Adt4::C0 => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f76(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C19(v51, v52) => {
                f77(v51, v0, v1, v2, v52, v4)?
            }
            Adt4::C20(v53, v54) => {
                f78(v53, v0, v1, v2, v54, v4)?
            }
            Adt4::C21 => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C22(v55, v56) => {
                f79(v0, v1, v2, v55, v56, v4)?
            }
            Adt4::C23(v70, v71, v72, v73) => {
                f86(v0, v73, v70, v71, v72, v4)?
            }
            Adt4::C24(v69) => {
                if v5 {
                    f84(Adt16::C0(v69), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f74(v0, v1, v2, v3)?
            }
            Adt4::C26(v65, v66, v67, v68) => {
                let a636 = v65;
                let a637 = 64u64;
                if if if nat_eq(a636, a637) {
                    true
                } else {
                    let a638 = v65;
                    let a639 = 65u64;
                    nat_eq(a638, a639)
                } {
                    true
                } else {
                    let a640 = v65;
                    let a641 = 66u64;
                    nat_eq(a640, a641)
                } {
                    if v5 {
                        f84(Adt16::C1(v65, v66, v67, v68), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a642 = v65;
                    let a643 = 0u64;
                    if if nat_eq(a642, a643) {
                        true
                    } else {
                        let a644 = v65;
                        let a645 = 30u64;
                        nat_eq(a644, a645)
                    } {
                        f85(v0, v1, v2, v65, v66, v67, v68, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v64) => {
                f83(v0, v1, v2, v64)?
            }
            Adt4::C28(v60) => {
                f80(v0, v1, v2, v60, v4)?
            }
            Adt4::C29(v61) => {
                f81(v0, v1, v2, v61)?
            }
            Adt4::C30(v62, _) => {
                f82(v0, v1, v2, v62)?
            }
            Adt4::C31(v33, v34) => {
                f75(v0, v1, v2, v33, v34)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m646 = v0;
    match m646 {
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
                let a651 = {
                    let a647 = 0u64;
                    let a648 = v47;
                    if nat_lt(a647, a648) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a652 = {
                    let a649 = 0u64;
                    let a650 = v49;
                    if nat_lt(a649, a650) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a651, a652)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v55, _, _, _) => {
            Ok(Adt9::C6({
                let a653 = v55;
                let a654 = 66u64;
                if nat_eq(a653, a654) {
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
    let a655 = v1;
    let a656 = 5u64;
    if if if if nat_lt(a655, a656) {
        let a657 = v5;
        let a658 = 5u64;
        nat_lt(a657, a658)
    } else {
        false
    } {
        let a659 = v7;
        let a660 = 5u64;
        nat_lt(a659, a660)
    } else {
        false
    } {
        let a661 = v9;
        let a662 = 2u64;
        if nat_lt(a661, a662) {
            true
        } else {
            let a663 = v9;
            let a664 = 3u64;
            nat_eq(a663, a664)
        }
    } else {
        false
    } {
        Adt9::C7(f87(v0, v1, v2, v3, v4), f88(v5, v6), f88(v7, v8), f89({
            let a665 = v9;
            let a666 = 0u64;
            if nat_eq(a665, a666) {
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
    let a678 = {
        let a672 = {
            let a668 = match v0.clone() {
                Adt2::C0(_, h667, _, _, _, _, _, _, _, _, _) => {
                    h667
                }
            };
            let a669 = v1;
            let a670 = 0u64;
            nat_quot(a668, a669, a670)
        };
        let a673 = match v0.clone() {
            Adt2::C0(_, _, h671, _, _, _, _, _, _, _, _) => {
                h671
            }
        };
        nat_mul(a672, a673)?
    };
    let a679 = {
        let a676 = match v0.clone() {
            Adt2::C0(_, _, _, h674, _, _, _, _, _, _, _) => {
                h674
            }
        };
        let a677 = match v0 {
            Adt2::C0(_, _, _, _, h675, _, _, _, _, _, _) => {
                h675
            }
        };
        nat_mul(a676, a677)?
    };
    nat_mul(a678, a679)
}

pub fn f17(v0: u64) -> bool {
    let a680 = v0;
    let a681 = 4294967296u64;
    if nat_lt(a680, a681) {
        let a685 = {
            let a682 = v0;
            let a683 = 2147483648u64;
            let a684 = 0u64;
            nat_rem(a682, a683, a684)
        };
        let a686 = 2139095040u64;
        nat_lt(a685, a686)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a687 = 4u64;
    let a688 = v0;
    nat_mul(a687, a688)
}

pub fn f19(v0: u64) -> R<u64> {
    let a689 = v0;
    let a690 = 0u64;
    if nat_eq(a689, a690) {
        Ok(0u64)
    } else {
        let a693 = 4u64;
        let a694 = {
            let a691 = 3u64;
            let a692 = v0;
            nat_mul(a691, a692)?
        };
        nat_add(a693, a694)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a696 = match v0.clone() {
            Adt2::C0(_, h695, _, _, _, _, _, _, _, _, _) => {
                h695
            }
        };
        let a697 = v1;
        let a698 = 0u64;
        nat_quot(a696, a697, a698)
    }, match v0.clone() {
        Adt2::C0(_, _, h699, _, _, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h700, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h701, _, _, _, _, _, _) => {
            h701
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h702, _, _, _, _, _) => {
            h702
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h703, _, _, _, _) => {
            h703
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h704, _, _, _) => {
            h704
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h705, _, _) => {
            h705
        }
    })
}

pub fn f21(v0: Adt2) -> R<bool> {
    let a707 = match v0.clone() {
        Adt2::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
            h706
        }
    };
    let a708 = 1u64;
    if if if if nat_eq(a707, a708) {
        true
    } else {
        let a710 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h709, _, _, _, _, _) => {
                h709
            }
        };
        let a711 = 4u64;
        nat_eq(a710, a711)
    } {
        let a713 = match v0.clone() {
            Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                h712
            }
        };
        let a714 = 1u64;
        if nat_eq(a713, a714) {
            true
        } else {
            let a719 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h715, _, _, _, _) => {
                    h715
                }
            };
            let a720 = {
                let a717 = match v0.clone() {
                    Adt2::C0(_, h716, _, _, _, _, _, _, _, _, _) => {
                        h716
                    }
                };
                let a718 = 4u64;
                nat_mul(a717, a718)?
            };
            nat_eq(a719, a720)
        }
    } else {
        false
    } {
        let a722 = match v0.clone() {
            Adt2::C0(_, _, _, h721, _, _, _, _, _, _, _) => {
                h721
            }
        };
        let a723 = 1u64;
        if nat_eq(a722, a723) {
            true
        } else {
            let a731 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h724, _, _, _) => {
                    h724
                }
            };
            let a732 = {
                let a729 = {
                    let a727 = match v0.clone() {
                        Adt2::C0(_, h725, _, _, _, _, _, _, _, _, _) => {
                            h725
                        }
                    };
                    let a728 = match v0.clone() {
                        Adt2::C0(_, _, h726, _, _, _, _, _, _, _, _) => {
                            h726
                        }
                    };
                    nat_mul(a727, a728)?
                };
                let a730 = 4u64;
                nat_mul(a729, a730)?
            };
            nat_eq(a731, a732)
        }
    } else {
        false
    } {
        let a734 = match v0.clone() {
            Adt2::C0(_, _, _, _, h733, _, _, _, _, _, _) => {
                h733
            }
        };
        let a735 = 1u64;
        if nat_eq(a734, a735) {
            Ok(true)
        } else {
            let a746 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h736, _, _) => {
                    h736
                }
            };
            let a747 = {
                let a744 = {
                    let a742 = {
                        let a739 = match v0.clone() {
                            Adt2::C0(_, h737, _, _, _, _, _, _, _, _, _) => {
                                h737
                            }
                        };
                        let a740 = match v0.clone() {
                            Adt2::C0(_, _, h738, _, _, _, _, _, _, _, _) => {
                                h738
                            }
                        };
                        nat_mul(a739, a740)?
                    };
                    let a743 = match v0 {
                        Adt2::C0(_, _, _, h741, _, _, _, _, _, _, _) => {
                            h741
                        }
                    };
                    nat_mul(a742, a743)?
                };
                let a745 = 4u64;
                nat_mul(a744, a745)?
            };
            Ok(nat_eq(a746, a747))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> R<bool> {
    let a749 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h748, _) => {
            h748
        }
    };
    let a750 = 0u64;
    if nat_eq(a749, a750) {
        Ok(true)
    } else {
        let a752 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h751, _) => {
                h751
            }
        };
        let a753 = 0u64;
        if nat_eq(a752, a753) {
            Ok(true)
        } else {
            let a755 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h754) => {
                    h754
                }
            };
            let a756 = 0u64;
            if nat_eq(a755, a756) {
                Ok(true)
            } else {
                let a758 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h757) => {
                        h757
                    }
                };
                let a759 = 0u64;
                if nat_eq(a758, a759) {
                    Ok(true)
                } else {
                    let a765 = {
                        let a762 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h760) => {
                                h760
                            }
                        };
                        let a763 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h761, _) => {
                                h761
                            }
                        };
                        nat_add(a762, a763)?
                    };
                    let a766 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h764) => {
                            h764
                        }
                    };
                    if nat_le(a765, a766) {
                        Ok(true)
                    } else {
                        let a772 = {
                            let a769 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h767) => {
                                    h767
                                }
                            };
                            let a770 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h768, _) => {
                                    h768
                                }
                            };
                            nat_add(a769, a770)?
                        };
                        let a773 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h771) => {
                                h771
                            }
                        };
                        Ok(nat_le(a772, a773))
                    }
                }
            }
        }
    }
}

pub fn f23(v0: Adt6, v1: u64) -> R<u64> {
    let a775 = match v0.clone() {
        Adt6::C0(h774, _, _, _) => {
            h774
        }
    };
    let a776 = 0u64;
    if nat_eq(a775, a776) {
        f90(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h777, _, _) => {
                h777
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h778, _) => {
                h778
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h779) => {
                h779
            }
        }), {
            let a781 = v1;
            let a782 = match v0 {
                Adt6::C0(_, _, _, h780) => {
                    h780
                }
            };
            nat_add(a781, a782)?
        })
    } else {
        f90(v0, v1)
    }
}

pub fn f24(v0: Adt2, v1: Adt2) -> bool {
    let a785 = match v0.clone() {
        Adt2::C0(_, h783, _, _, _, _, _, _, _, _, _) => {
            h783
        }
    };
    let a786 = match v1.clone() {
        Adt2::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
            h784
        }
    };
    if if if nat_eq(a785, a786) {
        let a789 = match v0.clone() {
            Adt2::C0(_, _, h787, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a790 = match v1.clone() {
            Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                h788
            }
        };
        nat_eq(a789, a790)
    } else {
        false
    } {
        let a793 = match v0.clone() {
            Adt2::C0(_, _, _, h791, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v1.clone() {
            Adt2::C0(_, _, _, h792, _, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_eq(a793, a794)
    } else {
        false
    } {
        let a797 = match v0 {
            Adt2::C0(_, _, _, _, h795, _, _, _, _, _, _) => {
                h795
            }
        };
        let a798 = match v1 {
            Adt2::C0(_, _, _, _, h796, _, _, _, _, _, _) => {
                h796
            }
        };
        nat_eq(a797, a798)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a803 = {
        let a799 = v0;
        let a800 = v1;
        nat_mul(a799, a800)?
    };
    let a804 = {
        let a801 = v2;
        let a802 = f91(v3);
        nat_mul(a801, a802)?
    };
    nat_mul(a803, a804)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a807 = v0;
    let a808 = {
        let a805 = f92(v1)?;
        let a806 = v2;
        nat_mul(a805, a806)?
    };
    nat_add(a807, a808)
}

pub fn f27(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h809, _, _, _, _, _, _, _, _, _, _) => {
            h809
        }
    }, match v0.clone() {
        Adt2::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
            h810
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h811, _, _, _, _, _, _, _, _) => {
            h811
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h812, _, _, _, _, _, _, _) => {
            h812
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h813, _, _, _, _, _, _) => {
            h813
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h814, _, _, _, _, _) => {
            h814
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h815, _, _, _, _) => {
            h815
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h816, _, _, _) => {
            h816
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h817, _, _) => {
            h817
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h818, _) => {
            h818
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h819) => {
            h819
        }
    })
}

pub fn f28(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a820 = v0;
    let a821 = 23u64;
    if nat_eq(a820, a821) {
        f93(v1, v4, v6)
    } else {
        let a822 = v0;
        let a823 = 26u64;
        if nat_eq(a822, a823) {
            f94(v1, v2, v4, v6)
        } else {
            let a824 = v0;
            let a825 = 25u64;
            if nat_eq(a824, a825) {
                f95(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f29(v0: u64) -> u64 {
    let a826 = v0;
    let a827 = 30u64;
    if nat_eq(a826, a827) {
        2u64
    } else {
        4u64
    }
}

pub fn f30(v0: u64) -> bool {
    let a828 = v0;
    let a829 = 2139095040u64;
    if nat_le(a828, a829) {
        true
    } else {
        let a830 = v0;
        let a831 = 2147483648u64;
        nat_eq(a830, a831)
    }
}

pub fn f31(v0: Adt2, v1: u64) -> bool {
    let a833 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h832, _, _, _, _, _) => {
            h832
        }
    };
    let a834 = v1;
    if if nat_eq(a833, a834) {
        let a839 = {
            let a836 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h835) => {
                    h835
                }
            };
            let a837 = v1;
            let a838 = 0u64;
            nat_rem(a836, a837, a838)
        };
        let a840 = 0u64;
        nat_eq(a839, a840)
    } else {
        false
    } {
        f96(match v0.clone() {
            Adt2::C0(_, _, h841, _, _, _, _, _, _, _, _) => {
                h841
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h842, _, _, _, _, _, _, _) => {
                h842
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h843, _, _, _, _, _, _) => {
                h843
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h844, _, _, _, _) => {
                h844
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h845, _, _, _) => {
                h845
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h846, _, _) => {
                h846
            }
        }, v1)
    } else {
        false
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a849 = match v0.clone() {
            Adt2::C0(h847, _, _, _, _, _, _, _, _, _, _) => {
                h847
            }
        };
        let a850 = match v1.clone() {
            Adt2::C0(h848, _, _, _, _, _, _, _, _, _, _) => {
                h848
            }
        };
        nat_eq(a849, a850)
    } else {
        false
    } {
        let a853 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h851) => {
                h851
            }
        };
        let a854 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h852) => {
                h852
            }
        };
        nat_eq(a853, a854)
    } else {
        false
    } {
        let a857 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h855, _, _, _, _, _) => {
                h855
            }
        };
        let a858 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h856, _, _, _, _, _) => {
                h856
            }
        };
        nat_eq(a857, a858)
    } else {
        false
    } {
        let a861 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h859, _, _, _, _) => {
                h859
            }
        };
        let a862 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h860, _, _, _, _) => {
                h860
            }
        };
        nat_eq(a861, a862)
    } else {
        false
    } {
        let a865 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h863, _, _, _) => {
                h863
            }
        };
        let a866 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h864, _, _, _) => {
                h864
            }
        };
        nat_eq(a865, a866)
    } else {
        false
    } {
        let a869 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h867, _, _) => {
                h867
            }
        };
        let a870 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h868, _, _) => {
                h868
            }
        };
        nat_eq(a869, a870)
    } else {
        false
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a871 = v0;
    let a872 = 0u64;
    if if if nat_eq(a871, a872) {
        true
    } else {
        let a873 = v1;
        let a874 = 0u64;
        nat_eq(a873, a874)
    } {
        true
    } else {
        let a875 = v2;
        let a876 = 0u64;
        nat_eq(a875, a876)
    } {
        true
    } else {
        let a877 = v3;
        let a878 = 0u64;
        nat_eq(a877, a878)
    }
}

pub fn f34(v0: Adt2, v1: u64) -> R<bool> {
    f97({
        let a880 = match v0.clone() {
            Adt2::C0(_, h879, _, _, _, _, _, _, _, _, _) => {
                h879
            }
        };
        let a881 = v1;
        nat_mul(a880, a881)?
    }, match v0.clone() {
        Adt2::C0(_, _, h882, _, _, _, _, _, _, _, _) => {
            h882
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h883, _, _, _, _, _, _, _) => {
            h883
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h884, _, _, _, _, _, _) => {
            h884
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h885, _, _, _, _) => {
            h885
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h886, _, _, _) => {
            h886
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h887, _, _) => {
            h887
        }
    })
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a888 = v1;
    let a889 = 0u64;
    if if nat_eq(a888, a889) {
        true
    } else {
        let a890 = v2;
        let a891 = 0u64;
        nat_eq(a890, a891)
    } {
        Ok(0u64)
    } else {
        let a898 = {
            let a892 = f98(v1, v2, v3)?;
            let a893 = f99(v0, v1, v2)?;
            nat_add(a892, a893)?
        };
        let a899 = {
            let a896 = f100(v0);
            let a897 = {
                let a894 = v1;
                let a895 = v2;
                nat_mul(a894, a895)?
            };
            nat_mul(a896, a897)?
        };
        nat_add(a898, a899)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a900 = v0;
    let a901 = 0u64;
    if if nat_eq(a900, a901) {
        true
    } else {
        let a902 = v1;
        let a903 = 0u64;
        nat_eq(a902, a903)
    } {
        Ok(3u64)
    } else {
        let a906 = {
            let a904 = 3u64;
            let a905 = v1;
            nat_add(a904, a905)?
        };
        let a907 = v2;
        nat_add(a906, a907)
    }
}

pub fn f37(v0: u64) -> bool {
    let a908 = v0;
    let a909 = 2139095040u64;
    if nat_le(a908, a909) {
        true
    } else {
        let a910 = v0;
        let a911 = 2147483648u64;
        nat_eq(a910, a911)
    }
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let a912 = v0;
    let a913 = 0u64;
    if nat_eq(a912, a913) {
        Ok(0u64)
    } else {
        let a914 = v1;
        let a915 = f101(v0)?;
        nat_mul(a914, a915)
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a916 = v0;
    let a917 = 0u64;
    if if if if nat_eq(a916, a917) {
        true
    } else {
        let a918 = v1;
        let a919 = 0u64;
        nat_eq(a918, a919)
    } {
        true
    } else {
        let a920 = v2;
        let a921 = 0u64;
        nat_eq(a920, a921)
    } {
        true
    } else {
        let a922 = v3;
        let a923 = 0u64;
        nat_eq(a922, a923)
    } {
        Ok(0u64)
    } else {
        let a936 = {
            let a926 = 2u64;
            let a927 = {
                let a924 = v1;
                let a925 = v2;
                nat_mul(a924, a925)?
            };
            nat_mul(a926, a927)?
        };
        let a937 = {
            let a934 = {
                let a930 = {
                    let a928 = v1;
                    let a929 = v2;
                    nat_mul(a928, a929)?
                };
                let a931 = v3;
                nat_mul(a930, a931)?
            };
            let a935 = {
                let a932 = 1u64;
                let a933 = v0;
                nat_add(a932, a933)?
            };
            nat_mul(a934, a935)?
        };
        nat_add(a936, a937)
    }
}

pub fn f40(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h938, _, _, _, _, _, _, _, _, _, _, _) => {
            h938
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h939, _, _, _, _, _, _, _, _, _, _) => {
            h939
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h940, _, _, _, _, _, _, _, _, _) => {
            h940
        }
    };
    let v6: u64 = {
        let a942 = match v1.clone() {
            Adt2::C0(_, h941, _, _, _, _, _, _, _, _, _) => {
                h941
            }
        };
        let a943 = v4;
        let a944 = 0u64;
        nat_quot(a942, a943, a944)
    };
    let v7: u64 = {
        let a946 = match v1.clone() {
            Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a947 = 0u64;
        if nat_eq(a946, a947) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f102(match v1.clone() {
        Adt2::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
            h948
        }
    }) {
        let a951 = match v1.clone() {
            Adt2::C0(h949, _, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        let a952 = match v2.clone() {
            Adt2::C0(h950, _, _, _, _, _, _, _, _, _, _) => {
                h950
            }
        };
        nat_eq(a951, a952)
    } else {
        false
    } {
        let a953 = 0u64;
        let a954 = v3;
        nat_lt(a953, a954)
    } else {
        false
    } {
        let a955 = v3;
        let a956 = 2147483648u64;
        nat_lt(a955, a956)
    } else {
        false
    } {
        let a957 = 0u64;
        let a958 = v4;
        nat_lt(a957, a958)
    } else {
        false
    } {
        let a959 = v4;
        let a960 = 2147483648u64;
        nat_lt(a959, a960)
    } else {
        false
    } {
        let a961 = v5;
        let a962 = 2147483648u64;
        nat_lt(a961, a962)
    } else {
        false
    } {
        let a963 = 0u64;
        let a964 = v6;
        nat_lt(a963, a964)
    } else {
        false
    } {
        let a966 = 0u64;
        let a967 = match v2.clone() {
            Adt2::C0(_, h965, _, _, _, _, _, _, _, _, _) => {
                h965
            }
        };
        nat_lt(a966, a967)
    } else {
        false
    } {
        let a971 = match v1.clone() {
            Adt2::C0(_, h968, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        };
        let a972 = {
            let a969 = v6;
            let a970 = v4;
            nat_mul(a969, a970)?
        };
        nat_eq(a971, a972)
    } else {
        false
    } {
        let a974 = match v2.clone() {
            Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                h973
            }
        };
        let a975 = v4;
        nat_eq(a974, a975)
    } else {
        false
    } {
        let a988 = {
            let a981 = {
                let a979 = match v2.clone() {
                    Adt2::C0(_, h976, _, _, _, _, _, _, _, _, _) => {
                        h976
                    }
                };
                let a980 = {
                    let a977 = v5;
                    let a978 = 2u64;
                    nat_mul(a977, a978)?
                };
                nat_add(a979, a980)?
            };
            let a982 = v3;
            nat_add(a981, a982)?
        };
        let a989 = {
            let a986 = {
                let a984 = match v1.clone() {
                    Adt2::C0(_, _, h983, _, _, _, _, _, _, _, _) => {
                        h983
                    }
                };
                let a985 = v3;
                nat_mul(a984, a985)?
            };
            let a987 = v6;
            nat_add(a986, a987)?
        };
        nat_eq(a988, a989)
    } else {
        false
    } {
        f103(v1.clone(), v7)?
    } else {
        false
    } {
        f103(v2.clone(), v7)?
    } else {
        false
    } {
        f22(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a990 = f16(v2.clone(), 1u64)?;
            let a991 = f104(v8.clone())?;
            nat_mul(a990, a991)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h992, _, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f41(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m993 = v1;
    match m993 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a995 = match v0.clone() {
                Adt2::C0(h994, _, _, _, _, _, _, _, _, _, _) => {
                    h994
                }
            };
            let a996 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a995, a996) {
                let a998 = match v3.clone() {
                    Adt2::C0(h997, _, _, _, _, _, _, _, _, _, _) => {
                        h997
                    }
                };
                let a999 = 0u64;
                nat_eq(a998, a999)
            } else {
                false
            } {
                let a1001 = match v2.clone() {
                    Adt2::C0(h1000, _, _, _, _, _, _, _, _, _, _) => {
                        h1000
                    }
                };
                let a1002 = 0u64;
                nat_eq(a1001, a1002)
            } else {
                false
            } {
                let a1004 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1003, _, _, _, _, _, _) => {
                        h1003
                    }
                };
                let a1005 = 1u64;
                nat_eq(a1004, a1005)
            } else {
                false
            } {
                let a1007 = match v3.clone() {
                    Adt2::C0(_, _, _, h1006, _, _, _, _, _, _, _) => {
                        h1006
                    }
                };
                let a1008 = 1u64;
                nat_eq(a1007, a1008)
            } else {
                false
            } {
                let a1010 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1009, _, _, _, _, _, _) => {
                        h1009
                    }
                };
                let a1011 = 1u64;
                nat_eq(a1010, a1011)
            } else {
                false
            } {
                let a1014 = match v0.clone() {
                    Adt2::C0(_, _, h1012, _, _, _, _, _, _, _, _) => {
                        h1012
                    }
                };
                let a1015 = match v3.clone() {
                    Adt2::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                };
                nat_eq(a1014, a1015)
            } else {
                false
            } {
                let a1020 = match v3.clone() {
                    Adt2::C0(_, h1016, _, _, _, _, _, _, _, _, _) => {
                        h1016
                    }
                };
                let a1021 = {
                    let a1018 = match v0.clone() {
                        Adt2::C0(_, h1017, _, _, _, _, _, _, _, _, _) => {
                            h1017
                        }
                    };
                    let a1019 = 1u64;
                    nat_add(a1018, a1019)?
                };
                nat_le(a1020, a1021)
            } else {
                false
            } {
                let a1024 = match v2.clone() {
                    Adt2::C0(_, h1022, _, _, _, _, _, _, _, _, _) => {
                        h1022
                    }
                };
                let a1025 = match v0.clone() {
                    Adt2::C0(_, _, h1023, _, _, _, _, _, _, _, _) => {
                        h1023
                    }
                };
                nat_eq(a1024, a1025)
            } else {
                false
            } {
                let a1033 = match v2.clone() {
                    Adt2::C0(_, _, h1026, _, _, _, _, _, _, _, _) => {
                        h1026
                    }
                };
                let a1034 = {
                    let a1031 = {
                        let a1028 = match v0.clone() {
                            Adt2::C0(_, h1027, _, _, _, _, _, _, _, _, _) => {
                                h1027
                            }
                        };
                        let a1029 = 1u64;
                        nat_add(a1028, a1029)?
                    };
                    let a1032 = match v3.clone() {
                        Adt2::C0(_, h1030, _, _, _, _, _, _, _, _, _) => {
                            h1030
                        }
                    };
                    nat_sub(a1031, a1032)
                };
                nat_eq(a1033, a1034)
            } else {
                false
            } {
                let a1037 = match v2.clone() {
                    Adt2::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                        h1035
                    }
                };
                let a1038 = match v0.clone() {
                    Adt2::C0(_, _, _, h1036, _, _, _, _, _, _, _) => {
                        h1036
                    }
                };
                nat_eq(a1037, a1038)
            } else {
                false
            } {
                let a1040 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1039, _, _, _, _, _, _) => {
                        h1039
                    }
                };
                let a1041 = 1u64;
                nat_eq(a1040, a1041)
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                let a1043 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1042, _, _, _, _, _) => {
                        h1042
                    }
                };
                let a1044 = 4u64;
                nat_eq(a1043, a1044)
            } else {
                false
            } {
                let a1046 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1045, _, _, _, _, _) => {
                        h1045
                    }
                };
                let a1047 = 4u64;
                nat_eq(a1046, a1047)
            } else {
                false
            } {
                let a1052 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1048, _, _, _, _) => {
                        h1048
                    }
                };
                let a1053 = {
                    let a1050 = match v0.clone() {
                        Adt2::C0(_, h1049, _, _, _, _, _, _, _, _, _) => {
                            h1049
                        }
                    };
                    let a1051 = 4u64;
                    nat_mul(a1050, a1051)?
                };
                nat_eq(a1052, a1053)
            } else {
                false
            } {
                let a1058 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1054, _, _, _, _) => {
                        h1054
                    }
                };
                let a1059 = {
                    let a1056 = match v3.clone() {
                        Adt2::C0(_, h1055, _, _, _, _, _, _, _, _, _) => {
                            h1055
                        }
                    };
                    let a1057 = 4u64;
                    nat_mul(a1056, a1057)?
                };
                nat_eq(a1058, a1059)
            } else {
                false
            } {
                f22(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f22(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1061 = f16(v2.clone(), 1u64)?;
                    let a1062 = f105(match v3.clone() {
                        Adt2::C0(_, h1060, _, _, _, _, _, _, _, _, _) => {
                            h1060
                        }
                    })?;
                    nat_mul(a1061, a1062)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f42(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1063, _, _, _, _, _, _, _, _, _, _) => {
            h1063
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1064, _, _, _, _, _, _, _, _) => {
                h1064
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1065, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1066, _, _, _, _, _, _) => {
                h1066
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1067, _, _, _, _, _, _, _, _) => {
                h1067
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1068, _, _, _, _, _, _, _, _, _) => {
                h1068
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1069, _, _, _, _, _, _, _) => {
                h1069
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1070, _, _, _, _, _) => {
                h1070
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1072 = match v1.clone() {
        Adt1::C0(h1071, _, _, _, _, _, _, _, _, _, _, _) => {
            h1071
        }
    };
    let a1073 = 1u64;
    if if if if if if if if if if if nat_le(a1072, a1073) {
        let a1075 = match v2.clone() {
            Adt2::C0(h1074, _, _, _, _, _, _, _, _, _, _) => {
                h1074
            }
        };
        let a1076 = 1u64;
        nat_le(a1075, a1076)
    } else {
        false
    } {
        let a1078 = match v3.clone() {
            Adt2::C0(h1077, _, _, _, _, _, _, _, _, _, _) => {
                h1077
            }
        };
        let a1079 = 0u64;
        nat_eq(a1078, a1079)
    } else {
        false
    } {
        let a1084 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1080, _, _, _, _, _) => {
                h1080
            }
        };
        let a1085 = {
            let a1082 = match v2.clone() {
                Adt2::C0(h1081, _, _, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            };
            let a1083 = 0u64;
            if nat_eq(a1082, a1083) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1084, a1085)
    } else {
        false
    } {
        f21(v3.clone())?
    } else {
        false
    } {
        f22(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f106(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1086, _, _, _, _, _, _, _, _, _) => {
                h1086
            }
        }, match v3.clone() {
            Adt2::C0(_, h1087, _, _, _, _, _, _, _, _, _) => {
                h1087
            }
        })?
    } else {
        false
    } {
        if v0 {
            f106(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1088, _, _, _, _, _, _, _, _) => {
                    h1088
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1089, _, _, _, _, _, _, _, _) => {
                    h1089
                }
            })?
        } else {
            let a1092 = match v2.clone() {
                Adt2::C0(_, _, h1090, _, _, _, _, _, _, _, _) => {
                    h1090
                }
            };
            let a1093 = match v3.clone() {
                Adt2::C0(_, _, h1091, _, _, _, _, _, _, _, _) => {
                    h1091
                }
            };
            nat_eq(a1092, a1093)
        }
    } else {
        false
    } {
        let a1096 = match v2.clone() {
            Adt2::C0(_, _, _, h1094, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1097 = match v3.clone() {
            Adt2::C0(_, _, _, h1095, _, _, _, _, _, _, _) => {
                h1095
            }
        };
        nat_eq(a1096, a1097)
    } else {
        false
    } {
        let a1100 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1098, _, _, _, _, _, _) => {
                h1098
            }
        };
        let a1101 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1099, _, _, _, _, _, _) => {
                h1099
            }
        };
        nat_eq(a1100, a1101)
    } else {
        false
    } {
        let a1106 = {
            let a1104 = match v4.clone() {
                Adt6::C0(h1102, _, _, _) => {
                    h1102
                }
            };
            let a1105 = match v5.clone() {
                Adt6::C0(h1103, _, _, _) => {
                    h1103
                }
            };
            nat_mul(a1104, a1105)?
        };
        let a1107 = 2147483648u64;
        nat_lt(a1106, a1107)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1109 = match v1 {
                Adt1::C0(h1108, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1108
                }
            };
            let a1110 = 0u64;
            nat_eq(a1109, a1110)
        }, match v2.clone() {
            Adt2::C0(h1111, _, _, _, _, _, _, _, _, _, _) => {
                h1111
            }
        });
        Ok(Adt10::C4({
            let a1112 = f16(v3.clone(), 1u64)?;
            let a1113 = f107(v6.clone())?;
            nat_mul(a1112, a1113)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f43(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1114 = v2;
    match m1114 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1116 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1115, _, _, _, _, _) => {
                        h1115
                    }
                };
                let a1117 = 1u64;
                nat_eq(a1116, a1117)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1118, _, _, _, _, _, _, _, _, _) => {
                    h1118
                }
            }, match v0.clone() {
                Adt1::C0(h1119, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1119
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1120, _, _, _, _, _, _, _, _, _) => {
                    h1120
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1121, _, _, _, _, _, _, _) => {
                    h1121
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1122, _, _, _, _, _, _, _, _) => {
                        h1122
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1123, _, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1124, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1125, _, _, _, _, _, _) => {
                        h1125
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1126, _, _, _, _, _, _, _) => {
                        h1126
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                        h1127
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1128, _, _, _, _, _, _) => {
                        h1128
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1129, _, _, _, _, _, _, _) => {
                        h1129
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1130, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
                    h1131
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1132, _, _, _, _, _, _, _, _, _, _) => {
                    h1132
                }
            };
            let v14: u64 = {
                let a1133 = v12;
                let a1134 = 0u64;
                if nat_eq(a1133, a1134) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1136 = match v4.clone() {
                    Adt2::C0(_, h1135, _, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                };
                let a1137 = v14;
                nat_mul(a1136, a1137)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1138, _, _, _) => {
                        h1138
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1139, _, _, _, _) => {
                        h1139
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1140, _, _) => {
                        h1140
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1141, _, _, _) => {
                        h1141
                    }
                }
            };
            let a1143 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1142, _, _, _, _, _) => {
                    h1142
                }
            };
            let a1144 = 1u64;
            if if if if if if if if if if nat_le(a1143, a1144) {
                let a1145 = v12;
                let a1146 = 0u64;
                if nat_eq(a1145, a1146) {
                    true
                } else {
                    let a1147 = v12;
                    let a1148 = 1u64;
                    if nat_eq(a1147, a1148) {
                        let a1149 = v13;
                        let a1150 = 1u64;
                        nat_eq(a1149, a1150)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1151 = v13;
                let a1152 = 0u64;
                if nat_eq(a1151, a1152) {
                    true
                } else {
                    let a1153 = v13;
                    let a1154 = 1u64;
                    nat_eq(a1153, a1154)
                }
            } else {
                false
            } {
                let a1156 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1155, _, _, _, _, _) => {
                        h1155
                    }
                };
                let a1157 = v14;
                nat_eq(a1156, a1157)
            } else {
                false
            } {
                let a1165 = match v3.clone() {
                    Adt2::C0(_, h1158, _, _, _, _, _, _, _, _, _) => {
                        h1158
                    }
                };
                let a1166 = {
                    let a1163 = v9;
                    let a1164 = {
                        let a1161 = match v6.clone() {
                            Adt6::C0(h1159, _, _, _) => {
                                h1159
                            }
                        };
                        let a1162 = match v7.clone() {
                            Adt6::C0(h1160, _, _, _) => {
                                h1160
                            }
                        };
                        nat_mul(a1161, a1162)?
                    };
                    nat_mul(a1163, a1164)?
                };
                nat_eq(a1165, a1166)
            } else {
                false
            } {
                let a1169 = match v3.clone() {
                    Adt2::C0(_, _, h1167, _, _, _, _, _, _, _, _) => {
                        h1167
                    }
                };
                let a1170 = f90(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1168, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                })?;
                nat_eq(a1169, a1170)
            } else {
                false
            } {
                let a1172 = 0u64;
                let a1173 = match v3.clone() {
                    Adt2::C0(_, _, h1171, _, _, _, _, _, _, _, _) => {
                        h1171
                    }
                };
                nat_lt(a1172, a1173)
            } else {
                false
            } {
                if v5 {
                    let a1175 = match v1 {
                        Adt2::C0(_, _, _, h1174, _, _, _, _, _, _, _) => {
                            h1174
                        }
                    };
                    let a1176 = v9;
                    if if if nat_eq(a1175, a1176) {
                        let a1178 = match v3.clone() {
                            Adt2::C0(_, _, _, h1177, _, _, _, _, _, _, _) => {
                                h1177
                            }
                        };
                        let a1179 = f90(v7.clone(), v11)?;
                        nat_eq(a1178, a1179)
                    } else {
                        false
                    } {
                        let a1181 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1180, _, _, _, _, _, _) => {
                                h1180
                            }
                        };
                        let a1182 = v10;
                        nat_eq(a1181, a1182)
                    } else {
                        false
                    } {
                        let a1184 = 0u64;
                        let a1185 = match v3.clone() {
                            Adt2::C0(_, _, _, h1183, _, _, _, _, _, _, _) => {
                                h1183
                            }
                        };
                        nat_lt(a1184, a1185)
                    } else {
                        false
                    }
                } else {
                    let a1187 = match v1 {
                        Adt2::C0(_, _, h1186, _, _, _, _, _, _, _, _) => {
                            h1186
                        }
                    };
                    let a1188 = v9;
                    if if if nat_eq(a1187, a1188) {
                        let a1190 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1189, _, _, _, _, _, _) => {
                                h1189
                            }
                        };
                        let a1191 = 1u64;
                        nat_eq(a1190, a1191)
                    } else {
                        false
                    } {
                        let a1193 = match v3.clone() {
                            Adt2::C0(_, _, _, h1192, _, _, _, _, _, _, _) => {
                                h1192
                            }
                        };
                        let a1194 = v10;
                        nat_eq(a1193, a1194)
                    } else {
                        false
                    } {
                        let a1196 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1195, _, _, _, _, _, _) => {
                                h1195
                            }
                        };
                        let a1197 = 1u64;
                        nat_eq(a1196, a1197)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f33(match v4.clone() {
                    Adt2::C0(_, h1198, _, _, _, _, _, _, _, _, _) => {
                        h1198
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1218 = {
                        let a1215 = {
                            let a1213 = {
                                let a1207 = {
                                    let a1201 = {
                                        let a1199 = v10;
                                        let a1200 = 1u64;
                                        nat_sub(a1199, a1200)
                                    };
                                    let a1202 = v17;
                                    nat_mul(a1201, a1202)?
                                };
                                let a1208 = {
                                    let a1205 = {
                                        let a1203 = v9;
                                        let a1204 = 1u64;
                                        nat_sub(a1203, a1204)
                                    };
                                    let a1206 = v16;
                                    nat_mul(a1205, a1206)?
                                };
                                nat_add(a1207, a1208)?
                            };
                            let a1214 = {
                                let a1211 = {
                                    let a1209 = v11;
                                    let a1210 = 1u64;
                                    nat_sub(a1209, a1210)
                                };
                                let a1212 = v15;
                                nat_mul(a1211, a1212)?
                            };
                            nat_add(a1213, a1214)?
                        };
                        let a1216 = v15;
                        nat_add(a1215, a1216)?
                    };
                    let a1219 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1217, _) => {
                            h1217
                        }
                    };
                    nat_le(a1218, a1219)
                }
            } else {
                false
            } {
                f22(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1220 = v13;
                    let a1221 = 0u64;
                    if nat_eq(a1220, a1221) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1222, _, _, _, _, _, _, _, _, _) => {
                        h1222
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1223, _, _, _, _, _, _, _, _, _) => {
                            h1223
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1224, _, _, _, _, _, _, _, _) => {
                            h1224
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1225, _, _, _, _, _, _, _) => {
                            h1225
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1226, _, _, _, _, _) => {
                            h1226
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1227, _, _, _, _) => {
                            h1227
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1228, _, _, _) => {
                            h1228
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f44(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1229 = v2;
    match m1229 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                    h1230
                }
            }, match v0.clone() {
                Adt1::C0(h1231, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1231
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1232, _, _, _, _, _, _, _, _) => {
                    h1232
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1233, _, _, _, _, _) => {
                    h1233
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1234, _, _, _, _, _, _, _, _) => {
                    h1234
                }
            }, match v0.clone() {
                Adt1::C0(_, h1235, _, _, _, _, _, _, _, _, _, _) => {
                    h1235
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1236, _, _, _, _, _, _, _) => {
                    h1236
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1237, _, _, _, _) => {
                    h1237
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1238, _, _, _, _, _, _, _) => {
                    h1238
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1239, _, _, _, _, _, _, _, _, _) => {
                    h1239
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1240, _, _, _, _, _, _) => {
                    h1240
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1241, _, _, _) => {
                    h1241
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1242, _, _) => {
                    h1242
                }
            };
            let v9: u64 = f90(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1243, _, _, _, _, _, _, _) => {
                    h1243
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                    h1244
                }
            };
            let v11: u64 = {
                let a1252 = v8;
                let a1253 = {
                    let a1250 = match v5.clone() {
                        Adt6::C0(h1245, _, _, _) => {
                            h1245
                        }
                    };
                    let a1251 = {
                        let a1248 = match v6.clone() {
                            Adt6::C0(h1246, _, _, _) => {
                                h1246
                            }
                        };
                        let a1249 = match v7.clone() {
                            Adt6::C0(h1247, _, _, _) => {
                                h1247
                            }
                        };
                        nat_mul(a1248, a1249)?
                    };
                    nat_mul(a1250, a1251)?
                };
                nat_mul(a1252, a1253)?
            };
            let a1255 = match v4.clone() {
                Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                    h1254
                }
            };
            let a1256 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1255, a1256) {
                let a1257 = v10;
                let a1258 = 0u64;
                if nat_eq(a1257, a1258) {
                    true
                } else {
                    let a1259 = v10;
                    let a1260 = 1u64;
                    nat_eq(a1259, a1260)
                }
            } else {
                false
            } {
                let a1261 = 0u64;
                let a1262 = v8;
                nat_lt(a1261, a1262)
            } else {
                false
            } {
                let a1263 = v8;
                let a1264 = 2147483648u64;
                nat_lt(a1263, a1264)
            } else {
                false
            } {
                let a1266 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1265, _, _, _, _, _) => {
                        h1265
                    }
                };
                let a1267 = 4u64;
                nat_eq(a1266, a1267)
            } else {
                false
            } {
                let a1268 = 0u64;
                let a1269 = v9;
                nat_lt(a1268, a1269)
            } else {
                false
            } {
                let a1271 = 0u64;
                let a1272 = match v3.clone() {
                    Adt2::C0(_, _, h1270, _, _, _, _, _, _, _, _) => {
                        h1270
                    }
                };
                nat_lt(a1271, a1272)
            } else {
                false
            } {
                let a1274 = 0u64;
                let a1275 = match v3.clone() {
                    Adt2::C0(_, _, _, h1273, _, _, _, _, _, _, _) => {
                        h1273
                    }
                };
                nat_lt(a1274, a1275)
            } else {
                false
            } {
                let a1277 = match v3.clone() {
                    Adt2::C0(_, h1276, _, _, _, _, _, _, _, _, _) => {
                        h1276
                    }
                };
                let a1278 = v11;
                nat_eq(a1277, a1278)
            } else {
                false
            } {
                let a1281 = match v3.clone() {
                    Adt2::C0(_, _, h1279, _, _, _, _, _, _, _, _) => {
                        h1279
                    }
                };
                let a1282 = f90(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1280, _, _, _, _, _, _, _, _, _) => {
                        h1280
                    }
                })?;
                nat_eq(a1281, a1282)
            } else {
                false
            } {
                let a1285 = match v3.clone() {
                    Adt2::C0(_, _, _, h1283, _, _, _, _, _, _, _) => {
                        h1283
                    }
                };
                let a1286 = f90(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1284, _, _, _, _, _, _, _, _) => {
                        h1284
                    }
                })?;
                nat_eq(a1285, a1286)
            } else {
                false
            } {
                let a1294 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1287, _, _, _, _, _, _) => {
                        h1287
                    }
                };
                let a1295 = {
                    let a1292 = v9;
                    let a1293 = {
                        let a1289 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1288, _, _, _, _, _, _) => {
                                h1288
                            }
                        };
                        let a1290 = v8;
                        let a1291 = 0u64;
                        nat_quot(a1289, a1290, a1291)
                    };
                    nat_mul(a1292, a1293)?
                };
                nat_eq(a1294, a1295)
            } else {
                false
            } {
                f22(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1296 = v10;
                    let a1297 = 0u64;
                    if nat_eq(a1296, a1297) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1298, _, _) => {
                        h1298
                    }
                }, {
                    let a1300 = v8;
                    let a1301 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1299, _, _) => {
                            h1299
                        }
                    };
                    nat_mul(a1300, a1301)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f45(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1302 = v1;
    match m1302 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f108(v0, v3, v2)
        }
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1304 = match v0.clone() {
        Adt2::C0(h1303, _, _, _, _, _, _, _, _, _, _) => {
            h1303
        }
    };
    let a1305 = 1u64;
    if if nat_eq(a1304, a1305) {
        true
    } else {
        let a1307 = match v0.clone() {
            Adt2::C0(h1306, _, _, _, _, _, _, _, _, _, _) => {
                h1306
            }
        };
        let a1308 = 30u64;
        nat_eq(a1307, a1308)
    } {
        let a1310 = match v1.clone() {
            Adt2::C0(h1309, _, _, _, _, _, _, _, _, _, _) => {
                h1309
            }
        };
        let a1311 = 1u64;
        if if if if if if if if if if if nat_eq(a1310, a1311) {
            let a1314 = match v0.clone() {
                Adt2::C0(_, h1312, _, _, _, _, _, _, _, _, _) => {
                    h1312
                }
            };
            let a1315 = match v1.clone() {
                Adt2::C0(_, h1313, _, _, _, _, _, _, _, _, _) => {
                    h1313
                }
            };
            nat_eq(a1314, a1315)
        } else {
            false
        } {
            let a1317 = 0u64;
            let a1318 = match v1.clone() {
                Adt2::C0(_, _, h1316, _, _, _, _, _, _, _, _) => {
                    h1316
                }
            };
            nat_lt(a1317, a1318)
        } else {
            false
        } {
            let a1321 = match v1.clone() {
                Adt2::C0(_, _, h1319, _, _, _, _, _, _, _, _) => {
                    h1319
                }
            };
            let a1322 = match v1.clone() {
                Adt2::C0(_, _, _, h1320, _, _, _, _, _, _, _) => {
                    h1320
                }
            };
            nat_eq(a1321, a1322)
        } else {
            false
        } {
            let a1324 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1323, _, _, _, _, _, _) => {
                    h1323
                }
            };
            let a1325 = 1u64;
            nat_eq(a1324, a1325)
        } else {
            false
        } {
            let a1333 = match v0.clone() {
                Adt2::C0(_, _, h1326, _, _, _, _, _, _, _, _) => {
                    h1326
                }
            };
            let a1334 = {
                let a1331 = {
                    let a1329 = match v1.clone() {
                        Adt2::C0(_, _, h1327, _, _, _, _, _, _, _, _) => {
                            h1327
                        }
                    };
                    let a1330 = match v1.clone() {
                        Adt2::C0(_, _, h1328, _, _, _, _, _, _, _, _) => {
                            h1328
                        }
                    };
                    nat_add(a1329, a1330)?
                };
                let a1332 = 1u64;
                nat_sub(a1331, a1332)
            };
            nat_eq(a1333, a1334)
        } else {
            false
        } {
            let a1336 = 0u64;
            let a1337 = match v0.clone() {
                Adt2::C0(_, _, _, h1335, _, _, _, _, _, _, _) => {
                    h1335
                }
            };
            nat_lt(a1336, a1337)
        } else {
            false
        } {
            let a1339 = 0u64;
            let a1340 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1338, _, _, _, _, _, _) => {
                    h1338
                }
            };
            nat_lt(a1339, a1340)
        } else {
            false
        } {
            let a1342 = 2u64;
            let a1343 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1341, _, _, _, _, _) => {
                    h1341
                }
            };
            nat_le(a1342, a1343)
        } else {
            false
        } {
            let a1345 = 2u64;
            let a1346 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1344, _, _, _, _, _) => {
                    h1344
                }
            };
            nat_le(a1345, a1346)
        } else {
            false
        } {
            f22(v0.clone(), v1.clone())?
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

pub fn f47(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1347 = v1;
    match m1347 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f109(v0, v3, v2)
        }
    }
}

pub fn f48(v0: u64) -> bool {
    let a1348 = v0;
    let a1349 = 1u64;
    if if nat_eq(a1348, a1349) {
        true
    } else {
        let a1350 = v0;
        let a1351 = 33u64;
        nat_eq(a1350, a1351)
    } {
        true
    } else {
        let a1352 = v0;
        let a1353 = 34u64;
        nat_eq(a1352, a1353)
    }
}

pub fn f49(v0: u64) -> bool {
    if f110(v0) {
        !f111(v0)
    } else {
        false
    }
}

pub fn f50(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1354 = v1;
    match m1354 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1356 = f112(match v0.clone() {
                Adt2::C0(h1355, _, _, _, _, _, _, _, _, _, _) => {
                    h1355
                }
            });
            match m1356 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1357, _) => {
                            h1357
                        }
                    };
                    if if if if if if f32(v3, v2.clone()) {
                        f22(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1358 = f16(v0.clone(), 1u64)?;
                        let a1359 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1358, a1359)
                    } else {
                        false
                    } {
                        let a1364 = {
                            let a1361 = match v0.clone() {
                                Adt2::C0(_, h1360, _, _, _, _, _, _, _, _, _) => {
                                    h1360
                                }
                            };
                            let a1362 = v5;
                            let a1363 = 1u64;
                            nat_rem(a1361, a1362, a1363)
                        };
                        let a1365 = 0u64;
                        if nat_eq(a1364, a1365) {
                            let a1368 = match v4 {
                                Adt18::C0(_, _, h1366) => {
                                    h1366
                                }
                            };
                            let a1369 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1367, _, _, _, _, _) => {
                                    h1367
                                }
                            };
                            nat_le(a1368, a1369)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1370 = v5;
                        let a1371 = 1u64;
                        if nat_eq(a1370, a1371) {
                            let a1373 = 4u64;
                            let a1374 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1372, _, _, _, _, _) => {
                                    h1372
                                }
                            };
                            nat_le(a1373, a1374)
                        } else {
                            let a1376 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1375, _, _, _, _, _) => {
                                    h1375
                                }
                            };
                            let a1377 = 4u64;
                            nat_eq(a1376, a1377)
                        }
                    } else {
                        false
                    } {
                        let a1382 = {
                            let a1379 = match v2.clone() {
                                Adt2::C0(_, h1378, _, _, _, _, _, _, _, _, _) => {
                                    h1378
                                }
                            };
                            let a1380 = v5;
                            let a1381 = 1u64;
                            nat_rem(a1379, a1380, a1381)
                        };
                        let a1383 = 0u64;
                        if nat_eq(a1382, a1383) {
                            true
                        } else {
                            f21(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1384, _, _, _, _, _, _, _, _, _, _) => {
                                h1384
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

pub fn f51(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1387 = match v1.clone() {
        Adt2::C0(h1385, _, _, _, _, _, _, _, _, _, _) => {
            h1385
        }
    };
    let a1388 = match v3.clone() {
        Adt2::C0(h1386, _, _, _, _, _, _, _, _, _, _) => {
            h1386
        }
    };
    if nat_eq(a1387, a1388) {
        f113(v0, v1, v2, v3)
    } else {
        if if if if f111(match v1.clone() {
            Adt2::C0(h1389, _, _, _, _, _, _, _, _, _, _) => {
                h1389
            }
        }) {
            f111(match v3.clone() {
                Adt2::C0(h1390, _, _, _, _, _, _, _, _, _, _) => {
                    h1390
                }
            })
        } else {
            false
        } {
            let a1391 = v0;
            let a1392 = 33u64;
            if nat_eq(a1391, a1392) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1393 = f16(v1.clone(), 1u64)?;
            let a1394 = f16(v3.clone(), 1u64)?;
            nat_eq(a1393, a1394)
        } else {
            false
        } {
            let m1396 = f112(match v3.clone() {
                Adt2::C0(h1395, _, _, _, _, _, _, _, _, _, _) => {
                    h1395
                }
            });
            match m1396 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1397, _, _, _, _, _, _, _, _, _, _) => {
                            h1397
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1398, _, _, _, _, _, _, _, _, _, _) => {
                            h1398
                        }
                    };
                    let v7: bool = {
                        let a1399 = v5;
                        let a1400 = 43u64;
                        if nat_eq(a1399, a1400) {
                            true
                        } else {
                            let a1401 = v6;
                            let a1402 = 43u64;
                            nat_eq(a1401, a1402)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1403 = f16(v3.clone(), 1u64)?;
                        let a1404 = if v7 {
                            f114(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1403, a1404)?
                    }, match v4 {
                        Adt18::C0(_, _, h1405) => {
                            h1405
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

pub fn f52(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1407 = match v0.clone() {
        Adt2::C0(h1406, _, _, _, _, _, _, _, _, _, _) => {
            h1406
        }
    };
    let a1408 = 0u64;
    if if if if nat_eq(a1407, a1408) {
        true
    } else {
        let a1410 = match v0.clone() {
            Adt2::C0(h1409, _, _, _, _, _, _, _, _, _, _) => {
                h1409
            }
        };
        let a1411 = 1u64;
        nat_eq(a1410, a1411)
    } {
        true
    } else {
        let a1413 = match v0.clone() {
            Adt2::C0(h1412, _, _, _, _, _, _, _, _, _, _) => {
                h1412
            }
        };
        let a1414 = 25u64;
        if nat_eq(a1413, a1414) {
            true
        } else {
            let a1416 = match v0.clone() {
                Adt2::C0(h1415, _, _, _, _, _, _, _, _, _, _) => {
                    h1415
                }
            };
            let a1417 = 26u64;
            nat_eq(a1416, a1417)
        }
    } {
        true
    } else {
        let a1419 = match v0.clone() {
            Adt2::C0(h1418, _, _, _, _, _, _, _, _, _, _) => {
                h1418
            }
        };
        let a1420 = 30u64;
        nat_eq(a1419, a1420)
    } {
        let a1423 = match v0.clone() {
            Adt2::C0(h1421, _, _, _, _, _, _, _, _, _, _) => {
                h1421
            }
        };
        let a1424 = match v1.clone() {
            Adt2::C0(h1422, _, _, _, _, _, _, _, _, _, _) => {
                h1422
            }
        };
        if if nat_eq(a1423, a1424) {
            f115(match v1.clone() {
                Adt2::C0(_, h1425, _, _, _, _, _, _, _, _, _) => {
                    h1425
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1426, _, _, _, _, _, _, _, _) => {
                    h1426
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1427, _, _, _, _, _, _, _) => {
                    h1427
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1428, _, _, _, _, _, _) => {
                    h1428
                }
            }, match v0.clone() {
                Adt2::C0(_, h1429, _, _, _, _, _, _, _, _, _) => {
                    h1429
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1430, _, _, _, _, _, _, _, _) => {
                    h1430
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1431, _, _, _, _, _, _, _) => {
                    h1431
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1432, _, _, _, _, _, _) => {
                    h1432
                }
            })
        } else {
            false
        } {
            let m1434 = f112(match v0.clone() {
                Adt2::C0(h1433, _, _, _, _, _, _, _, _, _, _) => {
                    h1433
                }
            });
            match m1434 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1435) => {
                            h1435
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

pub fn f53(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1436 = v2;
    match m1436 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1437 = v0;
            let a1438 = 4u64;
            if if if nat_lt(a1437, a1438) {
                let a1441 = match v1.clone() {
                    Adt2::C0(h1439, _, _, _, _, _, _, _, _, _, _) => {
                        h1439
                    }
                };
                let a1442 = match v4.clone() {
                    Adt2::C0(h1440, _, _, _, _, _, _, _, _, _, _) => {
                        h1440
                    }
                };
                if nat_eq(a1441, a1442) {
                    let a1445 = match v1.clone() {
                        Adt2::C0(h1443, _, _, _, _, _, _, _, _, _, _) => {
                            h1443
                        }
                    };
                    let a1446 = match v3.clone() {
                        Adt2::C0(h1444, _, _, _, _, _, _, _, _, _, _) => {
                            h1444
                        }
                    };
                    nat_eq(a1445, a1446)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f116(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1447, _, _, _, _, _, _, _, _, _) => {
                        h1447
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1448, _, _, _, _, _, _, _, _, _) => {
                        h1448
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1449, _, _, _, _, _, _, _, _, _) => {
                        h1449
                    }
                })? {
                    f116(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1450, _, _, _, _, _, _, _, _) => {
                            h1450
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1451, _, _, _, _, _, _, _, _) => {
                            h1451
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1452, _, _, _, _, _, _, _, _) => {
                            h1452
                        }
                    })?
                } else {
                    false
                } {
                    f116(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1453, _, _, _, _, _, _, _) => {
                            h1453
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1454, _, _, _, _, _, _, _) => {
                            h1454
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1455, _, _, _, _, _, _, _) => {
                            h1455
                        }
                    })?
                } else {
                    false
                } {
                    f116(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1456, _, _, _, _, _, _) => {
                            h1456
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1457, _, _, _, _, _, _) => {
                            h1457
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1458, _, _, _, _, _, _) => {
                            h1458
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1460 = f112(match v1.clone() {
                    Adt2::C0(h1459, _, _, _, _, _, _, _, _, _, _) => {
                        h1459
                    }
                });
                match m1460 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1461, _) => {
                                h1461
                            }
                        };
                        let a1466 = {
                            let a1463 = match v1.clone() {
                                Adt2::C0(_, h1462, _, _, _, _, _, _, _, _, _) => {
                                    h1462
                                }
                            };
                            let a1464 = v6;
                            let a1465 = 1u64;
                            nat_rem(a1463, a1464, a1465)
                        };
                        let a1467 = 0u64;
                        if if if nat_eq(a1466, a1467) {
                            let a1472 = {
                                let a1469 = match v4.clone() {
                                    Adt2::C0(_, h1468, _, _, _, _, _, _, _, _, _) => {
                                        h1468
                                    }
                                };
                                let a1470 = v6;
                                let a1471 = 1u64;
                                nat_rem(a1469, a1470, a1471)
                            };
                            let a1473 = 0u64;
                            nat_eq(a1472, a1473)
                        } else {
                            false
                        } {
                            let a1478 = {
                                let a1475 = match v3.clone() {
                                    Adt2::C0(_, h1474, _, _, _, _, _, _, _, _, _) => {
                                        h1474
                                    }
                                };
                                let a1476 = v6;
                                let a1477 = 1u64;
                                nat_rem(a1475, a1476, a1477)
                            };
                            let a1479 = 0u64;
                            nat_eq(a1478, a1479)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1480) => {
                                    h1480
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

pub fn f54(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1482 = match v2.clone() {
        Adt2::C0(h1481, _, _, _, _, _, _, _, _, _, _) => {
            h1481
        }
    };
    let a1483 = 0u64;
    if if nat_eq(a1482, a1483) {
        let a1485 = match v3.clone() {
            Adt2::C0(h1484, _, _, _, _, _, _, _, _, _, _) => {
                h1484
            }
        };
        let a1486 = 0u64;
        nat_eq(a1485, a1486)
    } else {
        false
    } {
        let a1487 = v0;
        let a1488 = 61u64;
        if nat_eq(a1487, a1488) {
            let v4: bool = {
                let a1490 = 0u64;
                let a1491 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1489, _, _, _) => {
                        h1489
                    }
                };
                nat_lt(a1490, a1491)
            };
            if if if if if if f117(match v2.clone() {
                Adt2::C0(_, h1492, _, _, _, _, _, _, _, _, _) => {
                    h1492
                }
            }, match v1.clone() {
                Adt1::C0(h1493, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1493
                }
            }, match v1.clone() {
                Adt1::C0(_, h1494, _, _, _, _, _, _, _, _, _, _) => {
                    h1494
                }
            }, match v3.clone() {
                Adt2::C0(_, h1495, _, _, _, _, _, _, _, _, _) => {
                    h1495
                }
            })? {
                f117(match v2.clone() {
                    Adt2::C0(_, _, h1496, _, _, _, _, _, _, _, _) => {
                        h1496
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1497, _, _, _, _, _, _, _, _, _) => {
                        h1497
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1498, _, _, _, _, _, _, _, _) => {
                        h1498
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1499, _, _, _, _, _, _, _, _) => {
                        h1499
                    }
                })?
            } else {
                false
            } {
                f117(match v2.clone() {
                    Adt2::C0(_, _, _, h1500, _, _, _, _, _, _, _) => {
                        h1500
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1501, _, _, _, _, _, _, _) => {
                        h1501
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1502, _, _, _, _, _, _) => {
                        h1502
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1503, _, _, _, _, _, _, _) => {
                        h1503
                    }
                })?
            } else {
                false
            } {
                f117(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1504, _, _, _, _, _, _) => {
                        h1504
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1505, _, _, _, _, _) => {
                        h1505
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1506, _, _, _, _) => {
                        h1506
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1507, _, _, _, _, _, _) => {
                        h1507
                    }
                })?
            } else {
                false
            } {
                let a1509 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1508, _, _, _) => {
                        h1508
                    }
                };
                let a1510 = 4294967295u64;
                nat_le(a1509, a1510)
            } else {
                false
            } {
                if v4 {
                    !f33(match v2.clone() {
                        Adt2::C0(_, h1511, _, _, _, _, _, _, _, _, _) => {
                            h1511
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1512, _, _, _, _, _, _, _, _) => {
                            h1512
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1513, _, _, _, _, _, _, _) => {
                            h1513
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1514, _, _, _, _, _, _) => {
                            h1514
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1515, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1515
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1516, _, _, _, _, _, _, _, _, _) => {
                        h1516
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1517, _, _, _, _, _, _, _) => {
                        h1517
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1518, _, _, _, _, _) => {
                        h1518
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1519 = v0;
            let a1520 = 62u64;
            if nat_eq(a1519, a1520) {
                let a1522 = match v1.clone() {
                    Adt1::C0(h1521, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1521
                    }
                };
                let a1523 = 2147483648u64;
                if if if if if nat_lt(a1522, a1523) {
                    let a1525 = match v1.clone() {
                        Adt1::C0(_, h1524, _, _, _, _, _, _, _, _, _, _) => {
                            h1524
                        }
                    };
                    let a1526 = 2147483648u64;
                    nat_lt(a1525, a1526)
                } else {
                    false
                } {
                    let a1529 = match v1.clone() {
                        Adt1::C0(h1527, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1527
                        }
                    };
                    let a1530 = match v2.clone() {
                        Adt2::C0(_, h1528, _, _, _, _, _, _, _, _, _) => {
                            h1528
                        }
                    };
                    nat_lt(a1529, a1530)
                } else {
                    false
                } {
                    let a1533 = match v1.clone() {
                        Adt1::C0(_, h1531, _, _, _, _, _, _, _, _, _, _) => {
                            h1531
                        }
                    };
                    let a1534 = match v2.clone() {
                        Adt2::C0(_, h1532, _, _, _, _, _, _, _, _, _) => {
                            h1532
                        }
                    };
                    nat_lt(a1533, a1534)
                } else {
                    false
                } {
                    let a1537 = match v2.clone() {
                        Adt2::C0(_, _, h1535, _, _, _, _, _, _, _, _) => {
                            h1535
                        }
                    };
                    let a1538 = match v3.clone() {
                        Adt2::C0(_, _, h1536, _, _, _, _, _, _, _, _) => {
                            h1536
                        }
                    };
                    if if if nat_eq(a1537, a1538) {
                        let a1541 = match v2.clone() {
                            Adt2::C0(_, _, _, h1539, _, _, _, _, _, _, _) => {
                                h1539
                            }
                        };
                        let a1542 = match v3.clone() {
                            Adt2::C0(_, _, _, h1540, _, _, _, _, _, _, _) => {
                                h1540
                            }
                        };
                        nat_eq(a1541, a1542)
                    } else {
                        false
                    } {
                        let a1545 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1543, _, _, _, _, _, _) => {
                                h1543
                            }
                        };
                        let a1546 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1544, _, _, _, _, _, _) => {
                                h1544
                            }
                        };
                        nat_eq(a1545, a1546)
                    } else {
                        false
                    } {
                        let a1555 = match v3.clone() {
                            Adt2::C0(_, h1547, _, _, _, _, _, _, _, _, _) => {
                                h1547
                            }
                        };
                        let a1556 = {
                            let a1553 = {
                                let a1550 = match v2.clone() {
                                    Adt2::C0(_, h1548, _, _, _, _, _, _, _, _, _) => {
                                        h1548
                                    }
                                };
                                let a1551 = match v1.clone() {
                                    Adt1::C0(h1549, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1549
                                    }
                                };
                                nat_add(a1550, a1551)?
                            };
                            let a1554 = match v1.clone() {
                                Adt1::C0(_, h1552, _, _, _, _, _, _, _, _, _, _) => {
                                    h1552
                                }
                            };
                            nat_add(a1553, a1554)?
                        };
                        nat_eq(a1555, a1556)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1557, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1557
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1558 = v0;
                let a1559 = 63u64;
                if nat_eq(a1558, a1559) {
                    if if if if if f118(match v2.clone() {
                        Adt2::C0(_, h1560, _, _, _, _, _, _, _, _, _) => {
                            h1560
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1561, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1561
                        }
                    })? {
                        f118(match v2.clone() {
                            Adt2::C0(_, _, h1562, _, _, _, _, _, _, _, _) => {
                                h1562
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1563, _, _, _, _, _, _, _, _, _, _) => {
                                h1563
                            }
                        })?
                    } else {
                        false
                    } {
                        f118(match v2.clone() {
                            Adt2::C0(_, _, _, h1564, _, _, _, _, _, _, _) => {
                                h1564
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1565, _, _, _, _, _, _, _, _, _) => {
                                h1565
                            }
                        })?
                    } else {
                        false
                    } {
                        f118(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1566, _, _, _, _, _, _) => {
                                h1566
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1567, _, _, _, _, _, _, _, _) => {
                                h1567
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1570 = match v2.clone() {
                            Adt2::C0(_, h1568, _, _, _, _, _, _, _, _, _) => {
                                h1568
                            }
                        };
                        let a1571 = match v3.clone() {
                            Adt2::C0(_, h1569, _, _, _, _, _, _, _, _, _) => {
                                h1569
                            }
                        };
                        if if if nat_eq(a1570, a1571) {
                            let a1574 = match v2.clone() {
                                Adt2::C0(_, _, h1572, _, _, _, _, _, _, _, _) => {
                                    h1572
                                }
                            };
                            let a1575 = match v3.clone() {
                                Adt2::C0(_, _, h1573, _, _, _, _, _, _, _, _) => {
                                    h1573
                                }
                            };
                            nat_eq(a1574, a1575)
                        } else {
                            false
                        } {
                            let a1578 = match v2.clone() {
                                Adt2::C0(_, _, _, h1576, _, _, _, _, _, _, _) => {
                                    h1576
                                }
                            };
                            let a1579 = match v3.clone() {
                                Adt2::C0(_, _, _, h1577, _, _, _, _, _, _, _) => {
                                    h1577
                                }
                            };
                            nat_eq(a1578, a1579)
                        } else {
                            false
                        } {
                            let a1582 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1580, _, _, _, _, _, _) => {
                                    h1580
                                }
                            };
                            let a1583 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1581, _, _, _, _, _, _) => {
                                    h1581
                                }
                            };
                            nat_eq(a1582, a1583)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1584, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1584
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1585, _, _, _, _, _, _, _, _, _, _) => {
                                h1585
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1586, _, _, _, _, _, _, _, _, _) => {
                                h1586
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1587, _, _, _, _, _, _, _, _) => {
                                h1587
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

pub fn f55(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1588 = v3;
    match m1588 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1589 = v0;
                let a1590 = 4u64;
                nat_eq(a1589, a1590)
            };
            let v7: bool = {
                let a1592 = 0u64;
                let a1593 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1591, _, _, _, _, _, _, _) => {
                        h1591
                    }
                };
                nat_lt(a1592, a1593)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                    h1594
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1595, _, _, _, _, _, _, _, _) => {
                    h1595
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1596, _, _, _, _, _, _, _) => {
                    h1596
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1597, _, _, _, _, _, _) => {
                    h1597
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1598, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1598
                }
            }, match v1.clone() {
                Adt1::C0(_, h1599, _, _, _, _, _, _, _, _, _, _) => {
                    h1599
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1600, _, _, _, _, _, _, _, _, _) => {
                    h1600
                }
            });
            if if if if if if if if if if if if v6 {
                let a1602 = match v2.clone() {
                    Adt2::C0(h1601, _, _, _, _, _, _, _, _, _, _) => {
                        h1601
                    }
                };
                let a1603 = 0u64;
                if nat_eq(a1602, a1603) {
                    let a1605 = match v5.clone() {
                        Adt2::C0(h1604, _, _, _, _, _, _, _, _, _, _) => {
                            h1604
                        }
                    };
                    let a1606 = 0u64;
                    nat_eq(a1605, a1606)
                } else {
                    false
                }
            } else {
                let a1608 = match v2.clone() {
                    Adt2::C0(h1607, _, _, _, _, _, _, _, _, _, _) => {
                        h1607
                    }
                };
                let a1609 = 0u64;
                if if nat_eq(a1608, a1609) {
                    true
                } else {
                    let a1611 = match v2.clone() {
                        Adt2::C0(h1610, _, _, _, _, _, _, _, _, _, _) => {
                            h1610
                        }
                    };
                    let a1612 = 26u64;
                    nat_eq(a1611, a1612)
                } {
                    let a1614 = match v5.clone() {
                        Adt2::C0(h1613, _, _, _, _, _, _, _, _, _, _) => {
                            h1613
                        }
                    };
                    let a1615 = 0u64;
                    if nat_eq(a1614, a1615) {
                        true
                    } else {
                        let a1617 = match v5.clone() {
                            Adt2::C0(h1616, _, _, _, _, _, _, _, _, _, _) => {
                                h1616
                            }
                        };
                        let a1618 = 26u64;
                        nat_eq(a1617, a1618)
                    }
                } else {
                    false
                }
            } {
                let a1621 = match v2.clone() {
                    Adt2::C0(h1619, _, _, _, _, _, _, _, _, _, _) => {
                        h1619
                    }
                };
                let a1622 = match v4.clone() {
                    Adt2::C0(h1620, _, _, _, _, _, _, _, _, _, _) => {
                        h1620
                    }
                };
                nat_eq(a1621, a1622)
            } else {
                false
            } {
                f24(v2.clone(), v4.clone())
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                f21(v4.clone())?
            } else {
                false
            } {
                let a1624 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1623, _, _, _, _, _) => {
                        h1623
                    }
                };
                let a1625 = 4u64;
                nat_eq(a1624, a1625)
            } else {
                false
            } {
                let a1626 = v8;
                let a1627 = f16(v2.clone(), 1u64)?;
                nat_le(a1626, a1627)
            } else {
                false
            } {
                let a1629 = match v1.clone() {
                    Adt1::C0(_, _, _, h1628, _, _, _, _, _, _, _, _) => {
                        h1628
                    }
                };
                let a1630 = 1073741824u64;
                nat_lt(a1629, a1630)
            } else {
                false
            } {
                let a1632 = match v1.clone() {
                    Adt1::C0(h1631, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1631
                    }
                };
                let a1633 = 4294967295u64;
                if if if if nat_le(a1632, a1633) {
                    let a1635 = match v1.clone() {
                        Adt1::C0(_, h1634, _, _, _, _, _, _, _, _, _, _) => {
                            h1634
                        }
                    };
                    let a1636 = 4294967295u64;
                    nat_le(a1635, a1636)
                } else {
                    false
                } {
                    let a1638 = match v1.clone() {
                        Adt1::C0(_, _, h1637, _, _, _, _, _, _, _, _, _) => {
                            h1637
                        }
                    };
                    let a1639 = 4294967295u64;
                    nat_le(a1638, a1639)
                } else {
                    false
                } {
                    let a1641 = match v1.clone() {
                        Adt1::C0(_, _, _, h1640, _, _, _, _, _, _, _, _) => {
                            h1640
                        }
                    };
                    let a1642 = 4294967295u64;
                    nat_le(a1641, a1642)
                } else {
                    false
                } {
                    let a1644 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1643, _, _, _, _, _, _, _) => {
                            h1643
                        }
                    };
                    let a1645 = 4294967295u64;
                    nat_le(a1644, a1645)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1648 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1646) => {
                            h1646
                        }
                    };
                    let a1649 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1647) => {
                            h1647
                        }
                    };
                    nat_eq(a1648, a1649)
                } else {
                    true
                }
            } else {
                false
            } {
                f119(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1650, _, _, _, _, _, _, _, _) => {
                        h1650
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1651, _) => {
                        h1651
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
                    let a1652 = v10;
                    let a1653 = v8;
                    nat_add(a1652, a1653)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1654, _, _, _, _, _, _, _, _) => {
                        h1654
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f56(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1656 = match v0.clone() {
        Adt2::C0(h1655, _, _, _, _, _, _, _, _, _, _) => {
            h1655
        }
    };
    let a1657 = 0u64;
    if if if nat_eq(a1656, a1657) {
        let a1659 = match v1.clone() {
            Adt2::C0(h1658, _, _, _, _, _, _, _, _, _, _) => {
                h1658
            }
        };
        let a1660 = 0u64;
        nat_eq(a1659, a1660)
    } else {
        false
    } {
        f115(match v0.clone() {
            Adt2::C0(_, h1661, _, _, _, _, _, _, _, _, _) => {
                h1661
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1662, _, _, _, _, _, _, _, _) => {
                h1662
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1663, _, _, _, _, _, _, _) => {
                h1663
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1664, _, _, _, _, _, _) => {
                h1664
            }
        }, match v1.clone() {
            Adt2::C0(_, h1665, _, _, _, _, _, _, _, _, _) => {
                h1665
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1666, _, _, _, _, _, _, _, _) => {
                h1666
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1667, _, _, _, _, _, _, _) => {
                h1667
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1668, _, _, _, _, _, _) => {
                h1668
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1669 = v2;
            let a1670 = f16(v0.clone(), 1u64)?;
            nat_add(a1669, a1670)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f57(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1672 = match v1.clone() {
        Adt2::C0(h1671, _, _, _, _, _, _, _, _, _, _) => {
            h1671
        }
    };
    let a1673 = 0u64;
    if if if nat_eq(a1672, a1673) {
        true
    } else {
        let a1675 = match v1.clone() {
            Adt2::C0(h1674, _, _, _, _, _, _, _, _, _, _) => {
                h1674
            }
        };
        let a1676 = 1u64;
        nat_eq(a1675, a1676)
    } {
        true
    } else {
        let a1678 = match v1.clone() {
            Adt2::C0(h1677, _, _, _, _, _, _, _, _, _, _) => {
                h1677
            }
        };
        let a1679 = 30u64;
        nat_eq(a1678, a1679)
    } {
        let a1682 = match v1.clone() {
            Adt2::C0(h1680, _, _, _, _, _, _, _, _, _, _) => {
                h1680
            }
        };
        let a1683 = match v2.clone() {
            Adt2::C0(h1681, _, _, _, _, _, _, _, _, _, _) => {
                h1681
            }
        };
        if if if if if nat_eq(a1682, a1683) {
            let a1686 = match v1.clone() {
                Adt2::C0(_, h1684, _, _, _, _, _, _, _, _, _) => {
                    h1684
                }
            };
            let a1687 = match v2.clone() {
                Adt2::C0(_, h1685, _, _, _, _, _, _, _, _, _) => {
                    h1685
                }
            };
            nat_eq(a1686, a1687)
        } else {
            false
        } {
            let a1690 = match v1.clone() {
                Adt2::C0(_, _, _, h1688, _, _, _, _, _, _, _) => {
                    h1688
                }
            };
            let a1691 = match v2.clone() {
                Adt2::C0(_, _, _, h1689, _, _, _, _, _, _, _) => {
                    h1689
                }
            };
            nat_eq(a1690, a1691)
        } else {
            false
        } {
            let a1694 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1692, _, _, _, _, _, _) => {
                    h1692
                }
            };
            let a1695 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1693, _, _, _, _, _, _) => {
                    h1693
                }
            };
            nat_eq(a1694, a1695)
        } else {
            false
        } {
            let a1696 = v0;
            let a1697 = 4u64;
            if nat_eq(a1696, a1697) {
                let a1699 = match v1.clone() {
                    Adt2::C0(_, _, h1698, _, _, _, _, _, _, _, _) => {
                        h1698
                    }
                };
                let a1700 = 1u64;
                if nat_eq(a1699, a1700) {
                    let a1703 = match v1.clone() {
                        Adt2::C0(_, h1701, _, _, _, _, _, _, _, _, _) => {
                            h1701
                        }
                    };
                    let a1704 = match v2.clone() {
                        Adt2::C0(_, _, h1702, _, _, _, _, _, _, _, _) => {
                            h1702
                        }
                    };
                    nat_eq(a1703, a1704)
                } else {
                    false
                }
            } else {
                let a1705 = v0;
                let a1706 = 4u64;
                if if nat_lt(a1705, a1706) {
                    let a1709 = match v1.clone() {
                        Adt2::C0(_, h1707, _, _, _, _, _, _, _, _, _) => {
                            h1707
                        }
                    };
                    let a1710 = match v1.clone() {
                        Adt2::C0(_, _, h1708, _, _, _, _, _, _, _, _) => {
                            h1708
                        }
                    };
                    nat_eq(a1709, a1710)
                } else {
                    false
                } {
                    let a1713 = match v1.clone() {
                        Adt2::C0(_, _, h1711, _, _, _, _, _, _, _, _) => {
                            h1711
                        }
                    };
                    let a1714 = match v2.clone() {
                        Adt2::C0(_, _, h1712, _, _, _, _, _, _, _, _) => {
                            h1712
                        }
                    };
                    nat_eq(a1713, a1714)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1716 = f112(match v1.clone() {
                Adt2::C0(h1715, _, _, _, _, _, _, _, _, _, _) => {
                    h1715
                }
            });
            match m1716 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1717) => {
                            h1717
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

pub fn f58(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f120(match v1.clone() {
        Adt2::C0(h1718, _, _, _, _, _, _, _, _, _, _) => {
            h1718
        }
    }, v0) {
        let a1721 = match v1.clone() {
            Adt2::C0(h1719, _, _, _, _, _, _, _, _, _, _) => {
                h1719
            }
        };
        let a1722 = match v2.clone() {
            Adt2::C0(h1720, _, _, _, _, _, _, _, _, _, _) => {
                h1720
            }
        };
        nat_eq(a1721, a1722)
    } else {
        false
    } {
        let a1725 = match v1.clone() {
            Adt2::C0(_, h1723, _, _, _, _, _, _, _, _, _) => {
                h1723
            }
        };
        let a1726 = match v2.clone() {
            Adt2::C0(_, h1724, _, _, _, _, _, _, _, _, _) => {
                h1724
            }
        };
        if if if nat_eq(a1725, a1726) {
            let a1729 = match v1.clone() {
                Adt2::C0(_, _, h1727, _, _, _, _, _, _, _, _) => {
                    h1727
                }
            };
            let a1730 = match v2.clone() {
                Adt2::C0(_, _, h1728, _, _, _, _, _, _, _, _) => {
                    h1728
                }
            };
            nat_eq(a1729, a1730)
        } else {
            false
        } {
            let a1733 = match v1.clone() {
                Adt2::C0(_, _, _, h1731, _, _, _, _, _, _, _) => {
                    h1731
                }
            };
            let a1734 = match v2.clone() {
                Adt2::C0(_, _, _, h1732, _, _, _, _, _, _, _) => {
                    h1732
                }
            };
            nat_eq(a1733, a1734)
        } else {
            false
        } {
            let a1737 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1735, _, _, _, _, _, _) => {
                    h1735
                }
            };
            let a1738 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1736, _, _, _, _, _, _) => {
                    h1736
                }
            };
            nat_eq(a1737, a1738)
        } else {
            false
        }
    } else {
        false
    } {
        let m1740 = f112(match v1.clone() {
            Adt2::C0(h1739, _, _, _, _, _, _, _, _, _, _) => {
                h1739
            }
        });
        match m1740 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1741) => {
                        h1741
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1742, _, _, _, _, _, _, _, _, _, _) => {
                        h1742
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f59(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1743, _, _, _, _, _, _, _, _, _, _, _) => {
                h1743
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1744, _, _, _, _, _, _, _, _, _) => {
                h1744
            }
        }
    };
    let a1745 = 0u64;
    let a1746 = v4;
    if if nat_lt(a1745, a1746) {
        let a1747 = v4;
        let a1748 = 2147483648u64;
        nat_lt(a1747, a1748)
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
        let v7: u64 = f121(match v5.clone() {
            Adt2::C0(_, _, h1749, _, _, _, _, _, _, _, _) => {
                h1749
            }
        }, v4)?;
        let v8: u64 = f121(match v5.clone() {
            Adt2::C0(_, _, _, h1750, _, _, _, _, _, _, _) => {
                h1750
            }
        }, v4)?;
        let a1752 = match v5.clone() {
            Adt2::C0(h1751, _, _, _, _, _, _, _, _, _, _) => {
                h1751
            }
        };
        let a1753 = 0u64;
        if if if if if nat_eq(a1752, a1753) {
            let a1755 = match v6.clone() {
                Adt2::C0(h1754, _, _, _, _, _, _, _, _, _, _) => {
                    h1754
                }
            };
            let a1756 = 0u64;
            nat_eq(a1755, a1756)
        } else {
            false
        } {
            let a1758 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1757, _, _, _, _, _, _) => {
                    h1757
                }
            };
            let a1759 = 1u64;
            nat_eq(a1758, a1759)
        } else {
            false
        } {
            let a1762 = match v5.clone() {
                Adt2::C0(_, h1760, _, _, _, _, _, _, _, _, _) => {
                    h1760
                }
            };
            let a1763 = match v6.clone() {
                Adt2::C0(_, h1761, _, _, _, _, _, _, _, _, _) => {
                    h1761
                }
            };
            nat_eq(a1762, a1763)
        } else {
            false
        } {
            if v0 {
                let a1765 = match v5.clone() {
                    Adt2::C0(_, _, h1764, _, _, _, _, _, _, _, _) => {
                        h1764
                    }
                };
                let a1766 = 2147483648u64;
                if if if nat_lt(a1765, a1766) {
                    let a1768 = match v5.clone() {
                        Adt2::C0(_, _, _, h1767, _, _, _, _, _, _, _) => {
                            h1767
                        }
                    };
                    let a1769 = 2147483648u64;
                    nat_lt(a1768, a1769)
                } else {
                    false
                } {
                    let a1772 = {
                        let a1770 = v7;
                        let a1771 = v8;
                        nat_mul(a1770, a1771)?
                    };
                    let a1773 = 2147483648u64;
                    nat_le(a1772, a1773)
                } else {
                    false
                } {
                    f122(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1775 = match v1.clone() {
                    Adt1::C0(h1774, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1774
                    }
                };
                let a1776 = v7;
                if if if if nat_eq(a1775, a1776) {
                    let a1778 = match v1 {
                        Adt1::C0(_, h1777, _, _, _, _, _, _, _, _, _, _) => {
                            h1777
                        }
                    };
                    let a1779 = v8;
                    nat_eq(a1778, a1779)
                } else {
                    false
                } {
                    f123(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1781 = match v6.clone() {
                        Adt2::C0(_, _, h1780, _, _, _, _, _, _, _, _) => {
                            h1780
                        }
                    };
                    let a1782 = v4;
                    if nat_eq(a1781, a1782) {
                        let a1784 = match v6.clone() {
                            Adt2::C0(_, _, _, h1783, _, _, _, _, _, _, _) => {
                                h1783
                            }
                        };
                        let a1785 = v4;
                        nat_eq(a1784, a1785)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1789 = match v6 {
                        Adt2::C0(_, _, _, _, h1786, _, _, _, _, _, _) => {
                            h1786
                        }
                    };
                    let a1790 = {
                        let a1787 = v7;
                        let a1788 = v8;
                        nat_mul(a1787, a1788)?
                    };
                    nat_eq(a1789, a1790)
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

pub fn f60(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1792 = match v2.clone() {
        Adt2::C0(h1791, _, _, _, _, _, _, _, _, _, _) => {
            h1791
        }
    };
    let a1793 = 0u64;
    if if if if if nat_eq(a1792, a1793) {
        true
    } else {
        let a1795 = match v2.clone() {
            Adt2::C0(h1794, _, _, _, _, _, _, _, _, _, _) => {
                h1794
            }
        };
        let a1796 = 1u64;
        nat_eq(a1795, a1796)
    } {
        true
    } else {
        let a1798 = match v2.clone() {
            Adt2::C0(h1797, _, _, _, _, _, _, _, _, _, _) => {
                h1797
            }
        };
        let a1799 = 30u64;
        nat_eq(a1798, a1799)
    } {
        let a1802 = match v2.clone() {
            Adt2::C0(h1800, _, _, _, _, _, _, _, _, _, _) => {
                h1800
            }
        };
        let a1803 = match v3.clone() {
            Adt2::C0(h1801, _, _, _, _, _, _, _, _, _, _) => {
                h1801
            }
        };
        nat_eq(a1802, a1803)
    } else {
        false
    } {
        if if f24(v2.clone(), v3.clone()) {
            let a1805 = match v1.clone() {
                Adt1::C0(h1804, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1804
                }
            };
            let a1806 = 4294967296u64;
            nat_lt(a1805, a1806)
        } else {
            false
        } {
            let a1808 = match v1.clone() {
                Adt1::C0(_, h1807, _, _, _, _, _, _, _, _, _, _) => {
                    h1807
                }
            };
            let a1809 = 4294967296u64;
            nat_lt(a1808, a1809)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a1811 = match v2.clone() {
                Adt2::C0(h1810, _, _, _, _, _, _, _, _, _, _) => {
                    h1810
                }
            };
            let a1812 = 0u64;
            if nat_eq(a1811, a1812) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a1813 = v0;
            let a1814 = 31u64;
            if nat_eq(a1813, a1814) {
                0u64
            } else {
                let a1815 = v0;
                let a1816 = 47u64;
                if nat_eq(a1815, a1816) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1817, _, _, _, _, _, _, _, _, _, _) => {
                h1817
            }
        }, match v1.clone() {
            Adt1::C0(h1818, _, _, _, _, _, _, _, _, _, _, _) => {
                h1818
            }
        }, match v1 {
            Adt1::C0(_, h1819, _, _, _, _, _, _, _, _, _, _) => {
                h1819
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f61(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1821 = match v0.clone() {
        Adt2::C0(h1820, _, _, _, _, _, _, _, _, _, _) => {
            h1820
        }
    };
    let a1822 = 0u64;
    if if if nat_eq(a1821, a1822) {
        let a1824 = match v1.clone() {
            Adt2::C0(h1823, _, _, _, _, _, _, _, _, _, _) => {
                h1823
            }
        };
        let a1825 = 0u64;
        nat_eq(a1824, a1825)
    } else {
        false
    } {
        f24(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v1.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f62(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1827 = match v0.clone() {
        Adt2::C0(h1826, _, _, _, _, _, _, _, _, _, _) => {
            h1826
        }
    };
    let a1828 = 0u64;
    if if nat_eq(a1827, a1828) {
        let a1830 = match v1.clone() {
            Adt2::C0(h1829, _, _, _, _, _, _, _, _, _, _) => {
                h1829
            }
        };
        let a1831 = 26u64;
        nat_eq(a1830, a1831)
    } else {
        false
    } {
        let a1833 = match v0.clone() {
            Adt2::C0(_, _, _, h1832, _, _, _, _, _, _, _) => {
                h1832
            }
        };
        let a1834 = 1u64;
        if if if if if if if nat_eq(a1833, a1834) {
            let a1836 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1835, _, _, _, _, _, _) => {
                    h1835
                }
            };
            let a1837 = 1u64;
            nat_eq(a1836, a1837)
        } else {
            false
        } {
            let a1839 = match v0.clone() {
                Adt2::C0(_, h1838, _, _, _, _, _, _, _, _, _) => {
                    h1838
                }
            };
            let a1840 = 2147483647u64;
            nat_le(a1839, a1840)
        } else {
            false
        } {
            let a1843 = match v1.clone() {
                Adt2::C0(_, h1841, _, _, _, _, _, _, _, _, _) => {
                    h1841
                }
            };
            let a1844 = match v0.clone() {
                Adt2::C0(_, _, h1842, _, _, _, _, _, _, _, _) => {
                    h1842
                }
            };
            nat_eq(a1843, a1844)
        } else {
            false
        } {
            let a1846 = match v1.clone() {
                Adt2::C0(_, _, h1845, _, _, _, _, _, _, _, _) => {
                    h1845
                }
            };
            let a1847 = 1u64;
            nat_eq(a1846, a1847)
        } else {
            false
        } {
            let a1849 = match v1.clone() {
                Adt2::C0(_, _, _, h1848, _, _, _, _, _, _, _) => {
                    h1848
                }
            };
            let a1850 = 1u64;
            nat_eq(a1849, a1850)
        } else {
            false
        } {
            let a1852 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1851, _, _, _, _, _, _) => {
                    h1851
                }
            };
            let a1853 = 1u64;
            nat_eq(a1852, a1853)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1856 = match v0.clone() {
                    Adt2::C0(_, _, h1854, _, _, _, _, _, _, _, _) => {
                        h1854
                    }
                };
                let a1857 = f124(match v0.clone() {
                    Adt2::C0(_, h1855, _, _, _, _, _, _, _, _, _) => {
                        h1855
                    }
                })?;
                nat_mul(a1856, a1857)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f63(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1858 = v1;
    match m1858 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f125(f20(v0.clone(), 1u64))?;
            let a1860 = match v0.clone() {
                Adt2::C0(h1859, _, _, _, _, _, _, _, _, _, _) => {
                    h1859
                }
            };
            let a1861 = 26u64;
            if if if nat_eq(a1860, a1861) {
                let a1863 = match v3.clone() {
                    Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                        h1862
                    }
                };
                let a1864 = 26u64;
                nat_eq(a1863, a1864)
            } else {
                false
            } {
                let a1866 = match v2.clone() {
                    Adt2::C0(h1865, _, _, _, _, _, _, _, _, _, _) => {
                        h1865
                    }
                };
                let a1867 = 27u64;
                nat_eq(a1866, a1867)
            } else {
                false
            } {
                if if if if if if f24(v0.clone(), v3.clone()) {
                    let a1869 = match v2.clone() {
                        Adt2::C0(_, h1868, _, _, _, _, _, _, _, _, _) => {
                            h1868
                        }
                    };
                    let a1870 = 1u64;
                    nat_eq(a1869, a1870)
                } else {
                    false
                } {
                    let a1872 = match v2.clone() {
                        Adt2::C0(_, _, h1871, _, _, _, _, _, _, _, _) => {
                            h1871
                        }
                    };
                    let a1873 = 1u64;
                    nat_eq(a1872, a1873)
                } else {
                    false
                } {
                    let a1875 = match v2.clone() {
                        Adt2::C0(_, _, _, h1874, _, _, _, _, _, _, _) => {
                            h1874
                        }
                    };
                    let a1876 = 1u64;
                    nat_eq(a1875, a1876)
                } else {
                    false
                } {
                    let a1878 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1877, _, _, _, _, _, _) => {
                            h1877
                        }
                    };
                    let a1879 = 1u64;
                    nat_eq(a1878, a1879)
                } else {
                    false
                } {
                    let a1880 = v4;
                    let a1881 = 9223372036854775808u64;
                    nat_lt(a1880, a1881)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1884 = 2u64;
                        let a1885 = {
                            let a1882 = v4;
                            let a1883 = 2u64;
                            nat_mul(a1882, a1883)?
                        };
                        nat_add(a1884, a1885)?
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

pub fn f64(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1886, _, _, _, _, _, _, _, _, _, _) => {
            h1886
        }
    };
    let a1887 = v3;
    let a1888 = 0u64;
    if if nat_eq(a1887, a1888) {
        true
    } else {
        let a1889 = v0;
        let a1890 = 13u64;
        if nat_eq(a1889, a1890) {
            let a1891 = v3;
            let a1892 = 1u64;
            if nat_eq(a1891, a1892) {
                true
            } else {
                let a1893 = v3;
                let a1894 = 30u64;
                nat_eq(a1893, a1894)
            }
        } else {
            false
        }
    } {
        let a1896 = v3;
        let a1897 = match v2.clone() {
            Adt2::C0(h1895, _, _, _, _, _, _, _, _, _, _) => {
                h1895
            }
        };
        if if nat_eq(a1896, a1897) {
            let a1898 = v0;
            let a1899 = 13u64;
            if nat_eq(a1898, a1899) {
                let a1901 = match v2.clone() {
                    Adt2::C0(_, h1900, _, _, _, _, _, _, _, _, _) => {
                        h1900
                    }
                };
                let a1902 = 1u64;
                if if if nat_eq(a1901, a1902) {
                    let a1904 = match v2.clone() {
                        Adt2::C0(_, _, h1903, _, _, _, _, _, _, _, _) => {
                            h1903
                        }
                    };
                    let a1905 = 1u64;
                    nat_eq(a1904, a1905)
                } else {
                    false
                } {
                    let a1907 = match v2.clone() {
                        Adt2::C0(_, _, _, h1906, _, _, _, _, _, _, _) => {
                            h1906
                        }
                    };
                    let a1908 = 1u64;
                    nat_eq(a1907, a1908)
                } else {
                    false
                } {
                    let a1910 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1909, _, _, _, _, _, _) => {
                            h1909
                        }
                    };
                    let a1911 = 1u64;
                    nat_eq(a1910, a1911)
                } else {
                    false
                }
            } else {
                let a1913 = match v2.clone() {
                    Adt2::C0(_, h1912, _, _, _, _, _, _, _, _, _) => {
                        h1912
                    }
                };
                let a1914 = 1u64;
                if if if nat_eq(a1913, a1914) {
                    let a1917 = match v1.clone() {
                        Adt2::C0(_, _, h1915, _, _, _, _, _, _, _, _) => {
                            h1915
                        }
                    };
                    let a1918 = match v2.clone() {
                        Adt2::C0(_, _, h1916, _, _, _, _, _, _, _, _) => {
                            h1916
                        }
                    };
                    nat_eq(a1917, a1918)
                } else {
                    false
                } {
                    let a1921 = match v1.clone() {
                        Adt2::C0(_, _, _, h1919, _, _, _, _, _, _, _) => {
                            h1919
                        }
                    };
                    let a1922 = match v2.clone() {
                        Adt2::C0(_, _, _, h1920, _, _, _, _, _, _, _) => {
                            h1920
                        }
                    };
                    nat_eq(a1921, a1922)
                } else {
                    false
                } {
                    let a1925 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1923, _, _, _, _, _, _) => {
                            h1923
                        }
                    };
                    let a1926 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1924, _, _, _, _, _, _) => {
                            h1924
                        }
                    };
                    nat_eq(a1925, a1926)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f126(v4.clone(), v0, v3)?, {
                let a1927 = v3;
                let a1928 = 0u64;
                if nat_eq(a1927, a1928) {
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

pub fn f65(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1929 = v0;
    let a1930 = 89u64;
    if if nat_eq(a1929, a1930) {
        let a1931 = v1;
        let a1932 = 15u64;
        nat_eq(a1931, a1932)
    } else {
        false
    } {
        Ok(Some(22u64))
    } else {
        let a1933 = v0;
        let a1934 = 89u64;
        if if nat_eq(a1933, a1934) {
            let a1935 = 8u64;
            let a1936 = v1;
            if nat_le(a1935, a1936) {
                let a1937 = v1;
                let a1938 = 9u64;
                nat_le(a1937, a1938)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Some({
                let a1939 = v1;
                let a1940 = 12u64;
                nat_add(a1939, a1940)?
            }))
        } else {
            let a1941 = v0;
            let a1942 = 89u64;
            if if nat_eq(a1941, a1942) {
                let a1943 = v1;
                let a1944 = 4u64;
                nat_eq(a1943, a1944)
            } else {
                false
            } {
                Ok(Some(19u64))
            } else {
                let a1945 = 10u64;
                let a1946 = v0;
                if if nat_le(a1945, a1946) {
                    let a1947 = v0;
                    let a1948 = 12u64;
                    nat_le(a1947, a1948)
                } else {
                    false
                } {
                    Ok(Some({
                        let a1949 = v0;
                        let a1950 = 3u64;
                        nat_add(a1949, a1950)?
                    }))
                } else {
                    let a1951 = v0;
                    let a1952 = 89u64;
                    if if nat_eq(a1951, a1952) {
                        let a1953 = v1;
                        let a1954 = 7u64;
                        nat_eq(a1953, a1954)
                    } else {
                        false
                    } {
                        Ok(Some(17u64))
                    } else {
                        let a1955 = v0;
                        let a1956 = 89u64;
                        if if nat_eq(a1955, a1956) {
                            let a1957 = v1;
                            let a1958 = 10u64;
                            nat_eq(a1957, a1958)
                        } else {
                            false
                        } {
                            Ok(Some(18u64))
                        } else {
                            let a1959 = v0;
                            let a1960 = 89u64;
                            if if nat_eq(a1959, a1960) {
                                let a1961 = v1;
                                let a1962 = 13u64;
                                nat_eq(a1961, a1962)
                            } else {
                                false
                            } {
                                Ok(Some(16u64))
                            } else {
                                let a1963 = v0;
                                let a1964 = 8u64;
                                if nat_eq(a1963, a1964) {
                                    Ok(Some(5u64))
                                } else {
                                    let a1965 = v0;
                                    let a1966 = 9u64;
                                    if nat_eq(a1965, a1966) {
                                        Ok(Some(6u64))
                                    } else {
                                        let a1967 = v0;
                                        let a1968 = 89u64;
                                        if nat_eq(a1967, a1968) {
                                            let a1969 = v1;
                                            let a1970 = 0u64;
                                            if nat_eq(a1969, a1970) {
                                                Ok(Some(0u64))
                                            } else {
                                                let a1971 = v1;
                                                let a1972 = 1u64;
                                                if nat_eq(a1971, a1972) {
                                                    Ok(Some(1u64))
                                                } else {
                                                    let a1973 = v1;
                                                    let a1974 = 2u64;
                                                    if nat_eq(a1973, a1974) {
                                                        Ok(Some(2u64))
                                                    } else {
                                                        let a1975 = v1;
                                                        let a1976 = 3u64;
                                                        if nat_eq(a1975, a1976) {
                                                            Ok(Some(3u64))
                                                        } else {
                                                            let a1977 = v1;
                                                            let a1978 = 6u64;
                                                            if nat_eq(a1977, a1978) {
                                                                Ok(Some(4u64))
                                                            } else {
                                                                let a1979 = v1;
                                                                let a1980 = 12u64;
                                                                if nat_eq(a1979, a1980) {
                                                                    Ok(Some(7u64))
                                                                } else {
                                                                    let a1981 = v1;
                                                                    let a1982 = 11u64;
                                                                    if nat_eq(a1981, a1982) {
                                                                        Ok(Some(8u64))
                                                                    } else {
                                                                        let a1983 = v1;
                                                                        let a1984 = 18u64;
                                                                        if nat_eq(a1983, a1984) {
                                                                            Ok(Some(9u64))
                                                                        } else {
                                                                            let a1985 = v1;
                                                                            let a1986 = 19u64;
                                                                            if nat_eq(a1985, a1986) {
                                                                                Ok(Some(10u64))
                                                                            } else {
                                                                                let a1987 = v1;
                                                                                let a1988 = 20u64;
                                                                                if nat_eq(a1987, a1988) {
                                                                                    Ok(Some(11u64))
                                                                                } else {
                                                                                    let a1989 = v1;
                                                                                    let a1990 = 21u64;
                                                                                    if nat_eq(a1989, a1990) {
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

pub fn f66(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1991 = f127(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1991 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1992 = v0;
            let a1993 = 2u64;
            if if if if nat_eq(a1992, a1993) {
                true
            } else {
                let a1994 = v0;
                let a1995 = 5u64;
                nat_eq(a1994, a1995)
            } {
                true
            } else {
                let a1996 = v0;
                let a1997 = 6u64;
                nat_eq(a1996, a1997)
            } {
                true
            } else {
                let a1998 = v0;
                let a1999 = 7u64;
                nat_eq(a1998, a1999)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2000 = v3;
                    match m2000 {
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

pub fn f67(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2002 = match v1.clone() {
        Adt2::C0(h2001, _, _, _, _, _, _, _, _, _, _) => {
            h2001
        }
    };
    let a2003 = 0u64;
    if if if if if nat_eq(a2002, a2003) {
        true
    } else {
        let a2005 = match v1.clone() {
            Adt2::C0(h2004, _, _, _, _, _, _, _, _, _, _) => {
                h2004
            }
        };
        let a2006 = 1u64;
        nat_eq(a2005, a2006)
    } {
        true
    } else {
        let a2008 = match v1.clone() {
            Adt2::C0(h2007, _, _, _, _, _, _, _, _, _, _) => {
                h2007
            }
        };
        let a2009 = 30u64;
        nat_eq(a2008, a2009)
    } {
        let a2012 = match v1.clone() {
            Adt2::C0(h2010, _, _, _, _, _, _, _, _, _, _) => {
                h2010
            }
        };
        let a2013 = match v2.clone() {
            Adt2::C0(h2011, _, _, _, _, _, _, _, _, _, _) => {
                h2011
            }
        };
        if nat_eq(a2012, a2013) {
            true
        } else {
            let a2015 = match v2.clone() {
                Adt2::C0(h2014, _, _, _, _, _, _, _, _, _, _) => {
                    h2014
                }
            };
            let a2016 = 0u64;
            nat_eq(a2015, a2016)
        }
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v2.clone(), 1u64)?, {
            let a2018 = match v2.clone() {
                Adt2::C0(h2017, _, _, _, _, _, _, _, _, _, _) => {
                    h2017
                }
            };
            let a2019 = 0u64;
            if nat_eq(a2018, a2019) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2020, _, _, _, _, _, _, _, _, _, _) => {
                h2020
            }
        }, match v2 {
            Adt2::C0(h2021, _, _, _, _, _, _, _, _, _, _) => {
                h2021
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f68(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f128(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2022, _, _, _) => {
            h2022
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2023, _, _) => {
            h2023
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2024, _) => {
            h2024
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2025) => {
            h2025
        }
    })
}

pub fn f69(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2027 = v0;
        let a2028 = match v1.clone() {
            Adt3::C0(h2026, _, _, _, _, _, _, _) => {
                h2026
            }
        };
        let a2029 = 0u64;
        nat_quot(a2027, a2028, a2029)
    };
    let v4: u64 = {
        let a2031 = v3;
        let a2032 = match v1.clone() {
            Adt3::C0(_, h2030, _, _, _, _, _, _) => {
                h2030
            }
        };
        let a2033 = 0u64;
        nat_quot(a2031, a2032, a2033)
    };
    let a2034 = v2;
    let a2035 = 0u64;
    if nat_eq(a2034, a2035) {
        let a2037 = v0;
        let a2038 = match v1 {
            Adt3::C0(h2036, _, _, _, _, _, _, _) => {
                h2036
            }
        };
        let a2039 = 0u64;
        nat_rem(a2037, a2038, a2039)
    } else {
        let a2040 = v2;
        let a2041 = 1u64;
        if nat_eq(a2040, a2041) {
            let a2043 = v3;
            let a2044 = match v1 {
                Adt3::C0(_, h2042, _, _, _, _, _, _) => {
                    h2042
                }
            };
            let a2045 = 0u64;
            nat_rem(a2043, a2044, a2045)
        } else {
            let a2046 = v2;
            let a2047 = 2u64;
            if nat_eq(a2046, a2047) {
                let a2049 = v4;
                let a2050 = match v1 {
                    Adt3::C0(_, _, h2048, _, _, _, _, _) => {
                        h2048
                    }
                };
                let a2051 = 0u64;
                nat_rem(a2049, a2050, a2051)
            } else {
                let a2053 = v4;
                let a2054 = match v1 {
                    Adt3::C0(_, _, h2052, _, _, _, _, _) => {
                        h2052
                    }
                };
                let a2055 = 0u64;
                nat_quot(a2053, a2054, a2055)
            }
        }
    }
}

pub fn f70(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2057 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2056) => {
            h2056
        }
    };
    let a2058 = 0u64;
    if nat_eq(a2057, a2058) {
        Ok(v2)
    } else {
        let a2063 = {
            let a2060 = v2;
            let a2061 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2059) => {
                    h2059
                }
            };
            nat_mul(a2060, a2061)?
        };
        let a2064 = f69(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2062, _, _, _, _, _, _) => {
                h2062
            }
        }, 3u64);
        nat_add(a2063, a2064)
    }
}

pub fn f71(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2074 = {
            let a2071 = {
                let a2066 = v2;
                let a2067 = match v0.clone() {
                    Adt6::C0(_, h2065, _, _) => {
                        h2065
                    }
                };
                nat_mul(a2066, a2067)?
            };
            let a2072 = {
                let a2069 = v3;
                let a2070 = match v0.clone() {
                    Adt6::C0(_, _, _, h2068) => {
                        h2068
                    }
                };
                nat_mul(a2069, a2070)?
            };
            nat_add(a2071, a2072)?
        };
        let a2075 = f129(match v0.clone() {
            Adt6::C0(_, _, h2073, _) => {
                h2073
            }
        });
        nat_add(a2074, a2075)?
    };
    let v5: u64 = f130(match v0 {
        Adt6::C0(_, _, h2076, _) => {
            h2076
        }
    });
    let a2077 = v5;
    let a2078 = v4;
    if if nat_le(a2077, a2078) {
        let a2081 = {
            let a2079 = v4;
            let a2080 = v5;
            nat_sub(a2079, a2080)
        };
        let a2082 = v1;
        nat_lt(a2081, a2082)
    } else {
        false
    } {
        Ok(Some({
            let a2083 = v4;
            let a2084 = v5;
            nat_sub(a2083, a2084)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f72(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2085, _, _, _, _, _) => {
            h2085
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2086, _, _, _, _, _, _) => {
            h2086
        }
    };
    let v7: u64 = f69(v1, v6.clone(), 2u64);
    let a2088 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2087) => {
            h2087
        }
    };
    let a2089 = 0u64;
    if nat_eq(a2088, a2089) {
        Ok(Adt9::C8(Some(f68(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2092 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2091, _, _, _, _) => {
                    h2091
                }
            };
            match m2092 {
                Adt3::C0(_, _, h2090, _, _, _, _, _) => {
                    h2090
                }
            }
        };
        let m2098 = f71(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2093, _) => {
                h2093
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2094, _, _, _, _, _) => {
                h2094
            }
        }, {
            let a2095 = v2;
            let a2096 = v8;
            let a2097 = 0u64;
            nat_rem(a2095, a2096, a2097)
        }, v7)?;
        match m2098 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2105 = {
                        let a2103 = {
                            let a2099 = v2;
                            let a2100 = v8;
                            let a2101 = 0u64;
                            nat_quot(a2099, a2100, a2101)
                        };
                        let a2104 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2102) => {
                                h2102
                            }
                        };
                        nat_mul(a2103, a2104)?
                    };
                    let a2106 = f69(v1, v6, 3u64);
                    nat_add(a2105, a2106)?
                };
                Ok(Adt9::C8(Some(f68(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f73(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2107 = v1;
    match m2107 {
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
            f131(f132(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f133(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f134(v0.clone(), v24);
            f131(f135(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2108 = v3;
    match m2108 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f136(v0, v1)?, f136(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f138(v0, v1, v2.clone())?, f136(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f139(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f140(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f141(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f140(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f142(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f143(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f144(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f136(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f145(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v30, v31) => {
            f146(v0, v1, v2, v30, v31)
        }
        Adt4::C12(v32, v33, v34) => {
            f147(v0, v1, v2, v32, v33, v34)
        }
        Adt4::C13(_, v36, _, _) => {
            Ok(Adt9::C3(v36, v36, f136(v0, v1)?, f136(v0, v2)?))
        }
        Adt4::C14(_, v40, v41) => {
            Ok(Adt9::C3(v40, v41, f136(v0, v1)?, f136(v0, v2)?))
        }
        Adt4::C15 => {
            f148(v0, v1, v2)
        }
        Adt4::C16(v42, v43) => {
            f149(v0, v1, v2, v42, v43)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v44, v45) => {
            f150(v0, v1, v45, v44)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f136(f137(v0, match v2.clone() {
                Adt3::C0(h2109, _, _, _, _, _, _, _) => {
                    h2109
                }
            }, match v2.clone() {
                Adt3::C0(_, h2110, _, _, _, _, _, _) => {
                    h2110
                }
            })?, v1)?, f136(v0, v2)?))
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
            f151(v0, v50, v51, v52)
        }
        Adt4::C26(v58, _, _, _) => {
            let a2111 = v58;
            let a2112 = 64u64;
            if if if nat_eq(a2111, a2112) {
                true
            } else {
                let a2113 = v58;
                let a2114 = 65u64;
                nat_eq(a2113, a2114)
            } {
                true
            } else {
                let a2115 = v58;
                let a2116 = 66u64;
                nat_eq(a2115, a2116)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v57) => {
            f83(v0, v1, v2, v57)
        }
        Adt4::C28(v53) => {
            f80(v0, v1, v2, v53, 0u64)
        }
        Adt4::C29(v54) => {
            f81(v0, v1, v2, v54)
        }
        Adt4::C30(v55, _) => {
            f82(v0, v1, v2, v55)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f114(v3, v4);
    let v6: u64 = {
        let a2117 = v0;
        let a2118 = v5;
        let a2119 = 0u64;
        nat_quot(a2117, a2118, a2119)
    };
    let a2120 = v5;
    let a2121 = 1u64;
    if nat_eq(a2120, a2121) {
        Ok(f131(f152(v3, v4), v3, v4, 0u64, f136(v6, v1)?, 2u64, 0u64, 3u64, f136(v6, v2)?))
    } else {
        let a2125 = {
            let a2122 = v0;
            let a2123 = 2u64;
            let a2124 = 0u64;
            nat_rem(a2122, a2123, a2124)
        };
        let a2126 = 0u64;
        if nat_eq(a2125, a2126) {
            Ok(f131(f152(v3, 0u64), v3, 0u64, 0u64, f136(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f131(f152(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f136(v6, v2)?))
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f124(match v1.clone() {
        Adt3::C0(h2127, _, _, _, _, _, _, _) => {
            h2127
        }
    })?;
    let v5: u64 = {
        let a2128 = v0;
        let a2129 = v4;
        let a2130 = 0u64;
        nat_quot(a2128, a2129, a2130)
    };
    let v6: u64 = {
        let a2131 = v0;
        let a2132 = v4;
        let a2133 = 0u64;
        nat_rem(a2131, a2132, a2133)
    };
    let a2134 = v6;
    let a2135 = 0u64;
    if nat_eq(a2134, a2135) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2136 = v6;
        let a2137 = 1u64;
        if nat_eq(a2136, a2137) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2140 = {
                let a2138 = v6;
                let a2139 = 1u64;
                nat_add(a2138, a2139)?
            };
            let a2141 = v4;
            if nat_eq(a2140, a2141) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f136(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2142 = v6;
                    let a2143 = 2u64;
                    nat_sub(a2142, a2143)
                };
                let v8: u64 = {
                    let a2144 = v7;
                    let a2145 = 3u64;
                    let a2146 = 0u64;
                    nat_quot(a2144, a2145, a2146)
                };
                let v9: u64 = {
                    let a2147 = v7;
                    let a2148 = 3u64;
                    let a2149 = 0u64;
                    nat_rem(a2147, a2148, a2149)
                };
                let v10: u64 = f136({
                    let a2153 = {
                        let a2151 = v5;
                        let a2152 = match v1.clone() {
                            Adt3::C0(h2150, _, _, _, _, _, _, _) => {
                                h2150
                            }
                        };
                        nat_mul(a2151, a2152)?
                    };
                    let a2154 = v8;
                    nat_add(a2153, a2154)?
                }, v1)?;
                let a2155 = v9;
                let a2156 = 0u64;
                if nat_eq(a2155, a2156) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2157 = v9;
                    let a2158 = 1u64;
                    if nat_eq(a2157, a2158) {
                        let a2159 = v3;
                        let a2160 = 2u64;
                        if nat_eq(a2159, a2160) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2161 = v3;
                        let a2162 = 2u64;
                        if if nat_eq(a2161, a2162) {
                            true
                        } else {
                            let a2163 = v3;
                            let a2164 = 3u64;
                            nat_eq(a2163, a2164)
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

pub fn f77(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2166 = 2u64;
        let a2167 = match v2.clone() {
            Adt3::C0(_, h2165, _, _, _, _, _, _) => {
                h2165
            }
        };
        nat_mul(a2166, a2167)?
    };
    let a2168 = v1;
    let a2169 = v6;
    if nat_lt(a2168, a2169) {
        let a2173 = {
            let a2170 = v1;
            let a2171 = 2u64;
            let a2172 = 0u64;
            nat_rem(a2170, a2171, a2172)
        };
        let a2174 = 0u64;
        if nat_eq(a2173, a2174) {
            Ok(Adt9::C4(19u64, 4u64, f136({
                let a2175 = v1;
                let a2176 = 2u64;
                let a2177 = 0u64;
                nat_quot(a2175, a2176, a2177)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2178 = v5;
            let a2179 = 2147483648u64;
            if if nat_lt(a2178, a2179) {
                let a2181 = v5;
                let a2182 = match v3 {
                    Adt3::C0(_, h2180, _, _, _, _, _, _) => {
                        h2180
                    }
                };
                nat_lt(a2181, a2182)
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
            let a2185 = match v3.clone() {
                Adt3::C0(h2183, _, _, _, _, _, _, _) => {
                    h2183
                }
            };
            let a2186 = match v3.clone() {
                Adt3::C0(_, h2184, _, _, _, _, _, _) => {
                    h2184
                }
            };
            nat_mul(a2185, a2186)?
        };
        let a2189 = {
            let a2187 = v1;
            let a2188 = v6;
            nat_sub(a2187, a2188)
        };
        let a2190 = v7;
        if nat_lt(a2189, a2190) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f136({
                let a2191 = v1;
                let a2192 = v6;
                nat_sub(a2191, a2192)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2195 = {
                    let a2193 = v1;
                    let a2194 = v6;
                    nat_sub(a2193, a2194)
                };
                let a2196 = v7;
                nat_sub(a2195, a2196)
            };
            let v9: u64 = {
                let a2198 = 1u64;
                let a2199 = f153(v0, match v2.clone() {
                    Adt3::C0(h2197, _, _, _, _, _, _, _) => {
                        h2197
                    }
                })?;
                nat_add(a2198, a2199)?
            };
            let v10: u64 = {
                let a2200 = v8;
                let a2201 = v9;
                let a2202 = 0u64;
                nat_quot(a2200, a2201, a2202)
            };
            let v11: u64 = {
                let a2203 = v8;
                let a2204 = v9;
                let a2205 = 0u64;
                nat_rem(a2203, a2204, a2205)
            };
            let a2206 = v11;
            let a2207 = 0u64;
            if nat_eq(a2206, a2207) {
                Ok(Adt9::C4(19u64, 4u64, f136(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f102(v0) {
                    let a2211 = {
                        let a2208 = v11;
                        let a2209 = 2u64;
                        let a2210 = 0u64;
                        nat_rem(a2208, a2209, a2210)
                    };
                    let a2212 = 1u64;
                    if nat_eq(a2211, a2212) {
                        Ok(Adt9::C4(6u64, 0u64, f136({
                            let a2221 = {
                                let a2214 = v10;
                                let a2215 = match v2.clone() {
                                    Adt3::C0(h2213, _, _, _, _, _, _, _) => {
                                        h2213
                                    }
                                };
                                nat_mul(a2214, a2215)?
                            };
                            let a2222 = {
                                let a2218 = {
                                    let a2216 = v11;
                                    let a2217 = 1u64;
                                    nat_sub(a2216, a2217)
                                };
                                let a2219 = 2u64;
                                let a2220 = 0u64;
                                nat_quot(a2218, a2219, a2220)
                            };
                            nat_add(a2221, a2222)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f136({
                            let a2231 = {
                                let a2224 = v5;
                                let a2225 = match v2.clone() {
                                    Adt3::C0(h2223, _, _, _, _, _, _, _) => {
                                        h2223
                                    }
                                };
                                nat_mul(a2224, a2225)?
                            };
                            let a2232 = {
                                let a2228 = {
                                    let a2226 = v11;
                                    let a2227 = 1u64;
                                    nat_sub(a2226, a2227)
                                };
                                let a2229 = 2u64;
                                let a2230 = 0u64;
                                nat_quot(a2228, a2229, a2230)
                            };
                            nat_add(a2231, a2232)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f136({
                            let a2241 = {
                                let a2234 = v5;
                                let a2235 = match v2 {
                                    Adt3::C0(h2233, _, _, _, _, _, _, _) => {
                                        h2233
                                    }
                                };
                                nat_mul(a2234, a2235)?
                            };
                            let a2242 = {
                                let a2238 = {
                                    let a2236 = v11;
                                    let a2237 = 1u64;
                                    nat_sub(a2236, a2237)
                                };
                                let a2239 = 2u64;
                                let a2240 = 0u64;
                                nat_quot(a2238, a2239, a2240)
                            };
                            nat_add(a2241, a2242)?
                        }, v3)?))
                    }
                } else {
                    f154(v0, v10, {
                        let a2243 = v11;
                        let a2244 = 1u64;
                        nat_sub(a2243, a2244)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2245 = v1;
    let a2246 = f155(v2.clone(), v4.clone())?;
    if nat_lt(a2245, a2246) {
        let a2249 = v1;
        let a2250 = {
            let a2247 = 2u64;
            let a2248 = f125(v4.clone())?;
            nat_mul(a2247, a2248)?
        };
        if nat_lt(a2249, a2250) {
            let a2254 = {
                let a2251 = v1;
                let a2252 = 2u64;
                let a2253 = 0u64;
                nat_rem(a2251, a2252, a2253)
            };
            let a2255 = 0u64;
            if nat_eq(a2254, a2255) {
                f156({
                    let a2256 = v1;
                    let a2257 = 2u64;
                    let a2258 = 0u64;
                    nat_quot(a2256, a2257, a2258)
                }, v4)
            } else {
                if f157(match v2 {
                    Adt3::C0(_, h2259, _, _, _, _, _, _) => {
                        h2259
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2262 = v1;
                let a2263 = {
                    let a2260 = 2u64;
                    let a2261 = f125(v4.clone())?;
                    nat_mul(a2260, a2261)?
                };
                nat_sub(a2262, a2263)
            };
            let v7: u64 = {
                let a2267 = v6;
                let a2268 = {
                    let a2265 = 1u64;
                    let a2266 = match v2.clone() {
                        Adt3::C0(h2264, _, _, _, _, _, _, _) => {
                            h2264
                        }
                    };
                    nat_add(a2265, a2266)?
                };
                let a2269 = 0u64;
                nat_quot(a2267, a2268, a2269)
            };
            let v8: u64 = {
                let a2273 = v6;
                let a2274 = {
                    let a2271 = 1u64;
                    let a2272 = match v2.clone() {
                        Adt3::C0(h2270, _, _, _, _, _, _, _) => {
                            h2270
                        }
                    };
                    nat_add(a2271, a2272)?
                };
                let a2275 = 0u64;
                nat_rem(a2273, a2274, a2275)
            };
            let a2276 = v8;
            let a2277 = 0u64;
            if nat_eq(a2276, a2277) {
                f156(v7, v4)
            } else {
                f158(v0, v7, {
                    let a2278 = v8;
                    let a2279 = 1u64;
                    nat_sub(a2278, a2279)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2282 = match v4.clone() {
            Adt3::C0(h2280, _, _, _, _, _, _, _) => {
                h2280
            }
        };
        let a2283 = match v4.clone() {
            Adt3::C0(_, h2281, _, _, _, _, _, _) => {
                h2281
            }
        };
        nat_mul(a2282, a2283)?
    };
    let v7: u64 = {
        let a2284 = 2u64;
        let a2285 = v6;
        nat_mul(a2284, a2285)?
    };
    let v8: u64 = f159(match v3.clone() {
        Adt3::C0(_, h2286, _, _, _, _, _, _) => {
            h2286
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2287, _, _, _, _, _) => {
            h2287
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2288, _, _, _, _) => {
            h2288
        }
    });
    let a2289 = v0;
    let a2290 = v7;
    if nat_lt(a2289, a2290) {
        let a2294 = {
            let a2291 = v0;
            let a2292 = 2u64;
            let a2293 = 0u64;
            nat_rem(a2291, a2292, a2293)
        };
        let a2295 = 0u64;
        if nat_eq(a2294, a2295) {
            Ok(Adt9::C4(19u64, 5u64, f136({
                let a2296 = v0;
                let a2297 = 2u64;
                let a2298 = 0u64;
                nat_quot(a2296, a2297, a2298)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f160(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2299 = v0;
            let a2300 = v7;
            nat_sub(a2299, a2300)
        };
        let v10: u64 = {
            let a2304 = v9;
            let a2305 = {
                let a2302 = 1u64;
                let a2303 = match v1.clone() {
                    Adt3::C0(h2301, _, _, _, _, _, _, _) => {
                        h2301
                    }
                };
                nat_add(a2302, a2303)?
            };
            let a2306 = 0u64;
            nat_quot(a2304, a2305, a2306)
        };
        let v11: u64 = {
            let a2310 = v9;
            let a2311 = {
                let a2308 = 1u64;
                let a2309 = match v1.clone() {
                    Adt3::C0(h2307, _, _, _, _, _, _, _) => {
                        h2307
                    }
                };
                nat_add(a2308, a2309)?
            };
            let a2312 = 0u64;
            nat_rem(a2310, a2311, a2312)
        };
        let a2313 = v11;
        let a2314 = 0u64;
        if nat_eq(a2313, a2314) {
            Ok(Adt9::C4(19u64, 5u64, f136({
                let a2315 = v10;
                let a2316 = v6;
                let a2317 = 0u64;
                nat_rem(a2315, a2316, a2317)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f160(v5, v8) {
                let v12: u64 = {
                    let a2323 = {
                        let a2319 = v10;
                        let a2320 = match v1.clone() {
                            Adt3::C0(h2318, _, _, _, _, _, _, _) => {
                                h2318
                            }
                        };
                        nat_mul(a2319, a2320)?
                    };
                    let a2324 = {
                        let a2321 = v11;
                        let a2322 = 1u64;
                        nat_sub(a2321, a2322)
                    };
                    nat_add(a2323, a2324)?
                };
                let a2325 = v5;
                let a2326 = 4294967295u64;
                if nat_eq(a2325, a2326) {
                    Ok(Adt9::C1(false, f136(v12, v1)?, f136(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f136(v12, v1)?, 4u64, f136({
                        let a2332 = {
                            let a2328 = v5;
                            let a2329 = match v3.clone() {
                                Adt3::C0(h2327, _, _, _, _, _, _, _) => {
                                    h2327
                                }
                            };
                            nat_mul(a2328, a2329)?
                        };
                        let a2333 = {
                            let a2330 = v11;
                            let a2331 = 1u64;
                            nat_sub(a2330, a2331)
                        };
                        nat_add(a2332, a2333)?
                    }, v3)?, 3u64, f136(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f107(v3.clone())?;
    let v6: u64 = {
        let a2334 = v0;
        let a2335 = v5;
        let a2336 = 0u64;
        nat_quot(a2334, a2335, a2336)
    };
    let v7: u64 = {
        let a2337 = v0;
        let a2338 = v5;
        let a2339 = 0u64;
        nat_rem(a2337, a2338, a2339)
    };
    let v8: u64 = f161(v3.clone())?;
    let a2340 = v7;
    let a2341 = 0u64;
    if nat_eq(a2340, a2341) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2342, _) => {
                h2342
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2346 = v5;
        let a2347 = {
            let a2344 = v7;
            let a2345 = if match v3.clone() {
                Adt7::C0(_, _, _, h2343, _) => {
                    h2343
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2344, a2345)?
        };
        if nat_le(a2346, a2347) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2348, _) => {
                    h2348
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f136(v6, v2)?))
            } else {
                let a2351 = {
                    let a2349 = v7;
                    let a2350 = 2u64;
                    nat_add(a2349, a2350)?
                };
                let a2352 = v5;
                if nat_eq(a2351, a2352) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2353, _, _) => {
                            h2353
                        }
                    } {
                        let a2360 = {
                            let m2356 = match v3.clone() {
                                Adt7::C0(h2355, _, _, _, _) => {
                                    h2355
                                }
                            };
                            match m2356 {
                                Adt6::C0(h2354, _, _, _) => {
                                    h2354
                                }
                            }
                        };
                        let a2361 = {
                            let m2359 = match v3 {
                                Adt7::C0(_, h2358, _, _, _) => {
                                    h2358
                                }
                            };
                            match m2359 {
                                Adt6::C0(h2357, _, _, _) => {
                                    h2357
                                }
                            }
                        };
                        nat_mul(a2360, a2361)?
                    } else {
                        f162(match v3 {
                            Adt7::C0(h2362, _, _, _, _) => {
                                h2362
                            }
                        }, match v1 {
                            Adt3::C0(h2363, _, _, _, _, _, _, _) => {
                                h2363
                            }
                        }, f69(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2364 = v9;
                        let a2365 = 0u64;
                        if nat_eq(a2364, a2365) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f136(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2368 = {
                    let a2366 = v7;
                    let a2367 = 1u64;
                    nat_sub(a2366, a2367)
                };
                let a2369 = v8;
                let a2370 = 0u64;
                nat_rem(a2368, a2369, a2370)
            };
            let m2376 = f163(v3.clone(), v1, v2, v6, {
                let a2373 = {
                    let a2371 = v7;
                    let a2372 = 1u64;
                    nat_sub(a2371, a2372)
                };
                let a2374 = v8;
                let a2375 = 0u64;
                nat_quot(a2373, a2374, a2375)
            })?;
            match m2376 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2378 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2377) => {
                            h2377
                        }
                    };
                    let a2379 = 1u64;
                    if if nat_eq(a2378, a2379) {
                        let a2380 = v10;
                        let a2381 = 0u64;
                        nat_eq(a2380, a2381)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2382, _) => {
                                h2382
                            }
                        } {
                            let a2385 = {
                                let a2383 = v10;
                                let a2384 = 1u64;
                                nat_add(a2383, a2384)?
                            };
                            let a2386 = v8;
                            if nat_eq(a2385, a2386) {
                                let a2387 = v4;
                                let a2388 = 2u64;
                                if nat_eq(a2387, a2388) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2390 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2389) => {
                                                h2389
                                            }
                                        };
                                        let a2391 = 1u64;
                                        if nat_eq(a2390, a2391) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2393 = match v3 {
                                            Adt7::C0(_, _, _, _, h2392) => {
                                                h2392
                                            }
                                        };
                                        let a2394 = 1u64;
                                        if nat_eq(a2393, a2394) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2396 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2395) => {
                                            h2395
                                        }
                                    };
                                    let a2397 = 1u64;
                                    if nat_eq(a2396, a2397) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2399 = match v3 {
                                        Adt7::C0(_, _, _, _, h2398) => {
                                            h2398
                                        }
                                    };
                                    let a2400 = 1u64;
                                    if nat_eq(a2399, a2400) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2402 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2401) => {
                                        h2401
                                    }
                                };
                                let a2403 = 1u64;
                                if nat_eq(a2402, a2403) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2405 = match v3 {
                                    Adt7::C0(_, _, _, _, h2404) => {
                                        h2404
                                    }
                                };
                                let a2406 = 1u64;
                                if nat_eq(a2405, a2406) {
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

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f105(match v3.clone() {
        Adt3::C0(h2407, _, _, _, _, _, _, _) => {
            h2407
        }
    })?;
    let v5: u64 = {
        let a2408 = v0;
        let a2409 = v4;
        let a2410 = 0u64;
        nat_rem(a2408, a2409, a2410)
    };
    let v6: u64 = {
        let a2411 = v0;
        let a2412 = v4;
        let a2413 = 0u64;
        nat_quot(a2411, a2412, a2413)
    };
    let a2414 = v5;
    let a2415 = 0u64;
    if nat_eq(a2414, a2415) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2418 = {
            let a2416 = v5;
            let a2417 = 1u64;
            nat_add(a2416, a2417)?
        };
        let a2419 = v4;
        if nat_eq(a2418, a2419) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f136(v6, v2)?))
        } else {
            let a2423 = {
                let a2420 = v5;
                let a2421 = 2u64;
                let a2422 = 0u64;
                nat_rem(a2420, a2421, a2422)
            };
            let a2424 = 0u64;
            if nat_eq(a2423, a2424) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2427 = {
                        let a2425 = v5;
                        let a2426 = 1u64;
                        nat_sub(a2425, a2426)
                    };
                    let a2428 = 2u64;
                    let a2429 = 0u64;
                    nat_quot(a2427, a2428, a2429)
                };
                let v8: u64 = f69(v6, v2.clone(), 0u64);
                let v9: u64 = f69(v6, v2.clone(), 1u64);
                let v10: u64 = f69(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f68(v1, {
                    let a2430 = v9;
                    let a2431 = v7;
                    nat_add(a2430, a2431)?
                }, v8, v10, 0u64)?, 4u64, f68(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f104(v3.clone())?;
    let v5: u64 = {
        let a2432 = v0;
        let a2433 = v4;
        let a2434 = 0u64;
        nat_rem(a2432, a2433, a2434)
    };
    let v6: u64 = {
        let a2435 = v0;
        let a2436 = v4;
        let a2437 = 0u64;
        nat_quot(a2435, a2436, a2437)
    };
    let a2438 = v5;
    let a2439 = 0u64;
    if nat_eq(a2438, a2439) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2442 = {
            let a2440 = v5;
            let a2441 = 1u64;
            nat_add(a2440, a2441)?
        };
        let a2443 = v4;
        if nat_eq(a2442, a2443) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f136(v6, v2)?))
        } else {
            let a2447 = {
                let a2444 = v5;
                let a2445 = 2u64;
                let a2446 = 0u64;
                nat_rem(a2444, a2445, a2446)
            };
            let a2448 = 0u64;
            if nat_eq(a2447, a2448) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2450 = f69(v6, v2.clone(), 0u64);
                    let a2451 = match v3.clone() {
                        Adt6::C0(_, _, h2449, _) => {
                            h2449
                        }
                    };
                    nat_add(a2450, a2451)?
                };
                let v8: u64 = {
                    let a2459 = {
                        let a2456 = {
                            let a2453 = v7;
                            let a2454 = match v3.clone() {
                                Adt6::C0(_, h2452, _, _) => {
                                    h2452
                                }
                            };
                            nat_add(a2453, a2454)?
                        };
                        let a2457 = match v3.clone() {
                            Adt6::C0(h2455, _, _, _) => {
                                h2455
                            }
                        };
                        nat_sub(a2456, a2457)
                    };
                    let a2460 = match v3.clone() {
                        Adt6::C0(_, h2458, _, _) => {
                            h2458
                        }
                    };
                    let a2461 = 0u64;
                    nat_quot(a2459, a2460, a2461)
                };
                let v9: u64 = {
                    let a2467 = v8;
                    let a2468 = {
                        let a2464 = {
                            let a2462 = v5;
                            let a2463 = 1u64;
                            nat_sub(a2462, a2463)
                        };
                        let a2465 = 2u64;
                        let a2466 = 0u64;
                        nat_quot(a2464, a2465, a2466)
                    };
                    nat_add(a2467, a2468)?
                };
                let v10: u64 = {
                    let a2470 = v9;
                    let a2471 = match v3.clone() {
                        Adt6::C0(_, h2469, _, _) => {
                            h2469
                        }
                    };
                    nat_mul(a2470, a2471)?
                };
                let v11: u64 = {
                    let a2472 = v7;
                    let a2473 = v10;
                    nat_sub(a2472, a2473)
                };
                let a2475 = v9;
                let a2476 = match v1.clone() {
                    Adt3::C0(_, h2474, _, _, _, _, _, _) => {
                        h2474
                    }
                };
                if if if nat_lt(a2475, a2476) {
                    let a2477 = v10;
                    let a2478 = v7;
                    nat_le(a2477, a2478)
                } else {
                    false
                } {
                    let a2480 = v11;
                    let a2481 = match v3.clone() {
                        Adt6::C0(h2479, _, _, _) => {
                            h2479
                        }
                    };
                    nat_lt(a2480, a2481)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f68(v1, {
                        let a2485 = {
                            let a2483 = f69(v6, v2, 1u64);
                            let a2484 = match v3 {
                                Adt6::C0(h2482, _, _, _) => {
                                    h2482
                                }
                            };
                            nat_mul(a2483, a2484)?
                        };
                        let a2486 = v11;
                        nat_add(a2485, a2486)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f69(v0, v2.clone(), 0u64);
    let v5: u64 = f69(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2488 = v5;
        let a2489 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2487, _) => {
                h2487
            }
        };
        let a2490 = 0u64;
        nat_rem(a2488, a2489, a2490)
    };
    let v7: u64 = {
        let a2492 = v5;
        let a2493 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2491, _) => {
                h2491
            }
        };
        let a2494 = 0u64;
        nat_quot(a2492, a2493, a2494)
    };
    let v8: Option<u64> = f71(match v3.clone() {
        Adt5::C0(h2495, _, _, _, _, _, _, _) => {
            h2495
        }
    }, match v1.clone() {
        Adt3::C0(h2496, _, _, _, _, _, _, _) => {
            h2496
        }
    }, f69(v0, v2.clone(), 1u64), {
        let a2500 = v4;
        let a2501 = {
            let m2499 = match v3.clone() {
                Adt5::C0(h2498, _, _, _, _, _, _, _) => {
                    h2498
                }
            };
            match m2499 {
                Adt6::C0(h2497, _, _, _) => {
                    h2497
                }
            }
        };
        let a2502 = 0u64;
        nat_rem(a2500, a2501, a2502)
    })?;
    let v9: u64 = {
        let a2506 = v4;
        let a2507 = {
            let m2505 = match v3.clone() {
                Adt5::C0(h2504, _, _, _, _, _, _, _) => {
                    h2504
                }
            };
            match m2505 {
                Adt6::C0(h2503, _, _, _) => {
                    h2503
                }
            }
        };
        let a2508 = 0u64;
        nat_quot(a2506, a2507, a2508)
    };
    let v10: Option<u64> = f71(match v3.clone() {
        Adt5::C0(_, h2509, _, _, _, _, _, _) => {
            h2509
        }
    }, match v1.clone() {
        Adt3::C0(_, h2510, _, _, _, _, _, _) => {
            h2510
        }
    }, f69(v0, v2.clone(), 2u64), {
        let a2514 = v9;
        let a2515 = {
            let m2513 = match v3.clone() {
                Adt5::C0(_, h2512, _, _, _, _, _, _) => {
                    h2512
                }
            };
            match m2513 {
                Adt6::C0(h2511, _, _, _) => {
                    h2511
                }
            }
        };
        let a2516 = 0u64;
        nat_rem(a2514, a2515, a2516)
    })?;
    let v11: u64 = {
        let a2520 = v9;
        let a2521 = {
            let m2519 = match v3.clone() {
                Adt5::C0(_, h2518, _, _, _, _, _, _) => {
                    h2518
                }
            };
            match m2519 {
                Adt6::C0(h2517, _, _, _) => {
                    h2517
                }
            }
        };
        let a2522 = 0u64;
        nat_quot(a2520, a2521, a2522)
    };
    let v12: Option<u64> = f71(match v3.clone() {
        Adt5::C0(_, _, h2523, _, _, _, _, _) => {
            h2523
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2524, _, _, _, _, _) => {
            h2524
        }
    }, v6, {
        let a2528 = v11;
        let a2529 = {
            let m2527 = match v3.clone() {
                Adt5::C0(_, _, h2526, _, _, _, _, _) => {
                    h2526
                }
            };
            match m2527 {
                Adt6::C0(h2525, _, _, _) => {
                    h2525
                }
            }
        };
        let a2530 = 0u64;
        nat_rem(a2528, a2529, a2530)
    })?;
    let v13: u64 = {
        let a2534 = v11;
        let a2535 = {
            let m2533 = match v3.clone() {
                Adt5::C0(_, _, h2532, _, _, _, _, _) => {
                    h2532
                }
            };
            match m2533 {
                Adt6::C0(h2531, _, _, _) => {
                    h2531
                }
            }
        };
        let a2536 = 0u64;
        nat_quot(a2534, a2535, a2536)
    };
    let v17: Adt12 = {
        let m2537 = v8;
        match m2537 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2538 = v10;
                match m2538 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2539 = v12;
                        match m2539 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2548 = {
                                        let a2543 = f68(v1, v14, v15, v16, 0u64)?;
                                        let a2544 = {
                                            let a2541 = v7;
                                            let a2542 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2540, _, _, _) => {
                                                    h2540
                                                }
                                            };
                                            nat_mul(a2541, a2542)?
                                        };
                                        nat_add(a2543, a2544)?
                                    };
                                    let a2549 = {
                                        let a2546 = v13;
                                        let a2547 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2545, _, _, _, _) => {
                                                h2545
                                            }
                                        };
                                        nat_mul(a2546, a2547)?
                                    };
                                    nat_add(a2548, a2549)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2550, _, _) => {
            h2550
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2551) => {
            h2551
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f136(v0, v2)?), {
        let a2553 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2552, _, _) => {
                h2552
            }
        };
        let a2554 = 0u64;
        if nat_eq(a2553, a2554) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2556 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2555) => {
                h2555
            }
        };
        let a2557 = 0u64;
        if nat_eq(a2556, a2557) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f84(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2570 = f164(v0, v1, match v2.clone() {
        Adt3::C0(h2558, _, _, _, _, _, _, _) => {
            h2558
        }
    }, match v2.clone() {
        Adt3::C0(_, h2559, _, _, _, _, _, _) => {
            h2559
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2560, _, _, _, _, _) => {
            h2560
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2561, _, _, _, _) => {
            h2561
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2562, _, _, _) => {
            h2562
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2563, _, _) => {
            h2563
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2564, _) => {
            h2564
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2565) => {
            h2565
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2566, _, _, _) => {
            h2566
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2567, _, _) => {
            h2567
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2568, _) => {
            h2568
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2569) => {
            h2569
        }
    }, v4)?;
    match m2570 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2573 = v6;
        let a2574 = {
            let a2571 = 3u64;
            let a2572 = v5;
            nat_add(a2571, a2572)?
        };
        nat_sub(a2573, a2574)
    };
    let a2576 = v0;
    let a2577 = f35(v3, match v1.clone() {
        Adt3::C0(h2575, _, _, _, _, _, _, _) => {
            h2575
        }
    }, v5, v8)?;
    if nat_lt(a2576, a2577) {
        let v9: u64 = f98(match v1.clone() {
            Adt3::C0(h2578, _, _, _, _, _, _, _) => {
                h2578
            }
        }, v5, v8)?;
        let v10: u64 = f99(v3, match v1.clone() {
            Adt3::C0(h2579, _, _, _, _, _, _, _) => {
                h2579
            }
        }, v5)?;
        let a2580 = v0;
        let a2581 = v9;
        if nat_lt(a2580, a2581) {
            f165(v0, v2, f29(v3), {
                let a2582 = 3u64;
                let a2583 = v5;
                nat_add(a2582, a2583)?
            }, v7)
        } else {
            let a2586 = v0;
            let a2587 = {
                let a2584 = v9;
                let a2585 = v10;
                nat_add(a2584, a2585)?
            };
            if nat_lt(a2586, a2587) {
                let v11: u64 = {
                    let a2588 = v0;
                    let a2589 = v9;
                    nat_sub(a2588, a2589)
                };
                let v12: u64 = f166(v3, match v1.clone() {
                    Adt3::C0(h2590, _, _, _, _, _, _, _) => {
                        h2590
                    }
                })?;
                f167({
                    let a2591 = v11;
                    let a2592 = v12;
                    let a2593 = 0u64;
                    nat_rem(a2591, a2592, a2593)
                }, {
                    let a2594 = v11;
                    let a2595 = v12;
                    let a2596 = 0u64;
                    nat_quot(a2594, a2595, a2596)
                }, v1, v3, v4, v7)
            } else {
                f168({
                    let a2599 = v0;
                    let a2600 = {
                        let a2597 = v9;
                        let a2598 = v10;
                        nat_add(a2597, a2598)?
                    };
                    nat_sub(a2599, a2600)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2601 = v0;
    let a2602 = f19(v1)?;
    if nat_lt(a2601, a2602) {
        let a2603 = v0;
        let a2604 = 4u64;
        if nat_lt(a2603, a2604) {
            let a2605 = v0;
            let a2606 = 0u64;
            if nat_eq(a2605, a2606) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2607 = v0;
                let a2608 = 1u64;
                if nat_eq(a2607, a2608) {
                    let a2609 = v5;
                    let a2610 = 0u64;
                    if nat_eq(a2609, a2610) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2611 = v0;
                    let a2612 = 2u64;
                    if nat_eq(a2611, a2612) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2613 = v0;
            let a2614 = 4u64;
            if if nat_eq(a2613, a2614) {
                let a2615 = f169(v5)?;
                let a2616 = v1;
                !nat_eq(a2615, a2616)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2619 = {
                        let a2617 = v0;
                        let a2618 = 4u64;
                        nat_sub(a2617, a2618)
                    };
                    let a2620 = 3u64;
                    let a2621 = 0u64;
                    nat_quot(a2619, a2620, a2621)
                };
                let v7: u64 = {
                    let a2624 = {
                        let a2622 = v0;
                        let a2623 = 4u64;
                        nat_sub(a2622, a2623)
                    };
                    let a2625 = 3u64;
                    let a2626 = 0u64;
                    nat_rem(a2624, a2625, a2626)
                };
                let a2627 = v7;
                let a2628 = 0u64;
                if nat_eq(a2627, a2628) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2629 = v7;
                    let a2630 = 1u64;
                    if nat_eq(a2629, a2630) {
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

pub fn f87(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2631 = v1;
    let a2632 = 0u64;
    if nat_eq(a2631, a2632) {
        if v0 {
            let a2633 = v3;
            let a2634 = 0u64;
            if if nat_eq(a2633, a2634) {
                let a2635 = v4;
                let a2636 = 43u64;
                nat_eq(a2635, a2636)
            } else {
                false
            } {
                f152(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2637 = v1;
        let a2638 = 1u64;
        if nat_eq(a2637, a2638) {
            Adt11::C1(v2)
        } else {
            let a2639 = v1;
            let a2640 = 2u64;
            if nat_eq(a2639, a2640) {
                Adt11::C5(v2)
            } else {
                let a2641 = v1;
                let a2642 = 3u64;
                if nat_eq(a2641, a2642) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2643 = v1;
                    let a2644 = 4u64;
                    if nat_eq(a2643, a2644) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f88(v0: u64, v1: u64) -> Adt12 {
    let a2645 = v0;
    let a2646 = 0u64;
    if nat_eq(a2645, a2646) {
        Adt12::C0(0u64, v1)
    } else {
        let a2647 = v0;
        let a2648 = 1u64;
        if nat_eq(a2647, a2648) {
            Adt12::C1(v1)
        } else {
            let a2649 = v0;
            let a2650 = 2u64;
            if nat_eq(a2649, a2650) {
                Adt12::C2(v1)
            } else {
                let a2651 = v0;
                let a2652 = 3u64;
                if nat_eq(a2651, a2652) {
                    Adt12::C3(v1)
                } else {
                    let a2653 = v0;
                    let a2654 = 13u64;
                    if nat_lt(a2653, a2654) {
                        Adt12::C0({
                            let a2655 = v0;
                            let a2656 = 3u64;
                            nat_sub(a2655, a2656)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f89(v0: u64, v1: u64) -> Adt13 {
    let a2657 = v0;
    let a2658 = 1u64;
    if nat_eq(a2657, a2658) {
        Adt13::C0(v1)
    } else {
        let a2659 = v0;
        let a2660 = 3u64;
        if if nat_eq(a2659, a2660) {
            true
        } else {
            let a2661 = v0;
            let a2662 = 6u64;
            nat_eq(a2661, a2662)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f90(v0: Adt6, v1: u64) -> R<u64> {
    let a2664 = 0u64;
    let a2665 = match v0.clone() {
        Adt6::C0(h2663, _, _, _) => {
            h2663
        }
    };
    if if if if if nat_lt(a2664, a2665) {
        let a2667 = 0u64;
        let a2668 = match v0.clone() {
            Adt6::C0(_, h2666, _, _) => {
                h2666
            }
        };
        nat_lt(a2667, a2668)
    } else {
        false
    } {
        let a2670 = 0u64;
        let a2671 = match v0.clone() {
            Adt6::C0(_, _, _, h2669) => {
                h2669
            }
        };
        nat_lt(a2670, a2671)
    } else {
        false
    } {
        let a2673 = match v0.clone() {
            Adt6::C0(_, h2672, _, _) => {
                h2672
            }
        };
        let a2674 = 2147483648u64;
        nat_lt(a2673, a2674)
    } else {
        false
    } {
        let a2676 = match v0.clone() {
            Adt6::C0(_, _, _, h2675) => {
                h2675
            }
        };
        let a2677 = 2147483648u64;
        nat_lt(a2676, a2677)
    } else {
        false
    } {
        let v2: u64 = {
            let a2681 = v1;
            let a2682 = {
                let a2679 = 2u64;
                let a2680 = f130(match v0.clone() {
                    Adt6::C0(_, _, h2678, _) => {
                        h2678
                    }
                });
                nat_mul(a2679, a2680)?
            };
            nat_add(a2681, a2682)?
        };
        let v3: u64 = {
            let a2694 = {
                let a2689 = 1u64;
                let a2690 = {
                    let a2687 = match v0.clone() {
                        Adt6::C0(_, _, _, h2683) => {
                            h2683
                        }
                    };
                    let a2688 = {
                        let a2685 = match v0.clone() {
                            Adt6::C0(h2684, _, _, _) => {
                                h2684
                            }
                        };
                        let a2686 = 1u64;
                        nat_sub(a2685, a2686)
                    };
                    nat_mul(a2687, a2688)?
                };
                nat_add(a2689, a2690)?
            };
            let a2695 = {
                let a2692 = 2u64;
                let a2693 = f129(match v0.clone() {
                    Adt6::C0(_, _, h2691, _) => {
                        h2691
                    }
                });
                nat_mul(a2692, a2693)?
            };
            nat_add(a2694, a2695)?
        };
        let a2696 = v3;
        let a2697 = v2;
        if nat_le(a2696, a2697) {
            let a2704 = 1u64;
            let a2705 = {
                let a2701 = {
                    let a2698 = v2;
                    let a2699 = v3;
                    nat_sub(a2698, a2699)
                };
                let a2702 = match v0 {
                    Adt6::C0(_, h2700, _, _) => {
                        h2700
                    }
                };
                let a2703 = 0u64;
                nat_quot(a2701, a2702, a2703)
            };
            nat_add(a2704, a2705)
        } else {
            let a2709 = {
                let a2706 = v3;
                let a2707 = v2;
                nat_sub(a2706, a2707)
            };
            let a2710 = match v0 {
                Adt6::C0(_, h2708, _, _) => {
                    h2708
                }
            };
            if nat_lt(a2709, a2710) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f91(v0: u64) -> u64 {
    let v1: u64 = {
        let a2711 = v0;
        let a2712 = 4294967296u64;
        let a2713 = 0u64;
        nat_rem(a2711, a2712, a2713)
    };
    let a2714 = v1;
    let a2715 = 2147483648u64;
    if nat_lt(a2714, a2715) {
        v1
    } else {
        0u64
    }
}

pub fn f92(v0: u64) -> R<u64> {
    let a2718 = 2u64;
    let a2719 = {
        let a2716 = 2u64;
        let a2717 = v0;
        nat_mul(a2716, a2717)?
    };
    nat_add(a2718, a2719)
}

pub fn f93(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2721 = match v1.clone() {
        Adt14::C0(h2720, _, _, _, _, _, _, _, _, _, _) => {
            h2720
        }
    };
    let a2722 = 0u64;
    if if if if nat_eq(a2721, a2722) {
        let a2724 = match v2.clone() {
            Adt14::C0(h2723, _, _, _, _, _, _, _, _, _, _) => {
                h2723
            }
        };
        let a2725 = 0u64;
        nat_eq(a2724, a2725)
    } else {
        false
    } {
        f170(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f171(match v1.clone() {
            Adt14::C0(_, h2726, _, _, _, _, _, _, _, _, _) => {
                h2726
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2727, _, _, _, _, _, _, _, _) => {
                h2727
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2728, _, _, _, _, _, _, _) => {
                h2728
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2729, _, _, _, _, _, _) => {
                h2729
            }
        }) {
            true
        } else {
            if if if if f30(v0) {
                f172(v1.clone(), v2.clone())
            } else {
                false
            } {
                f173(v1.clone())
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                if f174(v1.clone(), v2.clone())? {
                    true
                } else {
                    f175(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f171(match v1.clone() {
            Adt14::C0(_, h2730, _, _, _, _, _, _, _, _, _) => {
                h2730
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2731, _, _, _, _, _, _, _, _) => {
                h2731
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2732, _, _, _, _, _, _, _) => {
                h2732
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2733, _, _, _, _, _, _) => {
                h2733
            }
        }) {
            0u64
        } else {
            let a2739 = match v1.clone() {
                Adt14::C0(_, _, h2734, _, _, _, _, _, _, _, _) => {
                    h2734
                }
            };
            let a2740 = {
                let a2737 = match v1.clone() {
                    Adt14::C0(_, _, _, h2735, _, _, _, _, _, _, _) => {
                        h2735
                    }
                };
                let a2738 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2736, _, _, _, _, _, _) => {
                        h2736
                    }
                };
                nat_mul(a2737, a2738)?
            };
            nat_mul(a2739, a2740)?
        };
        let v4: u64 = {
            let a2741 = v3;
            let a2742 = 0u64;
            if if nat_eq(a2741, a2742) {
                true
            } else {
                f176(v2.clone())?
            } {
                0u64
            } else {
                let a2744 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2743, _) => {
                        h2743
                    }
                };
                let a2745 = 4u64;
                let a2746 = 0u64;
                nat_quot(a2744, a2745, a2746)
            }
        };
        Ok(Adt15::C1(f177(f178(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2747, _, _, _, _, _) => {
                h2747
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2748, _, _, _, _, _) => {
                h2748
            }
        }), match v1 {
            Adt14::C0(_, h2749, _, _, _, _, _, _, _, _, _) => {
                h2749
            }
        }, v3, v4)?, 4u64, 64u64, v3, f179(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f94(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f180(v3.clone());
    let a2751 = match v2.clone() {
        Adt14::C0(h2750, _, _, _, _, _, _, _, _, _, _) => {
            h2750
        }
    };
    let a2752 = 0u64;
    if if if if nat_eq(a2751, a2752) {
        let a2754 = match v3.clone() {
            Adt14::C0(h2753, _, _, _, _, _, _, _, _, _, _) => {
                h2753
            }
        };
        let a2755 = 0u64;
        nat_eq(a2754, a2755)
    } else {
        false
    } {
        f170(v2.clone(), v3)
    } else {
        false
    } {
        if f171(match v2.clone() {
            Adt14::C0(_, h2756, _, _, _, _, _, _, _, _, _) => {
                h2756
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2757, _, _, _, _, _, _, _, _) => {
                h2757
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2758, _, _, _, _, _, _, _) => {
                h2758
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2759, _, _, _, _, _, _) => {
                h2759
            }
        }) {
            true
        } else {
            let a2761 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2760, _, _, _, _, _) => {
                    h2760
                }
            };
            let a2762 = 4u64;
            if if if if if nat_eq(a2761, a2762) {
                f181(match v2.clone() {
                    Adt14::C0(_, h2763, _, _, _, _, _, _, _, _, _) => {
                        h2763
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2764, _, _, _, _, _, _, _, _) => {
                        h2764
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2765, _, _, _, _, _, _, _) => {
                        h2765
                    }
                }, v0)?
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                f173(v4.clone())
            } else {
                false
            } {
                let a2767 = f182(v4.clone())?;
                let a2768 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2766, _) => {
                        h2766
                    }
                };
                nat_le(a2767, a2768)
            } else {
                false
            } {
                if f174(v2.clone(), v4.clone())? {
                    true
                } else {
                    f175(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f171(match v2.clone() {
            Adt14::C0(_, h2769, _, _, _, _, _, _, _, _, _) => {
                h2769
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2770, _, _, _, _, _, _, _, _) => {
                h2770
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2771, _, _, _, _, _, _, _) => {
                h2771
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2772, _, _, _, _, _, _) => {
                h2772
            }
        }) {
            true
        } else {
            f176(v4.clone())?
        } {
            0u64
        } else {
            let a2774 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2773, _) => {
                    h2773
                }
            };
            let a2775 = 4u64;
            let a2776 = 0u64;
            nat_quot(a2774, a2775, a2776)
        };
        Ok(Adt15::C1(f183(match v2.clone() {
            Adt14::C0(_, h2777, _, _, _, _, _, _, _, _, _) => {
                h2777
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2778, _, _, _, _, _, _, _, _) => {
                h2778
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2779, _, _, _, _, _, _, _) => {
                h2779
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2780, _, _, _, _, _, _) => {
                h2780
            }
        }, v0, v5)?, 4u64, 65u64, v0, f179(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f95(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f180(v4.clone());
    if if if if if if if v1 {
        let a2782 = match v2.clone() {
            Adt14::C0(h2781, _, _, _, _, _, _, _, _, _, _) => {
                h2781
            }
        };
        let a2783 = 0u64;
        nat_eq(a2782, a2783)
    } else {
        false
    } {
        let a2785 = match v3.clone() {
            Adt14::C0(h2784, _, _, _, _, _, _, _, _, _, _) => {
                h2784
            }
        };
        let a2786 = 0u64;
        nat_eq(a2785, a2786)
    } else {
        false
    } {
        let a2788 = match v4.clone() {
            Adt14::C0(h2787, _, _, _, _, _, _, _, _, _, _) => {
                h2787
            }
        };
        let a2789 = 0u64;
        nat_eq(a2788, a2789)
    } else {
        false
    } {
        f170(v2.clone(), v3.clone())
    } else {
        false
    } {
        f170(v2.clone(), v4)
    } else {
        false
    } {
        if f171(match v2.clone() {
            Adt14::C0(_, h2790, _, _, _, _, _, _, _, _, _) => {
                h2790
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2791, _, _, _, _, _, _, _, _) => {
                h2791
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2792, _, _, _, _, _, _, _) => {
                h2792
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2793, _, _, _, _, _, _) => {
                h2793
            }
        }) {
            true
        } else {
            let a2795 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2794, _, _, _, _, _) => {
                    h2794
                }
            };
            let a2796 = 4u64;
            if if if if if if if nat_eq(a2795, a2796) {
                let a2798 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2797, _, _, _, _, _) => {
                        h2797
                    }
                };
                let a2799 = 4u64;
                nat_eq(a2798, a2799)
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                f173(v3.clone())
            } else {
                false
            } {
                f173(v5.clone())
            } else {
                false
            } {
                let a2801 = f182(v5.clone())?;
                let a2802 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2800, _) => {
                        h2800
                    }
                };
                nat_le(a2801, a2802)
            } else {
                false
            } {
                if f174(v2.clone(), v5.clone())? {
                    true
                } else {
                    f175(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f174(v3.clone(), v5.clone())? {
                    true
                } else {
                    f175(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f171(match v2.clone() {
            Adt14::C0(_, h2803, _, _, _, _, _, _, _, _, _) => {
                h2803
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2804, _, _, _, _, _, _, _, _) => {
                h2804
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2805, _, _, _, _, _, _, _) => {
                h2805
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2806, _, _, _, _, _, _) => {
                h2806
            }
        }) {
            0u64
        } else {
            let a2812 = match v2.clone() {
                Adt14::C0(_, _, h2807, _, _, _, _, _, _, _, _) => {
                    h2807
                }
            };
            let a2813 = {
                let a2810 = match v2.clone() {
                    Adt14::C0(_, _, _, h2808, _, _, _, _, _, _, _) => {
                        h2808
                    }
                };
                let a2811 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2809, _, _, _, _, _, _) => {
                        h2809
                    }
                };
                nat_mul(a2810, a2811)?
            };
            nat_mul(a2812, a2813)?
        };
        let v7: u64 = {
            let a2814 = v6;
            let a2815 = 0u64;
            if if nat_eq(a2814, a2815) {
                true
            } else {
                f176(v5.clone())?
            } {
                0u64
            } else {
                let a2817 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2816, _) => {
                        h2816
                    }
                };
                let a2818 = 4u64;
                let a2819 = 0u64;
                nat_quot(a2817, a2818, a2819)
            }
        };
        Ok(Adt15::C1(f184(match v2 {
            Adt14::C0(_, h2820, _, _, _, _, _, _, _, _, _) => {
                h2820
            }
        }, v6, v7)?, 4u64, 66u64, v6, f179(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f185(v0, v3, v6) {
        f185(v1, v4, v6)
    } else {
        false
    } {
        f185(v2, v5, v6)
    } else {
        false
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f186(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f186(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f186(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f186(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f186(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f186(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2821 = v2;
    let a2822 = 0u64;
    if nat_eq(a2821, a2822) {
        Ok(0u64)
    } else {
        let a2825 = 2u64;
        let a2826 = {
            let a2823 = v0;
            let a2824 = v1;
            nat_mul(a2823, a2824)?
        };
        nat_mul(a2825, a2826)
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2827 = v2;
    let a2828 = f166(v0, v1)?;
    nat_mul(a2827, a2828)
}

pub fn f100(v0: u64) -> u64 {
    let a2829 = v0;
    let a2830 = 30u64;
    if nat_eq(a2829, a2830) {
        3u64
    } else {
        1u64
    }
}

pub fn f101(v0: u64) -> R<u64> {
    let a2833 = 5u64;
    let a2834 = {
        let a2831 = 4u64;
        let a2832 = v0;
        nat_mul(a2831, a2832)?
    };
    nat_add(a2833, a2834)
}

pub fn f102(v0: u64) -> bool {
    let a2835 = v0;
    let a2836 = 0u64;
    if if nat_eq(a2835, a2836) {
        true
    } else {
        let a2837 = v0;
        let a2838 = 1u64;
        nat_eq(a2837, a2838)
    } {
        true
    } else {
        let a2839 = v0;
        let a2840 = 30u64;
        nat_eq(a2839, a2840)
    }
}

pub fn f103(v0: Adt2, v1: u64) -> R<bool> {
    let a2842 = match v0.clone() {
        Adt2::C0(_, _, _, h2841, _, _, _, _, _, _, _) => {
            h2841
        }
    };
    let a2843 = 1u64;
    if if if nat_eq(a2842, a2843) {
        let a2845 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2844, _, _, _, _, _, _) => {
                h2844
            }
        };
        let a2846 = 1u64;
        nat_eq(a2845, a2846)
    } else {
        false
    } {
        let a2848 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2847, _, _, _, _, _) => {
                h2847
            }
        };
        let a2849 = v1;
        nat_eq(a2848, a2849)
    } else {
        false
    } {
        let a2851 = match v0.clone() {
            Adt2::C0(_, _, h2850, _, _, _, _, _, _, _, _) => {
                h2850
            }
        };
        let a2852 = 1u64;
        if nat_eq(a2851, a2852) {
            Ok(true)
        } else {
            let a2857 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2853, _, _, _, _) => {
                    h2853
                }
            };
            let a2858 = {
                let a2855 = match v0 {
                    Adt2::C0(_, h2854, _, _, _, _, _, _, _, _, _) => {
                        h2854
                    }
                };
                let a2856 = v1;
                nat_mul(a2855, a2856)?
            };
            Ok(nat_eq(a2857, a2858))
        }
    } else {
        Ok(false)
    }
}

pub fn f104(v0: Adt6) -> R<u64> {
    let a2871 = 2u64;
    let a2872 = {
        let a2869 = 2u64;
        let a2870 = {
            let a2866 = {
                let a2863 = match v0.clone() {
                    Adt6::C0(h2859, _, _, _) => {
                        h2859
                    }
                };
                let a2864 = {
                    let a2861 = match v0.clone() {
                        Adt6::C0(_, h2860, _, _) => {
                            h2860
                        }
                    };
                    let a2862 = 1u64;
                    nat_sub(a2861, a2862)
                };
                nat_add(a2863, a2864)?
            };
            let a2867 = match v0 {
                Adt6::C0(_, h2865, _, _) => {
                    h2865
                }
            };
            let a2868 = 0u64;
            nat_quot(a2866, a2867, a2868)
        };
        nat_mul(a2869, a2870)?
    };
    nat_add(a2871, a2872)
}

pub fn f105(v0: u64) -> R<u64> {
    let a2875 = 2u64;
    let a2876 = {
        let a2873 = v0;
        let a2874 = 2u64;
        nat_mul(a2873, a2874)?
    };
    nat_add(a2875, a2876)
}

pub fn f106(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2880 = {
            let a2877 = v2;
            let a2878 = 1u64;
            nat_sub(a2877, a2878)
        };
        let a2881 = match v0.clone() {
            Adt6::C0(_, h2879, _, _) => {
                h2879
            }
        };
        nat_mul(a2880, a2881)?
    };
    let a2883 = 0u64;
    let a2884 = match v0.clone() {
        Adt6::C0(h2882, _, _, _) => {
            h2882
        }
    };
    if if if if if if if if nat_lt(a2883, a2884) {
        let a2886 = match v0.clone() {
            Adt6::C0(h2885, _, _, _) => {
                h2885
            }
        };
        let a2887 = 2147483648u64;
        nat_lt(a2886, a2887)
    } else {
        false
    } {
        let a2889 = 0u64;
        let a2890 = match v0.clone() {
            Adt6::C0(_, h2888, _, _) => {
                h2888
            }
        };
        nat_lt(a2889, a2890)
    } else {
        false
    } {
        let a2892 = match v0.clone() {
            Adt6::C0(_, h2891, _, _) => {
                h2891
            }
        };
        let a2893 = 2147483648u64;
        nat_lt(a2892, a2893)
    } else {
        false
    } {
        let a2894 = v1;
        let a2895 = 2147483648u64;
        nat_lt(a2894, a2895)
    } else {
        false
    } {
        let a2896 = 0u64;
        let a2897 = v2;
        nat_lt(a2896, a2897)
    } else {
        false
    } {
        let a2898 = v3;
        let a2899 = 2147483648u64;
        nat_lt(a2898, a2899)
    } else {
        false
    } {
        let a2911 = {
            let a2906 = {
                let a2903 = v3;
                let a2904 = {
                    let a2901 = match v0.clone() {
                        Adt6::C0(h2900, _, _, _) => {
                            h2900
                        }
                    };
                    let a2902 = 1u64;
                    nat_sub(a2901, a2902)
                };
                nat_add(a2903, a2904)?
            };
            let a2907 = f129(match v0.clone() {
                Adt6::C0(_, _, h2905, _) => {
                    h2905
                }
            });
            nat_add(a2906, a2907)?
        };
        let a2912 = {
            let a2909 = 2147483647u64;
            let a2910 = f130(match v0.clone() {
                Adt6::C0(_, _, h2908, _) => {
                    h2908
                }
            });
            nat_add(a2909, a2910)?
        };
        nat_le(a2911, a2912)
    } else {
        false
    } {
        let a2914 = match v0 {
            Adt6::C0(_, _, h2913, _) => {
                h2913
            }
        };
        let a2915 = 2147483648u64;
        Ok(!nat_eq(a2914, a2915))
    } else {
        Ok(false)
    }
}

pub fn f107(v0: Adt7) -> R<u64> {
    let a2927 = if match v0.clone() {
        Adt7::C0(_, _, _, h2916, _) => {
            h2916
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2928 = {
        let a2925 = {
            let a2923 = {
                let m2919 = match v0.clone() {
                    Adt7::C0(h2918, _, _, _, _) => {
                        h2918
                    }
                };
                match m2919 {
                    Adt6::C0(h2917, _, _, _) => {
                        h2917
                    }
                }
            };
            let a2924 = {
                let m2922 = match v0.clone() {
                    Adt7::C0(_, h2921, _, _, _) => {
                        h2921
                    }
                };
                match m2922 {
                    Adt6::C0(h2920, _, _, _) => {
                        h2920
                    }
                }
            };
            nat_mul(a2923, a2924)?
        };
        let a2926 = f161(v0)?;
        nat_mul(a2925, a2926)?
    };
    nat_add(a2927, a2928)
}

pub fn f108(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f187(match v0.clone() {
        Adt2::C0(h2929, _, _, _, _, _, _, _, _, _, _) => {
            h2929
        }
    }, match v0.clone() {
        Adt2::C0(_, h2930, _, _, _, _, _, _, _, _, _) => {
            h2930
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2931, _, _, _, _, _, _, _, _) => {
            h2931
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2932, _, _, _, _, _, _, _) => {
            h2932
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2933, _, _, _, _, _, _) => {
            h2933
        }
    }, match v1.clone() {
        Adt2::C0(h2934, _, _, _, _, _, _, _, _, _, _) => {
            h2934
        }
    }, match v1.clone() {
        Adt2::C0(_, h2935, _, _, _, _, _, _, _, _, _) => {
            h2935
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2936, _, _, _, _, _, _, _, _) => {
            h2936
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2937, _, _, _, _, _, _, _) => {
            h2937
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2938, _, _, _, _, _, _) => {
            h2938
        }
    }, match v2.clone() {
        Adt2::C0(h2939, _, _, _, _, _, _, _, _, _, _) => {
            h2939
        }
    }, match v2.clone() {
        Adt2::C0(_, h2940, _, _, _, _, _, _, _, _, _) => {
            h2940
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2941, _, _, _, _, _, _, _, _) => {
            h2941
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2942, _, _, _, _, _, _, _) => {
            h2942
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2943, _, _, _, _, _, _) => {
            h2943
        }
    }) {
        let m2945 = f112(match v0.clone() {
            Adt2::C0(h2944, _, _, _, _, _, _, _, _, _, _) => {
                h2944
            }
        });
        match m2945 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a2951 = {
                    let a2948 = match v0.clone() {
                        Adt2::C0(_, h2946, _, _, _, _, _, _, _, _, _) => {
                            h2946
                        }
                    };
                    let a2949 = match v3.clone() {
                        Adt18::C0(_, h2947, _) => {
                            h2947
                        }
                    };
                    let a2950 = 0u64;
                    nat_rem(a2948, a2949, a2950)
                };
                let a2952 = 0u64;
                if if nat_eq(a2951, a2952) {
                    if f188(match v0.clone() {
                        Adt2::C0(_, h2953, _, _, _, _, _, _, _, _, _) => {
                            h2953
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2954, _, _, _, _, _, _, _, _, _) => {
                            h2954
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2955, _, _, _, _, _, _, _, _) => {
                            h2955
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2956, _, _, _, _, _, _, _) => {
                            h2956
                        }
                    }) {
                        true
                    } else {
                        let a2958 = 4u64;
                        let a2959 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2957, _, _, _, _, _) => {
                                h2957
                            }
                        };
                        if if nat_le(a2958, a2959) {
                            let a2961 = match v3.clone() {
                                Adt18::C0(_, h2960, _) => {
                                    h2960
                                }
                            };
                            let a2962 = 1u64;
                            if nat_eq(a2961, a2962) {
                                true
                            } else {
                                let a2965 = match v3.clone() {
                                    Adt18::C0(_, _, h2963) => {
                                        h2963
                                    }
                                };
                                let a2966 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2964, _, _, _, _, _) => {
                                        h2964
                                    }
                                };
                                nat_le(a2965, a2966)
                            }
                        } else {
                            false
                        } {
                            if f22(v0.clone(), v2.clone())? {
                                f22(v1.clone(), v2.clone())?
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
                        Adt18::C0(_, h2967, _) => {
                            h2967
                        }
                    });
                    Ok(Adt10::C4(if f188(match v0.clone() {
                        Adt2::C0(_, h2968, _, _, _, _, _, _, _, _, _) => {
                            h2968
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2969, _, _, _, _, _, _, _, _, _) => {
                            h2969
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2970, _, _, _, _, _, _, _, _) => {
                            h2970
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2971, _, _, _, _, _, _, _) => {
                            h2971
                        }
                    }) {
                        0u64
                    } else {
                        f155(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2972, _, _, _, _, _, _, _, _, _, _) => {
                            h2972
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

pub fn f109(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f110(match v0.clone() {
        Adt2::C0(h2973, _, _, _, _, _, _, _, _, _, _) => {
            h2973
        }
    }) {
        let a2979 = {
            let a2976 = match v0.clone() {
                Adt2::C0(_, h2974, _, _, _, _, _, _, _, _, _) => {
                    h2974
                }
            };
            let a2977 = f189(match v0.clone() {
                Adt2::C0(h2975, _, _, _, _, _, _, _, _, _, _) => {
                    h2975
                }
            });
            let a2978 = 1u64;
            nat_rem(a2976, a2977, a2978)
        };
        let a2980 = 0u64;
        nat_eq(a2979, a2980)
    } else {
        false
    } {
        let a2982 = match v1.clone() {
            Adt2::C0(h2981, _, _, _, _, _, _, _, _, _, _) => {
                h2981
            }
        };
        let a2983 = 26u64;
        nat_eq(a2982, a2983)
    } else {
        false
    } {
        let a2985 = match v2.clone() {
            Adt2::C0(h2984, _, _, _, _, _, _, _, _, _, _) => {
                h2984
            }
        };
        let a2986 = 0u64;
        nat_eq(a2985, a2986)
    } else {
        false
    } {
        let a2989 = match v0.clone() {
            Adt2::C0(_, h2987, _, _, _, _, _, _, _, _, _) => {
                h2987
            }
        };
        let a2990 = match v2.clone() {
            Adt2::C0(_, h2988, _, _, _, _, _, _, _, _, _) => {
                h2988
            }
        };
        nat_eq(a2989, a2990)
    } else {
        false
    } {
        let a2993 = match v1.clone() {
            Adt2::C0(_, h2991, _, _, _, _, _, _, _, _, _) => {
                h2991
            }
        };
        let a2994 = match v0.clone() {
            Adt2::C0(_, _, h2992, _, _, _, _, _, _, _, _) => {
                h2992
            }
        };
        nat_le(a2993, a2994)
    } else {
        false
    } {
        let a2996 = match v0.clone() {
            Adt2::C0(_, _, _, h2995, _, _, _, _, _, _, _) => {
                h2995
            }
        };
        let a2997 = 1u64;
        nat_eq(a2996, a2997)
    } else {
        false
    } {
        let a2999 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2998, _, _, _, _, _, _) => {
                h2998
            }
        };
        let a3000 = 1u64;
        nat_eq(a2999, a3000)
    } else {
        false
    } {
        let a3002 = match v2.clone() {
            Adt2::C0(_, _, _, h3001, _, _, _, _, _, _, _) => {
                h3001
            }
        };
        let a3003 = 1u64;
        nat_eq(a3002, a3003)
    } else {
        false
    } {
        let a3005 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3004, _, _, _, _, _, _) => {
                h3004
            }
        };
        let a3006 = 1u64;
        nat_eq(a3005, a3006)
    } else {
        false
    } {
        let a3008 = match v1.clone() {
            Adt2::C0(_, _, h3007, _, _, _, _, _, _, _, _) => {
                h3007
            }
        };
        let a3009 = 1u64;
        nat_eq(a3008, a3009)
    } else {
        false
    } {
        let a3011 = match v1.clone() {
            Adt2::C0(_, _, _, h3010, _, _, _, _, _, _, _) => {
                h3010
            }
        };
        let a3012 = 1u64;
        nat_eq(a3011, a3012)
    } else {
        false
    } {
        let a3014 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3013, _, _, _, _, _, _) => {
                h3013
            }
        };
        let a3015 = 1u64;
        nat_eq(a3014, a3015)
    } else {
        false
    } {
        if f33(match v2.clone() {
            Adt2::C0(_, h3016, _, _, _, _, _, _, _, _, _) => {
                h3016
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3017, _, _, _, _, _, _, _, _) => {
                h3017
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3018, _, _, _, _, _, _, _) => {
                h3018
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3019, _, _, _, _, _, _) => {
                h3019
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3021 = match v1.clone() {
                Adt2::C0(_, h3020, _, _, _, _, _, _, _, _, _) => {
                    h3020
                }
            };
            let a3022 = 0u64;
            if nat_eq(a3021, a3022) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3024 = match v0.clone() {
                    Adt2::C0(_, h3023, _, _, _, _, _, _, _, _, _) => {
                        h3023
                    }
                };
                let a3025 = 2147483647u64;
                if if if if if if nat_le(a3024, a3025) {
                    let a3027 = match v1.clone() {
                        Adt2::C0(_, h3026, _, _, _, _, _, _, _, _, _) => {
                            h3026
                        }
                    };
                    let a3028 = 2147483647u64;
                    nat_le(a3027, a3028)
                } else {
                    false
                } {
                    let a3030 = 4u64;
                    let a3031 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3029, _, _, _, _, _) => {
                            h3029
                        }
                    };
                    nat_le(a3030, a3031)
                } else {
                    false
                } {
                    let a3037 = {
                        let a3034 = match v2.clone() {
                            Adt2::C0(_, h3032, _, _, _, _, _, _, _, _, _) => {
                                h3032
                            }
                        };
                        let a3035 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3033, _, _, _, _, _) => {
                                h3033
                            }
                        };
                        nat_mul(a3034, a3035)?
                    };
                    let a3038 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3036, _, _, _, _) => {
                            h3036
                        }
                    };
                    nat_le(a3037, a3038)
                } else {
                    false
                } {
                    f22(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f22(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3051 = {
                            let a3042 = {
                                let a3040 = 2u64;
                                let a3041 = match v1.clone() {
                                    Adt2::C0(_, h3039, _, _, _, _, _, _, _, _, _) => {
                                        h3039
                                    }
                                };
                                nat_mul(a3040, a3041)?
                            };
                            let a3043 = f16(v2.clone(), 1u64)?;
                            nat_add(a3042, a3043)?
                        };
                        let a3052 = {
                            let a3049 = match v1.clone() {
                                Adt2::C0(_, h3044, _, _, _, _, _, _, _, _, _) => {
                                    h3044
                                }
                            };
                            let a3050 = {
                                let a3047 = 1u64;
                                let a3048 = f153(match v0.clone() {
                                    Adt2::C0(h3045, _, _, _, _, _, _, _, _, _, _) => {
                                        h3045
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3046, _, _, _, _, _, _, _, _, _) => {
                                        h3046
                                    }
                                })?;
                                nat_add(a3047, a3048)?
                            };
                            nat_mul(a3049, a3050)?
                        };
                        nat_add(a3051, a3052)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3053, _, _, _, _, _, _, _, _, _) => {
                            h3053
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3054, _, _, _, _, _, _, _, _, _) => {
                            h3054
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3055, _, _, _, _, _) => {
                            h3055
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3056, _, _, _, _) => {
                            h3056
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3057, _, _, _) => {
                            h3057
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3058, _, _) => {
                            h3058
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3059, _, _, _, _, _, _, _, _, _, _) => {
                            h3059
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

pub fn f110(v0: u64) -> bool {
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

pub fn f111(v0: u64) -> bool {
    if f102(v0) {
        true
    } else {
        let a3120 = v0;
        let a3121 = 43u64;
        nat_eq(a3120, a3121)
    }
}

pub fn f112(v0: u64) -> Adt17 {
    let a3122 = v0;
    let a3123 = 0u64;
    if nat_eq(a3122, a3123) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3124 = v0;
        let a3125 = 1u64;
        if nat_eq(a3124, a3125) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3126 = v0;
            let a3127 = 2u64;
            if nat_eq(a3126, a3127) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3128 = v0;
                let a3129 = 3u64;
                if nat_eq(a3128, a3129) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3130 = v0;
                    let a3131 = 6u64;
                    if nat_eq(a3130, a3131) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3132 = v0;
                        let a3133 = 7u64;
                        if nat_eq(a3132, a3133) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3134 = v0;
                            let a3135 = 8u64;
                            if nat_eq(a3134, a3135) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3136 = v0;
                                let a3137 = 9u64;
                                if nat_eq(a3136, a3137) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3138 = v0;
                                    let a3139 = 10u64;
                                    if nat_eq(a3138, a3139) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3140 = v0;
                                        let a3141 = 11u64;
                                        if nat_eq(a3140, a3141) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3142 = v0;
                                            let a3143 = 12u64;
                                            if nat_eq(a3142, a3143) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3144 = v0;
                                                let a3145 = 13u64;
                                                if nat_eq(a3144, a3145) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3146 = v0;
                                                    let a3147 = 14u64;
                                                    if nat_eq(a3146, a3147) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3148 = v0;
                                                        let a3149 = 15u64;
                                                        if nat_eq(a3148, a3149) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3150 = v0;
                                                            let a3151 = 16u64;
                                                            if nat_eq(a3150, a3151) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3152 = v0;
                                                                let a3153 = 17u64;
                                                                if nat_eq(a3152, a3153) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3154 = v0;
                                                                    let a3155 = 18u64;
                                                                    if nat_eq(a3154, a3155) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3156 = v0;
                                                                        let a3157 = 19u64;
                                                                        if nat_eq(a3156, a3157) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3158 = v0;
                                                                            let a3159 = 20u64;
                                                                            if nat_eq(a3158, a3159) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3160 = v0;
                                                                                let a3161 = 21u64;
                                                                                if nat_eq(a3160, a3161) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3162 = v0;
                                                                                    let a3163 = 22u64;
                                                                                    if nat_eq(a3162, a3163) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3164 = v0;
                                                                                        let a3165 = 23u64;
                                                                                        if nat_eq(a3164, a3165) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3166 = v0;
                                                                                            let a3167 = 24u64;
                                                                                            if nat_eq(a3166, a3167) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3168 = v0;
                                                                                                let a3169 = 25u64;
                                                                                                if nat_eq(a3168, a3169) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3170 = v0;
                                                                                                    let a3171 = 26u64;
                                                                                                    if nat_eq(a3170, a3171) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3172 = v0;
                                                                                                        let a3173 = 27u64;
                                                                                                        if nat_eq(a3172, a3173) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3174 = v0;
                                                                                                            let a3175 = 28u64;
                                                                                                            if nat_eq(a3174, a3175) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3176 = v0;
                                                                                                                let a3177 = 29u64;
                                                                                                                if nat_eq(a3176, a3177) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3178 = v0;
                                                                                                                    let a3179 = 30u64;
                                                                                                                    if nat_eq(a3178, a3179) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3180 = v0;
                                                                                                                        let a3181 = 34u64;
                                                                                                                        if nat_eq(a3180, a3181) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3182 = v0;
                                                                                                                            let a3183 = 35u64;
                                                                                                                            if nat_eq(a3182, a3183) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3184 = v0;
                                                                                                                                let a3185 = 39u64;
                                                                                                                                if nat_eq(a3184, a3185) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3186 = v0;
                                                                                                                                    let a3187 = 40u64;
                                                                                                                                    if nat_eq(a3186, a3187) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3188 = v0;
                                                                                                                                        let a3189 = 41u64;
                                                                                                                                        if nat_eq(a3188, a3189) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3190 = v0;
                                                                                                                                            let a3191 = 42u64;
                                                                                                                                            if nat_eq(a3190, a3191) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3192 = v0;
                                                                                                                                                let a3193 = 43u64;
                                                                                                                                                if nat_eq(a3192, a3193) {
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

pub fn f113(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3196 = match v1.clone() {
        Adt2::C0(h3194, _, _, _, _, _, _, _, _, _, _) => {
            h3194
        }
    };
    let a3197 = match v3.clone() {
        Adt2::C0(h3195, _, _, _, _, _, _, _, _, _, _) => {
            h3195
        }
    };
    if if nat_eq(a3196, a3197) {
        let a3198 = v0;
        let a3199 = 33u64;
        if nat_eq(a3198, a3199) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3201 = f112(match v1.clone() {
            Adt2::C0(h3200, _, _, _, _, _, _, _, _, _, _) => {
                h3200
            }
        });
        match m3201 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3202, _) => {
                        h3202
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3207 = {
                    let a3204 = match v1.clone() {
                        Adt2::C0(_, h3203, _, _, _, _, _, _, _, _, _) => {
                            h3203
                        }
                    };
                    let a3205 = v5;
                    let a3206 = 1u64;
                    nat_rem(a3204, a3205, a3206)
                };
                let a3208 = 0u64;
                if if if nat_eq(a3207, a3208) {
                    let a3213 = {
                        let a3210 = match v3.clone() {
                            Adt2::C0(_, h3209, _, _, _, _, _, _, _, _, _) => {
                                h3209
                            }
                        };
                        let a3211 = v5;
                        let a3212 = 1u64;
                        nat_rem(a3210, a3211, a3212)
                    };
                    let a3214 = 0u64;
                    nat_eq(a3213, a3214)
                } else {
                    false
                } {
                    let a3215 = v6;
                    let a3216 = f16(v3.clone(), v5)?;
                    nat_eq(a3215, a3216)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3217) => {
                            h3217
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

pub fn f114(v0: u64, v1: u64) -> u64 {
    let a3218 = v0;
    let a3219 = 0u64;
    if if nat_eq(a3218, a3219) {
        true
    } else {
        let a3220 = v1;
        let a3221 = 0u64;
        nat_eq(a3220, a3221)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f33(v4, v5, v6, v7) {
        f33(v0, v1, v2, v3)
    } else {
        let a3225 = {
            let a3222 = v0;
            let a3223 = v4;
            let a3224 = 1u64;
            nat_rem(a3222, a3223, a3224)
        };
        let a3226 = 0u64;
        if if if nat_eq(a3225, a3226) {
            let a3230 = {
                let a3227 = v1;
                let a3228 = v5;
                let a3229 = 1u64;
                nat_rem(a3227, a3228, a3229)
            };
            let a3231 = 0u64;
            nat_eq(a3230, a3231)
        } else {
            false
        } {
            let a3235 = {
                let a3232 = v2;
                let a3233 = v6;
                let a3234 = 1u64;
                nat_rem(a3232, a3233, a3234)
            };
            let a3236 = 0u64;
            nat_eq(a3235, a3236)
        } else {
            false
        } {
            let a3240 = {
                let a3237 = v3;
                let a3238 = v7;
                let a3239 = 1u64;
                nat_rem(a3237, a3238, a3239)
            };
            let a3241 = 0u64;
            nat_eq(a3240, a3241)
        } else {
            false
        }
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3242 = v0;
    let a3243 = v1;
    if nat_eq(a3242, a3243) {
        let a3246 = v4;
        let a3247 = {
            let a3244 = v2;
            let a3245 = v3;
            nat_add(a3244, a3245)?
        };
        Ok(nat_eq(a3246, a3247))
    } else {
        let a3248 = v2;
        let a3249 = v3;
        if nat_eq(a3248, a3249) {
            let a3250 = v4;
            let a3251 = v2;
            Ok(nat_eq(a3250, a3251))
        } else {
            Ok(false)
        }
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3254 = {
            let a3252 = v0;
            let a3253 = f130(v1);
            nat_add(a3252, a3253)?
        };
        let a3255 = f130(v2);
        nat_add(a3254, a3255)?
    };
    let v5: u64 = {
        let a3256 = f129(v1);
        let a3257 = f129(v2);
        nat_add(a3256, a3257)?
    };
    let a3258 = v1;
    let a3259 = 4294967295u64;
    if if nat_le(a3258, a3259) {
        let a3260 = v2;
        let a3261 = 4294967295u64;
        nat_le(a3260, a3261)
    } else {
        false
    } {
        let a3262 = v5;
        let a3263 = v4;
        if nat_le(a3262, a3263) {
            let a3266 = v3;
            let a3267 = {
                let a3264 = v4;
                let a3265 = v5;
                nat_sub(a3264, a3265)
            };
            Ok(nat_eq(a3266, a3267))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f118(v0: u64, v1: u64) -> R<bool> {
    let a3268 = v1;
    let a3269 = 4294967295u64;
    if nat_le(a3268, a3269) {
        let a3272 = {
            let a3270 = f130(v1);
            let a3271 = f129(v1);
            nat_add(a3270, a3271)?
        };
        let a3273 = v0;
        Ok(nat_lt(a3272, a3273))
    } else {
        Ok(false)
    }
}

pub fn f119(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f190(v0.clone(), false)?;
    let v6: u64 = f190(v0, true)?;
    let a3274 = v6;
    let a3275 = v1;
    if if nat_le(a3274, a3275) {
        let a3282 = {
            let a3280 = {
                let a3276 = v1;
                let a3277 = v5;
                nat_add(a3276, a3277)?
            };
            let a3281 = {
                let a3278 = v3;
                let a3279 = 0u64;
                if nat_eq(a3278, a3279) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3280, a3281)?
        };
        let a3283 = v2;
        nat_le(a3282, a3283)
    } else {
        false
    } {
        if v4 {
            let a3288 = {
                let a3286 = {
                    let a3284 = v1;
                    let a3285 = v5;
                    nat_add(a3284, a3285)?
                };
                let a3287 = v6;
                nat_sub(a3286, a3287)
            };
            let a3289 = v2;
            Ok(nat_lt(a3288, a3289))
        } else {
            let a3294 = {
                let a3292 = {
                    let a3290 = v1;
                    let a3291 = v5;
                    nat_add(a3290, a3291)?
                };
                let a3293 = v6;
                nat_sub(a3292, a3293)
            };
            let a3295 = v2;
            Ok(nat_le(a3294, a3295))
        }
    } else {
        Ok(false)
    }
}

pub fn f120(v0: u64, v1: u64) -> bool {
    let a3296 = v0;
    let a3297 = 0u64;
    if if nat_eq(a3296, a3297) {
        true
    } else {
        let a3298 = v0;
        let a3299 = 1u64;
        nat_eq(a3298, a3299)
    } {
        let a3300 = v1;
        let a3301 = 4294967296u64;
        nat_lt(a3300, a3301)
    } else {
        false
    }
}

pub fn f121(v0: u64, v1: u64) -> R<u64> {
    let a3306 = {
        let a3304 = v0;
        let a3305 = {
            let a3302 = v1;
            let a3303 = 1u64;
            nat_sub(a3302, a3303)
        };
        nat_add(a3304, a3305)?
    };
    let a3307 = v1;
    let a3308 = 0u64;
    Ok(nat_quot(a3306, a3307, a3308))
}

pub fn f122(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f33(match v0.clone() {
        Adt2::C0(_, h3309, _, _, _, _, _, _, _, _, _) => {
            h3309
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3310, _, _, _, _, _, _, _, _) => {
            h3310
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3311, _, _, _, _, _, _, _) => {
            h3311
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3312, _, _, _, _, _, _) => {
            h3312
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3315 = match v1.clone() {
                Adt2::C0(_, h3313, _, _, _, _, _, _, _, _, _) => {
                    h3313
                }
            };
            let a3316 = match v1.clone() {
                Adt2::C0(_, _, h3314, _, _, _, _, _, _, _, _) => {
                    h3314
                }
            };
            nat_mul(a3315, a3316)?
        };
        let v5: u64 = {
            let a3318 = v4;
            let a3319 = match v1.clone() {
                Adt2::C0(_, _, _, h3317, _, _, _, _, _, _, _) => {
                    h3317
                }
            };
            nat_mul(a3318, a3319)?
        };
        let a3332 = {
            let a3330 = {
                let a3321 = match v0.clone() {
                    Adt2::C0(_, h3320, _, _, _, _, _, _, _, _, _) => {
                        h3320
                    }
                };
                let a3322 = 1u64;
                nat_sub(a3321, a3322)
            };
            let a3331 = {
                let a3328 = f191(match v0.clone() {
                    Adt2::C0(_, _, h3323, _, _, _, _, _, _, _, _) => {
                        h3323
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3324, _, _, _, _, _, _, _, _, _) => {
                        h3324
                    }
                })?;
                let a3329 = f191(match v0 {
                    Adt2::C0(_, _, _, h3325, _, _, _, _, _, _, _) => {
                        h3325
                    }
                }, v2, {
                    let a3326 = v3;
                    let a3327 = v5;
                    nat_mul(a3326, a3327)?
                }, v4)?;
                nat_add(a3328, a3329)?
            };
            nat_add(a3330, a3331)?
        };
        let a3333 = f16(v1, 1u64)?;
        Ok(nat_lt(a3332, a3333))
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3334 = v0;
    let a3335 = 2147483648u64;
    if if if if nat_lt(a3334, a3335) {
        let a3336 = v1;
        let a3337 = 2147483648u64;
        nat_lt(a3336, a3337)
    } else {
        false
    } {
        let a3340 = {
            let a3338 = v0;
            let a3339 = v1;
            nat_mul(a3338, a3339)?
        };
        let a3341 = 2147483648u64;
        nat_lt(a3340, a3341)
    } else {
        false
    } {
        let a3346 = {
            let a3344 = {
                let a3342 = v0;
                let a3343 = 1u64;
                nat_sub(a3342, a3343)
            };
            let a3345 = v2;
            nat_mul(a3344, a3345)?
        };
        let a3347 = 2147483648u64;
        nat_lt(a3346, a3347)
    } else {
        false
    } {
        let a3352 = {
            let a3350 = {
                let a3348 = v1;
                let a3349 = 1u64;
                nat_sub(a3348, a3349)
            };
            let a3351 = v2;
            nat_mul(a3350, a3351)?
        };
        let a3353 = 2147483648u64;
        Ok(nat_lt(a3352, a3353))
    } else {
        Ok(false)
    }
}

pub fn f124(v0: u64) -> R<u64> {
    let a3356 = 3u64;
    let a3357 = {
        let a3354 = v0;
        let a3355 = 3u64;
        nat_mul(a3354, a3355)?
    };
    nat_add(a3356, a3357)
}

pub fn f125(v0: Adt3) -> R<u64> {
    if f33(match v0.clone() {
        Adt3::C0(h3358, _, _, _, _, _, _, _) => {
            h3358
        }
    }, match v0.clone() {
        Adt3::C0(_, h3359, _, _, _, _, _, _) => {
            h3359
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3360, _, _, _, _, _) => {
            h3360
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3361, _, _, _, _) => {
            h3361
        }
    }) {
        Ok(0u64)
    } else {
        let a3370 = match v0.clone() {
            Adt3::C0(h3362, _, _, _, _, _, _, _) => {
                h3362
            }
        };
        let a3371 = {
            let a3368 = match v0.clone() {
                Adt3::C0(_, h3363, _, _, _, _, _, _) => {
                    h3363
                }
            };
            let a3369 = {
                let a3366 = match v0.clone() {
                    Adt3::C0(_, _, h3364, _, _, _, _, _) => {
                        h3364
                    }
                };
                let a3367 = match v0 {
                    Adt3::C0(_, _, _, h3365, _, _, _, _) => {
                        h3365
                    }
                };
                nat_mul(a3366, a3367)?
            };
            nat_mul(a3368, a3369)?
        };
        nat_mul(a3370, a3371)
    }
}

pub fn f126(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3372 = v1;
    let a3373 = 13u64;
    if if nat_eq(a3372, a3373) {
        f33(match v0.clone() {
            Adt3::C0(h3374, _, _, _, _, _, _, _) => {
                h3374
            }
        }, match v0.clone() {
            Adt3::C0(_, h3375, _, _, _, _, _, _) => {
                h3375
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3376, _, _, _, _, _) => {
                h3376
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3377, _, _, _, _) => {
                h3377
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3383 = match v0.clone() {
                Adt3::C0(_, h3378, _, _, _, _, _, _) => {
                    h3378
                }
            };
            let a3384 = {
                let a3381 = match v0.clone() {
                    Adt3::C0(_, _, h3379, _, _, _, _, _) => {
                        h3379
                    }
                };
                let a3382 = match v0.clone() {
                    Adt3::C0(_, _, _, h3380, _, _, _, _) => {
                        h3380
                    }
                };
                nat_mul(a3381, a3382)?
            };
            nat_mul(a3383, a3384)?
        };
        let a3385 = v1;
        let a3386 = 13u64;
        if nat_eq(a3385, a3386) {
            let a3395 = 2u64;
            let a3396 = if f192(v0.clone(), v2) {
                let a3388 = v3;
                let a3389 = f193(match v0 {
                    Adt3::C0(h3387, _, _, _, _, _, _, _) => {
                        h3387
                    }
                }, false)?;
                nat_mul(a3388, a3389)?
            } else {
                let a3393 = v3;
                let a3394 = {
                    let a3391 = match v0 {
                        Adt3::C0(h3390, _, _, _, _, _, _, _) => {
                            h3390
                        }
                    };
                    let a3392 = 2u64;
                    nat_mul(a3391, a3392)?
                };
                nat_mul(a3393, a3394)?
            };
            nat_add(a3395, a3396)
        } else {
            let a3400 = v3;
            let a3401 = f193(match v0 {
                Adt3::C0(h3397, _, _, _, _, _, _, _) => {
                    h3397
                }
            }, {
                let a3398 = v1;
                let a3399 = 16u64;
                nat_eq(a3398, a3399)
            })?;
            nat_mul(a3400, a3401)
        }
    }
}

pub fn f127(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3402 = v0;
    let a3403 = 30u64;
    if if nat_eq(a3402, a3403) {
        let a3406 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3404, _, _, _, _) => {
                h3404
            }
        };
        let a3407 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3405, _, _, _, _, _) => {
                h3405
            }
        };
        nat_lt(a3406, a3407)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3408 = v3;
        match m3408 {
            None => {
                f194(v0, v1, match v2.clone() {
                    Adt2::C0(h3409, _, _, _, _, _, _, _, _, _, _) => {
                        h3409
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3410, _, _, _, _, _, _, _, _, _, _) => {
                        h3410
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3411, _, _, _, _, _, _, _, _, _) => {
                        h3411
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3412, _, _, _, _, _, _, _, _) => {
                        h3412
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3413, _, _, _, _, _, _, _) => {
                        h3413
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3414, _, _, _, _, _, _) => {
                        h3414
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3415, _, _, _, _, _, _, _, _, _) => {
                        h3415
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3416, _, _, _, _, _, _, _, _) => {
                        h3416
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3417, _, _, _, _, _, _, _) => {
                        h3417
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3418, _, _, _, _, _, _) => {
                        h3418
                    }
                })
            }
            Some(v5) => {
                f194(v0, v1, match v2.clone() {
                    Adt2::C0(h3419, _, _, _, _, _, _, _, _, _, _) => {
                        h3419
                    }
                }, match v5.clone() {
                    Adt2::C0(h3420, _, _, _, _, _, _, _, _, _, _) => {
                        h3420
                    }
                }, match v4.clone() {
                    Adt2::C0(h3421, _, _, _, _, _, _, _, _, _, _) => {
                        h3421
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3422, _, _, _, _, _, _, _, _, _) => {
                        h3422
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3423, _, _, _, _, _, _, _, _) => {
                        h3423
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3424, _, _, _, _, _, _, _) => {
                        h3424
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3425, _, _, _, _, _, _) => {
                        h3425
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3426, _, _, _, _, _, _, _, _, _) => {
                        h3426
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3427, _, _, _, _, _, _, _, _) => {
                        h3427
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3428, _, _, _, _, _, _, _) => {
                        h3428
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3429, _, _, _, _, _, _) => {
                        h3429
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3430, _, _, _, _, _, _, _, _, _) => {
                        h3430
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3431, _, _, _, _, _, _, _, _) => {
                        h3431
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3432, _, _, _, _, _, _, _) => {
                        h3432
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3433, _, _, _, _, _, _) => {
                        h3433
                    }
                })
            }
        }
    }
}

pub fn f128(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3446 = {
        let a3442 = {
            let a3438 = {
                let a3434 = v0;
                let a3435 = v4;
                nat_mul(a3434, a3435)?
            };
            let a3439 = {
                let a3436 = v1;
                let a3437 = v5;
                nat_mul(a3436, a3437)?
            };
            nat_add(a3438, a3439)?
        };
        let a3443 = {
            let a3440 = v2;
            let a3441 = v6;
            nat_mul(a3440, a3441)?
        };
        nat_add(a3442, a3443)?
    };
    let a3447 = {
        let a3444 = v3;
        let a3445 = v7;
        nat_mul(a3444, a3445)?
    };
    nat_add(a3446, a3447)
}

pub fn f129(v0: u64) -> u64 {
    let a3448 = v0;
    let a3449 = 2147483648u64;
    if nat_lt(a3448, a3449) {
        0u64
    } else {
        let a3450 = 4294967296u64;
        let a3451 = v0;
        nat_sub(a3450, a3451)
    }
}

pub fn f130(v0: u64) -> u64 {
    let a3452 = v0;
    let a3453 = 2147483648u64;
    if nat_lt(a3452, a3453) {
        v0
    } else {
        0u64
    }
}

pub fn f131(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3454 = v7;
        let a3455 = 6u64;
        if nat_eq(a3454, a3455) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f88(v3, v4), f88(v5, v6), f89(v7, v8), f195(v1), f195(v9))
}

pub fn f132(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3456 = v0;
    match m3456 {
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
            let a3457 = 13u64;
            let a3458 = v36;
            if nat_le(a3457, a3458) {
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
                Adt5::C0(_, _, _, _, _, h3459, _, _) => {
                    h3459
                }
            }, match v56 {
                Adt5::C0(_, _, _, _, _, _, _, h3460) => {
                    h3460
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

pub fn f133(v0: Adt4) -> u64 {
    let m3461 = v0;
    match m3461 {
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

pub fn f134(v0: Adt4, v1: u64) -> u64 {
    let m3462 = v0;
    match m3462 {
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
            let a3463 = v1;
            let a3464 = 6u64;
            if nat_eq(a3463, a3464) {
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
            let a3473 = v1;
            let a3474 = 6u64;
            if if if nat_eq(a3473, a3474) {
                true
            } else {
                let a3475 = v1;
                let a3476 = 7u64;
                nat_eq(a3475, a3476)
            } {
                let a3477 = v56;
                let a3478 = 30u64;
                nat_eq(a3477, a3478)
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
                Adt5::C0(_, _, _, _, _, _, _, h3472) => {
                    h3472
                }
            }
        }
        Adt4::C28(v51) => {
            let a3465 = v1;
            let a3466 = 6u64;
            if nat_eq(a3465, a3466) {
                match v51 {
                    Adt7::C0(_, _, _, _, h3467) => {
                        h3467
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
            let a3468 = v1;
            let a3469 = 6u64;
            if if nat_eq(a3468, a3469) {
                true
            } else {
                let a3470 = v1;
                let a3471 = 7u64;
                nat_eq(a3470, a3471)
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

pub fn f135(v0: Adt4, v1: u64) -> Adt11 {
    let a3479 = v1;
    let a3480 = 4u64;
    if nat_lt(a3479, a3480) {
        Adt11::C0(v1)
    } else {
        let a3481 = v1;
        let a3482 = 4u64;
        if nat_eq(a3481, a3482) {
            Adt11::C1(0u64)
        } else {
            let a3483 = v1;
            let a3484 = 5u64;
            if nat_eq(a3483, a3484) {
                Adt11::C1(3u64)
            } else {
                let a3485 = v1;
                let a3486 = 6u64;
                if nat_eq(a3485, a3486) {
                    Adt11::C2(f134(v0, v1), 0u64, false)
                } else {
                    let a3487 = v1;
                    let a3488 = 7u64;
                    if nat_eq(a3487, a3488) {
                        Adt11::C2(0u64, f134(v0, v1), false)
                    } else {
                        let a3489 = v1;
                        let a3490 = 16u64;
                        if nat_lt(a3489, a3490) {
                            Adt11::C1({
                                let a3491 = v1;
                                let a3492 = 8u64;
                                nat_sub(a3491, a3492)
                            })
                        } else {
                            let a3493 = v1;
                            let a3494 = 20u64;
                            if nat_lt(a3493, a3494) {
                                Adt11::C5({
                                    let a3495 = v1;
                                    let a3496 = 16u64;
                                    nat_sub(a3495, a3496)
                                })
                            } else {
                                let a3497 = v1;
                                let a3498 = 20u64;
                                if nat_eq(a3497, a3498) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3499 = v1;
                                    let a3500 = 21u64;
                                    if nat_eq(a3499, a3500) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3501 = v1;
                                        let a3502 = 22u64;
                                        if nat_eq(a3501, a3502) {
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

pub fn f136(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3504 = v0;
        let a3505 = match v1.clone() {
            Adt3::C0(h3503, _, _, _, _, _, _, _) => {
                h3503
            }
        };
        let a3506 = 0u64;
        nat_quot(a3504, a3505, a3506)
    };
    let v3: u64 = {
        let a3508 = v2;
        let a3509 = match v1.clone() {
            Adt3::C0(_, h3507, _, _, _, _, _, _) => {
                h3507
            }
        };
        let a3510 = 0u64;
        nat_quot(a3508, a3509, a3510)
    };
    f128({
        let a3512 = v0;
        let a3513 = match v1.clone() {
            Adt3::C0(h3511, _, _, _, _, _, _, _) => {
                h3511
            }
        };
        let a3514 = 0u64;
        nat_rem(a3512, a3513, a3514)
    }, {
        let a3516 = v2;
        let a3517 = match v1.clone() {
            Adt3::C0(_, h3515, _, _, _, _, _, _) => {
                h3515
            }
        };
        let a3518 = 0u64;
        nat_rem(a3516, a3517, a3518)
    }, {
        let a3520 = v3;
        let a3521 = match v1.clone() {
            Adt3::C0(_, _, h3519, _, _, _, _, _) => {
                h3519
            }
        };
        let a3522 = 0u64;
        nat_rem(a3520, a3521, a3522)
    }, {
        let a3524 = v3;
        let a3525 = match v1.clone() {
            Adt3::C0(_, _, h3523, _, _, _, _, _) => {
                h3523
            }
        };
        let a3526 = 0u64;
        nat_quot(a3524, a3525, a3526)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3527, _, _, _) => {
            h3527
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3528, _, _) => {
            h3528
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3529, _) => {
            h3529
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3530) => {
            h3530
        }
    })
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3548 = {
        let a3543 = f196(v2, {
            let a3534 = {
                let a3531 = v0;
                let a3532 = v1;
                let a3533 = 0u64;
                nat_quot(a3531, a3532, a3533)
            };
            let a3535 = v2;
            let a3536 = 0u64;
            nat_rem(a3534, a3535, a3536)
        }, {
            let a3540 = {
                let a3537 = v0;
                let a3538 = v1;
                let a3539 = 0u64;
                nat_quot(a3537, a3538, a3539)
            };
            let a3541 = v2;
            let a3542 = 0u64;
            nat_quot(a3540, a3541, a3542)
        })?;
        let a3544 = v1;
        nat_mul(a3543, a3544)?
    };
    let a3549 = {
        let a3545 = v0;
        let a3546 = v1;
        let a3547 = 0u64;
        nat_rem(a3545, a3546, a3547)
    };
    nat_add(a3548, a3549)
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3551 = v0;
        let a3552 = match v2.clone() {
            Adt3::C0(h3550, _, _, _, _, _, _, _) => {
                h3550
            }
        };
        let a3553 = 0u64;
        nat_quot(a3551, a3552, a3553)
    };
    let v4: u64 = {
        let a3555 = v3;
        let a3556 = match v2.clone() {
            Adt3::C0(_, h3554, _, _, _, _, _, _) => {
                h3554
            }
        };
        let a3557 = 0u64;
        nat_quot(a3555, a3556, a3557)
    };
    f197({
        let a3559 = v0;
        let a3560 = match v2.clone() {
            Adt3::C0(h3558, _, _, _, _, _, _, _) => {
                h3558
            }
        };
        let a3561 = 0u64;
        nat_rem(a3559, a3560, a3561)
    }, {
        let a3563 = v3;
        let a3564 = match v2.clone() {
            Adt3::C0(_, h3562, _, _, _, _, _, _) => {
                h3562
            }
        };
        let a3565 = 0u64;
        nat_rem(a3563, a3564, a3565)
    }, {
        let a3567 = v4;
        let a3568 = match v2.clone() {
            Adt3::C0(_, _, h3566, _, _, _, _, _) => {
                h3566
            }
        };
        let a3569 = 0u64;
        nat_rem(a3567, a3568, a3569)
    }, {
        let a3571 = v4;
        let a3572 = match v2 {
            Adt3::C0(_, _, h3570, _, _, _, _, _) => {
                h3570
            }
        };
        let a3573 = 0u64;
        nat_quot(a3571, a3572, a3573)
    }, match v1.clone() {
        Adt3::C0(h3574, _, _, _, _, _, _, _) => {
            h3574
        }
    }, match v1.clone() {
        Adt3::C0(_, h3575, _, _, _, _, _, _) => {
            h3575
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3576, _, _, _, _, _) => {
            h3576
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3577, _, _, _, _) => {
            h3577
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3578, _, _, _) => {
            h3578
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3579, _, _) => {
            h3579
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3580, _) => {
            h3580
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3581) => {
            h3581
        }
    })
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3583 = v0;
        let a3584 = match v3.clone() {
            Adt3::C0(h3582, _, _, _, _, _, _, _) => {
                h3582
            }
        };
        let a3585 = 0u64;
        nat_quot(a3583, a3584, a3585)
    };
    let v6: u64 = {
        let a3587 = v5;
        let a3588 = match v3.clone() {
            Adt3::C0(_, h3586, _, _, _, _, _, _) => {
                h3586
            }
        };
        let a3589 = 0u64;
        nat_quot(a3587, a3588, a3589)
    };
    let v7: u64 = {
        let a3591 = v0;
        let a3592 = match v3.clone() {
            Adt3::C0(h3590, _, _, _, _, _, _, _) => {
                h3590
            }
        };
        let a3593 = 0u64;
        nat_rem(a3591, a3592, a3593)
    };
    let v8: u64 = {
        let a3595 = v5;
        let a3596 = match v3.clone() {
            Adt3::C0(_, h3594, _, _, _, _, _, _) => {
                h3594
            }
        };
        let a3597 = 0u64;
        nat_rem(a3595, a3596, a3597)
    };
    let v9: u64 = {
        let a3599 = v6;
        let a3600 = match v3.clone() {
            Adt3::C0(_, _, h3598, _, _, _, _, _) => {
                h3598
            }
        };
        let a3601 = 0u64;
        nat_rem(a3599, a3600, a3601)
    };
    let v10: u64 = {
        let a3603 = v6;
        let a3604 = match v3.clone() {
            Adt3::C0(_, _, h3602, _, _, _, _, _) => {
                h3602
            }
        };
        let a3605 = 0u64;
        nat_quot(a3603, a3604, a3605)
    };
    let v11: u64 = {
        let a3606 = v4;
        let a3607 = 0u64;
        if nat_eq(a3606, a3607) {
            match v1.clone() {
                Adt3::C0(h3608, _, _, _, _, _, _, _) => {
                    h3608
                }
            }
        } else {
            let a3609 = v4;
            let a3610 = 1u64;
            if nat_eq(a3609, a3610) {
                match v1.clone() {
                    Adt3::C0(_, h3611, _, _, _, _, _, _) => {
                        h3611
                    }
                }
            } else {
                let a3612 = v4;
                let a3613 = 2u64;
                if nat_eq(a3612, a3613) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3614, _, _, _, _, _) => {
                            h3614
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3615, _, _, _, _) => {
                            h3615
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3622 = v11;
        let a3623 = {
            let a3616 = v4;
            let a3617 = 0u64;
            if nat_eq(a3616, a3617) {
                v7
            } else {
                let a3618 = v4;
                let a3619 = 1u64;
                if nat_eq(a3618, a3619) {
                    v8
                } else {
                    let a3620 = v4;
                    let a3621 = 2u64;
                    if nat_eq(a3620, a3621) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3622, a3623)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f128(if if v12 {
        let a3624 = v4;
        let a3625 = 0u64;
        nat_eq(a3624, a3625)
    } else {
        false
    } {
        let a3626 = v7;
        let a3627 = v11;
        nat_sub(a3626, a3627)
    } else {
        v7
    }, if if v12 {
        let a3628 = v4;
        let a3629 = 1u64;
        nat_eq(a3628, a3629)
    } else {
        false
    } {
        let a3630 = v8;
        let a3631 = v11;
        nat_sub(a3630, a3631)
    } else {
        v8
    }, if if v12 {
        let a3632 = v4;
        let a3633 = 2u64;
        nat_eq(a3632, a3633)
    } else {
        false
    } {
        let a3634 = v9;
        let a3635 = v11;
        nat_sub(a3634, a3635)
    } else {
        v9
    }, if if v12 {
        let a3636 = v4;
        let a3637 = 3u64;
        nat_eq(a3636, a3637)
    } else {
        false
    } {
        let a3638 = v10;
        let a3639 = v11;
        nat_sub(a3638, a3639)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3640, _, _, _) => {
            h3640
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3641, _, _) => {
            h3641
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3642, _) => {
            h3642
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3643) => {
            h3643
        }
    })?, f136(v0, v3)?))
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f69(v0, v2.clone(), 0u64);
    let v9: u64 = f69(v0, v2.clone(), 1u64);
    let v10: u64 = f69(v0, v2.clone(), 2u64);
    let v11: u64 = f69(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f68(v1.clone(), f198(v8, v3, match v1.clone() {
            Adt3::C0(h3644, _, _, _, _, _, _, _) => {
                h3644
            }
        })?, f198(v9, v4, match v1.clone() {
            Adt3::C0(_, h3645, _, _, _, _, _, _) => {
                h3645
            }
        })?, f198(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3646, _, _, _, _, _) => {
                h3646
            }
        })?, f198(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3647, _, _, _, _) => {
                h3647
            }
        })?)?, f136(v0, v2)?))
    } else {
        if if if if f199(v8, v3, match v1.clone() {
            Adt3::C0(h3648, _, _, _, _, _, _, _) => {
                h3648
            }
        })? {
            f199(v9, v4, match v1.clone() {
                Adt3::C0(_, h3649, _, _, _, _, _, _) => {
                    h3649
                }
            })?
        } else {
            false
        } {
            f199(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3650, _, _, _, _, _) => {
                    h3650
                }
            })?
        } else {
            false
        } {
            f199(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3651, _, _, _, _) => {
                    h3651
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f68(v1, f200(v8, v3)?, f200(v9, v4)?, f200(v10, v5)?, f200(v11, v6)?)?, f136(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f136(v0, v2)?))
        }
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f69(v0, v2.clone(), 0u64);
    let v5: u64 = f69(v0, v2.clone(), 1u64);
    let v6: u64 = f69(v0, v2.clone(), 2u64);
    let v7: u64 = f69(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f68(v1.clone(), f201(v4, v3, match v1 {
        Adt3::C0(h3652, _, _, _, _, _, _, _) => {
            h3652
        }
    })?, v5, v6, v7)?, f136(v0, v2)?))
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3653 = v0;
    let a3654 = v3;
    if nat_lt(a3653, a3654) {
        Ok(Adt9::C1(false, {
            let a3655 = v0;
            let a3656 = 4u64;
            nat_mul(a3655, a3656)?
        }, {
            let a3657 = v0;
            let a3658 = 4u64;
            nat_mul(a3657, a3658)?
        }))
    } else {
        let v6: u64 = {
            let a3659 = v0;
            let a3660 = v3;
            nat_sub(a3659, a3660)
        };
        let v7: u64 = f136(v6, v1)?;
        let v8: u64 = f202(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3661 = v0;
    let a3662 = v3;
    if nat_lt(a3661, a3662) {
        Ok(Adt9::C1(true, 0u64, f136(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3663 = v0;
            let a3664 = v3;
            nat_sub(a3663, a3664)
        };
        let v5: u64 = f138(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f136(v4, v1)?, 3u64, v5))
    }
}

pub fn f144(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3666 = v1;
        let a3667 = match v3.clone() {
            Adt3::C0(h3665, _, _, _, _, _, _, _) => {
                h3665
            }
        };
        let a3668 = 0u64;
        nat_rem(a3666, a3667, a3668)
    };
    let v5: u64 = {
        let a3674 = {
            let a3670 = v1;
            let a3671 = match v3.clone() {
                Adt3::C0(h3669, _, _, _, _, _, _, _) => {
                    h3669
                }
            };
            let a3672 = 0u64;
            nat_quot(a3670, a3671, a3672)
        };
        let a3675 = match v3.clone() {
            Adt3::C0(_, h3673, _, _, _, _, _, _) => {
                h3673
            }
        };
        let a3676 = 0u64;
        nat_rem(a3674, a3675, a3676)
    };
    let v6: bool = f203(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f138(v1, v2, v3.clone())?
    } else {
        0u64
    }, f136(v1, v3)?))
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3677 = v3;
        let a3678 = 8u64;
        nat_add(a3677, a3678)?
    }, 0u64, f136(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f138(v0, v5, v2.clone())?
    }, 3u64, f136(v0, v2)?))
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3679 = v4;
    let a3680 = 0u64;
    if if nat_eq(a3679, a3680) {
        f49(v3)
    } else {
        false
    } {
        let m3681 = f112(v3);
        match m3681 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3682, _) => {
                        h3682
                    }
                };
                Ok(Adt9::C5(v3, f136(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3683) => {
                        h3683
                    }
                }, f136({
                    let a3684 = v0;
                    let a3685 = v6;
                    nat_mul(a3684, a3685)?
                }, v2.clone())?, {
                    let a3691 = {
                        let a3689 = {
                            let a3686 = v6;
                            let a3687 = 1u64;
                            nat_sub(a3686, a3687)
                        };
                        let a3690 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3688, _, _, _) => {
                                h3688
                            }
                        };
                        nat_mul(a3689, a3690)?
                    };
                    let a3692 = 4u64;
                    nat_add(a3691, a3692)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3693, _, _, _) => {
                        h3693
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f136(v0, v1)?, f136(v0, v2)?))
    }
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3695 = v0;
        let a3696 = match v2.clone() {
            Adt3::C0(h3694, _, _, _, _, _, _, _) => {
                h3694
            }
        };
        let a3697 = 0u64;
        nat_rem(a3695, a3696, a3697)
    };
    let v7: u64 = {
        let a3699 = v0;
        let a3700 = match v2.clone() {
            Adt3::C0(h3698, _, _, _, _, _, _, _) => {
                h3698
            }
        };
        let a3701 = 0u64;
        nat_quot(a3699, a3700, a3701)
    };
    let v8: u64 = {
        let a3703 = v7;
        let a3704 = match v2.clone() {
            Adt3::C0(_, h3702, _, _, _, _, _, _) => {
                h3702
            }
        };
        let a3705 = 0u64;
        nat_rem(a3703, a3704, a3705)
    };
    let v9: u64 = {
        let a3707 = v7;
        let a3708 = match v2.clone() {
            Adt3::C0(_, h3706, _, _, _, _, _, _) => {
                h3706
            }
        };
        let a3709 = 0u64;
        nat_quot(a3707, a3708, a3709)
    };
    let v10: u64 = {
        let a3711 = v9;
        let a3712 = match v2.clone() {
            Adt3::C0(_, _, h3710, _, _, _, _, _) => {
                h3710
            }
        };
        let a3713 = 0u64;
        nat_rem(a3711, a3712, a3713)
    };
    let v11: u64 = {
        let a3715 = v9;
        let a3716 = match v2.clone() {
            Adt3::C0(_, _, h3714, _, _, _, _, _) => {
                h3714
            }
        };
        let a3717 = 0u64;
        nat_quot(a3715, a3716, a3717)
    };
    let v12: u64 = f136(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3726 = {
                let a3721 = {
                    let a3718 = v10;
                    let a3719 = v4;
                    let a3720 = 0u64;
                    nat_quot(a3718, a3719, a3720)
                };
                let a3722 = v5;
                nat_mul(a3721, a3722)?
            };
            let a3727 = {
                let a3723 = v8;
                let a3724 = v4;
                let a3725 = 0u64;
                nat_quot(a3723, a3724, a3725)
            };
            nat_add(a3726, a3727)?
        };
        let v14: u64 = {
            let a3747 = v6;
            let a3748 = {
                let a3745 = match v1.clone() {
                    Adt3::C0(h3728, _, _, _, _, _, _, _) => {
                        h3728
                    }
                };
                let a3746 = {
                    let a3743 = {
                        let a3729 = v8;
                        let a3730 = v4;
                        let a3731 = 0u64;
                        nat_rem(a3729, a3730, a3731)
                    };
                    let a3744 = {
                        let a3741 = match v1.clone() {
                            Adt3::C0(_, h3732, _, _, _, _, _, _) => {
                                h3732
                            }
                        };
                        let a3742 = {
                            let a3739 = {
                                let a3733 = v10;
                                let a3734 = v4;
                                let a3735 = 0u64;
                                nat_rem(a3733, a3734, a3735)
                            };
                            let a3740 = {
                                let a3737 = match v1.clone() {
                                    Adt3::C0(_, _, h3736, _, _, _, _, _) => {
                                        h3736
                                    }
                                };
                                let a3738 = v13;
                                nat_mul(a3737, a3738)?
                            };
                            nat_add(a3739, a3740)?
                        };
                        nat_mul(a3741, a3742)?
                    };
                    nat_add(a3743, a3744)?
                };
                nat_mul(a3745, a3746)?
            };
            nat_add(a3747, a3748)?
        };
        Ok(Adt9::C1(false, f136(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3754 = {
                let a3752 = {
                    let a3749 = v11;
                    let a3750 = v5;
                    let a3751 = 0u64;
                    nat_rem(a3749, a3750, a3751)
                };
                let a3753 = v4;
                nat_mul(a3752, a3753)?
            };
            let a3755 = v8;
            nat_add(a3754, a3755)?
        };
        let v16: u64 = {
            let a3761 = {
                let a3759 = {
                    let a3756 = v11;
                    let a3757 = v5;
                    let a3758 = 0u64;
                    nat_quot(a3756, a3757, a3758)
                };
                let a3760 = v4;
                nat_mul(a3759, a3760)?
            };
            let a3762 = v10;
            nat_add(a3761, a3762)?
        };
        let a3764 = v15;
        let a3765 = match v1.clone() {
            Adt3::C0(_, h3763, _, _, _, _, _, _) => {
                h3763
            }
        };
        if if nat_lt(a3764, a3765) {
            let a3767 = v16;
            let a3768 = match v1.clone() {
                Adt3::C0(_, _, h3766, _, _, _, _, _) => {
                    h3766
                }
            };
            nat_lt(a3767, a3768)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f136({
                let a3777 = v6;
                let a3778 = {
                    let a3775 = match v1.clone() {
                        Adt3::C0(h3769, _, _, _, _, _, _, _) => {
                            h3769
                        }
                    };
                    let a3776 = {
                        let a3773 = v15;
                        let a3774 = {
                            let a3771 = match v1.clone() {
                                Adt3::C0(_, h3770, _, _, _, _, _, _) => {
                                    h3770
                                }
                            };
                            let a3772 = v16;
                            nat_mul(a3771, a3772)?
                        };
                        nat_add(a3773, a3774)?
                    };
                    nat_mul(a3775, a3776)?
                };
                nat_add(a3777, a3778)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f136(v0, v2.clone())?;
    let v4: u64 = f136(v0, v1.clone())?;
    let a3783 = {
        let a3780 = v0;
        let a3781 = match v1 {
            Adt3::C0(h3779, _, _, _, _, _, _, _) => {
                h3779
            }
        };
        let a3782 = 0u64;
        nat_rem(a3780, a3781, a3782)
    };
    let a3784 = 0u64;
    if nat_eq(a3783, a3784) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f136({
            let a3785 = v0;
            let a3786 = 1u64;
            nat_sub(a3785, a3786)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3787 = v3;
    let a3788 = 13u64;
    if nat_eq(a3787, a3788) {
        f204(v0, v1, v4)
    } else {
        let v5: u64 = f193(match v1.clone() {
            Adt3::C0(h3789, _, _, _, _, _, _, _) => {
                h3789
            }
        }, {
            let a3790 = v3;
            let a3791 = 16u64;
            nat_eq(a3790, a3791)
        })?;
        let v6: u64 = {
            let a3792 = v0;
            let a3793 = v5;
            let a3794 = 0u64;
            nat_quot(a3792, a3793, a3794)
        };
        let v7: u64 = {
            let a3795 = v0;
            let a3796 = v5;
            let a3797 = 0u64;
            nat_rem(a3795, a3796, a3797)
        };
        let v8: u64 = {
            let a3801 = 1u64;
            let a3802 = {
                let a3799 = match v1.clone() {
                    Adt3::C0(h3798, _, _, _, _, _, _, _) => {
                        h3798
                    }
                };
                let a3800 = 2u64;
                nat_mul(a3799, a3800)?
            };
            nat_add(a3801, a3802)?
        };
        let a3803 = v7;
        let a3804 = v8;
        if nat_lt(a3803, a3804) {
            f205(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f136(v6, v2)?;
            let a3805 = v3;
            let a3806 = 14u64;
            if nat_eq(a3805, a3806) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3807 = v7;
                let a3808 = v8;
                if nat_eq(a3807, a3808) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3811 = v7;
                    let a3812 = {
                        let a3809 = v8;
                        let a3810 = 1u64;
                        nat_add(a3809, a3810)?
                    };
                    if nat_eq(a3811, a3812) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3813, _, _, _, _, _, _, _) => {
                                h3813
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

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3814 = v0;
    let a3815 = 0u64;
    if nat_eq(a3814, a3815) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3822 = {
            let a3816 = v0;
            let a3817 = 1u64;
            nat_add(a3816, a3817)?
        };
        let a3823 = {
            let a3820 = 2u64;
            let a3821 = {
                let a3818 = v3;
                let a3819 = 2u64;
                nat_mul(a3818, a3819)?
            };
            nat_add(a3820, a3821)?
        };
        if nat_eq(a3822, a3823) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3824 = v0;
                let a3825 = 1u64;
                nat_sub(a3824, a3825)
            };
            let v5: u64 = {
                let a3826 = v4;
                let a3827 = 2u64;
                let a3828 = 0u64;
                nat_quot(a3826, a3827, a3828)
            };
            let a3832 = {
                let a3829 = v4;
                let a3830 = 2u64;
                let a3831 = 0u64;
                nat_rem(a3829, a3830, a3831)
            };
            let a3833 = 0u64;
            if nat_eq(a3832, a3833) {
                Ok(Adt9::C4(17u64, 0u64, f136(v5, v1)?, 4u64, f136(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f151(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3834 = v0;
    let a3835 = f26(v1, v2, v3)?;
    if nat_lt(a3834, a3835) {
        let a3836 = v0;
        let a3837 = v1;
        if nat_lt(a3836, a3837) {
            Ok(Adt9::C1(false, {
                let a3838 = 4u64;
                let a3839 = v0;
                nat_mul(a3838, a3839)?
            }, {
                let a3840 = 4u64;
                let a3841 = v0;
                nat_mul(a3840, a3841)?
            }))
        } else {
            let v4: u64 = {
                let a3842 = v0;
                let a3843 = v1;
                nat_sub(a3842, a3843)
            };
            let v5: u64 = f206(v4, v2)?;
            let a3844 = v5;
            let a3845 = 2u64;
            if nat_lt(a3844, a3845) {
                Ok(Adt9::C4(6u64, {
                    let a3846 = v5;
                    let a3847 = 0u64;
                    if nat_eq(a3846, a3847) {
                        4u64
                    } else {
                        5u64
                    }
                }, f207(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f208(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f209(v4, v2)? {
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

pub fn f152(v0: u64, v1: u64) -> Adt11 {
    let a3848 = v0;
    let a3849 = 43u64;
    if nat_eq(a3848, a3849) {
        Adt11::C8(3u64)
    } else {
        let a3850 = v1;
        let a3851 = 43u64;
        if nat_eq(a3850, a3851) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f153(v0: u64, v1: u64) -> R<u64> {
    if f102(v0) {
        let a3852 = 2u64;
        let a3853 = v1;
        nat_mul(a3852, a3853)
    } else {
        let a3854 = v1;
        let a3855 = f189(v0);
        let a3856 = 0u64;
        Ok(nat_quot(a3854, a3855, a3856))
    }
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m3857 = f112(v0);
    match m3857 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f128(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h3858, _, _, _) => {
                    h3858
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h3859, _, _) => {
                    h3859
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h3860, _) => {
                    h3860
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h3861) => {
                    h3861
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h3862) => {
                    h3862
                }
            }, f128({
                let a3864 = v2;
                let a3865 = match v6.clone() {
                    Adt18::C0(_, h3863, _) => {
                        h3863
                    }
                };
                nat_mul(a3864, a3865)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h3866, _, _, _) => {
                    h3866
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h3867, _, _) => {
                    h3867
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h3868, _) => {
                    h3868
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h3869) => {
                    h3869
                }
            })?, match v6 {
                Adt18::C0(_, h3870, _) => {
                    h3870
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h3871, _, _, _) => {
                    h3871
                }
            }))
        }
    }
}

pub fn f155(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3879 = {
        let a3872 = 2u64;
        let a3873 = f125(v1.clone())?;
        nat_mul(a3872, a3873)?
    };
    let a3880 = {
        let a3877 = f125(v1)?;
        let a3878 = {
            let a3875 = 1u64;
            let a3876 = match v0 {
                Adt3::C0(h3874, _, _, _, _, _, _, _) => {
                    h3874
                }
            };
            nat_add(a3875, a3876)?
        };
        nat_mul(a3877, a3878)?
    };
    nat_add(a3879, a3880)
}

pub fn f156(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f136(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f157(v0: u64, v1: u64) -> bool {
    let a3881 = v1;
    let a3882 = 2147483647u64;
    if nat_le(a3881, a3882) {
        let a3883 = v1;
        let a3884 = v0;
        nat_lt(a3883, a3884)
    } else {
        false
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3885 = f112(v0);
    match m3885 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a3887 = v2;
                let a3888 = match v7.clone() {
                    Adt18::C0(_, h3886, _) => {
                        h3886
                    }
                };
                nat_mul(a3887, a3888)?
            };
            let m3901 = f210(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3889, _, _, _, _, _, _, _) => {
                    h3889
                }
            }, match v5.clone() {
                Adt3::C0(_, h3890, _, _, _, _, _, _) => {
                    h3890
                }
            }, match v5 {
                Adt3::C0(_, _, h3891, _, _, _, _, _) => {
                    h3891
                }
            }, {
                let a3894 = match v3.clone() {
                    Adt3::C0(h3892, _, _, _, _, _, _, _) => {
                        h3892
                    }
                };
                let a3895 = match v7.clone() {
                    Adt18::C0(_, h3893, _) => {
                        h3893
                    }
                };
                nat_mul(a3894, a3895)?
            }, match v3.clone() {
                Adt3::C0(_, h3896, _, _, _, _, _, _) => {
                    h3896
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3897, _, _, _) => {
                    h3897
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3898, _, _) => {
                    h3898
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3899, _) => {
                    h3899
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3900) => {
                    h3900
                }
            })?;
            match m3901 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3902 = v0;
                    let a3903 = 0u64;
                    if if nat_eq(a3902, a3903) {
                        true
                    } else {
                        let a3904 = v0;
                        let a3905 = 26u64;
                        nat_eq(a3904, a3905)
                    } {
                        Ok(Adt9::C1(false, v9, f136({
                            let a3909 = {
                                let a3907 = v1;
                                let a3908 = match v4.clone() {
                                    Adt3::C0(h3906, _, _, _, _, _, _, _) => {
                                        h3906
                                    }
                                };
                                nat_mul(a3907, a3908)?
                            };
                            let a3910 = v8;
                            nat_add(a3909, a3910)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h3911) => {
                                h3911
                            }
                        }, f136({
                            let a3915 = {
                                let a3913 = v1;
                                let a3914 = match v4.clone() {
                                    Adt3::C0(h3912, _, _, _, _, _, _, _) => {
                                        h3912
                                    }
                                };
                                nat_mul(a3913, a3914)?
                            };
                            let a3916 = v8;
                            nat_add(a3915, a3916)?
                        }, v4.clone())?, {
                            let a3923 = {
                                let a3921 = {
                                    let a3918 = match v7 {
                                        Adt18::C0(_, h3917, _) => {
                                            h3917
                                        }
                                    };
                                    let a3919 = 1u64;
                                    nat_sub(a3918, a3919)
                                };
                                let a3922 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3920, _, _, _) => {
                                        h3920
                                    }
                                };
                                nat_mul(a3921, a3922)?
                            };
                            let a3924 = 4u64;
                            nat_add(a3923, a3924)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3925, _, _, _) => {
                                h3925
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f159(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3926 = v1;
    let a3927 = 0u64;
    if if nat_eq(a3926, a3927) {
        true
    } else {
        let a3928 = v2;
        let a3929 = 0u64;
        nat_eq(a3928, a3929)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f160(v0: u64, v1: u64) -> bool {
    let a3930 = v0;
    let a3931 = 4294967295u64;
    if nat_eq(a3930, a3931) {
        true
    } else {
        let a3932 = v0;
        let a3933 = 2147483648u64;
        if nat_lt(a3932, a3933) {
            let a3934 = v0;
            let a3935 = v1;
            nat_lt(a3934, a3935)
        } else {
            false
        }
    }
}

pub fn f161(v0: Adt7) -> R<u64> {
    let a3940 = if match v0.clone() {
        Adt7::C0(_, _, _, h3936, _) => {
            h3936
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3941 = {
        let a3938 = match v0 {
            Adt7::C0(_, _, _, _, h3937) => {
                h3937
            }
        };
        let a3939 = 1u64;
        if nat_eq(a3938, a3939) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3940, a3941)
}

pub fn f162(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3946 = {
            let a3943 = v2;
            let a3944 = match v0.clone() {
                Adt6::C0(_, h3942, _, _) => {
                    h3942
                }
            };
            nat_mul(a3943, a3944)?
        };
        let a3947 = f129(match v0.clone() {
            Adt6::C0(_, _, h3945, _) => {
                h3945
            }
        });
        nat_add(a3946, a3947)?
    };
    let v4: u64 = f130(match v0.clone() {
        Adt6::C0(_, _, h3948, _) => {
            h3948
        }
    });
    let v5: u64 = {
        let a3952 = {
            let a3950 = v3;
            let a3951 = match v0 {
                Adt6::C0(h3949, _, _, _) => {
                    h3949
                }
            };
            nat_add(a3950, a3951)?
        };
        let a3953 = v4;
        nat_sub(a3952, a3953)
    };
    let a3958 = {
        let a3954 = v5;
        let a3955 = v1;
        if nat_lt(a3954, a3955) {
            v5
        } else {
            v1
        }
    };
    let a3959 = {
        let a3956 = v3;
        let a3957 = v4;
        nat_sub(a3956, a3957)
    };
    Ok(nat_sub(a3958, a3959))
}

pub fn f163(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3968 = f71(match v0.clone() {
        Adt7::C0(h3960, _, _, _, _) => {
            h3960
        }
    }, match v1.clone() {
        Adt3::C0(h3961, _, _, _, _, _, _, _) => {
            h3961
        }
    }, f69(v3, v2.clone(), 0u64), {
        let a3965 = v4;
        let a3966 = {
            let m3964 = match v0.clone() {
                Adt7::C0(h3963, _, _, _, _) => {
                    h3963
                }
            };
            match m3964 {
                Adt6::C0(h3962, _, _, _) => {
                    h3962
                }
            }
        };
        let a3967 = 0u64;
        nat_rem(a3965, a3966, a3967)
    })?;
    match m3968 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3977 = f71(match v0.clone() {
                Adt7::C0(_, h3969, _, _, _) => {
                    h3969
                }
            }, match v1.clone() {
                Adt3::C0(_, h3970, _, _, _, _, _, _) => {
                    h3970
                }
            }, f69(v3, v2.clone(), 1u64), {
                let a3974 = v4;
                let a3975 = {
                    let m3973 = match v0 {
                        Adt7::C0(h3972, _, _, _, _) => {
                            h3972
                        }
                    };
                    match m3973 {
                        Adt6::C0(h3971, _, _, _) => {
                            h3971
                        }
                    }
                };
                let a3976 = 0u64;
                nat_quot(a3974, a3975, a3976)
            })?;
            match m3977 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f68(v1, v5, v6, f69(v3, v2.clone(), 2u64), f69(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f164(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m3978 = v0;
    match m3978 {
        Adt16::C0(v19) => {
            f211(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a3979 = v20;
            let a3980 = 66u64;
            if nat_eq(a3979, a3980) {
                f212(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f213(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3987 = v3;
        let a3988 = {
            let a3984 = f136({
                let a3981 = v0;
                let a3982 = 2u64;
                let a3983 = 0u64;
                nat_quot(a3981, a3982, a3983)
            }, v1)?;
            let a3985 = v2;
            let a3986 = 0u64;
            nat_quot(a3984, a3985, a3986)
        };
        nat_add(a3987, a3988)?
    };
    let a3992 = {
        let a3989 = v0;
        let a3990 = 2u64;
        let a3991 = 0u64;
        nat_rem(a3989, a3990, a3991)
    };
    let a3993 = 0u64;
    if nat_eq(a3992, a3993) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3994 = v4;
        let a3995 = 0u64;
        if nat_eq(a3994, a3995) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f166(v0: u64, v1: u64) -> R<u64> {
    let a3998 = 8u64;
    let a3999 = {
        let a3996 = f214(v0);
        let a3997 = v1;
        nat_mul(a3996, a3997)?
    };
    nat_add(a3998, a3999)
}

pub fn f167(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4000, _, _, _, _, _, _, _) => {
            h4000
        }
    };
    let v7: u64 = f214(v3);
    let v8: u64 = {
        let a4003 = 1u64;
        let a4004 = {
            let a4001 = v6;
            let a4002 = v7;
            nat_mul(a4001, a4002)?
        };
        nat_add(a4003, a4004)?
    };
    let a4005 = v0;
    let a4006 = 0u64;
    if nat_eq(a4005, a4006) {
        Ok(f215(0u64))
    } else {
        let a4007 = v0;
        let a4008 = v8;
        if nat_lt(a4007, a4008) {
            let v9: u64 = {
                let a4009 = v0;
                let a4010 = 1u64;
                nat_sub(a4009, a4010)
            };
            let v10: u64 = {
                let a4011 = v9;
                let a4012 = v7;
                let a4013 = 0u64;
                nat_rem(a4011, a4012, a4013)
            };
            let v11: u64 = f136({
                let a4019 = {
                    let a4014 = v1;
                    let a4015 = v6;
                    nat_mul(a4014, a4015)?
                };
                let a4020 = {
                    let a4016 = v9;
                    let a4017 = v7;
                    let a4018 = 0u64;
                    nat_quot(a4016, a4017, a4018)
                };
                nat_add(a4019, a4020)?
            }, v2)?;
            let a4021 = v3;
            let a4022 = 30u64;
            if nat_eq(a4021, a4022) {
                let a4023 = v10;
                let a4024 = 0u64;
                if nat_eq(a4023, a4024) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4025 = v10;
                    let a4026 = 1u64;
                    if nat_eq(a4025, a4026) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4027 = v10;
                        let a4028 = 2u64;
                        if nat_eq(a4027, a4028) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4029 = v10;
                let a4030 = 0u64;
                if nat_eq(a4029, a4030) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4031 = v10;
                    let a4032 = 1u64;
                    if nat_eq(a4031, a4032) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4033 = v0;
                let a4034 = v8;
                nat_sub(a4033, a4034)
            };
            let a4035 = v12;
            let a4036 = 0u64;
            if nat_eq(a4035, a4036) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4037 = v12;
                let a4038 = 1u64;
                if nat_eq(a4037, a4038) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4039 = v12;
                    let a4040 = 2u64;
                    if nat_eq(a4039, a4040) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4041 = v12;
                        let a4042 = 3u64;
                        if nat_eq(a4041, a4042) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4043 = v12;
                            let a4044 = 4u64;
                            if nat_eq(a4043, a4044) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4045 = v12;
                                let a4046 = 5u64;
                                if nat_eq(a4045, a4046) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4047 = v12;
                                    let a4048 = 6u64;
                                    if nat_eq(a4047, a4048) {
                                        if f216(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4049 = 3u64;
                                                let a4050 = v1;
                                                nat_add(a4049, a4050)?
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

pub fn f168(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4051 = v0;
        let a4052 = f100(v3);
        let a4053 = 0u64;
        nat_quot(a4051, a4052, a4053)
    };
    let v5: u64 = {
        let a4058 = 3u64;
        let a4059 = {
            let a4055 = v4;
            let a4056 = match v1.clone() {
                Adt3::C0(h4054, _, _, _, _, _, _, _) => {
                    h4054
                }
            };
            let a4057 = 0u64;
            nat_quot(a4055, a4056, a4057)
        };
        nat_add(a4058, a4059)?
    };
    let v6: u64 = f136(v4, v1)?;
    let v7: u64 = f136(v4, v2)?;
    let a4060 = v3;
    let a4061 = 30u64;
    if nat_eq(a4060, a4061) {
        let a4065 = {
            let a4062 = v0;
            let a4063 = 3u64;
            let a4064 = 0u64;
            nat_rem(a4062, a4063, a4064)
        };
        let a4066 = 0u64;
        if nat_eq(a4065, a4066) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4070 = {
                let a4067 = v0;
                let a4068 = 3u64;
                let a4069 = 0u64;
                nat_rem(a4067, a4068, a4069)
            };
            let a4071 = 1u64;
            if nat_eq(a4070, a4071) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f169(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4072 = v0;
        let a4073 = 8388608u64;
        let a4074 = 0u64;
        nat_quot(a4072, a4073, a4074)
    };
    let a4075 = v0;
    let a4076 = 2147483648u64;
    if if if nat_lt(a4075, a4076) {
        let a4077 = 127u64;
        let a4078 = v1;
        nat_le(a4077, a4078)
    } else {
        false
    } {
        let a4079 = v1;
        let a4080 = 190u64;
        nat_lt(a4079, a4080)
    } else {
        false
    } {
        let v2: u64 = {
            let a4084 = 8388608u64;
            let a4085 = {
                let a4081 = v0;
                let a4082 = 8388608u64;
                let a4083 = 0u64;
                nat_rem(a4081, a4082, a4083)
            };
            nat_add(a4084, a4085)?
        };
        let a4086 = v1;
        let a4087 = 150u64;
        if nat_lt(a4086, a4087) {
            let v3: u64 = f217({
                let a4088 = 150u64;
                let a4089 = v1;
                nat_sub(a4088, a4089)
            });
            let a4093 = {
                let a4090 = v2;
                let a4091 = v3;
                let a4092 = 0u64;
                nat_rem(a4090, a4091, a4092)
            };
            let a4094 = 0u64;
            if nat_eq(a4093, a4094) {
                let a4095 = v2;
                let a4096 = v3;
                let a4097 = 0u64;
                Ok(nat_quot(a4095, a4096, a4097))
            } else {
                Ok(0u64)
            }
        } else {
            let a4100 = f217({
                let a4098 = v1;
                let a4099 = 150u64;
                nat_sub(a4098, a4099)
            });
            let a4101 = v2;
            nat_mul(a4100, a4101)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f170(v0: Adt14, v1: Adt14) -> bool {
    let a4104 = match v0.clone() {
        Adt14::C0(_, h4102, _, _, _, _, _, _, _, _, _) => {
            h4102
        }
    };
    let a4105 = match v1.clone() {
        Adt14::C0(_, h4103, _, _, _, _, _, _, _, _, _) => {
            h4103
        }
    };
    if if if nat_eq(a4104, a4105) {
        let a4108 = match v0.clone() {
            Adt14::C0(_, _, h4106, _, _, _, _, _, _, _, _) => {
                h4106
            }
        };
        let a4109 = match v1.clone() {
            Adt14::C0(_, _, h4107, _, _, _, _, _, _, _, _) => {
                h4107
            }
        };
        nat_eq(a4108, a4109)
    } else {
        false
    } {
        let a4112 = match v0.clone() {
            Adt14::C0(_, _, _, h4110, _, _, _, _, _, _, _) => {
                h4110
            }
        };
        let a4113 = match v1.clone() {
            Adt14::C0(_, _, _, h4111, _, _, _, _, _, _, _) => {
                h4111
            }
        };
        nat_eq(a4112, a4113)
    } else {
        false
    } {
        let a4116 = match v0 {
            Adt14::C0(_, _, _, _, h4114, _, _, _, _, _, _) => {
                h4114
            }
        };
        let a4117 = match v1 {
            Adt14::C0(_, _, _, _, h4115, _, _, _, _, _, _) => {
                h4115
            }
        };
        nat_eq(a4116, a4117)
    } else {
        false
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4118 = v0;
    let a4119 = 0u64;
    if if if nat_eq(a4118, a4119) {
        true
    } else {
        let a4120 = v1;
        let a4121 = 0u64;
        nat_eq(a4120, a4121)
    } {
        true
    } else {
        let a4122 = v2;
        let a4123 = 0u64;
        nat_eq(a4122, a4123)
    } {
        true
    } else {
        let a4124 = v3;
        let a4125 = 0u64;
        nat_eq(a4124, a4125)
    }
}

pub fn f172(v0: Adt14, v1: Adt14) -> bool {
    if f178(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4126, _, _, _, _, _) => {
            h4126
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4127, _, _, _, _, _) => {
            h4127
        }
    }) {
        let a4129 = match v0 {
            Adt14::C0(_, h4128, _, _, _, _, _, _, _, _, _) => {
                h4128
            }
        };
        let a4130 = 2147483647u64;
        nat_le(a4129, a4130)
    } else {
        true
    }
}

pub fn f173(v0: Adt14) -> bool {
    let a4135 = {
        let a4132 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4131) => {
                h4131
            }
        };
        let a4133 = 4u64;
        let a4134 = 0u64;
        nat_rem(a4132, a4133, a4134)
    };
    let a4136 = 0u64;
    if if nat_eq(a4135, a4136) {
        f218(match v0.clone() {
            Adt14::C0(_, h4137, _, _, _, _, _, _, _, _, _) => {
                h4137
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4138, _, _, _, _, _) => {
                h4138
            }
        })
    } else {
        false
    } {
        f96(match v0.clone() {
            Adt14::C0(_, _, h4139, _, _, _, _, _, _, _, _) => {
                h4139
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4140, _, _, _, _, _, _, _) => {
                h4140
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4141, _, _, _, _, _, _) => {
                h4141
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4142, _, _, _, _) => {
                h4142
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4143, _, _, _) => {
                h4143
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4144, _, _) => {
                h4144
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f174(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4146 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4145, _) => {
            h4145
        }
    };
    let a4147 = 0u64;
    if nat_eq(a4146, a4147) {
        Ok(true)
    } else {
        let a4149 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4148, _) => {
                h4148
            }
        };
        let a4150 = 0u64;
        if nat_eq(a4149, a4150) {
            Ok(true)
        } else {
            let a4152 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4151) => {
                    h4151
                }
            };
            let a4153 = 0u64;
            if nat_eq(a4152, a4153) {
                Ok(true)
            } else {
                let a4155 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4154) => {
                        h4154
                    }
                };
                let a4156 = 0u64;
                if nat_eq(a4155, a4156) {
                    Ok(true)
                } else {
                    let a4162 = {
                        let a4159 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4157) => {
                                h4157
                            }
                        };
                        let a4160 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4158, _) => {
                                h4158
                            }
                        };
                        nat_add(a4159, a4160)?
                    };
                    let a4163 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4161) => {
                            h4161
                        }
                    };
                    if nat_le(a4162, a4163) {
                        Ok(true)
                    } else {
                        let a4169 = {
                            let a4166 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4164) => {
                                    h4164
                                }
                            };
                            let a4167 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4165, _) => {
                                    h4165
                                }
                            };
                            nat_add(a4166, a4167)?
                        };
                        let a4170 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4168) => {
                                h4168
                            }
                        };
                        Ok(nat_le(a4169, a4170))
                    }
                }
            }
        }
    }
}

pub fn f175(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f170(v0.clone(), v1.clone()) {
        let a4173 = match v0.clone() {
            Adt14::C0(h4171, _, _, _, _, _, _, _, _, _, _) => {
                h4171
            }
        };
        let a4174 = match v1.clone() {
            Adt14::C0(h4172, _, _, _, _, _, _, _, _, _, _) => {
                h4172
            }
        };
        nat_eq(a4173, a4174)
    } else {
        false
    } {
        let a4177 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4175) => {
                h4175
            }
        };
        let a4178 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4176) => {
                h4176
            }
        };
        nat_eq(a4177, a4178)
    } else {
        false
    } {
        let a4181 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4179, _, _, _, _, _) => {
                h4179
            }
        };
        let a4182 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4180, _, _, _, _, _) => {
                h4180
            }
        };
        nat_eq(a4181, a4182)
    } else {
        false
    } {
        let a4185 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4183, _, _, _, _) => {
                h4183
            }
        };
        let a4186 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4184, _, _, _, _) => {
                h4184
            }
        };
        nat_eq(a4185, a4186)
    } else {
        false
    } {
        let a4189 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4187, _, _, _) => {
                h4187
            }
        };
        let a4190 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4188, _, _, _) => {
                h4188
            }
        };
        nat_eq(a4189, a4190)
    } else {
        false
    } {
        let a4193 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4191, _, _) => {
                h4191
            }
        };
        let a4194 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4192, _, _) => {
                h4192
            }
        };
        nat_eq(a4193, a4194)
    } else {
        false
    }
}

pub fn f176(v0: Adt14) -> R<bool> {
    if f219(match v0.clone() {
        Adt14::C0(_, h4195, _, _, _, _, _, _, _, _, _) => {
            h4195
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4196, _, _, _, _, _) => {
            h4196
        }
    }) {
        f97(f220(match v0.clone() {
            Adt14::C0(_, h4197, _, _, _, _, _, _, _, _, _) => {
                h4197
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4198, _, _, _, _, _) => {
                h4198
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4199, _, _, _, _, _, _, _, _) => {
                h4199
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4200, _, _, _, _, _, _, _) => {
                h4200
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4201, _, _, _, _, _, _) => {
                h4201
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4202, _, _, _, _) => {
                h4202
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4203, _, _, _) => {
                h4203
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4204, _, _) => {
                h4204
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f177(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4205 = v1;
    let a4206 = 0u64;
    if if nat_eq(a4205, a4206) {
        true
    } else {
        let a4207 = v2;
        let a4208 = 0u64;
        nat_eq(a4207, a4208)
    } {
        Ok(0u64)
    } else {
        let a4211 = f98(v1, v2, v3)?;
        let a4212 = {
            let a4209 = v2;
            let a4210 = f221(v0, v1)?;
            nat_mul(a4209, a4210)?
        };
        nat_add(a4211, a4212)
    }
}

pub fn f178(v0: u64, v1: u64) -> bool {
    let a4213 = v0;
    let a4214 = 4u64;
    if nat_eq(a4213, a4214) {
        let a4215 = v1;
        let a4216 = 4u64;
        nat_eq(a4215, a4216)
    } else {
        false
    }
}

pub fn f179(v0: u64) -> R<u64> {
    let a4217 = 6u64;
    let a4218 = v0;
    nat_add(a4217, a4218)
}

pub fn f180(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4219, _, _, _, _, _, _, _, _, _, _) => {
            h4219
        }
    }, match v0.clone() {
        Adt14::C0(_, h4220, _, _, _, _, _, _, _, _, _) => {
            h4220
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4221, _, _, _, _, _, _, _, _) => {
            h4221
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4222, _, _, _, _, _, _, _) => {
            h4222
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4223, _, _, _, _, _, _) => {
            h4223
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4224, _, _, _, _) => {
            h4224
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4225, _, _, _) => {
            h4225
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4226, _, _) => {
            h4226
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4227, _) => {
            h4227
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4228) => {
            h4228
        }
    })
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4229 = v0;
    let a4230 = 2147483647u64;
    if if if nat_le(a4229, a4230) {
        let a4231 = 0u64;
        let a4232 = v3;
        if nat_lt(a4231, a4232) {
            let a4233 = v3;
            let a4234 = 2147483647u64;
            nat_le(a4233, a4234)
        } else {
            false
        }
    } else {
        false
    } {
        let a4239 = {
            let a4237 = {
                let a4235 = v2;
                let a4236 = v3;
                nat_add(a4235, a4236)?
            };
            let a4238 = 1u64;
            nat_sub(a4237, a4238)
        };
        let a4240 = 2147483647u64;
        nat_le(a4239, a4240)
    } else {
        false
    } {
        let a4245 = {
            let a4243 = {
                let a4241 = v0;
                let a4242 = v1;
                nat_mul(a4241, a4242)?
            };
            let a4244 = f222(v2, v3)?;
            nat_mul(a4243, a4244)?
        };
        let a4246 = 9223372036854775807u64;
        Ok(nat_le(a4245, a4246))
    } else {
        Ok(false)
    }
}

pub fn f182(v0: Adt14) -> R<u64> {
    let a4277 = {
        let a4269 = {
            let a4261 = {
                let a4253 = 4u64;
                let a4254 = {
                    let a4251 = {
                        let a4248 = match v0.clone() {
                            Adt14::C0(_, h4247, _, _, _, _, _, _, _, _, _) => {
                                h4247
                            }
                        };
                        let a4249 = 1u64;
                        nat_sub(a4248, a4249)
                    };
                    let a4252 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4250, _, _, _, _, _) => {
                            h4250
                        }
                    };
                    nat_mul(a4251, a4252)?
                };
                nat_add(a4253, a4254)?
            };
            let a4262 = {
                let a4259 = {
                    let a4256 = match v0.clone() {
                        Adt14::C0(_, _, h4255, _, _, _, _, _, _, _, _) => {
                            h4255
                        }
                    };
                    let a4257 = 1u64;
                    nat_sub(a4256, a4257)
                };
                let a4260 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4258, _, _, _, _) => {
                        h4258
                    }
                };
                nat_mul(a4259, a4260)?
            };
            nat_add(a4261, a4262)?
        };
        let a4270 = {
            let a4267 = {
                let a4264 = match v0.clone() {
                    Adt14::C0(_, _, _, h4263, _, _, _, _, _, _, _) => {
                        h4263
                    }
                };
                let a4265 = 1u64;
                nat_sub(a4264, a4265)
            };
            let a4268 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4266, _, _, _) => {
                    h4266
                }
            };
            nat_mul(a4267, a4268)?
        };
        nat_add(a4269, a4270)?
    };
    let a4278 = {
        let a4275 = {
            let a4272 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4271, _, _, _, _, _, _) => {
                    h4271
                }
            };
            let a4273 = 1u64;
            nat_sub(a4272, a4273)
        };
        let a4276 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4274, _, _) => {
                h4274
            }
        };
        nat_mul(a4275, a4276)?
    };
    nat_add(a4277, a4278)
}

pub fn f183(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4279 = v0;
    let a4280 = 0u64;
    if if if if nat_eq(a4279, a4280) {
        true
    } else {
        let a4281 = v1;
        let a4282 = 0u64;
        nat_eq(a4281, a4282)
    } {
        true
    } else {
        let a4283 = v2;
        let a4284 = 0u64;
        nat_eq(a4283, a4284)
    } {
        true
    } else {
        let a4285 = v3;
        let a4286 = 0u64;
        nat_eq(a4285, a4286)
    } {
        Ok(0u64)
    } else {
        let a4287 = f223(v0, v1, v2, v3, v5)?;
        let a4288 = f224(v0, v1, v2, v3, v4)?;
        nat_add(a4287, a4288)
    }
}

pub fn f184(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4289 = v0;
    let a4290 = 0u64;
    if if nat_eq(a4289, a4290) {
        true
    } else {
        let a4291 = v1;
        let a4292 = 0u64;
        nat_eq(a4291, a4292)
    } {
        Ok(0u64)
    } else {
        let a4295 = f98(v0, v1, v2)?;
        let a4296 = {
            let a4293 = v1;
            let a4294 = f225(v0)?;
            nat_mul(a4293, a4294)?
        };
        nat_add(a4295, a4296)
    }
}

pub fn f185(v0: u64, v1: u64, v2: u64) -> bool {
    let a4297 = v0;
    let a4298 = 1u64;
    if nat_le(a4297, a4298) {
        true
    } else {
        let a4302 = {
            let a4299 = v1;
            let a4300 = v2;
            let a4301 = 1u64;
            nat_rem(a4299, a4300, a4301)
        };
        let a4303 = 0u64;
        nat_eq(a4302, a4303)
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4304 = v1;
    let a4305 = 1u64;
    if if if nat_le(a4304, a4305) {
        true
    } else {
        let a4306 = v0;
        let a4307 = v2;
        nat_le(a4306, a4307)
    } {
        let a4308 = v3;
        let a4309 = 1u64;
        if nat_le(a4308, a4309) {
            true
        } else {
            let a4316 = {
                let a4314 = v0;
                let a4315 = {
                    let a4312 = {
                        let a4310 = v1;
                        let a4311 = 1u64;
                        nat_sub(a4310, a4311)
                    };
                    let a4313 = v2;
                    nat_mul(a4312, a4313)?
                };
                nat_add(a4314, a4315)?
            };
            let a4317 = v4;
            nat_le(a4316, a4317)
        }
    } else {
        false
    } {
        let a4318 = v5;
        let a4319 = 1u64;
        if nat_le(a4318, a4319) {
            Ok(true)
        } else {
            let a4332 = {
                let a4330 = {
                    let a4324 = v0;
                    let a4325 = {
                        let a4322 = {
                            let a4320 = v1;
                            let a4321 = 1u64;
                            nat_sub(a4320, a4321)
                        };
                        let a4323 = v2;
                        nat_mul(a4322, a4323)?
                    };
                    nat_add(a4324, a4325)?
                };
                let a4331 = {
                    let a4328 = {
                        let a4326 = v3;
                        let a4327 = 1u64;
                        nat_sub(a4326, a4327)
                    };
                    let a4329 = v4;
                    nat_mul(a4328, a4329)?
                };
                nat_add(a4330, a4331)?
            };
            let a4333 = v6;
            Ok(nat_le(a4332, a4333))
        }
    } else {
        Ok(false)
    }
}

pub fn f187(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f226(v0, v1, v6, v7, v8) {
        let a4334 = v5;
        let a4335 = 26u64;
        nat_eq(a4334, a4335)
    } else {
        false
    } {
        let a4336 = v3;
        let a4337 = v7;
        nat_eq(a4336, a4337)
    } else {
        false
    } {
        let a4338 = v4;
        let a4339 = v8;
        nat_eq(a4338, a4339)
    } else {
        false
    } {
        let a4340 = v9;
        let a4341 = 1u64;
        nat_eq(a4340, a4341)
    } else {
        false
    } {
        let a4342 = v10;
        let a4343 = f227(v0);
        nat_eq(a4342, a4343)
    } else {
        false
    } {
        let a4344 = v11;
        let a4345 = v1;
        nat_eq(a4344, a4345)
    } else {
        false
    } {
        let a4346 = v12;
        let a4347 = v6;
        nat_eq(a4346, a4347)
    } else {
        false
    } {
        let a4348 = v13;
        let a4349 = v7;
        nat_eq(a4348, a4349)
    } else {
        false
    } {
        let a4350 = v14;
        let a4351 = v8;
        nat_eq(a4350, a4351)
    } else {
        false
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4352 = v0;
    let a4353 = 0u64;
    if if if nat_eq(a4352, a4353) {
        true
    } else {
        let a4354 = v1;
        let a4355 = 0u64;
        nat_eq(a4354, a4355)
    } {
        true
    } else {
        let a4356 = v2;
        let a4357 = 0u64;
        nat_eq(a4356, a4357)
    } {
        true
    } else {
        let a4358 = v3;
        let a4359 = 0u64;
        nat_eq(a4358, a4359)
    }
}

pub fn f189(v0: u64) -> u64 {
    let m4360 = f112(v0);
    match m4360 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4361, _) => {
                    h4361
                }
            }
        }
    }
}

pub fn f190(v0: Adt3, v1: bool) -> R<u64> {
    f228(v0.clone(), {
        let a4363 = match v0.clone() {
            Adt3::C0(h4362, _, _, _, _, _, _, _) => {
                h4362
            }
        };
        let a4364 = 1u64;
        nat_sub(a4363, a4364)
    }, {
        let a4366 = match v0.clone() {
            Adt3::C0(_, h4365, _, _, _, _, _, _) => {
                h4365
            }
        };
        let a4367 = 1u64;
        nat_sub(a4366, a4367)
    }, {
        let a4369 = match v0.clone() {
            Adt3::C0(_, _, h4368, _, _, _, _, _) => {
                h4368
            }
        };
        let a4370 = 1u64;
        nat_sub(a4369, a4370)
    }, {
        let a4372 = match v0 {
            Adt3::C0(_, _, _, h4371, _, _, _, _) => {
                h4371
            }
        };
        let a4373 = 1u64;
        nat_sub(a4372, a4373)
    }, v1)
}

pub fn f191(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4374 = v0;
    let a4375 = 0u64;
    if nat_eq(a4374, a4375) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4376 = v0;
            let a4377 = 1u64;
            nat_sub(a4376, a4377)
        };
        let v5: u64 = {
            let a4378 = v4;
            let a4379 = v1;
            let a4380 = 0u64;
            nat_quot(a4378, a4379, a4380)
        };
        let v6: u64 = {
            let a4388 = {
                let a4381 = v5;
                let a4382 = v2;
                nat_mul(a4381, a4382)?
            };
            let a4389 = {
                let a4386 = {
                    let a4383 = v4;
                    let a4384 = v1;
                    let a4385 = 0u64;
                    nat_rem(a4383, a4384, a4385)
                };
                let a4387 = v3;
                nat_mul(a4386, a4387)?
            };
            nat_add(a4388, a4389)?
        };
        let a4390 = v5;
        let a4391 = 0u64;
        if nat_eq(a4390, a4391) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4400 = {
                    let a4394 = {
                        let a4392 = v5;
                        let a4393 = 1u64;
                        nat_sub(a4392, a4393)
                    };
                    let a4395 = v2;
                    nat_mul(a4394, a4395)?
                };
                let a4401 = {
                    let a4398 = {
                        let a4396 = v1;
                        let a4397 = 1u64;
                        nat_sub(a4396, a4397)
                    };
                    let a4399 = v3;
                    nat_mul(a4398, a4399)?
                };
                nat_add(a4400, a4401)?
            };
            let a4402 = v6;
            let a4403 = v7;
            if nat_lt(a4402, a4403) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f192(v0: Adt3, v1: u64) -> bool {
    let a4404 = v1;
    let a4405 = 0u64;
    if !nat_eq(a4404, a4405) {
        true
    } else {
        let a4407 = match v0 {
            Adt3::C0(_, _, _, _, h4406, _, _, _) => {
                h4406
            }
        };
        let a4408 = 4u64;
        nat_eq(a4407, a4408)
    }
}

pub fn f193(v0: u64, v1: bool) -> R<u64> {
    let a4411 = {
        let a4409 = v0;
        let a4410 = 2u64;
        nat_mul(a4409, a4410)?
    };
    let a4412 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4411, a4412)
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4413 = f229(v0, v1);
    match m4413 {
        None => {
            if v5 {
                let m4414 = f230(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4414 {
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
                let a4415 = 4u64;
                let a4416 = v28;
                nat_le(a4415, a4416)
            };
            let a4417 = v2;
            let a4418 = 0u64;
            if if if if if nat_eq(a4417, a4418) {
                let a4419 = v4;
                let a4420 = 0u64;
                nat_eq(a4419, a4420)
            } else {
                false
            } {
                let a4421 = v6;
                let a4422 = v14;
                if nat_eq(a4421, a4422) {
                    let a4423 = v7;
                    let a4424 = v15;
                    nat_eq(a4423, a4424)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4425 = v8;
                let a4426 = v16;
                if nat_eq(a4425, a4426) {
                    let a4427 = v9;
                    let a4428 = v17;
                    nat_eq(a4427, a4428)
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
                        let a4429 = v3;
                        let a4430 = 0u64;
                        nat_eq(a4429, a4430)
                    } else {
                        false
                    } {
                        f115(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4435 = {
                        let a4431 = v6;
                        let a4432 = v7;
                        nat_mul(a4431, a4432)?
                    };
                    let a4436 = {
                        let a4433 = v8;
                        let a4434 = v9;
                        nat_mul(a4433, a4434)?
                    };
                    nat_mul(a4435, a4436)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f195(v0: u64) -> u64 {
    let m4437 = f112(v0);
    match m4437 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4439 = match v1.clone() {
                Adt18::C0(_, h4438, _) => {
                    h4438
                }
            };
            let a4440 = 1u64;
            if if nat_eq(a4439, a4440) {
                let a4442 = match v1.clone() {
                    Adt18::C0(_, _, h4441) => {
                        h4441
                    }
                };
                let a4443 = 1u64;
                if if if nat_eq(a4442, a4443) {
                    true
                } else {
                    let a4445 = match v1.clone() {
                        Adt18::C0(_, _, h4444) => {
                            h4444
                        }
                    };
                    let a4446 = 2u64;
                    nat_eq(a4445, a4446)
                } {
                    true
                } else {
                    let a4448 = match v1.clone() {
                        Adt18::C0(_, _, h4447) => {
                            h4447
                        }
                    };
                    let a4449 = 4u64;
                    nat_eq(a4448, a4449)
                } {
                    true
                } else {
                    let a4451 = match v1.clone() {
                        Adt18::C0(_, _, h4450) => {
                            h4450
                        }
                    };
                    let a4452 = 8u64;
                    nat_eq(a4451, a4452)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4453) => {
                        h4453
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f196(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4458 = {
        let a4456 = {
            let a4454 = v0;
            let a4455 = v1;
            nat_sub(a4454, a4455)
        };
        let a4457 = 1u64;
        nat_sub(a4456, a4457)
    };
    let a4459 = v2;
    nat_add(a4458, a4459)
}

pub fn f197(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f128({
        let a4460 = v0;
        let a4461 = v4;
        let a4462 = 0u64;
        nat_rem(a4460, a4461, a4462)
    }, {
        let a4463 = v1;
        let a4464 = v5;
        let a4465 = 0u64;
        nat_rem(a4463, a4464, a4465)
    }, {
        let a4466 = v2;
        let a4467 = v6;
        let a4468 = 0u64;
        nat_rem(a4466, a4467, a4468)
    }, {
        let a4469 = v3;
        let a4470 = v7;
        let a4471 = 0u64;
        nat_rem(a4469, a4470, a4471)
    }, v8, v9, v10, v11)
}

pub fn f198(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4472 = v1;
    let a4473 = 2147483648u64;
    if nat_lt(a4472, a4473) {
        let a4484 = {
            let a4482 = {
                let a4477 = {
                    let a4474 = v0;
                    let a4475 = v2;
                    let a4476 = 0u64;
                    nat_rem(a4474, a4475, a4476)
                };
                let a4478 = v2;
                nat_add(a4477, a4478)?
            };
            let a4483 = {
                let a4479 = v1;
                let a4480 = v2;
                let a4481 = 0u64;
                nat_rem(a4479, a4480, a4481)
            };
            nat_sub(a4482, a4483)
        };
        let a4485 = v2;
        let a4486 = 0u64;
        Ok(nat_rem(a4484, a4485, a4486))
    } else {
        let a4489 = {
            let a4487 = v0;
            let a4488 = f129(v1);
            nat_add(a4487, a4488)?
        };
        let a4490 = v2;
        let a4491 = 0u64;
        Ok(nat_rem(a4489, a4490, a4491))
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4494 = f130(v1);
    let a4495 = {
        let a4492 = v0;
        let a4493 = f129(v1);
        nat_add(a4492, a4493)?
    };
    if nat_le(a4494, a4495) {
        let a4496 = f200(v0, v1)?;
        let a4497 = v2;
        Ok(nat_lt(a4496, a4497))
    } else {
        Ok(false)
    }
}

pub fn f200(v0: u64, v1: u64) -> R<u64> {
    let a4500 = {
        let a4498 = v0;
        let a4499 = f129(v1);
        nat_add(a4498, a4499)?
    };
    let a4501 = f130(v1);
    Ok(nat_sub(a4500, a4501))
}

pub fn f201(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4502 = v0;
    let a4503 = v1;
    if nat_lt(a4502, a4503) {
        let a4504 = v1;
        let a4505 = v0;
        Ok(nat_sub(a4504, a4505))
    } else {
        let a4508 = v0;
        let a4509 = {
            let a4506 = v1;
            let a4507 = v2;
            nat_add(a4506, a4507)?
        };
        if nat_lt(a4508, a4509) {
            let a4510 = v0;
            let a4511 = v1;
            Ok(nat_sub(a4510, a4511))
        } else {
            let a4518 = {
                let a4514 = 2u64;
                let a4515 = {
                    let a4512 = v2;
                    let a4513 = 1u64;
                    nat_sub(a4512, a4513)
                };
                nat_mul(a4514, a4515)?
            };
            let a4519 = {
                let a4516 = v0;
                let a4517 = v1;
                nat_sub(a4516, a4517)
            };
            Ok(nat_sub(a4518, a4519))
        }
    }
}

pub fn f202(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4522 = {
        let a4520 = v2;
        let a4521 = f228(v1.clone(), f69(v0, v1.clone(), 0u64), f69(v0, v1.clone(), 1u64), f69(v0, v1.clone(), 2u64), f69(v0, v1.clone(), 3u64), false)?;
        nat_add(a4520, a4521)?
    };
    let a4523 = f228(v1.clone(), f69(v0, v1.clone(), 0u64), f69(v0, v1.clone(), 1u64), f69(v0, v1.clone(), 2u64), f69(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4522, a4523))
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> bool {
    let a4524 = v0;
    let a4525 = 0u64;
    if nat_eq(a4524, a4525) {
        let a4526 = v2;
        let a4527 = v1;
        nat_le(a4526, a4527)
    } else {
        let a4528 = v0;
        let a4529 = 1u64;
        if nat_eq(a4528, a4529) {
            let a4530 = v2;
            let a4531 = v1;
            nat_lt(a4530, a4531)
        } else {
            let a4532 = v0;
            let a4533 = 2u64;
            if nat_eq(a4532, a4533) {
                let a4534 = v1;
                let a4535 = v2;
                nat_le(a4534, a4535)
            } else {
                let a4536 = v0;
                let a4537 = 3u64;
                if nat_eq(a4536, a4537) {
                    let a4538 = v1;
                    let a4539 = v2;
                    nat_lt(a4538, a4539)
                } else {
                    let a4540 = v1;
                    let a4541 = v2;
                    nat_eq(a4540, a4541)
                }
            }
        }
    }
}

pub fn f204(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4542 = v0;
    let a4543 = 0u64;
    if nat_eq(a4542, a4543) {
        Ok(f215(0u64))
    } else {
        let a4546 = {
            let a4544 = v0;
            let a4545 = 1u64;
            nat_add(a4544, a4545)?
        };
        let a4547 = f126(v1.clone(), 13u64, v2)?;
        if nat_eq(a4546, a4547) {
            Ok(Adt9::C4({
                let a4548 = v2;
                let a4549 = 0u64;
                if nat_eq(a4548, a4549) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4550 = v0;
                let a4551 = 1u64;
                nat_sub(a4550, a4551)
            };
            if f192(v1.clone(), v2) {
                let v4: u64 = f193(match v1.clone() {
                    Adt3::C0(h4552, _, _, _, _, _, _, _) => {
                        h4552
                    }
                }, false)?;
                let v5: u64 = {
                    let a4553 = v3;
                    let a4554 = v4;
                    let a4555 = 0u64;
                    nat_quot(a4553, a4554, a4555)
                };
                let v6: u64 = {
                    let a4556 = v3;
                    let a4557 = v4;
                    let a4558 = 0u64;
                    nat_rem(a4556, a4557, a4558)
                };
                let a4561 = {
                    let a4559 = v6;
                    let a4560 = 1u64;
                    nat_add(a4559, a4560)?
                };
                let a4562 = v4;
                if nat_eq(a4561, a4562) {
                    Ok(Adt9::C4({
                        let a4563 = v2;
                        let a4564 = 0u64;
                        if nat_eq(a4563, a4564) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f205(v6, v5, v1, v2)
                }
            } else {
                f231({
                    let a4565 = v3;
                    let a4566 = 2u64;
                    let a4567 = 0u64;
                    nat_quot(a4565, a4566, a4567)
                }, {
                    let a4568 = v3;
                    let a4569 = 2u64;
                    let a4570 = 0u64;
                    nat_rem(a4568, a4569, a4570)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f205(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4571 = v0;
    let a4572 = 0u64;
    if nat_eq(a4571, a4572) {
        Ok(f215(1u64))
    } else {
        f231({
            let a4581 = {
                let a4574 = v1;
                let a4575 = match v2.clone() {
                    Adt3::C0(h4573, _, _, _, _, _, _, _) => {
                        h4573
                    }
                };
                nat_mul(a4574, a4575)?
            };
            let a4582 = {
                let a4578 = {
                    let a4576 = v0;
                    let a4577 = 1u64;
                    nat_sub(a4576, a4577)
                };
                let a4579 = 2u64;
                let a4580 = 0u64;
                nat_quot(a4578, a4579, a4580)
            };
            nat_add(a4581, a4582)?
        }, {
            let a4585 = {
                let a4583 = v0;
                let a4584 = 1u64;
                nat_sub(a4583, a4584)
            };
            let a4586 = 2u64;
            let a4587 = 0u64;
            nat_rem(a4585, a4586, a4587)
        }, v2, v3, 1u64)
    }
}

pub fn f206(v0: u64, v1: u64) -> R<u64> {
    let a4588 = v0;
    let a4589 = f92(v1)?;
    let a4590 = 0u64;
    Ok(nat_rem(a4588, a4589, a4590))
}

pub fn f207(v0: u64, v1: u64) -> R<u64> {
    let a4591 = 4u64;
    let a4592 = f232(v0, v1)?;
    nat_mul(a4591, a4592)
}

pub fn f208(v0: u64, v1: u64) -> R<u64> {
    let a4593 = 4u64;
    let a4594 = f233(v0, v1)?;
    nat_mul(a4593, a4594)
}

pub fn f209(v0: u64, v1: u64) -> R<bool> {
    let a4600 = {
        let a4597 = {
            let a4595 = f206(v0, v1)?;
            let a4596 = 2u64;
            nat_sub(a4595, a4596)
        };
        let a4598 = 2u64;
        let a4599 = 0u64;
        nat_rem(a4597, a4598, a4599)
    };
    let a4601 = 0u64;
    Ok(nat_eq(a4600, a4601))
}

pub fn f210(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4602 = f112(v0);
    match m4602 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f110(v0) {
                let a4603 = v1;
                let a4604 = v7;
                nat_lt(a4603, a4604)
            } else {
                false
            } {
                f157(v8, v2)
            } else {
                false
            } {
                let a4605 = v3;
                let a4606 = f234(v4, v5, v6)?;
                nat_lt(a4605, a4606)
            } else {
                false
            } {
                let a4611 = {
                    let a4608 = v7;
                    let a4609 = match v13.clone() {
                        Adt18::C0(_, h4607, _) => {
                            h4607
                        }
                    };
                    let a4610 = 0u64;
                    nat_rem(a4608, a4609, a4610)
                };
                let a4612 = 0u64;
                nat_eq(a4611, a4612)
            } else {
                false
            } {
                Ok(Some({
                    let a4641 = {
                        let a4631 = {
                            let a4621 = {
                                let a4617 = {
                                    let a4614 = v1;
                                    let a4615 = match v13 {
                                        Adt18::C0(_, h4613, _) => {
                                            h4613
                                        }
                                    };
                                    let a4616 = 0u64;
                                    nat_quot(a4614, a4615, a4616)
                                };
                                let a4618 = v9;
                                nat_mul(a4617, a4618)?
                            };
                            let a4622 = {
                                let a4619 = v2;
                                let a4620 = v10;
                                nat_mul(a4619, a4620)?
                            };
                            nat_add(a4621, a4622)?
                        };
                        let a4632 = {
                            let a4629 = {
                                let a4626 = {
                                    let a4623 = v3;
                                    let a4624 = v4;
                                    let a4625 = 0u64;
                                    nat_quot(a4623, a4624, a4625)
                                };
                                let a4627 = v5;
                                let a4628 = 0u64;
                                nat_rem(a4626, a4627, a4628)
                            };
                            let a4630 = v11;
                            nat_mul(a4629, a4630)?
                        };
                        nat_add(a4631, a4632)?
                    };
                    let a4642 = {
                        let a4639 = {
                            let a4636 = {
                                let a4633 = v3;
                                let a4634 = v4;
                                let a4635 = 0u64;
                                nat_quot(a4633, a4634, a4635)
                            };
                            let a4637 = v5;
                            let a4638 = 0u64;
                            nat_quot(a4636, a4637, a4638)
                        };
                        let a4640 = v12;
                        nat_mul(a4639, a4640)?
                    };
                    nat_add(a4641, a4642)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f211(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f235(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f236(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f213(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4643 = v0;
    let a4644 = 64u64;
    if nat_eq(a4643, a4644) {
        f237(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4645 = v0;
        let a4646 = 65u64;
        if nat_eq(a4645, a4646) {
            f238(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f214(v0: u64) -> u64 {
    let a4647 = v0;
    let a4648 = 30u64;
    if nat_eq(a4647, a4648) {
        4u64
    } else {
        3u64
    }
}

pub fn f215(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f216(v0: u64) -> bool {
    let a4649 = 0u64;
    let a4650 = v0;
    if nat_lt(a4649, a4650) {
        let a4651 = v0;
        let a4652 = 2139095040u64;
        nat_le(a4651, a4652)
    } else {
        false
    }
}

pub fn f217(v0: u64) -> u64 {
    let a4653 = v0;
    let a4654 = 40u64;
    if nat_lt(a4653, a4654) {
        let a4655 = v0;
        let a4656 = 20u64;
        if nat_lt(a4655, a4656) {
            let a4657 = v0;
            let a4658 = 10u64;
            if nat_lt(a4657, a4658) {
                let a4659 = v0;
                let a4660 = 5u64;
                if nat_lt(a4659, a4660) {
                    let a4661 = v0;
                    let a4662 = 2u64;
                    if nat_lt(a4661, a4662) {
                        let a4663 = v0;
                        let a4664 = 1u64;
                        if nat_lt(a4663, a4664) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4665 = v0;
                        let a4666 = 3u64;
                        if nat_lt(a4665, a4666) {
                            4u64
                        } else {
                            let a4667 = v0;
                            let a4668 = 4u64;
                            if nat_lt(a4667, a4668) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4669 = v0;
                    let a4670 = 7u64;
                    if nat_lt(a4669, a4670) {
                        let a4671 = v0;
                        let a4672 = 6u64;
                        if nat_lt(a4671, a4672) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4673 = v0;
                        let a4674 = 8u64;
                        if nat_lt(a4673, a4674) {
                            128u64
                        } else {
                            let a4675 = v0;
                            let a4676 = 9u64;
                            if nat_lt(a4675, a4676) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4677 = v0;
                let a4678 = 15u64;
                if nat_lt(a4677, a4678) {
                    let a4679 = v0;
                    let a4680 = 12u64;
                    if nat_lt(a4679, a4680) {
                        let a4681 = v0;
                        let a4682 = 11u64;
                        if nat_lt(a4681, a4682) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4683 = v0;
                        let a4684 = 13u64;
                        if nat_lt(a4683, a4684) {
                            4096u64
                        } else {
                            let a4685 = v0;
                            let a4686 = 14u64;
                            if nat_lt(a4685, a4686) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4687 = v0;
                    let a4688 = 17u64;
                    if nat_lt(a4687, a4688) {
                        let a4689 = v0;
                        let a4690 = 16u64;
                        if nat_lt(a4689, a4690) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4691 = v0;
                        let a4692 = 18u64;
                        if nat_lt(a4691, a4692) {
                            131072u64
                        } else {
                            let a4693 = v0;
                            let a4694 = 19u64;
                            if nat_lt(a4693, a4694) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4695 = v0;
            let a4696 = 30u64;
            if nat_lt(a4695, a4696) {
                let a4697 = v0;
                let a4698 = 25u64;
                if nat_lt(a4697, a4698) {
                    let a4699 = v0;
                    let a4700 = 22u64;
                    if nat_lt(a4699, a4700) {
                        let a4701 = v0;
                        let a4702 = 21u64;
                        if nat_lt(a4701, a4702) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4703 = v0;
                        let a4704 = 23u64;
                        if nat_lt(a4703, a4704) {
                            4194304u64
                        } else {
                            let a4705 = v0;
                            let a4706 = 24u64;
                            if nat_lt(a4705, a4706) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4707 = v0;
                    let a4708 = 27u64;
                    if nat_lt(a4707, a4708) {
                        let a4709 = v0;
                        let a4710 = 26u64;
                        if nat_lt(a4709, a4710) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4711 = v0;
                        let a4712 = 28u64;
                        if nat_lt(a4711, a4712) {
                            134217728u64
                        } else {
                            let a4713 = v0;
                            let a4714 = 29u64;
                            if nat_lt(a4713, a4714) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4715 = v0;
                let a4716 = 35u64;
                if nat_lt(a4715, a4716) {
                    let a4717 = v0;
                    let a4718 = 32u64;
                    if nat_lt(a4717, a4718) {
                        let a4719 = v0;
                        let a4720 = 31u64;
                        if nat_lt(a4719, a4720) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4721 = v0;
                        let a4722 = 33u64;
                        if nat_lt(a4721, a4722) {
                            4294967296u64
                        } else {
                            let a4723 = v0;
                            let a4724 = 34u64;
                            if nat_lt(a4723, a4724) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4725 = v0;
                    let a4726 = 37u64;
                    if nat_lt(a4725, a4726) {
                        let a4727 = v0;
                        let a4728 = 36u64;
                        if nat_lt(a4727, a4728) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4729 = v0;
                        let a4730 = 38u64;
                        if nat_lt(a4729, a4730) {
                            137438953472u64
                        } else {
                            let a4731 = v0;
                            let a4732 = 39u64;
                            if nat_lt(a4731, a4732) {
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

pub fn f218(v0: u64, v1: u64) -> bool {
    let a4733 = v0;
    let a4734 = 1u64;
    if nat_le(a4733, a4734) {
        true
    } else {
        let a4738 = {
            let a4735 = v1;
            let a4736 = 4u64;
            let a4737 = 0u64;
            nat_rem(a4735, a4736, a4737)
        };
        let a4739 = 0u64;
        nat_eq(a4738, a4739)
    }
}

pub fn f219(v0: u64, v1: u64) -> bool {
    let a4740 = v0;
    let a4741 = 1u64;
    if nat_le(a4740, a4741) {
        true
    } else {
        let a4742 = 4u64;
        let a4743 = v1;
        nat_le(a4742, a4743)
    }
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let a4744 = v0;
    let a4745 = 0u64;
    if nat_eq(a4744, a4745) {
        Ok(0u64)
    } else {
        let a4750 = 4u64;
        let a4751 = {
            let a4748 = {
                let a4746 = v0;
                let a4747 = 1u64;
                nat_sub(a4746, a4747)
            };
            let a4749 = v1;
            nat_mul(a4748, a4749)?
        };
        nat_add(a4750, a4751)
    }
}

pub fn f221(v0: bool, v1: u64) -> R<u64> {
    let a4756 = {
        let a4754 = {
            let a4752 = f239(v0, v1)?;
            let a4753 = f240(v0, v1)?;
            nat_add(a4752, a4753)?
        };
        let a4755 = f241(v0);
        nat_add(a4754, a4755)?
    };
    let a4757 = v1;
    nat_add(a4756, a4757)
}

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f242(v0, v1)?;
    let v3: u64 = {
        let a4762 = {
            let a4760 = {
                let a4758 = v1;
                let a4759 = 1u64;
                nat_sub(a4758, a4759)
            };
            let a4761 = v2;
            nat_mul(a4760, a4761)?
        };
        let a4763 = v0;
        nat_sub(a4762, a4763)
    };
    let a4764 = v2;
    let a4765 = v3;
    if nat_lt(a4764, a4765) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f223(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4766 = v4;
    let a4767 = 0u64;
    if nat_eq(a4766, a4767) {
        Ok(0u64)
    } else {
        let a4774 = 2u64;
        let a4775 = {
            let a4772 = {
                let a4768 = v0;
                let a4769 = v1;
                nat_mul(a4768, a4769)?
            };
            let a4773 = {
                let a4770 = v2;
                let a4771 = v3;
                nat_mul(a4770, a4771)?
            };
            nat_mul(a4772, a4773)?
        };
        nat_mul(a4774, a4775)
    }
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4780 = v3;
    let a4781 = {
        let a4778 = {
            let a4776 = f243(v2, v4)?;
            let a4777 = f244(v0, v1, f242(v2, v4)?)?;
            nat_mul(a4776, a4777)?
        };
        let a4779 = f244(v0, v1, f245(v2, v4)?)?;
        nat_add(a4778, a4779)?
    };
    nat_mul(a4780, a4781)
}

pub fn f225(v0: u64) -> R<u64> {
    let a4784 = 13u64;
    let a4785 = {
        let a4782 = 9u64;
        let a4783 = v0;
        nat_mul(a4782, a4783)?
    };
    nat_add(a4784, a4785)
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f188(v1, v2, v3, v4) {
        f246(v0)
    } else {
        f110(v0)
    }
}

pub fn f227(v0: u64) -> u64 {
    let a4786 = v0;
    let a4787 = 26u64;
    if nat_eq(a4786, a4787) {
        26u64
    } else {
        0u64
    }
}

pub fn f228(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4808 = {
        let a4802 = {
            let a4796 = {
                let a4790 = v1;
                let a4791 = if v5 {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4788, _, _, _) => {
                            h4788
                        }
                    })
                } else {
                    f130(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4789, _, _, _) => {
                            h4789
                        }
                    })
                };
                nat_mul(a4790, a4791)?
            };
            let a4797 = {
                let a4794 = v2;
                let a4795 = if v5 {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4792, _, _) => {
                            h4792
                        }
                    })
                } else {
                    f130(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4793, _, _) => {
                            h4793
                        }
                    })
                };
                nat_mul(a4794, a4795)?
            };
            nat_add(a4796, a4797)?
        };
        let a4803 = {
            let a4800 = v3;
            let a4801 = if v5 {
                f129(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4798, _) => {
                        h4798
                    }
                })
            } else {
                f130(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4799, _) => {
                        h4799
                    }
                })
            };
            nat_mul(a4800, a4801)?
        };
        nat_add(a4802, a4803)?
    };
    let a4809 = {
        let a4806 = v4;
        let a4807 = if v5 {
            f129(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4804) => {
                    h4804
                }
            })
        } else {
            f130(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4805) => {
                    h4805
                }
            })
        };
        nat_mul(a4806, a4807)?
    };
    nat_add(a4808, a4809)
}

pub fn f229(v0: u64, v1: u64) -> Option<u64> {
    let a4810 = v0;
    let a4811 = 2u64;
    if nat_eq(a4810, a4811) {
        Some(0u64)
    } else {
        let a4812 = v0;
        let a4813 = 5u64;
        if nat_eq(a4812, a4813) {
            Some(1u64)
        } else {
            let a4814 = v0;
            let a4815 = 6u64;
            if nat_eq(a4814, a4815) {
                Some(2u64)
            } else {
                let a4816 = v0;
                let a4817 = 7u64;
                if nat_eq(a4816, a4817) {
                    Some(3u64)
                } else {
                    let a4818 = v0;
                    let a4819 = 9u64;
                    if nat_eq(a4818, a4819) {
                        Some(4u64)
                    } else {
                        let a4820 = v0;
                        let a4821 = 8u64;
                        if nat_eq(a4820, a4821) {
                            Some(5u64)
                        } else {
                            let a4822 = v0;
                            let a4823 = 89u64;
                            if nat_eq(a4822, a4823) {
                                let a4824 = v1;
                                let a4825 = 2u64;
                                if nat_eq(a4824, a4825) {
                                    Some(6u64)
                                } else {
                                    let a4826 = v1;
                                    let a4827 = 0u64;
                                    if nat_eq(a4826, a4827) {
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

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4828 = v0;
    let a4829 = 30u64;
    if nat_eq(a4828, a4829) {
        let a4830 = v2;
        let a4831 = 0u64;
        if if if nat_eq(a4830, a4831) {
            true
        } else {
            let a4832 = v2;
            let a4833 = 1u64;
            nat_eq(a4832, a4833)
        } {
            let a4834 = v1;
            let a4835 = 0u64;
            if nat_eq(a4834, a4835) {
                true
            } else {
                f247(v1, v4)
            }
        } else {
            false
        } {
            f248(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f249(v1) {
            f249(v2)
        } else {
            false
        } {
            let a4836 = v1;
            let a4837 = 0u64;
            if if nat_eq(a4836, a4837) {
                true
            } else {
                let a4838 = v2;
                let a4839 = 0u64;
                nat_eq(a4838, a4839)
            } {
                true
            } else {
                let a4840 = v1;
                let a4841 = v2;
                nat_eq(a4840, a4841)
            }
        } else {
            false
        } {
            true
        } else {
            let a4842 = v1;
            let a4843 = 43u64;
            if nat_eq(a4842, a4843) {
                f249(v2)
            } else {
                false
            }
        } {
            f248(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f231(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4844 = v1;
    let a4845 = 0u64;
    if nat_eq(a4844, a4845) {
        Ok(Adt9::C4({
            let a4846 = v3;
            let a4847 = 0u64;
            if nat_eq(a4846, a4847) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f136(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4848 = v3;
            let a4849 = 0u64;
            if nat_eq(a4848, a4849) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f232(v0: u64, v1: u64) -> R<u64> {
    let a4850 = v0;
    let a4851 = f92(v1)?;
    let a4852 = 0u64;
    Ok(nat_quot(a4850, a4851, a4852))
}

pub fn f233(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f232(v0, v1)?;
    let v3: u64 = {
        let a4855 = {
            let a4853 = f206(v0, v1)?;
            let a4854 = 2u64;
            nat_sub(a4853, a4854)
        };
        let a4856 = 2u64;
        let a4857 = 0u64;
        nat_quot(a4855, a4856, a4857)
    };
    let v4: u64 = {
        let a4858 = v2;
        let a4859 = v1;
        nat_mul(a4858, a4859)?
    };
    if f209(v0, v1)? {
        let a4860 = v4;
        let a4861 = v3;
        nat_add(a4860, a4861)
    } else {
        let a4873 = {
            let a4869 = v4;
            let a4870 = {
                let a4867 = {
                    let a4862 = v1;
                    let a4863 = 1u64;
                    nat_sub(a4862, a4863)
                };
                let a4868 = {
                    let a4864 = v2;
                    let a4865 = v1;
                    let a4866 = 0u64;
                    nat_rem(a4864, a4865, a4866)
                };
                nat_mul(a4867, a4868)?
            };
            nat_sub(a4869, a4870)
        };
        let a4874 = {
            let a4871 = v3;
            let a4872 = v1;
            nat_mul(a4871, a4872)?
        };
        nat_add(a4873, a4874)
    }
}

pub fn f234(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4875 = v0;
    let a4876 = 0u64;
    if if if nat_eq(a4875, a4876) {
        true
    } else {
        let a4877 = v1;
        let a4878 = 0u64;
        nat_eq(a4877, a4878)
    } {
        true
    } else {
        let a4879 = v2;
        let a4880 = 0u64;
        nat_eq(a4879, a4880)
    } {
        Ok(0u64)
    } else {
        let a4883 = {
            let a4881 = v0;
            let a4882 = v1;
            nat_mul(a4881, a4882)?
        };
        let a4884 = v2;
        nat_mul(a4883, a4884)
    }
}

pub fn f235(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f101(match v1.clone() {
        Adt20::C0(h4885, _, _, _, _, _, _, _) => {
            h4885
        }
    })?;
    let v6: u64 = {
        let a4891 = match v1.clone() {
            Adt20::C0(_, h4886, _, _, _, _, _, _) => {
                h4886
            }
        };
        let a4892 = {
            let a4889 = match v1.clone() {
                Adt20::C0(_, _, h4887, _, _, _, _, _) => {
                    h4887
                }
            };
            let a4890 = match v1.clone() {
                Adt20::C0(_, _, _, h4888, _, _, _, _) => {
                    h4888
                }
            };
            nat_mul(a4889, a4890)?
        };
        nat_mul(a4891, a4892)?
    };
    let a4894 = v0;
    let a4895 = f38(match v1.clone() {
        Adt20::C0(h4893, _, _, _, _, _, _, _) => {
            h4893
        }
    }, v6)?;
    if nat_lt(a4894, a4895) {
        f250({
            let a4896 = v0;
            let a4897 = v5;
            let a4898 = 0u64;
            nat_rem(a4896, a4897, a4898)
        }, {
            let a4899 = v0;
            let a4900 = v5;
            let a4901 = 0u64;
            nat_quot(a4899, a4900, a4901)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f236(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a4902 = v6;
        let a4903 = 6u64;
        nat_sub(a4902, a4903)
    };
    let v9: Adt20 = f251(v3);
    let a4905 = v0;
    let a4906 = f184(match v1.clone() {
        Adt20::C0(h4904, _, _, _, _, _, _, _) => {
            h4904
        }
    }, v5, v8)?;
    if nat_lt(a4905, a4906) {
        let v10: u64 = f98(match v1.clone() {
            Adt20::C0(h4907, _, _, _, _, _, _, _) => {
                h4907
            }
        }, v5, v8)?;
        let a4908 = v0;
        let a4909 = v10;
        if nat_lt(a4908, a4909) {
            f252(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a4910 = v0;
                let a4911 = v10;
                nat_sub(a4910, a4911)
            };
            let v12: u64 = f225(match v1.clone() {
                Adt20::C0(h4912, _, _, _, _, _, _, _) => {
                    h4912
                }
            })?;
            f253({
                let a4913 = v11;
                let a4914 = v12;
                let a4915 = 0u64;
                nat_rem(a4913, a4914, a4915)
            }, {
                let a4916 = v11;
                let a4917 = v12;
                let a4918 = 0u64;
                nat_quot(a4916, a4917, a4918)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f237(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4919 = v5;
        let a4920 = 6u64;
        nat_sub(a4919, a4920)
    };
    let v8: bool = f178(match v1.clone() {
        Adt20::C0(_, _, _, _, h4921, _, _, _) => {
            h4921
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h4922, _, _, _) => {
            h4922
        }
    });
    let a4924 = v0;
    let a4925 = f177(v8, match v1.clone() {
        Adt20::C0(h4923, _, _, _, _, _, _, _) => {
            h4923
        }
    }, v4, v7)?;
    if nat_lt(a4924, a4925) {
        let v9: u64 = f98(match v1.clone() {
            Adt20::C0(h4926, _, _, _, _, _, _, _) => {
                h4926
            }
        }, v4, v7)?;
        let a4927 = v0;
        let a4928 = v9;
        if nat_lt(a4927, a4928) {
            f252(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a4929 = v0;
                let a4930 = v9;
                nat_sub(a4929, a4930)
            };
            let v11: u64 = f221(v8, match v1.clone() {
                Adt20::C0(h4931, _, _, _, _, _, _, _) => {
                    h4931
                }
            })?;
            f254({
                let a4932 = v10;
                let a4933 = v11;
                let a4934 = 0u64;
                nat_rem(a4932, a4933, a4934)
            }, {
                let a4935 = v10;
                let a4936 = v11;
                let a4937 = 0u64;
                nat_quot(a4935, a4936, a4937)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f238(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4938 = v5;
        let a4939 = 6u64;
        nat_sub(a4938, a4939)
    };
    let v8: Adt20 = f251(v2);
    let v9: u64 = f223(match v1.clone() {
        Adt20::C0(h4940, _, _, _, _, _, _, _) => {
            h4940
        }
    }, match v1.clone() {
        Adt20::C0(_, h4941, _, _, _, _, _, _) => {
            h4941
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4942, _, _, _, _, _) => {
            h4942
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4943, _, _, _, _) => {
            h4943
        }
    }, v7)?;
    let a4948 = v0;
    let a4949 = f183(match v1.clone() {
        Adt20::C0(h4944, _, _, _, _, _, _, _) => {
            h4944
        }
    }, match v1.clone() {
        Adt20::C0(_, h4945, _, _, _, _, _, _) => {
            h4945
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4946, _, _, _, _, _) => {
            h4946
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4947, _, _, _, _) => {
            h4947
        }
    }, v4, v7)?;
    if nat_lt(a4948, a4949) {
        let a4950 = v0;
        let a4951 = v9;
        if nat_lt(a4950, a4951) {
            f252(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f255({
                let a4952 = v0;
                let a4953 = v9;
                nat_sub(a4952, a4953)
            }, match v1.clone() {
                Adt20::C0(h4954, _, _, _, _, _, _, _) => {
                    h4954
                }
            }, match v1.clone() {
                Adt20::C0(_, h4955, _, _, _, _, _, _) => {
                    h4955
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h4956, _, _, _, _, _) => {
                    h4956
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h4957, _, _, _, _) => {
                    h4957
                }
            }, v4)?;
            f256(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f239(v0: bool, v1: u64) -> R<u64> {
    let a4960 = if v0 {
        5u64
    } else {
        4u64
    };
    let a4961 = {
        let a4958 = f257(v0);
        let a4959 = v1;
        nat_mul(a4958, a4959)?
    };
    nat_add(a4960, a4961)
}

pub fn f240(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4969 = {
            let a4962 = 13u64;
            let a4963 = f258(v1);
            nat_mul(a4962, a4963)?
        };
        let a4970 = {
            let a4967 = 4u64;
            let a4968 = {
                let a4964 = v1;
                let a4965 = 4u64;
                let a4966 = 0u64;
                nat_rem(a4964, a4965, a4966)
            };
            nat_mul(a4967, a4968)?
        };
        nat_add(a4969, a4970)
    } else {
        let a4971 = 3u64;
        let a4972 = v1;
        nat_mul(a4971, a4972)
    }
}

pub fn f241(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f242(v0: u64, v1: u64) -> R<u64> {
    let a4977 = {
        let a4975 = {
            let a4973 = v0;
            let a4974 = v1;
            nat_add(a4973, a4974)?
        };
        let a4976 = 1u64;
        nat_sub(a4975, a4976)
    };
    let a4978 = v1;
    let a4979 = 0u64;
    Ok(nat_quot(a4977, a4978, a4979))
}

pub fn f243(v0: u64, v1: u64) -> R<u64> {
    let a4980 = v0;
    let a4981 = f242(v0, v1)?;
    let a4982 = 0u64;
    Ok(nat_quot(a4980, a4981, a4982))
}

pub fn f244(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4983 = v2;
    let a4984 = 0u64;
    if nat_eq(a4983, a4984) {
        Ok(0u64)
    } else {
        let a4989 = 11u64;
        let a4990 = {
            let a4987 = {
                let a4985 = v1;
                let a4986 = v2;
                nat_mul(a4985, a4986)?
            };
            let a4988 = f259(v0)?;
            nat_mul(a4987, a4988)?
        };
        nat_add(a4989, a4990)
    }
}

pub fn f245(v0: u64, v1: u64) -> R<u64> {
    let a4991 = v0;
    let a4992 = f242(v0, v1)?;
    let a4993 = 0u64;
    Ok(nat_rem(a4991, a4992, a4993))
}

pub fn f246(v0: u64) -> bool {
    let m4994 = f112(v0);
    match m4994 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f247(v0: u64, v1: u64) -> bool {
    let a4995 = v0;
    let a4996 = 1u64;
    if nat_eq(a4995, a4996) {
        true
    } else {
        let v2: u64 = f260(v0);
        let a4997 = 0u64;
        let a4998 = v2;
        if nat_lt(a4997, a4998) {
            let a5002 = {
                let a4999 = v1;
                let a5000 = v2;
                let a5001 = 1u64;
                nat_rem(a4999, a5000, a5001)
            };
            let a5003 = 0u64;
            nat_eq(a5002, a5003)
        } else {
            false
        }
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5004 = v0;
    let a5005 = 28u64;
    if if if if nat_eq(a5004, a5005) {
        let a5006 = v1;
        let a5007 = 0u64;
        nat_eq(a5006, a5007)
    } else {
        false
    } {
        let a5008 = v2;
        let a5009 = 0u64;
        nat_eq(a5008, a5009)
    } else {
        false
    } {
        let a5010 = v3;
        let a5011 = 0u64;
        nat_eq(a5010, a5011)
    } else {
        false
    } {
        let a5012 = v4;
        let a5013 = v8;
        if if if if if if if if nat_eq(a5012, a5013) {
            let a5014 = v5;
            let a5015 = v12;
            nat_eq(a5014, a5015)
        } else {
            false
        } {
            let a5016 = v9;
            let a5017 = v13;
            nat_eq(a5016, a5017)
        } else {
            false
        } {
            let a5018 = v10;
            let a5019 = v14;
            nat_eq(a5018, a5019)
        } else {
            false
        } {
            let a5020 = v11;
            let a5021 = v15;
            nat_eq(a5020, a5021)
        } else {
            false
        } {
            let a5022 = 0u64;
            let a5023 = v6;
            nat_lt(a5022, a5023)
        } else {
            false
        } {
            let a5024 = 0u64;
            let a5025 = v7;
            nat_lt(a5024, a5025)
        } else {
            false
        } {
            let a5029 = {
                let a5026 = v10;
                let a5027 = v6;
                let a5028 = 1u64;
                nat_rem(a5026, a5027, a5028)
            };
            let a5030 = 0u64;
            if nat_eq(a5029, a5030) {
                let a5034 = {
                    let a5031 = v11;
                    let a5032 = v7;
                    let a5033 = 1u64;
                    nat_rem(a5031, a5032, a5033)
                };
                let a5035 = 0u64;
                nat_eq(a5034, a5035)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5036 = v10;
                let a5037 = v6;
                let a5038 = 0u64;
                nat_quot(a5036, a5037, a5038)
            }, {
                let a5039 = v11;
                let a5040 = v7;
                let a5041 = 0u64;
                nat_quot(a5039, a5040, a5041)
            }, {
                let a5042 = v5;
                let a5043 = v4;
                nat_mul(a5042, a5043)?
            }, {
                let a5044 = v4;
                let a5045 = v9;
                nat_mul(a5044, a5045)?
            }, {
                let a5046 = v5;
                let a5047 = v9;
                nat_mul(a5046, a5047)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f249(v0: u64) -> bool {
    let a5048 = v0;
    let a5049 = 0u64;
    if if nat_eq(a5048, a5049) {
        true
    } else {
        let a5050 = v0;
        let a5051 = 1u64;
        nat_eq(a5050, a5051)
    } {
        true
    } else {
        let a5052 = v0;
        let a5053 = 30u64;
        nat_eq(a5052, a5053)
    }
}

pub fn f250(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5054, _, _, _, _, _, _, _) => {
            h5054
        }
    };
    let v7: u64 = {
        let a5057 = 1u64;
        let a5058 = {
            let a5055 = 3u64;
            let a5056 = v6;
            nat_mul(a5055, a5056)?
        };
        nat_add(a5057, a5058)?
    };
    let a5059 = v0;
    let a5060 = 0u64;
    if nat_eq(a5059, a5060) {
        Ok(f261(0u64))
    } else {
        let a5061 = v0;
        let a5062 = v7;
        if nat_lt(a5061, a5062) {
            let v8: u64 = {
                let a5063 = v0;
                let a5064 = 1u64;
                nat_sub(a5063, a5064)
            };
            let v9: u64 = {
                let a5070 = {
                    let a5065 = v1;
                    let a5066 = v6;
                    nat_mul(a5065, a5066)?
                };
                let a5071 = {
                    let a5067 = v8;
                    let a5068 = 3u64;
                    let a5069 = 0u64;
                    nat_quot(a5067, a5068, a5069)
                };
                nat_add(a5070, a5071)?
            };
            let v10: u64 = f262(v9, v2)?;
            let v11: u64 = {
                let a5072 = v8;
                let a5073 = 3u64;
                let a5074 = 0u64;
                nat_rem(a5072, a5073, a5074)
            };
            let a5075 = v11;
            let a5076 = 0u64;
            if nat_eq(a5075, a5076) {
                Ok(f263(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5077 = v11;
                let a5078 = 1u64;
                if nat_eq(a5077, a5078) {
                    Ok(f263(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f263(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5079 = v0;
            let a5080 = v7;
            if nat_eq(a5079, a5080) {
                Ok(f263(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5083 = v0;
                let a5084 = {
                    let a5081 = v7;
                    let a5082 = 1u64;
                    nat_add(a5081, a5082)?
                };
                if nat_eq(a5083, a5084) {
                    Ok(f263(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5087 = v0;
                    let a5088 = {
                        let a5085 = v7;
                        let a5086 = 2u64;
                        nat_add(a5085, a5086)?
                    };
                    if nat_eq(a5087, a5088) {
                        Ok(f263(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5091 = v0;
                        let a5092 = {
                            let a5089 = v7;
                            let a5090 = 3u64;
                            nat_add(a5089, a5090)?
                        };
                        if nat_eq(a5091, a5092) {
                            let a5093 = v5;
                            let a5094 = 1u64;
                            if if nat_eq(a5093, a5094) {
                                true
                            } else {
                                let a5095 = v5;
                                let a5096 = 2u64;
                                nat_eq(a5095, a5096)
                            } {
                                Ok(f263(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f263(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5103 = {
                                    let a5097 = v1;
                                    let a5098 = v6;
                                    nat_mul(a5097, a5098)?
                                };
                                let a5104 = {
                                    let a5101 = v0;
                                    let a5102 = {
                                        let a5099 = v7;
                                        let a5100 = 4u64;
                                        nat_add(a5099, a5100)?
                                    };
                                    nat_sub(a5101, a5102)
                                };
                                nat_add(a5103, a5104)?
                            };
                            Ok(f263(10u64, 0u64, f262(v12, v2)?, 1u64, 2u64, 3u64, f262(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f251(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5105, _, _, _, _, _, _, _) => {
            h5105
        }
    }, match v0.clone() {
        Adt20::C0(_, h5106, _, _, _, _, _, _) => {
            h5106
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5107, _, _, _, _, _) => {
            h5107
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5108, _, _, _, _) => {
            h5108
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5109, _, _) => {
            h5109
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5110, _) => {
            h5110
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5111) => {
            h5111
        }
    })
}

pub fn f252(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5118 = v3;
        let a5119 = {
            let a5115 = f262({
                let a5112 = v0;
                let a5113 = 2u64;
                let a5114 = 0u64;
                nat_quot(a5112, a5113, a5114)
            }, v1)?;
            let a5116 = v2;
            let a5117 = 0u64;
            nat_quot(a5115, a5116, a5117)
        };
        nat_add(a5118, a5119)?
    };
    let a5123 = {
        let a5120 = v0;
        let a5121 = 2u64;
        let a5122 = 0u64;
        nat_rem(a5120, a5121, a5122)
    };
    let a5124 = 0u64;
    if nat_eq(a5123, a5124) {
        Ok(f263(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5125 = v4;
        let a5126 = 0u64;
        if nat_eq(a5125, a5126) {
            Ok(f263(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f253(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f264(match v2.clone() {
        Adt20::C0(h5127, _, _, _, _, _, _, _) => {
            h5127
        }
    })?;
    let v7: u64 = f265(match v2.clone() {
        Adt20::C0(h5128, _, _, _, _, _, _, _) => {
            h5128
        }
    })?;
    let a5129 = v0;
    let a5130 = 2u64;
    if nat_lt(a5129, a5130) {
        Ok(f261(v0))
    } else {
        let a5131 = v0;
        let a5132 = v6;
        if nat_lt(a5131, a5132) {
            f266({
                let a5133 = v0;
                let a5134 = 2u64;
                nat_sub(a5133, a5134)
            }, v1, v2, v3)
        } else {
            let a5135 = v0;
            let a5136 = v7;
            if nat_lt(a5135, a5136) {
                Ok(f267({
                    let a5137 = v0;
                    let a5138 = v6;
                    nat_sub(a5137, a5138)
                }, match v2 {
                    Adt20::C0(h5139, _, _, _, _, _, _, _) => {
                        h5139
                    }
                }, v5))
            } else {
                let a5141 = v0;
                let a5142 = f225(match v2.clone() {
                    Adt20::C0(h5140, _, _, _, _, _, _, _) => {
                        h5140
                    }
                })?;
                if nat_lt(a5141, a5142) {
                    f268({
                        let a5143 = v0;
                        let a5144 = v7;
                        nat_sub(a5143, a5144)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f254(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f239(v5, match v2.clone() {
        Adt20::C0(h5145, _, _, _, _, _, _, _) => {
            h5145
        }
    })?;
    let v7: u64 = {
        let a5147 = v6;
        let a5148 = f240(v5, match v2.clone() {
            Adt20::C0(h5146, _, _, _, _, _, _, _) => {
                h5146
            }
        })?;
        nat_add(a5147, a5148)?
    };
    let v8: u64 = {
        let a5149 = v7;
        let a5150 = f241(v5);
        nat_add(a5149, a5150)?
    };
    let a5151 = v0;
    let a5152 = v6;
    if nat_lt(a5151, a5152) {
        f269(v0, v1, v2, v5)
    } else {
        let a5153 = v0;
        let a5154 = v7;
        if nat_lt(a5153, a5154) {
            f270({
                let a5155 = v0;
                let a5156 = v6;
                nat_sub(a5155, a5156)
            }, v1, v2, v3, v5)
        } else {
            let a5157 = v0;
            let a5158 = v8;
            if nat_lt(a5157, a5158) {
                Ok(f271({
                    let a5159 = v0;
                    let a5160 = v7;
                    nat_sub(a5159, a5160)
                }, match v2 {
                    Adt20::C0(h5161, _, _, _, _, _, _, _) => {
                        h5161
                    }
                }, v4, v5))
            } else {
                let a5165 = v0;
                let a5166 = {
                    let a5163 = v8;
                    let a5164 = match v2.clone() {
                        Adt20::C0(h5162, _, _, _, _, _, _, _) => {
                            h5162
                        }
                    };
                    nat_add(a5163, a5164)?
                };
                if nat_lt(a5165, a5166) {
                    let v9: u64 = f262({
                        let a5172 = {
                            let a5168 = v1;
                            let a5169 = match v2 {
                                Adt20::C0(h5167, _, _, _, _, _, _, _) => {
                                    h5167
                                }
                            };
                            nat_mul(a5168, a5169)?
                        };
                        let a5173 = {
                            let a5170 = v0;
                            let a5171 = v8;
                            nat_sub(a5170, a5171)
                        };
                        nat_add(a5172, a5173)?
                    }, v3)?;
                    Ok(f263(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f255(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f242(v3, v5)?;
    let v7: u64 = f244(v1, v2, v6)?;
    let v8: u64 = {
        let a5176 = {
            let a5174 = f243(v3, v5)?;
            let a5175 = v4;
            nat_mul(a5174, a5175)?
        };
        let a5177 = v7;
        nat_mul(a5176, a5177)?
    };
    let a5178 = v0;
    let a5179 = v8;
    if nat_lt(a5178, a5179) {
        let v9: u64 = {
            let a5180 = v0;
            let a5181 = v7;
            let a5182 = 0u64;
            nat_quot(a5180, a5181, a5182)
        };
        Ok(Adt22::C0({
            let a5186 = {
                let a5183 = v9;
                let a5184 = v4;
                let a5185 = 0u64;
                nat_quot(a5183, a5184, a5185)
            };
            let a5187 = v6;
            nat_mul(a5186, a5187)?
        }, v6, {
            let a5188 = v9;
            let a5189 = v4;
            let a5190 = 0u64;
            nat_rem(a5188, a5189, a5190)
        }, {
            let a5191 = v0;
            let a5192 = v7;
            let a5193 = 0u64;
            nat_rem(a5191, a5192, a5193)
        }))
    } else {
        let v10: u64 = {
            let a5194 = v0;
            let a5195 = v8;
            nat_sub(a5194, a5195)
        };
        let v11: u64 = f244(v1, v2, f245(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5196 = f243(v3, v5)?;
            let a5197 = v6;
            nat_mul(a5196, a5197)?
        }, f245(v3, v5)?, {
            let a5198 = v10;
            let a5199 = v11;
            let a5200 = 0u64;
            nat_quot(a5198, a5199, a5200)
        }, {
            let a5201 = v10;
            let a5202 = v11;
            let a5203 = 0u64;
            nat_rem(a5201, a5202, a5203)
        }))
    }
}

pub fn f256(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5204) => {
            h5204
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5205, _, _, _, _, _, _, _) => {
            h5205
        }
    };
    let v6: u64 = {
        let a5208 = match v0.clone() {
            Adt22::C0(_, h5206, _, _) => {
                h5206
            }
        };
        let a5209 = match v1.clone() {
            Adt20::C0(_, h5207, _, _, _, _, _, _) => {
                h5207
            }
        };
        nat_mul(a5208, a5209)?
    };
    let v7: u64 = {
        let a5210 = v6;
        let a5211 = v5;
        nat_mul(a5210, a5211)?
    };
    let v8: u64 = {
        let a5214 = {
            let a5212 = 2u64;
            let a5213 = v5;
            nat_mul(a5212, a5213)?
        };
        let a5215 = 2u64;
        nat_add(a5214, a5215)?
    };
    let v9: u64 = {
        let a5218 = {
            let a5216 = 4u64;
            let a5217 = v5;
            nat_mul(a5216, a5217)?
        };
        let a5219 = 2u64;
        nat_add(a5218, a5219)?
    };
    let v10: u64 = {
        let a5222 = 1u64;
        let a5223 = {
            let a5220 = v6;
            let a5221 = v8;
            nat_mul(a5220, a5221)?
        };
        nat_add(a5222, a5223)?
    };
    let v11: u64 = {
        let a5224 = v10;
        let a5225 = 4u64;
        nat_add(a5224, a5225)?
    };
    let v12: u64 = {
        let a5228 = v11;
        let a5229 = {
            let a5226 = v6;
            let a5227 = v9;
            nat_mul(a5226, a5227)?
        };
        nat_add(a5228, a5229)?
    };
    let v13: u64 = {
        let a5230 = v12;
        let a5231 = 6u64;
        nat_add(a5230, a5231)?
    };
    let a5232 = v4;
    let a5233 = 0u64;
    if nat_eq(a5232, a5233) {
        Ok(f261(0u64))
    } else {
        let a5234 = v4;
        let a5235 = v10;
        if nat_lt(a5234, a5235) {
            let v14: u64 = {
                let a5236 = v4;
                let a5237 = 1u64;
                nat_sub(a5236, a5237)
            };
            f272(false, {
                let a5238 = v14;
                let a5239 = v8;
                let a5240 = 0u64;
                nat_rem(a5238, a5239, a5240)
            }, {
                let a5241 = v14;
                let a5242 = v8;
                let a5243 = 0u64;
                nat_quot(a5241, a5242, a5243)
            }, v0, v1, v2)
        } else {
            let a5244 = v4;
            let a5245 = v11;
            if nat_lt(a5244, a5245) {
                Ok(f273({
                    let a5246 = v4;
                    let a5247 = v10;
                    nat_sub(a5246, a5247)
                }, v7))
            } else {
                let a5248 = v4;
                let a5249 = v12;
                if nat_lt(a5248, a5249) {
                    let v15: u64 = {
                        let a5250 = v4;
                        let a5251 = v11;
                        nat_sub(a5250, a5251)
                    };
                    f272(true, {
                        let a5252 = v15;
                        let a5253 = v9;
                        let a5254 = 0u64;
                        nat_rem(a5252, a5253, a5254)
                    }, {
                        let a5255 = v15;
                        let a5256 = v9;
                        let a5257 = 0u64;
                        nat_quot(a5255, a5256, a5257)
                    }, v0, v1, v2)
                } else {
                    let a5258 = v4;
                    let a5259 = v13;
                    if nat_lt(a5258, a5259) {
                        Ok(f274({
                            let a5260 = v4;
                            let a5261 = v12;
                            nat_sub(a5260, a5261)
                        }, v7, v3))
                    } else {
                        let a5264 = v4;
                        let a5265 = {
                            let a5262 = v13;
                            let a5263 = v7;
                            nat_add(a5262, a5263)?
                        };
                        if nat_lt(a5264, a5265) {
                            let v16: u64 = {
                                let a5266 = v4;
                                let a5267 = v13;
                                nat_sub(a5266, a5267)
                            };
                            let v17: u64 = {
                                let a5268 = v16;
                                let a5269 = v5;
                                let a5270 = 0u64;
                                nat_quot(a5268, a5269, a5270)
                            };
                            let v18: u64 = {
                                let a5276 = {
                                    let a5271 = f275(v17, v0, v1)?;
                                    let a5272 = v5;
                                    nat_mul(a5271, a5272)?
                                };
                                let a5277 = {
                                    let a5273 = v16;
                                    let a5274 = v5;
                                    let a5275 = 0u64;
                                    nat_rem(a5273, a5274, a5275)
                                };
                                nat_add(a5276, a5277)?
                            };
                            let v19: u64 = f262(v18, v2)?;
                            Ok(f263(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f257(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f258(v0: u64) -> u64 {
    let a5278 = v0;
    let a5279 = 4u64;
    let a5280 = 0u64;
    nat_quot(a5278, a5279, a5280)
}

pub fn f259(v0: u64) -> R<u64> {
    let a5283 = {
        let a5281 = 7u64;
        let a5282 = v0;
        nat_mul(a5281, a5282)?
    };
    let a5284 = 4u64;
    nat_add(a5283, a5284)
}

pub fn f260(v0: u64) -> u64 {
    if f110(v0) {
        let m5285 = f112(v0);
        match m5285 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5287 = 1u64;
                let a5288 = match v1.clone() {
                    Adt18::C0(_, h5286, _) => {
                        h5286
                    }
                };
                if if nat_lt(a5287, a5288) {
                    true
                } else {
                    let a5289 = v0;
                    let a5290 = 43u64;
                    nat_eq(a5289, a5290)
                } {
                    match v1 {
                        Adt18::C0(_, h5291, _) => {
                            h5291
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

pub fn f261(v0: u64) -> Adt19 {
    f263(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f262(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5293 = v0;
        let a5294 = match v1.clone() {
            Adt20::C0(h5292, _, _, _, _, _, _, _) => {
                h5292
            }
        };
        let a5295 = 0u64;
        nat_quot(a5293, a5294, a5295)
    };
    let v3: u64 = {
        let a5297 = v2;
        let a5298 = match v1.clone() {
            Adt20::C0(_, h5296, _, _, _, _, _, _) => {
                h5296
            }
        };
        let a5299 = 0u64;
        nat_quot(a5297, a5298, a5299)
    };
    f128({
        let a5301 = v0;
        let a5302 = match v1.clone() {
            Adt20::C0(h5300, _, _, _, _, _, _, _) => {
                h5300
            }
        };
        let a5303 = 0u64;
        nat_rem(a5301, a5302, a5303)
    }, {
        let a5305 = v2;
        let a5306 = match v1.clone() {
            Adt20::C0(_, h5304, _, _, _, _, _, _) => {
                h5304
            }
        };
        let a5307 = 0u64;
        nat_rem(a5305, a5306, a5307)
    }, {
        let a5309 = v3;
        let a5310 = match v1.clone() {
            Adt20::C0(_, _, h5308, _, _, _, _, _) => {
                h5308
            }
        };
        let a5311 = 0u64;
        nat_rem(a5309, a5310, a5311)
    }, {
        let a5313 = v3;
        let a5314 = match v1.clone() {
            Adt20::C0(_, _, h5312, _, _, _, _, _) => {
                h5312
            }
        };
        let a5315 = 0u64;
        nat_quot(a5313, a5314, a5315)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5316, _, _, _) => {
            h5316
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5317, _, _) => {
            h5317
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5318, _) => {
            h5318
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5319) => {
            h5319
        }
    })
}

pub fn f263(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5320 = v0;
    let a5321 = 4u64;
    if if if nat_le(a5320, a5321) {
        true
    } else {
        let a5322 = 9u64;
        let a5323 = v0;
        if nat_le(a5322, a5323) {
            let a5324 = v0;
            let a5325 = 12u64;
            nat_le(a5324, a5325)
        } else {
            false
        }
    } {
        true
    } else {
        let a5326 = v0;
        let a5327 = 16u64;
        if if nat_eq(a5326, a5327) {
            true
        } else {
            let a5328 = v0;
            let a5329 = 19u64;
            nat_eq(a5328, a5329)
        } {
            true
        } else {
            let a5330 = v0;
            let a5331 = 21u64;
            if nat_eq(a5330, a5331) {
                true
            } else {
                let a5332 = v0;
                let a5333 = 22u64;
                nat_eq(a5332, a5333)
            }
        }
    } {
        Adt19::C1({
            let a5334 = v0;
            let a5335 = 4u64;
            if if nat_lt(a5334, a5335) {
                true
            } else {
                let a5336 = 21u64;
                let a5337 = v0;
                nat_le(a5336, a5337)
            } {
                0u64
            } else {
                let a5338 = v0;
                let a5339 = 16u64;
                if if nat_eq(a5338, a5339) {
                    true
                } else {
                    let a5340 = v0;
                    let a5341 = 19u64;
                    nat_eq(a5340, a5341)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5342 = v0;
            let a5343 = 4u64;
            if nat_lt(a5342, a5343) {
                v0
            } else {
                let a5344 = v0;
                let a5345 = 4u64;
                if if nat_eq(a5344, a5345) {
                    true
                } else {
                    let a5346 = v0;
                    let a5347 = 16u64;
                    nat_eq(a5346, a5347)
                } {
                    0u64
                } else {
                    let a5348 = v0;
                    let a5349 = 19u64;
                    if nat_eq(a5348, a5349) {
                        3u64
                    } else {
                        let a5350 = 21u64;
                        let a5351 = v0;
                        if nat_le(a5350, a5351) {
                            let a5352 = v0;
                            let a5353 = 17u64;
                            nat_sub(a5352, a5353)
                        } else {
                            let a5354 = v0;
                            let a5355 = 8u64;
                            nat_sub(a5354, a5355)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f264(v0: u64) -> R<u64> {
    let a5358 = 2u64;
    let a5359 = {
        let a5356 = 6u64;
        let a5357 = v0;
        nat_mul(a5356, a5357)?
    };
    nat_add(a5358, a5359)
}

pub fn f265(v0: u64) -> R<u64> {
    let a5360 = f264(v0)?;
    let a5361 = 11u64;
    nat_add(a5360, a5361)
}

pub fn f266(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5362 = v0;
        let a5363 = 6u64;
        let a5364 = 0u64;
        nat_rem(a5362, a5363, a5364)
    };
    let v5: u64 = {
        let a5371 = {
            let a5366 = v1;
            let a5367 = match v2.clone() {
                Adt20::C0(h5365, _, _, _, _, _, _, _) => {
                    h5365
                }
            };
            nat_mul(a5366, a5367)?
        };
        let a5372 = {
            let a5368 = v0;
            let a5369 = 6u64;
            let a5370 = 0u64;
            nat_quot(a5368, a5369, a5370)
        };
        nat_add(a5371, a5372)?
    };
    let v6: u64 = f262(v5, v3)?;
    let v7: u64 = f262(v5, v2)?;
    let a5373 = v4;
    let a5374 = 0u64;
    if nat_eq(a5373, a5374) {
        Ok(f263(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5375 = v4;
        let a5376 = 1u64;
        if nat_eq(a5375, a5376) {
            Ok(f263(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5377 = v4;
            let a5378 = 2u64;
            if nat_eq(a5377, a5378) {
                Ok(f263(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5379 = v4;
                let a5380 = 3u64;
                if nat_eq(a5379, a5380) {
                    Ok(f263(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5381 = v4;
                    let a5382 = 4u64;
                    if nat_eq(a5381, a5382) {
                        Ok(f263(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f263(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f267(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5383 = v0;
    let a5384 = 0u64;
    if nat_eq(a5383, a5384) {
        f263(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5385 = v0;
        let a5386 = 1u64;
        if nat_eq(a5385, a5386) {
            f263(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5387 = v0;
            let a5388 = 2u64;
            if nat_eq(a5387, a5388) {
                f263(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5389 = v0;
                let a5390 = 3u64;
                if nat_eq(a5389, a5390) {
                    f263(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5391 = v0;
                    let a5392 = 4u64;
                    if nat_eq(a5391, a5392) {
                        f263(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5393 = v0;
                        let a5394 = 5u64;
                        if nat_eq(a5393, a5394) {
                            f263(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5395 = v0;
                            let a5396 = 6u64;
                            if nat_eq(a5395, a5396) {
                                f263(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5397 = v0;
                                let a5398 = 7u64;
                                if nat_eq(a5397, a5398) {
                                    f263(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5399 = v0;
                                    let a5400 = 8u64;
                                    if nat_eq(a5399, a5400) {
                                        f263(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5401 = v0;
                                        let a5402 = 9u64;
                                        if nat_eq(a5401, a5402) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5403 = v0;
                                            let a5404 = 10u64;
                                            if nat_eq(a5403, a5404) {
                                                f263(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f268(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5405 = v0;
        let a5406 = 3u64;
        let a5407 = 0u64;
        nat_rem(a5405, a5406, a5407)
    };
    let v6: u64 = {
        let a5414 = {
            let a5409 = v1;
            let a5410 = match v2.clone() {
                Adt20::C0(h5408, _, _, _, _, _, _, _) => {
                    h5408
                }
            };
            nat_mul(a5409, a5410)?
        };
        let a5415 = {
            let a5411 = v0;
            let a5412 = 3u64;
            let a5413 = 0u64;
            nat_quot(a5411, a5412, a5413)
        };
        nat_add(a5414, a5415)?
    };
    let v7: u64 = f262(v6, v3)?;
    let v8: u64 = f262(v6, v2)?;
    let v9: u64 = f262(v6, v4)?;
    let a5416 = v5;
    let a5417 = 0u64;
    if nat_eq(a5416, a5417) {
        Ok(f263(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5418 = v5;
        let a5419 = 1u64;
        if nat_eq(a5418, a5419) {
            Ok(f263(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f263(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5420, _, _, _, _, _, _, _) => {
            h5420
        }
    };
    let v5: u64 = {
        let a5423 = 1u64;
        let a5424 = {
            let a5421 = v4;
            let a5422 = f257(v3);
            nat_mul(a5421, a5422)?
        };
        nat_add(a5423, a5424)?
    };
    let a5425 = v0;
    let a5426 = 0u64;
    if nat_eq(a5425, a5426) {
        Ok(f261(0u64))
    } else {
        let a5427 = v0;
        let a5428 = v5;
        if nat_lt(a5427, a5428) {
            let v6: u64 = {
                let a5429 = v0;
                let a5430 = 1u64;
                nat_sub(a5429, a5430)
            };
            let v7: u64 = f262({
                let a5436 = {
                    let a5431 = v1;
                    let a5432 = v4;
                    nat_mul(a5431, a5432)?
                };
                let a5437 = {
                    let a5433 = v6;
                    let a5434 = f257(v3);
                    let a5435 = 0u64;
                    nat_quot(a5433, a5434, a5435)
                };
                nat_add(a5436, a5437)?
            }, v2)?;
            if v3 {
                let a5441 = {
                    let a5438 = v6;
                    let a5439 = 2u64;
                    let a5440 = 0u64;
                    nat_rem(a5438, a5439, a5440)
                };
                let a5442 = 0u64;
                if nat_eq(a5441, a5442) {
                    Ok(f263(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f263(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f263(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5443 = v0;
                let a5444 = v5;
                nat_sub(a5443, a5444)
            };
            let a5445 = v8;
            let a5446 = 0u64;
            if nat_eq(a5445, a5446) {
                Ok(f263(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5447 = v8;
                    let a5448 = 1u64;
                    nat_eq(a5447, a5448)
                } else {
                    false
                } {
                    Ok(f263(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5449 = v8;
                    let a5450 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5449, a5450) {
                        Ok(f263(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5451 = v8;
                        let a5452 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5451, a5452) {
                            Ok(f261(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5454 = 13u64;
        let a5455 = f258(match v2.clone() {
            Adt20::C0(h5453, _, _, _, _, _, _, _) => {
                h5453
            }
        });
        nat_mul(a5454, a5455)?
    } else {
        0u64
    };
    let a5456 = v0;
    let a5457 = v5;
    if nat_lt(a5456, a5457) {
        f276({
            let a5458 = v0;
            let a5459 = 13u64;
            let a5460 = 0u64;
            nat_rem(a5458, a5459, a5460)
        }, v1, {
            let a5461 = v0;
            let a5462 = 13u64;
            let a5463 = 0u64;
            nat_quot(a5461, a5462, a5463)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5464 = v0;
            let a5465 = v5;
            nat_sub(a5464, a5465)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f277({
            let a5466 = v6;
            let a5467 = v7;
            let a5468 = 0u64;
            nat_rem(a5466, a5467, a5468)
        }, {
            let a5478 = {
                let a5473 = {
                    let a5470 = v1;
                    let a5471 = match v2.clone() {
                        Adt20::C0(h5469, _, _, _, _, _, _, _) => {
                            h5469
                        }
                    };
                    nat_mul(a5470, a5471)?
                };
                let a5474 = if v4 {
                    f278(match v2.clone() {
                        Adt20::C0(h5472, _, _, _, _, _, _, _) => {
                            h5472
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5473, a5474)?
            };
            let a5479 = {
                let a5475 = v6;
                let a5476 = v7;
                let a5477 = 0u64;
                nat_quot(a5475, a5476, a5477)
            };
            nat_add(a5478, a5479)?
        }, v2, v3, v4)
    }
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5480 = v0;
    let a5481 = 0u64;
    if nat_eq(a5480, a5481) {
        f263(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5482 = v0;
        let a5483 = 1u64;
        if nat_eq(a5482, a5483) {
            f263(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5484 = v0;
                let a5485 = 2u64;
                nat_eq(a5484, a5485)
            } else {
                false
            } {
                f263(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5486 = v0;
                let a5487 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5486, a5487) {
                    f263(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5488 = v0;
                    let a5489 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5488, a5489) {
                        f263(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5490 = v0;
                        let a5491 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5490, a5491) {
                            f263(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f272(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5495 = 1u64;
        let a5496 = {
            let a5493 = match v4.clone() {
                Adt20::C0(h5492, _, _, _, _, _, _, _) => {
                    h5492
                }
            };
            let a5494 = v6;
            nat_mul(a5493, a5494)?
        };
        nat_add(a5495, a5496)?
    };
    let a5497 = v1;
    let a5498 = 0u64;
    if nat_eq(a5497, a5498) {
        Ok(f261(1u64))
    } else {
        let a5499 = v1;
        let a5500 = v7;
        if nat_lt(a5499, a5500) {
            let v8: u64 = {
                let a5501 = v1;
                let a5502 = 1u64;
                nat_sub(a5501, a5502)
            };
            let v9: u64 = {
                let a5509 = {
                    let a5504 = f275(v2, v3, v4.clone())?;
                    let a5505 = match v4.clone() {
                        Adt20::C0(h5503, _, _, _, _, _, _, _) => {
                            h5503
                        }
                    };
                    nat_mul(a5504, a5505)?
                };
                let a5510 = {
                    let a5506 = v8;
                    let a5507 = v6;
                    let a5508 = 0u64;
                    nat_quot(a5506, a5507, a5508)
                };
                nat_add(a5509, a5510)?
            };
            let v10: u64 = f262(v9, v4)?;
            let v11: u64 = f262(v9, v5)?;
            let v12: u64 = {
                let a5511 = v8;
                let a5512 = v6;
                let a5513 = 0u64;
                nat_rem(a5511, a5512, a5513)
            };
            if v0 {
                let a5514 = v12;
                let a5515 = 0u64;
                if nat_eq(a5514, a5515) {
                    Ok(f263(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5516 = v12;
                    let a5517 = 1u64;
                    if nat_eq(a5516, a5517) {
                        Ok(f263(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5518 = v12;
                        let a5519 = 2u64;
                        if nat_eq(a5518, a5519) {
                            Ok(f263(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f263(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5520 = v12;
                let a5521 = 0u64;
                if nat_eq(a5520, a5521) {
                    Ok(f263(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f263(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5522 = v1;
            let a5523 = v7;
            if nat_eq(a5522, a5523) {
                Ok(f263(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f273(v0: u64, v1: u64) -> Adt19 {
    let a5524 = v0;
    let a5525 = 0u64;
    if nat_eq(a5524, a5525) {
        f263(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5526 = v0;
        let a5527 = 1u64;
        if nat_eq(a5526, a5527) {
            f263(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5528 = v0;
            let a5529 = 2u64;
            if nat_eq(a5528, a5529) {
                f263(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5530 = v0;
                let a5531 = 3u64;
                if nat_eq(a5530, a5531) {
                    f261(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5532 = v0;
    let a5533 = 0u64;
    if nat_eq(a5532, a5533) {
        f263(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5534 = v0;
        let a5535 = 1u64;
        if nat_eq(a5534, a5535) {
            f263(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5536 = v0;
            let a5537 = 2u64;
            if nat_eq(a5536, a5537) {
                f263(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5538 = v0;
                let a5539 = 3u64;
                if nat_eq(a5538, a5539) {
                    f263(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5540 = v0;
                    let a5541 = 4u64;
                    if nat_eq(a5540, a5541) {
                        f263(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5542 = v0;
                        let a5543 = 5u64;
                        if nat_eq(a5542, a5543) {
                            f263(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f275(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5554 = {
        let a5552 = {
            let a5549 = {
                let a5546 = match v1.clone() {
                    Adt22::C0(_, _, h5544, _) => {
                        h5544
                    }
                };
                let a5547 = match v2.clone() {
                    Adt20::C0(_, _, h5545, _, _, _, _, _) => {
                        h5545
                    }
                };
                nat_mul(a5546, a5547)?
            };
            let a5550 = match v1 {
                Adt22::C0(h5548, _, _, _) => {
                    h5548
                }
            };
            nat_add(a5549, a5550)?
        };
        let a5553 = match v2 {
            Adt20::C0(_, h5551, _, _, _, _, _, _) => {
                h5551
            }
        };
        nat_mul(a5552, a5553)?
    };
    let a5555 = v0;
    nat_add(a5554, a5555)
}

pub fn f276(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5563 = {
            let a5561 = {
                let a5557 = v1;
                let a5558 = match v3.clone() {
                    Adt20::C0(h5556, _, _, _, _, _, _, _) => {
                        h5556
                    }
                };
                nat_mul(a5557, a5558)?
            };
            let a5562 = {
                let a5559 = v2;
                let a5560 = 4u64;
                nat_mul(a5559, a5560)?
            };
            nat_add(a5561, a5562)?
        };
        let a5564 = f279(v0);
        nat_add(a5563, a5564)?
    };
    let v6: u64 = f262(v5, v3)?;
    let v7: u64 = f262(v5, v4)?;
    let a5565 = v0;
    let a5566 = 0u64;
    if if if if nat_eq(a5565, a5566) {
        true
    } else {
        let a5567 = v0;
        let a5568 = 2u64;
        nat_eq(a5567, a5568)
    } {
        true
    } else {
        let a5569 = v0;
        let a5570 = 5u64;
        nat_eq(a5569, a5570)
    } {
        true
    } else {
        let a5571 = v0;
        let a5572 = 7u64;
        nat_eq(a5571, a5572)
    } {
        Ok(f263(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5573 = v0;
        let a5574 = 1u64;
        if if if if nat_eq(a5573, a5574) {
            true
        } else {
            let a5575 = v0;
            let a5576 = 3u64;
            nat_eq(a5575, a5576)
        } {
            true
        } else {
            let a5577 = v0;
            let a5578 = 6u64;
            nat_eq(a5577, a5578)
        } {
            true
        } else {
            let a5579 = v0;
            let a5580 = 8u64;
            nat_eq(a5579, a5580)
        } {
            Ok(f263(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5581 = v0;
                let a5582 = 1u64;
                if nat_eq(a5581, a5582) {
                    4u64
                } else {
                    let a5583 = v0;
                    let a5584 = 6u64;
                    if nat_eq(a5583, a5584) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5585 = v0;
            let a5586 = 4u64;
            if nat_eq(a5585, a5586) {
                Ok(f263(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5587 = v0;
                let a5588 = 9u64;
                if nat_eq(a5587, a5588) {
                    Ok(f263(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5589 = v0;
                    let a5590 = 10u64;
                    if nat_eq(a5589, a5590) {
                        Ok(f263(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5591 = v0;
                        let a5592 = 11u64;
                        if nat_eq(a5591, a5592) {
                            Ok(f263(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5593 = v0;
                            let a5594 = 12u64;
                            if nat_eq(a5593, a5594) {
                                Ok(f263(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f277(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f262(v1, v2)?;
    let v6: u64 = f262(v1, v3)?;
    let a5595 = v0;
    let a5596 = 0u64;
    if nat_eq(a5595, a5596) {
        Ok(f263(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5597 = v0;
        let a5598 = 1u64;
        if nat_eq(a5597, a5598) {
            Ok(f263(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5599 = v0;
                let a5600 = 2u64;
                if nat_eq(a5599, a5600) {
                    Ok(f263(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5601 = v0;
                    let a5602 = 3u64;
                    if nat_eq(a5601, a5602) {
                        Ok(f263(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5603 = v0;
                let a5604 = 2u64;
                if nat_eq(a5603, a5604) {
                    Ok(f263(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f278(v0: u64) -> R<u64> {
    let a5605 = 4u64;
    let a5606 = f258(v0);
    nat_mul(a5605, a5606)
}

pub fn f279(v0: u64) -> u64 {
    let a5607 = v0;
    let a5608 = 5u64;
    if nat_lt(a5607, a5608) {
        let a5609 = v0;
        let a5610 = 2u64;
        if nat_lt(a5609, a5610) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5611 = v0;
        let a5612 = 7u64;
        if nat_lt(a5611, a5612) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
