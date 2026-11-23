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
                f71(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v50) => {
                f72(v0, v1, v2, v50, v4)?
            }
            Adt4::C20(v51, v52) => {
                f73(v51, v0, v1, v2, v52, v4)?
            }
            Adt4::C21 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C22(v53, v54) => {
                f74(v0, v1, v2, v53, v54, v4)?
            }
            Adt4::C23(v68, v69, v70, v71) => {
                f81(v0, v71, v68, v69, v70, v4)?
            }
            Adt4::C24(v67) => {
                if v5 {
                    f79(Adt16::C0(v67), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C26(v63, v64, v65, v66) => {
                let a478 = v63;
                let a479 = 64u64;
                if if if nat_eq(a478, a479) {
                    true
                } else {
                    let a480 = v63;
                    let a481 = 65u64;
                    nat_eq(a480, a481)
                } {
                    true
                } else {
                    let a482 = v63;
                    let a483 = 66u64;
                    nat_eq(a482, a483)
                } {
                    if v5 {
                        f79(Adt16::C1(v63, v64, v65, v66), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a484 = v63;
                    let a485 = 0u64;
                    if if nat_eq(a484, a485) {
                        true
                    } else {
                        let a486 = v63;
                        let a487 = 30u64;
                        nat_eq(a486, a487)
                    } {
                        f80(v0, v1, v2, v63, v64, v65, v66, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v62) => {
                f78(v0, v1, v2, v62)?
            }
            Adt4::C28(v58) => {
                f75(v0, v1, v2, v58, v4)?
            }
            Adt4::C29(v59) => {
                f76(v0, v1, v2, v59)?
            }
            Adt4::C30(v60, _) => {
                f77(v0, v1, v2, v60)?
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
        Adt4::C25(v45, _, v47) => {
            Ok(Adt9::C6({
                let a493 = {
                    let a489 = 0u64;
                    let a490 = v45;
                    if nat_lt(a489, a490) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a494 = {
                    let a491 = 0u64;
                    let a492 = v47;
                    if nat_lt(a491, a492) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a493, a494)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v53, _, _, _) => {
            Ok(Adt9::C6({
                let a495 = v53;
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
        Adt9::C7(f82(v0, v1, v2, v3, v4), f83(v5, v6), f83(v7, v8), f84({
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
        f85(Adt6::C0(1u64, match v0.clone() {
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
        f85(v0, v1)
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
        let a644 = f86(v3);
        nat_mul(a643, a644)?
    };
    nat_mul(a645, a646)
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a649 = v0;
    let a650 = {
        let a647 = f87(v1)?;
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
        f88(v1, v4, v6)
    } else {
        let a664 = v0;
        let a665 = 26u64;
        if nat_eq(a664, a665) {
            f89(v1, v2, v4, v6)
        } else {
            let a666 = v0;
            let a667 = 25u64;
            if nat_eq(a666, a667) {
                f90(v1, v3, v4, v5, v6)
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
        f91(match v0.clone() {
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
    f92({
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
            let a734 = f93(v1, v2, v3)?;
            let a735 = f94(v0, v1, v2)?;
            nat_add(a734, a735)?
        };
        let a741 = {
            let a738 = f95(v0);
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
        let a757 = f96(v0)?;
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
    if if if if if if if if if if if if if if if f97(match v1.clone() {
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
        f98(v1.clone(), v7)?
    } else {
        false
    } {
        f98(v2.clone(), v7)?
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
            let a833 = f99(v8.clone())?;
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
                    let a904 = f100(match v3.clone() {
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
        f101(v4.clone(), match v2.clone() {
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
            f101(v5.clone(), match v2.clone() {
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
            let a955 = f102(v6.clone())?;
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
                let a1012 = f85(v6.clone(), match v4.clone() {
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
                        let a1021 = f85(v7.clone(), v11)?;
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
            let v9: u64 = f85(v7.clone(), match v4.clone() {
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
                let a1124 = f85(v5.clone(), match v4.clone() {
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
                let a1128 = f85(v6.clone(), match v4.clone() {
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
            f103(v0, v3, v2)
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
            f104(v0, v3, v2)
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
        f105(v0, v1, v2, v3)
    } else {
        if if if if f97(match v1.clone() {
            Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
                h1200
            }
        }) {
            f97(match v3.clone() {
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
            let m1207 = f106(match v3.clone() {
                Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                    h1206
                }
            });
            match m1207 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    Ok(Adt10::C4(f14(v3.clone(), 1u64)?, match v4 {
                        Adt18::C0(_, _, h1208) => {
                            h1208
                        }
                    }, f18(v1.clone(), 1u64), f18(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                            h1209
                        }
                    }, match v3 {
                        Adt2::C0(h1210, _, _, _, _, _, _, _, _, _, _) => {
                            h1210
                        }
                    })))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f48(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1212 = match v0.clone() {
        Adt2::C0(h1211, _, _, _, _, _, _, _, _, _, _) => {
            h1211
        }
    };
    let a1213 = 0u64;
    if if if if nat_eq(a1212, a1213) {
        true
    } else {
        let a1215 = match v0.clone() {
            Adt2::C0(h1214, _, _, _, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        let a1216 = 1u64;
        nat_eq(a1215, a1216)
    } {
        true
    } else {
        let a1218 = match v0.clone() {
            Adt2::C0(h1217, _, _, _, _, _, _, _, _, _, _) => {
                h1217
            }
        };
        let a1219 = 25u64;
        if nat_eq(a1218, a1219) {
            true
        } else {
            let a1221 = match v0.clone() {
                Adt2::C0(h1220, _, _, _, _, _, _, _, _, _, _) => {
                    h1220
                }
            };
            let a1222 = 26u64;
            nat_eq(a1221, a1222)
        }
    } {
        true
    } else {
        let a1224 = match v0.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 30u64;
        nat_eq(a1224, a1225)
    } {
        let a1228 = match v0.clone() {
            Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1229 = match v1.clone() {
            Adt2::C0(h1227, _, _, _, _, _, _, _, _, _, _) => {
                h1227
            }
        };
        if if nat_eq(a1228, a1229) {
            f107(match v1.clone() {
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
            }, match v0.clone() {
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
            })
        } else {
            false
        } {
            let m1239 = f106(match v0.clone() {
                Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                    h1238
                }
            });
            match m1239 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f14(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1240) => {
                            h1240
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
    let m1241 = v2;
    match m1241 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1242 = v0;
            let a1243 = 4u64;
            if if if nat_lt(a1242, a1243) {
                let a1246 = match v1.clone() {
                    Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                        h1244
                    }
                };
                let a1247 = match v4.clone() {
                    Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                        h1245
                    }
                };
                if nat_eq(a1246, a1247) {
                    let a1250 = match v1.clone() {
                        Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
                            h1248
                        }
                    };
                    let a1251 = match v3.clone() {
                        Adt2::C0(h1249, _, _, _, _, _, _, _, _, _, _) => {
                            h1249
                        }
                    };
                    nat_eq(a1250, a1251)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f108(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1252, _, _, _, _, _, _, _, _, _) => {
                        h1252
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                        h1253
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1254, _, _, _, _, _, _, _, _, _) => {
                        h1254
                    }
                })? {
                    f108(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1255, _, _, _, _, _, _, _, _) => {
                            h1255
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1256, _, _, _, _, _, _, _, _) => {
                            h1256
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1257, _, _, _, _, _, _, _, _) => {
                            h1257
                        }
                    })?
                } else {
                    false
                } {
                    f108(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1258, _, _, _, _, _, _, _) => {
                            h1258
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1259, _, _, _, _, _, _, _) => {
                            h1259
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1260, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    })?
                } else {
                    false
                } {
                    f108(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1261, _, _, _, _, _, _) => {
                            h1261
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1262, _, _, _, _, _, _) => {
                            h1262
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1263, _, _, _, _, _, _) => {
                            h1263
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1265 = f106(match v1.clone() {
                    Adt2::C0(h1264, _, _, _, _, _, _, _, _, _, _) => {
                        h1264
                    }
                });
                match m1265 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1266, _) => {
                                h1266
                            }
                        };
                        let a1271 = {
                            let a1268 = match v1.clone() {
                                Adt2::C0(_, h1267, _, _, _, _, _, _, _, _, _) => {
                                    h1267
                                }
                            };
                            let a1269 = v6;
                            let a1270 = 1u64;
                            nat_rem(a1268, a1269, a1270)
                        };
                        let a1272 = 0u64;
                        if if if nat_eq(a1271, a1272) {
                            let a1277 = {
                                let a1274 = match v4.clone() {
                                    Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                                        h1273
                                    }
                                };
                                let a1275 = v6;
                                let a1276 = 1u64;
                                nat_rem(a1274, a1275, a1276)
                            };
                            let a1278 = 0u64;
                            nat_eq(a1277, a1278)
                        } else {
                            false
                        } {
                            let a1283 = {
                                let a1280 = match v3.clone() {
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
                            Ok(Adt10::C4(f14(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1285) => {
                                    h1285
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
    let a1287 = match v2.clone() {
        Adt2::C0(h1286, _, _, _, _, _, _, _, _, _, _) => {
            h1286
        }
    };
    let a1288 = 0u64;
    if if nat_eq(a1287, a1288) {
        let a1290 = match v3.clone() {
            Adt2::C0(h1289, _, _, _, _, _, _, _, _, _, _) => {
                h1289
            }
        };
        let a1291 = 0u64;
        nat_eq(a1290, a1291)
    } else {
        false
    } {
        let a1292 = v0;
        let a1293 = 61u64;
        if nat_eq(a1292, a1293) {
            let v4: bool = {
                let a1295 = 0u64;
                let a1296 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1294, _, _, _) => {
                        h1294
                    }
                };
                nat_lt(a1295, a1296)
            };
            if if if if if if f109(match v2.clone() {
                Adt2::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                    h1297
                }
            }, match v1.clone() {
                Adt1::C0(h1298, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1298
                }
            }, match v1.clone() {
                Adt1::C0(_, h1299, _, _, _, _, _, _, _, _, _, _) => {
                    h1299
                }
            }, match v3.clone() {
                Adt2::C0(_, h1300, _, _, _, _, _, _, _, _, _) => {
                    h1300
                }
            })? {
                f109(match v2.clone() {
                    Adt2::C0(_, _, h1301, _, _, _, _, _, _, _, _) => {
                        h1301
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1302, _, _, _, _, _, _, _, _, _) => {
                        h1302
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1303, _, _, _, _, _, _, _, _) => {
                        h1303
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1304, _, _, _, _, _, _, _, _) => {
                        h1304
                    }
                })?
            } else {
                false
            } {
                f109(match v2.clone() {
                    Adt2::C0(_, _, _, h1305, _, _, _, _, _, _, _) => {
                        h1305
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1306, _, _, _, _, _, _, _) => {
                        h1306
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1307, _, _, _, _, _, _) => {
                        h1307
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1308, _, _, _, _, _, _, _) => {
                        h1308
                    }
                })?
            } else {
                false
            } {
                f109(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1309, _, _, _, _, _, _) => {
                        h1309
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1310, _, _, _, _, _) => {
                        h1310
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1311, _, _, _, _) => {
                        h1311
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1312, _, _, _, _, _, _) => {
                        h1312
                    }
                })?
            } else {
                false
            } {
                let a1314 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1313, _, _, _) => {
                        h1313
                    }
                };
                let a1315 = 4294967295u64;
                nat_le(a1314, a1315)
            } else {
                false
            } {
                if v4 {
                    !f31(match v2.clone() {
                        Adt2::C0(_, h1316, _, _, _, _, _, _, _, _, _) => {
                            h1316
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                            h1317
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1318, _, _, _, _, _, _, _) => {
                            h1318
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1319, _, _, _, _, _, _) => {
                            h1319
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1320, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1320
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1321, _, _, _, _, _, _, _, _, _) => {
                        h1321
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1322, _, _, _, _, _, _, _) => {
                        h1322
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1323, _, _, _, _, _) => {
                        h1323
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1324 = v0;
            let a1325 = 62u64;
            if nat_eq(a1324, a1325) {
                let a1327 = match v1.clone() {
                    Adt1::C0(h1326, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1326
                    }
                };
                let a1328 = 2147483648u64;
                if if if if if nat_lt(a1327, a1328) {
                    let a1330 = match v1.clone() {
                        Adt1::C0(_, h1329, _, _, _, _, _, _, _, _, _, _) => {
                            h1329
                        }
                    };
                    let a1331 = 2147483648u64;
                    nat_lt(a1330, a1331)
                } else {
                    false
                } {
                    let a1334 = match v1.clone() {
                        Adt1::C0(h1332, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1332
                        }
                    };
                    let a1335 = match v2.clone() {
                        Adt2::C0(_, h1333, _, _, _, _, _, _, _, _, _) => {
                            h1333
                        }
                    };
                    nat_lt(a1334, a1335)
                } else {
                    false
                } {
                    let a1338 = match v1.clone() {
                        Adt1::C0(_, h1336, _, _, _, _, _, _, _, _, _, _) => {
                            h1336
                        }
                    };
                    let a1339 = match v2.clone() {
                        Adt2::C0(_, h1337, _, _, _, _, _, _, _, _, _) => {
                            h1337
                        }
                    };
                    nat_lt(a1338, a1339)
                } else {
                    false
                } {
                    let a1342 = match v2.clone() {
                        Adt2::C0(_, _, h1340, _, _, _, _, _, _, _, _) => {
                            h1340
                        }
                    };
                    let a1343 = match v3.clone() {
                        Adt2::C0(_, _, h1341, _, _, _, _, _, _, _, _) => {
                            h1341
                        }
                    };
                    if if if nat_eq(a1342, a1343) {
                        let a1346 = match v2.clone() {
                            Adt2::C0(_, _, _, h1344, _, _, _, _, _, _, _) => {
                                h1344
                            }
                        };
                        let a1347 = match v3.clone() {
                            Adt2::C0(_, _, _, h1345, _, _, _, _, _, _, _) => {
                                h1345
                            }
                        };
                        nat_eq(a1346, a1347)
                    } else {
                        false
                    } {
                        let a1350 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1348, _, _, _, _, _, _) => {
                                h1348
                            }
                        };
                        let a1351 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1349, _, _, _, _, _, _) => {
                                h1349
                            }
                        };
                        nat_eq(a1350, a1351)
                    } else {
                        false
                    } {
                        let a1360 = match v3.clone() {
                            Adt2::C0(_, h1352, _, _, _, _, _, _, _, _, _) => {
                                h1352
                            }
                        };
                        let a1361 = {
                            let a1358 = {
                                let a1355 = match v2.clone() {
                                    Adt2::C0(_, h1353, _, _, _, _, _, _, _, _, _) => {
                                        h1353
                                    }
                                };
                                let a1356 = match v1.clone() {
                                    Adt1::C0(h1354, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1354
                                    }
                                };
                                nat_add(a1355, a1356)?
                            };
                            let a1359 = match v1.clone() {
                                Adt1::C0(_, h1357, _, _, _, _, _, _, _, _, _, _) => {
                                    h1357
                                }
                            };
                            nat_add(a1358, a1359)?
                        };
                        nat_eq(a1360, a1361)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1362, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1362
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1363 = v0;
                let a1364 = 63u64;
                if nat_eq(a1363, a1364) {
                    if if if if if f110(match v2.clone() {
                        Adt2::C0(_, h1365, _, _, _, _, _, _, _, _, _) => {
                            h1365
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1366, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1366
                        }
                    })? {
                        f110(match v2.clone() {
                            Adt2::C0(_, _, h1367, _, _, _, _, _, _, _, _) => {
                                h1367
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1368, _, _, _, _, _, _, _, _, _, _) => {
                                h1368
                            }
                        })?
                    } else {
                        false
                    } {
                        f110(match v2.clone() {
                            Adt2::C0(_, _, _, h1369, _, _, _, _, _, _, _) => {
                                h1369
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1370, _, _, _, _, _, _, _, _, _) => {
                                h1370
                            }
                        })?
                    } else {
                        false
                    } {
                        f110(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1371, _, _, _, _, _, _) => {
                                h1371
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1372, _, _, _, _, _, _, _, _) => {
                                h1372
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1375 = match v2.clone() {
                            Adt2::C0(_, h1373, _, _, _, _, _, _, _, _, _) => {
                                h1373
                            }
                        };
                        let a1376 = match v3.clone() {
                            Adt2::C0(_, h1374, _, _, _, _, _, _, _, _, _) => {
                                h1374
                            }
                        };
                        if if if nat_eq(a1375, a1376) {
                            let a1379 = match v2.clone() {
                                Adt2::C0(_, _, h1377, _, _, _, _, _, _, _, _) => {
                                    h1377
                                }
                            };
                            let a1380 = match v3.clone() {
                                Adt2::C0(_, _, h1378, _, _, _, _, _, _, _, _) => {
                                    h1378
                                }
                            };
                            nat_eq(a1379, a1380)
                        } else {
                            false
                        } {
                            let a1383 = match v2.clone() {
                                Adt2::C0(_, _, _, h1381, _, _, _, _, _, _, _) => {
                                    h1381
                                }
                            };
                            let a1384 = match v3.clone() {
                                Adt2::C0(_, _, _, h1382, _, _, _, _, _, _, _) => {
                                    h1382
                                }
                            };
                            nat_eq(a1383, a1384)
                        } else {
                            false
                        } {
                            let a1387 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1385, _, _, _, _, _, _) => {
                                    h1385
                                }
                            };
                            let a1388 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1386, _, _, _, _, _, _) => {
                                    h1386
                                }
                            };
                            nat_eq(a1387, a1388)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, 4u64, f18(v2, 1u64), f18(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1389, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1389
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1390, _, _, _, _, _, _, _, _, _, _) => {
                                h1390
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1391, _, _, _, _, _, _, _, _, _) => {
                                h1391
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1392, _, _, _, _, _, _, _, _) => {
                                h1392
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
    let m1393 = v3;
    match m1393 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1394 = v0;
                let a1395 = 4u64;
                nat_eq(a1394, a1395)
            };
            let v7: bool = {
                let a1397 = 0u64;
                let a1398 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1396, _, _, _, _, _, _, _) => {
                        h1396
                    }
                };
                nat_lt(a1397, a1398)
            };
            let v8: u64 = f14(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1399, _, _, _, _, _, _, _, _, _) => {
                    h1399
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1400, _, _, _, _, _, _, _, _) => {
                    h1400
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1401, _, _, _, _, _, _, _) => {
                    h1401
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1402, _, _, _, _, _, _) => {
                    h1402
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1403, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1403
                }
            }, match v1.clone() {
                Adt1::C0(_, h1404, _, _, _, _, _, _, _, _, _, _) => {
                    h1404
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1405, _, _, _, _, _, _, _, _, _) => {
                    h1405
                }
            });
            if if if if if if if if if if if if v6 {
                let a1407 = match v2.clone() {
                    Adt2::C0(h1406, _, _, _, _, _, _, _, _, _, _) => {
                        h1406
                    }
                };
                let a1408 = 0u64;
                if nat_eq(a1407, a1408) {
                    let a1410 = match v5.clone() {
                        Adt2::C0(h1409, _, _, _, _, _, _, _, _, _, _) => {
                            h1409
                        }
                    };
                    let a1411 = 0u64;
                    nat_eq(a1410, a1411)
                } else {
                    false
                }
            } else {
                let a1413 = match v2.clone() {
                    Adt2::C0(h1412, _, _, _, _, _, _, _, _, _, _) => {
                        h1412
                    }
                };
                let a1414 = 0u64;
                if if nat_eq(a1413, a1414) {
                    true
                } else {
                    let a1416 = match v2.clone() {
                        Adt2::C0(h1415, _, _, _, _, _, _, _, _, _, _) => {
                            h1415
                        }
                    };
                    let a1417 = 26u64;
                    nat_eq(a1416, a1417)
                } {
                    let a1419 = match v5.clone() {
                        Adt2::C0(h1418, _, _, _, _, _, _, _, _, _, _) => {
                            h1418
                        }
                    };
                    let a1420 = 0u64;
                    if nat_eq(a1419, a1420) {
                        true
                    } else {
                        let a1422 = match v5.clone() {
                            Adt2::C0(h1421, _, _, _, _, _, _, _, _, _, _) => {
                                h1421
                            }
                        };
                        let a1423 = 26u64;
                        nat_eq(a1422, a1423)
                    }
                } else {
                    false
                }
            } {
                let a1426 = match v2.clone() {
                    Adt2::C0(h1424, _, _, _, _, _, _, _, _, _, _) => {
                        h1424
                    }
                };
                let a1427 = match v4.clone() {
                    Adt2::C0(h1425, _, _, _, _, _, _, _, _, _, _) => {
                        h1425
                    }
                };
                nat_eq(a1426, a1427)
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
                let a1429 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1428, _, _, _, _, _) => {
                        h1428
                    }
                };
                let a1430 = 4u64;
                nat_eq(a1429, a1430)
            } else {
                false
            } {
                let a1431 = v8;
                let a1432 = f14(v2.clone(), 1u64)?;
                nat_le(a1431, a1432)
            } else {
                false
            } {
                let a1434 = match v1.clone() {
                    Adt1::C0(_, _, _, h1433, _, _, _, _, _, _, _, _) => {
                        h1433
                    }
                };
                let a1435 = 1073741824u64;
                nat_lt(a1434, a1435)
            } else {
                false
            } {
                let a1437 = match v1.clone() {
                    Adt1::C0(h1436, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1436
                    }
                };
                let a1438 = 4294967295u64;
                if if if if nat_le(a1437, a1438) {
                    let a1440 = match v1.clone() {
                        Adt1::C0(_, h1439, _, _, _, _, _, _, _, _, _, _) => {
                            h1439
                        }
                    };
                    let a1441 = 4294967295u64;
                    nat_le(a1440, a1441)
                } else {
                    false
                } {
                    let a1443 = match v1.clone() {
                        Adt1::C0(_, _, h1442, _, _, _, _, _, _, _, _, _) => {
                            h1442
                        }
                    };
                    let a1444 = 4294967295u64;
                    nat_le(a1443, a1444)
                } else {
                    false
                } {
                    let a1446 = match v1.clone() {
                        Adt1::C0(_, _, _, h1445, _, _, _, _, _, _, _, _) => {
                            h1445
                        }
                    };
                    let a1447 = 4294967295u64;
                    nat_le(a1446, a1447)
                } else {
                    false
                } {
                    let a1449 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1448, _, _, _, _, _, _, _) => {
                            h1448
                        }
                    };
                    let a1450 = 4294967295u64;
                    nat_le(a1449, a1450)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1453 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1451) => {
                            h1451
                        }
                    };
                    let a1454 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1452) => {
                            h1452
                        }
                    };
                    nat_eq(a1453, a1454)
                } else {
                    true
                }
            } else {
                false
            } {
                f111(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1455, _, _, _, _, _, _, _, _) => {
                        h1455
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1456, _) => {
                        h1456
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
                    let a1457 = v10;
                    let a1458 = v8;
                    nat_add(a1457, a1458)?
                }, 4u64, f18(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1459, _, _, _, _, _, _, _, _) => {
                        h1459
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f52(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1461 = match v0.clone() {
        Adt2::C0(h1460, _, _, _, _, _, _, _, _, _, _) => {
            h1460
        }
    };
    let a1462 = 0u64;
    if if if nat_eq(a1461, a1462) {
        let a1464 = match v1.clone() {
            Adt2::C0(h1463, _, _, _, _, _, _, _, _, _, _) => {
                h1463
            }
        };
        let a1465 = 0u64;
        nat_eq(a1464, a1465)
    } else {
        false
    } {
        f107(match v0.clone() {
            Adt2::C0(_, h1466, _, _, _, _, _, _, _, _, _) => {
                h1466
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1467, _, _, _, _, _, _, _, _) => {
                h1467
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1468, _, _, _, _, _, _, _) => {
                h1468
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1469, _, _, _, _, _, _) => {
                h1469
            }
        }, match v1.clone() {
            Adt2::C0(_, h1470, _, _, _, _, _, _, _, _, _) => {
                h1470
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1471, _, _, _, _, _, _, _, _) => {
                h1471
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1472, _, _, _, _, _, _, _) => {
                h1472
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1473, _, _, _, _, _, _) => {
                h1473
            }
        })
    } else {
        false
    } {
        let v2: u64 = f14(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1474 = v2;
            let a1475 = f14(v0.clone(), 1u64)?;
            nat_add(a1474, a1475)?
        }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1477 = match v1.clone() {
        Adt2::C0(h1476, _, _, _, _, _, _, _, _, _, _) => {
            h1476
        }
    };
    let a1478 = 0u64;
    if if if nat_eq(a1477, a1478) {
        true
    } else {
        let a1480 = match v1.clone() {
            Adt2::C0(h1479, _, _, _, _, _, _, _, _, _, _) => {
                h1479
            }
        };
        let a1481 = 1u64;
        nat_eq(a1480, a1481)
    } {
        true
    } else {
        let a1483 = match v1.clone() {
            Adt2::C0(h1482, _, _, _, _, _, _, _, _, _, _) => {
                h1482
            }
        };
        let a1484 = 30u64;
        nat_eq(a1483, a1484)
    } {
        let a1487 = match v1.clone() {
            Adt2::C0(h1485, _, _, _, _, _, _, _, _, _, _) => {
                h1485
            }
        };
        let a1488 = match v2.clone() {
            Adt2::C0(h1486, _, _, _, _, _, _, _, _, _, _) => {
                h1486
            }
        };
        if if if if if nat_eq(a1487, a1488) {
            let a1491 = match v1.clone() {
                Adt2::C0(_, h1489, _, _, _, _, _, _, _, _, _) => {
                    h1489
                }
            };
            let a1492 = match v2.clone() {
                Adt2::C0(_, h1490, _, _, _, _, _, _, _, _, _) => {
                    h1490
                }
            };
            nat_eq(a1491, a1492)
        } else {
            false
        } {
            let a1495 = match v1.clone() {
                Adt2::C0(_, _, _, h1493, _, _, _, _, _, _, _) => {
                    h1493
                }
            };
            let a1496 = match v2.clone() {
                Adt2::C0(_, _, _, h1494, _, _, _, _, _, _, _) => {
                    h1494
                }
            };
            nat_eq(a1495, a1496)
        } else {
            false
        } {
            let a1499 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1497, _, _, _, _, _, _) => {
                    h1497
                }
            };
            let a1500 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1498, _, _, _, _, _, _) => {
                    h1498
                }
            };
            nat_eq(a1499, a1500)
        } else {
            false
        } {
            let a1501 = v0;
            let a1502 = 4u64;
            if nat_eq(a1501, a1502) {
                let a1504 = match v1.clone() {
                    Adt2::C0(_, _, h1503, _, _, _, _, _, _, _, _) => {
                        h1503
                    }
                };
                let a1505 = 1u64;
                if nat_eq(a1504, a1505) {
                    let a1508 = match v1.clone() {
                        Adt2::C0(_, h1506, _, _, _, _, _, _, _, _, _) => {
                            h1506
                        }
                    };
                    let a1509 = match v2.clone() {
                        Adt2::C0(_, _, h1507, _, _, _, _, _, _, _, _) => {
                            h1507
                        }
                    };
                    nat_eq(a1508, a1509)
                } else {
                    false
                }
            } else {
                let a1510 = v0;
                let a1511 = 4u64;
                if if nat_lt(a1510, a1511) {
                    let a1514 = match v1.clone() {
                        Adt2::C0(_, h1512, _, _, _, _, _, _, _, _, _) => {
                            h1512
                        }
                    };
                    let a1515 = match v1.clone() {
                        Adt2::C0(_, _, h1513, _, _, _, _, _, _, _, _) => {
                            h1513
                        }
                    };
                    nat_eq(a1514, a1515)
                } else {
                    false
                } {
                    let a1518 = match v1.clone() {
                        Adt2::C0(_, _, h1516, _, _, _, _, _, _, _, _) => {
                            h1516
                        }
                    };
                    let a1519 = match v2.clone() {
                        Adt2::C0(_, _, h1517, _, _, _, _, _, _, _, _) => {
                            h1517
                        }
                    };
                    nat_eq(a1518, a1519)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1521 = f106(match v1.clone() {
                Adt2::C0(h1520, _, _, _, _, _, _, _, _, _, _) => {
                    h1520
                }
            });
            match m1521 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1522) => {
                            h1522
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
    if if if f112(match v1.clone() {
        Adt2::C0(h1523, _, _, _, _, _, _, _, _, _, _) => {
            h1523
        }
    }, v0) {
        let a1526 = match v1.clone() {
            Adt2::C0(h1524, _, _, _, _, _, _, _, _, _, _) => {
                h1524
            }
        };
        let a1527 = match v2.clone() {
            Adt2::C0(h1525, _, _, _, _, _, _, _, _, _, _) => {
                h1525
            }
        };
        nat_eq(a1526, a1527)
    } else {
        false
    } {
        let a1530 = match v1.clone() {
            Adt2::C0(_, h1528, _, _, _, _, _, _, _, _, _) => {
                h1528
            }
        };
        let a1531 = match v2.clone() {
            Adt2::C0(_, h1529, _, _, _, _, _, _, _, _, _) => {
                h1529
            }
        };
        if if if nat_eq(a1530, a1531) {
            let a1534 = match v1.clone() {
                Adt2::C0(_, _, h1532, _, _, _, _, _, _, _, _) => {
                    h1532
                }
            };
            let a1535 = match v2.clone() {
                Adt2::C0(_, _, h1533, _, _, _, _, _, _, _, _) => {
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
        }
    } else {
        false
    } {
        let m1545 = f106(match v1.clone() {
            Adt2::C0(h1544, _, _, _, _, _, _, _, _, _, _) => {
                h1544
            }
        });
        match m1545 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1546) => {
                        h1546
                    }
                }, f18(v1.clone(), 1u64), f18(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1547, _, _, _, _, _, _, _, _, _, _) => {
                        h1547
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
            Adt1::C0(h1548, _, _, _, _, _, _, _, _, _, _, _) => {
                h1548
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1549, _, _, _, _, _, _, _, _, _) => {
                h1549
            }
        }
    };
    let a1550 = 0u64;
    let a1551 = v4;
    if if nat_lt(a1550, a1551) {
        let a1552 = v4;
        let a1553 = 2147483648u64;
        nat_lt(a1552, a1553)
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
        let v7: u64 = f113(match v5.clone() {
            Adt2::C0(_, _, h1554, _, _, _, _, _, _, _, _) => {
                h1554
            }
        }, v4)?;
        let v8: u64 = f113(match v5.clone() {
            Adt2::C0(_, _, _, h1555, _, _, _, _, _, _, _) => {
                h1555
            }
        }, v4)?;
        let a1557 = match v5.clone() {
            Adt2::C0(h1556, _, _, _, _, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1558 = 0u64;
        if if if if if nat_eq(a1557, a1558) {
            let a1560 = match v6.clone() {
                Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                    h1559
                }
            };
            let a1561 = 0u64;
            nat_eq(a1560, a1561)
        } else {
            false
        } {
            let a1563 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1562, _, _, _, _, _, _) => {
                    h1562
                }
            };
            let a1564 = 1u64;
            nat_eq(a1563, a1564)
        } else {
            false
        } {
            let a1567 = match v5.clone() {
                Adt2::C0(_, h1565, _, _, _, _, _, _, _, _, _) => {
                    h1565
                }
            };
            let a1568 = match v6.clone() {
                Adt2::C0(_, h1566, _, _, _, _, _, _, _, _, _) => {
                    h1566
                }
            };
            nat_eq(a1567, a1568)
        } else {
            false
        } {
            if v0 {
                let a1570 = match v5.clone() {
                    Adt2::C0(_, _, h1569, _, _, _, _, _, _, _, _) => {
                        h1569
                    }
                };
                let a1571 = 2147483648u64;
                if if if nat_lt(a1570, a1571) {
                    let a1573 = match v5.clone() {
                        Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
                            h1572
                        }
                    };
                    let a1574 = 2147483648u64;
                    nat_lt(a1573, a1574)
                } else {
                    false
                } {
                    let a1577 = {
                        let a1575 = v7;
                        let a1576 = v8;
                        nat_mul(a1575, a1576)?
                    };
                    let a1578 = 2147483648u64;
                    nat_le(a1577, a1578)
                } else {
                    false
                } {
                    f114(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1580 = match v1.clone() {
                    Adt1::C0(h1579, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1579
                    }
                };
                let a1581 = v7;
                if if if if nat_eq(a1580, a1581) {
                    let a1583 = match v1 {
                        Adt1::C0(_, h1582, _, _, _, _, _, _, _, _, _, _) => {
                            h1582
                        }
                    };
                    let a1584 = v8;
                    nat_eq(a1583, a1584)
                } else {
                    false
                } {
                    f115(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1586 = match v6.clone() {
                        Adt2::C0(_, _, h1585, _, _, _, _, _, _, _, _) => {
                            h1585
                        }
                    };
                    let a1587 = v4;
                    if nat_eq(a1586, a1587) {
                        let a1589 = match v6.clone() {
                            Adt2::C0(_, _, _, h1588, _, _, _, _, _, _, _) => {
                                h1588
                            }
                        };
                        let a1590 = v4;
                        nat_eq(a1589, a1590)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1594 = match v6 {
                        Adt2::C0(_, _, _, _, h1591, _, _, _, _, _, _) => {
                            h1591
                        }
                    };
                    let a1595 = {
                        let a1592 = v7;
                        let a1593 = v8;
                        nat_mul(a1592, a1593)?
                    };
                    nat_eq(a1594, a1595)
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
    let a1597 = match v2.clone() {
        Adt2::C0(h1596, _, _, _, _, _, _, _, _, _, _) => {
            h1596
        }
    };
    let a1598 = 0u64;
    if if if if if nat_eq(a1597, a1598) {
        true
    } else {
        let a1600 = match v2.clone() {
            Adt2::C0(h1599, _, _, _, _, _, _, _, _, _, _) => {
                h1599
            }
        };
        let a1601 = 1u64;
        nat_eq(a1600, a1601)
    } {
        true
    } else {
        let a1603 = match v2.clone() {
            Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
                h1602
            }
        };
        let a1604 = 30u64;
        nat_eq(a1603, a1604)
    } {
        let a1607 = match v2.clone() {
            Adt2::C0(h1605, _, _, _, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1608 = match v3.clone() {
            Adt2::C0(h1606, _, _, _, _, _, _, _, _, _, _) => {
                h1606
            }
        };
        nat_eq(a1607, a1608)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a1610 = match v1.clone() {
                Adt1::C0(h1609, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1609
                }
            };
            let a1611 = 4294967296u64;
            nat_lt(a1610, a1611)
        } else {
            false
        } {
            let a1613 = match v1.clone() {
                Adt1::C0(_, h1612, _, _, _, _, _, _, _, _, _, _) => {
                    h1612
                }
            };
            let a1614 = 4294967296u64;
            nat_lt(a1613, a1614)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v3.clone(), 1u64)?, {
            let a1616 = match v2.clone() {
                Adt2::C0(h1615, _, _, _, _, _, _, _, _, _, _) => {
                    h1615
                }
            };
            let a1617 = 0u64;
            if nat_eq(a1616, a1617) {
                4u64
            } else {
                2u64
            }
        }, f18(v2.clone(), 1u64), f18(v3, 1u64), Adt4::C13({
            let a1618 = v0;
            let a1619 = 31u64;
            if nat_eq(a1618, a1619) {
                0u64
            } else {
                let a1620 = v0;
                let a1621 = 47u64;
                if nat_eq(a1620, a1621) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1622, _, _, _, _, _, _, _, _, _, _) => {
                h1622
            }
        }, match v1.clone() {
            Adt1::C0(h1623, _, _, _, _, _, _, _, _, _, _, _) => {
                h1623
            }
        }, match v1 {
            Adt1::C0(_, h1624, _, _, _, _, _, _, _, _, _, _) => {
                h1624
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f57(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1626 = match v0.clone() {
        Adt2::C0(h1625, _, _, _, _, _, _, _, _, _, _) => {
            h1625
        }
    };
    let a1627 = 0u64;
    if if if nat_eq(a1626, a1627) {
        let a1629 = match v1.clone() {
            Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
                h1628
            }
        };
        let a1630 = 0u64;
        nat_eq(a1629, a1630)
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
    let a1632 = match v0.clone() {
        Adt2::C0(h1631, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    };
    let a1633 = 0u64;
    if if nat_eq(a1632, a1633) {
        let a1635 = match v1.clone() {
            Adt2::C0(h1634, _, _, _, _, _, _, _, _, _, _) => {
                h1634
            }
        };
        let a1636 = 26u64;
        nat_eq(a1635, a1636)
    } else {
        false
    } {
        let a1638 = match v0.clone() {
            Adt2::C0(_, _, _, h1637, _, _, _, _, _, _, _) => {
                h1637
            }
        };
        let a1639 = 1u64;
        if if if if if if if nat_eq(a1638, a1639) {
            let a1641 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1640, _, _, _, _, _, _) => {
                    h1640
                }
            };
            let a1642 = 1u64;
            nat_eq(a1641, a1642)
        } else {
            false
        } {
            let a1644 = match v0.clone() {
                Adt2::C0(_, h1643, _, _, _, _, _, _, _, _, _) => {
                    h1643
                }
            };
            let a1645 = 2147483647u64;
            nat_le(a1644, a1645)
        } else {
            false
        } {
            let a1648 = match v1.clone() {
                Adt2::C0(_, h1646, _, _, _, _, _, _, _, _, _) => {
                    h1646
                }
            };
            let a1649 = match v0.clone() {
                Adt2::C0(_, _, h1647, _, _, _, _, _, _, _, _) => {
                    h1647
                }
            };
            nat_eq(a1648, a1649)
        } else {
            false
        } {
            let a1651 = match v1.clone() {
                Adt2::C0(_, _, h1650, _, _, _, _, _, _, _, _) => {
                    h1650
                }
            };
            let a1652 = 1u64;
            nat_eq(a1651, a1652)
        } else {
            false
        } {
            let a1654 = match v1.clone() {
                Adt2::C0(_, _, _, h1653, _, _, _, _, _, _, _) => {
                    h1653
                }
            };
            let a1655 = 1u64;
            nat_eq(a1654, a1655)
        } else {
            false
        } {
            let a1657 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1656, _, _, _, _, _, _) => {
                    h1656
                }
            };
            let a1658 = 1u64;
            nat_eq(a1657, a1658)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1661 = match v0.clone() {
                    Adt2::C0(_, _, h1659, _, _, _, _, _, _, _, _) => {
                        h1659
                    }
                };
                let a1662 = f116(match v0.clone() {
                    Adt2::C0(_, h1660, _, _, _, _, _, _, _, _, _) => {
                        h1660
                    }
                })?;
                nat_mul(a1661, a1662)?
            }, 4u64, f18(v0, 1u64), f18(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f59(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1663 = v1;
    match m1663 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f117(f18(v0.clone(), 1u64))?;
            let a1665 = match v0.clone() {
                Adt2::C0(h1664, _, _, _, _, _, _, _, _, _, _) => {
                    h1664
                }
            };
            let a1666 = 26u64;
            if if if nat_eq(a1665, a1666) {
                let a1668 = match v3.clone() {
                    Adt2::C0(h1667, _, _, _, _, _, _, _, _, _, _) => {
                        h1667
                    }
                };
                let a1669 = 26u64;
                nat_eq(a1668, a1669)
            } else {
                false
            } {
                let a1671 = match v2.clone() {
                    Adt2::C0(h1670, _, _, _, _, _, _, _, _, _, _) => {
                        h1670
                    }
                };
                let a1672 = 27u64;
                nat_eq(a1671, a1672)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a1674 = match v2.clone() {
                        Adt2::C0(_, h1673, _, _, _, _, _, _, _, _, _) => {
                            h1673
                        }
                    };
                    let a1675 = 1u64;
                    nat_eq(a1674, a1675)
                } else {
                    false
                } {
                    let a1677 = match v2.clone() {
                        Adt2::C0(_, _, h1676, _, _, _, _, _, _, _, _) => {
                            h1676
                        }
                    };
                    let a1678 = 1u64;
                    nat_eq(a1677, a1678)
                } else {
                    false
                } {
                    let a1680 = match v2.clone() {
                        Adt2::C0(_, _, _, h1679, _, _, _, _, _, _, _) => {
                            h1679
                        }
                    };
                    let a1681 = 1u64;
                    nat_eq(a1680, a1681)
                } else {
                    false
                } {
                    let a1683 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1682, _, _, _, _, _, _) => {
                            h1682
                        }
                    };
                    let a1684 = 1u64;
                    nat_eq(a1683, a1684)
                } else {
                    false
                } {
                    let a1685 = v4;
                    let a1686 = 9223372036854775808u64;
                    nat_lt(a1685, a1686)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1689 = 2u64;
                        let a1690 = {
                            let a1687 = v4;
                            let a1688 = 2u64;
                            nat_mul(a1687, a1688)?
                        };
                        nat_add(a1689, a1690)?
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
        Adt2::C0(h1691, _, _, _, _, _, _, _, _, _, _) => {
            h1691
        }
    };
    let a1692 = v3;
    let a1693 = 0u64;
    if if nat_eq(a1692, a1693) {
        true
    } else {
        let a1694 = v0;
        let a1695 = 13u64;
        if nat_eq(a1694, a1695) {
            let a1696 = v3;
            let a1697 = 1u64;
            if nat_eq(a1696, a1697) {
                true
            } else {
                let a1698 = v3;
                let a1699 = 30u64;
                nat_eq(a1698, a1699)
            }
        } else {
            false
        }
    } {
        let a1701 = v3;
        let a1702 = match v2.clone() {
            Adt2::C0(h1700, _, _, _, _, _, _, _, _, _, _) => {
                h1700
            }
        };
        if if nat_eq(a1701, a1702) {
            let a1703 = v0;
            let a1704 = 13u64;
            if nat_eq(a1703, a1704) {
                let a1706 = match v2.clone() {
                    Adt2::C0(_, h1705, _, _, _, _, _, _, _, _, _) => {
                        h1705
                    }
                };
                let a1707 = 1u64;
                if if if nat_eq(a1706, a1707) {
                    let a1709 = match v2.clone() {
                        Adt2::C0(_, _, h1708, _, _, _, _, _, _, _, _) => {
                            h1708
                        }
                    };
                    let a1710 = 1u64;
                    nat_eq(a1709, a1710)
                } else {
                    false
                } {
                    let a1712 = match v2.clone() {
                        Adt2::C0(_, _, _, h1711, _, _, _, _, _, _, _) => {
                            h1711
                        }
                    };
                    let a1713 = 1u64;
                    nat_eq(a1712, a1713)
                } else {
                    false
                } {
                    let a1715 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1714, _, _, _, _, _, _) => {
                            h1714
                        }
                    };
                    let a1716 = 1u64;
                    nat_eq(a1715, a1716)
                } else {
                    false
                }
            } else {
                let a1718 = match v2.clone() {
                    Adt2::C0(_, h1717, _, _, _, _, _, _, _, _, _) => {
                        h1717
                    }
                };
                let a1719 = 1u64;
                if if if nat_eq(a1718, a1719) {
                    let a1722 = match v1.clone() {
                        Adt2::C0(_, _, h1720, _, _, _, _, _, _, _, _) => {
                            h1720
                        }
                    };
                    let a1723 = match v2.clone() {
                        Adt2::C0(_, _, h1721, _, _, _, _, _, _, _, _) => {
                            h1721
                        }
                    };
                    nat_eq(a1722, a1723)
                } else {
                    false
                } {
                    let a1726 = match v1.clone() {
                        Adt2::C0(_, _, _, h1724, _, _, _, _, _, _, _) => {
                            h1724
                        }
                    };
                    let a1727 = match v2.clone() {
                        Adt2::C0(_, _, _, h1725, _, _, _, _, _, _, _) => {
                            h1725
                        }
                    };
                    nat_eq(a1726, a1727)
                } else {
                    false
                } {
                    let a1730 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1728, _, _, _, _, _, _) => {
                            h1728
                        }
                    };
                    let a1731 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1729, _, _, _, _, _, _) => {
                            h1729
                        }
                    };
                    nat_eq(a1730, a1731)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f18(v1, 1u64);
            Ok(Adt10::C4(f118(v4.clone(), v0, v3)?, {
                let a1732 = v3;
                let a1733 = 0u64;
                if nat_eq(a1732, a1733) {
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
    let a1734 = v0;
    let a1735 = 89u64;
    if if nat_eq(a1734, a1735) {
        let a1736 = 8u64;
        let a1737 = v1;
        if nat_le(a1736, a1737) {
            let a1738 = v1;
            let a1739 = 9u64;
            nat_le(a1738, a1739)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1740 = v1;
            let a1741 = 12u64;
            nat_add(a1740, a1741)?
        }))
    } else {
        let a1742 = v0;
        let a1743 = 89u64;
        if if nat_eq(a1742, a1743) {
            let a1744 = v1;
            let a1745 = 4u64;
            nat_eq(a1744, a1745)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1746 = 10u64;
            let a1747 = v0;
            if if nat_le(a1746, a1747) {
                let a1748 = v0;
                let a1749 = 12u64;
                nat_le(a1748, a1749)
            } else {
                false
            } {
                Ok(Some({
                    let a1750 = v0;
                    let a1751 = 3u64;
                    nat_add(a1750, a1751)?
                }))
            } else {
                let a1752 = v0;
                let a1753 = 89u64;
                if if nat_eq(a1752, a1753) {
                    let a1754 = v1;
                    let a1755 = 7u64;
                    nat_eq(a1754, a1755)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1756 = v0;
                    let a1757 = 89u64;
                    if if nat_eq(a1756, a1757) {
                        let a1758 = v1;
                        let a1759 = 10u64;
                        nat_eq(a1758, a1759)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1760 = v0;
                        let a1761 = 89u64;
                        if if nat_eq(a1760, a1761) {
                            let a1762 = v1;
                            let a1763 = 13u64;
                            nat_eq(a1762, a1763)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1764 = v0;
                            let a1765 = 8u64;
                            if nat_eq(a1764, a1765) {
                                Ok(Some(5u64))
                            } else {
                                let a1766 = v0;
                                let a1767 = 9u64;
                                if nat_eq(a1766, a1767) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1768 = v0;
                                    let a1769 = 89u64;
                                    if nat_eq(a1768, a1769) {
                                        let a1770 = v1;
                                        let a1771 = 0u64;
                                        if nat_eq(a1770, a1771) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1772 = v1;
                                            let a1773 = 1u64;
                                            if nat_eq(a1772, a1773) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1774 = v1;
                                                let a1775 = 2u64;
                                                if nat_eq(a1774, a1775) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1776 = v1;
                                                    let a1777 = 3u64;
                                                    if nat_eq(a1776, a1777) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1778 = v1;
                                                        let a1779 = 6u64;
                                                        if nat_eq(a1778, a1779) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1780 = v1;
                                                            let a1781 = 12u64;
                                                            if nat_eq(a1780, a1781) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1782 = v1;
                                                                let a1783 = 11u64;
                                                                if nat_eq(a1782, a1783) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1784 = v1;
                                                                    let a1785 = 18u64;
                                                                    if nat_eq(a1784, a1785) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1786 = v1;
                                                                        let a1787 = 19u64;
                                                                        if nat_eq(a1786, a1787) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1788 = v1;
                                                                            let a1789 = 20u64;
                                                                            if nat_eq(a1788, a1789) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1790 = v1;
                                                                                let a1791 = 21u64;
                                                                                if nat_eq(a1790, a1791) {
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

pub fn f62(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1792 = f119(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1792 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1793 = v0;
            let a1794 = 2u64;
            if if if if nat_eq(a1793, a1794) {
                true
            } else {
                let a1795 = v0;
                let a1796 = 5u64;
                nat_eq(a1795, a1796)
            } {
                true
            } else {
                let a1797 = v0;
                let a1798 = 6u64;
                nat_eq(a1797, a1798)
            } {
                true
            } else {
                let a1799 = v0;
                let a1800 = 7u64;
                nat_eq(a1799, a1800)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f18(v2.clone(), 1u64), f18(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1801 = v3;
                    match m1801 {
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
    let a1803 = match v1.clone() {
        Adt2::C0(h1802, _, _, _, _, _, _, _, _, _, _) => {
            h1802
        }
    };
    let a1804 = 0u64;
    if if if if if nat_eq(a1803, a1804) {
        true
    } else {
        let a1806 = match v1.clone() {
            Adt2::C0(h1805, _, _, _, _, _, _, _, _, _, _) => {
                h1805
            }
        };
        let a1807 = 1u64;
        nat_eq(a1806, a1807)
    } {
        true
    } else {
        let a1809 = match v1.clone() {
            Adt2::C0(h1808, _, _, _, _, _, _, _, _, _, _) => {
                h1808
            }
        };
        let a1810 = 30u64;
        nat_eq(a1809, a1810)
    } {
        let a1813 = match v1.clone() {
            Adt2::C0(h1811, _, _, _, _, _, _, _, _, _, _) => {
                h1811
            }
        };
        let a1814 = match v2.clone() {
            Adt2::C0(h1812, _, _, _, _, _, _, _, _, _, _) => {
                h1812
            }
        };
        if nat_eq(a1813, a1814) {
            true
        } else {
            let a1816 = match v2.clone() {
                Adt2::C0(h1815, _, _, _, _, _, _, _, _, _, _) => {
                    h1815
                }
            };
            let a1817 = 0u64;
            nat_eq(a1816, a1817)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f14(v2.clone(), 1u64)?, {
            let a1819 = match v2.clone() {
                Adt2::C0(h1818, _, _, _, _, _, _, _, _, _, _) => {
                    h1818
                }
            };
            let a1820 = 0u64;
            if nat_eq(a1819, a1820) {
                4u64
            } else {
                2u64
            }
        }, f18(v1.clone(), 1u64), f18(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1821, _, _, _, _, _, _, _, _, _, _) => {
                h1821
            }
        }, match v2 {
            Adt2::C0(h1822, _, _, _, _, _, _, _, _, _, _) => {
                h1822
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f64(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f120(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1823, _, _, _) => {
            h1823
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1824, _, _) => {
            h1824
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1825, _) => {
            h1825
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1826) => {
            h1826
        }
    })
}

pub fn f65(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1828 = v0;
        let a1829 = match v1.clone() {
            Adt3::C0(h1827, _, _, _, _, _, _, _) => {
                h1827
            }
        };
        let a1830 = 0u64;
        nat_quot(a1828, a1829, a1830)
    };
    let v4: u64 = {
        let a1832 = v3;
        let a1833 = match v1.clone() {
            Adt3::C0(_, h1831, _, _, _, _, _, _) => {
                h1831
            }
        };
        let a1834 = 0u64;
        nat_quot(a1832, a1833, a1834)
    };
    let a1835 = v2;
    let a1836 = 0u64;
    if nat_eq(a1835, a1836) {
        let a1838 = v0;
        let a1839 = match v1 {
            Adt3::C0(h1837, _, _, _, _, _, _, _) => {
                h1837
            }
        };
        let a1840 = 0u64;
        nat_rem(a1838, a1839, a1840)
    } else {
        let a1841 = v2;
        let a1842 = 1u64;
        if nat_eq(a1841, a1842) {
            let a1844 = v3;
            let a1845 = match v1 {
                Adt3::C0(_, h1843, _, _, _, _, _, _) => {
                    h1843
                }
            };
            let a1846 = 0u64;
            nat_rem(a1844, a1845, a1846)
        } else {
            let a1847 = v2;
            let a1848 = 2u64;
            if nat_eq(a1847, a1848) {
                let a1850 = v4;
                let a1851 = match v1 {
                    Adt3::C0(_, _, h1849, _, _, _, _, _) => {
                        h1849
                    }
                };
                let a1852 = 0u64;
                nat_rem(a1850, a1851, a1852)
            } else {
                let a1854 = v4;
                let a1855 = match v1 {
                    Adt3::C0(_, _, h1853, _, _, _, _, _) => {
                        h1853
                    }
                };
                let a1856 = 0u64;
                nat_quot(a1854, a1855, a1856)
            }
        }
    }
}

pub fn f66(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1858 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1857) => {
            h1857
        }
    };
    let a1859 = 0u64;
    if nat_eq(a1858, a1859) {
        Ok(v2)
    } else {
        let a1864 = {
            let a1861 = v2;
            let a1862 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1860) => {
                    h1860
                }
            };
            nat_mul(a1861, a1862)?
        };
        let a1865 = f65(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1863, _, _, _, _, _, _) => {
                h1863
            }
        }, 3u64);
        nat_add(a1864, a1865)
    }
}

pub fn f67(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1875 = {
            let a1872 = {
                let a1867 = v2;
                let a1868 = match v0.clone() {
                    Adt6::C0(_, h1866, _, _) => {
                        h1866
                    }
                };
                nat_mul(a1867, a1868)?
            };
            let a1873 = {
                let a1870 = v3;
                let a1871 = match v0.clone() {
                    Adt6::C0(_, _, _, h1869) => {
                        h1869
                    }
                };
                nat_mul(a1870, a1871)?
            };
            nat_add(a1872, a1873)?
        };
        let a1876 = f121(match v0.clone() {
            Adt6::C0(_, _, h1874, _) => {
                h1874
            }
        });
        nat_add(a1875, a1876)?
    };
    let v5: u64 = f122(match v0 {
        Adt6::C0(_, _, h1877, _) => {
            h1877
        }
    });
    let a1878 = v5;
    let a1879 = v4;
    if if nat_le(a1878, a1879) {
        let a1882 = {
            let a1880 = v4;
            let a1881 = v5;
            nat_sub(a1880, a1881)
        };
        let a1883 = v1;
        nat_lt(a1882, a1883)
    } else {
        false
    } {
        Ok(Some({
            let a1884 = v4;
            let a1885 = v5;
            nat_sub(a1884, a1885)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f68(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1886, _, _, _, _, _) => {
            h1886
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1887, _, _, _, _, _, _) => {
            h1887
        }
    };
    let v7: u64 = f65(v1, v6.clone(), 2u64);
    let a1889 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1888) => {
            h1888
        }
    };
    let a1890 = 0u64;
    if nat_eq(a1889, a1890) {
        Ok(Adt9::C8(Some(f64(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1893 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1892, _, _, _, _) => {
                    h1892
                }
            };
            match m1893 {
                Adt3::C0(_, _, h1891, _, _, _, _, _) => {
                    h1891
                }
            }
        };
        let m1899 = f67(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1894, _) => {
                h1894
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1895, _, _, _, _, _) => {
                h1895
            }
        }, {
            let a1896 = v2;
            let a1897 = v8;
            let a1898 = 0u64;
            nat_rem(a1896, a1897, a1898)
        }, v7)?;
        match m1899 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1906 = {
                        let a1904 = {
                            let a1900 = v2;
                            let a1901 = v8;
                            let a1902 = 0u64;
                            nat_quot(a1900, a1901, a1902)
                        };
                        let a1905 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1903) => {
                                h1903
                            }
                        };
                        nat_mul(a1904, a1905)?
                    };
                    let a1907 = f65(v1, v6, 3u64);
                    nat_add(a1906, a1907)?
                };
                Ok(Adt9::C8(Some(f64(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f69(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1908 = v1;
    match m1908 {
        Adt9::C0(v4) => {
            Adt9::C0(v4)
        }
        Adt9::C1(v5, v6, v7) => {
            Adt9::C1(v5, v6, v7)
        }
        Adt9::C2(v8, v9, v10, v11, v12, v13) => {
            Adt9::C2(v8, v9, v10, v11, v12, v13)
        }
        Adt9::C3(v14, v15, v16, v17) => {
            f123(f124(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f125(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f126(v0.clone(), v18);
            f123(f127(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
        }
        Adt9::C5(v26, v27, v28, v29, v30, v31) => {
            Adt9::C5(v26, v27, v28, v29, v30, v31)
        }
        Adt9::C6(v32, v33, v34, v35) => {
            Adt9::C6(v32, v33, v34, v35)
        }
        Adt9::C7(v36, v37, v38, v39, v40, v41) => {
            Adt9::C7(v36, v37, v38, v39, v40, v41)
        }
        Adt9::C8(v2) => {
            Adt9::C8(v2)
        }
        Adt9::C9(v3) => {
            Adt9::C9(v3)
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1909 = v3;
    match m1909 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f128(v0, v1)?, f128(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f130(v0, v1, v2.clone())?, f128(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f131(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f132(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f133(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f132(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f134(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f135(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f136(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f128(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f137(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f128(v0, v1)?, f128(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f138(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f128(v0, v1)?, f128(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f128(v0, v1)?, f128(v0, v2)?))
        }
        Adt4::C15 => {
            f139(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f140(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f141(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f128(f129(v0, match v2.clone() {
                Adt3::C0(h1910, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v2.clone() {
                Adt3::C0(_, h1911, _, _, _, _, _, _) => {
                    h1911
                }
            })?, v1)?, f128(v0, v2)?))
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
        Adt4::C25(v48, v49, v50) => {
            f142(v0, v48, v49, v50)
        }
        Adt4::C26(v56, _, _, _) => {
            let a1912 = v56;
            let a1913 = 64u64;
            if if if nat_eq(a1912, a1913) {
                true
            } else {
                let a1914 = v56;
                let a1915 = 65u64;
                nat_eq(a1914, a1915)
            } {
                true
            } else {
                let a1916 = v56;
                let a1917 = 66u64;
                nat_eq(a1916, a1917)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v55) => {
            f78(v0, v1, v2, v55)
        }
        Adt4::C28(v51) => {
            f75(v0, v1, v2, v51, 0u64)
        }
        Adt4::C29(v52) => {
            f76(v0, v1, v2, v52)
        }
        Adt4::C30(v53, _) => {
            f77(v0, v1, v2, v53)
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f116(match v1.clone() {
        Adt3::C0(h1918, _, _, _, _, _, _, _) => {
            h1918
        }
    })?;
    let v5: u64 = {
        let a1919 = v0;
        let a1920 = v4;
        let a1921 = 0u64;
        nat_quot(a1919, a1920, a1921)
    };
    let v6: u64 = {
        let a1922 = v0;
        let a1923 = v4;
        let a1924 = 0u64;
        nat_rem(a1922, a1923, a1924)
    };
    let a1925 = v6;
    let a1926 = 0u64;
    if nat_eq(a1925, a1926) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1927 = v6;
        let a1928 = 1u64;
        if nat_eq(a1927, a1928) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1931 = {
                let a1929 = v6;
                let a1930 = 1u64;
                nat_add(a1929, a1930)?
            };
            let a1932 = v4;
            if nat_eq(a1931, a1932) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f128(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1933 = v6;
                    let a1934 = 2u64;
                    nat_sub(a1933, a1934)
                };
                let v8: u64 = {
                    let a1935 = v7;
                    let a1936 = 3u64;
                    let a1937 = 0u64;
                    nat_quot(a1935, a1936, a1937)
                };
                let v9: u64 = {
                    let a1938 = v7;
                    let a1939 = 3u64;
                    let a1940 = 0u64;
                    nat_rem(a1938, a1939, a1940)
                };
                let v10: u64 = f128({
                    let a1944 = {
                        let a1942 = v5;
                        let a1943 = match v1.clone() {
                            Adt3::C0(h1941, _, _, _, _, _, _, _) => {
                                h1941
                            }
                        };
                        nat_mul(a1942, a1943)?
                    };
                    let a1945 = v8;
                    nat_add(a1944, a1945)?
                }, v1)?;
                let a1946 = v9;
                let a1947 = 0u64;
                if nat_eq(a1946, a1947) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1948 = v9;
                    let a1949 = 1u64;
                    if nat_eq(a1948, a1949) {
                        let a1950 = v3;
                        let a1951 = 2u64;
                        if nat_eq(a1950, a1951) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1952 = v3;
                        let a1953 = 2u64;
                        if if nat_eq(a1952, a1953) {
                            true
                        } else {
                            let a1954 = v3;
                            let a1955 = 3u64;
                            nat_eq(a1954, a1955)
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

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1957 = 2u64;
        let a1958 = match v1.clone() {
            Adt3::C0(_, h1956, _, _, _, _, _, _) => {
                h1956
            }
        };
        nat_mul(a1957, a1958)?
    };
    let a1959 = v0;
    let a1960 = v5;
    if nat_lt(a1959, a1960) {
        let a1964 = {
            let a1961 = v0;
            let a1962 = 2u64;
            let a1963 = 0u64;
            nat_rem(a1961, a1962, a1963)
        };
        let a1965 = 0u64;
        if nat_eq(a1964, a1965) {
            Ok(Adt9::C4(19u64, 4u64, f128({
                let a1966 = v0;
                let a1967 = 2u64;
                let a1968 = 0u64;
                nat_quot(a1966, a1967, a1968)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1969 = v4;
            let a1970 = 2147483648u64;
            if if nat_lt(a1969, a1970) {
                let a1972 = v4;
                let a1973 = match v2 {
                    Adt3::C0(_, h1971, _, _, _, _, _, _) => {
                        h1971
                    }
                };
                nat_lt(a1972, a1973)
            } else {
                false
            } {
                Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1976 = match v2.clone() {
                Adt3::C0(h1974, _, _, _, _, _, _, _) => {
                    h1974
                }
            };
            let a1977 = match v2.clone() {
                Adt3::C0(_, h1975, _, _, _, _, _, _) => {
                    h1975
                }
            };
            nat_mul(a1976, a1977)?
        };
        let a1980 = {
            let a1978 = v0;
            let a1979 = v5;
            nat_sub(a1978, a1979)
        };
        let a1981 = v6;
        if nat_lt(a1980, a1981) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f128({
                let a1982 = v0;
                let a1983 = v5;
                nat_sub(a1982, a1983)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1986 = {
                    let a1984 = v0;
                    let a1985 = v5;
                    nat_sub(a1984, a1985)
                };
                let a1987 = v6;
                nat_sub(a1986, a1987)
            };
            let v8: u64 = {
                let a1991 = 1u64;
                let a1992 = {
                    let a1989 = 2u64;
                    let a1990 = match v1.clone() {
                        Adt3::C0(h1988, _, _, _, _, _, _, _) => {
                            h1988
                        }
                    };
                    nat_mul(a1989, a1990)?
                };
                nat_add(a1991, a1992)?
            };
            let v9: u64 = {
                let a1993 = v7;
                let a1994 = v8;
                let a1995 = 0u64;
                nat_quot(a1993, a1994, a1995)
            };
            let v10: u64 = {
                let a1996 = v7;
                let a1997 = v8;
                let a1998 = 0u64;
                nat_rem(a1996, a1997, a1998)
            };
            let a1999 = v10;
            let a2000 = 0u64;
            if nat_eq(a1999, a2000) {
                Ok(Adt9::C4(19u64, 4u64, f128(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a2004 = {
                    let a2001 = v10;
                    let a2002 = 2u64;
                    let a2003 = 0u64;
                    nat_rem(a2001, a2002, a2003)
                };
                let a2005 = 1u64;
                if nat_eq(a2004, a2005) {
                    Ok(Adt9::C4(6u64, 0u64, f128({
                        let a2014 = {
                            let a2007 = v9;
                            let a2008 = match v1.clone() {
                                Adt3::C0(h2006, _, _, _, _, _, _, _) => {
                                    h2006
                                }
                            };
                            nat_mul(a2007, a2008)?
                        };
                        let a2015 = {
                            let a2011 = {
                                let a2009 = v10;
                                let a2010 = 1u64;
                                nat_sub(a2009, a2010)
                            };
                            let a2012 = 2u64;
                            let a2013 = 0u64;
                            nat_quot(a2011, a2012, a2013)
                        };
                        nat_add(a2014, a2015)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f128({
                        let a2024 = {
                            let a2017 = v4;
                            let a2018 = match v1.clone() {
                                Adt3::C0(h2016, _, _, _, _, _, _, _) => {
                                    h2016
                                }
                            };
                            nat_mul(a2017, a2018)?
                        };
                        let a2025 = {
                            let a2021 = {
                                let a2019 = v10;
                                let a2020 = 1u64;
                                nat_sub(a2019, a2020)
                            };
                            let a2022 = 2u64;
                            let a2023 = 0u64;
                            nat_quot(a2021, a2022, a2023)
                        };
                        nat_add(a2024, a2025)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f128({
                        let a2034 = {
                            let a2027 = v4;
                            let a2028 = match v1 {
                                Adt3::C0(h2026, _, _, _, _, _, _, _) => {
                                    h2026
                                }
                            };
                            nat_mul(a2027, a2028)?
                        };
                        let a2035 = {
                            let a2031 = {
                                let a2029 = v10;
                                let a2030 = 1u64;
                                nat_sub(a2029, a2030)
                            };
                            let a2032 = 2u64;
                            let a2033 = 0u64;
                            nat_quot(a2031, a2032, a2033)
                        };
                        nat_add(a2034, a2035)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2036 = v1;
    let a2037 = f143(v2.clone(), v4.clone())?;
    if nat_lt(a2036, a2037) {
        let a2040 = v1;
        let a2041 = {
            let a2038 = 2u64;
            let a2039 = f117(v4.clone())?;
            nat_mul(a2038, a2039)?
        };
        if nat_lt(a2040, a2041) {
            let a2045 = {
                let a2042 = v1;
                let a2043 = 2u64;
                let a2044 = 0u64;
                nat_rem(a2042, a2043, a2044)
            };
            let a2046 = 0u64;
            if nat_eq(a2045, a2046) {
                f144({
                    let a2047 = v1;
                    let a2048 = 2u64;
                    let a2049 = 0u64;
                    nat_quot(a2047, a2048, a2049)
                }, v4)
            } else {
                if f145(match v2 {
                    Adt3::C0(_, h2050, _, _, _, _, _, _) => {
                        h2050
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2053 = v1;
                let a2054 = {
                    let a2051 = 2u64;
                    let a2052 = f117(v4.clone())?;
                    nat_mul(a2051, a2052)?
                };
                nat_sub(a2053, a2054)
            };
            let v7: u64 = {
                let a2058 = v6;
                let a2059 = {
                    let a2056 = 1u64;
                    let a2057 = match v2.clone() {
                        Adt3::C0(h2055, _, _, _, _, _, _, _) => {
                            h2055
                        }
                    };
                    nat_add(a2056, a2057)?
                };
                let a2060 = 0u64;
                nat_quot(a2058, a2059, a2060)
            };
            let v8: u64 = {
                let a2064 = v6;
                let a2065 = {
                    let a2062 = 1u64;
                    let a2063 = match v2.clone() {
                        Adt3::C0(h2061, _, _, _, _, _, _, _) => {
                            h2061
                        }
                    };
                    nat_add(a2062, a2063)?
                };
                let a2066 = 0u64;
                nat_rem(a2064, a2065, a2066)
            };
            let a2067 = v8;
            let a2068 = 0u64;
            if nat_eq(a2067, a2068) {
                f144(v7, v4)
            } else {
                f146(v0, v7, {
                    let a2069 = v8;
                    let a2070 = 1u64;
                    nat_sub(a2069, a2070)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2073 = match v4.clone() {
            Adt3::C0(h2071, _, _, _, _, _, _, _) => {
                h2071
            }
        };
        let a2074 = match v4.clone() {
            Adt3::C0(_, h2072, _, _, _, _, _, _) => {
                h2072
            }
        };
        nat_mul(a2073, a2074)?
    };
    let v7: u64 = {
        let a2075 = 2u64;
        let a2076 = v6;
        nat_mul(a2075, a2076)?
    };
    let v8: u64 = f147(match v3.clone() {
        Adt3::C0(_, h2077, _, _, _, _, _, _) => {
            h2077
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2078, _, _, _, _, _) => {
            h2078
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2079, _, _, _, _) => {
            h2079
        }
    });
    let a2080 = v0;
    let a2081 = v7;
    if nat_lt(a2080, a2081) {
        let a2085 = {
            let a2082 = v0;
            let a2083 = 2u64;
            let a2084 = 0u64;
            nat_rem(a2082, a2083, a2084)
        };
        let a2086 = 0u64;
        if nat_eq(a2085, a2086) {
            Ok(Adt9::C4(19u64, 5u64, f128({
                let a2087 = v0;
                let a2088 = 2u64;
                let a2089 = 0u64;
                nat_quot(a2087, a2088, a2089)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f148(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2090 = v0;
            let a2091 = v7;
            nat_sub(a2090, a2091)
        };
        let v10: u64 = {
            let a2095 = v9;
            let a2096 = {
                let a2093 = 1u64;
                let a2094 = match v1.clone() {
                    Adt3::C0(h2092, _, _, _, _, _, _, _) => {
                        h2092
                    }
                };
                nat_add(a2093, a2094)?
            };
            let a2097 = 0u64;
            nat_quot(a2095, a2096, a2097)
        };
        let v11: u64 = {
            let a2101 = v9;
            let a2102 = {
                let a2099 = 1u64;
                let a2100 = match v1.clone() {
                    Adt3::C0(h2098, _, _, _, _, _, _, _) => {
                        h2098
                    }
                };
                nat_add(a2099, a2100)?
            };
            let a2103 = 0u64;
            nat_rem(a2101, a2102, a2103)
        };
        let a2104 = v11;
        let a2105 = 0u64;
        if nat_eq(a2104, a2105) {
            Ok(Adt9::C4(19u64, 5u64, f128({
                let a2106 = v10;
                let a2107 = v6;
                let a2108 = 0u64;
                nat_rem(a2106, a2107, a2108)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f148(v5, v8) {
                let v12: u64 = {
                    let a2114 = {
                        let a2110 = v10;
                        let a2111 = match v1.clone() {
                            Adt3::C0(h2109, _, _, _, _, _, _, _) => {
                                h2109
                            }
                        };
                        nat_mul(a2110, a2111)?
                    };
                    let a2115 = {
                        let a2112 = v11;
                        let a2113 = 1u64;
                        nat_sub(a2112, a2113)
                    };
                    nat_add(a2114, a2115)?
                };
                let a2116 = v5;
                let a2117 = 4294967295u64;
                if nat_eq(a2116, a2117) {
                    Ok(Adt9::C1(false, f128(v12, v1)?, f128(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f128(v12, v1)?, 4u64, f128({
                        let a2123 = {
                            let a2119 = v5;
                            let a2120 = match v3.clone() {
                                Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                                    h2118
                                }
                            };
                            nat_mul(a2119, a2120)?
                        };
                        let a2124 = {
                            let a2121 = v11;
                            let a2122 = 1u64;
                            nat_sub(a2121, a2122)
                        };
                        nat_add(a2123, a2124)?
                    }, v3)?, 3u64, f128(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f102(v3.clone())?;
    let v6: u64 = {
        let a2125 = v0;
        let a2126 = v5;
        let a2127 = 0u64;
        nat_quot(a2125, a2126, a2127)
    };
    let v7: u64 = {
        let a2128 = v0;
        let a2129 = v5;
        let a2130 = 0u64;
        nat_rem(a2128, a2129, a2130)
    };
    let v8: u64 = f149(v3.clone())?;
    let a2131 = v7;
    let a2132 = 0u64;
    if nat_eq(a2131, a2132) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2133, _) => {
                h2133
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2137 = v5;
        let a2138 = {
            let a2135 = v7;
            let a2136 = if match v3.clone() {
                Adt7::C0(_, _, _, h2134, _) => {
                    h2134
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2135, a2136)?
        };
        if nat_le(a2137, a2138) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2139, _) => {
                    h2139
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f128(v6, v2)?))
            } else {
                let a2142 = {
                    let a2140 = v7;
                    let a2141 = 2u64;
                    nat_add(a2140, a2141)?
                };
                let a2143 = v5;
                if nat_eq(a2142, a2143) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2144, _, _) => {
                            h2144
                        }
                    } {
                        let a2151 = {
                            let m2147 = match v3.clone() {
                                Adt7::C0(h2146, _, _, _, _) => {
                                    h2146
                                }
                            };
                            match m2147 {
                                Adt6::C0(h2145, _, _, _) => {
                                    h2145
                                }
                            }
                        };
                        let a2152 = {
                            let m2150 = match v3 {
                                Adt7::C0(_, h2149, _, _, _) => {
                                    h2149
                                }
                            };
                            match m2150 {
                                Adt6::C0(h2148, _, _, _) => {
                                    h2148
                                }
                            }
                        };
                        nat_mul(a2151, a2152)?
                    } else {
                        f150(match v3 {
                            Adt7::C0(h2153, _, _, _, _) => {
                                h2153
                            }
                        }, match v1 {
                            Adt3::C0(h2154, _, _, _, _, _, _, _) => {
                                h2154
                            }
                        }, f65(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2155 = v9;
                        let a2156 = 0u64;
                        if nat_eq(a2155, a2156) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f128(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2159 = {
                    let a2157 = v7;
                    let a2158 = 1u64;
                    nat_sub(a2157, a2158)
                };
                let a2160 = v8;
                let a2161 = 0u64;
                nat_rem(a2159, a2160, a2161)
            };
            let m2167 = f151(v3.clone(), v1, v2, v6, {
                let a2164 = {
                    let a2162 = v7;
                    let a2163 = 1u64;
                    nat_sub(a2162, a2163)
                };
                let a2165 = v8;
                let a2166 = 0u64;
                nat_quot(a2164, a2165, a2166)
            })?;
            match m2167 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2169 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2168) => {
                            h2168
                        }
                    };
                    let a2170 = 1u64;
                    if if nat_eq(a2169, a2170) {
                        let a2171 = v10;
                        let a2172 = 0u64;
                        nat_eq(a2171, a2172)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2173, _) => {
                                h2173
                            }
                        } {
                            let a2176 = {
                                let a2174 = v10;
                                let a2175 = 1u64;
                                nat_add(a2174, a2175)?
                            };
                            let a2177 = v8;
                            if nat_eq(a2176, a2177) {
                                let a2178 = v4;
                                let a2179 = 2u64;
                                if nat_eq(a2178, a2179) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2181 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2180) => {
                                                h2180
                                            }
                                        };
                                        let a2182 = 1u64;
                                        if nat_eq(a2181, a2182) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2184 = match v3 {
                                            Adt7::C0(_, _, _, _, h2183) => {
                                                h2183
                                            }
                                        };
                                        let a2185 = 1u64;
                                        if nat_eq(a2184, a2185) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2187 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2186) => {
                                            h2186
                                        }
                                    };
                                    let a2188 = 1u64;
                                    if nat_eq(a2187, a2188) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2190 = match v3 {
                                        Adt7::C0(_, _, _, _, h2189) => {
                                            h2189
                                        }
                                    };
                                    let a2191 = 1u64;
                                    if nat_eq(a2190, a2191) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2193 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2192) => {
                                        h2192
                                    }
                                };
                                let a2194 = 1u64;
                                if nat_eq(a2193, a2194) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2196 = match v3 {
                                    Adt7::C0(_, _, _, _, h2195) => {
                                        h2195
                                    }
                                };
                                let a2197 = 1u64;
                                if nat_eq(a2196, a2197) {
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

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f100(match v3.clone() {
        Adt3::C0(h2198, _, _, _, _, _, _, _) => {
            h2198
        }
    })?;
    let v5: u64 = {
        let a2199 = v0;
        let a2200 = v4;
        let a2201 = 0u64;
        nat_rem(a2199, a2200, a2201)
    };
    let v6: u64 = {
        let a2202 = v0;
        let a2203 = v4;
        let a2204 = 0u64;
        nat_quot(a2202, a2203, a2204)
    };
    let a2205 = v5;
    let a2206 = 0u64;
    if nat_eq(a2205, a2206) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2209 = {
            let a2207 = v5;
            let a2208 = 1u64;
            nat_add(a2207, a2208)?
        };
        let a2210 = v4;
        if nat_eq(a2209, a2210) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f128(v6, v2)?))
        } else {
            let a2214 = {
                let a2211 = v5;
                let a2212 = 2u64;
                let a2213 = 0u64;
                nat_rem(a2211, a2212, a2213)
            };
            let a2215 = 0u64;
            if nat_eq(a2214, a2215) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2218 = {
                        let a2216 = v5;
                        let a2217 = 1u64;
                        nat_sub(a2216, a2217)
                    };
                    let a2219 = 2u64;
                    let a2220 = 0u64;
                    nat_quot(a2218, a2219, a2220)
                };
                let v8: u64 = f65(v6, v2.clone(), 0u64);
                let v9: u64 = f65(v6, v2.clone(), 1u64);
                let v10: u64 = f65(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f64(v1, {
                    let a2221 = v9;
                    let a2222 = v7;
                    nat_add(a2221, a2222)?
                }, v8, v10, 0u64)?, 4u64, f64(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f99(v3.clone())?;
    let v5: u64 = {
        let a2223 = v0;
        let a2224 = v4;
        let a2225 = 0u64;
        nat_rem(a2223, a2224, a2225)
    };
    let v6: u64 = {
        let a2226 = v0;
        let a2227 = v4;
        let a2228 = 0u64;
        nat_quot(a2226, a2227, a2228)
    };
    let a2229 = v5;
    let a2230 = 0u64;
    if nat_eq(a2229, a2230) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2233 = {
            let a2231 = v5;
            let a2232 = 1u64;
            nat_add(a2231, a2232)?
        };
        let a2234 = v4;
        if nat_eq(a2233, a2234) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f128(v6, v2)?))
        } else {
            let a2238 = {
                let a2235 = v5;
                let a2236 = 2u64;
                let a2237 = 0u64;
                nat_rem(a2235, a2236, a2237)
            };
            let a2239 = 0u64;
            if nat_eq(a2238, a2239) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2241 = f65(v6, v2.clone(), 0u64);
                    let a2242 = match v3.clone() {
                        Adt6::C0(_, _, h2240, _) => {
                            h2240
                        }
                    };
                    nat_add(a2241, a2242)?
                };
                let v8: u64 = {
                    let a2250 = {
                        let a2247 = {
                            let a2244 = v7;
                            let a2245 = match v3.clone() {
                                Adt6::C0(_, h2243, _, _) => {
                                    h2243
                                }
                            };
                            nat_add(a2244, a2245)?
                        };
                        let a2248 = match v3.clone() {
                            Adt6::C0(h2246, _, _, _) => {
                                h2246
                            }
                        };
                        nat_sub(a2247, a2248)
                    };
                    let a2251 = match v3.clone() {
                        Adt6::C0(_, h2249, _, _) => {
                            h2249
                        }
                    };
                    let a2252 = 0u64;
                    nat_quot(a2250, a2251, a2252)
                };
                let v9: u64 = {
                    let a2258 = v8;
                    let a2259 = {
                        let a2255 = {
                            let a2253 = v5;
                            let a2254 = 1u64;
                            nat_sub(a2253, a2254)
                        };
                        let a2256 = 2u64;
                        let a2257 = 0u64;
                        nat_quot(a2255, a2256, a2257)
                    };
                    nat_add(a2258, a2259)?
                };
                let v10: u64 = {
                    let a2261 = v9;
                    let a2262 = match v3.clone() {
                        Adt6::C0(_, h2260, _, _) => {
                            h2260
                        }
                    };
                    nat_mul(a2261, a2262)?
                };
                let v11: u64 = {
                    let a2263 = v7;
                    let a2264 = v10;
                    nat_sub(a2263, a2264)
                };
                let a2266 = v9;
                let a2267 = match v1.clone() {
                    Adt3::C0(_, h2265, _, _, _, _, _, _) => {
                        h2265
                    }
                };
                if if if nat_lt(a2266, a2267) {
                    let a2268 = v10;
                    let a2269 = v7;
                    nat_le(a2268, a2269)
                } else {
                    false
                } {
                    let a2271 = v11;
                    let a2272 = match v3.clone() {
                        Adt6::C0(h2270, _, _, _) => {
                            h2270
                        }
                    };
                    nat_lt(a2271, a2272)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f64(v1, {
                        let a2276 = {
                            let a2274 = f65(v6, v2, 1u64);
                            let a2275 = match v3 {
                                Adt6::C0(h2273, _, _, _) => {
                                    h2273
                                }
                            };
                            nat_mul(a2274, a2275)?
                        };
                        let a2277 = v11;
                        nat_add(a2276, a2277)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2279 = v5;
        let a2280 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2278, _) => {
                h2278
            }
        };
        let a2281 = 0u64;
        nat_rem(a2279, a2280, a2281)
    };
    let v7: u64 = {
        let a2283 = v5;
        let a2284 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2282, _) => {
                h2282
            }
        };
        let a2285 = 0u64;
        nat_quot(a2283, a2284, a2285)
    };
    let v8: Option<u64> = f67(match v3.clone() {
        Adt5::C0(h2286, _, _, _, _, _, _, _) => {
            h2286
        }
    }, match v1.clone() {
        Adt3::C0(h2287, _, _, _, _, _, _, _) => {
            h2287
        }
    }, f65(v0, v2.clone(), 1u64), {
        let a2291 = v4;
        let a2292 = {
            let m2290 = match v3.clone() {
                Adt5::C0(h2289, _, _, _, _, _, _, _) => {
                    h2289
                }
            };
            match m2290 {
                Adt6::C0(h2288, _, _, _) => {
                    h2288
                }
            }
        };
        let a2293 = 0u64;
        nat_rem(a2291, a2292, a2293)
    })?;
    let v9: u64 = {
        let a2297 = v4;
        let a2298 = {
            let m2296 = match v3.clone() {
                Adt5::C0(h2295, _, _, _, _, _, _, _) => {
                    h2295
                }
            };
            match m2296 {
                Adt6::C0(h2294, _, _, _) => {
                    h2294
                }
            }
        };
        let a2299 = 0u64;
        nat_quot(a2297, a2298, a2299)
    };
    let v10: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, h2300, _, _, _, _, _, _) => {
            h2300
        }
    }, match v1.clone() {
        Adt3::C0(_, h2301, _, _, _, _, _, _) => {
            h2301
        }
    }, f65(v0, v2.clone(), 2u64), {
        let a2305 = v9;
        let a2306 = {
            let m2304 = match v3.clone() {
                Adt5::C0(_, h2303, _, _, _, _, _, _) => {
                    h2303
                }
            };
            match m2304 {
                Adt6::C0(h2302, _, _, _) => {
                    h2302
                }
            }
        };
        let a2307 = 0u64;
        nat_rem(a2305, a2306, a2307)
    })?;
    let v11: u64 = {
        let a2311 = v9;
        let a2312 = {
            let m2310 = match v3.clone() {
                Adt5::C0(_, h2309, _, _, _, _, _, _) => {
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
        nat_quot(a2311, a2312, a2313)
    };
    let v12: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, _, h2314, _, _, _, _, _) => {
            h2314
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2315, _, _, _, _, _) => {
            h2315
        }
    }, v6, {
        let a2319 = v11;
        let a2320 = {
            let m2318 = match v3.clone() {
                Adt5::C0(_, _, h2317, _, _, _, _, _) => {
                    h2317
                }
            };
            match m2318 {
                Adt6::C0(h2316, _, _, _) => {
                    h2316
                }
            }
        };
        let a2321 = 0u64;
        nat_rem(a2319, a2320, a2321)
    })?;
    let v13: u64 = {
        let a2325 = v11;
        let a2326 = {
            let m2324 = match v3.clone() {
                Adt5::C0(_, _, h2323, _, _, _, _, _) => {
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
        nat_quot(a2325, a2326, a2327)
    };
    let v17: Adt12 = {
        let m2328 = v8;
        match m2328 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2329 = v10;
                match m2329 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2330 = v12;
                        match m2330 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2339 = {
                                        let a2334 = f64(v1, v14, v15, v16, 0u64)?;
                                        let a2335 = {
                                            let a2332 = v7;
                                            let a2333 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2331, _, _, _) => {
                                                    h2331
                                                }
                                            };
                                            nat_mul(a2332, a2333)?
                                        };
                                        nat_add(a2334, a2335)?
                                    };
                                    let a2340 = {
                                        let a2337 = v13;
                                        let a2338 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2336, _, _, _, _) => {
                                                h2336
                                            }
                                        };
                                        nat_mul(a2337, a2338)?
                                    };
                                    nat_add(a2339, a2340)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2341, _, _) => {
            h2341
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2342) => {
            h2342
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f128(v0, v2)?), {
        let a2344 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2343, _, _) => {
                h2343
            }
        };
        let a2345 = 0u64;
        if nat_eq(a2344, a2345) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2347 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2346) => {
                h2346
            }
        };
        let a2348 = 0u64;
        if nat_eq(a2347, a2348) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f79(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2361 = f152(v0, v1, match v2.clone() {
        Adt3::C0(h2349, _, _, _, _, _, _, _) => {
            h2349
        }
    }, match v2.clone() {
        Adt3::C0(_, h2350, _, _, _, _, _, _) => {
            h2350
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2351, _, _, _, _, _) => {
            h2351
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2352, _, _, _, _) => {
            h2352
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2353, _, _, _) => {
            h2353
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2354, _, _) => {
            h2354
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2355, _) => {
            h2355
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2356) => {
            h2356
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2357, _, _, _) => {
            h2357
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2358, _, _) => {
            h2358
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2359, _) => {
            h2359
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2360) => {
            h2360
        }
    }, v4)?;
    match m2361 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f13(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2364 = v6;
        let a2365 = {
            let a2362 = 3u64;
            let a2363 = v5;
            nat_add(a2362, a2363)?
        };
        nat_sub(a2364, a2365)
    };
    let a2367 = v0;
    let a2368 = f33(v3, match v1.clone() {
        Adt3::C0(h2366, _, _, _, _, _, _, _) => {
            h2366
        }
    }, v5, v8)?;
    if nat_lt(a2367, a2368) {
        let v9: u64 = f93(match v1.clone() {
            Adt3::C0(h2369, _, _, _, _, _, _, _) => {
                h2369
            }
        }, v5, v8)?;
        let v10: u64 = f94(v3, match v1.clone() {
            Adt3::C0(h2370, _, _, _, _, _, _, _) => {
                h2370
            }
        }, v5)?;
        let a2371 = v0;
        let a2372 = v9;
        if nat_lt(a2371, a2372) {
            f153(v0, v2, f27(v3), {
                let a2373 = 3u64;
                let a2374 = v5;
                nat_add(a2373, a2374)?
            }, v7)
        } else {
            let a2377 = v0;
            let a2378 = {
                let a2375 = v9;
                let a2376 = v10;
                nat_add(a2375, a2376)?
            };
            if nat_lt(a2377, a2378) {
                let v11: u64 = {
                    let a2379 = v0;
                    let a2380 = v9;
                    nat_sub(a2379, a2380)
                };
                let v12: u64 = f154(v3, match v1.clone() {
                    Adt3::C0(h2381, _, _, _, _, _, _, _) => {
                        h2381
                    }
                })?;
                f155({
                    let a2382 = v11;
                    let a2383 = v12;
                    let a2384 = 0u64;
                    nat_rem(a2382, a2383, a2384)
                }, {
                    let a2385 = v11;
                    let a2386 = v12;
                    let a2387 = 0u64;
                    nat_quot(a2385, a2386, a2387)
                }, v1, v3, v4, v7)
            } else {
                f156({
                    let a2390 = v0;
                    let a2391 = {
                        let a2388 = v9;
                        let a2389 = v10;
                        nat_add(a2388, a2389)?
                    };
                    nat_sub(a2390, a2391)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2392 = v0;
    let a2393 = f17(v1)?;
    if nat_lt(a2392, a2393) {
        let a2394 = v0;
        let a2395 = 4u64;
        if nat_lt(a2394, a2395) {
            let a2396 = v0;
            let a2397 = 0u64;
            if nat_eq(a2396, a2397) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2398 = v0;
                let a2399 = 1u64;
                if nat_eq(a2398, a2399) {
                    let a2400 = v5;
                    let a2401 = 0u64;
                    if nat_eq(a2400, a2401) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2402 = v0;
                    let a2403 = 2u64;
                    if nat_eq(a2402, a2403) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2404 = v0;
            let a2405 = 4u64;
            if if nat_eq(a2404, a2405) {
                let a2406 = f157(v5)?;
                let a2407 = v1;
                !nat_eq(a2406, a2407)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2410 = {
                        let a2408 = v0;
                        let a2409 = 4u64;
                        nat_sub(a2408, a2409)
                    };
                    let a2411 = 3u64;
                    let a2412 = 0u64;
                    nat_quot(a2410, a2411, a2412)
                };
                let v7: u64 = {
                    let a2415 = {
                        let a2413 = v0;
                        let a2414 = 4u64;
                        nat_sub(a2413, a2414)
                    };
                    let a2416 = 3u64;
                    let a2417 = 0u64;
                    nat_rem(a2415, a2416, a2417)
                };
                let a2418 = v7;
                let a2419 = 0u64;
                if nat_eq(a2418, a2419) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2420 = v7;
                    let a2421 = 1u64;
                    if nat_eq(a2420, a2421) {
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

pub fn f82(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2422 = v1;
    let a2423 = 0u64;
    if nat_eq(a2422, a2423) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2424 = v1;
        let a2425 = 1u64;
        if nat_eq(a2424, a2425) {
            Adt11::C1(v2)
        } else {
            let a2426 = v1;
            let a2427 = 2u64;
            if nat_eq(a2426, a2427) {
                Adt11::C5(v2)
            } else {
                let a2428 = v1;
                let a2429 = 3u64;
                if nat_eq(a2428, a2429) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2430 = v1;
                    let a2431 = 4u64;
                    if nat_eq(a2430, a2431) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: u64) -> Adt12 {
    let a2432 = v0;
    let a2433 = 0u64;
    if nat_eq(a2432, a2433) {
        Adt12::C0(0u64, v1)
    } else {
        let a2434 = v0;
        let a2435 = 1u64;
        if nat_eq(a2434, a2435) {
            Adt12::C1(v1)
        } else {
            let a2436 = v0;
            let a2437 = 2u64;
            if nat_eq(a2436, a2437) {
                Adt12::C2(v1)
            } else {
                let a2438 = v0;
                let a2439 = 3u64;
                if nat_eq(a2438, a2439) {
                    Adt12::C3(v1)
                } else {
                    let a2440 = v0;
                    let a2441 = 13u64;
                    if nat_lt(a2440, a2441) {
                        Adt12::C0({
                            let a2442 = v0;
                            let a2443 = 3u64;
                            nat_sub(a2442, a2443)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: u64) -> Adt13 {
    let a2444 = v0;
    let a2445 = 1u64;
    if nat_eq(a2444, a2445) {
        Adt13::C0(v1)
    } else {
        let a2446 = v0;
        let a2447 = 3u64;
        if if nat_eq(a2446, a2447) {
            true
        } else {
            let a2448 = v0;
            let a2449 = 6u64;
            nat_eq(a2448, a2449)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f85(v0: Adt6, v1: u64) -> R<u64> {
    let a2451 = 0u64;
    let a2452 = match v0.clone() {
        Adt6::C0(h2450, _, _, _) => {
            h2450
        }
    };
    if if if if if nat_lt(a2451, a2452) {
        let a2454 = 0u64;
        let a2455 = match v0.clone() {
            Adt6::C0(_, h2453, _, _) => {
                h2453
            }
        };
        nat_lt(a2454, a2455)
    } else {
        false
    } {
        let a2457 = 0u64;
        let a2458 = match v0.clone() {
            Adt6::C0(_, _, _, h2456) => {
                h2456
            }
        };
        nat_lt(a2457, a2458)
    } else {
        false
    } {
        let a2460 = match v0.clone() {
            Adt6::C0(_, h2459, _, _) => {
                h2459
            }
        };
        let a2461 = 2147483648u64;
        nat_lt(a2460, a2461)
    } else {
        false
    } {
        let a2463 = match v0.clone() {
            Adt6::C0(_, _, _, h2462) => {
                h2462
            }
        };
        let a2464 = 2147483648u64;
        nat_lt(a2463, a2464)
    } else {
        false
    } {
        let v2: u64 = {
            let a2468 = v1;
            let a2469 = {
                let a2466 = 2u64;
                let a2467 = f122(match v0.clone() {
                    Adt6::C0(_, _, h2465, _) => {
                        h2465
                    }
                });
                nat_mul(a2466, a2467)?
            };
            nat_add(a2468, a2469)?
        };
        let v3: u64 = {
            let a2481 = {
                let a2476 = 1u64;
                let a2477 = {
                    let a2474 = match v0.clone() {
                        Adt6::C0(_, _, _, h2470) => {
                            h2470
                        }
                    };
                    let a2475 = {
                        let a2472 = match v0.clone() {
                            Adt6::C0(h2471, _, _, _) => {
                                h2471
                            }
                        };
                        let a2473 = 1u64;
                        nat_sub(a2472, a2473)
                    };
                    nat_mul(a2474, a2475)?
                };
                nat_add(a2476, a2477)?
            };
            let a2482 = {
                let a2479 = 2u64;
                let a2480 = f121(match v0.clone() {
                    Adt6::C0(_, _, h2478, _) => {
                        h2478
                    }
                });
                nat_mul(a2479, a2480)?
            };
            nat_add(a2481, a2482)?
        };
        let a2483 = v3;
        let a2484 = v2;
        if nat_le(a2483, a2484) {
            let a2491 = 1u64;
            let a2492 = {
                let a2488 = {
                    let a2485 = v2;
                    let a2486 = v3;
                    nat_sub(a2485, a2486)
                };
                let a2489 = match v0 {
                    Adt6::C0(_, h2487, _, _) => {
                        h2487
                    }
                };
                let a2490 = 0u64;
                nat_quot(a2488, a2489, a2490)
            };
            nat_add(a2491, a2492)
        } else {
            let a2496 = {
                let a2493 = v3;
                let a2494 = v2;
                nat_sub(a2493, a2494)
            };
            let a2497 = match v0 {
                Adt6::C0(_, h2495, _, _) => {
                    h2495
                }
            };
            if nat_lt(a2496, a2497) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f86(v0: u64) -> u64 {
    let v1: u64 = {
        let a2498 = v0;
        let a2499 = 4294967296u64;
        let a2500 = 0u64;
        nat_rem(a2498, a2499, a2500)
    };
    let a2501 = v1;
    let a2502 = 2147483648u64;
    if nat_lt(a2501, a2502) {
        v1
    } else {
        0u64
    }
}

pub fn f87(v0: u64) -> R<u64> {
    let a2505 = 2u64;
    let a2506 = {
        let a2503 = 2u64;
        let a2504 = v0;
        nat_mul(a2503, a2504)?
    };
    nat_add(a2505, a2506)
}

pub fn f88(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2508 = match v1.clone() {
        Adt14::C0(h2507, _, _, _, _, _, _, _, _, _, _) => {
            h2507
        }
    };
    let a2509 = 0u64;
    if if if if nat_eq(a2508, a2509) {
        let a2511 = match v2.clone() {
            Adt14::C0(h2510, _, _, _, _, _, _, _, _, _, _) => {
                h2510
            }
        };
        let a2512 = 0u64;
        nat_eq(a2511, a2512)
    } else {
        false
    } {
        f158(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f159(match v1.clone() {
            Adt14::C0(_, h2513, _, _, _, _, _, _, _, _, _) => {
                h2513
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2514, _, _, _, _, _, _, _, _) => {
                h2514
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2515, _, _, _, _, _, _, _) => {
                h2515
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2516, _, _, _, _, _, _) => {
                h2516
            }
        }) {
            true
        } else {
            if if if if f28(v0) {
                f160(v1.clone(), v2.clone())
            } else {
                false
            } {
                f161(v1.clone())
            } else {
                false
            } {
                f161(v2.clone())
            } else {
                false
            } {
                if f162(v1.clone(), v2.clone())? {
                    true
                } else {
                    f163(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f159(match v1.clone() {
            Adt14::C0(_, h2517, _, _, _, _, _, _, _, _, _) => {
                h2517
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2518, _, _, _, _, _, _, _, _) => {
                h2518
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2519, _, _, _, _, _, _, _) => {
                h2519
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2520, _, _, _, _, _, _) => {
                h2520
            }
        }) {
            0u64
        } else {
            let a2526 = match v1.clone() {
                Adt14::C0(_, _, h2521, _, _, _, _, _, _, _, _) => {
                    h2521
                }
            };
            let a2527 = {
                let a2524 = match v1.clone() {
                    Adt14::C0(_, _, _, h2522, _, _, _, _, _, _, _) => {
                        h2522
                    }
                };
                let a2525 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2523, _, _, _, _, _, _) => {
                        h2523
                    }
                };
                nat_mul(a2524, a2525)?
            };
            nat_mul(a2526, a2527)?
        };
        let v4: u64 = {
            let a2528 = v3;
            let a2529 = 0u64;
            if if nat_eq(a2528, a2529) {
                true
            } else {
                f164(v2.clone())?
            } {
                0u64
            } else {
                let a2531 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2530, _) => {
                        h2530
                    }
                };
                let a2532 = 4u64;
                let a2533 = 0u64;
                nat_quot(a2531, a2532, a2533)
            }
        };
        Ok(Adt15::C1(f165(f166(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2534, _, _, _, _, _) => {
                h2534
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2535, _, _, _, _, _) => {
                h2535
            }
        }), match v1 {
            Adt14::C0(_, h2536, _, _, _, _, _, _, _, _, _) => {
                h2536
            }
        }, v3, v4)?, 4u64, 64u64, v3, f167(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f89(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f168(v3.clone());
    let a2538 = match v2.clone() {
        Adt14::C0(h2537, _, _, _, _, _, _, _, _, _, _) => {
            h2537
        }
    };
    let a2539 = 0u64;
    if if if if nat_eq(a2538, a2539) {
        let a2541 = match v3.clone() {
            Adt14::C0(h2540, _, _, _, _, _, _, _, _, _, _) => {
                h2540
            }
        };
        let a2542 = 0u64;
        nat_eq(a2541, a2542)
    } else {
        false
    } {
        f158(v2.clone(), v3)
    } else {
        false
    } {
        if f159(match v2.clone() {
            Adt14::C0(_, h2543, _, _, _, _, _, _, _, _, _) => {
                h2543
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2544, _, _, _, _, _, _, _, _) => {
                h2544
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2545, _, _, _, _, _, _, _) => {
                h2545
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2546, _, _, _, _, _, _) => {
                h2546
            }
        }) {
            true
        } else {
            let a2548 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2547, _, _, _, _, _) => {
                    h2547
                }
            };
            let a2549 = 4u64;
            if if if if if nat_eq(a2548, a2549) {
                f169(match v2.clone() {
                    Adt14::C0(_, h2550, _, _, _, _, _, _, _, _, _) => {
                        h2550
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2551, _, _, _, _, _, _, _, _) => {
                        h2551
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2552, _, _, _, _, _, _, _) => {
                        h2552
                    }
                }, v0)?
            } else {
                false
            } {
                f161(v2.clone())
            } else {
                false
            } {
                f161(v4.clone())
            } else {
                false
            } {
                let a2554 = f170(v4.clone())?;
                let a2555 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2553, _) => {
                        h2553
                    }
                };
                nat_le(a2554, a2555)
            } else {
                false
            } {
                if f162(v2.clone(), v4.clone())? {
                    true
                } else {
                    f163(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f159(match v2.clone() {
            Adt14::C0(_, h2556, _, _, _, _, _, _, _, _, _) => {
                h2556
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2557, _, _, _, _, _, _, _, _) => {
                h2557
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2558, _, _, _, _, _, _, _) => {
                h2558
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2559, _, _, _, _, _, _) => {
                h2559
            }
        }) {
            true
        } else {
            f164(v4.clone())?
        } {
            0u64
        } else {
            let a2561 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2560, _) => {
                    h2560
                }
            };
            let a2562 = 4u64;
            let a2563 = 0u64;
            nat_quot(a2561, a2562, a2563)
        };
        Ok(Adt15::C1(f171(match v2.clone() {
            Adt14::C0(_, h2564, _, _, _, _, _, _, _, _, _) => {
                h2564
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2565, _, _, _, _, _, _, _, _) => {
                h2565
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2566, _, _, _, _, _, _, _) => {
                h2566
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2567, _, _, _, _, _, _) => {
                h2567
            }
        }, v0, v5)?, 4u64, 65u64, v0, f167(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f90(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f168(v4.clone());
    if if if if if if if v1 {
        let a2569 = match v2.clone() {
            Adt14::C0(h2568, _, _, _, _, _, _, _, _, _, _) => {
                h2568
            }
        };
        let a2570 = 0u64;
        nat_eq(a2569, a2570)
    } else {
        false
    } {
        let a2572 = match v3.clone() {
            Adt14::C0(h2571, _, _, _, _, _, _, _, _, _, _) => {
                h2571
            }
        };
        let a2573 = 0u64;
        nat_eq(a2572, a2573)
    } else {
        false
    } {
        let a2575 = match v4.clone() {
            Adt14::C0(h2574, _, _, _, _, _, _, _, _, _, _) => {
                h2574
            }
        };
        let a2576 = 0u64;
        nat_eq(a2575, a2576)
    } else {
        false
    } {
        f158(v2.clone(), v3.clone())
    } else {
        false
    } {
        f158(v2.clone(), v4)
    } else {
        false
    } {
        if f159(match v2.clone() {
            Adt14::C0(_, h2577, _, _, _, _, _, _, _, _, _) => {
                h2577
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2578, _, _, _, _, _, _, _, _) => {
                h2578
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2579, _, _, _, _, _, _, _) => {
                h2579
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2580, _, _, _, _, _, _) => {
                h2580
            }
        }) {
            true
        } else {
            let a2582 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2581, _, _, _, _, _) => {
                    h2581
                }
            };
            let a2583 = 4u64;
            if if if if if if if nat_eq(a2582, a2583) {
                let a2585 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2584, _, _, _, _, _) => {
                        h2584
                    }
                };
                let a2586 = 4u64;
                nat_eq(a2585, a2586)
            } else {
                false
            } {
                f161(v2.clone())
            } else {
                false
            } {
                f161(v3.clone())
            } else {
                false
            } {
                f161(v5.clone())
            } else {
                false
            } {
                let a2588 = f170(v5.clone())?;
                let a2589 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2587, _) => {
                        h2587
                    }
                };
                nat_le(a2588, a2589)
            } else {
                false
            } {
                if f162(v2.clone(), v5.clone())? {
                    true
                } else {
                    f163(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f162(v3.clone(), v5.clone())? {
                    true
                } else {
                    f163(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f159(match v2.clone() {
            Adt14::C0(_, h2590, _, _, _, _, _, _, _, _, _) => {
                h2590
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2591, _, _, _, _, _, _, _, _) => {
                h2591
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2592, _, _, _, _, _, _, _) => {
                h2592
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2593, _, _, _, _, _, _) => {
                h2593
            }
        }) {
            0u64
        } else {
            let a2599 = match v2.clone() {
                Adt14::C0(_, _, h2594, _, _, _, _, _, _, _, _) => {
                    h2594
                }
            };
            let a2600 = {
                let a2597 = match v2.clone() {
                    Adt14::C0(_, _, _, h2595, _, _, _, _, _, _, _) => {
                        h2595
                    }
                };
                let a2598 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2596, _, _, _, _, _, _) => {
                        h2596
                    }
                };
                nat_mul(a2597, a2598)?
            };
            nat_mul(a2599, a2600)?
        };
        let v7: u64 = {
            let a2601 = v6;
            let a2602 = 0u64;
            if if nat_eq(a2601, a2602) {
                true
            } else {
                f164(v5.clone())?
            } {
                0u64
            } else {
                let a2604 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2603, _) => {
                        h2603
                    }
                };
                let a2605 = 4u64;
                let a2606 = 0u64;
                nat_quot(a2604, a2605, a2606)
            }
        };
        Ok(Adt15::C1(f172(match v2 {
            Adt14::C0(_, h2607, _, _, _, _, _, _, _, _, _) => {
                h2607
            }
        }, v6, v7)?, 4u64, 66u64, v6, f167(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f173(v0, v3, v6) {
        f173(v1, v4, v6)
    } else {
        false
    } {
        f173(v2, v5, v6)
    } else {
        false
    }
}

pub fn f92(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f174(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f174(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f174(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f174(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f174(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f174(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2608 = v2;
    let a2609 = 0u64;
    if nat_eq(a2608, a2609) {
        Ok(0u64)
    } else {
        let a2612 = 2u64;
        let a2613 = {
            let a2610 = v0;
            let a2611 = v1;
            nat_mul(a2610, a2611)?
        };
        nat_mul(a2612, a2613)
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2614 = v2;
    let a2615 = f154(v0, v1)?;
    nat_mul(a2614, a2615)
}

pub fn f95(v0: u64) -> u64 {
    let a2616 = v0;
    let a2617 = 30u64;
    if nat_eq(a2616, a2617) {
        3u64
    } else {
        1u64
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a2620 = 5u64;
    let a2621 = {
        let a2618 = 4u64;
        let a2619 = v0;
        nat_mul(a2618, a2619)?
    };
    nat_add(a2620, a2621)
}

pub fn f97(v0: u64) -> bool {
    let a2622 = v0;
    let a2623 = 0u64;
    if if nat_eq(a2622, a2623) {
        true
    } else {
        let a2624 = v0;
        let a2625 = 1u64;
        nat_eq(a2624, a2625)
    } {
        true
    } else {
        let a2626 = v0;
        let a2627 = 30u64;
        nat_eq(a2626, a2627)
    }
}

pub fn f98(v0: Adt2, v1: u64) -> R<bool> {
    let a2629 = match v0.clone() {
        Adt2::C0(_, _, _, h2628, _, _, _, _, _, _, _) => {
            h2628
        }
    };
    let a2630 = 1u64;
    if if if nat_eq(a2629, a2630) {
        let a2632 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2631, _, _, _, _, _, _) => {
                h2631
            }
        };
        let a2633 = 1u64;
        nat_eq(a2632, a2633)
    } else {
        false
    } {
        let a2635 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2634, _, _, _, _, _) => {
                h2634
            }
        };
        let a2636 = v1;
        nat_eq(a2635, a2636)
    } else {
        false
    } {
        let a2638 = match v0.clone() {
            Adt2::C0(_, _, h2637, _, _, _, _, _, _, _, _) => {
                h2637
            }
        };
        let a2639 = 1u64;
        if nat_eq(a2638, a2639) {
            Ok(true)
        } else {
            let a2644 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2640, _, _, _, _) => {
                    h2640
                }
            };
            let a2645 = {
                let a2642 = match v0 {
                    Adt2::C0(_, h2641, _, _, _, _, _, _, _, _, _) => {
                        h2641
                    }
                };
                let a2643 = v1;
                nat_mul(a2642, a2643)?
            };
            Ok(nat_eq(a2644, a2645))
        }
    } else {
        Ok(false)
    }
}

pub fn f99(v0: Adt6) -> R<u64> {
    let a2658 = 2u64;
    let a2659 = {
        let a2656 = 2u64;
        let a2657 = {
            let a2653 = {
                let a2650 = match v0.clone() {
                    Adt6::C0(h2646, _, _, _) => {
                        h2646
                    }
                };
                let a2651 = {
                    let a2648 = match v0.clone() {
                        Adt6::C0(_, h2647, _, _) => {
                            h2647
                        }
                    };
                    let a2649 = 1u64;
                    nat_sub(a2648, a2649)
                };
                nat_add(a2650, a2651)?
            };
            let a2654 = match v0 {
                Adt6::C0(_, h2652, _, _) => {
                    h2652
                }
            };
            let a2655 = 0u64;
            nat_quot(a2653, a2654, a2655)
        };
        nat_mul(a2656, a2657)?
    };
    nat_add(a2658, a2659)
}

pub fn f100(v0: u64) -> R<u64> {
    let a2662 = 2u64;
    let a2663 = {
        let a2660 = v0;
        let a2661 = 2u64;
        nat_mul(a2660, a2661)?
    };
    nat_add(a2662, a2663)
}

pub fn f101(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2667 = {
            let a2664 = v2;
            let a2665 = 1u64;
            nat_sub(a2664, a2665)
        };
        let a2668 = match v0.clone() {
            Adt6::C0(_, h2666, _, _) => {
                h2666
            }
        };
        nat_mul(a2667, a2668)?
    };
    let a2670 = 0u64;
    let a2671 = match v0.clone() {
        Adt6::C0(h2669, _, _, _) => {
            h2669
        }
    };
    if if if if if if if if nat_lt(a2670, a2671) {
        let a2673 = match v0.clone() {
            Adt6::C0(h2672, _, _, _) => {
                h2672
            }
        };
        let a2674 = 2147483648u64;
        nat_lt(a2673, a2674)
    } else {
        false
    } {
        let a2676 = 0u64;
        let a2677 = match v0.clone() {
            Adt6::C0(_, h2675, _, _) => {
                h2675
            }
        };
        nat_lt(a2676, a2677)
    } else {
        false
    } {
        let a2679 = match v0.clone() {
            Adt6::C0(_, h2678, _, _) => {
                h2678
            }
        };
        let a2680 = 2147483648u64;
        nat_lt(a2679, a2680)
    } else {
        false
    } {
        let a2681 = v1;
        let a2682 = 2147483648u64;
        nat_lt(a2681, a2682)
    } else {
        false
    } {
        let a2683 = 0u64;
        let a2684 = v2;
        nat_lt(a2683, a2684)
    } else {
        false
    } {
        let a2685 = v3;
        let a2686 = 2147483648u64;
        nat_lt(a2685, a2686)
    } else {
        false
    } {
        let a2698 = {
            let a2693 = {
                let a2690 = v3;
                let a2691 = {
                    let a2688 = match v0.clone() {
                        Adt6::C0(h2687, _, _, _) => {
                            h2687
                        }
                    };
                    let a2689 = 1u64;
                    nat_sub(a2688, a2689)
                };
                nat_add(a2690, a2691)?
            };
            let a2694 = f121(match v0.clone() {
                Adt6::C0(_, _, h2692, _) => {
                    h2692
                }
            });
            nat_add(a2693, a2694)?
        };
        let a2699 = {
            let a2696 = 2147483647u64;
            let a2697 = f122(match v0.clone() {
                Adt6::C0(_, _, h2695, _) => {
                    h2695
                }
            });
            nat_add(a2696, a2697)?
        };
        nat_le(a2698, a2699)
    } else {
        false
    } {
        let a2701 = match v0 {
            Adt6::C0(_, _, h2700, _) => {
                h2700
            }
        };
        let a2702 = 2147483648u64;
        Ok(!nat_eq(a2701, a2702))
    } else {
        Ok(false)
    }
}

pub fn f102(v0: Adt7) -> R<u64> {
    let a2714 = if match v0.clone() {
        Adt7::C0(_, _, _, h2703, _) => {
            h2703
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2715 = {
        let a2712 = {
            let a2710 = {
                let m2706 = match v0.clone() {
                    Adt7::C0(h2705, _, _, _, _) => {
                        h2705
                    }
                };
                match m2706 {
                    Adt6::C0(h2704, _, _, _) => {
                        h2704
                    }
                }
            };
            let a2711 = {
                let m2709 = match v0.clone() {
                    Adt7::C0(_, h2708, _, _, _) => {
                        h2708
                    }
                };
                match m2709 {
                    Adt6::C0(h2707, _, _, _) => {
                        h2707
                    }
                }
            };
            nat_mul(a2710, a2711)?
        };
        let a2713 = f149(v0)?;
        nat_mul(a2712, a2713)?
    };
    nat_add(a2714, a2715)
}

pub fn f103(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f175(match v0.clone() {
        Adt2::C0(h2716, _, _, _, _, _, _, _, _, _, _) => {
            h2716
        }
    }, match v0.clone() {
        Adt2::C0(_, h2717, _, _, _, _, _, _, _, _, _) => {
            h2717
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2718, _, _, _, _, _, _, _, _) => {
            h2718
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2719, _, _, _, _, _, _, _) => {
            h2719
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2720, _, _, _, _, _, _) => {
            h2720
        }
    }, match v1.clone() {
        Adt2::C0(h2721, _, _, _, _, _, _, _, _, _, _) => {
            h2721
        }
    }, match v1.clone() {
        Adt2::C0(_, h2722, _, _, _, _, _, _, _, _, _) => {
            h2722
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2723, _, _, _, _, _, _, _, _) => {
            h2723
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2724, _, _, _, _, _, _, _) => {
            h2724
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2725, _, _, _, _, _, _) => {
            h2725
        }
    }, match v2.clone() {
        Adt2::C0(h2726, _, _, _, _, _, _, _, _, _, _) => {
            h2726
        }
    }, match v2.clone() {
        Adt2::C0(_, h2727, _, _, _, _, _, _, _, _, _) => {
            h2727
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2728, _, _, _, _, _, _, _, _) => {
            h2728
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2729, _, _, _, _, _, _, _) => {
            h2729
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2730, _, _, _, _, _, _) => {
            h2730
        }
    }) {
        let m2732 = f106(match v0.clone() {
            Adt2::C0(h2731, _, _, _, _, _, _, _, _, _, _) => {
                h2731
            }
        });
        match m2732 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a2738 = {
                    let a2735 = match v0.clone() {
                        Adt2::C0(_, h2733, _, _, _, _, _, _, _, _, _) => {
                            h2733
                        }
                    };
                    let a2736 = match v3.clone() {
                        Adt18::C0(_, h2734, _) => {
                            h2734
                        }
                    };
                    let a2737 = 0u64;
                    nat_rem(a2735, a2736, a2737)
                };
                let a2739 = 0u64;
                if if nat_eq(a2738, a2739) {
                    if f176(match v0.clone() {
                        Adt2::C0(_, h2740, _, _, _, _, _, _, _, _, _) => {
                            h2740
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2741, _, _, _, _, _, _, _, _, _) => {
                            h2741
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2742, _, _, _, _, _, _, _, _) => {
                            h2742
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2743, _, _, _, _, _, _, _) => {
                            h2743
                        }
                    }) {
                        true
                    } else {
                        let a2745 = 4u64;
                        let a2746 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2744, _, _, _, _, _) => {
                                h2744
                            }
                        };
                        if if nat_le(a2745, a2746) {
                            let a2748 = match v3.clone() {
                                Adt18::C0(_, h2747, _) => {
                                    h2747
                                }
                            };
                            let a2749 = 1u64;
                            if nat_eq(a2748, a2749) {
                                true
                            } else {
                                let a2752 = match v3.clone() {
                                    Adt18::C0(_, _, h2750) => {
                                        h2750
                                    }
                                };
                                let a2753 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2751, _, _, _, _, _) => {
                                        h2751
                                    }
                                };
                                nat_le(a2752, a2753)
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
                        Adt18::C0(_, h2754, _) => {
                            h2754
                        }
                    });
                    Ok(Adt10::C4(if f176(match v0.clone() {
                        Adt2::C0(_, h2755, _, _, _, _, _, _, _, _, _) => {
                            h2755
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2756, _, _, _, _, _, _, _, _, _) => {
                            h2756
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2757, _, _, _, _, _, _, _, _) => {
                            h2757
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2758, _, _, _, _, _, _, _) => {
                            h2758
                        }
                    }) {
                        0u64
                    } else {
                        f143(v4.clone(), f18(v1.clone(), 1u64))?
                    }, 4u64, v4, f18(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2759, _, _, _, _, _, _, _, _, _, _) => {
                            h2759
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

pub fn f104(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f97(match v0.clone() {
        Adt2::C0(h2760, _, _, _, _, _, _, _, _, _, _) => {
            h2760
        }
    }) {
        let a2762 = match v1.clone() {
            Adt2::C0(h2761, _, _, _, _, _, _, _, _, _, _) => {
                h2761
            }
        };
        let a2763 = 26u64;
        nat_eq(a2762, a2763)
    } else {
        false
    } {
        let a2765 = match v2.clone() {
            Adt2::C0(h2764, _, _, _, _, _, _, _, _, _, _) => {
                h2764
            }
        };
        let a2766 = 0u64;
        nat_eq(a2765, a2766)
    } else {
        false
    } {
        let a2769 = match v0.clone() {
            Adt2::C0(_, h2767, _, _, _, _, _, _, _, _, _) => {
                h2767
            }
        };
        let a2770 = match v2.clone() {
            Adt2::C0(_, h2768, _, _, _, _, _, _, _, _, _) => {
                h2768
            }
        };
        nat_eq(a2769, a2770)
    } else {
        false
    } {
        let a2773 = match v1.clone() {
            Adt2::C0(_, h2771, _, _, _, _, _, _, _, _, _) => {
                h2771
            }
        };
        let a2774 = match v0.clone() {
            Adt2::C0(_, _, h2772, _, _, _, _, _, _, _, _) => {
                h2772
            }
        };
        nat_le(a2773, a2774)
    } else {
        false
    } {
        let a2776 = match v0.clone() {
            Adt2::C0(_, _, _, h2775, _, _, _, _, _, _, _) => {
                h2775
            }
        };
        let a2777 = 1u64;
        nat_eq(a2776, a2777)
    } else {
        false
    } {
        let a2779 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2778, _, _, _, _, _, _) => {
                h2778
            }
        };
        let a2780 = 1u64;
        nat_eq(a2779, a2780)
    } else {
        false
    } {
        let a2782 = match v2.clone() {
            Adt2::C0(_, _, _, h2781, _, _, _, _, _, _, _) => {
                h2781
            }
        };
        let a2783 = 1u64;
        nat_eq(a2782, a2783)
    } else {
        false
    } {
        let a2785 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2784, _, _, _, _, _, _) => {
                h2784
            }
        };
        let a2786 = 1u64;
        nat_eq(a2785, a2786)
    } else {
        false
    } {
        let a2788 = match v1.clone() {
            Adt2::C0(_, _, h2787, _, _, _, _, _, _, _, _) => {
                h2787
            }
        };
        let a2789 = 1u64;
        nat_eq(a2788, a2789)
    } else {
        false
    } {
        let a2791 = match v1.clone() {
            Adt2::C0(_, _, _, h2790, _, _, _, _, _, _, _) => {
                h2790
            }
        };
        let a2792 = 1u64;
        nat_eq(a2791, a2792)
    } else {
        false
    } {
        let a2794 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2793, _, _, _, _, _, _) => {
                h2793
            }
        };
        let a2795 = 1u64;
        nat_eq(a2794, a2795)
    } else {
        false
    } {
        if f31(match v2.clone() {
            Adt2::C0(_, h2796, _, _, _, _, _, _, _, _, _) => {
                h2796
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2797, _, _, _, _, _, _, _, _) => {
                h2797
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2798, _, _, _, _, _, _, _) => {
                h2798
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2799, _, _, _, _, _, _) => {
                h2799
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2801 = match v1.clone() {
                Adt2::C0(_, h2800, _, _, _, _, _, _, _, _, _) => {
                    h2800
                }
            };
            let a2802 = 0u64;
            if nat_eq(a2801, a2802) {
                Ok(Adt10::C4(f14(v2.clone(), 1u64)?, 4u64, f18(v0, 1u64), f18(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2804 = match v0.clone() {
                    Adt2::C0(_, h2803, _, _, _, _, _, _, _, _, _) => {
                        h2803
                    }
                };
                let a2805 = 2147483647u64;
                if if if if if if nat_le(a2804, a2805) {
                    let a2807 = match v1.clone() {
                        Adt2::C0(_, h2806, _, _, _, _, _, _, _, _, _) => {
                            h2806
                        }
                    };
                    let a2808 = 2147483647u64;
                    nat_le(a2807, a2808)
                } else {
                    false
                } {
                    let a2810 = 4u64;
                    let a2811 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2809, _, _, _, _, _) => {
                            h2809
                        }
                    };
                    nat_le(a2810, a2811)
                } else {
                    false
                } {
                    let a2817 = {
                        let a2814 = match v2.clone() {
                            Adt2::C0(_, h2812, _, _, _, _, _, _, _, _, _) => {
                                h2812
                            }
                        };
                        let a2815 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2813, _, _, _, _, _) => {
                                h2813
                            }
                        };
                        nat_mul(a2814, a2815)?
                    };
                    let a2818 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2816, _, _, _, _) => {
                            h2816
                        }
                    };
                    nat_le(a2817, a2818)
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
                        let a2832 = {
                            let a2822 = {
                                let a2820 = 2u64;
                                let a2821 = match v1.clone() {
                                    Adt2::C0(_, h2819, _, _, _, _, _, _, _, _, _) => {
                                        h2819
                                    }
                                };
                                nat_mul(a2820, a2821)?
                            };
                            let a2823 = f14(v2.clone(), 1u64)?;
                            nat_add(a2822, a2823)?
                        };
                        let a2833 = {
                            let a2830 = match v1.clone() {
                                Adt2::C0(_, h2824, _, _, _, _, _, _, _, _, _) => {
                                    h2824
                                }
                            };
                            let a2831 = {
                                let a2828 = 1u64;
                                let a2829 = {
                                    let a2826 = 2u64;
                                    let a2827 = match v0.clone() {
                                        Adt2::C0(_, h2825, _, _, _, _, _, _, _, _, _) => {
                                            h2825
                                        }
                                    };
                                    nat_mul(a2826, a2827)?
                                };
                                nat_add(a2828, a2829)?
                            };
                            nat_mul(a2830, a2831)?
                        };
                        nat_add(a2832, a2833)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2834, _, _, _, _, _, _, _, _, _) => {
                            h2834
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2835, _, _, _, _, _, _, _, _, _) => {
                            h2835
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2836, _, _, _, _, _) => {
                            h2836
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2837, _, _, _, _) => {
                            h2837
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2838, _, _, _) => {
                            h2838
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2839, _, _) => {
                            h2839
                        }
                    }), f18(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2840, _, _, _, _, _, _, _, _, _, _) => {
                            h2840
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

pub fn f105(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2843 = match v1.clone() {
        Adt2::C0(h2841, _, _, _, _, _, _, _, _, _, _) => {
            h2841
        }
    };
    let a2844 = match v3.clone() {
        Adt2::C0(h2842, _, _, _, _, _, _, _, _, _, _) => {
            h2842
        }
    };
    if if nat_eq(a2843, a2844) {
        let a2845 = v0;
        let a2846 = 33u64;
        if nat_eq(a2845, a2846) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2848 = f106(match v1.clone() {
            Adt2::C0(h2847, _, _, _, _, _, _, _, _, _, _) => {
                h2847
            }
        });
        match m2848 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h2849, _) => {
                        h2849
                    }
                };
                let v6: u64 = f14(v1.clone(), v5)?;
                let a2854 = {
                    let a2851 = match v1.clone() {
                        Adt2::C0(_, h2850, _, _, _, _, _, _, _, _, _) => {
                            h2850
                        }
                    };
                    let a2852 = v5;
                    let a2853 = 1u64;
                    nat_rem(a2851, a2852, a2853)
                };
                let a2855 = 0u64;
                if if if nat_eq(a2854, a2855) {
                    let a2860 = {
                        let a2857 = match v3.clone() {
                            Adt2::C0(_, h2856, _, _, _, _, _, _, _, _, _) => {
                                h2856
                            }
                        };
                        let a2858 = v5;
                        let a2859 = 1u64;
                        nat_rem(a2857, a2858, a2859)
                    };
                    let a2861 = 0u64;
                    nat_eq(a2860, a2861)
                } else {
                    false
                } {
                    let a2862 = v6;
                    let a2863 = f14(v3.clone(), v5)?;
                    nat_eq(a2862, a2863)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h2864) => {
                            h2864
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

pub fn f106(v0: u64) -> Adt17 {
    let a2865 = v0;
    let a2866 = 0u64;
    if nat_eq(a2865, a2866) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a2867 = v0;
        let a2868 = 1u64;
        if nat_eq(a2867, a2868) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a2869 = v0;
            let a2870 = 2u64;
            if nat_eq(a2869, a2870) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a2871 = v0;
                let a2872 = 3u64;
                if nat_eq(a2871, a2872) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a2873 = v0;
                    let a2874 = 6u64;
                    if nat_eq(a2873, a2874) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2875 = v0;
                        let a2876 = 7u64;
                        if nat_eq(a2875, a2876) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2877 = v0;
                            let a2878 = 8u64;
                            if nat_eq(a2877, a2878) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2879 = v0;
                                let a2880 = 9u64;
                                if nat_eq(a2879, a2880) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2881 = v0;
                                    let a2882 = 10u64;
                                    if nat_eq(a2881, a2882) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2883 = v0;
                                        let a2884 = 11u64;
                                        if nat_eq(a2883, a2884) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2885 = v0;
                                            let a2886 = 12u64;
                                            if nat_eq(a2885, a2886) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2887 = v0;
                                                let a2888 = 13u64;
                                                if nat_eq(a2887, a2888) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2889 = v0;
                                                    let a2890 = 14u64;
                                                    if nat_eq(a2889, a2890) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2891 = v0;
                                                        let a2892 = 15u64;
                                                        if nat_eq(a2891, a2892) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2893 = v0;
                                                            let a2894 = 16u64;
                                                            if nat_eq(a2893, a2894) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2895 = v0;
                                                                let a2896 = 17u64;
                                                                if nat_eq(a2895, a2896) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2897 = v0;
                                                                    let a2898 = 18u64;
                                                                    if nat_eq(a2897, a2898) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2899 = v0;
                                                                        let a2900 = 19u64;
                                                                        if nat_eq(a2899, a2900) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2901 = v0;
                                                                            let a2902 = 20u64;
                                                                            if nat_eq(a2901, a2902) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2903 = v0;
                                                                                let a2904 = 21u64;
                                                                                if nat_eq(a2903, a2904) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2905 = v0;
                                                                                    let a2906 = 22u64;
                                                                                    if nat_eq(a2905, a2906) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2907 = v0;
                                                                                        let a2908 = 23u64;
                                                                                        if nat_eq(a2907, a2908) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2909 = v0;
                                                                                            let a2910 = 24u64;
                                                                                            if nat_eq(a2909, a2910) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2911 = v0;
                                                                                                let a2912 = 25u64;
                                                                                                if nat_eq(a2911, a2912) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2913 = v0;
                                                                                                    let a2914 = 26u64;
                                                                                                    if nat_eq(a2913, a2914) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2915 = v0;
                                                                                                        let a2916 = 27u64;
                                                                                                        if nat_eq(a2915, a2916) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2917 = v0;
                                                                                                            let a2918 = 28u64;
                                                                                                            if nat_eq(a2917, a2918) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2919 = v0;
                                                                                                                let a2920 = 29u64;
                                                                                                                if nat_eq(a2919, a2920) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2921 = v0;
                                                                                                                    let a2922 = 30u64;
                                                                                                                    if nat_eq(a2921, a2922) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2923 = v0;
                                                                                                                        let a2924 = 34u64;
                                                                                                                        if nat_eq(a2923, a2924) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2925 = v0;
                                                                                                                            let a2926 = 35u64;
                                                                                                                            if nat_eq(a2925, a2926) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2927 = v0;
                                                                                                                                let a2928 = 39u64;
                                                                                                                                if nat_eq(a2927, a2928) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2929 = v0;
                                                                                                                                    let a2930 = 40u64;
                                                                                                                                    if nat_eq(a2929, a2930) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2931 = v0;
                                                                                                                                        let a2932 = 41u64;
                                                                                                                                        if nat_eq(a2931, a2932) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2933 = v0;
                                                                                                                                            let a2934 = 42u64;
                                                                                                                                            if nat_eq(a2933, a2934) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2935 = v0;
                                                                                                                                                let a2936 = 43u64;
                                                                                                                                                if nat_eq(a2935, a2936) {
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

pub fn f107(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f31(v4, v5, v6, v7) {
        f31(v0, v1, v2, v3)
    } else {
        let a2940 = {
            let a2937 = v0;
            let a2938 = v4;
            let a2939 = 1u64;
            nat_rem(a2937, a2938, a2939)
        };
        let a2941 = 0u64;
        if if if nat_eq(a2940, a2941) {
            let a2945 = {
                let a2942 = v1;
                let a2943 = v5;
                let a2944 = 1u64;
                nat_rem(a2942, a2943, a2944)
            };
            let a2946 = 0u64;
            nat_eq(a2945, a2946)
        } else {
            false
        } {
            let a2950 = {
                let a2947 = v2;
                let a2948 = v6;
                let a2949 = 1u64;
                nat_rem(a2947, a2948, a2949)
            };
            let a2951 = 0u64;
            nat_eq(a2950, a2951)
        } else {
            false
        } {
            let a2955 = {
                let a2952 = v3;
                let a2953 = v7;
                let a2954 = 1u64;
                nat_rem(a2952, a2953, a2954)
            };
            let a2956 = 0u64;
            nat_eq(a2955, a2956)
        } else {
            false
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2957 = v0;
    let a2958 = v1;
    if nat_eq(a2957, a2958) {
        let a2961 = v4;
        let a2962 = {
            let a2959 = v2;
            let a2960 = v3;
            nat_add(a2959, a2960)?
        };
        Ok(nat_eq(a2961, a2962))
    } else {
        let a2963 = v2;
        let a2964 = v3;
        if nat_eq(a2963, a2964) {
            let a2965 = v4;
            let a2966 = v2;
            Ok(nat_eq(a2965, a2966))
        } else {
            Ok(false)
        }
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2969 = {
            let a2967 = v0;
            let a2968 = f122(v1);
            nat_add(a2967, a2968)?
        };
        let a2970 = f122(v2);
        nat_add(a2969, a2970)?
    };
    let v5: u64 = {
        let a2971 = f121(v1);
        let a2972 = f121(v2);
        nat_add(a2971, a2972)?
    };
    let a2973 = v1;
    let a2974 = 4294967295u64;
    if if nat_le(a2973, a2974) {
        let a2975 = v2;
        let a2976 = 4294967295u64;
        nat_le(a2975, a2976)
    } else {
        false
    } {
        let a2977 = v5;
        let a2978 = v4;
        if nat_le(a2977, a2978) {
            let a2981 = v3;
            let a2982 = {
                let a2979 = v4;
                let a2980 = v5;
                nat_sub(a2979, a2980)
            };
            Ok(nat_eq(a2981, a2982))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f110(v0: u64, v1: u64) -> R<bool> {
    let a2983 = v1;
    let a2984 = 4294967295u64;
    if nat_le(a2983, a2984) {
        let a2987 = {
            let a2985 = f122(v1);
            let a2986 = f121(v1);
            nat_add(a2985, a2986)?
        };
        let a2988 = v0;
        Ok(nat_lt(a2987, a2988))
    } else {
        Ok(false)
    }
}

pub fn f111(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f177(v0.clone(), false)?;
    let v6: u64 = f177(v0, true)?;
    let a2989 = v6;
    let a2990 = v1;
    if if nat_le(a2989, a2990) {
        let a2997 = {
            let a2995 = {
                let a2991 = v1;
                let a2992 = v5;
                nat_add(a2991, a2992)?
            };
            let a2996 = {
                let a2993 = v3;
                let a2994 = 0u64;
                if nat_eq(a2993, a2994) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2995, a2996)?
        };
        let a2998 = v2;
        nat_le(a2997, a2998)
    } else {
        false
    } {
        if v4 {
            let a3003 = {
                let a3001 = {
                    let a2999 = v1;
                    let a3000 = v5;
                    nat_add(a2999, a3000)?
                };
                let a3002 = v6;
                nat_sub(a3001, a3002)
            };
            let a3004 = v2;
            Ok(nat_lt(a3003, a3004))
        } else {
            let a3009 = {
                let a3007 = {
                    let a3005 = v1;
                    let a3006 = v5;
                    nat_add(a3005, a3006)?
                };
                let a3008 = v6;
                nat_sub(a3007, a3008)
            };
            let a3010 = v2;
            Ok(nat_le(a3009, a3010))
        }
    } else {
        Ok(false)
    }
}

pub fn f112(v0: u64, v1: u64) -> bool {
    let a3011 = v0;
    let a3012 = 0u64;
    if if nat_eq(a3011, a3012) {
        true
    } else {
        let a3013 = v0;
        let a3014 = 1u64;
        nat_eq(a3013, a3014)
    } {
        let a3015 = v1;
        let a3016 = 4294967296u64;
        nat_lt(a3015, a3016)
    } else {
        false
    }
}

pub fn f113(v0: u64, v1: u64) -> R<u64> {
    let a3021 = {
        let a3019 = v0;
        let a3020 = {
            let a3017 = v1;
            let a3018 = 1u64;
            nat_sub(a3017, a3018)
        };
        nat_add(a3019, a3020)?
    };
    let a3022 = v1;
    let a3023 = 0u64;
    Ok(nat_quot(a3021, a3022, a3023))
}

pub fn f114(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f31(match v0.clone() {
        Adt2::C0(_, h3024, _, _, _, _, _, _, _, _, _) => {
            h3024
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3025, _, _, _, _, _, _, _, _) => {
            h3025
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3026, _, _, _, _, _, _, _) => {
            h3026
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3027, _, _, _, _, _, _) => {
            h3027
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3030 = match v1.clone() {
                Adt2::C0(_, h3028, _, _, _, _, _, _, _, _, _) => {
                    h3028
                }
            };
            let a3031 = match v1.clone() {
                Adt2::C0(_, _, h3029, _, _, _, _, _, _, _, _) => {
                    h3029
                }
            };
            nat_mul(a3030, a3031)?
        };
        let v5: u64 = {
            let a3033 = v4;
            let a3034 = match v1.clone() {
                Adt2::C0(_, _, _, h3032, _, _, _, _, _, _, _) => {
                    h3032
                }
            };
            nat_mul(a3033, a3034)?
        };
        let a3047 = {
            let a3045 = {
                let a3036 = match v0.clone() {
                    Adt2::C0(_, h3035, _, _, _, _, _, _, _, _, _) => {
                        h3035
                    }
                };
                let a3037 = 1u64;
                nat_sub(a3036, a3037)
            };
            let a3046 = {
                let a3043 = f178(match v0.clone() {
                    Adt2::C0(_, _, h3038, _, _, _, _, _, _, _, _) => {
                        h3038
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3039, _, _, _, _, _, _, _, _, _) => {
                        h3039
                    }
                })?;
                let a3044 = f178(match v0 {
                    Adt2::C0(_, _, _, h3040, _, _, _, _, _, _, _) => {
                        h3040
                    }
                }, v2, {
                    let a3041 = v3;
                    let a3042 = v5;
                    nat_mul(a3041, a3042)?
                }, v4)?;
                nat_add(a3043, a3044)?
            };
            nat_add(a3045, a3046)?
        };
        let a3048 = f14(v1, 1u64)?;
        Ok(nat_lt(a3047, a3048))
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3049 = v0;
    let a3050 = 2147483648u64;
    if if if if nat_lt(a3049, a3050) {
        let a3051 = v1;
        let a3052 = 2147483648u64;
        nat_lt(a3051, a3052)
    } else {
        false
    } {
        let a3055 = {
            let a3053 = v0;
            let a3054 = v1;
            nat_mul(a3053, a3054)?
        };
        let a3056 = 2147483648u64;
        nat_lt(a3055, a3056)
    } else {
        false
    } {
        let a3061 = {
            let a3059 = {
                let a3057 = v0;
                let a3058 = 1u64;
                nat_sub(a3057, a3058)
            };
            let a3060 = v2;
            nat_mul(a3059, a3060)?
        };
        let a3062 = 2147483648u64;
        nat_lt(a3061, a3062)
    } else {
        false
    } {
        let a3067 = {
            let a3065 = {
                let a3063 = v1;
                let a3064 = 1u64;
                nat_sub(a3063, a3064)
            };
            let a3066 = v2;
            nat_mul(a3065, a3066)?
        };
        let a3068 = 2147483648u64;
        Ok(nat_lt(a3067, a3068))
    } else {
        Ok(false)
    }
}

pub fn f116(v0: u64) -> R<u64> {
    let a3071 = 3u64;
    let a3072 = {
        let a3069 = v0;
        let a3070 = 3u64;
        nat_mul(a3069, a3070)?
    };
    nat_add(a3071, a3072)
}

pub fn f117(v0: Adt3) -> R<u64> {
    if f31(match v0.clone() {
        Adt3::C0(h3073, _, _, _, _, _, _, _) => {
            h3073
        }
    }, match v0.clone() {
        Adt3::C0(_, h3074, _, _, _, _, _, _) => {
            h3074
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3075, _, _, _, _, _) => {
            h3075
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3076, _, _, _, _) => {
            h3076
        }
    }) {
        Ok(0u64)
    } else {
        let a3085 = match v0.clone() {
            Adt3::C0(h3077, _, _, _, _, _, _, _) => {
                h3077
            }
        };
        let a3086 = {
            let a3083 = match v0.clone() {
                Adt3::C0(_, h3078, _, _, _, _, _, _) => {
                    h3078
                }
            };
            let a3084 = {
                let a3081 = match v0.clone() {
                    Adt3::C0(_, _, h3079, _, _, _, _, _) => {
                        h3079
                    }
                };
                let a3082 = match v0 {
                    Adt3::C0(_, _, _, h3080, _, _, _, _) => {
                        h3080
                    }
                };
                nat_mul(a3081, a3082)?
            };
            nat_mul(a3083, a3084)?
        };
        nat_mul(a3085, a3086)
    }
}

pub fn f118(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3087 = v1;
    let a3088 = 13u64;
    if if nat_eq(a3087, a3088) {
        f31(match v0.clone() {
            Adt3::C0(h3089, _, _, _, _, _, _, _) => {
                h3089
            }
        }, match v0.clone() {
            Adt3::C0(_, h3090, _, _, _, _, _, _) => {
                h3090
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3091, _, _, _, _, _) => {
                h3091
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3092, _, _, _, _) => {
                h3092
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3098 = match v0.clone() {
                Adt3::C0(_, h3093, _, _, _, _, _, _) => {
                    h3093
                }
            };
            let a3099 = {
                let a3096 = match v0.clone() {
                    Adt3::C0(_, _, h3094, _, _, _, _, _) => {
                        h3094
                    }
                };
                let a3097 = match v0.clone() {
                    Adt3::C0(_, _, _, h3095, _, _, _, _) => {
                        h3095
                    }
                };
                nat_mul(a3096, a3097)?
            };
            nat_mul(a3098, a3099)?
        };
        let a3100 = v1;
        let a3101 = 13u64;
        if nat_eq(a3100, a3101) {
            let a3110 = 2u64;
            let a3111 = if f179(v0.clone(), v2) {
                let a3103 = v3;
                let a3104 = f180(match v0 {
                    Adt3::C0(h3102, _, _, _, _, _, _, _) => {
                        h3102
                    }
                }, false)?;
                nat_mul(a3103, a3104)?
            } else {
                let a3108 = v3;
                let a3109 = {
                    let a3106 = match v0 {
                        Adt3::C0(h3105, _, _, _, _, _, _, _) => {
                            h3105
                        }
                    };
                    let a3107 = 2u64;
                    nat_mul(a3106, a3107)?
                };
                nat_mul(a3108, a3109)?
            };
            nat_add(a3110, a3111)
        } else {
            let a3115 = v3;
            let a3116 = f180(match v0 {
                Adt3::C0(h3112, _, _, _, _, _, _, _) => {
                    h3112
                }
            }, {
                let a3113 = v1;
                let a3114 = 16u64;
                nat_eq(a3113, a3114)
            })?;
            nat_mul(a3115, a3116)
        }
    }
}

pub fn f119(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3117 = v0;
    let a3118 = 30u64;
    if if nat_eq(a3117, a3118) {
        let a3121 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3119, _, _, _, _) => {
                h3119
            }
        };
        let a3122 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3120, _, _, _, _, _) => {
                h3120
            }
        };
        nat_lt(a3121, a3122)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3123 = v3;
        match m3123 {
            None => {
                f181(v0, v1, match v2.clone() {
                    Adt2::C0(h3124, _, _, _, _, _, _, _, _, _, _) => {
                        h3124
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3125, _, _, _, _, _, _, _, _, _, _) => {
                        h3125
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3126, _, _, _, _, _, _, _, _, _) => {
                        h3126
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3127, _, _, _, _, _, _, _, _) => {
                        h3127
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3128, _, _, _, _, _, _, _) => {
                        h3128
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3129, _, _, _, _, _, _) => {
                        h3129
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3130, _, _, _, _, _, _, _, _, _) => {
                        h3130
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3131, _, _, _, _, _, _, _, _) => {
                        h3131
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3132, _, _, _, _, _, _, _) => {
                        h3132
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3133, _, _, _, _, _, _) => {
                        h3133
                    }
                })
            }
            Some(v5) => {
                f181(v0, v1, match v2.clone() {
                    Adt2::C0(h3134, _, _, _, _, _, _, _, _, _, _) => {
                        h3134
                    }
                }, match v5.clone() {
                    Adt2::C0(h3135, _, _, _, _, _, _, _, _, _, _) => {
                        h3135
                    }
                }, match v4.clone() {
                    Adt2::C0(h3136, _, _, _, _, _, _, _, _, _, _) => {
                        h3136
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3137, _, _, _, _, _, _, _, _, _) => {
                        h3137
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3138, _, _, _, _, _, _, _, _) => {
                        h3138
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3139, _, _, _, _, _, _, _) => {
                        h3139
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3140, _, _, _, _, _, _) => {
                        h3140
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3141, _, _, _, _, _, _, _, _, _) => {
                        h3141
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3142, _, _, _, _, _, _, _, _) => {
                        h3142
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3143, _, _, _, _, _, _, _) => {
                        h3143
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3144, _, _, _, _, _, _) => {
                        h3144
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3145, _, _, _, _, _, _, _, _, _) => {
                        h3145
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3146, _, _, _, _, _, _, _, _) => {
                        h3146
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3147, _, _, _, _, _, _, _) => {
                        h3147
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3148, _, _, _, _, _, _) => {
                        h3148
                    }
                })
            }
        }
    }
}

pub fn f120(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3161 = {
        let a3157 = {
            let a3153 = {
                let a3149 = v0;
                let a3150 = v4;
                nat_mul(a3149, a3150)?
            };
            let a3154 = {
                let a3151 = v1;
                let a3152 = v5;
                nat_mul(a3151, a3152)?
            };
            nat_add(a3153, a3154)?
        };
        let a3158 = {
            let a3155 = v2;
            let a3156 = v6;
            nat_mul(a3155, a3156)?
        };
        nat_add(a3157, a3158)?
    };
    let a3162 = {
        let a3159 = v3;
        let a3160 = v7;
        nat_mul(a3159, a3160)?
    };
    nat_add(a3161, a3162)
}

pub fn f121(v0: u64) -> u64 {
    let a3163 = v0;
    let a3164 = 2147483648u64;
    if nat_lt(a3163, a3164) {
        0u64
    } else {
        let a3165 = 4294967296u64;
        let a3166 = v0;
        nat_sub(a3165, a3166)
    }
}

pub fn f122(v0: u64) -> u64 {
    let a3167 = v0;
    let a3168 = 2147483648u64;
    if nat_lt(a3167, a3168) {
        v0
    } else {
        0u64
    }
}

pub fn f123(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3169 = v7;
        let a3170 = 6u64;
        if nat_eq(a3169, a3170) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f83(v3, v4), f83(v5, v6), f84(v7, v8), f182(v1), f182(v9))
}

pub fn f124(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3171 = v0;
    match m3171 {
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
        Adt4::C13(v30, v31, v32, v33) => {
            Adt11::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a3172 = 13u64;
            let a3173 = v34;
            if nat_le(a3172, a3173) {
                Adt11::C7(v34, v1, v2)
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
        Adt4::C27(v54) => {
            Adt11::C2(match v54.clone() {
                Adt5::C0(_, _, _, _, _, h3174, _, _) => {
                    h3174
                }
            }, match v54 {
                Adt5::C0(_, _, _, _, _, _, _, h3175) => {
                    h3175
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
    }
}

pub fn f125(v0: Adt4) -> u64 {
    let m3176 = v0;
    match m3176 {
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
        Adt4::C28(_) => {
            0u64
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, _) => {
            0u64
        }
    }
}

pub fn f126(v0: Adt4, v1: u64) -> u64 {
    let m3177 = v0;
    match m3177 {
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
            let a3178 = v1;
            let a3179 = 6u64;
            if nat_eq(a3178, a3179) {
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
        Adt4::C26(v54, _, _, _) => {
            let a3188 = v1;
            let a3189 = 6u64;
            if if if nat_eq(a3188, a3189) {
                true
            } else {
                let a3190 = v1;
                let a3191 = 7u64;
                nat_eq(a3190, a3191)
            } {
                let a3192 = v54;
                let a3193 = 30u64;
                nat_eq(a3192, a3193)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v53) => {
            match v53 {
                Adt5::C0(_, _, _, _, _, _, _, h3187) => {
                    h3187
                }
            }
        }
        Adt4::C28(v49) => {
            let a3180 = v1;
            let a3181 = 6u64;
            if nat_eq(a3180, a3181) {
                match v49 {
                    Adt7::C0(_, _, _, _, h3182) => {
                        h3182
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v52) => {
            let a3183 = v1;
            let a3184 = 6u64;
            if if nat_eq(a3183, a3184) {
                true
            } else {
                let a3185 = v1;
                let a3186 = 7u64;
                nat_eq(a3185, a3186)
            } {
                v52
            } else {
                0u64
            }
        }
    }
}

pub fn f127(v0: Adt4, v1: u64) -> Adt11 {
    let a3194 = v1;
    let a3195 = 4u64;
    if nat_lt(a3194, a3195) {
        Adt11::C0(v1)
    } else {
        let a3196 = v1;
        let a3197 = 4u64;
        if nat_eq(a3196, a3197) {
            Adt11::C1(0u64)
        } else {
            let a3198 = v1;
            let a3199 = 5u64;
            if nat_eq(a3198, a3199) {
                Adt11::C1(3u64)
            } else {
                let a3200 = v1;
                let a3201 = 6u64;
                if nat_eq(a3200, a3201) {
                    Adt11::C2(f126(v0, v1), 0u64, false)
                } else {
                    let a3202 = v1;
                    let a3203 = 7u64;
                    if nat_eq(a3202, a3203) {
                        Adt11::C2(0u64, f126(v0, v1), false)
                    } else {
                        let a3204 = v1;
                        let a3205 = 16u64;
                        if nat_lt(a3204, a3205) {
                            Adt11::C1({
                                let a3206 = v1;
                                let a3207 = 8u64;
                                nat_sub(a3206, a3207)
                            })
                        } else {
                            let a3208 = v1;
                            let a3209 = 20u64;
                            if nat_lt(a3208, a3209) {
                                Adt11::C5({
                                    let a3210 = v1;
                                    let a3211 = 16u64;
                                    nat_sub(a3210, a3211)
                                })
                            } else {
                                let a3212 = v1;
                                let a3213 = 20u64;
                                if nat_eq(a3212, a3213) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3214 = v1;
                                    let a3215 = 21u64;
                                    if nat_eq(a3214, a3215) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3216 = v1;
                                        let a3217 = 22u64;
                                        if nat_eq(a3216, a3217) {
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

pub fn f128(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3219 = v0;
        let a3220 = match v1.clone() {
            Adt3::C0(h3218, _, _, _, _, _, _, _) => {
                h3218
            }
        };
        let a3221 = 0u64;
        nat_quot(a3219, a3220, a3221)
    };
    let v3: u64 = {
        let a3223 = v2;
        let a3224 = match v1.clone() {
            Adt3::C0(_, h3222, _, _, _, _, _, _) => {
                h3222
            }
        };
        let a3225 = 0u64;
        nat_quot(a3223, a3224, a3225)
    };
    f120({
        let a3227 = v0;
        let a3228 = match v1.clone() {
            Adt3::C0(h3226, _, _, _, _, _, _, _) => {
                h3226
            }
        };
        let a3229 = 0u64;
        nat_rem(a3227, a3228, a3229)
    }, {
        let a3231 = v2;
        let a3232 = match v1.clone() {
            Adt3::C0(_, h3230, _, _, _, _, _, _) => {
                h3230
            }
        };
        let a3233 = 0u64;
        nat_rem(a3231, a3232, a3233)
    }, {
        let a3235 = v3;
        let a3236 = match v1.clone() {
            Adt3::C0(_, _, h3234, _, _, _, _, _) => {
                h3234
            }
        };
        let a3237 = 0u64;
        nat_rem(a3235, a3236, a3237)
    }, {
        let a3239 = v3;
        let a3240 = match v1.clone() {
            Adt3::C0(_, _, h3238, _, _, _, _, _) => {
                h3238
            }
        };
        let a3241 = 0u64;
        nat_quot(a3239, a3240, a3241)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3242, _, _, _) => {
            h3242
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3243, _, _) => {
            h3243
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3244, _) => {
            h3244
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3245) => {
            h3245
        }
    })
}

pub fn f129(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3263 = {
        let a3258 = f183(v2, {
            let a3249 = {
                let a3246 = v0;
                let a3247 = v1;
                let a3248 = 0u64;
                nat_quot(a3246, a3247, a3248)
            };
            let a3250 = v2;
            let a3251 = 0u64;
            nat_rem(a3249, a3250, a3251)
        }, {
            let a3255 = {
                let a3252 = v0;
                let a3253 = v1;
                let a3254 = 0u64;
                nat_quot(a3252, a3253, a3254)
            };
            let a3256 = v2;
            let a3257 = 0u64;
            nat_quot(a3255, a3256, a3257)
        })?;
        let a3259 = v1;
        nat_mul(a3258, a3259)?
    };
    let a3264 = {
        let a3260 = v0;
        let a3261 = v1;
        let a3262 = 0u64;
        nat_rem(a3260, a3261, a3262)
    };
    nat_add(a3263, a3264)
}

pub fn f130(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3266 = v0;
        let a3267 = match v2.clone() {
            Adt3::C0(h3265, _, _, _, _, _, _, _) => {
                h3265
            }
        };
        let a3268 = 0u64;
        nat_quot(a3266, a3267, a3268)
    };
    let v4: u64 = {
        let a3270 = v3;
        let a3271 = match v2.clone() {
            Adt3::C0(_, h3269, _, _, _, _, _, _) => {
                h3269
            }
        };
        let a3272 = 0u64;
        nat_quot(a3270, a3271, a3272)
    };
    f184({
        let a3274 = v0;
        let a3275 = match v2.clone() {
            Adt3::C0(h3273, _, _, _, _, _, _, _) => {
                h3273
            }
        };
        let a3276 = 0u64;
        nat_rem(a3274, a3275, a3276)
    }, {
        let a3278 = v3;
        let a3279 = match v2.clone() {
            Adt3::C0(_, h3277, _, _, _, _, _, _) => {
                h3277
            }
        };
        let a3280 = 0u64;
        nat_rem(a3278, a3279, a3280)
    }, {
        let a3282 = v4;
        let a3283 = match v2.clone() {
            Adt3::C0(_, _, h3281, _, _, _, _, _) => {
                h3281
            }
        };
        let a3284 = 0u64;
        nat_rem(a3282, a3283, a3284)
    }, {
        let a3286 = v4;
        let a3287 = match v2 {
            Adt3::C0(_, _, h3285, _, _, _, _, _) => {
                h3285
            }
        };
        let a3288 = 0u64;
        nat_quot(a3286, a3287, a3288)
    }, match v1.clone() {
        Adt3::C0(h3289, _, _, _, _, _, _, _) => {
            h3289
        }
    }, match v1.clone() {
        Adt3::C0(_, h3290, _, _, _, _, _, _) => {
            h3290
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3291, _, _, _, _, _) => {
            h3291
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3292, _, _, _, _) => {
            h3292
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3293, _, _, _) => {
            h3293
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3294, _, _) => {
            h3294
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3295, _) => {
            h3295
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3296) => {
            h3296
        }
    })
}

pub fn f131(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3298 = v0;
        let a3299 = match v3.clone() {
            Adt3::C0(h3297, _, _, _, _, _, _, _) => {
                h3297
            }
        };
        let a3300 = 0u64;
        nat_quot(a3298, a3299, a3300)
    };
    let v6: u64 = {
        let a3302 = v5;
        let a3303 = match v3.clone() {
            Adt3::C0(_, h3301, _, _, _, _, _, _) => {
                h3301
            }
        };
        let a3304 = 0u64;
        nat_quot(a3302, a3303, a3304)
    };
    let v7: u64 = {
        let a3306 = v0;
        let a3307 = match v3.clone() {
            Adt3::C0(h3305, _, _, _, _, _, _, _) => {
                h3305
            }
        };
        let a3308 = 0u64;
        nat_rem(a3306, a3307, a3308)
    };
    let v8: u64 = {
        let a3310 = v5;
        let a3311 = match v3.clone() {
            Adt3::C0(_, h3309, _, _, _, _, _, _) => {
                h3309
            }
        };
        let a3312 = 0u64;
        nat_rem(a3310, a3311, a3312)
    };
    let v9: u64 = {
        let a3314 = v6;
        let a3315 = match v3.clone() {
            Adt3::C0(_, _, h3313, _, _, _, _, _) => {
                h3313
            }
        };
        let a3316 = 0u64;
        nat_rem(a3314, a3315, a3316)
    };
    let v10: u64 = {
        let a3318 = v6;
        let a3319 = match v3.clone() {
            Adt3::C0(_, _, h3317, _, _, _, _, _) => {
                h3317
            }
        };
        let a3320 = 0u64;
        nat_quot(a3318, a3319, a3320)
    };
    let v11: u64 = {
        let a3321 = v4;
        let a3322 = 0u64;
        if nat_eq(a3321, a3322) {
            match v1.clone() {
                Adt3::C0(h3323, _, _, _, _, _, _, _) => {
                    h3323
                }
            }
        } else {
            let a3324 = v4;
            let a3325 = 1u64;
            if nat_eq(a3324, a3325) {
                match v1.clone() {
                    Adt3::C0(_, h3326, _, _, _, _, _, _) => {
                        h3326
                    }
                }
            } else {
                let a3327 = v4;
                let a3328 = 2u64;
                if nat_eq(a3327, a3328) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3329, _, _, _, _, _) => {
                            h3329
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3330, _, _, _, _) => {
                            h3330
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3337 = v11;
        let a3338 = {
            let a3331 = v4;
            let a3332 = 0u64;
            if nat_eq(a3331, a3332) {
                v7
            } else {
                let a3333 = v4;
                let a3334 = 1u64;
                if nat_eq(a3333, a3334) {
                    v8
                } else {
                    let a3335 = v4;
                    let a3336 = 2u64;
                    if nat_eq(a3335, a3336) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3337, a3338)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f120(if if v12 {
        let a3339 = v4;
        let a3340 = 0u64;
        nat_eq(a3339, a3340)
    } else {
        false
    } {
        let a3341 = v7;
        let a3342 = v11;
        nat_sub(a3341, a3342)
    } else {
        v7
    }, if if v12 {
        let a3343 = v4;
        let a3344 = 1u64;
        nat_eq(a3343, a3344)
    } else {
        false
    } {
        let a3345 = v8;
        let a3346 = v11;
        nat_sub(a3345, a3346)
    } else {
        v8
    }, if if v12 {
        let a3347 = v4;
        let a3348 = 2u64;
        nat_eq(a3347, a3348)
    } else {
        false
    } {
        let a3349 = v9;
        let a3350 = v11;
        nat_sub(a3349, a3350)
    } else {
        v9
    }, if if v12 {
        let a3351 = v4;
        let a3352 = 3u64;
        nat_eq(a3351, a3352)
    } else {
        false
    } {
        let a3353 = v10;
        let a3354 = v11;
        nat_sub(a3353, a3354)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3355, _, _, _) => {
            h3355
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3356, _, _) => {
            h3356
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3357, _) => {
            h3357
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3358) => {
            h3358
        }
    })?, f128(v0, v3)?))
}

pub fn f132(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f65(v0, v2.clone(), 0u64);
    let v9: u64 = f65(v0, v2.clone(), 1u64);
    let v10: u64 = f65(v0, v2.clone(), 2u64);
    let v11: u64 = f65(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f64(v1.clone(), f185(v8, v3, match v1.clone() {
            Adt3::C0(h3359, _, _, _, _, _, _, _) => {
                h3359
            }
        })?, f185(v9, v4, match v1.clone() {
            Adt3::C0(_, h3360, _, _, _, _, _, _) => {
                h3360
            }
        })?, f185(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3361, _, _, _, _, _) => {
                h3361
            }
        })?, f185(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3362, _, _, _, _) => {
                h3362
            }
        })?)?, f128(v0, v2)?))
    } else {
        if if if if f186(v8, v3, match v1.clone() {
            Adt3::C0(h3363, _, _, _, _, _, _, _) => {
                h3363
            }
        })? {
            f186(v9, v4, match v1.clone() {
                Adt3::C0(_, h3364, _, _, _, _, _, _) => {
                    h3364
                }
            })?
        } else {
            false
        } {
            f186(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3365, _, _, _, _, _) => {
                    h3365
                }
            })?
        } else {
            false
        } {
            f186(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3366, _, _, _, _) => {
                    h3366
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f64(v1, f187(v8, v3)?, f187(v9, v4)?, f187(v10, v5)?, f187(v11, v6)?)?, f128(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f128(v0, v2)?))
        }
    }
}

pub fn f133(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 1u64);
    let v6: u64 = f65(v0, v2.clone(), 2u64);
    let v7: u64 = f65(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f64(v1.clone(), f188(v4, v3, match v1 {
        Adt3::C0(h3367, _, _, _, _, _, _, _) => {
            h3367
        }
    })?, v5, v6, v7)?, f128(v0, v2)?))
}

pub fn f134(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3368 = v0;
    let a3369 = v3;
    if nat_lt(a3368, a3369) {
        Ok(Adt9::C1(false, {
            let a3370 = v0;
            let a3371 = 4u64;
            nat_mul(a3370, a3371)?
        }, {
            let a3372 = v0;
            let a3373 = 4u64;
            nat_mul(a3372, a3373)?
        }))
    } else {
        let v6: u64 = {
            let a3374 = v0;
            let a3375 = v3;
            nat_sub(a3374, a3375)
        };
        let v7: u64 = f128(v6, v1)?;
        let v8: u64 = f189(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3376 = v0;
    let a3377 = v3;
    if nat_lt(a3376, a3377) {
        Ok(Adt9::C1(true, 0u64, f128(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3378 = v0;
            let a3379 = v3;
            nat_sub(a3378, a3379)
        };
        let v5: u64 = f130(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f128(v4, v1)?, 3u64, v5))
    }
}

pub fn f136(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3381 = v1;
        let a3382 = match v3.clone() {
            Adt3::C0(h3380, _, _, _, _, _, _, _) => {
                h3380
            }
        };
        let a3383 = 0u64;
        nat_rem(a3381, a3382, a3383)
    };
    let v5: u64 = {
        let a3389 = {
            let a3385 = v1;
            let a3386 = match v3.clone() {
                Adt3::C0(h3384, _, _, _, _, _, _, _) => {
                    h3384
                }
            };
            let a3387 = 0u64;
            nat_quot(a3385, a3386, a3387)
        };
        let a3390 = match v3.clone() {
            Adt3::C0(_, h3388, _, _, _, _, _, _) => {
                h3388
            }
        };
        let a3391 = 0u64;
        nat_rem(a3389, a3390, a3391)
    };
    let v6: bool = f190(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f130(v1, v2, v3.clone())?
    } else {
        0u64
    }, f128(v1, v3)?))
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3392 = v3;
        let a3393 = 8u64;
        nat_add(a3392, a3393)?
    }, 0u64, f128(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f130(v0, v5, v2.clone())?
    }, 3u64, f128(v0, v2)?))
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3395 = v0;
        let a3396 = match v2.clone() {
            Adt3::C0(h3394, _, _, _, _, _, _, _) => {
                h3394
            }
        };
        let a3397 = 0u64;
        nat_rem(a3395, a3396, a3397)
    };
    let v7: u64 = {
        let a3399 = v0;
        let a3400 = match v2.clone() {
            Adt3::C0(h3398, _, _, _, _, _, _, _) => {
                h3398
            }
        };
        let a3401 = 0u64;
        nat_quot(a3399, a3400, a3401)
    };
    let v8: u64 = {
        let a3403 = v7;
        let a3404 = match v2.clone() {
            Adt3::C0(_, h3402, _, _, _, _, _, _) => {
                h3402
            }
        };
        let a3405 = 0u64;
        nat_rem(a3403, a3404, a3405)
    };
    let v9: u64 = {
        let a3407 = v7;
        let a3408 = match v2.clone() {
            Adt3::C0(_, h3406, _, _, _, _, _, _) => {
                h3406
            }
        };
        let a3409 = 0u64;
        nat_quot(a3407, a3408, a3409)
    };
    let v10: u64 = {
        let a3411 = v9;
        let a3412 = match v2.clone() {
            Adt3::C0(_, _, h3410, _, _, _, _, _) => {
                h3410
            }
        };
        let a3413 = 0u64;
        nat_rem(a3411, a3412, a3413)
    };
    let v11: u64 = {
        let a3415 = v9;
        let a3416 = match v2.clone() {
            Adt3::C0(_, _, h3414, _, _, _, _, _) => {
                h3414
            }
        };
        let a3417 = 0u64;
        nat_quot(a3415, a3416, a3417)
    };
    let v12: u64 = f128(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3426 = {
                let a3421 = {
                    let a3418 = v10;
                    let a3419 = v4;
                    let a3420 = 0u64;
                    nat_quot(a3418, a3419, a3420)
                };
                let a3422 = v5;
                nat_mul(a3421, a3422)?
            };
            let a3427 = {
                let a3423 = v8;
                let a3424 = v4;
                let a3425 = 0u64;
                nat_quot(a3423, a3424, a3425)
            };
            nat_add(a3426, a3427)?
        };
        let v14: u64 = {
            let a3447 = v6;
            let a3448 = {
                let a3445 = match v1.clone() {
                    Adt3::C0(h3428, _, _, _, _, _, _, _) => {
                        h3428
                    }
                };
                let a3446 = {
                    let a3443 = {
                        let a3429 = v8;
                        let a3430 = v4;
                        let a3431 = 0u64;
                        nat_rem(a3429, a3430, a3431)
                    };
                    let a3444 = {
                        let a3441 = match v1.clone() {
                            Adt3::C0(_, h3432, _, _, _, _, _, _) => {
                                h3432
                            }
                        };
                        let a3442 = {
                            let a3439 = {
                                let a3433 = v10;
                                let a3434 = v4;
                                let a3435 = 0u64;
                                nat_rem(a3433, a3434, a3435)
                            };
                            let a3440 = {
                                let a3437 = match v1.clone() {
                                    Adt3::C0(_, _, h3436, _, _, _, _, _) => {
                                        h3436
                                    }
                                };
                                let a3438 = v13;
                                nat_mul(a3437, a3438)?
                            };
                            nat_add(a3439, a3440)?
                        };
                        nat_mul(a3441, a3442)?
                    };
                    nat_add(a3443, a3444)?
                };
                nat_mul(a3445, a3446)?
            };
            nat_add(a3447, a3448)?
        };
        Ok(Adt9::C1(false, f128(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3454 = {
                let a3452 = {
                    let a3449 = v11;
                    let a3450 = v5;
                    let a3451 = 0u64;
                    nat_rem(a3449, a3450, a3451)
                };
                let a3453 = v4;
                nat_mul(a3452, a3453)?
            };
            let a3455 = v8;
            nat_add(a3454, a3455)?
        };
        let v16: u64 = {
            let a3461 = {
                let a3459 = {
                    let a3456 = v11;
                    let a3457 = v5;
                    let a3458 = 0u64;
                    nat_quot(a3456, a3457, a3458)
                };
                let a3460 = v4;
                nat_mul(a3459, a3460)?
            };
            let a3462 = v10;
            nat_add(a3461, a3462)?
        };
        let a3464 = v15;
        let a3465 = match v1.clone() {
            Adt3::C0(_, h3463, _, _, _, _, _, _) => {
                h3463
            }
        };
        if if nat_lt(a3464, a3465) {
            let a3467 = v16;
            let a3468 = match v1.clone() {
                Adt3::C0(_, _, h3466, _, _, _, _, _) => {
                    h3466
                }
            };
            nat_lt(a3467, a3468)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f128({
                let a3477 = v6;
                let a3478 = {
                    let a3475 = match v1.clone() {
                        Adt3::C0(h3469, _, _, _, _, _, _, _) => {
                            h3469
                        }
                    };
                    let a3476 = {
                        let a3473 = v15;
                        let a3474 = {
                            let a3471 = match v1.clone() {
                                Adt3::C0(_, h3470, _, _, _, _, _, _) => {
                                    h3470
                                }
                            };
                            let a3472 = v16;
                            nat_mul(a3471, a3472)?
                        };
                        nat_add(a3473, a3474)?
                    };
                    nat_mul(a3475, a3476)?
                };
                nat_add(a3477, a3478)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f128(v0, v2.clone())?;
    let v4: u64 = f128(v0, v1.clone())?;
    let a3483 = {
        let a3480 = v0;
        let a3481 = match v1 {
            Adt3::C0(h3479, _, _, _, _, _, _, _) => {
                h3479
            }
        };
        let a3482 = 0u64;
        nat_rem(a3480, a3481, a3482)
    };
    let a3484 = 0u64;
    if nat_eq(a3483, a3484) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f128({
            let a3485 = v0;
            let a3486 = 1u64;
            nat_sub(a3485, a3486)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3487 = v3;
    let a3488 = 13u64;
    if nat_eq(a3487, a3488) {
        f191(v0, v1, v4)
    } else {
        let v5: u64 = f180(match v1.clone() {
            Adt3::C0(h3489, _, _, _, _, _, _, _) => {
                h3489
            }
        }, {
            let a3490 = v3;
            let a3491 = 16u64;
            nat_eq(a3490, a3491)
        })?;
        let v6: u64 = {
            let a3492 = v0;
            let a3493 = v5;
            let a3494 = 0u64;
            nat_quot(a3492, a3493, a3494)
        };
        let v7: u64 = {
            let a3495 = v0;
            let a3496 = v5;
            let a3497 = 0u64;
            nat_rem(a3495, a3496, a3497)
        };
        let v8: u64 = {
            let a3501 = 1u64;
            let a3502 = {
                let a3499 = match v1.clone() {
                    Adt3::C0(h3498, _, _, _, _, _, _, _) => {
                        h3498
                    }
                };
                let a3500 = 2u64;
                nat_mul(a3499, a3500)?
            };
            nat_add(a3501, a3502)?
        };
        let a3503 = v7;
        let a3504 = v8;
        if nat_lt(a3503, a3504) {
            f192(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f128(v6, v2)?;
            let a3505 = v3;
            let a3506 = 14u64;
            if nat_eq(a3505, a3506) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3507 = v7;
                let a3508 = v8;
                if nat_eq(a3507, a3508) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3511 = v7;
                    let a3512 = {
                        let a3509 = v8;
                        let a3510 = 1u64;
                        nat_add(a3509, a3510)?
                    };
                    if nat_eq(a3511, a3512) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3513, _, _, _, _, _, _, _) => {
                                h3513
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

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3514 = v0;
    let a3515 = 0u64;
    if nat_eq(a3514, a3515) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3522 = {
            let a3516 = v0;
            let a3517 = 1u64;
            nat_add(a3516, a3517)?
        };
        let a3523 = {
            let a3520 = 2u64;
            let a3521 = {
                let a3518 = v3;
                let a3519 = 2u64;
                nat_mul(a3518, a3519)?
            };
            nat_add(a3520, a3521)?
        };
        if nat_eq(a3522, a3523) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3524 = v0;
                let a3525 = 1u64;
                nat_sub(a3524, a3525)
            };
            let v5: u64 = {
                let a3526 = v4;
                let a3527 = 2u64;
                let a3528 = 0u64;
                nat_quot(a3526, a3527, a3528)
            };
            let a3532 = {
                let a3529 = v4;
                let a3530 = 2u64;
                let a3531 = 0u64;
                nat_rem(a3529, a3530, a3531)
            };
            let a3533 = 0u64;
            if nat_eq(a3532, a3533) {
                Ok(Adt9::C4(17u64, 0u64, f128(v5, v1)?, 4u64, f128(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3534 = v0;
    let a3535 = f24(v1, v2, v3)?;
    if nat_lt(a3534, a3535) {
        let a3536 = v0;
        let a3537 = v1;
        if nat_lt(a3536, a3537) {
            Ok(Adt9::C1(false, {
                let a3538 = 4u64;
                let a3539 = v0;
                nat_mul(a3538, a3539)?
            }, {
                let a3540 = 4u64;
                let a3541 = v0;
                nat_mul(a3540, a3541)?
            }))
        } else {
            let v4: u64 = {
                let a3542 = v0;
                let a3543 = v1;
                nat_sub(a3542, a3543)
            };
            let v5: u64 = f193(v4, v2)?;
            let a3544 = v5;
            let a3545 = 2u64;
            if nat_lt(a3544, a3545) {
                Ok(Adt9::C4(6u64, {
                    let a3546 = v5;
                    let a3547 = 0u64;
                    if nat_eq(a3546, a3547) {
                        4u64
                    } else {
                        5u64
                    }
                }, f194(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f195(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f196(v4, v2)? {
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

pub fn f143(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3555 = {
        let a3548 = 2u64;
        let a3549 = f117(v1.clone())?;
        nat_mul(a3548, a3549)?
    };
    let a3556 = {
        let a3553 = f117(v1)?;
        let a3554 = {
            let a3551 = 1u64;
            let a3552 = match v0 {
                Adt3::C0(h3550, _, _, _, _, _, _, _) => {
                    h3550
                }
            };
            nat_add(a3551, a3552)?
        };
        nat_mul(a3553, a3554)?
    };
    nat_add(a3555, a3556)
}

pub fn f144(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f128(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f145(v0: u64, v1: u64) -> bool {
    let a3557 = v1;
    let a3558 = 2147483647u64;
    if nat_le(a3557, a3558) {
        let a3559 = v1;
        let a3560 = v0;
        nat_lt(a3559, a3560)
    } else {
        false
    }
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3561 = f106(v0);
    match m3561 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a3563 = v2;
                let a3564 = match v7.clone() {
                    Adt18::C0(_, h3562, _) => {
                        h3562
                    }
                };
                nat_mul(a3563, a3564)?
            };
            let m3577 = f197(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3565, _, _, _, _, _, _, _) => {
                    h3565
                }
            }, match v5.clone() {
                Adt3::C0(_, h3566, _, _, _, _, _, _) => {
                    h3566
                }
            }, match v5 {
                Adt3::C0(_, _, h3567, _, _, _, _, _) => {
                    h3567
                }
            }, {
                let a3570 = match v3.clone() {
                    Adt3::C0(h3568, _, _, _, _, _, _, _) => {
                        h3568
                    }
                };
                let a3571 = match v7.clone() {
                    Adt18::C0(_, h3569, _) => {
                        h3569
                    }
                };
                nat_mul(a3570, a3571)?
            }, match v3.clone() {
                Adt3::C0(_, h3572, _, _, _, _, _, _) => {
                    h3572
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3573, _, _, _) => {
                    h3573
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3574, _, _) => {
                    h3574
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3575, _) => {
                    h3575
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3576) => {
                    h3576
                }
            })?;
            match m3577 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3578 = v0;
                    let a3579 = 0u64;
                    if if nat_eq(a3578, a3579) {
                        true
                    } else {
                        let a3580 = v0;
                        let a3581 = 26u64;
                        nat_eq(a3580, a3581)
                    } {
                        Ok(Adt9::C1(false, v9, f128({
                            let a3585 = {
                                let a3583 = v1;
                                let a3584 = match v4.clone() {
                                    Adt3::C0(h3582, _, _, _, _, _, _, _) => {
                                        h3582
                                    }
                                };
                                nat_mul(a3583, a3584)?
                            };
                            let a3586 = v8;
                            nat_add(a3585, a3586)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h3587) => {
                                h3587
                            }
                        }, f128({
                            let a3591 = {
                                let a3589 = v1;
                                let a3590 = match v4.clone() {
                                    Adt3::C0(h3588, _, _, _, _, _, _, _) => {
                                        h3588
                                    }
                                };
                                nat_mul(a3589, a3590)?
                            };
                            let a3592 = v8;
                            nat_add(a3591, a3592)?
                        }, v4.clone())?, {
                            let a3599 = {
                                let a3597 = {
                                    let a3594 = match v7 {
                                        Adt18::C0(_, h3593, _) => {
                                            h3593
                                        }
                                    };
                                    let a3595 = 1u64;
                                    nat_sub(a3594, a3595)
                                };
                                let a3598 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3596, _, _, _) => {
                                        h3596
                                    }
                                };
                                nat_mul(a3597, a3598)?
                            };
                            let a3600 = 4u64;
                            nat_add(a3599, a3600)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3601, _, _, _) => {
                                h3601
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f147(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3602 = v1;
    let a3603 = 0u64;
    if if nat_eq(a3602, a3603) {
        true
    } else {
        let a3604 = v2;
        let a3605 = 0u64;
        nat_eq(a3604, a3605)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f148(v0: u64, v1: u64) -> bool {
    let a3606 = v0;
    let a3607 = 4294967295u64;
    if nat_eq(a3606, a3607) {
        true
    } else {
        let a3608 = v0;
        let a3609 = 2147483648u64;
        if nat_lt(a3608, a3609) {
            let a3610 = v0;
            let a3611 = v1;
            nat_lt(a3610, a3611)
        } else {
            false
        }
    }
}

pub fn f149(v0: Adt7) -> R<u64> {
    let a3616 = if match v0.clone() {
        Adt7::C0(_, _, _, h3612, _) => {
            h3612
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3617 = {
        let a3614 = match v0 {
            Adt7::C0(_, _, _, _, h3613) => {
                h3613
            }
        };
        let a3615 = 1u64;
        if nat_eq(a3614, a3615) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3616, a3617)
}

pub fn f150(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3622 = {
            let a3619 = v2;
            let a3620 = match v0.clone() {
                Adt6::C0(_, h3618, _, _) => {
                    h3618
                }
            };
            nat_mul(a3619, a3620)?
        };
        let a3623 = f121(match v0.clone() {
            Adt6::C0(_, _, h3621, _) => {
                h3621
            }
        });
        nat_add(a3622, a3623)?
    };
    let v4: u64 = f122(match v0.clone() {
        Adt6::C0(_, _, h3624, _) => {
            h3624
        }
    });
    let v5: u64 = {
        let a3628 = {
            let a3626 = v3;
            let a3627 = match v0 {
                Adt6::C0(h3625, _, _, _) => {
                    h3625
                }
            };
            nat_add(a3626, a3627)?
        };
        let a3629 = v4;
        nat_sub(a3628, a3629)
    };
    let a3634 = {
        let a3630 = v5;
        let a3631 = v1;
        if nat_lt(a3630, a3631) {
            v5
        } else {
            v1
        }
    };
    let a3635 = {
        let a3632 = v3;
        let a3633 = v4;
        nat_sub(a3632, a3633)
    };
    Ok(nat_sub(a3634, a3635))
}

pub fn f151(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3644 = f67(match v0.clone() {
        Adt7::C0(h3636, _, _, _, _) => {
            h3636
        }
    }, match v1.clone() {
        Adt3::C0(h3637, _, _, _, _, _, _, _) => {
            h3637
        }
    }, f65(v3, v2.clone(), 0u64), {
        let a3641 = v4;
        let a3642 = {
            let m3640 = match v0.clone() {
                Adt7::C0(h3639, _, _, _, _) => {
                    h3639
                }
            };
            match m3640 {
                Adt6::C0(h3638, _, _, _) => {
                    h3638
                }
            }
        };
        let a3643 = 0u64;
        nat_rem(a3641, a3642, a3643)
    })?;
    match m3644 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3653 = f67(match v0.clone() {
                Adt7::C0(_, h3645, _, _, _) => {
                    h3645
                }
            }, match v1.clone() {
                Adt3::C0(_, h3646, _, _, _, _, _, _) => {
                    h3646
                }
            }, f65(v3, v2.clone(), 1u64), {
                let a3650 = v4;
                let a3651 = {
                    let m3649 = match v0 {
                        Adt7::C0(h3648, _, _, _, _) => {
                            h3648
                        }
                    };
                    match m3649 {
                        Adt6::C0(h3647, _, _, _) => {
                            h3647
                        }
                    }
                };
                let a3652 = 0u64;
                nat_quot(a3650, a3651, a3652)
            })?;
            match m3653 {
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

pub fn f152(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m3654 = v0;
    match m3654 {
        Adt16::C0(v19) => {
            f198(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a3655 = v20;
            let a3656 = 66u64;
            if nat_eq(a3655, a3656) {
                f199(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f200(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f153(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3663 = v3;
        let a3664 = {
            let a3660 = f128({
                let a3657 = v0;
                let a3658 = 2u64;
                let a3659 = 0u64;
                nat_quot(a3657, a3658, a3659)
            }, v1)?;
            let a3661 = v2;
            let a3662 = 0u64;
            nat_quot(a3660, a3661, a3662)
        };
        nat_add(a3663, a3664)?
    };
    let a3668 = {
        let a3665 = v0;
        let a3666 = 2u64;
        let a3667 = 0u64;
        nat_rem(a3665, a3666, a3667)
    };
    let a3669 = 0u64;
    if nat_eq(a3668, a3669) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3670 = v4;
        let a3671 = 0u64;
        if nat_eq(a3670, a3671) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f154(v0: u64, v1: u64) -> R<u64> {
    let a3674 = 8u64;
    let a3675 = {
        let a3672 = f201(v0);
        let a3673 = v1;
        nat_mul(a3672, a3673)?
    };
    nat_add(a3674, a3675)
}

pub fn f155(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3676, _, _, _, _, _, _, _) => {
            h3676
        }
    };
    let v7: u64 = f201(v3);
    let v8: u64 = {
        let a3679 = 1u64;
        let a3680 = {
            let a3677 = v6;
            let a3678 = v7;
            nat_mul(a3677, a3678)?
        };
        nat_add(a3679, a3680)?
    };
    let a3681 = v0;
    let a3682 = 0u64;
    if nat_eq(a3681, a3682) {
        Ok(f202(0u64))
    } else {
        let a3683 = v0;
        let a3684 = v8;
        if nat_lt(a3683, a3684) {
            let v9: u64 = {
                let a3685 = v0;
                let a3686 = 1u64;
                nat_sub(a3685, a3686)
            };
            let v10: u64 = {
                let a3687 = v9;
                let a3688 = v7;
                let a3689 = 0u64;
                nat_rem(a3687, a3688, a3689)
            };
            let v11: u64 = f128({
                let a3695 = {
                    let a3690 = v1;
                    let a3691 = v6;
                    nat_mul(a3690, a3691)?
                };
                let a3696 = {
                    let a3692 = v9;
                    let a3693 = v7;
                    let a3694 = 0u64;
                    nat_quot(a3692, a3693, a3694)
                };
                nat_add(a3695, a3696)?
            }, v2)?;
            let a3697 = v3;
            let a3698 = 30u64;
            if nat_eq(a3697, a3698) {
                let a3699 = v10;
                let a3700 = 0u64;
                if nat_eq(a3699, a3700) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3701 = v10;
                    let a3702 = 1u64;
                    if nat_eq(a3701, a3702) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3703 = v10;
                        let a3704 = 2u64;
                        if nat_eq(a3703, a3704) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3705 = v10;
                let a3706 = 0u64;
                if nat_eq(a3705, a3706) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3707 = v10;
                    let a3708 = 1u64;
                    if nat_eq(a3707, a3708) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3709 = v0;
                let a3710 = v8;
                nat_sub(a3709, a3710)
            };
            let a3711 = v12;
            let a3712 = 0u64;
            if nat_eq(a3711, a3712) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3713 = v12;
                let a3714 = 1u64;
                if nat_eq(a3713, a3714) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3715 = v12;
                    let a3716 = 2u64;
                    if nat_eq(a3715, a3716) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3717 = v12;
                        let a3718 = 3u64;
                        if nat_eq(a3717, a3718) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3719 = v12;
                            let a3720 = 4u64;
                            if nat_eq(a3719, a3720) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3721 = v12;
                                let a3722 = 5u64;
                                if nat_eq(a3721, a3722) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3723 = v12;
                                    let a3724 = 6u64;
                                    if nat_eq(a3723, a3724) {
                                        if f203(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3725 = 3u64;
                                                let a3726 = v1;
                                                nat_add(a3725, a3726)?
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

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3727 = v0;
        let a3728 = f95(v3);
        let a3729 = 0u64;
        nat_quot(a3727, a3728, a3729)
    };
    let v5: u64 = {
        let a3734 = 3u64;
        let a3735 = {
            let a3731 = v4;
            let a3732 = match v1.clone() {
                Adt3::C0(h3730, _, _, _, _, _, _, _) => {
                    h3730
                }
            };
            let a3733 = 0u64;
            nat_quot(a3731, a3732, a3733)
        };
        nat_add(a3734, a3735)?
    };
    let v6: u64 = f128(v4, v1)?;
    let v7: u64 = f128(v4, v2)?;
    let a3736 = v3;
    let a3737 = 30u64;
    if nat_eq(a3736, a3737) {
        let a3741 = {
            let a3738 = v0;
            let a3739 = 3u64;
            let a3740 = 0u64;
            nat_rem(a3738, a3739, a3740)
        };
        let a3742 = 0u64;
        if nat_eq(a3741, a3742) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3746 = {
                let a3743 = v0;
                let a3744 = 3u64;
                let a3745 = 0u64;
                nat_rem(a3743, a3744, a3745)
            };
            let a3747 = 1u64;
            if nat_eq(a3746, a3747) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f157(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3748 = v0;
        let a3749 = 8388608u64;
        let a3750 = 0u64;
        nat_quot(a3748, a3749, a3750)
    };
    let a3751 = v0;
    let a3752 = 2147483648u64;
    if if if nat_lt(a3751, a3752) {
        let a3753 = 127u64;
        let a3754 = v1;
        nat_le(a3753, a3754)
    } else {
        false
    } {
        let a3755 = v1;
        let a3756 = 190u64;
        nat_lt(a3755, a3756)
    } else {
        false
    } {
        let v2: u64 = {
            let a3760 = 8388608u64;
            let a3761 = {
                let a3757 = v0;
                let a3758 = 8388608u64;
                let a3759 = 0u64;
                nat_rem(a3757, a3758, a3759)
            };
            nat_add(a3760, a3761)?
        };
        let a3762 = v1;
        let a3763 = 150u64;
        if nat_lt(a3762, a3763) {
            let v3: u64 = f204({
                let a3764 = 150u64;
                let a3765 = v1;
                nat_sub(a3764, a3765)
            });
            let a3769 = {
                let a3766 = v2;
                let a3767 = v3;
                let a3768 = 0u64;
                nat_rem(a3766, a3767, a3768)
            };
            let a3770 = 0u64;
            if nat_eq(a3769, a3770) {
                let a3771 = v2;
                let a3772 = v3;
                let a3773 = 0u64;
                Ok(nat_quot(a3771, a3772, a3773))
            } else {
                Ok(0u64)
            }
        } else {
            let a3776 = f204({
                let a3774 = v1;
                let a3775 = 150u64;
                nat_sub(a3774, a3775)
            });
            let a3777 = v2;
            nat_mul(a3776, a3777)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f158(v0: Adt14, v1: Adt14) -> bool {
    let a3780 = match v0.clone() {
        Adt14::C0(_, h3778, _, _, _, _, _, _, _, _, _) => {
            h3778
        }
    };
    let a3781 = match v1.clone() {
        Adt14::C0(_, h3779, _, _, _, _, _, _, _, _, _) => {
            h3779
        }
    };
    if if if nat_eq(a3780, a3781) {
        let a3784 = match v0.clone() {
            Adt14::C0(_, _, h3782, _, _, _, _, _, _, _, _) => {
                h3782
            }
        };
        let a3785 = match v1.clone() {
            Adt14::C0(_, _, h3783, _, _, _, _, _, _, _, _) => {
                h3783
            }
        };
        nat_eq(a3784, a3785)
    } else {
        false
    } {
        let a3788 = match v0.clone() {
            Adt14::C0(_, _, _, h3786, _, _, _, _, _, _, _) => {
                h3786
            }
        };
        let a3789 = match v1.clone() {
            Adt14::C0(_, _, _, h3787, _, _, _, _, _, _, _) => {
                h3787
            }
        };
        nat_eq(a3788, a3789)
    } else {
        false
    } {
        let a3792 = match v0 {
            Adt14::C0(_, _, _, _, h3790, _, _, _, _, _, _) => {
                h3790
            }
        };
        let a3793 = match v1 {
            Adt14::C0(_, _, _, _, h3791, _, _, _, _, _, _) => {
                h3791
            }
        };
        nat_eq(a3792, a3793)
    } else {
        false
    }
}

pub fn f159(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3794 = v0;
    let a3795 = 0u64;
    if if if nat_eq(a3794, a3795) {
        true
    } else {
        let a3796 = v1;
        let a3797 = 0u64;
        nat_eq(a3796, a3797)
    } {
        true
    } else {
        let a3798 = v2;
        let a3799 = 0u64;
        nat_eq(a3798, a3799)
    } {
        true
    } else {
        let a3800 = v3;
        let a3801 = 0u64;
        nat_eq(a3800, a3801)
    }
}

pub fn f160(v0: Adt14, v1: Adt14) -> bool {
    if f166(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3802, _, _, _, _, _) => {
            h3802
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h3803, _, _, _, _, _) => {
            h3803
        }
    }) {
        let a3805 = match v0 {
            Adt14::C0(_, h3804, _, _, _, _, _, _, _, _, _) => {
                h3804
            }
        };
        let a3806 = 2147483647u64;
        nat_le(a3805, a3806)
    } else {
        true
    }
}

pub fn f161(v0: Adt14) -> bool {
    let a3811 = {
        let a3808 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3807) => {
                h3807
            }
        };
        let a3809 = 4u64;
        let a3810 = 0u64;
        nat_rem(a3808, a3809, a3810)
    };
    let a3812 = 0u64;
    if if nat_eq(a3811, a3812) {
        f205(match v0.clone() {
            Adt14::C0(_, h3813, _, _, _, _, _, _, _, _, _) => {
                h3813
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3814, _, _, _, _, _) => {
                h3814
            }
        })
    } else {
        false
    } {
        f91(match v0.clone() {
            Adt14::C0(_, _, h3815, _, _, _, _, _, _, _, _) => {
                h3815
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3816, _, _, _, _, _, _, _) => {
                h3816
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3817, _, _, _, _, _, _) => {
                h3817
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3818, _, _, _, _) => {
                h3818
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3819, _, _, _) => {
                h3819
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3820, _, _) => {
                h3820
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f162(v0: Adt14, v1: Adt14) -> R<bool> {
    let a3822 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3821, _) => {
            h3821
        }
    };
    let a3823 = 0u64;
    if nat_eq(a3822, a3823) {
        Ok(true)
    } else {
        let a3825 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h3824, _) => {
                h3824
            }
        };
        let a3826 = 0u64;
        if nat_eq(a3825, a3826) {
            Ok(true)
        } else {
            let a3828 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3827) => {
                    h3827
                }
            };
            let a3829 = 0u64;
            if nat_eq(a3828, a3829) {
                Ok(true)
            } else {
                let a3831 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3830) => {
                        h3830
                    }
                };
                let a3832 = 0u64;
                if nat_eq(a3831, a3832) {
                    Ok(true)
                } else {
                    let a3838 = {
                        let a3835 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3833) => {
                                h3833
                            }
                        };
                        let a3836 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h3834, _) => {
                                h3834
                            }
                        };
                        nat_add(a3835, a3836)?
                    };
                    let a3839 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3837) => {
                            h3837
                        }
                    };
                    if nat_le(a3838, a3839) {
                        Ok(true)
                    } else {
                        let a3845 = {
                            let a3842 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3840) => {
                                    h3840
                                }
                            };
                            let a3843 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h3841, _) => {
                                    h3841
                                }
                            };
                            nat_add(a3842, a3843)?
                        };
                        let a3846 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3844) => {
                                h3844
                            }
                        };
                        Ok(nat_le(a3845, a3846))
                    }
                }
            }
        }
    }
}

pub fn f163(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f158(v0.clone(), v1.clone()) {
        let a3849 = match v0.clone() {
            Adt14::C0(h3847, _, _, _, _, _, _, _, _, _, _) => {
                h3847
            }
        };
        let a3850 = match v1.clone() {
            Adt14::C0(h3848, _, _, _, _, _, _, _, _, _, _) => {
                h3848
            }
        };
        nat_eq(a3849, a3850)
    } else {
        false
    } {
        let a3853 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3851) => {
                h3851
            }
        };
        let a3854 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3852) => {
                h3852
            }
        };
        nat_eq(a3853, a3854)
    } else {
        false
    } {
        let a3857 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3855, _, _, _, _, _) => {
                h3855
            }
        };
        let a3858 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3856, _, _, _, _, _) => {
                h3856
            }
        };
        nat_eq(a3857, a3858)
    } else {
        false
    } {
        let a3861 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3859, _, _, _, _) => {
                h3859
            }
        };
        let a3862 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h3860, _, _, _, _) => {
                h3860
            }
        };
        nat_eq(a3861, a3862)
    } else {
        false
    } {
        let a3865 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3863, _, _, _) => {
                h3863
            }
        };
        let a3866 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3864, _, _, _) => {
                h3864
            }
        };
        nat_eq(a3865, a3866)
    } else {
        false
    } {
        let a3869 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3867, _, _) => {
                h3867
            }
        };
        let a3870 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3868, _, _) => {
                h3868
            }
        };
        nat_eq(a3869, a3870)
    } else {
        false
    }
}

pub fn f164(v0: Adt14) -> R<bool> {
    if f206(match v0.clone() {
        Adt14::C0(_, h3871, _, _, _, _, _, _, _, _, _) => {
            h3871
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3872, _, _, _, _, _) => {
            h3872
        }
    }) {
        f92(f207(match v0.clone() {
            Adt14::C0(_, h3873, _, _, _, _, _, _, _, _, _) => {
                h3873
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3874, _, _, _, _, _) => {
                h3874
            }
        })?, match v0.clone() {
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
        })
    } else {
        Ok(false)
    }
}

pub fn f165(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3881 = v1;
    let a3882 = 0u64;
    if if nat_eq(a3881, a3882) {
        true
    } else {
        let a3883 = v2;
        let a3884 = 0u64;
        nat_eq(a3883, a3884)
    } {
        Ok(0u64)
    } else {
        let a3887 = f93(v1, v2, v3)?;
        let a3888 = {
            let a3885 = v2;
            let a3886 = f208(v0, v1)?;
            nat_mul(a3885, a3886)?
        };
        nat_add(a3887, a3888)
    }
}

pub fn f166(v0: u64, v1: u64) -> bool {
    let a3889 = v0;
    let a3890 = 4u64;
    if nat_eq(a3889, a3890) {
        let a3891 = v1;
        let a3892 = 4u64;
        nat_eq(a3891, a3892)
    } else {
        false
    }
}

pub fn f167(v0: u64) -> R<u64> {
    let a3893 = 6u64;
    let a3894 = v0;
    nat_add(a3893, a3894)
}

pub fn f168(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h3895, _, _, _, _, _, _, _, _, _, _) => {
            h3895
        }
    }, match v0.clone() {
        Adt14::C0(_, h3896, _, _, _, _, _, _, _, _, _) => {
            h3896
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h3897, _, _, _, _, _, _, _, _) => {
            h3897
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h3898, _, _, _, _, _, _, _) => {
            h3898
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h3899, _, _, _, _, _, _) => {
            h3899
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h3900, _, _, _, _) => {
            h3900
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h3901, _, _, _) => {
            h3901
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h3902, _, _) => {
            h3902
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3903, _) => {
            h3903
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3904) => {
            h3904
        }
    })
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a3905 = v0;
    let a3906 = 2147483647u64;
    if if if nat_le(a3905, a3906) {
        let a3907 = 0u64;
        let a3908 = v3;
        if nat_lt(a3907, a3908) {
            let a3909 = v3;
            let a3910 = 2147483647u64;
            nat_le(a3909, a3910)
        } else {
            false
        }
    } else {
        false
    } {
        let a3915 = {
            let a3913 = {
                let a3911 = v2;
                let a3912 = v3;
                nat_add(a3911, a3912)?
            };
            let a3914 = 1u64;
            nat_sub(a3913, a3914)
        };
        let a3916 = 2147483647u64;
        nat_le(a3915, a3916)
    } else {
        false
    } {
        let a3921 = {
            let a3919 = {
                let a3917 = v0;
                let a3918 = v1;
                nat_mul(a3917, a3918)?
            };
            let a3920 = f209(v2, v3)?;
            nat_mul(a3919, a3920)?
        };
        let a3922 = 9223372036854775807u64;
        Ok(nat_le(a3921, a3922))
    } else {
        Ok(false)
    }
}

pub fn f170(v0: Adt14) -> R<u64> {
    let a3953 = {
        let a3945 = {
            let a3937 = {
                let a3929 = 4u64;
                let a3930 = {
                    let a3927 = {
                        let a3924 = match v0.clone() {
                            Adt14::C0(_, h3923, _, _, _, _, _, _, _, _, _) => {
                                h3923
                            }
                        };
                        let a3925 = 1u64;
                        nat_sub(a3924, a3925)
                    };
                    let a3928 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h3926, _, _, _, _, _) => {
                            h3926
                        }
                    };
                    nat_mul(a3927, a3928)?
                };
                nat_add(a3929, a3930)?
            };
            let a3938 = {
                let a3935 = {
                    let a3932 = match v0.clone() {
                        Adt14::C0(_, _, h3931, _, _, _, _, _, _, _, _) => {
                            h3931
                        }
                    };
                    let a3933 = 1u64;
                    nat_sub(a3932, a3933)
                };
                let a3936 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h3934, _, _, _, _) => {
                        h3934
                    }
                };
                nat_mul(a3935, a3936)?
            };
            nat_add(a3937, a3938)?
        };
        let a3946 = {
            let a3943 = {
                let a3940 = match v0.clone() {
                    Adt14::C0(_, _, _, h3939, _, _, _, _, _, _, _) => {
                        h3939
                    }
                };
                let a3941 = 1u64;
                nat_sub(a3940, a3941)
            };
            let a3944 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h3942, _, _, _) => {
                    h3942
                }
            };
            nat_mul(a3943, a3944)?
        };
        nat_add(a3945, a3946)?
    };
    let a3954 = {
        let a3951 = {
            let a3948 = match v0.clone() {
                Adt14::C0(_, _, _, _, h3947, _, _, _, _, _, _) => {
                    h3947
                }
            };
            let a3949 = 1u64;
            nat_sub(a3948, a3949)
        };
        let a3952 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3950, _, _) => {
                h3950
            }
        };
        nat_mul(a3951, a3952)?
    };
    nat_add(a3953, a3954)
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a3955 = v0;
    let a3956 = 0u64;
    if if if if nat_eq(a3955, a3956) {
        true
    } else {
        let a3957 = v1;
        let a3958 = 0u64;
        nat_eq(a3957, a3958)
    } {
        true
    } else {
        let a3959 = v2;
        let a3960 = 0u64;
        nat_eq(a3959, a3960)
    } {
        true
    } else {
        let a3961 = v3;
        let a3962 = 0u64;
        nat_eq(a3961, a3962)
    } {
        Ok(0u64)
    } else {
        let a3963 = f210(v0, v1, v2, v3, v5)?;
        let a3964 = f211(v0, v1, v2, v3, v4)?;
        nat_add(a3963, a3964)
    }
}

pub fn f172(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3965 = v0;
    let a3966 = 0u64;
    if if nat_eq(a3965, a3966) {
        true
    } else {
        let a3967 = v1;
        let a3968 = 0u64;
        nat_eq(a3967, a3968)
    } {
        Ok(0u64)
    } else {
        let a3971 = f93(v0, v1, v2)?;
        let a3972 = {
            let a3969 = v1;
            let a3970 = f212(v0)?;
            nat_mul(a3969, a3970)?
        };
        nat_add(a3971, a3972)
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64) -> bool {
    let a3973 = v0;
    let a3974 = 1u64;
    if nat_le(a3973, a3974) {
        true
    } else {
        let a3978 = {
            let a3975 = v1;
            let a3976 = v2;
            let a3977 = 1u64;
            nat_rem(a3975, a3976, a3977)
        };
        let a3979 = 0u64;
        nat_eq(a3978, a3979)
    }
}

pub fn f174(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3980 = v1;
    let a3981 = 1u64;
    if if if nat_le(a3980, a3981) {
        true
    } else {
        let a3982 = v0;
        let a3983 = v2;
        nat_le(a3982, a3983)
    } {
        let a3984 = v3;
        let a3985 = 1u64;
        if nat_le(a3984, a3985) {
            true
        } else {
            let a3992 = {
                let a3990 = v0;
                let a3991 = {
                    let a3988 = {
                        let a3986 = v1;
                        let a3987 = 1u64;
                        nat_sub(a3986, a3987)
                    };
                    let a3989 = v2;
                    nat_mul(a3988, a3989)?
                };
                nat_add(a3990, a3991)?
            };
            let a3993 = v4;
            nat_le(a3992, a3993)
        }
    } else {
        false
    } {
        let a3994 = v5;
        let a3995 = 1u64;
        if nat_le(a3994, a3995) {
            Ok(true)
        } else {
            let a4008 = {
                let a4006 = {
                    let a4000 = v0;
                    let a4001 = {
                        let a3998 = {
                            let a3996 = v1;
                            let a3997 = 1u64;
                            nat_sub(a3996, a3997)
                        };
                        let a3999 = v2;
                        nat_mul(a3998, a3999)?
                    };
                    nat_add(a4000, a4001)?
                };
                let a4007 = {
                    let a4004 = {
                        let a4002 = v3;
                        let a4003 = 1u64;
                        nat_sub(a4002, a4003)
                    };
                    let a4005 = v4;
                    nat_mul(a4004, a4005)?
                };
                nat_add(a4006, a4007)?
            };
            let a4009 = v6;
            Ok(nat_le(a4008, a4009))
        }
    } else {
        Ok(false)
    }
}

pub fn f175(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f213(v0, v1, v6, v7, v8) {
        let a4010 = v5;
        let a4011 = 26u64;
        nat_eq(a4010, a4011)
    } else {
        false
    } {
        let a4012 = v3;
        let a4013 = v7;
        nat_eq(a4012, a4013)
    } else {
        false
    } {
        let a4014 = v4;
        let a4015 = v8;
        nat_eq(a4014, a4015)
    } else {
        false
    } {
        let a4016 = v9;
        let a4017 = 1u64;
        nat_eq(a4016, a4017)
    } else {
        false
    } {
        let a4018 = v10;
        let a4019 = f214(v0);
        nat_eq(a4018, a4019)
    } else {
        false
    } {
        let a4020 = v11;
        let a4021 = v1;
        nat_eq(a4020, a4021)
    } else {
        false
    } {
        let a4022 = v12;
        let a4023 = v6;
        nat_eq(a4022, a4023)
    } else {
        false
    } {
        let a4024 = v13;
        let a4025 = v7;
        nat_eq(a4024, a4025)
    } else {
        false
    } {
        let a4026 = v14;
        let a4027 = v8;
        nat_eq(a4026, a4027)
    } else {
        false
    }
}

pub fn f176(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4028 = v0;
    let a4029 = 0u64;
    if if if nat_eq(a4028, a4029) {
        true
    } else {
        let a4030 = v1;
        let a4031 = 0u64;
        nat_eq(a4030, a4031)
    } {
        true
    } else {
        let a4032 = v2;
        let a4033 = 0u64;
        nat_eq(a4032, a4033)
    } {
        true
    } else {
        let a4034 = v3;
        let a4035 = 0u64;
        nat_eq(a4034, a4035)
    }
}

pub fn f177(v0: Adt3, v1: bool) -> R<u64> {
    f215(v0.clone(), {
        let a4037 = match v0.clone() {
            Adt3::C0(h4036, _, _, _, _, _, _, _) => {
                h4036
            }
        };
        let a4038 = 1u64;
        nat_sub(a4037, a4038)
    }, {
        let a4040 = match v0.clone() {
            Adt3::C0(_, h4039, _, _, _, _, _, _) => {
                h4039
            }
        };
        let a4041 = 1u64;
        nat_sub(a4040, a4041)
    }, {
        let a4043 = match v0.clone() {
            Adt3::C0(_, _, h4042, _, _, _, _, _) => {
                h4042
            }
        };
        let a4044 = 1u64;
        nat_sub(a4043, a4044)
    }, {
        let a4046 = match v0 {
            Adt3::C0(_, _, _, h4045, _, _, _, _) => {
                h4045
            }
        };
        let a4047 = 1u64;
        nat_sub(a4046, a4047)
    }, v1)
}

pub fn f178(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4048 = v0;
    let a4049 = 0u64;
    if nat_eq(a4048, a4049) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4050 = v0;
            let a4051 = 1u64;
            nat_sub(a4050, a4051)
        };
        let v5: u64 = {
            let a4052 = v4;
            let a4053 = v1;
            let a4054 = 0u64;
            nat_quot(a4052, a4053, a4054)
        };
        let v6: u64 = {
            let a4062 = {
                let a4055 = v5;
                let a4056 = v2;
                nat_mul(a4055, a4056)?
            };
            let a4063 = {
                let a4060 = {
                    let a4057 = v4;
                    let a4058 = v1;
                    let a4059 = 0u64;
                    nat_rem(a4057, a4058, a4059)
                };
                let a4061 = v3;
                nat_mul(a4060, a4061)?
            };
            nat_add(a4062, a4063)?
        };
        let a4064 = v5;
        let a4065 = 0u64;
        if nat_eq(a4064, a4065) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4074 = {
                    let a4068 = {
                        let a4066 = v5;
                        let a4067 = 1u64;
                        nat_sub(a4066, a4067)
                    };
                    let a4069 = v2;
                    nat_mul(a4068, a4069)?
                };
                let a4075 = {
                    let a4072 = {
                        let a4070 = v1;
                        let a4071 = 1u64;
                        nat_sub(a4070, a4071)
                    };
                    let a4073 = v3;
                    nat_mul(a4072, a4073)?
                };
                nat_add(a4074, a4075)?
            };
            let a4076 = v6;
            let a4077 = v7;
            if nat_lt(a4076, a4077) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f179(v0: Adt3, v1: u64) -> bool {
    let a4078 = v1;
    let a4079 = 0u64;
    if !nat_eq(a4078, a4079) {
        true
    } else {
        let a4081 = match v0 {
            Adt3::C0(_, _, _, _, h4080, _, _, _) => {
                h4080
            }
        };
        let a4082 = 4u64;
        nat_eq(a4081, a4082)
    }
}

pub fn f180(v0: u64, v1: bool) -> R<u64> {
    let a4085 = {
        let a4083 = v0;
        let a4084 = 2u64;
        nat_mul(a4083, a4084)?
    };
    let a4086 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4085, a4086)
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4087 = f216(v0, v1);
    match m4087 {
        None => {
            if v5 {
                let m4088 = f217(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4088 {
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
                let a4089 = 4u64;
                let a4090 = v28;
                nat_le(a4089, a4090)
            };
            let a4091 = v2;
            let a4092 = 0u64;
            if if if if if nat_eq(a4091, a4092) {
                let a4093 = v4;
                let a4094 = 0u64;
                nat_eq(a4093, a4094)
            } else {
                false
            } {
                let a4095 = v6;
                let a4096 = v14;
                if nat_eq(a4095, a4096) {
                    let a4097 = v7;
                    let a4098 = v15;
                    nat_eq(a4097, a4098)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4099 = v8;
                let a4100 = v16;
                if nat_eq(a4099, a4100) {
                    let a4101 = v9;
                    let a4102 = v17;
                    nat_eq(a4101, a4102)
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
                        let a4103 = v3;
                        let a4104 = 0u64;
                        nat_eq(a4103, a4104)
                    } else {
                        false
                    } {
                        f107(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4109 = {
                        let a4105 = v6;
                        let a4106 = v7;
                        nat_mul(a4105, a4106)?
                    };
                    let a4110 = {
                        let a4107 = v8;
                        let a4108 = v9;
                        nat_mul(a4107, a4108)?
                    };
                    nat_mul(a4109, a4110)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f182(v0: u64) -> u64 {
    let m4111 = f106(v0);
    match m4111 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4113 = match v1.clone() {
                Adt18::C0(_, h4112, _) => {
                    h4112
                }
            };
            let a4114 = 1u64;
            if if nat_eq(a4113, a4114) {
                let a4116 = match v1.clone() {
                    Adt18::C0(_, _, h4115) => {
                        h4115
                    }
                };
                let a4117 = 1u64;
                if if if nat_eq(a4116, a4117) {
                    true
                } else {
                    let a4119 = match v1.clone() {
                        Adt18::C0(_, _, h4118) => {
                            h4118
                        }
                    };
                    let a4120 = 2u64;
                    nat_eq(a4119, a4120)
                } {
                    true
                } else {
                    let a4122 = match v1.clone() {
                        Adt18::C0(_, _, h4121) => {
                            h4121
                        }
                    };
                    let a4123 = 4u64;
                    nat_eq(a4122, a4123)
                } {
                    true
                } else {
                    let a4125 = match v1.clone() {
                        Adt18::C0(_, _, h4124) => {
                            h4124
                        }
                    };
                    let a4126 = 8u64;
                    nat_eq(a4125, a4126)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4127) => {
                        h4127
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f183(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4132 = {
        let a4130 = {
            let a4128 = v0;
            let a4129 = v1;
            nat_sub(a4128, a4129)
        };
        let a4131 = 1u64;
        nat_sub(a4130, a4131)
    };
    let a4133 = v2;
    nat_add(a4132, a4133)
}

pub fn f184(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f120({
        let a4134 = v0;
        let a4135 = v4;
        let a4136 = 0u64;
        nat_rem(a4134, a4135, a4136)
    }, {
        let a4137 = v1;
        let a4138 = v5;
        let a4139 = 0u64;
        nat_rem(a4137, a4138, a4139)
    }, {
        let a4140 = v2;
        let a4141 = v6;
        let a4142 = 0u64;
        nat_rem(a4140, a4141, a4142)
    }, {
        let a4143 = v3;
        let a4144 = v7;
        let a4145 = 0u64;
        nat_rem(a4143, a4144, a4145)
    }, v8, v9, v10, v11)
}

pub fn f185(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4146 = v1;
    let a4147 = 2147483648u64;
    if nat_lt(a4146, a4147) {
        let a4158 = {
            let a4156 = {
                let a4151 = {
                    let a4148 = v0;
                    let a4149 = v2;
                    let a4150 = 0u64;
                    nat_rem(a4148, a4149, a4150)
                };
                let a4152 = v2;
                nat_add(a4151, a4152)?
            };
            let a4157 = {
                let a4153 = v1;
                let a4154 = v2;
                let a4155 = 0u64;
                nat_rem(a4153, a4154, a4155)
            };
            nat_sub(a4156, a4157)
        };
        let a4159 = v2;
        let a4160 = 0u64;
        Ok(nat_rem(a4158, a4159, a4160))
    } else {
        let a4163 = {
            let a4161 = v0;
            let a4162 = f121(v1);
            nat_add(a4161, a4162)?
        };
        let a4164 = v2;
        let a4165 = 0u64;
        Ok(nat_rem(a4163, a4164, a4165))
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4168 = f122(v1);
    let a4169 = {
        let a4166 = v0;
        let a4167 = f121(v1);
        nat_add(a4166, a4167)?
    };
    if nat_le(a4168, a4169) {
        let a4170 = f187(v0, v1)?;
        let a4171 = v2;
        Ok(nat_lt(a4170, a4171))
    } else {
        Ok(false)
    }
}

pub fn f187(v0: u64, v1: u64) -> R<u64> {
    let a4174 = {
        let a4172 = v0;
        let a4173 = f121(v1);
        nat_add(a4172, a4173)?
    };
    let a4175 = f122(v1);
    Ok(nat_sub(a4174, a4175))
}

pub fn f188(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4176 = v0;
    let a4177 = v1;
    if nat_lt(a4176, a4177) {
        let a4178 = v1;
        let a4179 = v0;
        Ok(nat_sub(a4178, a4179))
    } else {
        let a4182 = v0;
        let a4183 = {
            let a4180 = v1;
            let a4181 = v2;
            nat_add(a4180, a4181)?
        };
        if nat_lt(a4182, a4183) {
            let a4184 = v0;
            let a4185 = v1;
            Ok(nat_sub(a4184, a4185))
        } else {
            let a4192 = {
                let a4188 = 2u64;
                let a4189 = {
                    let a4186 = v2;
                    let a4187 = 1u64;
                    nat_sub(a4186, a4187)
                };
                nat_mul(a4188, a4189)?
            };
            let a4193 = {
                let a4190 = v0;
                let a4191 = v1;
                nat_sub(a4190, a4191)
            };
            Ok(nat_sub(a4192, a4193))
        }
    }
}

pub fn f189(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4196 = {
        let a4194 = v2;
        let a4195 = f215(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1.clone(), 3u64), false)?;
        nat_add(a4194, a4195)?
    };
    let a4197 = f215(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4196, a4197))
}

pub fn f190(v0: u64, v1: u64, v2: u64) -> bool {
    let a4198 = v0;
    let a4199 = 0u64;
    if nat_eq(a4198, a4199) {
        let a4200 = v2;
        let a4201 = v1;
        nat_le(a4200, a4201)
    } else {
        let a4202 = v0;
        let a4203 = 1u64;
        if nat_eq(a4202, a4203) {
            let a4204 = v2;
            let a4205 = v1;
            nat_lt(a4204, a4205)
        } else {
            let a4206 = v0;
            let a4207 = 2u64;
            if nat_eq(a4206, a4207) {
                let a4208 = v1;
                let a4209 = v2;
                nat_le(a4208, a4209)
            } else {
                let a4210 = v0;
                let a4211 = 3u64;
                if nat_eq(a4210, a4211) {
                    let a4212 = v1;
                    let a4213 = v2;
                    nat_lt(a4212, a4213)
                } else {
                    let a4214 = v1;
                    let a4215 = v2;
                    nat_eq(a4214, a4215)
                }
            }
        }
    }
}

pub fn f191(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4216 = v0;
    let a4217 = 0u64;
    if nat_eq(a4216, a4217) {
        Ok(f202(0u64))
    } else {
        let a4220 = {
            let a4218 = v0;
            let a4219 = 1u64;
            nat_add(a4218, a4219)?
        };
        let a4221 = f118(v1.clone(), 13u64, v2)?;
        if nat_eq(a4220, a4221) {
            Ok(Adt9::C4({
                let a4222 = v2;
                let a4223 = 0u64;
                if nat_eq(a4222, a4223) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4224 = v0;
                let a4225 = 1u64;
                nat_sub(a4224, a4225)
            };
            if f179(v1.clone(), v2) {
                let v4: u64 = f180(match v1.clone() {
                    Adt3::C0(h4226, _, _, _, _, _, _, _) => {
                        h4226
                    }
                }, false)?;
                let v5: u64 = {
                    let a4227 = v3;
                    let a4228 = v4;
                    let a4229 = 0u64;
                    nat_quot(a4227, a4228, a4229)
                };
                let v6: u64 = {
                    let a4230 = v3;
                    let a4231 = v4;
                    let a4232 = 0u64;
                    nat_rem(a4230, a4231, a4232)
                };
                let a4235 = {
                    let a4233 = v6;
                    let a4234 = 1u64;
                    nat_add(a4233, a4234)?
                };
                let a4236 = v4;
                if nat_eq(a4235, a4236) {
                    Ok(Adt9::C4({
                        let a4237 = v2;
                        let a4238 = 0u64;
                        if nat_eq(a4237, a4238) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f192(v6, v5, v1, v2)
                }
            } else {
                f218({
                    let a4239 = v3;
                    let a4240 = 2u64;
                    let a4241 = 0u64;
                    nat_quot(a4239, a4240, a4241)
                }, {
                    let a4242 = v3;
                    let a4243 = 2u64;
                    let a4244 = 0u64;
                    nat_rem(a4242, a4243, a4244)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f192(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4245 = v0;
    let a4246 = 0u64;
    if nat_eq(a4245, a4246) {
        Ok(f202(1u64))
    } else {
        f218({
            let a4255 = {
                let a4248 = v1;
                let a4249 = match v2.clone() {
                    Adt3::C0(h4247, _, _, _, _, _, _, _) => {
                        h4247
                    }
                };
                nat_mul(a4248, a4249)?
            };
            let a4256 = {
                let a4252 = {
                    let a4250 = v0;
                    let a4251 = 1u64;
                    nat_sub(a4250, a4251)
                };
                let a4253 = 2u64;
                let a4254 = 0u64;
                nat_quot(a4252, a4253, a4254)
            };
            nat_add(a4255, a4256)?
        }, {
            let a4259 = {
                let a4257 = v0;
                let a4258 = 1u64;
                nat_sub(a4257, a4258)
            };
            let a4260 = 2u64;
            let a4261 = 0u64;
            nat_rem(a4259, a4260, a4261)
        }, v2, v3, 1u64)
    }
}

pub fn f193(v0: u64, v1: u64) -> R<u64> {
    let a4262 = v0;
    let a4263 = f87(v1)?;
    let a4264 = 0u64;
    Ok(nat_rem(a4262, a4263, a4264))
}

pub fn f194(v0: u64, v1: u64) -> R<u64> {
    let a4265 = 4u64;
    let a4266 = f219(v0, v1)?;
    nat_mul(a4265, a4266)
}

pub fn f195(v0: u64, v1: u64) -> R<u64> {
    let a4267 = 4u64;
    let a4268 = f220(v0, v1)?;
    nat_mul(a4267, a4268)
}

pub fn f196(v0: u64, v1: u64) -> R<bool> {
    let a4274 = {
        let a4271 = {
            let a4269 = f193(v0, v1)?;
            let a4270 = 2u64;
            nat_sub(a4269, a4270)
        };
        let a4272 = 2u64;
        let a4273 = 0u64;
        nat_rem(a4271, a4272, a4273)
    };
    let a4275 = 0u64;
    Ok(nat_eq(a4274, a4275))
}

pub fn f197(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4276 = f106(v0);
    match m4276 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f221(v0) {
                let a4277 = v1;
                let a4278 = v7;
                nat_lt(a4277, a4278)
            } else {
                false
            } {
                f145(v8, v2)
            } else {
                false
            } {
                let a4279 = v3;
                let a4280 = f222(v4, v5, v6)?;
                nat_lt(a4279, a4280)
            } else {
                false
            } {
                let a4285 = {
                    let a4282 = v7;
                    let a4283 = match v13.clone() {
                        Adt18::C0(_, h4281, _) => {
                            h4281
                        }
                    };
                    let a4284 = 0u64;
                    nat_rem(a4282, a4283, a4284)
                };
                let a4286 = 0u64;
                nat_eq(a4285, a4286)
            } else {
                false
            } {
                Ok(Some({
                    let a4315 = {
                        let a4305 = {
                            let a4295 = {
                                let a4291 = {
                                    let a4288 = v1;
                                    let a4289 = match v13 {
                                        Adt18::C0(_, h4287, _) => {
                                            h4287
                                        }
                                    };
                                    let a4290 = 0u64;
                                    nat_quot(a4288, a4289, a4290)
                                };
                                let a4292 = v9;
                                nat_mul(a4291, a4292)?
                            };
                            let a4296 = {
                                let a4293 = v2;
                                let a4294 = v10;
                                nat_mul(a4293, a4294)?
                            };
                            nat_add(a4295, a4296)?
                        };
                        let a4306 = {
                            let a4303 = {
                                let a4300 = {
                                    let a4297 = v3;
                                    let a4298 = v4;
                                    let a4299 = 0u64;
                                    nat_quot(a4297, a4298, a4299)
                                };
                                let a4301 = v5;
                                let a4302 = 0u64;
                                nat_rem(a4300, a4301, a4302)
                            };
                            let a4304 = v11;
                            nat_mul(a4303, a4304)?
                        };
                        nat_add(a4305, a4306)?
                    };
                    let a4316 = {
                        let a4313 = {
                            let a4310 = {
                                let a4307 = v3;
                                let a4308 = v4;
                                let a4309 = 0u64;
                                nat_quot(a4307, a4308, a4309)
                            };
                            let a4311 = v5;
                            let a4312 = 0u64;
                            nat_quot(a4310, a4311, a4312)
                        };
                        let a4314 = v12;
                        nat_mul(a4313, a4314)?
                    };
                    nat_add(a4315, a4316)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f198(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f223(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f224(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f200(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4317 = v0;
    let a4318 = 64u64;
    if nat_eq(a4317, a4318) {
        f225(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4319 = v0;
        let a4320 = 65u64;
        if nat_eq(a4319, a4320) {
            f226(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f201(v0: u64) -> u64 {
    let a4321 = v0;
    let a4322 = 30u64;
    if nat_eq(a4321, a4322) {
        4u64
    } else {
        3u64
    }
}

pub fn f202(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f203(v0: u64) -> bool {
    let a4323 = 0u64;
    let a4324 = v0;
    if nat_lt(a4323, a4324) {
        let a4325 = v0;
        let a4326 = 2139095040u64;
        nat_le(a4325, a4326)
    } else {
        false
    }
}

pub fn f204(v0: u64) -> u64 {
    let a4327 = v0;
    let a4328 = 40u64;
    if nat_lt(a4327, a4328) {
        let a4329 = v0;
        let a4330 = 20u64;
        if nat_lt(a4329, a4330) {
            let a4331 = v0;
            let a4332 = 10u64;
            if nat_lt(a4331, a4332) {
                let a4333 = v0;
                let a4334 = 5u64;
                if nat_lt(a4333, a4334) {
                    let a4335 = v0;
                    let a4336 = 2u64;
                    if nat_lt(a4335, a4336) {
                        let a4337 = v0;
                        let a4338 = 1u64;
                        if nat_lt(a4337, a4338) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4339 = v0;
                        let a4340 = 3u64;
                        if nat_lt(a4339, a4340) {
                            4u64
                        } else {
                            let a4341 = v0;
                            let a4342 = 4u64;
                            if nat_lt(a4341, a4342) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4343 = v0;
                    let a4344 = 7u64;
                    if nat_lt(a4343, a4344) {
                        let a4345 = v0;
                        let a4346 = 6u64;
                        if nat_lt(a4345, a4346) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4347 = v0;
                        let a4348 = 8u64;
                        if nat_lt(a4347, a4348) {
                            128u64
                        } else {
                            let a4349 = v0;
                            let a4350 = 9u64;
                            if nat_lt(a4349, a4350) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4351 = v0;
                let a4352 = 15u64;
                if nat_lt(a4351, a4352) {
                    let a4353 = v0;
                    let a4354 = 12u64;
                    if nat_lt(a4353, a4354) {
                        let a4355 = v0;
                        let a4356 = 11u64;
                        if nat_lt(a4355, a4356) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4357 = v0;
                        let a4358 = 13u64;
                        if nat_lt(a4357, a4358) {
                            4096u64
                        } else {
                            let a4359 = v0;
                            let a4360 = 14u64;
                            if nat_lt(a4359, a4360) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4361 = v0;
                    let a4362 = 17u64;
                    if nat_lt(a4361, a4362) {
                        let a4363 = v0;
                        let a4364 = 16u64;
                        if nat_lt(a4363, a4364) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4365 = v0;
                        let a4366 = 18u64;
                        if nat_lt(a4365, a4366) {
                            131072u64
                        } else {
                            let a4367 = v0;
                            let a4368 = 19u64;
                            if nat_lt(a4367, a4368) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4369 = v0;
            let a4370 = 30u64;
            if nat_lt(a4369, a4370) {
                let a4371 = v0;
                let a4372 = 25u64;
                if nat_lt(a4371, a4372) {
                    let a4373 = v0;
                    let a4374 = 22u64;
                    if nat_lt(a4373, a4374) {
                        let a4375 = v0;
                        let a4376 = 21u64;
                        if nat_lt(a4375, a4376) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4377 = v0;
                        let a4378 = 23u64;
                        if nat_lt(a4377, a4378) {
                            4194304u64
                        } else {
                            let a4379 = v0;
                            let a4380 = 24u64;
                            if nat_lt(a4379, a4380) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4381 = v0;
                    let a4382 = 27u64;
                    if nat_lt(a4381, a4382) {
                        let a4383 = v0;
                        let a4384 = 26u64;
                        if nat_lt(a4383, a4384) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4385 = v0;
                        let a4386 = 28u64;
                        if nat_lt(a4385, a4386) {
                            134217728u64
                        } else {
                            let a4387 = v0;
                            let a4388 = 29u64;
                            if nat_lt(a4387, a4388) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4389 = v0;
                let a4390 = 35u64;
                if nat_lt(a4389, a4390) {
                    let a4391 = v0;
                    let a4392 = 32u64;
                    if nat_lt(a4391, a4392) {
                        let a4393 = v0;
                        let a4394 = 31u64;
                        if nat_lt(a4393, a4394) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4395 = v0;
                        let a4396 = 33u64;
                        if nat_lt(a4395, a4396) {
                            4294967296u64
                        } else {
                            let a4397 = v0;
                            let a4398 = 34u64;
                            if nat_lt(a4397, a4398) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4399 = v0;
                    let a4400 = 37u64;
                    if nat_lt(a4399, a4400) {
                        let a4401 = v0;
                        let a4402 = 36u64;
                        if nat_lt(a4401, a4402) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4403 = v0;
                        let a4404 = 38u64;
                        if nat_lt(a4403, a4404) {
                            137438953472u64
                        } else {
                            let a4405 = v0;
                            let a4406 = 39u64;
                            if nat_lt(a4405, a4406) {
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

pub fn f205(v0: u64, v1: u64) -> bool {
    let a4407 = v0;
    let a4408 = 1u64;
    if nat_le(a4407, a4408) {
        true
    } else {
        let a4412 = {
            let a4409 = v1;
            let a4410 = 4u64;
            let a4411 = 0u64;
            nat_rem(a4409, a4410, a4411)
        };
        let a4413 = 0u64;
        nat_eq(a4412, a4413)
    }
}

pub fn f206(v0: u64, v1: u64) -> bool {
    let a4414 = v0;
    let a4415 = 1u64;
    if nat_le(a4414, a4415) {
        true
    } else {
        let a4416 = 4u64;
        let a4417 = v1;
        nat_le(a4416, a4417)
    }
}

pub fn f207(v0: u64, v1: u64) -> R<u64> {
    let a4418 = v0;
    let a4419 = 0u64;
    if nat_eq(a4418, a4419) {
        Ok(0u64)
    } else {
        let a4424 = 4u64;
        let a4425 = {
            let a4422 = {
                let a4420 = v0;
                let a4421 = 1u64;
                nat_sub(a4420, a4421)
            };
            let a4423 = v1;
            nat_mul(a4422, a4423)?
        };
        nat_add(a4424, a4425)
    }
}

pub fn f208(v0: bool, v1: u64) -> R<u64> {
    let a4430 = {
        let a4428 = {
            let a4426 = f227(v0, v1)?;
            let a4427 = f228(v0, v1)?;
            nat_add(a4426, a4427)?
        };
        let a4429 = f229(v0);
        nat_add(a4428, a4429)?
    };
    let a4431 = v1;
    nat_add(a4430, a4431)
}

pub fn f209(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f230(v0, v1)?;
    let v3: u64 = {
        let a4436 = {
            let a4434 = {
                let a4432 = v1;
                let a4433 = 1u64;
                nat_sub(a4432, a4433)
            };
            let a4435 = v2;
            nat_mul(a4434, a4435)?
        };
        let a4437 = v0;
        nat_sub(a4436, a4437)
    };
    let a4438 = v2;
    let a4439 = v3;
    if nat_lt(a4438, a4439) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4440 = v4;
    let a4441 = 0u64;
    if nat_eq(a4440, a4441) {
        Ok(0u64)
    } else {
        let a4448 = 2u64;
        let a4449 = {
            let a4446 = {
                let a4442 = v0;
                let a4443 = v1;
                nat_mul(a4442, a4443)?
            };
            let a4447 = {
                let a4444 = v2;
                let a4445 = v3;
                nat_mul(a4444, a4445)?
            };
            nat_mul(a4446, a4447)?
        };
        nat_mul(a4448, a4449)
    }
}

pub fn f211(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4454 = v3;
    let a4455 = {
        let a4452 = {
            let a4450 = f231(v2, v4)?;
            let a4451 = f232(v0, v1, f230(v2, v4)?)?;
            nat_mul(a4450, a4451)?
        };
        let a4453 = f232(v0, v1, f233(v2, v4)?)?;
        nat_add(a4452, a4453)?
    };
    nat_mul(a4454, a4455)
}

pub fn f212(v0: u64) -> R<u64> {
    let a4458 = 13u64;
    let a4459 = {
        let a4456 = 9u64;
        let a4457 = v0;
        nat_mul(a4456, a4457)?
    };
    nat_add(a4458, a4459)
}

pub fn f213(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f176(v1, v2, v3, v4) {
        f234(v0)
    } else {
        f221(v0)
    }
}

pub fn f214(v0: u64) -> u64 {
    let a4460 = v0;
    let a4461 = 26u64;
    if nat_eq(a4460, a4461) {
        26u64
    } else {
        0u64
    }
}

pub fn f215(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4482 = {
        let a4476 = {
            let a4470 = {
                let a4464 = v1;
                let a4465 = if v5 {
                    f121(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4462, _, _, _) => {
                            h4462
                        }
                    })
                } else {
                    f122(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4463, _, _, _) => {
                            h4463
                        }
                    })
                };
                nat_mul(a4464, a4465)?
            };
            let a4471 = {
                let a4468 = v2;
                let a4469 = if v5 {
                    f121(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4466, _, _) => {
                            h4466
                        }
                    })
                } else {
                    f122(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4467, _, _) => {
                            h4467
                        }
                    })
                };
                nat_mul(a4468, a4469)?
            };
            nat_add(a4470, a4471)?
        };
        let a4477 = {
            let a4474 = v3;
            let a4475 = if v5 {
                f121(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4472, _) => {
                        h4472
                    }
                })
            } else {
                f122(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4473, _) => {
                        h4473
                    }
                })
            };
            nat_mul(a4474, a4475)?
        };
        nat_add(a4476, a4477)?
    };
    let a4483 = {
        let a4480 = v4;
        let a4481 = if v5 {
            f121(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4478) => {
                    h4478
                }
            })
        } else {
            f122(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4479) => {
                    h4479
                }
            })
        };
        nat_mul(a4480, a4481)?
    };
    nat_add(a4482, a4483)
}

pub fn f216(v0: u64, v1: u64) -> Option<u64> {
    let a4484 = v0;
    let a4485 = 2u64;
    if nat_eq(a4484, a4485) {
        Some(0u64)
    } else {
        let a4486 = v0;
        let a4487 = 5u64;
        if nat_eq(a4486, a4487) {
            Some(1u64)
        } else {
            let a4488 = v0;
            let a4489 = 6u64;
            if nat_eq(a4488, a4489) {
                Some(2u64)
            } else {
                let a4490 = v0;
                let a4491 = 7u64;
                if nat_eq(a4490, a4491) {
                    Some(3u64)
                } else {
                    let a4492 = v0;
                    let a4493 = 9u64;
                    if nat_eq(a4492, a4493) {
                        Some(4u64)
                    } else {
                        let a4494 = v0;
                        let a4495 = 8u64;
                        if nat_eq(a4494, a4495) {
                            Some(5u64)
                        } else {
                            let a4496 = v0;
                            let a4497 = 89u64;
                            if nat_eq(a4496, a4497) {
                                let a4498 = v1;
                                let a4499 = 2u64;
                                if nat_eq(a4498, a4499) {
                                    Some(6u64)
                                } else {
                                    let a4500 = v1;
                                    let a4501 = 0u64;
                                    if nat_eq(a4500, a4501) {
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

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4502 = v0;
    let a4503 = 30u64;
    if nat_eq(a4502, a4503) {
        let a4504 = v2;
        let a4505 = 0u64;
        if if if nat_eq(a4504, a4505) {
            true
        } else {
            let a4506 = v2;
            let a4507 = 1u64;
            nat_eq(a4506, a4507)
        } {
            let a4508 = v1;
            let a4509 = 0u64;
            if nat_eq(a4508, a4509) {
                true
            } else {
                f235(v1, v4)
            }
        } else {
            false
        } {
            f236(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f237(v1) {
            f237(v2)
        } else {
            false
        } {
            let a4510 = v1;
            let a4511 = 0u64;
            if if nat_eq(a4510, a4511) {
                true
            } else {
                let a4512 = v2;
                let a4513 = 0u64;
                nat_eq(a4512, a4513)
            } {
                true
            } else {
                let a4514 = v1;
                let a4515 = v2;
                nat_eq(a4514, a4515)
            }
        } else {
            false
        } {
            true
        } else {
            let a4516 = v1;
            let a4517 = 43u64;
            if nat_eq(a4516, a4517) {
                f237(v2)
            } else {
                false
            }
        } {
            f236(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f218(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4518 = v1;
    let a4519 = 0u64;
    if nat_eq(a4518, a4519) {
        Ok(Adt9::C4({
            let a4520 = v3;
            let a4521 = 0u64;
            if nat_eq(a4520, a4521) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f128(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4522 = v3;
            let a4523 = 0u64;
            if nat_eq(a4522, a4523) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f219(v0: u64, v1: u64) -> R<u64> {
    let a4524 = v0;
    let a4525 = f87(v1)?;
    let a4526 = 0u64;
    Ok(nat_quot(a4524, a4525, a4526))
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f219(v0, v1)?;
    let v3: u64 = {
        let a4529 = {
            let a4527 = f193(v0, v1)?;
            let a4528 = 2u64;
            nat_sub(a4527, a4528)
        };
        let a4530 = 2u64;
        let a4531 = 0u64;
        nat_quot(a4529, a4530, a4531)
    };
    let v4: u64 = {
        let a4532 = v2;
        let a4533 = v1;
        nat_mul(a4532, a4533)?
    };
    if f196(v0, v1)? {
        let a4534 = v4;
        let a4535 = v3;
        nat_add(a4534, a4535)
    } else {
        let a4547 = {
            let a4543 = v4;
            let a4544 = {
                let a4541 = {
                    let a4536 = v1;
                    let a4537 = 1u64;
                    nat_sub(a4536, a4537)
                };
                let a4542 = {
                    let a4538 = v2;
                    let a4539 = v1;
                    let a4540 = 0u64;
                    nat_rem(a4538, a4539, a4540)
                };
                nat_mul(a4541, a4542)?
            };
            nat_sub(a4543, a4544)
        };
        let a4548 = {
            let a4545 = v3;
            let a4546 = v1;
            nat_mul(a4545, a4546)?
        };
        nat_add(a4547, a4548)
    }
}

pub fn f221(v0: u64) -> bool {
    let a4549 = v0;
    let a4550 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4549, a4550) {
        true
    } else {
        let a4551 = v0;
        let a4552 = 1u64;
        nat_eq(a4551, a4552)
    } {
        true
    } else {
        let a4553 = v0;
        let a4554 = 2u64;
        nat_eq(a4553, a4554)
    } {
        true
    } else {
        let a4555 = v0;
        let a4556 = 3u64;
        nat_eq(a4555, a4556)
    } {
        true
    } else {
        let a4557 = v0;
        let a4558 = 6u64;
        nat_eq(a4557, a4558)
    } {
        true
    } else {
        let a4559 = v0;
        let a4560 = 7u64;
        nat_eq(a4559, a4560)
    } {
        true
    } else {
        let a4561 = v0;
        let a4562 = 8u64;
        nat_eq(a4561, a4562)
    } {
        true
    } else {
        let a4563 = v0;
        let a4564 = 10u64;
        nat_eq(a4563, a4564)
    } {
        true
    } else {
        let a4565 = v0;
        let a4566 = 11u64;
        nat_eq(a4565, a4566)
    } {
        true
    } else {
        let a4567 = v0;
        let a4568 = 12u64;
        nat_eq(a4567, a4568)
    } {
        true
    } else {
        let a4569 = v0;
        let a4570 = 13u64;
        nat_eq(a4569, a4570)
    } {
        true
    } else {
        let a4571 = v0;
        let a4572 = 14u64;
        nat_eq(a4571, a4572)
    } {
        true
    } else {
        let a4573 = v0;
        let a4574 = 16u64;
        nat_eq(a4573, a4574)
    } {
        true
    } else {
        let a4575 = v0;
        let a4576 = 17u64;
        nat_eq(a4575, a4576)
    } {
        true
    } else {
        let a4577 = v0;
        let a4578 = 18u64;
        nat_eq(a4577, a4578)
    } {
        true
    } else {
        let a4579 = v0;
        let a4580 = 19u64;
        nat_eq(a4579, a4580)
    } {
        true
    } else {
        let a4581 = v0;
        let a4582 = 20u64;
        nat_eq(a4581, a4582)
    } {
        true
    } else {
        let a4583 = v0;
        let a4584 = 21u64;
        nat_eq(a4583, a4584)
    } {
        true
    } else {
        let a4585 = v0;
        let a4586 = 22u64;
        nat_eq(a4585, a4586)
    } {
        true
    } else {
        let a4587 = v0;
        let a4588 = 23u64;
        nat_eq(a4587, a4588)
    } {
        true
    } else {
        let a4589 = v0;
        let a4590 = 26u64;
        nat_eq(a4589, a4590)
    } {
        true
    } else {
        let a4591 = v0;
        let a4592 = 29u64;
        nat_eq(a4591, a4592)
    } {
        true
    } else {
        let a4593 = v0;
        let a4594 = 30u64;
        nat_eq(a4593, a4594)
    } {
        true
    } else {
        let a4595 = v0;
        let a4596 = 34u64;
        nat_eq(a4595, a4596)
    } {
        true
    } else {
        let a4597 = v0;
        let a4598 = 35u64;
        nat_eq(a4597, a4598)
    } {
        true
    } else {
        let a4599 = v0;
        let a4600 = 39u64;
        nat_eq(a4599, a4600)
    } {
        true
    } else {
        let a4601 = v0;
        let a4602 = 40u64;
        nat_eq(a4601, a4602)
    } {
        true
    } else {
        let a4603 = v0;
        let a4604 = 41u64;
        nat_eq(a4603, a4604)
    } {
        true
    } else {
        let a4605 = v0;
        let a4606 = 42u64;
        nat_eq(a4605, a4606)
    } {
        true
    } else {
        let a4607 = v0;
        let a4608 = 43u64;
        nat_eq(a4607, a4608)
    }
}

pub fn f222(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4609 = v0;
    let a4610 = 0u64;
    if if if nat_eq(a4609, a4610) {
        true
    } else {
        let a4611 = v1;
        let a4612 = 0u64;
        nat_eq(a4611, a4612)
    } {
        true
    } else {
        let a4613 = v2;
        let a4614 = 0u64;
        nat_eq(a4613, a4614)
    } {
        Ok(0u64)
    } else {
        let a4617 = {
            let a4615 = v0;
            let a4616 = v1;
            nat_mul(a4615, a4616)?
        };
        let a4618 = v2;
        nat_mul(a4617, a4618)
    }
}

pub fn f223(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f96(match v1.clone() {
        Adt20::C0(h4619, _, _, _, _, _, _, _) => {
            h4619
        }
    })?;
    let v6: u64 = {
        let a4625 = match v1.clone() {
            Adt20::C0(_, h4620, _, _, _, _, _, _) => {
                h4620
            }
        };
        let a4626 = {
            let a4623 = match v1.clone() {
                Adt20::C0(_, _, h4621, _, _, _, _, _) => {
                    h4621
                }
            };
            let a4624 = match v1.clone() {
                Adt20::C0(_, _, _, h4622, _, _, _, _) => {
                    h4622
                }
            };
            nat_mul(a4623, a4624)?
        };
        nat_mul(a4625, a4626)?
    };
    let a4628 = v0;
    let a4629 = f36(match v1.clone() {
        Adt20::C0(h4627, _, _, _, _, _, _, _) => {
            h4627
        }
    }, v6)?;
    if nat_lt(a4628, a4629) {
        f238({
            let a4630 = v0;
            let a4631 = v5;
            let a4632 = 0u64;
            nat_rem(a4630, a4631, a4632)
        }, {
            let a4633 = v0;
            let a4634 = v5;
            let a4635 = 0u64;
            nat_quot(a4633, a4634, a4635)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f224(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a4636 = v6;
        let a4637 = 6u64;
        nat_sub(a4636, a4637)
    };
    let v9: Adt20 = f239(v3);
    let a4639 = v0;
    let a4640 = f172(match v1.clone() {
        Adt20::C0(h4638, _, _, _, _, _, _, _) => {
            h4638
        }
    }, v5, v8)?;
    if nat_lt(a4639, a4640) {
        let v10: u64 = f93(match v1.clone() {
            Adt20::C0(h4641, _, _, _, _, _, _, _) => {
                h4641
            }
        }, v5, v8)?;
        let a4642 = v0;
        let a4643 = v10;
        if nat_lt(a4642, a4643) {
            f240(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a4644 = v0;
                let a4645 = v10;
                nat_sub(a4644, a4645)
            };
            let v12: u64 = f212(match v1.clone() {
                Adt20::C0(h4646, _, _, _, _, _, _, _) => {
                    h4646
                }
            })?;
            f241({
                let a4647 = v11;
                let a4648 = v12;
                let a4649 = 0u64;
                nat_rem(a4647, a4648, a4649)
            }, {
                let a4650 = v11;
                let a4651 = v12;
                let a4652 = 0u64;
                nat_quot(a4650, a4651, a4652)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f225(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4653 = v5;
        let a4654 = 6u64;
        nat_sub(a4653, a4654)
    };
    let v8: bool = f166(match v1.clone() {
        Adt20::C0(_, _, _, _, h4655, _, _, _) => {
            h4655
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h4656, _, _, _) => {
            h4656
        }
    });
    let a4658 = v0;
    let a4659 = f165(v8, match v1.clone() {
        Adt20::C0(h4657, _, _, _, _, _, _, _) => {
            h4657
        }
    }, v4, v7)?;
    if nat_lt(a4658, a4659) {
        let v9: u64 = f93(match v1.clone() {
            Adt20::C0(h4660, _, _, _, _, _, _, _) => {
                h4660
            }
        }, v4, v7)?;
        let a4661 = v0;
        let a4662 = v9;
        if nat_lt(a4661, a4662) {
            f240(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a4663 = v0;
                let a4664 = v9;
                nat_sub(a4663, a4664)
            };
            let v11: u64 = f208(v8, match v1.clone() {
                Adt20::C0(h4665, _, _, _, _, _, _, _) => {
                    h4665
                }
            })?;
            f242({
                let a4666 = v10;
                let a4667 = v11;
                let a4668 = 0u64;
                nat_rem(a4666, a4667, a4668)
            }, {
                let a4669 = v10;
                let a4670 = v11;
                let a4671 = 0u64;
                nat_quot(a4669, a4670, a4671)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f226(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a4672 = v5;
        let a4673 = 6u64;
        nat_sub(a4672, a4673)
    };
    let v8: Adt20 = f239(v2);
    let v9: u64 = f210(match v1.clone() {
        Adt20::C0(h4674, _, _, _, _, _, _, _) => {
            h4674
        }
    }, match v1.clone() {
        Adt20::C0(_, h4675, _, _, _, _, _, _) => {
            h4675
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4676, _, _, _, _, _) => {
            h4676
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4677, _, _, _, _) => {
            h4677
        }
    }, v7)?;
    let a4682 = v0;
    let a4683 = f171(match v1.clone() {
        Adt20::C0(h4678, _, _, _, _, _, _, _) => {
            h4678
        }
    }, match v1.clone() {
        Adt20::C0(_, h4679, _, _, _, _, _, _) => {
            h4679
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h4680, _, _, _, _, _) => {
            h4680
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h4681, _, _, _, _) => {
            h4681
        }
    }, v4, v7)?;
    if nat_lt(a4682, a4683) {
        let a4684 = v0;
        let a4685 = v9;
        if nat_lt(a4684, a4685) {
            f240(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f243({
                let a4686 = v0;
                let a4687 = v9;
                nat_sub(a4686, a4687)
            }, match v1.clone() {
                Adt20::C0(h4688, _, _, _, _, _, _, _) => {
                    h4688
                }
            }, match v1.clone() {
                Adt20::C0(_, h4689, _, _, _, _, _, _) => {
                    h4689
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h4690, _, _, _, _, _) => {
                    h4690
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h4691, _, _, _, _) => {
                    h4691
                }
            }, v4)?;
            f244(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f227(v0: bool, v1: u64) -> R<u64> {
    let a4694 = if v0 {
        5u64
    } else {
        4u64
    };
    let a4695 = {
        let a4692 = f245(v0);
        let a4693 = v1;
        nat_mul(a4692, a4693)?
    };
    nat_add(a4694, a4695)
}

pub fn f228(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4703 = {
            let a4696 = 13u64;
            let a4697 = f246(v1);
            nat_mul(a4696, a4697)?
        };
        let a4704 = {
            let a4701 = 4u64;
            let a4702 = {
                let a4698 = v1;
                let a4699 = 4u64;
                let a4700 = 0u64;
                nat_rem(a4698, a4699, a4700)
            };
            nat_mul(a4701, a4702)?
        };
        nat_add(a4703, a4704)
    } else {
        let a4705 = 3u64;
        let a4706 = v1;
        nat_mul(a4705, a4706)
    }
}

pub fn f229(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f230(v0: u64, v1: u64) -> R<u64> {
    let a4711 = {
        let a4709 = {
            let a4707 = v0;
            let a4708 = v1;
            nat_add(a4707, a4708)?
        };
        let a4710 = 1u64;
        nat_sub(a4709, a4710)
    };
    let a4712 = v1;
    let a4713 = 0u64;
    Ok(nat_quot(a4711, a4712, a4713))
}

pub fn f231(v0: u64, v1: u64) -> R<u64> {
    let a4714 = v0;
    let a4715 = f230(v0, v1)?;
    let a4716 = 0u64;
    Ok(nat_quot(a4714, a4715, a4716))
}

pub fn f232(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4717 = v2;
    let a4718 = 0u64;
    if nat_eq(a4717, a4718) {
        Ok(0u64)
    } else {
        let a4723 = 11u64;
        let a4724 = {
            let a4721 = {
                let a4719 = v1;
                let a4720 = v2;
                nat_mul(a4719, a4720)?
            };
            let a4722 = f247(v0)?;
            nat_mul(a4721, a4722)?
        };
        nat_add(a4723, a4724)
    }
}

pub fn f233(v0: u64, v1: u64) -> R<u64> {
    let a4725 = v0;
    let a4726 = f230(v0, v1)?;
    let a4727 = 0u64;
    Ok(nat_rem(a4725, a4726, a4727))
}

pub fn f234(v0: u64) -> bool {
    let m4728 = f106(v0);
    match m4728 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f235(v0: u64, v1: u64) -> bool {
    let a4729 = v0;
    let a4730 = 1u64;
    if nat_eq(a4729, a4730) {
        true
    } else {
        let v2: u64 = f248(v0);
        let a4731 = 0u64;
        let a4732 = v2;
        if nat_lt(a4731, a4732) {
            let a4736 = {
                let a4733 = v1;
                let a4734 = v2;
                let a4735 = 1u64;
                nat_rem(a4733, a4734, a4735)
            };
            let a4737 = 0u64;
            nat_eq(a4736, a4737)
        } else {
            false
        }
    }
}

pub fn f236(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a4738 = v0;
    let a4739 = 28u64;
    if if if if nat_eq(a4738, a4739) {
        let a4740 = v1;
        let a4741 = 0u64;
        nat_eq(a4740, a4741)
    } else {
        false
    } {
        let a4742 = v2;
        let a4743 = 0u64;
        nat_eq(a4742, a4743)
    } else {
        false
    } {
        let a4744 = v3;
        let a4745 = 0u64;
        nat_eq(a4744, a4745)
    } else {
        false
    } {
        let a4746 = v4;
        let a4747 = v8;
        if if if if if if if if nat_eq(a4746, a4747) {
            let a4748 = v5;
            let a4749 = v12;
            nat_eq(a4748, a4749)
        } else {
            false
        } {
            let a4750 = v9;
            let a4751 = v13;
            nat_eq(a4750, a4751)
        } else {
            false
        } {
            let a4752 = v10;
            let a4753 = v14;
            nat_eq(a4752, a4753)
        } else {
            false
        } {
            let a4754 = v11;
            let a4755 = v15;
            nat_eq(a4754, a4755)
        } else {
            false
        } {
            let a4756 = 0u64;
            let a4757 = v6;
            nat_lt(a4756, a4757)
        } else {
            false
        } {
            let a4758 = 0u64;
            let a4759 = v7;
            nat_lt(a4758, a4759)
        } else {
            false
        } {
            let a4763 = {
                let a4760 = v10;
                let a4761 = v6;
                let a4762 = 1u64;
                nat_rem(a4760, a4761, a4762)
            };
            let a4764 = 0u64;
            if nat_eq(a4763, a4764) {
                let a4768 = {
                    let a4765 = v11;
                    let a4766 = v7;
                    let a4767 = 1u64;
                    nat_rem(a4765, a4766, a4767)
                };
                let a4769 = 0u64;
                nat_eq(a4768, a4769)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a4770 = v10;
                let a4771 = v6;
                let a4772 = 0u64;
                nat_quot(a4770, a4771, a4772)
            }, {
                let a4773 = v11;
                let a4774 = v7;
                let a4775 = 0u64;
                nat_quot(a4773, a4774, a4775)
            }, {
                let a4776 = v5;
                let a4777 = v4;
                nat_mul(a4776, a4777)?
            }, {
                let a4778 = v4;
                let a4779 = v9;
                nat_mul(a4778, a4779)?
            }, {
                let a4780 = v5;
                let a4781 = v9;
                nat_mul(a4780, a4781)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f237(v0: u64) -> bool {
    let a4782 = v0;
    let a4783 = 0u64;
    if if nat_eq(a4782, a4783) {
        true
    } else {
        let a4784 = v0;
        let a4785 = 1u64;
        nat_eq(a4784, a4785)
    } {
        true
    } else {
        let a4786 = v0;
        let a4787 = 30u64;
        nat_eq(a4786, a4787)
    }
}

pub fn f238(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h4788, _, _, _, _, _, _, _) => {
            h4788
        }
    };
    let v7: u64 = {
        let a4791 = 1u64;
        let a4792 = {
            let a4789 = 3u64;
            let a4790 = v6;
            nat_mul(a4789, a4790)?
        };
        nat_add(a4791, a4792)?
    };
    let a4793 = v0;
    let a4794 = 0u64;
    if nat_eq(a4793, a4794) {
        Ok(f249(0u64))
    } else {
        let a4795 = v0;
        let a4796 = v7;
        if nat_lt(a4795, a4796) {
            let v8: u64 = {
                let a4797 = v0;
                let a4798 = 1u64;
                nat_sub(a4797, a4798)
            };
            let v9: u64 = {
                let a4804 = {
                    let a4799 = v1;
                    let a4800 = v6;
                    nat_mul(a4799, a4800)?
                };
                let a4805 = {
                    let a4801 = v8;
                    let a4802 = 3u64;
                    let a4803 = 0u64;
                    nat_quot(a4801, a4802, a4803)
                };
                nat_add(a4804, a4805)?
            };
            let v10: u64 = f250(v9, v2)?;
            let v11: u64 = {
                let a4806 = v8;
                let a4807 = 3u64;
                let a4808 = 0u64;
                nat_rem(a4806, a4807, a4808)
            };
            let a4809 = v11;
            let a4810 = 0u64;
            if nat_eq(a4809, a4810) {
                Ok(f251(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a4811 = v11;
                let a4812 = 1u64;
                if nat_eq(a4811, a4812) {
                    Ok(f251(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f251(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a4813 = v0;
            let a4814 = v7;
            if nat_eq(a4813, a4814) {
                Ok(f251(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4817 = v0;
                let a4818 = {
                    let a4815 = v7;
                    let a4816 = 1u64;
                    nat_add(a4815, a4816)?
                };
                if nat_eq(a4817, a4818) {
                    Ok(f251(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4821 = v0;
                    let a4822 = {
                        let a4819 = v7;
                        let a4820 = 2u64;
                        nat_add(a4819, a4820)?
                    };
                    if nat_eq(a4821, a4822) {
                        Ok(f251(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a4825 = v0;
                        let a4826 = {
                            let a4823 = v7;
                            let a4824 = 3u64;
                            nat_add(a4823, a4824)?
                        };
                        if nat_eq(a4825, a4826) {
                            let a4827 = v5;
                            let a4828 = 1u64;
                            if if nat_eq(a4827, a4828) {
                                true
                            } else {
                                let a4829 = v5;
                                let a4830 = 2u64;
                                nat_eq(a4829, a4830)
                            } {
                                Ok(f251(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f251(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a4837 = {
                                    let a4831 = v1;
                                    let a4832 = v6;
                                    nat_mul(a4831, a4832)?
                                };
                                let a4838 = {
                                    let a4835 = v0;
                                    let a4836 = {
                                        let a4833 = v7;
                                        let a4834 = 4u64;
                                        nat_add(a4833, a4834)?
                                    };
                                    nat_sub(a4835, a4836)
                                };
                                nat_add(a4837, a4838)?
                            };
                            Ok(f251(10u64, 0u64, f250(v12, v2)?, 1u64, 2u64, 3u64, f250(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f239(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h4839, _, _, _, _, _, _, _) => {
            h4839
        }
    }, match v0.clone() {
        Adt20::C0(_, h4840, _, _, _, _, _, _) => {
            h4840
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h4841, _, _, _, _, _) => {
            h4841
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h4842, _, _, _, _) => {
            h4842
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h4843, _, _) => {
            h4843
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h4844, _) => {
            h4844
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h4845) => {
            h4845
        }
    })
}

pub fn f240(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a4852 = v3;
        let a4853 = {
            let a4849 = f250({
                let a4846 = v0;
                let a4847 = 2u64;
                let a4848 = 0u64;
                nat_quot(a4846, a4847, a4848)
            }, v1)?;
            let a4850 = v2;
            let a4851 = 0u64;
            nat_quot(a4849, a4850, a4851)
        };
        nat_add(a4852, a4853)?
    };
    let a4857 = {
        let a4854 = v0;
        let a4855 = 2u64;
        let a4856 = 0u64;
        nat_rem(a4854, a4855, a4856)
    };
    let a4858 = 0u64;
    if nat_eq(a4857, a4858) {
        Ok(f251(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4859 = v4;
        let a4860 = 0u64;
        if nat_eq(a4859, a4860) {
            Ok(f251(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f241(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f252(match v2.clone() {
        Adt20::C0(h4861, _, _, _, _, _, _, _) => {
            h4861
        }
    })?;
    let v7: u64 = f253(match v2.clone() {
        Adt20::C0(h4862, _, _, _, _, _, _, _) => {
            h4862
        }
    })?;
    let a4863 = v0;
    let a4864 = 2u64;
    if nat_lt(a4863, a4864) {
        Ok(f249(v0))
    } else {
        let a4865 = v0;
        let a4866 = v6;
        if nat_lt(a4865, a4866) {
            f254({
                let a4867 = v0;
                let a4868 = 2u64;
                nat_sub(a4867, a4868)
            }, v1, v2, v3)
        } else {
            let a4869 = v0;
            let a4870 = v7;
            if nat_lt(a4869, a4870) {
                Ok(f255({
                    let a4871 = v0;
                    let a4872 = v6;
                    nat_sub(a4871, a4872)
                }, match v2 {
                    Adt20::C0(h4873, _, _, _, _, _, _, _) => {
                        h4873
                    }
                }, v5))
            } else {
                let a4875 = v0;
                let a4876 = f212(match v2.clone() {
                    Adt20::C0(h4874, _, _, _, _, _, _, _) => {
                        h4874
                    }
                })?;
                if nat_lt(a4875, a4876) {
                    f256({
                        let a4877 = v0;
                        let a4878 = v7;
                        nat_sub(a4877, a4878)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f242(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f227(v5, match v2.clone() {
        Adt20::C0(h4879, _, _, _, _, _, _, _) => {
            h4879
        }
    })?;
    let v7: u64 = {
        let a4881 = v6;
        let a4882 = f228(v5, match v2.clone() {
            Adt20::C0(h4880, _, _, _, _, _, _, _) => {
                h4880
            }
        })?;
        nat_add(a4881, a4882)?
    };
    let v8: u64 = {
        let a4883 = v7;
        let a4884 = f229(v5);
        nat_add(a4883, a4884)?
    };
    let a4885 = v0;
    let a4886 = v6;
    if nat_lt(a4885, a4886) {
        f257(v0, v1, v2, v5)
    } else {
        let a4887 = v0;
        let a4888 = v7;
        if nat_lt(a4887, a4888) {
            f258({
                let a4889 = v0;
                let a4890 = v6;
                nat_sub(a4889, a4890)
            }, v1, v2, v3, v5)
        } else {
            let a4891 = v0;
            let a4892 = v8;
            if nat_lt(a4891, a4892) {
                Ok(f259({
                    let a4893 = v0;
                    let a4894 = v7;
                    nat_sub(a4893, a4894)
                }, match v2 {
                    Adt20::C0(h4895, _, _, _, _, _, _, _) => {
                        h4895
                    }
                }, v4, v5))
            } else {
                let a4899 = v0;
                let a4900 = {
                    let a4897 = v8;
                    let a4898 = match v2.clone() {
                        Adt20::C0(h4896, _, _, _, _, _, _, _) => {
                            h4896
                        }
                    };
                    nat_add(a4897, a4898)?
                };
                if nat_lt(a4899, a4900) {
                    let v9: u64 = f250({
                        let a4906 = {
                            let a4902 = v1;
                            let a4903 = match v2 {
                                Adt20::C0(h4901, _, _, _, _, _, _, _) => {
                                    h4901
                                }
                            };
                            nat_mul(a4902, a4903)?
                        };
                        let a4907 = {
                            let a4904 = v0;
                            let a4905 = v8;
                            nat_sub(a4904, a4905)
                        };
                        nat_add(a4906, a4907)?
                    }, v3)?;
                    Ok(f251(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f230(v3, v5)?;
    let v7: u64 = f232(v1, v2, v6)?;
    let v8: u64 = {
        let a4910 = {
            let a4908 = f231(v3, v5)?;
            let a4909 = v4;
            nat_mul(a4908, a4909)?
        };
        let a4911 = v7;
        nat_mul(a4910, a4911)?
    };
    let a4912 = v0;
    let a4913 = v8;
    if nat_lt(a4912, a4913) {
        let v9: u64 = {
            let a4914 = v0;
            let a4915 = v7;
            let a4916 = 0u64;
            nat_quot(a4914, a4915, a4916)
        };
        Ok(Adt22::C0({
            let a4920 = {
                let a4917 = v9;
                let a4918 = v4;
                let a4919 = 0u64;
                nat_quot(a4917, a4918, a4919)
            };
            let a4921 = v6;
            nat_mul(a4920, a4921)?
        }, v6, {
            let a4922 = v9;
            let a4923 = v4;
            let a4924 = 0u64;
            nat_rem(a4922, a4923, a4924)
        }, {
            let a4925 = v0;
            let a4926 = v7;
            let a4927 = 0u64;
            nat_rem(a4925, a4926, a4927)
        }))
    } else {
        let v10: u64 = {
            let a4928 = v0;
            let a4929 = v8;
            nat_sub(a4928, a4929)
        };
        let v11: u64 = f232(v1, v2, f233(v3, v5)?)?;
        Ok(Adt22::C0({
            let a4930 = f231(v3, v5)?;
            let a4931 = v6;
            nat_mul(a4930, a4931)?
        }, f233(v3, v5)?, {
            let a4932 = v10;
            let a4933 = v11;
            let a4934 = 0u64;
            nat_quot(a4932, a4933, a4934)
        }, {
            let a4935 = v10;
            let a4936 = v11;
            let a4937 = 0u64;
            nat_rem(a4935, a4936, a4937)
        }))
    }
}

pub fn f244(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h4938) => {
            h4938
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h4939, _, _, _, _, _, _, _) => {
            h4939
        }
    };
    let v6: u64 = {
        let a4942 = match v0.clone() {
            Adt22::C0(_, h4940, _, _) => {
                h4940
            }
        };
        let a4943 = match v1.clone() {
            Adt20::C0(_, h4941, _, _, _, _, _, _) => {
                h4941
            }
        };
        nat_mul(a4942, a4943)?
    };
    let v7: u64 = {
        let a4944 = v6;
        let a4945 = v5;
        nat_mul(a4944, a4945)?
    };
    let v8: u64 = {
        let a4948 = {
            let a4946 = 2u64;
            let a4947 = v5;
            nat_mul(a4946, a4947)?
        };
        let a4949 = 2u64;
        nat_add(a4948, a4949)?
    };
    let v9: u64 = {
        let a4952 = {
            let a4950 = 4u64;
            let a4951 = v5;
            nat_mul(a4950, a4951)?
        };
        let a4953 = 2u64;
        nat_add(a4952, a4953)?
    };
    let v10: u64 = {
        let a4956 = 1u64;
        let a4957 = {
            let a4954 = v6;
            let a4955 = v8;
            nat_mul(a4954, a4955)?
        };
        nat_add(a4956, a4957)?
    };
    let v11: u64 = {
        let a4958 = v10;
        let a4959 = 4u64;
        nat_add(a4958, a4959)?
    };
    let v12: u64 = {
        let a4962 = v11;
        let a4963 = {
            let a4960 = v6;
            let a4961 = v9;
            nat_mul(a4960, a4961)?
        };
        nat_add(a4962, a4963)?
    };
    let v13: u64 = {
        let a4964 = v12;
        let a4965 = 6u64;
        nat_add(a4964, a4965)?
    };
    let a4966 = v4;
    let a4967 = 0u64;
    if nat_eq(a4966, a4967) {
        Ok(f249(0u64))
    } else {
        let a4968 = v4;
        let a4969 = v10;
        if nat_lt(a4968, a4969) {
            let v14: u64 = {
                let a4970 = v4;
                let a4971 = 1u64;
                nat_sub(a4970, a4971)
            };
            f260(false, {
                let a4972 = v14;
                let a4973 = v8;
                let a4974 = 0u64;
                nat_rem(a4972, a4973, a4974)
            }, {
                let a4975 = v14;
                let a4976 = v8;
                let a4977 = 0u64;
                nat_quot(a4975, a4976, a4977)
            }, v0, v1, v2)
        } else {
            let a4978 = v4;
            let a4979 = v11;
            if nat_lt(a4978, a4979) {
                Ok(f261({
                    let a4980 = v4;
                    let a4981 = v10;
                    nat_sub(a4980, a4981)
                }, v7))
            } else {
                let a4982 = v4;
                let a4983 = v12;
                if nat_lt(a4982, a4983) {
                    let v15: u64 = {
                        let a4984 = v4;
                        let a4985 = v11;
                        nat_sub(a4984, a4985)
                    };
                    f260(true, {
                        let a4986 = v15;
                        let a4987 = v9;
                        let a4988 = 0u64;
                        nat_rem(a4986, a4987, a4988)
                    }, {
                        let a4989 = v15;
                        let a4990 = v9;
                        let a4991 = 0u64;
                        nat_quot(a4989, a4990, a4991)
                    }, v0, v1, v2)
                } else {
                    let a4992 = v4;
                    let a4993 = v13;
                    if nat_lt(a4992, a4993) {
                        Ok(f262({
                            let a4994 = v4;
                            let a4995 = v12;
                            nat_sub(a4994, a4995)
                        }, v7, v3))
                    } else {
                        let a4998 = v4;
                        let a4999 = {
                            let a4996 = v13;
                            let a4997 = v7;
                            nat_add(a4996, a4997)?
                        };
                        if nat_lt(a4998, a4999) {
                            let v16: u64 = {
                                let a5000 = v4;
                                let a5001 = v13;
                                nat_sub(a5000, a5001)
                            };
                            let v17: u64 = {
                                let a5002 = v16;
                                let a5003 = v5;
                                let a5004 = 0u64;
                                nat_quot(a5002, a5003, a5004)
                            };
                            let v18: u64 = {
                                let a5010 = {
                                    let a5005 = f263(v17, v0, v1)?;
                                    let a5006 = v5;
                                    nat_mul(a5005, a5006)?
                                };
                                let a5011 = {
                                    let a5007 = v16;
                                    let a5008 = v5;
                                    let a5009 = 0u64;
                                    nat_rem(a5007, a5008, a5009)
                                };
                                nat_add(a5010, a5011)?
                            };
                            let v19: u64 = f250(v18, v2)?;
                            Ok(f251(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f245(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f246(v0: u64) -> u64 {
    let a5012 = v0;
    let a5013 = 4u64;
    let a5014 = 0u64;
    nat_quot(a5012, a5013, a5014)
}

pub fn f247(v0: u64) -> R<u64> {
    let a5017 = {
        let a5015 = 7u64;
        let a5016 = v0;
        nat_mul(a5015, a5016)?
    };
    let a5018 = 4u64;
    nat_add(a5017, a5018)
}

pub fn f248(v0: u64) -> u64 {
    if f221(v0) {
        let m5019 = f106(v0);
        match m5019 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5021 = 1u64;
                let a5022 = match v1.clone() {
                    Adt18::C0(_, h5020, _) => {
                        h5020
                    }
                };
                if if nat_lt(a5021, a5022) {
                    true
                } else {
                    let a5023 = v0;
                    let a5024 = 43u64;
                    nat_eq(a5023, a5024)
                } {
                    match v1 {
                        Adt18::C0(_, h5025, _) => {
                            h5025
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

pub fn f249(v0: u64) -> Adt19 {
    f251(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f250(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5027 = v0;
        let a5028 = match v1.clone() {
            Adt20::C0(h5026, _, _, _, _, _, _, _) => {
                h5026
            }
        };
        let a5029 = 0u64;
        nat_quot(a5027, a5028, a5029)
    };
    let v3: u64 = {
        let a5031 = v2;
        let a5032 = match v1.clone() {
            Adt20::C0(_, h5030, _, _, _, _, _, _) => {
                h5030
            }
        };
        let a5033 = 0u64;
        nat_quot(a5031, a5032, a5033)
    };
    f120({
        let a5035 = v0;
        let a5036 = match v1.clone() {
            Adt20::C0(h5034, _, _, _, _, _, _, _) => {
                h5034
            }
        };
        let a5037 = 0u64;
        nat_rem(a5035, a5036, a5037)
    }, {
        let a5039 = v2;
        let a5040 = match v1.clone() {
            Adt20::C0(_, h5038, _, _, _, _, _, _) => {
                h5038
            }
        };
        let a5041 = 0u64;
        nat_rem(a5039, a5040, a5041)
    }, {
        let a5043 = v3;
        let a5044 = match v1.clone() {
            Adt20::C0(_, _, h5042, _, _, _, _, _) => {
                h5042
            }
        };
        let a5045 = 0u64;
        nat_rem(a5043, a5044, a5045)
    }, {
        let a5047 = v3;
        let a5048 = match v1.clone() {
            Adt20::C0(_, _, h5046, _, _, _, _, _) => {
                h5046
            }
        };
        let a5049 = 0u64;
        nat_quot(a5047, a5048, a5049)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5050, _, _, _) => {
            h5050
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5051, _, _) => {
            h5051
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5052, _) => {
            h5052
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5053) => {
            h5053
        }
    })
}

pub fn f251(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5054 = v0;
    let a5055 = 4u64;
    if if if nat_le(a5054, a5055) {
        true
    } else {
        let a5056 = 9u64;
        let a5057 = v0;
        if nat_le(a5056, a5057) {
            let a5058 = v0;
            let a5059 = 12u64;
            nat_le(a5058, a5059)
        } else {
            false
        }
    } {
        true
    } else {
        let a5060 = v0;
        let a5061 = 16u64;
        if if nat_eq(a5060, a5061) {
            true
        } else {
            let a5062 = v0;
            let a5063 = 19u64;
            nat_eq(a5062, a5063)
        } {
            true
        } else {
            let a5064 = v0;
            let a5065 = 21u64;
            if nat_eq(a5064, a5065) {
                true
            } else {
                let a5066 = v0;
                let a5067 = 22u64;
                nat_eq(a5066, a5067)
            }
        }
    } {
        Adt19::C1({
            let a5068 = v0;
            let a5069 = 4u64;
            if if nat_lt(a5068, a5069) {
                true
            } else {
                let a5070 = 21u64;
                let a5071 = v0;
                nat_le(a5070, a5071)
            } {
                0u64
            } else {
                let a5072 = v0;
                let a5073 = 16u64;
                if if nat_eq(a5072, a5073) {
                    true
                } else {
                    let a5074 = v0;
                    let a5075 = 19u64;
                    nat_eq(a5074, a5075)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5076 = v0;
            let a5077 = 4u64;
            if nat_lt(a5076, a5077) {
                v0
            } else {
                let a5078 = v0;
                let a5079 = 4u64;
                if if nat_eq(a5078, a5079) {
                    true
                } else {
                    let a5080 = v0;
                    let a5081 = 16u64;
                    nat_eq(a5080, a5081)
                } {
                    0u64
                } else {
                    let a5082 = v0;
                    let a5083 = 19u64;
                    if nat_eq(a5082, a5083) {
                        3u64
                    } else {
                        let a5084 = 21u64;
                        let a5085 = v0;
                        if nat_le(a5084, a5085) {
                            let a5086 = v0;
                            let a5087 = 17u64;
                            nat_sub(a5086, a5087)
                        } else {
                            let a5088 = v0;
                            let a5089 = 8u64;
                            nat_sub(a5088, a5089)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f252(v0: u64) -> R<u64> {
    let a5092 = 2u64;
    let a5093 = {
        let a5090 = 6u64;
        let a5091 = v0;
        nat_mul(a5090, a5091)?
    };
    nat_add(a5092, a5093)
}

pub fn f253(v0: u64) -> R<u64> {
    let a5094 = f252(v0)?;
    let a5095 = 11u64;
    nat_add(a5094, a5095)
}

pub fn f254(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5096 = v0;
        let a5097 = 6u64;
        let a5098 = 0u64;
        nat_rem(a5096, a5097, a5098)
    };
    let v5: u64 = {
        let a5105 = {
            let a5100 = v1;
            let a5101 = match v2.clone() {
                Adt20::C0(h5099, _, _, _, _, _, _, _) => {
                    h5099
                }
            };
            nat_mul(a5100, a5101)?
        };
        let a5106 = {
            let a5102 = v0;
            let a5103 = 6u64;
            let a5104 = 0u64;
            nat_quot(a5102, a5103, a5104)
        };
        nat_add(a5105, a5106)?
    };
    let v6: u64 = f250(v5, v3)?;
    let v7: u64 = f250(v5, v2)?;
    let a5107 = v4;
    let a5108 = 0u64;
    if nat_eq(a5107, a5108) {
        Ok(f251(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5109 = v4;
        let a5110 = 1u64;
        if nat_eq(a5109, a5110) {
            Ok(f251(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5111 = v4;
            let a5112 = 2u64;
            if nat_eq(a5111, a5112) {
                Ok(f251(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5113 = v4;
                let a5114 = 3u64;
                if nat_eq(a5113, a5114) {
                    Ok(f251(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5115 = v4;
                    let a5116 = 4u64;
                    if nat_eq(a5115, a5116) {
                        Ok(f251(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f251(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f255(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5117 = v0;
    let a5118 = 0u64;
    if nat_eq(a5117, a5118) {
        f251(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5119 = v0;
        let a5120 = 1u64;
        if nat_eq(a5119, a5120) {
            f251(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5121 = v0;
            let a5122 = 2u64;
            if nat_eq(a5121, a5122) {
                f251(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5123 = v0;
                let a5124 = 3u64;
                if nat_eq(a5123, a5124) {
                    f251(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5125 = v0;
                    let a5126 = 4u64;
                    if nat_eq(a5125, a5126) {
                        f251(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5127 = v0;
                        let a5128 = 5u64;
                        if nat_eq(a5127, a5128) {
                            f251(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5129 = v0;
                            let a5130 = 6u64;
                            if nat_eq(a5129, a5130) {
                                f251(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5131 = v0;
                                let a5132 = 7u64;
                                if nat_eq(a5131, a5132) {
                                    f251(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5133 = v0;
                                    let a5134 = 8u64;
                                    if nat_eq(a5133, a5134) {
                                        f251(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5135 = v0;
                                        let a5136 = 9u64;
                                        if nat_eq(a5135, a5136) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5137 = v0;
                                            let a5138 = 10u64;
                                            if nat_eq(a5137, a5138) {
                                                f251(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f256(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5139 = v0;
        let a5140 = 3u64;
        let a5141 = 0u64;
        nat_rem(a5139, a5140, a5141)
    };
    let v6: u64 = {
        let a5148 = {
            let a5143 = v1;
            let a5144 = match v2.clone() {
                Adt20::C0(h5142, _, _, _, _, _, _, _) => {
                    h5142
                }
            };
            nat_mul(a5143, a5144)?
        };
        let a5149 = {
            let a5145 = v0;
            let a5146 = 3u64;
            let a5147 = 0u64;
            nat_quot(a5145, a5146, a5147)
        };
        nat_add(a5148, a5149)?
    };
    let v7: u64 = f250(v6, v3)?;
    let v8: u64 = f250(v6, v2)?;
    let v9: u64 = f250(v6, v4)?;
    let a5150 = v5;
    let a5151 = 0u64;
    if nat_eq(a5150, a5151) {
        Ok(f251(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5152 = v5;
        let a5153 = 1u64;
        if nat_eq(a5152, a5153) {
            Ok(f251(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f251(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5154, _, _, _, _, _, _, _) => {
            h5154
        }
    };
    let v5: u64 = {
        let a5157 = 1u64;
        let a5158 = {
            let a5155 = v4;
            let a5156 = f245(v3);
            nat_mul(a5155, a5156)?
        };
        nat_add(a5157, a5158)?
    };
    let a5159 = v0;
    let a5160 = 0u64;
    if nat_eq(a5159, a5160) {
        Ok(f249(0u64))
    } else {
        let a5161 = v0;
        let a5162 = v5;
        if nat_lt(a5161, a5162) {
            let v6: u64 = {
                let a5163 = v0;
                let a5164 = 1u64;
                nat_sub(a5163, a5164)
            };
            let v7: u64 = f250({
                let a5170 = {
                    let a5165 = v1;
                    let a5166 = v4;
                    nat_mul(a5165, a5166)?
                };
                let a5171 = {
                    let a5167 = v6;
                    let a5168 = f245(v3);
                    let a5169 = 0u64;
                    nat_quot(a5167, a5168, a5169)
                };
                nat_add(a5170, a5171)?
            }, v2)?;
            if v3 {
                let a5175 = {
                    let a5172 = v6;
                    let a5173 = 2u64;
                    let a5174 = 0u64;
                    nat_rem(a5172, a5173, a5174)
                };
                let a5176 = 0u64;
                if nat_eq(a5175, a5176) {
                    Ok(f251(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f251(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f251(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5177 = v0;
                let a5178 = v5;
                nat_sub(a5177, a5178)
            };
            let a5179 = v8;
            let a5180 = 0u64;
            if nat_eq(a5179, a5180) {
                Ok(f251(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5181 = v8;
                    let a5182 = 1u64;
                    nat_eq(a5181, a5182)
                } else {
                    false
                } {
                    Ok(f251(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5183 = v8;
                    let a5184 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5183, a5184) {
                        Ok(f251(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5185 = v8;
                        let a5186 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5185, a5186) {
                            Ok(f249(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5188 = 13u64;
        let a5189 = f246(match v2.clone() {
            Adt20::C0(h5187, _, _, _, _, _, _, _) => {
                h5187
            }
        });
        nat_mul(a5188, a5189)?
    } else {
        0u64
    };
    let a5190 = v0;
    let a5191 = v5;
    if nat_lt(a5190, a5191) {
        f264({
            let a5192 = v0;
            let a5193 = 13u64;
            let a5194 = 0u64;
            nat_rem(a5192, a5193, a5194)
        }, v1, {
            let a5195 = v0;
            let a5196 = 13u64;
            let a5197 = 0u64;
            nat_quot(a5195, a5196, a5197)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5198 = v0;
            let a5199 = v5;
            nat_sub(a5198, a5199)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f265({
            let a5200 = v6;
            let a5201 = v7;
            let a5202 = 0u64;
            nat_rem(a5200, a5201, a5202)
        }, {
            let a5212 = {
                let a5207 = {
                    let a5204 = v1;
                    let a5205 = match v2.clone() {
                        Adt20::C0(h5203, _, _, _, _, _, _, _) => {
                            h5203
                        }
                    };
                    nat_mul(a5204, a5205)?
                };
                let a5208 = if v4 {
                    f266(match v2.clone() {
                        Adt20::C0(h5206, _, _, _, _, _, _, _) => {
                            h5206
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5207, a5208)?
            };
            let a5213 = {
                let a5209 = v6;
                let a5210 = v7;
                let a5211 = 0u64;
                nat_quot(a5209, a5210, a5211)
            };
            nat_add(a5212, a5213)?
        }, v2, v3, v4)
    }
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5214 = v0;
    let a5215 = 0u64;
    if nat_eq(a5214, a5215) {
        f251(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5216 = v0;
        let a5217 = 1u64;
        if nat_eq(a5216, a5217) {
            f251(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5218 = v0;
                let a5219 = 2u64;
                nat_eq(a5218, a5219)
            } else {
                false
            } {
                f251(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5220 = v0;
                let a5221 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5220, a5221) {
                    f251(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5222 = v0;
                    let a5223 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5222, a5223) {
                        f251(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5224 = v0;
                        let a5225 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5224, a5225) {
                            f251(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f260(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5229 = 1u64;
        let a5230 = {
            let a5227 = match v4.clone() {
                Adt20::C0(h5226, _, _, _, _, _, _, _) => {
                    h5226
                }
            };
            let a5228 = v6;
            nat_mul(a5227, a5228)?
        };
        nat_add(a5229, a5230)?
    };
    let a5231 = v1;
    let a5232 = 0u64;
    if nat_eq(a5231, a5232) {
        Ok(f249(1u64))
    } else {
        let a5233 = v1;
        let a5234 = v7;
        if nat_lt(a5233, a5234) {
            let v8: u64 = {
                let a5235 = v1;
                let a5236 = 1u64;
                nat_sub(a5235, a5236)
            };
            let v9: u64 = {
                let a5243 = {
                    let a5238 = f263(v2, v3, v4.clone())?;
                    let a5239 = match v4.clone() {
                        Adt20::C0(h5237, _, _, _, _, _, _, _) => {
                            h5237
                        }
                    };
                    nat_mul(a5238, a5239)?
                };
                let a5244 = {
                    let a5240 = v8;
                    let a5241 = v6;
                    let a5242 = 0u64;
                    nat_quot(a5240, a5241, a5242)
                };
                nat_add(a5243, a5244)?
            };
            let v10: u64 = f250(v9, v4)?;
            let v11: u64 = f250(v9, v5)?;
            let v12: u64 = {
                let a5245 = v8;
                let a5246 = v6;
                let a5247 = 0u64;
                nat_rem(a5245, a5246, a5247)
            };
            if v0 {
                let a5248 = v12;
                let a5249 = 0u64;
                if nat_eq(a5248, a5249) {
                    Ok(f251(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5250 = v12;
                    let a5251 = 1u64;
                    if nat_eq(a5250, a5251) {
                        Ok(f251(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5252 = v12;
                        let a5253 = 2u64;
                        if nat_eq(a5252, a5253) {
                            Ok(f251(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f251(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5254 = v12;
                let a5255 = 0u64;
                if nat_eq(a5254, a5255) {
                    Ok(f251(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f251(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5256 = v1;
            let a5257 = v7;
            if nat_eq(a5256, a5257) {
                Ok(f251(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f261(v0: u64, v1: u64) -> Adt19 {
    let a5258 = v0;
    let a5259 = 0u64;
    if nat_eq(a5258, a5259) {
        f251(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5260 = v0;
        let a5261 = 1u64;
        if nat_eq(a5260, a5261) {
            f251(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5262 = v0;
            let a5263 = 2u64;
            if nat_eq(a5262, a5263) {
                f251(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5264 = v0;
                let a5265 = 3u64;
                if nat_eq(a5264, a5265) {
                    f249(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f262(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5266 = v0;
    let a5267 = 0u64;
    if nat_eq(a5266, a5267) {
        f251(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5268 = v0;
        let a5269 = 1u64;
        if nat_eq(a5268, a5269) {
            f251(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5270 = v0;
            let a5271 = 2u64;
            if nat_eq(a5270, a5271) {
                f251(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5272 = v0;
                let a5273 = 3u64;
                if nat_eq(a5272, a5273) {
                    f251(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5274 = v0;
                    let a5275 = 4u64;
                    if nat_eq(a5274, a5275) {
                        f251(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5276 = v0;
                        let a5277 = 5u64;
                        if nat_eq(a5276, a5277) {
                            f251(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f263(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5288 = {
        let a5286 = {
            let a5283 = {
                let a5280 = match v1.clone() {
                    Adt22::C0(_, _, h5278, _) => {
                        h5278
                    }
                };
                let a5281 = match v2.clone() {
                    Adt20::C0(_, _, h5279, _, _, _, _, _) => {
                        h5279
                    }
                };
                nat_mul(a5280, a5281)?
            };
            let a5284 = match v1 {
                Adt22::C0(h5282, _, _, _) => {
                    h5282
                }
            };
            nat_add(a5283, a5284)?
        };
        let a5287 = match v2 {
            Adt20::C0(_, h5285, _, _, _, _, _, _) => {
                h5285
            }
        };
        nat_mul(a5286, a5287)?
    };
    let a5289 = v0;
    nat_add(a5288, a5289)
}

pub fn f264(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5297 = {
            let a5295 = {
                let a5291 = v1;
                let a5292 = match v3.clone() {
                    Adt20::C0(h5290, _, _, _, _, _, _, _) => {
                        h5290
                    }
                };
                nat_mul(a5291, a5292)?
            };
            let a5296 = {
                let a5293 = v2;
                let a5294 = 4u64;
                nat_mul(a5293, a5294)?
            };
            nat_add(a5295, a5296)?
        };
        let a5298 = f267(v0);
        nat_add(a5297, a5298)?
    };
    let v6: u64 = f250(v5, v3)?;
    let v7: u64 = f250(v5, v4)?;
    let a5299 = v0;
    let a5300 = 0u64;
    if if if if nat_eq(a5299, a5300) {
        true
    } else {
        let a5301 = v0;
        let a5302 = 2u64;
        nat_eq(a5301, a5302)
    } {
        true
    } else {
        let a5303 = v0;
        let a5304 = 5u64;
        nat_eq(a5303, a5304)
    } {
        true
    } else {
        let a5305 = v0;
        let a5306 = 7u64;
        nat_eq(a5305, a5306)
    } {
        Ok(f251(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5307 = v0;
        let a5308 = 1u64;
        if if if if nat_eq(a5307, a5308) {
            true
        } else {
            let a5309 = v0;
            let a5310 = 3u64;
            nat_eq(a5309, a5310)
        } {
            true
        } else {
            let a5311 = v0;
            let a5312 = 6u64;
            nat_eq(a5311, a5312)
        } {
            true
        } else {
            let a5313 = v0;
            let a5314 = 8u64;
            nat_eq(a5313, a5314)
        } {
            Ok(f251(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5315 = v0;
                let a5316 = 1u64;
                if nat_eq(a5315, a5316) {
                    4u64
                } else {
                    let a5317 = v0;
                    let a5318 = 6u64;
                    if nat_eq(a5317, a5318) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5319 = v0;
            let a5320 = 4u64;
            if nat_eq(a5319, a5320) {
                Ok(f251(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5321 = v0;
                let a5322 = 9u64;
                if nat_eq(a5321, a5322) {
                    Ok(f251(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5323 = v0;
                    let a5324 = 10u64;
                    if nat_eq(a5323, a5324) {
                        Ok(f251(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5325 = v0;
                        let a5326 = 11u64;
                        if nat_eq(a5325, a5326) {
                            Ok(f251(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5327 = v0;
                            let a5328 = 12u64;
                            if nat_eq(a5327, a5328) {
                                Ok(f251(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f265(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f250(v1, v2)?;
    let v6: u64 = f250(v1, v3)?;
    let a5329 = v0;
    let a5330 = 0u64;
    if nat_eq(a5329, a5330) {
        Ok(f251(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5331 = v0;
        let a5332 = 1u64;
        if nat_eq(a5331, a5332) {
            Ok(f251(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5333 = v0;
                let a5334 = 2u64;
                if nat_eq(a5333, a5334) {
                    Ok(f251(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5335 = v0;
                    let a5336 = 3u64;
                    if nat_eq(a5335, a5336) {
                        Ok(f251(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5337 = v0;
                let a5338 = 2u64;
                if nat_eq(a5337, a5338) {
                    Ok(f251(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f266(v0: u64) -> R<u64> {
    let a5339 = 4u64;
    let a5340 = f246(v0);
    nat_mul(a5339, a5340)
}

pub fn f267(v0: u64) -> u64 {
    let a5341 = v0;
    let a5342 = 5u64;
    if nat_lt(a5341, a5342) {
        let a5343 = v0;
        let a5344 = 2u64;
        if nat_lt(a5343, a5344) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5345 = v0;
        let a5346 = 7u64;
        if nat_lt(a5345, a5346) {
            2u64
        } else {
            3u64
        }
    }
}
