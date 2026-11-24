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
    C33(Adt3),
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
        let a465 = 22u64;
        if nat_eq(a464, a465) {
            let m466 = v3;
            match m466 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f41(v2, v6, v4)?
                }
            }
        } else {
            let a467 = v0;
            let a468 = 44u64;
            if nat_eq(a467, a468) {
                f42(v1, v2, v3, v4)?
            } else {
                let a469 = v0;
                let a470 = 52u64;
                if nat_eq(a469, a470) {
                    f43(v1, v2, v4)?
                } else {
                    let a471 = v0;
                    let a472 = 73u64;
                    if nat_eq(a471, a472) {
                        f44(v2, v3, v4)?
                    } else {
                        let a473 = v0;
                        let a474 = 57u64;
                        if if nat_eq(a473, a474) {
                            true
                        } else {
                            let a475 = v0;
                            let a476 = 58u64;
                            nat_eq(a475, a476)
                        } {
                            let m477 = v3;
                            match m477 {
                                None => {
                                    f45({
                                        let a478 = v0;
                                        let a479 = 58u64;
                                        nat_eq(a478, a479)
                                    }, v1, v2, v4)?
                                }
                                Some(_) => {
                                    Adt10::C1
                                }
                            }
                        } else {
                            let a480 = v0;
                            let a481 = 49u64;
                            if nat_eq(a480, a481) {
                                f46(v1, v2, v3, v4)?
                            } else {
                                let a482 = v0;
                                let a483 = 51u64;
                                if nat_eq(a482, a483) {
                                    f47(v1, v2, v3, v4)?
                                } else {
                                    let a484 = v0;
                                    let a485 = 39u64;
                                    if nat_eq(a484, a485) {
                                        f48(v2, v3, v4)?
                                    } else {
                                        let a486 = v0;
                                        let a487 = 77u64;
                                        if nat_eq(a486, a487) {
                                            f49(v2, v4)?
                                        } else {
                                            let a488 = v0;
                                            let a489 = 40u64;
                                            if nat_eq(a488, a489) {
                                                f50(v2, v3, v4)?
                                            } else {
                                                if f51(v0) {
                                                    let a490 = v0;
                                                    let a491 = 33u64;
                                                    if if if nat_eq(a490, a491) {
                                                        let a493 = match v4.clone() {
                                                            Adt2::C0(h492, _, _, _, _, _, _, _, _, _, _) => {
                                                                h492
                                                            }
                                                        };
                                                        let a494 = 0u64;
                                                        nat_eq(a493, a494)
                                                    } else {
                                                        false
                                                    } {
                                                        f52(match v2.clone() {
                                                            Adt2::C0(h495, _, _, _, _, _, _, _, _, _, _) => {
                                                                h495
                                                            }
                                                        })
                                                    } else {
                                                        false
                                                    } {
                                                        f53(v2, v3, v4)?
                                                    } else {
                                                        f54(v0, v2, {
                                                            let m496 = v3;
                                                            match m496 {
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
                                                    let a497 = v0;
                                                    let a498 = 19u64;
                                                    if nat_eq(a497, a498) {
                                                        f55(v2, v4)?
                                                    } else {
                                                        let a499 = v0;
                                                        let a500 = 21u64;
                                                        if nat_eq(a499, a500) {
                                                            f56(v5, v2, v3, v4)?
                                                        } else {
                                                            let a501 = v0;
                                                            let a502 = 61u64;
                                                            if if if nat_eq(a501, a502) {
                                                                true
                                                            } else {
                                                                let a503 = v0;
                                                                let a504 = 62u64;
                                                                nat_eq(a503, a504)
                                                            } {
                                                                true
                                                            } else {
                                                                let a505 = v0;
                                                                let a506 = 63u64;
                                                                nat_eq(a505, a506)
                                                            } {
                                                                f57(v0, v1, v2, v4)?
                                                            } else {
                                                                let a507 = v0;
                                                                let a508 = 4u64;
                                                                if if nat_eq(a507, a508) {
                                                                    true
                                                                } else {
                                                                    let a509 = v0;
                                                                    let a510 = 32u64;
                                                                    nat_eq(a509, a510)
                                                                } {
                                                                    f58(v0, v1, v2, v3, v4)?
                                                                } else {
                                                                    let a511 = v0;
                                                                    let a512 = 20u64;
                                                                    if nat_eq(a511, a512) {
                                                                        f59(v2, v4)?
                                                                    } else {
                                                                        let a513 = v0;
                                                                        let a514 = 42u64;
                                                                        if nat_eq(a513, a514) {
                                                                            f60(4u64, v2, v4)?
                                                                        } else {
                                                                            let a515 = v0;
                                                                            let a516 = 69u64;
                                                                            if nat_eq(a515, a516) {
                                                                                let a517 = v5;
                                                                                let a518 = 4u64;
                                                                                if nat_lt(a517, a518) {
                                                                                    f60(v5, v2, v4)?
                                                                                } else {
                                                                                    Adt10::C1
                                                                                }
                                                                            } else {
                                                                                let a519 = v0;
                                                                                let a520 = 70u64;
                                                                                if nat_eq(a519, a520) {
                                                                                    f61(v5, v2, v4)?
                                                                                } else {
                                                                                    let a521 = v0;
                                                                                    let a522 = 75u64;
                                                                                    if if nat_eq(a521, a522) {
                                                                                        true
                                                                                    } else {
                                                                                        let a523 = v0;
                                                                                        let a524 = 76u64;
                                                                                        nat_eq(a523, a524)
                                                                                    } {
                                                                                        f62({
                                                                                            let a525 = v0;
                                                                                            let a526 = 76u64;
                                                                                            nat_eq(a525, a526)
                                                                                        }, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a527 = v0;
                                                                                        let a528 = 31u64;
                                                                                        if if if nat_eq(a527, a528) {
                                                                                            true
                                                                                        } else {
                                                                                            let a529 = v0;
                                                                                            let a530 = 47u64;
                                                                                            nat_eq(a529, a530)
                                                                                        } {
                                                                                            true
                                                                                        } else {
                                                                                            let a531 = v0;
                                                                                            let a532 = 68u64;
                                                                                            nat_eq(a531, a532)
                                                                                        } {
                                                                                            f63(v0, v1, v2, v4)?
                                                                                        } else {
                                                                                            let a533 = v0;
                                                                                            let a534 = 15u64;
                                                                                            if nat_eq(a533, a534) {
                                                                                                f64(v2, v4)?
                                                                                            } else {
                                                                                                let a535 = v0;
                                                                                                let a536 = 17u64;
                                                                                                if nat_eq(a535, a536) {
                                                                                                    f65(v2, v4)?
                                                                                                } else {
                                                                                                    let a537 = v0;
                                                                                                    let a538 = 18u64;
                                                                                                    if nat_eq(a537, a538) {
                                                                                                        f66(v2, v3, v4)?
                                                                                                    } else {
                                                                                                        let a539 = v0;
                                                                                                        let a540 = 13u64;
                                                                                                        if if if nat_eq(a539, a540) {
                                                                                                            true
                                                                                                        } else {
                                                                                                            let a541 = v0;
                                                                                                            let a542 = 14u64;
                                                                                                            nat_eq(a541, a542)
                                                                                                        } {
                                                                                                            true
                                                                                                        } else {
                                                                                                            let a543 = v0;
                                                                                                            let a544 = 16u64;
                                                                                                            nat_eq(a543, a544)
                                                                                                        } {
                                                                                                            f67(v0, v2, v4)?
                                                                                                        } else {
                                                                                                            let m545 = f68(v0, v5)?;
                                                                                                            match m545 {
                                                                                                                None => {
                                                                                                                    f69(v0, v5, v2, v3, v4)?
                                                                                                                }
                                                                                                                Some(v9) => {
                                                                                                                    f70(v9, v2, v4)?
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h546, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h546
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h547, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h547
        }
    };
    let v6: u64 = {
        let a548 = v3;
        let a549 = v5;
        let a550 = 0u64;
        nat_rem(a548, a549, a550)
    };
    let a551 = v1;
    let a552 = 0u64;
    if nat_eq(a551, a552) {
        let a554 = v3;
        let a555 = match v0.clone() {
            Adt8::C0(_, _, _, h553, _, _, _, _, _, _, _, _, _, _, _) => {
                h553
            }
        };
        if nat_lt(a554, a555) {
            Ok(Adt9::C8(Some(f71(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h556, _, _, _, _, _, _) => {
                    h556
                }
            }, {
                let a560 = v6;
                let a561 = {
                    let m559 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h558, _, _, _, _, _, _) => {
                            h558
                        }
                    };
                    match m559 {
                        Adt3::C0(h557, _, _, _, _, _, _, _) => {
                            h557
                        }
                    }
                };
                let a562 = 0u64;
                nat_rem(a560, a561, a562)
            }, {
                let a572 = {
                    let a566 = v6;
                    let a567 = {
                        let m565 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h564, _, _, _, _, _, _) => {
                                h564
                            }
                        };
                        match m565 {
                            Adt3::C0(h563, _, _, _, _, _, _, _) => {
                                h563
                            }
                        }
                    };
                    let a568 = 0u64;
                    nat_quot(a566, a567, a568)
                };
                let a573 = {
                    let m571 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h570, _, _, _, _, _, _) => {
                            h570
                        }
                    };
                    match m571 {
                        Adt3::C0(_, h569, _, _, _, _, _, _) => {
                            h569
                        }
                    }
                };
                let a574 = 0u64;
                nat_rem(a572, a573, a574)
            }, {
                let a584 = {
                    let a578 = v6;
                    let a579 = {
                        let m577 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h576, _, _, _, _, _, _) => {
                                h576
                            }
                        };
                        match m577 {
                            Adt3::C0(h575, _, _, _, _, _, _, _) => {
                                h575
                            }
                        }
                    };
                    let a580 = 0u64;
                    nat_quot(a578, a579, a580)
                };
                let a585 = {
                    let m583 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h582, _, _, _, _, _, _) => {
                            h582
                        }
                    };
                    match m583 {
                        Adt3::C0(_, h581, _, _, _, _, _, _) => {
                            h581
                        }
                    }
                };
                let a586 = 0u64;
                nat_quot(a584, a585, a586)
            }, {
                let a587 = v3;
                let a588 = v5;
                let a589 = 0u64;
                nat_quot(a587, a588, a589)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a595 = v2;
            let a596 = {
                let a592 = v3;
                let a593 = {
                    let a590 = v1;
                    let a591 = 1u64;
                    if nat_eq(a590, a591) {
                        v4
                    } else {
                        v5
                    }
                };
                let a594 = 0u64;
                nat_quot(a592, a593, a594)
            };
            nat_add(a595, a596)?
        };
        let a598 = v7;
        let a599 = match v0.clone() {
            Adt8::C0(h597, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        if nat_lt(a598, a599) {
            let v8: u64 = {
                let a603 = v7;
                let a604 = {
                    let m602 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h601, _, _, _, _, _) => {
                            h601
                        }
                    };
                    match m602 {
                        Adt3::C0(h600, _, _, _, _, _, _, _) => {
                            h600
                        }
                    }
                };
                let a605 = 0u64;
                nat_rem(a603, a604, a605)
            };
            let v9: u64 = {
                let a609 = v7;
                let a610 = {
                    let m608 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h607, _, _, _, _, _) => {
                            h607
                        }
                    };
                    match m608 {
                        Adt3::C0(h606, _, _, _, _, _, _, _) => {
                            h606
                        }
                    }
                };
                let a611 = 0u64;
                nat_quot(a609, a610, a611)
            };
            let v10: u64 = {
                let a615 = v9;
                let a616 = {
                    let m614 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h613, _, _, _, _, _) => {
                            h613
                        }
                    };
                    match m614 {
                        Adt3::C0(_, h612, _, _, _, _, _, _) => {
                            h612
                        }
                    }
                };
                let a617 = 0u64;
                nat_rem(a615, a616, a617)
            };
            let v11: u64 = {
                let a621 = v9;
                let a622 = {
                    let m620 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h619, _, _, _, _, _) => {
                            h619
                        }
                    };
                    match m620 {
                        Adt3::C0(_, h618, _, _, _, _, _, _) => {
                            h618
                        }
                    }
                };
                let a623 = 0u64;
                nat_quot(a621, a622, a623)
            };
            let a624 = v1;
            let a625 = 1u64;
            if nat_eq(a624, a625) {
                Ok(Adt9::C8(Some(f71(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h626, _, _, _, _, _) => {
                        h626
                    }
                }, v8, v10, {
                    let a627 = v3;
                    let a628 = v4;
                    let a629 = 0u64;
                    nat_rem(a627, a628, a629)
                }, v11)?)))
            } else {
                let a630 = v1;
                let a631 = 2u64;
                if nat_eq(a630, a631) {
                    Ok(Adt9::C8(Some({
                        let a677 = f71(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h632, _, _, _, _) => {
                                h632
                            }
                        }, {
                            let a644 = {
                                let a636 = v8;
                                let a637 = {
                                    let m635 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h634, _, _, _) => {
                                            h634
                                        }
                                    };
                                    match m635 {
                                        Adt6::C0(_, h633, _, _) => {
                                            h633
                                        }
                                    }
                                };
                                nat_mul(a636, a637)?
                            };
                            let a645 = {
                                let a641 = v6;
                                let a642 = {
                                    let m640 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h639, _, _, _, _, _, _) => {
                                            h639
                                        }
                                    };
                                    match m640 {
                                        Adt3::C0(h638, _, _, _, _, _, _, _) => {
                                            h638
                                        }
                                    }
                                };
                                let a643 = 0u64;
                                nat_rem(a641, a642, a643)
                            };
                            nat_add(a644, a645)?
                        }, {
                            let a663 = {
                                let a649 = v10;
                                let a650 = {
                                    let m648 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h647, _, _, _) => {
                                            h647
                                        }
                                    };
                                    match m648 {
                                        Adt6::C0(_, h646, _, _) => {
                                            h646
                                        }
                                    }
                                };
                                nat_mul(a649, a650)?
                            };
                            let a664 = {
                                let a660 = {
                                    let a654 = v6;
                                    let a655 = {
                                        let m653 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h652, _, _, _, _, _, _) => {
                                                h652
                                            }
                                        };
                                        match m653 {
                                            Adt3::C0(h651, _, _, _, _, _, _, _) => {
                                                h651
                                            }
                                        }
                                    };
                                    let a656 = 0u64;
                                    nat_quot(a654, a655, a656)
                                };
                                let a661 = {
                                    let m659 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h658, _, _, _, _, _, _) => {
                                            h658
                                        }
                                    };
                                    match m659 {
                                        Adt3::C0(_, h657, _, _, _, _, _, _) => {
                                            h657
                                        }
                                    }
                                };
                                let a662 = 0u64;
                                nat_rem(a660, a661, a662)
                            };
                            nat_add(a663, a664)?
                        }, {
                            let a674 = {
                                let a668 = v6;
                                let a669 = {
                                    let m667 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h666, _, _, _, _, _, _) => {
                                            h666
                                        }
                                    };
                                    match m667 {
                                        Adt3::C0(h665, _, _, _, _, _, _, _) => {
                                            h665
                                        }
                                    }
                                };
                                let a670 = 0u64;
                                nat_quot(a668, a669, a670)
                            };
                            let a675 = {
                                let m673 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h672, _, _, _, _, _, _) => {
                                        h672
                                    }
                                };
                                match m673 {
                                    Adt3::C0(_, h671, _, _, _, _, _, _) => {
                                        h671
                                    }
                                }
                            };
                            let a676 = 0u64;
                            nat_quot(a674, a675, a676)
                        }, v11)?;
                        let a678 = 4u64;
                        let a679 = 0u64;
                        nat_quot(a677, a678, a679)
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
    let a680 = v1;
    let a681 = 0u64;
    if nat_eq(a680, a681) {
        let a683 = v3;
        let a684 = match v0.clone() {
            Adt8::C0(_, _, _, h682, _, _, _, _, _, _, _, _, _, _, _) => {
                h682
            }
        };
        if nat_lt(a683, a684) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h685, _, _, _, _, _, _) => {
                    h685
                }
            };
            let v5: u64 = {
                let a687 = v3;
                let a688 = match v0.clone() {
                    Adt8::C0(_, _, h686, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h686
                    }
                };
                let a689 = 0u64;
                nat_quot(a687, a688, a689)
            };
            Ok(Adt9::C8(Some(f71(v4.clone(), f72(v5, v4.clone(), 0u64), f72(v5, v4.clone(), 1u64), f72(v5, v4, 2u64), f73(v0.clone(), v5, {
                let a691 = v3;
                let a692 = match v0 {
                    Adt8::C0(_, _, h690, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h690
                    }
                };
                let a693 = 0u64;
                nat_rem(a691, a692, a693)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h694, _, _, _, _) => {
                h694
            }
        };
        let v7: u64 = {
            let a695 = v1;
            let a696 = 1u64;
            if nat_eq(a695, a696) {
                match v0.clone() {
                    Adt8::C0(_, h697, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h697
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h698, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h698
                    }
                }
            }
        };
        let v8: u64 = {
            let a702 = v2;
            let a703 = {
                let a699 = v3;
                let a700 = v7;
                let a701 = 0u64;
                nat_quot(a699, a700, a701)
            };
            nat_add(a702, a703)?
        };
        let v9: u64 = {
            let a704 = v3;
            let a705 = v7;
            let a706 = 0u64;
            nat_rem(a704, a705, a706)
        };
        let v10: u64 = {
            let a708 = v8;
            let a709 = match v6.clone() {
                Adt3::C0(h707, _, _, _, _, _, _, _) => {
                    h707
                }
            };
            let a710 = 0u64;
            nat_rem(a708, a709, a710)
        };
        let v11: u64 = {
            let a712 = v8;
            let a713 = match v6.clone() {
                Adt3::C0(h711, _, _, _, _, _, _, _) => {
                    h711
                }
            };
            let a714 = 0u64;
            nat_quot(a712, a713, a714)
        };
        let v12: u64 = {
            let a716 = v11;
            let a717 = match v6.clone() {
                Adt3::C0(_, h715, _, _, _, _, _, _) => {
                    h715
                }
            };
            let a718 = 0u64;
            nat_rem(a716, a717, a718)
        };
        let v13: u64 = {
            let a720 = v11;
            let a721 = match v6.clone() {
                Adt3::C0(_, h719, _, _, _, _, _, _) => {
                    h719
                }
            };
            let a722 = 0u64;
            nat_quot(a720, a721, a722)
        };
        let a724 = v8;
        let a725 = match v0.clone() {
            Adt8::C0(h723, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h723
            }
        };
        if nat_lt(a724, a725) {
            let a726 = v1;
            let a727 = 1u64;
            if nat_eq(a726, a727) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h728, _, _, _, _, _, _) => {
                        h728
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h729, _, _, _, _, _) => {
                        h729
                    }
                };
                let m732 = f74(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h730, _, _, _) => {
                        h730
                    }
                }, match v15.clone() {
                    Adt3::C0(h731, _, _, _, _, _, _, _) => {
                        h731
                    }
                }, v10, f72(v9, v14.clone(), 0u64))?;
                match m732 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m735 = f74(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h733, _, _) => {
                                h733
                            }
                        }, match v15 {
                            Adt3::C0(_, h734, _, _, _, _, _, _) => {
                                h734
                            }
                        }, v12, f72(v9, v14, 1u64))?;
                        match m735 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f75(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a736 = v1;
                let a737 = 2u64;
                if nat_eq(a736, a737) {
                    Ok(Adt9::C8(Some({
                        let a739 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h738) => {
                                h738
                            }
                        };
                        let a740 = 0u64;
                        if nat_eq(a739, a740) {
                            f71(v6, v10, v12, v9, v13)?
                        } else {
                            f71(v6.clone(), v10, v12, {
                                let a742 = v13;
                                let a743 = match v6.clone() {
                                    Adt3::C0(_, _, h741, _, _, _, _, _) => {
                                        h741
                                    }
                                };
                                let a744 = 0u64;
                                nat_rem(a742, a743, a744)
                            }, {
                                let a752 = {
                                    let a750 = {
                                        let a746 = v13;
                                        let a747 = match v6 {
                                            Adt3::C0(_, _, h745, _, _, _, _, _) => {
                                                h745
                                            }
                                        };
                                        let a748 = 0u64;
                                        nat_quot(a746, a747, a748)
                                    };
                                    let a751 = match v0 {
                                        Adt8::C0(_, _, h749, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h749
                                        }
                                    };
                                    nat_mul(a750, a751)?
                                };
                                let a753 = v9;
                                nat_add(a752, a753)?
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
    Ok(f76(v3.clone(), {
        let m754 = v3.clone();
        match m754 {
            Adt4::C0 => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f81(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C19(v53, v54) => {
                f82(v53, v0, v1, v2, v54, v4)?
            }
            Adt4::C20(v55, v56) => {
                f83(v55, v0, v1, v2, v56, v4)?
            }
            Adt4::C21 => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C22(v57, v58) => {
                f84(v0, v1, v2, v57, v58, v4)?
            }
            Adt4::C23(v72, v73, v74, v75) => {
                f91(v0, v75, v72, v73, v74, v4)?
            }
            Adt4::C24(v71) => {
                if v5 {
                    f89(Adt16::C0(v71), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f77(v0, v1, v2, v3)?
            }
            Adt4::C26(v67, v68, v69, v70) => {
                let a756 = v67;
                let a757 = 64u64;
                if if if nat_eq(a756, a757) {
                    true
                } else {
                    let a758 = v67;
                    let a759 = 65u64;
                    nat_eq(a758, a759)
                } {
                    true
                } else {
                    let a760 = v67;
                    let a761 = 66u64;
                    nat_eq(a760, a761)
                } {
                    if v5 {
                        f89(Adt16::C1(v67, v68, v69, v70), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a762 = v67;
                    let a763 = 0u64;
                    if if nat_eq(a762, a763) {
                        true
                    } else {
                        let a764 = v67;
                        let a765 = 30u64;
                        nat_eq(a764, a765)
                    } {
                        f90(v0, v1, v2, v67, v68, v69, v70, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v66) => {
                f88(v0, v1, v2, v66)?
            }
            Adt4::C28(v62) => {
                f85(v0, v1, v2, v62, v4)?
            }
            Adt4::C29(v63) => {
                f86(v0, v1, v2, v63)?
            }
            Adt4::C30(v64, _) => {
                f87(v0, v1, v2, v64)?
            }
            Adt4::C31(v33, v34) => {
                f78(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f79(v0, match v1 {
                    Adt3::C0(h755, _, _, _, _, _, _, _) => {
                        h755
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f80(v0, v1, v36, v2)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m766 = v0;
    match m766 {
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
        Adt4::C25(v49, _, v51) => {
            Ok(Adt9::C6({
                let a771 = {
                    let a767 = 0u64;
                    let a768 = v49;
                    if nat_lt(a767, a768) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a772 = {
                    let a769 = 0u64;
                    let a770 = v51;
                    if nat_lt(a769, a770) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a771, a772)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v57, _, _, _) => {
            Ok(Adt9::C6({
                let a773 = v57;
                let a774 = 66u64;
                if nat_eq(a773, a774) {
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
        Adt4::C33(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a775 = v1;
    let a776 = 5u64;
    if if if if if nat_lt(a775, a776) {
        true
    } else {
        f92(v0, v1, v2, v3, v4)
    } {
        let a777 = v5;
        let a778 = 5u64;
        nat_lt(a777, a778)
    } else {
        false
    } {
        let a779 = v7;
        let a780 = 5u64;
        nat_lt(a779, a780)
    } else {
        false
    } {
        let a781 = v9;
        let a782 = 2u64;
        if nat_lt(a781, a782) {
            true
        } else {
            let a783 = v9;
            let a784 = 3u64;
            nat_eq(a783, a784)
        }
    } else {
        false
    } {
        Adt9::C7(f93(v0, v1, v2, v3, v4), f94(v5, v6), f94(v7, v8), f95({
            let a785 = v9;
            let a786 = 0u64;
            if nat_eq(a785, a786) {
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
    let a798 = {
        let a792 = {
            let a788 = match v0.clone() {
                Adt2::C0(_, h787, _, _, _, _, _, _, _, _, _) => {
                    h787
                }
            };
            let a789 = v1;
            let a790 = 0u64;
            nat_quot(a788, a789, a790)
        };
        let a793 = match v0.clone() {
            Adt2::C0(_, _, h791, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        nat_mul(a792, a793)?
    };
    let a799 = {
        let a796 = match v0.clone() {
            Adt2::C0(_, _, _, h794, _, _, _, _, _, _, _) => {
                h794
            }
        };
        let a797 = match v0 {
            Adt2::C0(_, _, _, _, h795, _, _, _, _, _, _) => {
                h795
            }
        };
        nat_mul(a796, a797)?
    };
    nat_mul(a798, a799)
}

pub fn f17(v0: u64) -> bool {
    let a800 = v0;
    let a801 = 4294967296u64;
    if nat_lt(a800, a801) {
        let a805 = {
            let a802 = v0;
            let a803 = 2147483648u64;
            let a804 = 0u64;
            nat_rem(a802, a803, a804)
        };
        let a806 = 2139095040u64;
        nat_lt(a805, a806)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a807 = 4u64;
    let a808 = v0;
    nat_mul(a807, a808)
}

pub fn f19(v0: u64) -> R<u64> {
    let a809 = v0;
    let a810 = 0u64;
    if nat_eq(a809, a810) {
        Ok(0u64)
    } else {
        let a813 = 4u64;
        let a814 = {
            let a811 = 3u64;
            let a812 = v0;
            nat_mul(a811, a812)?
        };
        nat_add(a813, a814)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a816 = match v0.clone() {
            Adt2::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
                h815
            }
        };
        let a817 = v1;
        let a818 = 0u64;
        nat_quot(a816, a817, a818)
    }, match v0.clone() {
        Adt2::C0(_, _, h819, _, _, _, _, _, _, _, _) => {
            h819
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h820, _, _, _, _, _, _, _) => {
            h820
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h821, _, _, _, _, _, _) => {
            h821
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h822, _, _, _, _, _) => {
            h822
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h823, _, _, _, _) => {
            h823
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h824, _, _, _) => {
            h824
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h825, _, _) => {
            h825
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                h827
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h828, _, _, _, _, _, _, _) => {
                h828
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h829, _, _, _, _, _) => {
                h829
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h830, _, _, _, _) => {
                h830
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h831, _, _, _) => {
                h831
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a833 = match v0.clone() {
        Adt2::C0(_, h832, _, _, _, _, _, _, _, _, _) => {
            h832
        }
    };
    let a834 = 1u64;
    if if if if nat_eq(a833, a834) {
        true
    } else {
        let a836 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h835, _, _, _, _, _) => {
                h835
            }
        };
        let a837 = 4u64;
        nat_eq(a836, a837)
    } {
        let a839 = match v0.clone() {
            Adt2::C0(_, _, h838, _, _, _, _, _, _, _, _) => {
                h838
            }
        };
        let a840 = 1u64;
        if nat_eq(a839, a840) {
            true
        } else {
            let a845 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h841, _, _, _, _) => {
                    h841
                }
            };
            let a846 = {
                let a843 = match v0.clone() {
                    Adt2::C0(_, h842, _, _, _, _, _, _, _, _, _) => {
                        h842
                    }
                };
                let a844 = 4u64;
                nat_mul(a843, a844)?
            };
            nat_eq(a845, a846)
        }
    } else {
        false
    } {
        let a848 = match v0.clone() {
            Adt2::C0(_, _, _, h847, _, _, _, _, _, _, _) => {
                h847
            }
        };
        let a849 = 1u64;
        if nat_eq(a848, a849) {
            true
        } else {
            let a857 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h850, _, _, _) => {
                    h850
                }
            };
            let a858 = {
                let a855 = {
                    let a853 = match v0.clone() {
                        Adt2::C0(_, h851, _, _, _, _, _, _, _, _, _) => {
                            h851
                        }
                    };
                    let a854 = match v0.clone() {
                        Adt2::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
                            h852
                        }
                    };
                    nat_mul(a853, a854)?
                };
                let a856 = 4u64;
                nat_mul(a855, a856)?
            };
            nat_eq(a857, a858)
        }
    } else {
        false
    } {
        let a860 = match v0.clone() {
            Adt2::C0(_, _, _, _, h859, _, _, _, _, _, _) => {
                h859
            }
        };
        let a861 = 1u64;
        if nat_eq(a860, a861) {
            Ok(true)
        } else {
            let a872 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h862, _, _) => {
                    h862
                }
            };
            let a873 = {
                let a870 = {
                    let a868 = {
                        let a865 = match v0.clone() {
                            Adt2::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
                                h863
                            }
                        };
                        let a866 = match v0.clone() {
                            Adt2::C0(_, _, h864, _, _, _, _, _, _, _, _) => {
                                h864
                            }
                        };
                        nat_mul(a865, a866)?
                    };
                    let a869 = match v0 {
                        Adt2::C0(_, _, _, h867, _, _, _, _, _, _, _) => {
                            h867
                        }
                    };
                    nat_mul(a868, a869)?
                };
                let a871 = 4u64;
                nat_mul(a870, a871)?
            };
            Ok(nat_eq(a872, a873))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a875 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h874, _) => {
            h874
        }
    };
    let a876 = 0u64;
    if nat_eq(a875, a876) {
        Ok(true)
    } else {
        let a878 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h877, _) => {
                h877
            }
        };
        let a879 = 0u64;
        if nat_eq(a878, a879) {
            Ok(true)
        } else {
            let a881 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h880) => {
                    h880
                }
            };
            let a882 = 0u64;
            if nat_eq(a881, a882) {
                Ok(true)
            } else {
                let a884 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h883) => {
                        h883
                    }
                };
                let a885 = 0u64;
                if nat_eq(a884, a885) {
                    Ok(true)
                } else {
                    let a891 = {
                        let a888 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h886) => {
                                h886
                            }
                        };
                        let a889 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h887, _) => {
                                h887
                            }
                        };
                        nat_add(a888, a889)?
                    };
                    let a892 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h890) => {
                            h890
                        }
                    };
                    if nat_le(a891, a892) {
                        Ok(true)
                    } else {
                        let a898 = {
                            let a895 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h893) => {
                                    h893
                                }
                            };
                            let a896 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h894, _) => {
                                    h894
                                }
                            };
                            nat_add(a895, a896)?
                        };
                        let a899 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h897) => {
                                h897
                            }
                        };
                        Ok(nat_le(a898, a899))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a901 = match v0.clone() {
        Adt6::C0(h900, _, _, _) => {
            h900
        }
    };
    let a902 = 0u64;
    if nat_eq(a901, a902) {
        f96(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h903, _, _) => {
                h903
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h904, _) => {
                h904
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h905) => {
                h905
            }
        }), {
            let a907 = v1;
            let a908 = match v0 {
                Adt6::C0(_, _, _, h906) => {
                    h906
                }
            };
            nat_add(a907, a908)?
        })
    } else {
        f96(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a911 = match v0.clone() {
        Adt2::C0(_, h909, _, _, _, _, _, _, _, _, _) => {
            h909
        }
    };
    let a912 = match v1.clone() {
        Adt2::C0(_, h910, _, _, _, _, _, _, _, _, _) => {
            h910
        }
    };
    if if if nat_eq(a911, a912) {
        let a915 = match v0.clone() {
            Adt2::C0(_, _, h913, _, _, _, _, _, _, _, _) => {
                h913
            }
        };
        let a916 = match v1.clone() {
            Adt2::C0(_, _, h914, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        nat_eq(a915, a916)
    } else {
        false
    } {
        let a919 = match v0.clone() {
            Adt2::C0(_, _, _, h917, _, _, _, _, _, _, _) => {
                h917
            }
        };
        let a920 = match v1.clone() {
            Adt2::C0(_, _, _, h918, _, _, _, _, _, _, _) => {
                h918
            }
        };
        nat_eq(a919, a920)
    } else {
        false
    } {
        let a923 = match v0 {
            Adt2::C0(_, _, _, _, h921, _, _, _, _, _, _) => {
                h921
            }
        };
        let a924 = match v1 {
            Adt2::C0(_, _, _, _, h922, _, _, _, _, _, _) => {
                h922
            }
        };
        nat_eq(a923, a924)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a929 = {
        let a925 = v0;
        let a926 = v1;
        nat_mul(a925, a926)?
    };
    let a930 = {
        let a927 = v2;
        let a928 = f97(v3);
        nat_mul(a927, a928)?
    };
    nat_mul(a929, a930)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a933 = v0;
    let a934 = {
        let a931 = f98(v1)?;
        let a932 = v2;
        nat_mul(a931, a932)?
    };
    nat_add(a933, a934)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h935, _, _, _, _, _, _, _, _, _, _) => {
            h935
        }
    }, match v0.clone() {
        Adt2::C0(_, h936, _, _, _, _, _, _, _, _, _) => {
            h936
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
            h937
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h938, _, _, _, _, _, _, _) => {
            h938
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h939, _, _, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h940, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h941, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h942, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h943, _, _) => {
            h943
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h944, _) => {
            h944
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h945) => {
            h945
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a946 = v0;
    let a947 = 23u64;
    if nat_eq(a946, a947) {
        f99(v1, v4, v6)
    } else {
        let a948 = v0;
        let a949 = 26u64;
        if nat_eq(a948, a949) {
            f100(v1, v2, v4, v6)
        } else {
            let a950 = v0;
            let a951 = 25u64;
            if nat_eq(a950, a951) {
                f101(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a952 = v0;
    let a953 = 30u64;
    if nat_eq(a952, a953) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a954 = v0;
    let a955 = 2139095040u64;
    if nat_le(a954, a955) {
        true
    } else {
        let a956 = v0;
        let a957 = 2147483648u64;
        nat_eq(a956, a957)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a959 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h958, _, _, _, _, _) => {
            h958
        }
    };
    let a960 = v1;
    if if nat_eq(a959, a960) {
        let a965 = {
            let a962 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h961) => {
                    h961
                }
            };
            let a963 = v1;
            let a964 = 0u64;
            nat_rem(a962, a963, a964)
        };
        let a966 = 0u64;
        nat_eq(a965, a966)
    } else {
        false
    } {
        f102(match v0.clone() {
            Adt2::C0(_, _, h967, _, _, _, _, _, _, _, _) => {
                h967
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                h968
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h969, _, _, _, _, _, _) => {
                h969
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h970, _, _, _, _) => {
                h970
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h971, _, _, _) => {
                h971
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h972, _, _) => {
                h972
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a975 = match v0.clone() {
            Adt2::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                h973
            }
        };
        let a976 = match v1.clone() {
            Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        nat_eq(a975, a976)
    } else {
        false
    } {
        let a979 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h977) => {
                h977
            }
        };
        let a980 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h978) => {
                h978
            }
        };
        nat_eq(a979, a980)
    } else {
        false
    } {
        let a983 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h981, _, _, _, _, _) => {
                h981
            }
        };
        let a984 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h982, _, _, _, _, _) => {
                h982
            }
        };
        nat_eq(a983, a984)
    } else {
        false
    } {
        let a987 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h985, _, _, _, _) => {
                h985
            }
        };
        let a988 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h986, _, _, _, _) => {
                h986
            }
        };
        nat_eq(a987, a988)
    } else {
        false
    } {
        let a991 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h989, _, _, _) => {
                h989
            }
        };
        let a992 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h990, _, _, _) => {
                h990
            }
        };
        nat_eq(a991, a992)
    } else {
        false
    } {
        let a995 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h993, _, _) => {
                h993
            }
        };
        let a996 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h994, _, _) => {
                h994
            }
        };
        nat_eq(a995, a996)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a997 = v0;
    let a998 = 0u64;
    if if if nat_eq(a997, a998) {
        true
    } else {
        let a999 = v1;
        let a1000 = 0u64;
        nat_eq(a999, a1000)
    } {
        true
    } else {
        let a1001 = v2;
        let a1002 = 0u64;
        nat_eq(a1001, a1002)
    } {
        true
    } else {
        let a1003 = v3;
        let a1004 = 0u64;
        nat_eq(a1003, a1004)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f103({
        let a1006 = match v0.clone() {
            Adt2::C0(_, h1005, _, _, _, _, _, _, _, _, _) => {
                h1005
            }
        };
        let a1007 = v1;
        nat_mul(a1006, a1007)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1008, _, _, _, _, _, _, _, _) => {
            h1008
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1009, _, _, _, _, _, _, _) => {
            h1009
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1010, _, _, _, _, _, _) => {
            h1010
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1011, _, _, _, _) => {
            h1011
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1012, _, _, _) => {
            h1012
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1013, _, _) => {
            h1013
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1014 = v1;
    let a1015 = 0u64;
    if if nat_eq(a1014, a1015) {
        true
    } else {
        let a1016 = v2;
        let a1017 = 0u64;
        nat_eq(a1016, a1017)
    } {
        Ok(0u64)
    } else {
        let a1024 = {
            let a1018 = f104(v1, v2, v3)?;
            let a1019 = f105(v0, v1, v2)?;
            nat_add(a1018, a1019)?
        };
        let a1025 = {
            let a1022 = f106(v0);
            let a1023 = {
                let a1020 = v1;
                let a1021 = v2;
                nat_mul(a1020, a1021)?
            };
            nat_mul(a1022, a1023)?
        };
        nat_add(a1024, a1025)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1026 = v0;
    let a1027 = 0u64;
    if if nat_eq(a1026, a1027) {
        true
    } else {
        let a1028 = v1;
        let a1029 = 0u64;
        nat_eq(a1028, a1029)
    } {
        Ok(3u64)
    } else {
        let a1032 = {
            let a1030 = 3u64;
            let a1031 = v1;
            nat_add(a1030, a1031)?
        };
        let a1033 = v2;
        nat_add(a1032, a1033)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1034 = v0;
    let a1035 = 2139095040u64;
    if nat_le(a1034, a1035) {
        true
    } else {
        let a1036 = v0;
        let a1037 = 2147483648u64;
        nat_eq(a1036, a1037)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1038 = v0;
    let a1039 = 0u64;
    if nat_eq(a1038, a1039) {
        Ok(0u64)
    } else {
        let a1040 = v1;
        let a1041 = f107(v0)?;
        nat_mul(a1040, a1041)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1042 = v0;
    let a1043 = 0u64;
    if if if if nat_eq(a1042, a1043) {
        true
    } else {
        let a1044 = v1;
        let a1045 = 0u64;
        nat_eq(a1044, a1045)
    } {
        true
    } else {
        let a1046 = v2;
        let a1047 = 0u64;
        nat_eq(a1046, a1047)
    } {
        true
    } else {
        let a1048 = v3;
        let a1049 = 0u64;
        nat_eq(a1048, a1049)
    } {
        Ok(0u64)
    } else {
        let a1062 = {
            let a1052 = 2u64;
            let a1053 = {
                let a1050 = v1;
                let a1051 = v2;
                nat_mul(a1050, a1051)?
            };
            nat_mul(a1052, a1053)?
        };
        let a1063 = {
            let a1060 = {
                let a1056 = {
                    let a1054 = v1;
                    let a1055 = v2;
                    nat_mul(a1054, a1055)?
                };
                let a1057 = v3;
                nat_mul(a1056, a1057)?
            };
            let a1061 = {
                let a1058 = 1u64;
                let a1059 = v0;
                nat_add(a1058, a1059)?
            };
            nat_mul(a1060, a1061)?
        };
        nat_add(a1062, a1063)
    }
}

pub fn f41(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f25(v0.clone(), v1.clone()) {
        f25(v0.clone(), v2.clone())
    } else {
        false
    } {
        f108(v0.clone())?
    } else {
        false
    } {
        f108(v1.clone())?
    } else {
        false
    } {
        f108(v2.clone())?
    } else {
        false
    } {
        if f23(v0.clone(), v2.clone())? {
            true
        } else {
            f33(v0.clone(), v2.clone())
        }
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
        Ok(Adt10::C4({
            let a1064 = 6u64;
            let a1065 = f16(v0.clone(), 1u64)?;
            nat_mul(a1064, a1065)?
        }, 2u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C33(f20(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f42(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1066 = v2;
    match m1066 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h1067, _, _, _, _, _, _, _, _, _) => {
                    h1067
                }
            };
            let v6: u64 = {
                let a1073 = match v1.clone() {
                    Adt2::C0(_, _, h1068, _, _, _, _, _, _, _, _) => {
                        h1068
                    }
                };
                let a1074 = {
                    let a1071 = match v1.clone() {
                        Adt2::C0(_, _, _, h1069, _, _, _, _, _, _, _) => {
                            h1069
                        }
                    };
                    let a1072 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1070, _, _, _, _, _, _) => {
                            h1070
                        }
                    };
                    nat_mul(a1071, a1072)?
                };
                nat_mul(a1073, a1074)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f109(v1.clone(), v6)?
            } else {
                false
            } {
                f109(v4.clone(), v6)?
            } else {
                false
            } {
                f109(v3.clone(), v6)?
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
                let a1075 = v5;
                let a1076 = 2147483647u64;
                nat_le(a1075, a1076)
            } else {
                false
            } {
                let a1077 = v6;
                let a1078 = 2147483647u64;
                nat_le(a1077, a1078)
            } else {
                false
            } {
                let a1080 = match v0.clone() {
                    Adt1::C0(h1079, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1079
                    }
                };
                let a1081 = 4294967295u64;
                if nat_le(a1080, a1081) {
                    let a1083 = match v0.clone() {
                        Adt1::C0(_, h1082, _, _, _, _, _, _, _, _, _, _) => {
                            h1082
                        }
                    };
                    let a1084 = 4294967295u64;
                    nat_le(a1083, a1084)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1085 = v5;
                    let a1086 = 0u64;
                    if nat_eq(a1085, a1086) {
                        0u64
                    } else {
                        let a1087 = v6;
                        let a1088 = f110(v5)?;
                        nat_mul(a1087, a1088)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h1089, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1089
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f43(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h1090, _, _, _, _, _, _, _, _, _, _, _) => {
            h1090
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1091, _, _, _, _, _, _, _, _, _, _) => {
            h1091
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1092, _, _, _, _, _, _, _, _, _) => {
            h1092
        }
    };
    let v6: u64 = {
        let a1094 = match v1.clone() {
            Adt2::C0(_, h1093, _, _, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1095 = v4;
        let a1096 = 0u64;
        nat_quot(a1094, a1095, a1096)
    };
    let v7: u64 = {
        let a1098 = match v1.clone() {
            Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1099 = 0u64;
        if nat_eq(a1098, a1099) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f111(match v1.clone() {
        Adt2::C0(h1100, _, _, _, _, _, _, _, _, _, _) => {
            h1100
        }
    }) {
        let a1103 = match v1.clone() {
            Adt2::C0(h1101, _, _, _, _, _, _, _, _, _, _) => {
                h1101
            }
        };
        let a1104 = match v2.clone() {
            Adt2::C0(h1102, _, _, _, _, _, _, _, _, _, _) => {
                h1102
            }
        };
        nat_eq(a1103, a1104)
    } else {
        false
    } {
        let a1105 = 0u64;
        let a1106 = v3;
        nat_lt(a1105, a1106)
    } else {
        false
    } {
        let a1107 = v3;
        let a1108 = 2147483648u64;
        nat_lt(a1107, a1108)
    } else {
        false
    } {
        let a1109 = 0u64;
        let a1110 = v4;
        nat_lt(a1109, a1110)
    } else {
        false
    } {
        let a1111 = v4;
        let a1112 = 2147483648u64;
        nat_lt(a1111, a1112)
    } else {
        false
    } {
        let a1113 = v5;
        let a1114 = 2147483648u64;
        nat_lt(a1113, a1114)
    } else {
        false
    } {
        let a1115 = 0u64;
        let a1116 = v6;
        nat_lt(a1115, a1116)
    } else {
        false
    } {
        let a1118 = 0u64;
        let a1119 = match v2.clone() {
            Adt2::C0(_, h1117, _, _, _, _, _, _, _, _, _) => {
                h1117
            }
        };
        nat_lt(a1118, a1119)
    } else {
        false
    } {
        let a1123 = match v1.clone() {
            Adt2::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1124 = {
            let a1121 = v6;
            let a1122 = v4;
            nat_mul(a1121, a1122)?
        };
        nat_eq(a1123, a1124)
    } else {
        false
    } {
        let a1126 = match v2.clone() {
            Adt2::C0(_, _, h1125, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        let a1127 = v4;
        nat_eq(a1126, a1127)
    } else {
        false
    } {
        let a1140 = {
            let a1133 = {
                let a1131 = match v2.clone() {
                    Adt2::C0(_, h1128, _, _, _, _, _, _, _, _, _) => {
                        h1128
                    }
                };
                let a1132 = {
                    let a1129 = v5;
                    let a1130 = 2u64;
                    nat_mul(a1129, a1130)?
                };
                nat_add(a1131, a1132)?
            };
            let a1134 = v3;
            nat_add(a1133, a1134)?
        };
        let a1141 = {
            let a1138 = {
                let a1136 = match v1.clone() {
                    Adt2::C0(_, _, h1135, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                };
                let a1137 = v3;
                nat_mul(a1136, a1137)?
            };
            let a1139 = v6;
            nat_add(a1138, a1139)?
        };
        nat_eq(a1140, a1141)
    } else {
        false
    } {
        f112(v1.clone(), v7)?
    } else {
        false
    } {
        f112(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1142 = f16(v2.clone(), 1u64)?;
            let a1143 = f113(v8.clone())?;
            nat_mul(a1142, a1143)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
                h1144
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f44(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1145 = v1;
    match m1145 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1147 = match v0.clone() {
                Adt2::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                    h1146
                }
            };
            let a1148 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1147, a1148) {
                let a1150 = match v3.clone() {
                    Adt2::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                        h1149
                    }
                };
                let a1151 = 0u64;
                nat_eq(a1150, a1151)
            } else {
                false
            } {
                let a1153 = match v2.clone() {
                    Adt2::C0(h1152, _, _, _, _, _, _, _, _, _, _) => {
                        h1152
                    }
                };
                let a1154 = 0u64;
                nat_eq(a1153, a1154)
            } else {
                false
            } {
                let a1156 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1155, _, _, _, _, _, _) => {
                        h1155
                    }
                };
                let a1157 = 1u64;
                nat_eq(a1156, a1157)
            } else {
                false
            } {
                let a1159 = match v3.clone() {
                    Adt2::C0(_, _, _, h1158, _, _, _, _, _, _, _) => {
                        h1158
                    }
                };
                let a1160 = 1u64;
                nat_eq(a1159, a1160)
            } else {
                false
            } {
                let a1162 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1161, _, _, _, _, _, _) => {
                        h1161
                    }
                };
                let a1163 = 1u64;
                nat_eq(a1162, a1163)
            } else {
                false
            } {
                let a1166 = match v0.clone() {
                    Adt2::C0(_, _, h1164, _, _, _, _, _, _, _, _) => {
                        h1164
                    }
                };
                let a1167 = match v3.clone() {
                    Adt2::C0(_, _, h1165, _, _, _, _, _, _, _, _) => {
                        h1165
                    }
                };
                nat_eq(a1166, a1167)
            } else {
                false
            } {
                let a1172 = match v3.clone() {
                    Adt2::C0(_, h1168, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                };
                let a1173 = {
                    let a1170 = match v0.clone() {
                        Adt2::C0(_, h1169, _, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1171 = 1u64;
                    nat_add(a1170, a1171)?
                };
                nat_le(a1172, a1173)
            } else {
                false
            } {
                let a1176 = match v2.clone() {
                    Adt2::C0(_, h1174, _, _, _, _, _, _, _, _, _) => {
                        h1174
                    }
                };
                let a1177 = match v0.clone() {
                    Adt2::C0(_, _, h1175, _, _, _, _, _, _, _, _) => {
                        h1175
                    }
                };
                nat_eq(a1176, a1177)
            } else {
                false
            } {
                let a1185 = match v2.clone() {
                    Adt2::C0(_, _, h1178, _, _, _, _, _, _, _, _) => {
                        h1178
                    }
                };
                let a1186 = {
                    let a1183 = {
                        let a1180 = match v0.clone() {
                            Adt2::C0(_, h1179, _, _, _, _, _, _, _, _, _) => {
                                h1179
                            }
                        };
                        let a1181 = 1u64;
                        nat_add(a1180, a1181)?
                    };
                    let a1184 = match v3.clone() {
                        Adt2::C0(_, h1182, _, _, _, _, _, _, _, _, _) => {
                            h1182
                        }
                    };
                    nat_sub(a1183, a1184)
                };
                nat_eq(a1185, a1186)
            } else {
                false
            } {
                let a1189 = match v2.clone() {
                    Adt2::C0(_, _, _, h1187, _, _, _, _, _, _, _) => {
                        h1187
                    }
                };
                let a1190 = match v0.clone() {
                    Adt2::C0(_, _, _, h1188, _, _, _, _, _, _, _) => {
                        h1188
                    }
                };
                nat_eq(a1189, a1190)
            } else {
                false
            } {
                let a1192 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1191, _, _, _, _, _, _) => {
                        h1191
                    }
                };
                let a1193 = 1u64;
                nat_eq(a1192, a1193)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1195 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1194, _, _, _, _, _) => {
                        h1194
                    }
                };
                let a1196 = 4u64;
                nat_eq(a1195, a1196)
            } else {
                false
            } {
                let a1198 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1197, _, _, _, _, _) => {
                        h1197
                    }
                };
                let a1199 = 4u64;
                nat_eq(a1198, a1199)
            } else {
                false
            } {
                let a1204 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1200, _, _, _, _) => {
                        h1200
                    }
                };
                let a1205 = {
                    let a1202 = match v0.clone() {
                        Adt2::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
                            h1201
                        }
                    };
                    let a1203 = 4u64;
                    nat_mul(a1202, a1203)?
                };
                nat_eq(a1204, a1205)
            } else {
                false
            } {
                let a1210 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1206, _, _, _, _) => {
                        h1206
                    }
                };
                let a1211 = {
                    let a1208 = match v3.clone() {
                        Adt2::C0(_, h1207, _, _, _, _, _, _, _, _, _) => {
                            h1207
                        }
                    };
                    let a1209 = 4u64;
                    nat_mul(a1208, a1209)?
                };
                nat_eq(a1210, a1211)
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
                    let a1213 = f16(v2.clone(), 1u64)?;
                    let a1214 = f114(match v3.clone() {
                        Adt2::C0(_, h1212, _, _, _, _, _, _, _, _, _) => {
                            h1212
                        }
                    })?;
                    nat_mul(a1213, a1214)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f45(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1215, _, _, _, _, _, _, _, _, _, _) => {
            h1215
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1216, _, _, _, _, _, _, _, _) => {
                h1216
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1217, _, _, _, _, _, _, _, _, _) => {
                h1217
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1218, _, _, _, _, _, _) => {
                h1218
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1219, _, _, _, _, _, _, _, _) => {
                h1219
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1220, _, _, _, _, _, _, _, _, _) => {
                h1220
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1221, _, _, _, _, _, _, _) => {
                h1221
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1222, _, _, _, _, _) => {
                h1222
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1224 = match v1.clone() {
        Adt1::C0(h1223, _, _, _, _, _, _, _, _, _, _, _) => {
            h1223
        }
    };
    let a1225 = 1u64;
    if if if if if if if if if if if nat_le(a1224, a1225) {
        let a1227 = match v2.clone() {
            Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1228 = 1u64;
        nat_le(a1227, a1228)
    } else {
        false
    } {
        let a1230 = match v3.clone() {
            Adt2::C0(h1229, _, _, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1231 = 0u64;
        nat_eq(a1230, a1231)
    } else {
        false
    } {
        let a1236 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1232, _, _, _, _, _) => {
                h1232
            }
        };
        let a1237 = {
            let a1234 = match v2.clone() {
                Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                    h1233
                }
            };
            let a1235 = 0u64;
            if nat_eq(a1234, a1235) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1236, a1237)
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
        f115(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1238, _, _, _, _, _, _, _, _, _) => {
                h1238
            }
        }, match v3.clone() {
            Adt2::C0(_, h1239, _, _, _, _, _, _, _, _, _) => {
                h1239
            }
        })?
    } else {
        false
    } {
        if v0 {
            f115(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1240, _, _, _, _, _, _, _, _) => {
                    h1240
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1241, _, _, _, _, _, _, _, _) => {
                    h1241
                }
            })?
        } else {
            let a1244 = match v2.clone() {
                Adt2::C0(_, _, h1242, _, _, _, _, _, _, _, _) => {
                    h1242
                }
            };
            let a1245 = match v3.clone() {
                Adt2::C0(_, _, h1243, _, _, _, _, _, _, _, _) => {
                    h1243
                }
            };
            nat_eq(a1244, a1245)
        }
    } else {
        false
    } {
        let a1248 = match v2.clone() {
            Adt2::C0(_, _, _, h1246, _, _, _, _, _, _, _) => {
                h1246
            }
        };
        let a1249 = match v3.clone() {
            Adt2::C0(_, _, _, h1247, _, _, _, _, _, _, _) => {
                h1247
            }
        };
        nat_eq(a1248, a1249)
    } else {
        false
    } {
        let a1252 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1250, _, _, _, _, _, _) => {
                h1250
            }
        };
        let a1253 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1251, _, _, _, _, _, _) => {
                h1251
            }
        };
        nat_eq(a1252, a1253)
    } else {
        false
    } {
        let a1258 = {
            let a1256 = match v4.clone() {
                Adt6::C0(h1254, _, _, _) => {
                    h1254
                }
            };
            let a1257 = match v5.clone() {
                Adt6::C0(h1255, _, _, _) => {
                    h1255
                }
            };
            nat_mul(a1256, a1257)?
        };
        let a1259 = 2147483648u64;
        nat_lt(a1258, a1259)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1261 = match v1 {
                Adt1::C0(h1260, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1260
                }
            };
            let a1262 = 0u64;
            nat_eq(a1261, a1262)
        }, match v2.clone() {
            Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
                h1263
            }
        });
        Ok(Adt10::C4({
            let a1264 = f16(v3.clone(), 1u64)?;
            let a1265 = f116(v6.clone())?;
            nat_mul(a1264, a1265)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f46(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1266 = v2;
    match m1266 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1268 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1267, _, _, _, _, _) => {
                        h1267
                    }
                };
                let a1269 = 1u64;
                nat_eq(a1268, a1269)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1270, _, _, _, _, _, _, _, _, _) => {
                    h1270
                }
            }, match v0.clone() {
                Adt1::C0(h1271, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1271
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1272, _, _, _, _, _, _, _, _, _) => {
                    h1272
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1273, _, _, _, _, _, _, _) => {
                    h1273
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1274, _, _, _, _, _, _, _, _) => {
                        h1274
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1275, _, _, _, _, _, _, _, _, _, _) => {
                        h1275
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1276, _, _, _, _, _, _, _, _) => {
                        h1276
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1277, _, _, _, _, _, _) => {
                        h1277
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1278, _, _, _, _, _, _, _) => {
                        h1278
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1279, _, _, _, _, _, _, _, _) => {
                        h1279
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1280, _, _, _, _, _, _) => {
                        h1280
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1281, _, _, _, _, _, _, _) => {
                        h1281
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1282, _, _, _, _, _, _, _, _) => {
                        h1282
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1283, _, _, _, _, _, _, _, _, _, _) => {
                    h1283
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1284, _, _, _, _, _, _, _, _, _, _) => {
                    h1284
                }
            };
            let v14: u64 = {
                let a1285 = v12;
                let a1286 = 0u64;
                if nat_eq(a1285, a1286) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1288 = match v4.clone() {
                    Adt2::C0(_, h1287, _, _, _, _, _, _, _, _, _) => {
                        h1287
                    }
                };
                let a1289 = v14;
                nat_mul(a1288, a1289)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1290, _, _, _) => {
                        h1290
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1291, _, _, _, _) => {
                        h1291
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1292, _, _) => {
                        h1292
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1293, _, _, _) => {
                        h1293
                    }
                }
            };
            let a1295 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1294, _, _, _, _, _) => {
                    h1294
                }
            };
            let a1296 = 1u64;
            if if if if if if if if if if nat_le(a1295, a1296) {
                let a1297 = v12;
                let a1298 = 0u64;
                if nat_eq(a1297, a1298) {
                    true
                } else {
                    let a1299 = v12;
                    let a1300 = 1u64;
                    if nat_eq(a1299, a1300) {
                        let a1301 = v13;
                        let a1302 = 1u64;
                        nat_eq(a1301, a1302)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1303 = v13;
                let a1304 = 0u64;
                if nat_eq(a1303, a1304) {
                    true
                } else {
                    let a1305 = v13;
                    let a1306 = 1u64;
                    nat_eq(a1305, a1306)
                }
            } else {
                false
            } {
                let a1308 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1307, _, _, _, _, _) => {
                        h1307
                    }
                };
                let a1309 = v14;
                nat_eq(a1308, a1309)
            } else {
                false
            } {
                let a1317 = match v3.clone() {
                    Adt2::C0(_, h1310, _, _, _, _, _, _, _, _, _) => {
                        h1310
                    }
                };
                let a1318 = {
                    let a1315 = v9;
                    let a1316 = {
                        let a1313 = match v6.clone() {
                            Adt6::C0(h1311, _, _, _) => {
                                h1311
                            }
                        };
                        let a1314 = match v7.clone() {
                            Adt6::C0(h1312, _, _, _) => {
                                h1312
                            }
                        };
                        nat_mul(a1313, a1314)?
                    };
                    nat_mul(a1315, a1316)?
                };
                nat_eq(a1317, a1318)
            } else {
                false
            } {
                let a1321 = match v3.clone() {
                    Adt2::C0(_, _, h1319, _, _, _, _, _, _, _, _) => {
                        h1319
                    }
                };
                let a1322 = f96(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1320, _, _, _, _, _, _, _, _, _) => {
                        h1320
                    }
                })?;
                nat_eq(a1321, a1322)
            } else {
                false
            } {
                let a1324 = 0u64;
                let a1325 = match v3.clone() {
                    Adt2::C0(_, _, h1323, _, _, _, _, _, _, _, _) => {
                        h1323
                    }
                };
                nat_lt(a1324, a1325)
            } else {
                false
            } {
                if v5 {
                    let a1327 = match v1 {
                        Adt2::C0(_, _, _, h1326, _, _, _, _, _, _, _) => {
                            h1326
                        }
                    };
                    let a1328 = v9;
                    if if if nat_eq(a1327, a1328) {
                        let a1330 = match v3.clone() {
                            Adt2::C0(_, _, _, h1329, _, _, _, _, _, _, _) => {
                                h1329
                            }
                        };
                        let a1331 = f96(v7.clone(), v11)?;
                        nat_eq(a1330, a1331)
                    } else {
                        false
                    } {
                        let a1333 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1332, _, _, _, _, _, _) => {
                                h1332
                            }
                        };
                        let a1334 = v10;
                        nat_eq(a1333, a1334)
                    } else {
                        false
                    } {
                        let a1336 = 0u64;
                        let a1337 = match v3.clone() {
                            Adt2::C0(_, _, _, h1335, _, _, _, _, _, _, _) => {
                                h1335
                            }
                        };
                        nat_lt(a1336, a1337)
                    } else {
                        false
                    }
                } else {
                    let a1339 = match v1 {
                        Adt2::C0(_, _, h1338, _, _, _, _, _, _, _, _) => {
                            h1338
                        }
                    };
                    let a1340 = v9;
                    if if if nat_eq(a1339, a1340) {
                        let a1342 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1341, _, _, _, _, _, _) => {
                                h1341
                            }
                        };
                        let a1343 = 1u64;
                        nat_eq(a1342, a1343)
                    } else {
                        false
                    } {
                        let a1345 = match v3.clone() {
                            Adt2::C0(_, _, _, h1344, _, _, _, _, _, _, _) => {
                                h1344
                            }
                        };
                        let a1346 = v10;
                        nat_eq(a1345, a1346)
                    } else {
                        false
                    } {
                        let a1348 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1347, _, _, _, _, _, _) => {
                                h1347
                            }
                        };
                        let a1349 = 1u64;
                        nat_eq(a1348, a1349)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1350, _, _, _, _, _, _, _, _, _) => {
                        h1350
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1370 = {
                        let a1367 = {
                            let a1365 = {
                                let a1359 = {
                                    let a1353 = {
                                        let a1351 = v10;
                                        let a1352 = 1u64;
                                        nat_sub(a1351, a1352)
                                    };
                                    let a1354 = v17;
                                    nat_mul(a1353, a1354)?
                                };
                                let a1360 = {
                                    let a1357 = {
                                        let a1355 = v9;
                                        let a1356 = 1u64;
                                        nat_sub(a1355, a1356)
                                    };
                                    let a1358 = v16;
                                    nat_mul(a1357, a1358)?
                                };
                                nat_add(a1359, a1360)?
                            };
                            let a1366 = {
                                let a1363 = {
                                    let a1361 = v11;
                                    let a1362 = 1u64;
                                    nat_sub(a1361, a1362)
                                };
                                let a1364 = v15;
                                nat_mul(a1363, a1364)?
                            };
                            nat_add(a1365, a1366)?
                        };
                        let a1368 = v15;
                        nat_add(a1367, a1368)?
                    };
                    let a1371 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1369, _) => {
                            h1369
                        }
                    };
                    nat_le(a1370, a1371)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1372 = v13;
                    let a1373 = 0u64;
                    if nat_eq(a1372, a1373) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1374, _, _, _, _, _, _, _, _, _) => {
                        h1374
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1375, _, _, _, _, _, _, _, _, _) => {
                            h1375
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1376, _, _, _, _, _, _, _, _) => {
                            h1376
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1377, _, _, _, _, _, _, _) => {
                            h1377
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1378, _, _, _, _, _) => {
                            h1378
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1379, _, _, _, _) => {
                            h1379
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1380, _, _, _) => {
                            h1380
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f47(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1381 = v2;
    match m1381 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1382, _, _, _, _, _, _, _, _, _) => {
                    h1382
                }
            }, match v0.clone() {
                Adt1::C0(h1383, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1383
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1384, _, _, _, _, _, _, _, _) => {
                    h1384
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1385, _, _, _, _, _) => {
                    h1385
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1386, _, _, _, _, _, _, _, _) => {
                    h1386
                }
            }, match v0.clone() {
                Adt1::C0(_, h1387, _, _, _, _, _, _, _, _, _, _) => {
                    h1387
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1388, _, _, _, _, _, _, _) => {
                    h1388
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1389, _, _, _, _) => {
                    h1389
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1390, _, _, _, _, _, _, _) => {
                    h1390
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1391, _, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1392, _, _, _, _, _, _) => {
                    h1392
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1393, _, _, _) => {
                    h1393
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1394, _, _) => {
                    h1394
                }
            };
            let v9: u64 = f96(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1395, _, _, _, _, _, _, _) => {
                    h1395
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1396, _, _, _, _, _, _, _, _, _, _) => {
                    h1396
                }
            };
            let v11: u64 = {
                let a1404 = v8;
                let a1405 = {
                    let a1402 = match v5.clone() {
                        Adt6::C0(h1397, _, _, _) => {
                            h1397
                        }
                    };
                    let a1403 = {
                        let a1400 = match v6.clone() {
                            Adt6::C0(h1398, _, _, _) => {
                                h1398
                            }
                        };
                        let a1401 = match v7.clone() {
                            Adt6::C0(h1399, _, _, _) => {
                                h1399
                            }
                        };
                        nat_mul(a1400, a1401)?
                    };
                    nat_mul(a1402, a1403)?
                };
                nat_mul(a1404, a1405)?
            };
            let a1407 = match v4.clone() {
                Adt2::C0(h1406, _, _, _, _, _, _, _, _, _, _) => {
                    h1406
                }
            };
            let a1408 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1407, a1408) {
                let a1409 = v10;
                let a1410 = 0u64;
                if nat_eq(a1409, a1410) {
                    true
                } else {
                    let a1411 = v10;
                    let a1412 = 1u64;
                    nat_eq(a1411, a1412)
                }
            } else {
                false
            } {
                let a1413 = 0u64;
                let a1414 = v8;
                nat_lt(a1413, a1414)
            } else {
                false
            } {
                let a1415 = v8;
                let a1416 = 2147483648u64;
                nat_lt(a1415, a1416)
            } else {
                false
            } {
                let a1418 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1417, _, _, _, _, _) => {
                        h1417
                    }
                };
                let a1419 = 4u64;
                nat_eq(a1418, a1419)
            } else {
                false
            } {
                let a1420 = 0u64;
                let a1421 = v9;
                nat_lt(a1420, a1421)
            } else {
                false
            } {
                let a1423 = 0u64;
                let a1424 = match v3.clone() {
                    Adt2::C0(_, _, h1422, _, _, _, _, _, _, _, _) => {
                        h1422
                    }
                };
                nat_lt(a1423, a1424)
            } else {
                false
            } {
                let a1426 = 0u64;
                let a1427 = match v3.clone() {
                    Adt2::C0(_, _, _, h1425, _, _, _, _, _, _, _) => {
                        h1425
                    }
                };
                nat_lt(a1426, a1427)
            } else {
                false
            } {
                let a1429 = match v3.clone() {
                    Adt2::C0(_, h1428, _, _, _, _, _, _, _, _, _) => {
                        h1428
                    }
                };
                let a1430 = v11;
                nat_eq(a1429, a1430)
            } else {
                false
            } {
                let a1433 = match v3.clone() {
                    Adt2::C0(_, _, h1431, _, _, _, _, _, _, _, _) => {
                        h1431
                    }
                };
                let a1434 = f96(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1432, _, _, _, _, _, _, _, _, _) => {
                        h1432
                    }
                })?;
                nat_eq(a1433, a1434)
            } else {
                false
            } {
                let a1437 = match v3.clone() {
                    Adt2::C0(_, _, _, h1435, _, _, _, _, _, _, _) => {
                        h1435
                    }
                };
                let a1438 = f96(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1436, _, _, _, _, _, _, _, _) => {
                        h1436
                    }
                })?;
                nat_eq(a1437, a1438)
            } else {
                false
            } {
                let a1446 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1439, _, _, _, _, _, _) => {
                        h1439
                    }
                };
                let a1447 = {
                    let a1444 = v9;
                    let a1445 = {
                        let a1441 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1440, _, _, _, _, _, _) => {
                                h1440
                            }
                        };
                        let a1442 = v8;
                        let a1443 = 0u64;
                        nat_quot(a1441, a1442, a1443)
                    };
                    nat_mul(a1444, a1445)?
                };
                nat_eq(a1446, a1447)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1448 = v10;
                    let a1449 = 0u64;
                    if nat_eq(a1448, a1449) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1450, _, _) => {
                        h1450
                    }
                }, {
                    let a1452 = v8;
                    let a1453 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1451, _, _) => {
                            h1451
                        }
                    };
                    nat_mul(a1452, a1453)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f48(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1454 = v1;
    match m1454 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f117(v0, v3, v2)
        }
    }
}

pub fn f49(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1456 = match v0.clone() {
        Adt2::C0(h1455, _, _, _, _, _, _, _, _, _, _) => {
            h1455
        }
    };
    let a1457 = 1u64;
    if if nat_eq(a1456, a1457) {
        true
    } else {
        let a1459 = match v0.clone() {
            Adt2::C0(h1458, _, _, _, _, _, _, _, _, _, _) => {
                h1458
            }
        };
        let a1460 = 30u64;
        nat_eq(a1459, a1460)
    } {
        let a1462 = match v1.clone() {
            Adt2::C0(h1461, _, _, _, _, _, _, _, _, _, _) => {
                h1461
            }
        };
        let a1463 = 1u64;
        if if if if if if if if if if if nat_eq(a1462, a1463) {
            let a1466 = match v0.clone() {
                Adt2::C0(_, h1464, _, _, _, _, _, _, _, _, _) => {
                    h1464
                }
            };
            let a1467 = match v1.clone() {
                Adt2::C0(_, h1465, _, _, _, _, _, _, _, _, _) => {
                    h1465
                }
            };
            nat_eq(a1466, a1467)
        } else {
            false
        } {
            let a1469 = 0u64;
            let a1470 = match v1.clone() {
                Adt2::C0(_, _, h1468, _, _, _, _, _, _, _, _) => {
                    h1468
                }
            };
            nat_lt(a1469, a1470)
        } else {
            false
        } {
            let a1473 = match v1.clone() {
                Adt2::C0(_, _, h1471, _, _, _, _, _, _, _, _) => {
                    h1471
                }
            };
            let a1474 = match v1.clone() {
                Adt2::C0(_, _, _, h1472, _, _, _, _, _, _, _) => {
                    h1472
                }
            };
            nat_eq(a1473, a1474)
        } else {
            false
        } {
            let a1476 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1475, _, _, _, _, _, _) => {
                    h1475
                }
            };
            let a1477 = 1u64;
            nat_eq(a1476, a1477)
        } else {
            false
        } {
            let a1485 = match v0.clone() {
                Adt2::C0(_, _, h1478, _, _, _, _, _, _, _, _) => {
                    h1478
                }
            };
            let a1486 = {
                let a1483 = {
                    let a1481 = match v1.clone() {
                        Adt2::C0(_, _, h1479, _, _, _, _, _, _, _, _) => {
                            h1479
                        }
                    };
                    let a1482 = match v1.clone() {
                        Adt2::C0(_, _, h1480, _, _, _, _, _, _, _, _) => {
                            h1480
                        }
                    };
                    nat_add(a1481, a1482)?
                };
                let a1484 = 1u64;
                nat_sub(a1483, a1484)
            };
            nat_eq(a1485, a1486)
        } else {
            false
        } {
            let a1488 = 0u64;
            let a1489 = match v0.clone() {
                Adt2::C0(_, _, _, h1487, _, _, _, _, _, _, _) => {
                    h1487
                }
            };
            nat_lt(a1488, a1489)
        } else {
            false
        } {
            let a1491 = 0u64;
            let a1492 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1490, _, _, _, _, _, _) => {
                    h1490
                }
            };
            nat_lt(a1491, a1492)
        } else {
            false
        } {
            let a1494 = 2u64;
            let a1495 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1493, _, _, _, _, _) => {
                    h1493
                }
            };
            nat_le(a1494, a1495)
        } else {
            false
        } {
            let a1497 = 2u64;
            let a1498 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1496, _, _, _, _, _) => {
                    h1496
                }
            };
            nat_le(a1497, a1498)
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

pub fn f50(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1499 = v1;
    match m1499 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f118(v0, v3, v2)
        }
    }
}

pub fn f51(v0: u64) -> bool {
    let a1500 = v0;
    let a1501 = 1u64;
    if if nat_eq(a1500, a1501) {
        true
    } else {
        let a1502 = v0;
        let a1503 = 33u64;
        nat_eq(a1502, a1503)
    } {
        true
    } else {
        let a1504 = v0;
        let a1505 = 34u64;
        nat_eq(a1504, a1505)
    }
}

pub fn f52(v0: u64) -> bool {
    if f119(v0) {
        !f120(v0)
    } else {
        false
    }
}

pub fn f53(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1506 = v1;
    match m1506 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1508 = f121(match v0.clone() {
                Adt2::C0(h1507, _, _, _, _, _, _, _, _, _, _) => {
                    h1507
                }
            });
            match m1508 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1509, _) => {
                            h1509
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1510 = f16(v0.clone(), 1u64)?;
                        let a1511 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1510, a1511)
                    } else {
                        false
                    } {
                        let a1516 = {
                            let a1513 = match v0.clone() {
                                Adt2::C0(_, h1512, _, _, _, _, _, _, _, _, _) => {
                                    h1512
                                }
                            };
                            let a1514 = v5;
                            let a1515 = 1u64;
                            nat_rem(a1513, a1514, a1515)
                        };
                        let a1517 = 0u64;
                        if nat_eq(a1516, a1517) {
                            let a1520 = match v4 {
                                Adt18::C0(_, _, h1518) => {
                                    h1518
                                }
                            };
                            let a1521 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1519, _, _, _, _, _) => {
                                    h1519
                                }
                            };
                            nat_le(a1520, a1521)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1522 = v5;
                        let a1523 = 1u64;
                        if nat_eq(a1522, a1523) {
                            let a1525 = 4u64;
                            let a1526 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1524, _, _, _, _, _) => {
                                    h1524
                                }
                            };
                            nat_le(a1525, a1526)
                        } else {
                            let a1528 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1527, _, _, _, _, _) => {
                                    h1527
                                }
                            };
                            let a1529 = 4u64;
                            nat_eq(a1528, a1529)
                        }
                    } else {
                        false
                    } {
                        let a1534 = {
                            let a1531 = match v2.clone() {
                                Adt2::C0(_, h1530, _, _, _, _, _, _, _, _, _) => {
                                    h1530
                                }
                            };
                            let a1532 = v5;
                            let a1533 = 1u64;
                            nat_rem(a1531, a1532, a1533)
                        };
                        let a1535 = 0u64;
                        if nat_eq(a1534, a1535) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
                                h1536
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

pub fn f54(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1539 = match v1.clone() {
        Adt2::C0(h1537, _, _, _, _, _, _, _, _, _, _) => {
            h1537
        }
    };
    let a1540 = match v3.clone() {
        Adt2::C0(h1538, _, _, _, _, _, _, _, _, _, _) => {
            h1538
        }
    };
    if nat_eq(a1539, a1540) {
        f122(v0, v1, v2, v3)
    } else {
        if if if if f120(match v1.clone() {
            Adt2::C0(h1541, _, _, _, _, _, _, _, _, _, _) => {
                h1541
            }
        }) {
            f120(match v3.clone() {
                Adt2::C0(h1542, _, _, _, _, _, _, _, _, _, _) => {
                    h1542
                }
            })
        } else {
            false
        } {
            let a1543 = v0;
            let a1544 = 33u64;
            if nat_eq(a1543, a1544) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1545 = f16(v1.clone(), 1u64)?;
            let a1546 = f16(v3.clone(), 1u64)?;
            nat_eq(a1545, a1546)
        } else {
            false
        } {
            let m1548 = f121(match v3.clone() {
                Adt2::C0(h1547, _, _, _, _, _, _, _, _, _, _) => {
                    h1547
                }
            });
            match m1548 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1549, _, _, _, _, _, _, _, _, _, _) => {
                            h1549
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1550, _, _, _, _, _, _, _, _, _, _) => {
                            h1550
                        }
                    };
                    let v7: bool = {
                        let a1551 = v5;
                        let a1552 = 43u64;
                        if nat_eq(a1551, a1552) {
                            true
                        } else {
                            let a1553 = v6;
                            let a1554 = 43u64;
                            nat_eq(a1553, a1554)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1555 = f16(v3.clone(), 1u64)?;
                        let a1556 = if v7 {
                            f123(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1555, a1556)?
                    }, match v4 {
                        Adt18::C0(_, _, h1557) => {
                            h1557
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

pub fn f55(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1559 = match v0.clone() {
        Adt2::C0(h1558, _, _, _, _, _, _, _, _, _, _) => {
            h1558
        }
    };
    let a1560 = 0u64;
    if if if if nat_eq(a1559, a1560) {
        true
    } else {
        let a1562 = match v0.clone() {
            Adt2::C0(h1561, _, _, _, _, _, _, _, _, _, _) => {
                h1561
            }
        };
        let a1563 = 1u64;
        nat_eq(a1562, a1563)
    } {
        true
    } else {
        let a1565 = match v0.clone() {
            Adt2::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
                h1564
            }
        };
        let a1566 = 25u64;
        if nat_eq(a1565, a1566) {
            true
        } else {
            let a1568 = match v0.clone() {
                Adt2::C0(h1567, _, _, _, _, _, _, _, _, _, _) => {
                    h1567
                }
            };
            let a1569 = 26u64;
            nat_eq(a1568, a1569)
        }
    } {
        true
    } else {
        let a1571 = match v0.clone() {
            Adt2::C0(h1570, _, _, _, _, _, _, _, _, _, _) => {
                h1570
            }
        };
        let a1572 = 30u64;
        nat_eq(a1571, a1572)
    } {
        let a1575 = match v0.clone() {
            Adt2::C0(h1573, _, _, _, _, _, _, _, _, _, _) => {
                h1573
            }
        };
        let a1576 = match v1.clone() {
            Adt2::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
                h1574
            }
        };
        if if nat_eq(a1575, a1576) {
            f124(match v1.clone() {
                Adt2::C0(_, h1577, _, _, _, _, _, _, _, _, _) => {
                    h1577
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1578, _, _, _, _, _, _, _, _) => {
                    h1578
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1579, _, _, _, _, _, _, _) => {
                    h1579
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1580, _, _, _, _, _, _) => {
                    h1580
                }
            }, match v0.clone() {
                Adt2::C0(_, h1581, _, _, _, _, _, _, _, _, _) => {
                    h1581
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1582, _, _, _, _, _, _, _, _) => {
                    h1582
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1583, _, _, _, _, _, _, _) => {
                    h1583
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1584, _, _, _, _, _, _) => {
                    h1584
                }
            })
        } else {
            false
        } {
            let m1586 = f121(match v0.clone() {
                Adt2::C0(h1585, _, _, _, _, _, _, _, _, _, _) => {
                    h1585
                }
            });
            match m1586 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1587) => {
                            h1587
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

pub fn f56(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1588 = v2;
    match m1588 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1589 = v0;
            let a1590 = 4u64;
            if if if nat_lt(a1589, a1590) {
                let a1593 = match v1.clone() {
                    Adt2::C0(h1591, _, _, _, _, _, _, _, _, _, _) => {
                        h1591
                    }
                };
                let a1594 = match v4.clone() {
                    Adt2::C0(h1592, _, _, _, _, _, _, _, _, _, _) => {
                        h1592
                    }
                };
                if nat_eq(a1593, a1594) {
                    let a1597 = match v1.clone() {
                        Adt2::C0(h1595, _, _, _, _, _, _, _, _, _, _) => {
                            h1595
                        }
                    };
                    let a1598 = match v3.clone() {
                        Adt2::C0(h1596, _, _, _, _, _, _, _, _, _, _) => {
                            h1596
                        }
                    };
                    nat_eq(a1597, a1598)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f125(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1599, _, _, _, _, _, _, _, _, _) => {
                        h1599
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1600, _, _, _, _, _, _, _, _, _) => {
                        h1600
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1601, _, _, _, _, _, _, _, _, _) => {
                        h1601
                    }
                })? {
                    f125(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1602, _, _, _, _, _, _, _, _) => {
                            h1602
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1603, _, _, _, _, _, _, _, _) => {
                            h1603
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1604, _, _, _, _, _, _, _, _) => {
                            h1604
                        }
                    })?
                } else {
                    false
                } {
                    f125(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1605, _, _, _, _, _, _, _) => {
                            h1605
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1606, _, _, _, _, _, _, _) => {
                            h1606
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1607, _, _, _, _, _, _, _) => {
                            h1607
                        }
                    })?
                } else {
                    false
                } {
                    f125(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1608, _, _, _, _, _, _) => {
                            h1608
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1609, _, _, _, _, _, _) => {
                            h1609
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1610, _, _, _, _, _, _) => {
                            h1610
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1612 = f121(match v1.clone() {
                    Adt2::C0(h1611, _, _, _, _, _, _, _, _, _, _) => {
                        h1611
                    }
                });
                match m1612 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1613, _) => {
                                h1613
                            }
                        };
                        let a1618 = {
                            let a1615 = match v1.clone() {
                                Adt2::C0(_, h1614, _, _, _, _, _, _, _, _, _) => {
                                    h1614
                                }
                            };
                            let a1616 = v6;
                            let a1617 = 1u64;
                            nat_rem(a1615, a1616, a1617)
                        };
                        let a1619 = 0u64;
                        if if if nat_eq(a1618, a1619) {
                            let a1624 = {
                                let a1621 = match v4.clone() {
                                    Adt2::C0(_, h1620, _, _, _, _, _, _, _, _, _) => {
                                        h1620
                                    }
                                };
                                let a1622 = v6;
                                let a1623 = 1u64;
                                nat_rem(a1621, a1622, a1623)
                            };
                            let a1625 = 0u64;
                            nat_eq(a1624, a1625)
                        } else {
                            false
                        } {
                            let a1630 = {
                                let a1627 = match v3.clone() {
                                    Adt2::C0(_, h1626, _, _, _, _, _, _, _, _, _) => {
                                        h1626
                                    }
                                };
                                let a1628 = v6;
                                let a1629 = 1u64;
                                nat_rem(a1627, a1628, a1629)
                            };
                            let a1631 = 0u64;
                            nat_eq(a1630, a1631)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1632) => {
                                    h1632
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

pub fn f57(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1634 = match v2.clone() {
        Adt2::C0(h1633, _, _, _, _, _, _, _, _, _, _) => {
            h1633
        }
    };
    let a1635 = 0u64;
    if if nat_eq(a1634, a1635) {
        let a1637 = match v3.clone() {
            Adt2::C0(h1636, _, _, _, _, _, _, _, _, _, _) => {
                h1636
            }
        };
        let a1638 = 0u64;
        nat_eq(a1637, a1638)
    } else {
        false
    } {
        let a1639 = v0;
        let a1640 = 61u64;
        if nat_eq(a1639, a1640) {
            let v4: bool = {
                let a1642 = 0u64;
                let a1643 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1641, _, _, _) => {
                        h1641
                    }
                };
                nat_lt(a1642, a1643)
            };
            if if if if if if f126(match v2.clone() {
                Adt2::C0(_, h1644, _, _, _, _, _, _, _, _, _) => {
                    h1644
                }
            }, match v1.clone() {
                Adt1::C0(h1645, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1645
                }
            }, match v1.clone() {
                Adt1::C0(_, h1646, _, _, _, _, _, _, _, _, _, _) => {
                    h1646
                }
            }, match v3.clone() {
                Adt2::C0(_, h1647, _, _, _, _, _, _, _, _, _) => {
                    h1647
                }
            })? {
                f126(match v2.clone() {
                    Adt2::C0(_, _, h1648, _, _, _, _, _, _, _, _) => {
                        h1648
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1649, _, _, _, _, _, _, _, _, _) => {
                        h1649
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1650, _, _, _, _, _, _, _, _) => {
                        h1650
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1651, _, _, _, _, _, _, _, _) => {
                        h1651
                    }
                })?
            } else {
                false
            } {
                f126(match v2.clone() {
                    Adt2::C0(_, _, _, h1652, _, _, _, _, _, _, _) => {
                        h1652
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1653, _, _, _, _, _, _, _) => {
                        h1653
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1654, _, _, _, _, _, _) => {
                        h1654
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1655, _, _, _, _, _, _, _) => {
                        h1655
                    }
                })?
            } else {
                false
            } {
                f126(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1656, _, _, _, _, _, _) => {
                        h1656
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1657, _, _, _, _, _) => {
                        h1657
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1658, _, _, _, _) => {
                        h1658
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1659, _, _, _, _, _, _) => {
                        h1659
                    }
                })?
            } else {
                false
            } {
                let a1661 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1660, _, _, _) => {
                        h1660
                    }
                };
                let a1662 = 4294967295u64;
                nat_le(a1661, a1662)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1663, _, _, _, _, _, _, _, _, _) => {
                            h1663
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1664, _, _, _, _, _, _, _, _) => {
                            h1664
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1665, _, _, _, _, _, _, _) => {
                            h1665
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1666, _, _, _, _, _, _) => {
                            h1666
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1667, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1667
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1668, _, _, _, _, _, _, _, _, _) => {
                        h1668
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1669, _, _, _, _, _, _, _) => {
                        h1669
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1670, _, _, _, _, _) => {
                        h1670
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1671 = v0;
            let a1672 = 62u64;
            if nat_eq(a1671, a1672) {
                let a1674 = match v1.clone() {
                    Adt1::C0(h1673, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1673
                    }
                };
                let a1675 = 2147483648u64;
                if if if if if nat_lt(a1674, a1675) {
                    let a1677 = match v1.clone() {
                        Adt1::C0(_, h1676, _, _, _, _, _, _, _, _, _, _) => {
                            h1676
                        }
                    };
                    let a1678 = 2147483648u64;
                    nat_lt(a1677, a1678)
                } else {
                    false
                } {
                    let a1681 = match v1.clone() {
                        Adt1::C0(h1679, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1679
                        }
                    };
                    let a1682 = match v2.clone() {
                        Adt2::C0(_, h1680, _, _, _, _, _, _, _, _, _) => {
                            h1680
                        }
                    };
                    nat_lt(a1681, a1682)
                } else {
                    false
                } {
                    let a1685 = match v1.clone() {
                        Adt1::C0(_, h1683, _, _, _, _, _, _, _, _, _, _) => {
                            h1683
                        }
                    };
                    let a1686 = match v2.clone() {
                        Adt2::C0(_, h1684, _, _, _, _, _, _, _, _, _) => {
                            h1684
                        }
                    };
                    nat_lt(a1685, a1686)
                } else {
                    false
                } {
                    let a1689 = match v2.clone() {
                        Adt2::C0(_, _, h1687, _, _, _, _, _, _, _, _) => {
                            h1687
                        }
                    };
                    let a1690 = match v3.clone() {
                        Adt2::C0(_, _, h1688, _, _, _, _, _, _, _, _) => {
                            h1688
                        }
                    };
                    if if if nat_eq(a1689, a1690) {
                        let a1693 = match v2.clone() {
                            Adt2::C0(_, _, _, h1691, _, _, _, _, _, _, _) => {
                                h1691
                            }
                        };
                        let a1694 = match v3.clone() {
                            Adt2::C0(_, _, _, h1692, _, _, _, _, _, _, _) => {
                                h1692
                            }
                        };
                        nat_eq(a1693, a1694)
                    } else {
                        false
                    } {
                        let a1697 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1695, _, _, _, _, _, _) => {
                                h1695
                            }
                        };
                        let a1698 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1696, _, _, _, _, _, _) => {
                                h1696
                            }
                        };
                        nat_eq(a1697, a1698)
                    } else {
                        false
                    } {
                        let a1707 = match v3.clone() {
                            Adt2::C0(_, h1699, _, _, _, _, _, _, _, _, _) => {
                                h1699
                            }
                        };
                        let a1708 = {
                            let a1705 = {
                                let a1702 = match v2.clone() {
                                    Adt2::C0(_, h1700, _, _, _, _, _, _, _, _, _) => {
                                        h1700
                                    }
                                };
                                let a1703 = match v1.clone() {
                                    Adt1::C0(h1701, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1701
                                    }
                                };
                                nat_add(a1702, a1703)?
                            };
                            let a1706 = match v1.clone() {
                                Adt1::C0(_, h1704, _, _, _, _, _, _, _, _, _, _) => {
                                    h1704
                                }
                            };
                            nat_add(a1705, a1706)?
                        };
                        nat_eq(a1707, a1708)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1709, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1709
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1710 = v0;
                let a1711 = 63u64;
                if nat_eq(a1710, a1711) {
                    if if if if if f127(match v2.clone() {
                        Adt2::C0(_, h1712, _, _, _, _, _, _, _, _, _) => {
                            h1712
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1713, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1713
                        }
                    })? {
                        f127(match v2.clone() {
                            Adt2::C0(_, _, h1714, _, _, _, _, _, _, _, _) => {
                                h1714
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1715, _, _, _, _, _, _, _, _, _, _) => {
                                h1715
                            }
                        })?
                    } else {
                        false
                    } {
                        f127(match v2.clone() {
                            Adt2::C0(_, _, _, h1716, _, _, _, _, _, _, _) => {
                                h1716
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1717, _, _, _, _, _, _, _, _, _) => {
                                h1717
                            }
                        })?
                    } else {
                        false
                    } {
                        f127(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1718, _, _, _, _, _, _) => {
                                h1718
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1719, _, _, _, _, _, _, _, _) => {
                                h1719
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1722 = match v2.clone() {
                            Adt2::C0(_, h1720, _, _, _, _, _, _, _, _, _) => {
                                h1720
                            }
                        };
                        let a1723 = match v3.clone() {
                            Adt2::C0(_, h1721, _, _, _, _, _, _, _, _, _) => {
                                h1721
                            }
                        };
                        if if if nat_eq(a1722, a1723) {
                            let a1726 = match v2.clone() {
                                Adt2::C0(_, _, h1724, _, _, _, _, _, _, _, _) => {
                                    h1724
                                }
                            };
                            let a1727 = match v3.clone() {
                                Adt2::C0(_, _, h1725, _, _, _, _, _, _, _, _) => {
                                    h1725
                                }
                            };
                            nat_eq(a1726, a1727)
                        } else {
                            false
                        } {
                            let a1730 = match v2.clone() {
                                Adt2::C0(_, _, _, h1728, _, _, _, _, _, _, _) => {
                                    h1728
                                }
                            };
                            let a1731 = match v3.clone() {
                                Adt2::C0(_, _, _, h1729, _, _, _, _, _, _, _) => {
                                    h1729
                                }
                            };
                            nat_eq(a1730, a1731)
                        } else {
                            false
                        } {
                            let a1734 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1732, _, _, _, _, _, _) => {
                                    h1732
                                }
                            };
                            let a1735 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1733, _, _, _, _, _, _) => {
                                    h1733
                                }
                            };
                            nat_eq(a1734, a1735)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1736, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1736
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1737, _, _, _, _, _, _, _, _, _, _) => {
                                h1737
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1738, _, _, _, _, _, _, _, _, _) => {
                                h1738
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1739, _, _, _, _, _, _, _, _) => {
                                h1739
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

pub fn f58(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1740 = v3;
    match m1740 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1741 = v0;
                let a1742 = 4u64;
                nat_eq(a1741, a1742)
            };
            let v7: bool = {
                let a1744 = 0u64;
                let a1745 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1743, _, _, _, _, _, _, _) => {
                        h1743
                    }
                };
                nat_lt(a1744, a1745)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1746, _, _, _, _, _, _, _, _, _) => {
                    h1746
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1747, _, _, _, _, _, _, _, _) => {
                    h1747
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1748, _, _, _, _, _, _, _) => {
                    h1748
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1749, _, _, _, _, _, _) => {
                    h1749
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1750, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1750
                }
            }, match v1.clone() {
                Adt1::C0(_, h1751, _, _, _, _, _, _, _, _, _, _) => {
                    h1751
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1752, _, _, _, _, _, _, _, _, _) => {
                    h1752
                }
            });
            if if if if if if if if if if if if v6 {
                let a1754 = match v2.clone() {
                    Adt2::C0(h1753, _, _, _, _, _, _, _, _, _, _) => {
                        h1753
                    }
                };
                let a1755 = 0u64;
                if nat_eq(a1754, a1755) {
                    let a1757 = match v5.clone() {
                        Adt2::C0(h1756, _, _, _, _, _, _, _, _, _, _) => {
                            h1756
                        }
                    };
                    let a1758 = 0u64;
                    nat_eq(a1757, a1758)
                } else {
                    false
                }
            } else {
                let a1760 = match v2.clone() {
                    Adt2::C0(h1759, _, _, _, _, _, _, _, _, _, _) => {
                        h1759
                    }
                };
                let a1761 = 0u64;
                if if nat_eq(a1760, a1761) {
                    true
                } else {
                    let a1763 = match v2.clone() {
                        Adt2::C0(h1762, _, _, _, _, _, _, _, _, _, _) => {
                            h1762
                        }
                    };
                    let a1764 = 26u64;
                    nat_eq(a1763, a1764)
                } {
                    let a1766 = match v5.clone() {
                        Adt2::C0(h1765, _, _, _, _, _, _, _, _, _, _) => {
                            h1765
                        }
                    };
                    let a1767 = 0u64;
                    if nat_eq(a1766, a1767) {
                        true
                    } else {
                        let a1769 = match v5.clone() {
                            Adt2::C0(h1768, _, _, _, _, _, _, _, _, _, _) => {
                                h1768
                            }
                        };
                        let a1770 = 26u64;
                        nat_eq(a1769, a1770)
                    }
                } else {
                    false
                }
            } {
                let a1773 = match v2.clone() {
                    Adt2::C0(h1771, _, _, _, _, _, _, _, _, _, _) => {
                        h1771
                    }
                };
                let a1774 = match v4.clone() {
                    Adt2::C0(h1772, _, _, _, _, _, _, _, _, _, _) => {
                        h1772
                    }
                };
                nat_eq(a1773, a1774)
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
                let a1776 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1775, _, _, _, _, _) => {
                        h1775
                    }
                };
                let a1777 = 4u64;
                nat_eq(a1776, a1777)
            } else {
                false
            } {
                let a1778 = v8;
                let a1779 = f16(v2.clone(), 1u64)?;
                nat_le(a1778, a1779)
            } else {
                false
            } {
                let a1781 = match v1.clone() {
                    Adt1::C0(_, _, _, h1780, _, _, _, _, _, _, _, _) => {
                        h1780
                    }
                };
                let a1782 = 1073741824u64;
                nat_lt(a1781, a1782)
            } else {
                false
            } {
                let a1784 = match v1.clone() {
                    Adt1::C0(h1783, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1783
                    }
                };
                let a1785 = 4294967295u64;
                if if if if nat_le(a1784, a1785) {
                    let a1787 = match v1.clone() {
                        Adt1::C0(_, h1786, _, _, _, _, _, _, _, _, _, _) => {
                            h1786
                        }
                    };
                    let a1788 = 4294967295u64;
                    nat_le(a1787, a1788)
                } else {
                    false
                } {
                    let a1790 = match v1.clone() {
                        Adt1::C0(_, _, h1789, _, _, _, _, _, _, _, _, _) => {
                            h1789
                        }
                    };
                    let a1791 = 4294967295u64;
                    nat_le(a1790, a1791)
                } else {
                    false
                } {
                    let a1793 = match v1.clone() {
                        Adt1::C0(_, _, _, h1792, _, _, _, _, _, _, _, _) => {
                            h1792
                        }
                    };
                    let a1794 = 4294967295u64;
                    nat_le(a1793, a1794)
                } else {
                    false
                } {
                    let a1796 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1795, _, _, _, _, _, _, _) => {
                            h1795
                        }
                    };
                    let a1797 = 4294967295u64;
                    nat_le(a1796, a1797)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1800 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1798) => {
                            h1798
                        }
                    };
                    let a1801 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1799) => {
                            h1799
                        }
                    };
                    nat_eq(a1800, a1801)
                } else {
                    true
                }
            } else {
                false
            } {
                f128(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1802, _, _, _, _, _, _, _, _) => {
                        h1802
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1803, _) => {
                        h1803
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
                    let a1804 = v10;
                    let a1805 = v8;
                    nat_add(a1804, a1805)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1806, _, _, _, _, _, _, _, _) => {
                        h1806
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f59(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1808 = match v0.clone() {
        Adt2::C0(h1807, _, _, _, _, _, _, _, _, _, _) => {
            h1807
        }
    };
    let a1809 = 0u64;
    if if if nat_eq(a1808, a1809) {
        let a1811 = match v1.clone() {
            Adt2::C0(h1810, _, _, _, _, _, _, _, _, _, _) => {
                h1810
            }
        };
        let a1812 = 0u64;
        nat_eq(a1811, a1812)
    } else {
        false
    } {
        f124(match v0.clone() {
            Adt2::C0(_, h1813, _, _, _, _, _, _, _, _, _) => {
                h1813
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1814, _, _, _, _, _, _, _, _) => {
                h1814
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1815, _, _, _, _, _, _, _) => {
                h1815
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1816, _, _, _, _, _, _) => {
                h1816
            }
        }, match v1.clone() {
            Adt2::C0(_, h1817, _, _, _, _, _, _, _, _, _) => {
                h1817
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1818, _, _, _, _, _, _, _, _) => {
                h1818
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1819, _, _, _, _, _, _, _) => {
                h1819
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1820, _, _, _, _, _, _) => {
                h1820
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1821 = v2;
            let a1822 = f16(v0.clone(), 1u64)?;
            nat_add(a1821, a1822)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f60(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1824 = match v1.clone() {
        Adt2::C0(h1823, _, _, _, _, _, _, _, _, _, _) => {
            h1823
        }
    };
    let a1825 = 0u64;
    if if if nat_eq(a1824, a1825) {
        true
    } else {
        let a1827 = match v1.clone() {
            Adt2::C0(h1826, _, _, _, _, _, _, _, _, _, _) => {
                h1826
            }
        };
        let a1828 = 1u64;
        nat_eq(a1827, a1828)
    } {
        true
    } else {
        let a1830 = match v1.clone() {
            Adt2::C0(h1829, _, _, _, _, _, _, _, _, _, _) => {
                h1829
            }
        };
        let a1831 = 30u64;
        nat_eq(a1830, a1831)
    } {
        let a1834 = match v1.clone() {
            Adt2::C0(h1832, _, _, _, _, _, _, _, _, _, _) => {
                h1832
            }
        };
        let a1835 = match v2.clone() {
            Adt2::C0(h1833, _, _, _, _, _, _, _, _, _, _) => {
                h1833
            }
        };
        if if if if if nat_eq(a1834, a1835) {
            let a1838 = match v1.clone() {
                Adt2::C0(_, h1836, _, _, _, _, _, _, _, _, _) => {
                    h1836
                }
            };
            let a1839 = match v2.clone() {
                Adt2::C0(_, h1837, _, _, _, _, _, _, _, _, _) => {
                    h1837
                }
            };
            nat_eq(a1838, a1839)
        } else {
            false
        } {
            let a1842 = match v1.clone() {
                Adt2::C0(_, _, _, h1840, _, _, _, _, _, _, _) => {
                    h1840
                }
            };
            let a1843 = match v2.clone() {
                Adt2::C0(_, _, _, h1841, _, _, _, _, _, _, _) => {
                    h1841
                }
            };
            nat_eq(a1842, a1843)
        } else {
            false
        } {
            let a1846 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1844, _, _, _, _, _, _) => {
                    h1844
                }
            };
            let a1847 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1845, _, _, _, _, _, _) => {
                    h1845
                }
            };
            nat_eq(a1846, a1847)
        } else {
            false
        } {
            let a1848 = v0;
            let a1849 = 4u64;
            if nat_eq(a1848, a1849) {
                let a1851 = match v1.clone() {
                    Adt2::C0(_, _, h1850, _, _, _, _, _, _, _, _) => {
                        h1850
                    }
                };
                let a1852 = 1u64;
                if nat_eq(a1851, a1852) {
                    let a1855 = match v1.clone() {
                        Adt2::C0(_, h1853, _, _, _, _, _, _, _, _, _) => {
                            h1853
                        }
                    };
                    let a1856 = match v2.clone() {
                        Adt2::C0(_, _, h1854, _, _, _, _, _, _, _, _) => {
                            h1854
                        }
                    };
                    nat_eq(a1855, a1856)
                } else {
                    false
                }
            } else {
                let a1857 = v0;
                let a1858 = 4u64;
                if if nat_lt(a1857, a1858) {
                    let a1861 = match v1.clone() {
                        Adt2::C0(_, h1859, _, _, _, _, _, _, _, _, _) => {
                            h1859
                        }
                    };
                    let a1862 = match v1.clone() {
                        Adt2::C0(_, _, h1860, _, _, _, _, _, _, _, _) => {
                            h1860
                        }
                    };
                    nat_eq(a1861, a1862)
                } else {
                    false
                } {
                    let a1865 = match v1.clone() {
                        Adt2::C0(_, _, h1863, _, _, _, _, _, _, _, _) => {
                            h1863
                        }
                    };
                    let a1866 = match v2.clone() {
                        Adt2::C0(_, _, h1864, _, _, _, _, _, _, _, _) => {
                            h1864
                        }
                    };
                    nat_eq(a1865, a1866)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1868 = f121(match v1.clone() {
                Adt2::C0(h1867, _, _, _, _, _, _, _, _, _, _) => {
                    h1867
                }
            });
            match m1868 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1869) => {
                            h1869
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

pub fn f61(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f129(match v1.clone() {
        Adt2::C0(h1870, _, _, _, _, _, _, _, _, _, _) => {
            h1870
        }
    }, v0) {
        let a1873 = match v1.clone() {
            Adt2::C0(h1871, _, _, _, _, _, _, _, _, _, _) => {
                h1871
            }
        };
        let a1874 = match v2.clone() {
            Adt2::C0(h1872, _, _, _, _, _, _, _, _, _, _) => {
                h1872
            }
        };
        nat_eq(a1873, a1874)
    } else {
        false
    } {
        let a1877 = match v1.clone() {
            Adt2::C0(_, h1875, _, _, _, _, _, _, _, _, _) => {
                h1875
            }
        };
        let a1878 = match v2.clone() {
            Adt2::C0(_, h1876, _, _, _, _, _, _, _, _, _) => {
                h1876
            }
        };
        if if if nat_eq(a1877, a1878) {
            let a1881 = match v1.clone() {
                Adt2::C0(_, _, h1879, _, _, _, _, _, _, _, _) => {
                    h1879
                }
            };
            let a1882 = match v2.clone() {
                Adt2::C0(_, _, h1880, _, _, _, _, _, _, _, _) => {
                    h1880
                }
            };
            nat_eq(a1881, a1882)
        } else {
            false
        } {
            let a1885 = match v1.clone() {
                Adt2::C0(_, _, _, h1883, _, _, _, _, _, _, _) => {
                    h1883
                }
            };
            let a1886 = match v2.clone() {
                Adt2::C0(_, _, _, h1884, _, _, _, _, _, _, _) => {
                    h1884
                }
            };
            nat_eq(a1885, a1886)
        } else {
            false
        } {
            let a1889 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1887, _, _, _, _, _, _) => {
                    h1887
                }
            };
            let a1890 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1888, _, _, _, _, _, _) => {
                    h1888
                }
            };
            nat_eq(a1889, a1890)
        } else {
            false
        }
    } else {
        false
    } {
        let m1892 = f121(match v1.clone() {
            Adt2::C0(h1891, _, _, _, _, _, _, _, _, _, _) => {
                h1891
            }
        });
        match m1892 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1893) => {
                        h1893
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1894, _, _, _, _, _, _, _, _, _, _) => {
                        h1894
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f62(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1895, _, _, _, _, _, _, _, _, _, _, _) => {
                h1895
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1896, _, _, _, _, _, _, _, _, _) => {
                h1896
            }
        }
    };
    let a1897 = 0u64;
    let a1898 = v4;
    if if nat_lt(a1897, a1898) {
        let a1899 = v4;
        let a1900 = 2147483648u64;
        nat_lt(a1899, a1900)
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
        let v7: u64 = f130(match v5.clone() {
            Adt2::C0(_, _, h1901, _, _, _, _, _, _, _, _) => {
                h1901
            }
        }, v4)?;
        let v8: u64 = f130(match v5.clone() {
            Adt2::C0(_, _, _, h1902, _, _, _, _, _, _, _) => {
                h1902
            }
        }, v4)?;
        let a1904 = match v5.clone() {
            Adt2::C0(h1903, _, _, _, _, _, _, _, _, _, _) => {
                h1903
            }
        };
        let a1905 = 0u64;
        if if if if if nat_eq(a1904, a1905) {
            let a1907 = match v6.clone() {
                Adt2::C0(h1906, _, _, _, _, _, _, _, _, _, _) => {
                    h1906
                }
            };
            let a1908 = 0u64;
            nat_eq(a1907, a1908)
        } else {
            false
        } {
            let a1910 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1909, _, _, _, _, _, _) => {
                    h1909
                }
            };
            let a1911 = 1u64;
            nat_eq(a1910, a1911)
        } else {
            false
        } {
            let a1914 = match v5.clone() {
                Adt2::C0(_, h1912, _, _, _, _, _, _, _, _, _) => {
                    h1912
                }
            };
            let a1915 = match v6.clone() {
                Adt2::C0(_, h1913, _, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            };
            nat_eq(a1914, a1915)
        } else {
            false
        } {
            if v0 {
                let a1917 = match v5.clone() {
                    Adt2::C0(_, _, h1916, _, _, _, _, _, _, _, _) => {
                        h1916
                    }
                };
                let a1918 = 2147483648u64;
                if if if nat_lt(a1917, a1918) {
                    let a1920 = match v5.clone() {
                        Adt2::C0(_, _, _, h1919, _, _, _, _, _, _, _) => {
                            h1919
                        }
                    };
                    let a1921 = 2147483648u64;
                    nat_lt(a1920, a1921)
                } else {
                    false
                } {
                    let a1924 = {
                        let a1922 = v7;
                        let a1923 = v8;
                        nat_mul(a1922, a1923)?
                    };
                    let a1925 = 2147483648u64;
                    nat_le(a1924, a1925)
                } else {
                    false
                } {
                    f131(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1927 = match v1.clone() {
                    Adt1::C0(h1926, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1926
                    }
                };
                let a1928 = v7;
                if if if if nat_eq(a1927, a1928) {
                    let a1930 = match v1 {
                        Adt1::C0(_, h1929, _, _, _, _, _, _, _, _, _, _) => {
                            h1929
                        }
                    };
                    let a1931 = v8;
                    nat_eq(a1930, a1931)
                } else {
                    false
                } {
                    f132(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1933 = match v6.clone() {
                        Adt2::C0(_, _, h1932, _, _, _, _, _, _, _, _) => {
                            h1932
                        }
                    };
                    let a1934 = v4;
                    if nat_eq(a1933, a1934) {
                        let a1936 = match v6.clone() {
                            Adt2::C0(_, _, _, h1935, _, _, _, _, _, _, _) => {
                                h1935
                            }
                        };
                        let a1937 = v4;
                        nat_eq(a1936, a1937)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1941 = match v6 {
                        Adt2::C0(_, _, _, _, h1938, _, _, _, _, _, _) => {
                            h1938
                        }
                    };
                    let a1942 = {
                        let a1939 = v7;
                        let a1940 = v8;
                        nat_mul(a1939, a1940)?
                    };
                    nat_eq(a1941, a1942)
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

pub fn f63(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1944 = match v2.clone() {
        Adt2::C0(h1943, _, _, _, _, _, _, _, _, _, _) => {
            h1943
        }
    };
    let a1945 = 0u64;
    if if if if if nat_eq(a1944, a1945) {
        true
    } else {
        let a1947 = match v2.clone() {
            Adt2::C0(h1946, _, _, _, _, _, _, _, _, _, _) => {
                h1946
            }
        };
        let a1948 = 1u64;
        nat_eq(a1947, a1948)
    } {
        true
    } else {
        let a1950 = match v2.clone() {
            Adt2::C0(h1949, _, _, _, _, _, _, _, _, _, _) => {
                h1949
            }
        };
        let a1951 = 30u64;
        nat_eq(a1950, a1951)
    } {
        let a1954 = match v2.clone() {
            Adt2::C0(h1952, _, _, _, _, _, _, _, _, _, _) => {
                h1952
            }
        };
        let a1955 = match v3.clone() {
            Adt2::C0(h1953, _, _, _, _, _, _, _, _, _, _) => {
                h1953
            }
        };
        nat_eq(a1954, a1955)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a1957 = match v1.clone() {
                Adt1::C0(h1956, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1956
                }
            };
            let a1958 = 4294967296u64;
            nat_lt(a1957, a1958)
        } else {
            false
        } {
            let a1960 = match v1.clone() {
                Adt1::C0(_, h1959, _, _, _, _, _, _, _, _, _, _) => {
                    h1959
                }
            };
            let a1961 = 4294967296u64;
            nat_lt(a1960, a1961)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a1963 = match v2.clone() {
                Adt2::C0(h1962, _, _, _, _, _, _, _, _, _, _) => {
                    h1962
                }
            };
            let a1964 = 0u64;
            if nat_eq(a1963, a1964) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a1965 = v0;
            let a1966 = 31u64;
            if nat_eq(a1965, a1966) {
                0u64
            } else {
                let a1967 = v0;
                let a1968 = 47u64;
                if nat_eq(a1967, a1968) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1969, _, _, _, _, _, _, _, _, _, _) => {
                h1969
            }
        }, match v1.clone() {
            Adt1::C0(h1970, _, _, _, _, _, _, _, _, _, _, _) => {
                h1970
            }
        }, match v1 {
            Adt1::C0(_, h1971, _, _, _, _, _, _, _, _, _, _) => {
                h1971
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f64(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1973 = match v0.clone() {
        Adt2::C0(h1972, _, _, _, _, _, _, _, _, _, _) => {
            h1972
        }
    };
    let a1974 = 0u64;
    if if if nat_eq(a1973, a1974) {
        let a1976 = match v1.clone() {
            Adt2::C0(h1975, _, _, _, _, _, _, _, _, _, _) => {
                h1975
            }
        };
        let a1977 = 0u64;
        nat_eq(a1976, a1977)
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

pub fn f65(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1979 = match v0.clone() {
        Adt2::C0(h1978, _, _, _, _, _, _, _, _, _, _) => {
            h1978
        }
    };
    let a1980 = 0u64;
    if if nat_eq(a1979, a1980) {
        let a1982 = match v1.clone() {
            Adt2::C0(h1981, _, _, _, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        let a1983 = 26u64;
        nat_eq(a1982, a1983)
    } else {
        false
    } {
        let a1985 = match v0.clone() {
            Adt2::C0(_, _, _, h1984, _, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1986 = 1u64;
        if if if if if if if nat_eq(a1985, a1986) {
            let a1988 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1987, _, _, _, _, _, _) => {
                    h1987
                }
            };
            let a1989 = 1u64;
            nat_eq(a1988, a1989)
        } else {
            false
        } {
            let a1991 = match v0.clone() {
                Adt2::C0(_, h1990, _, _, _, _, _, _, _, _, _) => {
                    h1990
                }
            };
            let a1992 = 2147483647u64;
            nat_le(a1991, a1992)
        } else {
            false
        } {
            let a1995 = match v1.clone() {
                Adt2::C0(_, h1993, _, _, _, _, _, _, _, _, _) => {
                    h1993
                }
            };
            let a1996 = match v0.clone() {
                Adt2::C0(_, _, h1994, _, _, _, _, _, _, _, _) => {
                    h1994
                }
            };
            nat_eq(a1995, a1996)
        } else {
            false
        } {
            let a1998 = match v1.clone() {
                Adt2::C0(_, _, h1997, _, _, _, _, _, _, _, _) => {
                    h1997
                }
            };
            let a1999 = 1u64;
            nat_eq(a1998, a1999)
        } else {
            false
        } {
            let a2001 = match v1.clone() {
                Adt2::C0(_, _, _, h2000, _, _, _, _, _, _, _) => {
                    h2000
                }
            };
            let a2002 = 1u64;
            nat_eq(a2001, a2002)
        } else {
            false
        } {
            let a2004 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2003, _, _, _, _, _, _) => {
                    h2003
                }
            };
            let a2005 = 1u64;
            nat_eq(a2004, a2005)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a2008 = match v0.clone() {
                    Adt2::C0(_, _, h2006, _, _, _, _, _, _, _, _) => {
                        h2006
                    }
                };
                let a2009 = f133(match v0.clone() {
                    Adt2::C0(_, h2007, _, _, _, _, _, _, _, _, _) => {
                        h2007
                    }
                })?;
                nat_mul(a2008, a2009)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f66(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2010 = v1;
    match m2010 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f134(f20(v0.clone(), 1u64))?;
            let a2012 = match v0.clone() {
                Adt2::C0(h2011, _, _, _, _, _, _, _, _, _, _) => {
                    h2011
                }
            };
            let a2013 = 26u64;
            if if if nat_eq(a2012, a2013) {
                let a2015 = match v3.clone() {
                    Adt2::C0(h2014, _, _, _, _, _, _, _, _, _, _) => {
                        h2014
                    }
                };
                let a2016 = 26u64;
                nat_eq(a2015, a2016)
            } else {
                false
            } {
                let a2018 = match v2.clone() {
                    Adt2::C0(h2017, _, _, _, _, _, _, _, _, _, _) => {
                        h2017
                    }
                };
                let a2019 = 27u64;
                nat_eq(a2018, a2019)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a2021 = match v2.clone() {
                        Adt2::C0(_, h2020, _, _, _, _, _, _, _, _, _) => {
                            h2020
                        }
                    };
                    let a2022 = 1u64;
                    nat_eq(a2021, a2022)
                } else {
                    false
                } {
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
                } {
                    let a2032 = v4;
                    let a2033 = 9223372036854775808u64;
                    nat_lt(a2032, a2033)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2036 = 2u64;
                        let a2037 = {
                            let a2034 = v4;
                            let a2035 = 2u64;
                            nat_mul(a2034, a2035)?
                        };
                        nat_add(a2036, a2037)?
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

pub fn f67(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2038, _, _, _, _, _, _, _, _, _, _) => {
            h2038
        }
    };
    let a2039 = v3;
    let a2040 = 0u64;
    if if nat_eq(a2039, a2040) {
        true
    } else {
        let a2041 = v0;
        let a2042 = 13u64;
        if nat_eq(a2041, a2042) {
            let a2043 = v3;
            let a2044 = 1u64;
            if nat_eq(a2043, a2044) {
                true
            } else {
                let a2045 = v3;
                let a2046 = 30u64;
                nat_eq(a2045, a2046)
            }
        } else {
            false
        }
    } {
        let a2048 = v3;
        let a2049 = match v2.clone() {
            Adt2::C0(h2047, _, _, _, _, _, _, _, _, _, _) => {
                h2047
            }
        };
        if if nat_eq(a2048, a2049) {
            let a2050 = v0;
            let a2051 = 13u64;
            if nat_eq(a2050, a2051) {
                let a2053 = match v2.clone() {
                    Adt2::C0(_, h2052, _, _, _, _, _, _, _, _, _) => {
                        h2052
                    }
                };
                let a2054 = 1u64;
                if if if nat_eq(a2053, a2054) {
                    let a2056 = match v2.clone() {
                        Adt2::C0(_, _, h2055, _, _, _, _, _, _, _, _) => {
                            h2055
                        }
                    };
                    let a2057 = 1u64;
                    nat_eq(a2056, a2057)
                } else {
                    false
                } {
                    let a2059 = match v2.clone() {
                        Adt2::C0(_, _, _, h2058, _, _, _, _, _, _, _) => {
                            h2058
                        }
                    };
                    let a2060 = 1u64;
                    nat_eq(a2059, a2060)
                } else {
                    false
                } {
                    let a2062 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2061, _, _, _, _, _, _) => {
                            h2061
                        }
                    };
                    let a2063 = 1u64;
                    nat_eq(a2062, a2063)
                } else {
                    false
                }
            } else {
                let a2065 = match v2.clone() {
                    Adt2::C0(_, h2064, _, _, _, _, _, _, _, _, _) => {
                        h2064
                    }
                };
                let a2066 = 1u64;
                if if if nat_eq(a2065, a2066) {
                    let a2069 = match v1.clone() {
                        Adt2::C0(_, _, h2067, _, _, _, _, _, _, _, _) => {
                            h2067
                        }
                    };
                    let a2070 = match v2.clone() {
                        Adt2::C0(_, _, h2068, _, _, _, _, _, _, _, _) => {
                            h2068
                        }
                    };
                    nat_eq(a2069, a2070)
                } else {
                    false
                } {
                    let a2073 = match v1.clone() {
                        Adt2::C0(_, _, _, h2071, _, _, _, _, _, _, _) => {
                            h2071
                        }
                    };
                    let a2074 = match v2.clone() {
                        Adt2::C0(_, _, _, h2072, _, _, _, _, _, _, _) => {
                            h2072
                        }
                    };
                    nat_eq(a2073, a2074)
                } else {
                    false
                } {
                    let a2077 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2075, _, _, _, _, _, _) => {
                            h2075
                        }
                    };
                    let a2078 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2076, _, _, _, _, _, _) => {
                            h2076
                        }
                    };
                    nat_eq(a2077, a2078)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f135(v4.clone(), v0, v3)?, {
                let a2079 = v3;
                let a2080 = 0u64;
                if nat_eq(a2079, a2080) {
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

pub fn f68(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2081 = v0;
    let a2082 = 89u64;
    if if nat_eq(a2081, a2082) {
        let a2083 = v1;
        let a2084 = 14u64;
        nat_eq(a2083, a2084)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2085 = v0;
        let a2086 = 89u64;
        if if nat_eq(a2085, a2086) {
            let a2087 = v1;
            let a2088 = 15u64;
            nat_eq(a2087, a2088)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2089 = v0;
            let a2090 = 89u64;
            if if nat_eq(a2089, a2090) {
                let a2091 = 8u64;
                let a2092 = v1;
                if nat_le(a2091, a2092) {
                    let a2093 = v1;
                    let a2094 = 9u64;
                    nat_le(a2093, a2094)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2095 = v1;
                    let a2096 = 12u64;
                    nat_add(a2095, a2096)?
                }))
            } else {
                let a2097 = v0;
                let a2098 = 89u64;
                if if nat_eq(a2097, a2098) {
                    let a2099 = v1;
                    let a2100 = 4u64;
                    nat_eq(a2099, a2100)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2101 = 10u64;
                    let a2102 = v0;
                    if if nat_le(a2101, a2102) {
                        let a2103 = v0;
                        let a2104 = 12u64;
                        nat_le(a2103, a2104)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2105 = v0;
                            let a2106 = 3u64;
                            nat_add(a2105, a2106)?
                        }))
                    } else {
                        let a2107 = v0;
                        let a2108 = 89u64;
                        if if nat_eq(a2107, a2108) {
                            let a2109 = v1;
                            let a2110 = 7u64;
                            nat_eq(a2109, a2110)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2111 = v0;
                            let a2112 = 89u64;
                            if if nat_eq(a2111, a2112) {
                                let a2113 = v1;
                                let a2114 = 10u64;
                                nat_eq(a2113, a2114)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2115 = v0;
                                let a2116 = 89u64;
                                if if nat_eq(a2115, a2116) {
                                    let a2117 = v1;
                                    let a2118 = 13u64;
                                    nat_eq(a2117, a2118)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2119 = v0;
                                    let a2120 = 8u64;
                                    if nat_eq(a2119, a2120) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2121 = v0;
                                        let a2122 = 9u64;
                                        if nat_eq(a2121, a2122) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2123 = v0;
                                            let a2124 = 89u64;
                                            if nat_eq(a2123, a2124) {
                                                let a2125 = v1;
                                                let a2126 = 0u64;
                                                if nat_eq(a2125, a2126) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2127 = v1;
                                                    let a2128 = 1u64;
                                                    if nat_eq(a2127, a2128) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2129 = v1;
                                                        let a2130 = 2u64;
                                                        if nat_eq(a2129, a2130) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2131 = v1;
                                                            let a2132 = 3u64;
                                                            if nat_eq(a2131, a2132) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2133 = v1;
                                                                let a2134 = 6u64;
                                                                if nat_eq(a2133, a2134) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2135 = v1;
                                                                    let a2136 = 12u64;
                                                                    if nat_eq(a2135, a2136) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2137 = v1;
                                                                        let a2138 = 11u64;
                                                                        if nat_eq(a2137, a2138) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2139 = v1;
                                                                            let a2140 = 18u64;
                                                                            if nat_eq(a2139, a2140) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2141 = v1;
                                                                                let a2142 = 19u64;
                                                                                if nat_eq(a2141, a2142) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2143 = v1;
                                                                                    let a2144 = 20u64;
                                                                                    if nat_eq(a2143, a2144) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2145 = v1;
                                                                                        let a2146 = 21u64;
                                                                                        if nat_eq(a2145, a2146) {
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

pub fn f69(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2147 = f136(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2147 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2148 = v0;
            let a2149 = 2u64;
            if if if if nat_eq(a2148, a2149) {
                true
            } else {
                let a2150 = v0;
                let a2151 = 5u64;
                nat_eq(a2150, a2151)
            } {
                true
            } else {
                let a2152 = v0;
                let a2153 = 6u64;
                nat_eq(a2152, a2153)
            } {
                true
            } else {
                let a2154 = v0;
                let a2155 = 7u64;
                nat_eq(a2154, a2155)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2156 = v3;
                    match m2156 {
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

pub fn f70(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2158 = match v1.clone() {
        Adt2::C0(h2157, _, _, _, _, _, _, _, _, _, _) => {
            h2157
        }
    };
    let a2159 = 0u64;
    if if if if if nat_eq(a2158, a2159) {
        true
    } else {
        let a2161 = match v1.clone() {
            Adt2::C0(h2160, _, _, _, _, _, _, _, _, _, _) => {
                h2160
            }
        };
        let a2162 = 1u64;
        nat_eq(a2161, a2162)
    } {
        true
    } else {
        let a2164 = match v1.clone() {
            Adt2::C0(h2163, _, _, _, _, _, _, _, _, _, _) => {
                h2163
            }
        };
        let a2165 = 30u64;
        nat_eq(a2164, a2165)
    } {
        let a2168 = match v1.clone() {
            Adt2::C0(h2166, _, _, _, _, _, _, _, _, _, _) => {
                h2166
            }
        };
        let a2169 = match v2.clone() {
            Adt2::C0(h2167, _, _, _, _, _, _, _, _, _, _) => {
                h2167
            }
        };
        if nat_eq(a2168, a2169) {
            true
        } else {
            let a2171 = match v2.clone() {
                Adt2::C0(h2170, _, _, _, _, _, _, _, _, _, _) => {
                    h2170
                }
            };
            let a2172 = 0u64;
            nat_eq(a2171, a2172)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2174 = f16(v2.clone(), 1u64)?;
            let a2175 = f137(v0, match v2.clone() {
                Adt2::C0(h2173, _, _, _, _, _, _, _, _, _, _) => {
                    h2173
                }
            });
            nat_mul(a2174, a2175)?
        }, {
            let a2177 = match v2.clone() {
                Adt2::C0(h2176, _, _, _, _, _, _, _, _, _, _) => {
                    h2176
                }
            };
            let a2178 = 0u64;
            if nat_eq(a2177, a2178) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2179, _, _, _, _, _, _, _, _, _, _) => {
                h2179
            }
        }, match v2 {
            Adt2::C0(h2180, _, _, _, _, _, _, _, _, _, _) => {
                h2180
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f71(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f138(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2181, _, _, _) => {
            h2181
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2182, _, _) => {
            h2182
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2183, _) => {
            h2183
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2184) => {
            h2184
        }
    })
}

pub fn f72(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2186 = v0;
        let a2187 = match v1.clone() {
            Adt3::C0(h2185, _, _, _, _, _, _, _) => {
                h2185
            }
        };
        let a2188 = 0u64;
        nat_quot(a2186, a2187, a2188)
    };
    let v4: u64 = {
        let a2190 = v3;
        let a2191 = match v1.clone() {
            Adt3::C0(_, h2189, _, _, _, _, _, _) => {
                h2189
            }
        };
        let a2192 = 0u64;
        nat_quot(a2190, a2191, a2192)
    };
    let a2193 = v2;
    let a2194 = 0u64;
    if nat_eq(a2193, a2194) {
        let a2196 = v0;
        let a2197 = match v1 {
            Adt3::C0(h2195, _, _, _, _, _, _, _) => {
                h2195
            }
        };
        let a2198 = 0u64;
        nat_rem(a2196, a2197, a2198)
    } else {
        let a2199 = v2;
        let a2200 = 1u64;
        if nat_eq(a2199, a2200) {
            let a2202 = v3;
            let a2203 = match v1 {
                Adt3::C0(_, h2201, _, _, _, _, _, _) => {
                    h2201
                }
            };
            let a2204 = 0u64;
            nat_rem(a2202, a2203, a2204)
        } else {
            let a2205 = v2;
            let a2206 = 2u64;
            if nat_eq(a2205, a2206) {
                let a2208 = v4;
                let a2209 = match v1 {
                    Adt3::C0(_, _, h2207, _, _, _, _, _) => {
                        h2207
                    }
                };
                let a2210 = 0u64;
                nat_rem(a2208, a2209, a2210)
            } else {
                let a2212 = v4;
                let a2213 = match v1 {
                    Adt3::C0(_, _, h2211, _, _, _, _, _) => {
                        h2211
                    }
                };
                let a2214 = 0u64;
                nat_quot(a2212, a2213, a2214)
            }
        }
    }
}

pub fn f73(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2216 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2215) => {
            h2215
        }
    };
    let a2217 = 0u64;
    if nat_eq(a2216, a2217) {
        Ok(v2)
    } else {
        let a2222 = {
            let a2219 = v2;
            let a2220 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2218) => {
                    h2218
                }
            };
            nat_mul(a2219, a2220)?
        };
        let a2223 = f72(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2221, _, _, _, _, _, _) => {
                h2221
            }
        }, 3u64);
        nat_add(a2222, a2223)
    }
}

pub fn f74(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2233 = {
            let a2230 = {
                let a2225 = v2;
                let a2226 = match v0.clone() {
                    Adt6::C0(_, h2224, _, _) => {
                        h2224
                    }
                };
                nat_mul(a2225, a2226)?
            };
            let a2231 = {
                let a2228 = v3;
                let a2229 = match v0.clone() {
                    Adt6::C0(_, _, _, h2227) => {
                        h2227
                    }
                };
                nat_mul(a2228, a2229)?
            };
            nat_add(a2230, a2231)?
        };
        let a2234 = f139(match v0.clone() {
            Adt6::C0(_, _, h2232, _) => {
                h2232
            }
        });
        nat_add(a2233, a2234)?
    };
    let v5: u64 = f140(match v0 {
        Adt6::C0(_, _, h2235, _) => {
            h2235
        }
    });
    let a2236 = v5;
    let a2237 = v4;
    if if nat_le(a2236, a2237) {
        let a2240 = {
            let a2238 = v4;
            let a2239 = v5;
            nat_sub(a2238, a2239)
        };
        let a2241 = v1;
        nat_lt(a2240, a2241)
    } else {
        false
    } {
        Ok(Some({
            let a2242 = v4;
            let a2243 = v5;
            nat_sub(a2242, a2243)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f75(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2244, _, _, _, _, _) => {
            h2244
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2245, _, _, _, _, _, _) => {
            h2245
        }
    };
    let v7: u64 = f72(v1, v6.clone(), 2u64);
    let a2247 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2246) => {
            h2246
        }
    };
    let a2248 = 0u64;
    if nat_eq(a2247, a2248) {
        Ok(Adt9::C8(Some(f71(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2251 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2250, _, _, _, _) => {
                    h2250
                }
            };
            match m2251 {
                Adt3::C0(_, _, h2249, _, _, _, _, _) => {
                    h2249
                }
            }
        };
        let m2257 = f74(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2252, _) => {
                h2252
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2253, _, _, _, _, _) => {
                h2253
            }
        }, {
            let a2254 = v2;
            let a2255 = v8;
            let a2256 = 0u64;
            nat_rem(a2254, a2255, a2256)
        }, v7)?;
        match m2257 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2264 = {
                        let a2262 = {
                            let a2258 = v2;
                            let a2259 = v8;
                            let a2260 = 0u64;
                            nat_quot(a2258, a2259, a2260)
                        };
                        let a2263 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2261) => {
                                h2261
                            }
                        };
                        nat_mul(a2262, a2263)?
                    };
                    let a2265 = f72(v1, v6, 3u64);
                    nat_add(a2264, a2265)?
                };
                Ok(Adt9::C8(Some(f71(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f76(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2266 = v1;
    match m2266 {
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
            f141(f142(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f143(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f144(v0.clone(), v24);
            f141(f145(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2267 = v3;
    match m2267 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f146(v0, v1)?, f146(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f148(v0, v1, v2.clone())?, f146(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f149(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f150(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f151(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f150(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f152(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f153(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f154(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f146(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f155(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v32, v33) => {
            f156(v0, v1, v2, v32, v33)
        }
        Adt4::C12(v34, v35, v36) => {
            f157(v0, v1, v2, v34, v35, v36)
        }
        Adt4::C13(_, v38, _, _) => {
            Ok(Adt9::C3(v38, v38, f146(v0, v1)?, f146(v0, v2)?))
        }
        Adt4::C14(v41, v42, v43) => {
            let a2270 = v41;
            let a2271 = 23u64;
            if nat_eq(a2270, a2271) {
                f158(v0, v1, v2, v42, v43)
            } else {
                Ok(Adt9::C3(v42, v43, f146(v0, v1)?, f146(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f159(v0, v1, v2)
        }
        Adt4::C16(v44, v45) => {
            f160(v0, v1, v2, v44, v45)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v46, v47) => {
            f161(v0, v1, v47, v46)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f146(f147(v0, match v2.clone() {
                Adt3::C0(h2268, _, _, _, _, _, _, _) => {
                    h2268
                }
            }, match v2.clone() {
                Adt3::C0(_, h2269, _, _, _, _, _, _) => {
                    h2269
                }
            })?, v1)?, f146(v0, v2)?))
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
        Adt4::C25(v52, v53, v54) => {
            f162(v0, v52, v53, v54)
        }
        Adt4::C26(v60, _, _, _) => {
            let a2272 = v60;
            let a2273 = 64u64;
            if if if nat_eq(a2272, a2273) {
                true
            } else {
                let a2274 = v60;
                let a2275 = 65u64;
                nat_eq(a2274, a2275)
            } {
                true
            } else {
                let a2276 = v60;
                let a2277 = 66u64;
                nat_eq(a2276, a2277)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v59) => {
            f88(v0, v1, v2, v59)
        }
        Adt4::C28(v55) => {
            f85(v0, v1, v2, v55, 0u64)
        }
        Adt4::C29(v56) => {
            f86(v0, v1, v2, v56)
        }
        Adt4::C30(v57, _) => {
            f87(v0, v1, v2, v57)
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C32(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C33(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f123(v3, v4);
    let v6: u64 = {
        let a2278 = v0;
        let a2279 = v5;
        let a2280 = 0u64;
        nat_quot(a2278, a2279, a2280)
    };
    let a2281 = v5;
    let a2282 = 1u64;
    if nat_eq(a2281, a2282) {
        Ok(f141(f163(v3, v4), v3, v4, 0u64, f146(v6, v1)?, 2u64, 0u64, 3u64, f146(v6, v2)?))
    } else {
        let a2286 = {
            let a2283 = v0;
            let a2284 = 2u64;
            let a2285 = 0u64;
            nat_rem(a2283, a2284, a2285)
        };
        let a2287 = 0u64;
        if nat_eq(a2286, a2287) {
            Ok(f141(f163(v3, 0u64), v3, 0u64, 0u64, f146(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f141(f163(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f146(v6, v2)?))
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f110(v1)?;
    let v4: u64 = {
        let a2288 = v0;
        let a2289 = v3;
        let a2290 = 0u64;
        nat_rem(a2288, a2289, a2290)
    };
    let v5: u64 = {
        let a2293 = {
            let a2291 = v1;
            let a2292 = 3u64;
            nat_mul(a2291, a2292)?
        };
        let a2294 = 1u64;
        nat_add(a2293, a2294)?
    };
    let a2295 = v4;
    let a2296 = 0u64;
    if nat_eq(a2295, a2296) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a2297 = v4;
        let a2298 = v5;
        if nat_eq(a2297, a2298) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a2303 = v4;
                let a2304 = {
                    let a2299 = v4;
                    let a2300 = v5;
                    if nat_lt(a2299, a2300) {
                        1u64
                    } else {
                        let a2301 = v5;
                        let a2302 = 1u64;
                        nat_add(a2301, a2302)?
                    }
                };
                nat_sub(a2303, a2304)
            };
            let v7: u64 = {
                let a2305 = v6;
                let a2306 = 3u64;
                let a2307 = 0u64;
                nat_rem(a2305, a2306, a2307)
            };
            let v8: u64 = {
                let a2318 = 4u64;
                let a2319 = {
                    let a2316 = {
                        let a2311 = {
                            let a2308 = v0;
                            let a2309 = v3;
                            let a2310 = 0u64;
                            nat_quot(a2308, a2309, a2310)
                        };
                        let a2312 = v1;
                        nat_mul(a2311, a2312)?
                    };
                    let a2317 = {
                        let a2313 = v6;
                        let a2314 = 3u64;
                        let a2315 = 0u64;
                        nat_quot(a2313, a2314, a2315)
                    };
                    nat_add(a2316, a2317)?
                };
                nat_mul(a2318, a2319)?
            };
            let a2320 = v4;
            let a2321 = v5;
            if nat_lt(a2320, a2321) {
                let a2322 = v7;
                let a2323 = 0u64;
                if nat_eq(a2322, a2323) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2324 = v7;
                    let a2325 = 1u64;
                    if nat_eq(a2324, a2325) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a2326 = v7;
                let a2327 = 0u64;
                if nat_eq(a2326, a2327) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2328 = v7;
                    let a2329 = 1u64;
                    if nat_eq(a2328, a2329) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2330 = v0;
        let a2331 = 6u64;
        let a2332 = 0u64;
        nat_quot(a2330, a2331, a2332)
    };
    let v5: u64 = {
        let a2333 = v0;
        let a2334 = 6u64;
        let a2335 = 0u64;
        nat_rem(a2333, a2334, a2335)
    };
    let a2336 = v5;
    let a2337 = 0u64;
    if nat_eq(a2336, a2337) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f146(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2338 = v5;
        let a2339 = 1u64;
        if nat_eq(a2338, a2339) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a2340 = v5;
            let a2341 = 2u64;
            if nat_eq(a2340, a2341) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f146(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2342 = v5;
                let a2343 = 3u64;
                if nat_eq(a2342, a2343) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2344 = v5;
                    let a2345 = 4u64;
                    if nat_eq(a2344, a2345) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f146(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f146(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f133(match v1.clone() {
        Adt3::C0(h2346, _, _, _, _, _, _, _) => {
            h2346
        }
    })?;
    let v5: u64 = {
        let a2347 = v0;
        let a2348 = v4;
        let a2349 = 0u64;
        nat_quot(a2347, a2348, a2349)
    };
    let v6: u64 = {
        let a2350 = v0;
        let a2351 = v4;
        let a2352 = 0u64;
        nat_rem(a2350, a2351, a2352)
    };
    let a2353 = v6;
    let a2354 = 0u64;
    if nat_eq(a2353, a2354) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2355 = v6;
        let a2356 = 1u64;
        if nat_eq(a2355, a2356) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2359 = {
                let a2357 = v6;
                let a2358 = 1u64;
                nat_add(a2357, a2358)?
            };
            let a2360 = v4;
            if nat_eq(a2359, a2360) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f146(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2361 = v6;
                    let a2362 = 2u64;
                    nat_sub(a2361, a2362)
                };
                let v8: u64 = {
                    let a2363 = v7;
                    let a2364 = 3u64;
                    let a2365 = 0u64;
                    nat_quot(a2363, a2364, a2365)
                };
                let v9: u64 = {
                    let a2366 = v7;
                    let a2367 = 3u64;
                    let a2368 = 0u64;
                    nat_rem(a2366, a2367, a2368)
                };
                let v10: u64 = f146({
                    let a2372 = {
                        let a2370 = v5;
                        let a2371 = match v1.clone() {
                            Adt3::C0(h2369, _, _, _, _, _, _, _) => {
                                h2369
                            }
                        };
                        nat_mul(a2370, a2371)?
                    };
                    let a2373 = v8;
                    nat_add(a2372, a2373)?
                }, v1)?;
                let a2374 = v9;
                let a2375 = 0u64;
                if nat_eq(a2374, a2375) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2376 = v9;
                    let a2377 = 1u64;
                    if nat_eq(a2376, a2377) {
                        let a2378 = v3;
                        let a2379 = 2u64;
                        if nat_eq(a2378, a2379) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2380 = v3;
                        let a2381 = 2u64;
                        if if nat_eq(a2380, a2381) {
                            true
                        } else {
                            let a2382 = v3;
                            let a2383 = 3u64;
                            nat_eq(a2382, a2383)
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

pub fn f82(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2385 = 2u64;
        let a2386 = match v2.clone() {
            Adt3::C0(_, h2384, _, _, _, _, _, _) => {
                h2384
            }
        };
        nat_mul(a2385, a2386)?
    };
    let a2387 = v1;
    let a2388 = v6;
    if nat_lt(a2387, a2388) {
        let a2392 = {
            let a2389 = v1;
            let a2390 = 2u64;
            let a2391 = 0u64;
            nat_rem(a2389, a2390, a2391)
        };
        let a2393 = 0u64;
        if nat_eq(a2392, a2393) {
            Ok(Adt9::C4(19u64, 4u64, f146({
                let a2394 = v1;
                let a2395 = 2u64;
                let a2396 = 0u64;
                nat_quot(a2394, a2395, a2396)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2397 = v5;
            let a2398 = 2147483648u64;
            if if nat_lt(a2397, a2398) {
                let a2400 = v5;
                let a2401 = match v3 {
                    Adt3::C0(_, h2399, _, _, _, _, _, _) => {
                        h2399
                    }
                };
                nat_lt(a2400, a2401)
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
            let a2404 = match v3.clone() {
                Adt3::C0(h2402, _, _, _, _, _, _, _) => {
                    h2402
                }
            };
            let a2405 = match v3.clone() {
                Adt3::C0(_, h2403, _, _, _, _, _, _) => {
                    h2403
                }
            };
            nat_mul(a2404, a2405)?
        };
        let a2408 = {
            let a2406 = v1;
            let a2407 = v6;
            nat_sub(a2406, a2407)
        };
        let a2409 = v7;
        if nat_lt(a2408, a2409) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f146({
                let a2410 = v1;
                let a2411 = v6;
                nat_sub(a2410, a2411)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2414 = {
                    let a2412 = v1;
                    let a2413 = v6;
                    nat_sub(a2412, a2413)
                };
                let a2415 = v7;
                nat_sub(a2414, a2415)
            };
            let v9: u64 = {
                let a2417 = 1u64;
                let a2418 = f164(v0, match v2.clone() {
                    Adt3::C0(h2416, _, _, _, _, _, _, _) => {
                        h2416
                    }
                })?;
                nat_add(a2417, a2418)?
            };
            let v10: u64 = {
                let a2419 = v8;
                let a2420 = v9;
                let a2421 = 0u64;
                nat_quot(a2419, a2420, a2421)
            };
            let v11: u64 = {
                let a2422 = v8;
                let a2423 = v9;
                let a2424 = 0u64;
                nat_rem(a2422, a2423, a2424)
            };
            let a2425 = v11;
            let a2426 = 0u64;
            if nat_eq(a2425, a2426) {
                Ok(Adt9::C4(19u64, 4u64, f146(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f111(v0) {
                    let a2430 = {
                        let a2427 = v11;
                        let a2428 = 2u64;
                        let a2429 = 0u64;
                        nat_rem(a2427, a2428, a2429)
                    };
                    let a2431 = 1u64;
                    if nat_eq(a2430, a2431) {
                        Ok(Adt9::C4(6u64, 0u64, f146({
                            let a2440 = {
                                let a2433 = v10;
                                let a2434 = match v2.clone() {
                                    Adt3::C0(h2432, _, _, _, _, _, _, _) => {
                                        h2432
                                    }
                                };
                                nat_mul(a2433, a2434)?
                            };
                            let a2441 = {
                                let a2437 = {
                                    let a2435 = v11;
                                    let a2436 = 1u64;
                                    nat_sub(a2435, a2436)
                                };
                                let a2438 = 2u64;
                                let a2439 = 0u64;
                                nat_quot(a2437, a2438, a2439)
                            };
                            nat_add(a2440, a2441)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f146({
                            let a2450 = {
                                let a2443 = v5;
                                let a2444 = match v2.clone() {
                                    Adt3::C0(h2442, _, _, _, _, _, _, _) => {
                                        h2442
                                    }
                                };
                                nat_mul(a2443, a2444)?
                            };
                            let a2451 = {
                                let a2447 = {
                                    let a2445 = v11;
                                    let a2446 = 1u64;
                                    nat_sub(a2445, a2446)
                                };
                                let a2448 = 2u64;
                                let a2449 = 0u64;
                                nat_quot(a2447, a2448, a2449)
                            };
                            nat_add(a2450, a2451)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f146({
                            let a2460 = {
                                let a2453 = v5;
                                let a2454 = match v2 {
                                    Adt3::C0(h2452, _, _, _, _, _, _, _) => {
                                        h2452
                                    }
                                };
                                nat_mul(a2453, a2454)?
                            };
                            let a2461 = {
                                let a2457 = {
                                    let a2455 = v11;
                                    let a2456 = 1u64;
                                    nat_sub(a2455, a2456)
                                };
                                let a2458 = 2u64;
                                let a2459 = 0u64;
                                nat_quot(a2457, a2458, a2459)
                            };
                            nat_add(a2460, a2461)?
                        }, v3)?))
                    }
                } else {
                    f165(v0, v10, {
                        let a2462 = v11;
                        let a2463 = 1u64;
                        nat_sub(a2462, a2463)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2464 = v1;
    let a2465 = f166(v2.clone(), v4.clone())?;
    if nat_lt(a2464, a2465) {
        let a2468 = v1;
        let a2469 = {
            let a2466 = 2u64;
            let a2467 = f134(v4.clone())?;
            nat_mul(a2466, a2467)?
        };
        if nat_lt(a2468, a2469) {
            let a2473 = {
                let a2470 = v1;
                let a2471 = 2u64;
                let a2472 = 0u64;
                nat_rem(a2470, a2471, a2472)
            };
            let a2474 = 0u64;
            if nat_eq(a2473, a2474) {
                f167({
                    let a2475 = v1;
                    let a2476 = 2u64;
                    let a2477 = 0u64;
                    nat_quot(a2475, a2476, a2477)
                }, v4)
            } else {
                if f168(match v2 {
                    Adt3::C0(_, h2478, _, _, _, _, _, _) => {
                        h2478
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2481 = v1;
                let a2482 = {
                    let a2479 = 2u64;
                    let a2480 = f134(v4.clone())?;
                    nat_mul(a2479, a2480)?
                };
                nat_sub(a2481, a2482)
            };
            let v7: u64 = {
                let a2486 = v6;
                let a2487 = {
                    let a2484 = 1u64;
                    let a2485 = match v2.clone() {
                        Adt3::C0(h2483, _, _, _, _, _, _, _) => {
                            h2483
                        }
                    };
                    nat_add(a2484, a2485)?
                };
                let a2488 = 0u64;
                nat_quot(a2486, a2487, a2488)
            };
            let v8: u64 = {
                let a2492 = v6;
                let a2493 = {
                    let a2490 = 1u64;
                    let a2491 = match v2.clone() {
                        Adt3::C0(h2489, _, _, _, _, _, _, _) => {
                            h2489
                        }
                    };
                    nat_add(a2490, a2491)?
                };
                let a2494 = 0u64;
                nat_rem(a2492, a2493, a2494)
            };
            let a2495 = v8;
            let a2496 = 0u64;
            if nat_eq(a2495, a2496) {
                f167(v7, v4)
            } else {
                f169(v0, v7, {
                    let a2497 = v8;
                    let a2498 = 1u64;
                    nat_sub(a2497, a2498)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2501 = match v4.clone() {
            Adt3::C0(h2499, _, _, _, _, _, _, _) => {
                h2499
            }
        };
        let a2502 = match v4.clone() {
            Adt3::C0(_, h2500, _, _, _, _, _, _) => {
                h2500
            }
        };
        nat_mul(a2501, a2502)?
    };
    let v7: u64 = {
        let a2503 = 2u64;
        let a2504 = v6;
        nat_mul(a2503, a2504)?
    };
    let v8: u64 = f170(match v3.clone() {
        Adt3::C0(_, h2505, _, _, _, _, _, _) => {
            h2505
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2506, _, _, _, _, _) => {
            h2506
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2507, _, _, _, _) => {
            h2507
        }
    });
    let a2508 = v0;
    let a2509 = v7;
    if nat_lt(a2508, a2509) {
        let a2513 = {
            let a2510 = v0;
            let a2511 = 2u64;
            let a2512 = 0u64;
            nat_rem(a2510, a2511, a2512)
        };
        let a2514 = 0u64;
        if nat_eq(a2513, a2514) {
            Ok(Adt9::C4(19u64, 5u64, f146({
                let a2515 = v0;
                let a2516 = 2u64;
                let a2517 = 0u64;
                nat_quot(a2515, a2516, a2517)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f171(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2518 = v0;
            let a2519 = v7;
            nat_sub(a2518, a2519)
        };
        let v10: u64 = {
            let a2523 = v9;
            let a2524 = {
                let a2521 = 1u64;
                let a2522 = match v1.clone() {
                    Adt3::C0(h2520, _, _, _, _, _, _, _) => {
                        h2520
                    }
                };
                nat_add(a2521, a2522)?
            };
            let a2525 = 0u64;
            nat_quot(a2523, a2524, a2525)
        };
        let v11: u64 = {
            let a2529 = v9;
            let a2530 = {
                let a2527 = 1u64;
                let a2528 = match v1.clone() {
                    Adt3::C0(h2526, _, _, _, _, _, _, _) => {
                        h2526
                    }
                };
                nat_add(a2527, a2528)?
            };
            let a2531 = 0u64;
            nat_rem(a2529, a2530, a2531)
        };
        let a2532 = v11;
        let a2533 = 0u64;
        if nat_eq(a2532, a2533) {
            Ok(Adt9::C4(19u64, 5u64, f146({
                let a2534 = v10;
                let a2535 = v6;
                let a2536 = 0u64;
                nat_rem(a2534, a2535, a2536)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f171(v5, v8) {
                let v12: u64 = {
                    let a2542 = {
                        let a2538 = v10;
                        let a2539 = match v1.clone() {
                            Adt3::C0(h2537, _, _, _, _, _, _, _) => {
                                h2537
                            }
                        };
                        nat_mul(a2538, a2539)?
                    };
                    let a2543 = {
                        let a2540 = v11;
                        let a2541 = 1u64;
                        nat_sub(a2540, a2541)
                    };
                    nat_add(a2542, a2543)?
                };
                let a2544 = v5;
                let a2545 = 4294967295u64;
                if nat_eq(a2544, a2545) {
                    Ok(Adt9::C1(false, f146(v12, v1)?, f146(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f146(v12, v1)?, 4u64, f146({
                        let a2551 = {
                            let a2547 = v5;
                            let a2548 = match v3.clone() {
                                Adt3::C0(h2546, _, _, _, _, _, _, _) => {
                                    h2546
                                }
                            };
                            nat_mul(a2547, a2548)?
                        };
                        let a2552 = {
                            let a2549 = v11;
                            let a2550 = 1u64;
                            nat_sub(a2549, a2550)
                        };
                        nat_add(a2551, a2552)?
                    }, v3)?, 3u64, f146(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f116(v3.clone())?;
    let v6: u64 = {
        let a2553 = v0;
        let a2554 = v5;
        let a2555 = 0u64;
        nat_quot(a2553, a2554, a2555)
    };
    let v7: u64 = {
        let a2556 = v0;
        let a2557 = v5;
        let a2558 = 0u64;
        nat_rem(a2556, a2557, a2558)
    };
    let v8: u64 = f172(v3.clone())?;
    let a2559 = v7;
    let a2560 = 0u64;
    if nat_eq(a2559, a2560) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2561, _) => {
                h2561
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2565 = v5;
        let a2566 = {
            let a2563 = v7;
            let a2564 = if match v3.clone() {
                Adt7::C0(_, _, _, h2562, _) => {
                    h2562
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2563, a2564)?
        };
        if nat_le(a2565, a2566) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2567, _) => {
                    h2567
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f146(v6, v2)?))
            } else {
                let a2570 = {
                    let a2568 = v7;
                    let a2569 = 2u64;
                    nat_add(a2568, a2569)?
                };
                let a2571 = v5;
                if nat_eq(a2570, a2571) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2572, _, _) => {
                            h2572
                        }
                    } {
                        let a2579 = {
                            let m2575 = match v3.clone() {
                                Adt7::C0(h2574, _, _, _, _) => {
                                    h2574
                                }
                            };
                            match m2575 {
                                Adt6::C0(h2573, _, _, _) => {
                                    h2573
                                }
                            }
                        };
                        let a2580 = {
                            let m2578 = match v3 {
                                Adt7::C0(_, h2577, _, _, _) => {
                                    h2577
                                }
                            };
                            match m2578 {
                                Adt6::C0(h2576, _, _, _) => {
                                    h2576
                                }
                            }
                        };
                        nat_mul(a2579, a2580)?
                    } else {
                        f173(match v3 {
                            Adt7::C0(h2581, _, _, _, _) => {
                                h2581
                            }
                        }, match v1 {
                            Adt3::C0(h2582, _, _, _, _, _, _, _) => {
                                h2582
                            }
                        }, f72(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2583 = v9;
                        let a2584 = 0u64;
                        if nat_eq(a2583, a2584) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f146(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2587 = {
                    let a2585 = v7;
                    let a2586 = 1u64;
                    nat_sub(a2585, a2586)
                };
                let a2588 = v8;
                let a2589 = 0u64;
                nat_rem(a2587, a2588, a2589)
            };
            let m2595 = f174(v3.clone(), v1, v2, v6, {
                let a2592 = {
                    let a2590 = v7;
                    let a2591 = 1u64;
                    nat_sub(a2590, a2591)
                };
                let a2593 = v8;
                let a2594 = 0u64;
                nat_quot(a2592, a2593, a2594)
            })?;
            match m2595 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2597 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2596) => {
                            h2596
                        }
                    };
                    let a2598 = 1u64;
                    if if nat_eq(a2597, a2598) {
                        let a2599 = v10;
                        let a2600 = 0u64;
                        nat_eq(a2599, a2600)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2601, _) => {
                                h2601
                            }
                        } {
                            let a2604 = {
                                let a2602 = v10;
                                let a2603 = 1u64;
                                nat_add(a2602, a2603)?
                            };
                            let a2605 = v8;
                            if nat_eq(a2604, a2605) {
                                let a2606 = v4;
                                let a2607 = 2u64;
                                if nat_eq(a2606, a2607) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2609 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2608) => {
                                                h2608
                                            }
                                        };
                                        let a2610 = 1u64;
                                        if nat_eq(a2609, a2610) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2612 = match v3 {
                                            Adt7::C0(_, _, _, _, h2611) => {
                                                h2611
                                            }
                                        };
                                        let a2613 = 1u64;
                                        if nat_eq(a2612, a2613) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2615 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2614) => {
                                            h2614
                                        }
                                    };
                                    let a2616 = 1u64;
                                    if nat_eq(a2615, a2616) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2618 = match v3 {
                                        Adt7::C0(_, _, _, _, h2617) => {
                                            h2617
                                        }
                                    };
                                    let a2619 = 1u64;
                                    if nat_eq(a2618, a2619) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2621 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2620) => {
                                        h2620
                                    }
                                };
                                let a2622 = 1u64;
                                if nat_eq(a2621, a2622) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2624 = match v3 {
                                    Adt7::C0(_, _, _, _, h2623) => {
                                        h2623
                                    }
                                };
                                let a2625 = 1u64;
                                if nat_eq(a2624, a2625) {
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

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f114(match v3.clone() {
        Adt3::C0(h2626, _, _, _, _, _, _, _) => {
            h2626
        }
    })?;
    let v5: u64 = {
        let a2627 = v0;
        let a2628 = v4;
        let a2629 = 0u64;
        nat_rem(a2627, a2628, a2629)
    };
    let v6: u64 = {
        let a2630 = v0;
        let a2631 = v4;
        let a2632 = 0u64;
        nat_quot(a2630, a2631, a2632)
    };
    let a2633 = v5;
    let a2634 = 0u64;
    if nat_eq(a2633, a2634) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2637 = {
            let a2635 = v5;
            let a2636 = 1u64;
            nat_add(a2635, a2636)?
        };
        let a2638 = v4;
        if nat_eq(a2637, a2638) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f146(v6, v2)?))
        } else {
            let a2642 = {
                let a2639 = v5;
                let a2640 = 2u64;
                let a2641 = 0u64;
                nat_rem(a2639, a2640, a2641)
            };
            let a2643 = 0u64;
            if nat_eq(a2642, a2643) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2646 = {
                        let a2644 = v5;
                        let a2645 = 1u64;
                        nat_sub(a2644, a2645)
                    };
                    let a2647 = 2u64;
                    let a2648 = 0u64;
                    nat_quot(a2646, a2647, a2648)
                };
                let v8: u64 = f72(v6, v2.clone(), 0u64);
                let v9: u64 = f72(v6, v2.clone(), 1u64);
                let v10: u64 = f72(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f71(v1, {
                    let a2649 = v9;
                    let a2650 = v7;
                    nat_add(a2649, a2650)?
                }, v8, v10, 0u64)?, 4u64, f71(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f87(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f113(v3.clone())?;
    let v5: u64 = {
        let a2651 = v0;
        let a2652 = v4;
        let a2653 = 0u64;
        nat_rem(a2651, a2652, a2653)
    };
    let v6: u64 = {
        let a2654 = v0;
        let a2655 = v4;
        let a2656 = 0u64;
        nat_quot(a2654, a2655, a2656)
    };
    let a2657 = v5;
    let a2658 = 0u64;
    if nat_eq(a2657, a2658) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2661 = {
            let a2659 = v5;
            let a2660 = 1u64;
            nat_add(a2659, a2660)?
        };
        let a2662 = v4;
        if nat_eq(a2661, a2662) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f146(v6, v2)?))
        } else {
            let a2666 = {
                let a2663 = v5;
                let a2664 = 2u64;
                let a2665 = 0u64;
                nat_rem(a2663, a2664, a2665)
            };
            let a2667 = 0u64;
            if nat_eq(a2666, a2667) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2669 = f72(v6, v2.clone(), 0u64);
                    let a2670 = match v3.clone() {
                        Adt6::C0(_, _, h2668, _) => {
                            h2668
                        }
                    };
                    nat_add(a2669, a2670)?
                };
                let v8: u64 = {
                    let a2678 = {
                        let a2675 = {
                            let a2672 = v7;
                            let a2673 = match v3.clone() {
                                Adt6::C0(_, h2671, _, _) => {
                                    h2671
                                }
                            };
                            nat_add(a2672, a2673)?
                        };
                        let a2676 = match v3.clone() {
                            Adt6::C0(h2674, _, _, _) => {
                                h2674
                            }
                        };
                        nat_sub(a2675, a2676)
                    };
                    let a2679 = match v3.clone() {
                        Adt6::C0(_, h2677, _, _) => {
                            h2677
                        }
                    };
                    let a2680 = 0u64;
                    nat_quot(a2678, a2679, a2680)
                };
                let v9: u64 = {
                    let a2686 = v8;
                    let a2687 = {
                        let a2683 = {
                            let a2681 = v5;
                            let a2682 = 1u64;
                            nat_sub(a2681, a2682)
                        };
                        let a2684 = 2u64;
                        let a2685 = 0u64;
                        nat_quot(a2683, a2684, a2685)
                    };
                    nat_add(a2686, a2687)?
                };
                let v10: u64 = {
                    let a2689 = v9;
                    let a2690 = match v3.clone() {
                        Adt6::C0(_, h2688, _, _) => {
                            h2688
                        }
                    };
                    nat_mul(a2689, a2690)?
                };
                let v11: u64 = {
                    let a2691 = v7;
                    let a2692 = v10;
                    nat_sub(a2691, a2692)
                };
                let a2694 = v9;
                let a2695 = match v1.clone() {
                    Adt3::C0(_, h2693, _, _, _, _, _, _) => {
                        h2693
                    }
                };
                if if if nat_lt(a2694, a2695) {
                    let a2696 = v10;
                    let a2697 = v7;
                    nat_le(a2696, a2697)
                } else {
                    false
                } {
                    let a2699 = v11;
                    let a2700 = match v3.clone() {
                        Adt6::C0(h2698, _, _, _) => {
                            h2698
                        }
                    };
                    nat_lt(a2699, a2700)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f71(v1, {
                        let a2704 = {
                            let a2702 = f72(v6, v2, 1u64);
                            let a2703 = match v3 {
                                Adt6::C0(h2701, _, _, _) => {
                                    h2701
                                }
                            };
                            nat_mul(a2702, a2703)?
                        };
                        let a2705 = v11;
                        nat_add(a2704, a2705)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f72(v0, v2.clone(), 0u64);
    let v5: u64 = f72(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2707 = v5;
        let a2708 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2706, _) => {
                h2706
            }
        };
        let a2709 = 0u64;
        nat_rem(a2707, a2708, a2709)
    };
    let v7: u64 = {
        let a2711 = v5;
        let a2712 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2710, _) => {
                h2710
            }
        };
        let a2713 = 0u64;
        nat_quot(a2711, a2712, a2713)
    };
    let v8: Option<u64> = f74(match v3.clone() {
        Adt5::C0(h2714, _, _, _, _, _, _, _) => {
            h2714
        }
    }, match v1.clone() {
        Adt3::C0(h2715, _, _, _, _, _, _, _) => {
            h2715
        }
    }, f72(v0, v2.clone(), 1u64), {
        let a2719 = v4;
        let a2720 = {
            let m2718 = match v3.clone() {
                Adt5::C0(h2717, _, _, _, _, _, _, _) => {
                    h2717
                }
            };
            match m2718 {
                Adt6::C0(h2716, _, _, _) => {
                    h2716
                }
            }
        };
        let a2721 = 0u64;
        nat_rem(a2719, a2720, a2721)
    })?;
    let v9: u64 = {
        let a2725 = v4;
        let a2726 = {
            let m2724 = match v3.clone() {
                Adt5::C0(h2723, _, _, _, _, _, _, _) => {
                    h2723
                }
            };
            match m2724 {
                Adt6::C0(h2722, _, _, _) => {
                    h2722
                }
            }
        };
        let a2727 = 0u64;
        nat_quot(a2725, a2726, a2727)
    };
    let v10: Option<u64> = f74(match v3.clone() {
        Adt5::C0(_, h2728, _, _, _, _, _, _) => {
            h2728
        }
    }, match v1.clone() {
        Adt3::C0(_, h2729, _, _, _, _, _, _) => {
            h2729
        }
    }, f72(v0, v2.clone(), 2u64), {
        let a2733 = v9;
        let a2734 = {
            let m2732 = match v3.clone() {
                Adt5::C0(_, h2731, _, _, _, _, _, _) => {
                    h2731
                }
            };
            match m2732 {
                Adt6::C0(h2730, _, _, _) => {
                    h2730
                }
            }
        };
        let a2735 = 0u64;
        nat_rem(a2733, a2734, a2735)
    })?;
    let v11: u64 = {
        let a2739 = v9;
        let a2740 = {
            let m2738 = match v3.clone() {
                Adt5::C0(_, h2737, _, _, _, _, _, _) => {
                    h2737
                }
            };
            match m2738 {
                Adt6::C0(h2736, _, _, _) => {
                    h2736
                }
            }
        };
        let a2741 = 0u64;
        nat_quot(a2739, a2740, a2741)
    };
    let v12: Option<u64> = f74(match v3.clone() {
        Adt5::C0(_, _, h2742, _, _, _, _, _) => {
            h2742
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2743, _, _, _, _, _) => {
            h2743
        }
    }, v6, {
        let a2747 = v11;
        let a2748 = {
            let m2746 = match v3.clone() {
                Adt5::C0(_, _, h2745, _, _, _, _, _) => {
                    h2745
                }
            };
            match m2746 {
                Adt6::C0(h2744, _, _, _) => {
                    h2744
                }
            }
        };
        let a2749 = 0u64;
        nat_rem(a2747, a2748, a2749)
    })?;
    let v13: u64 = {
        let a2753 = v11;
        let a2754 = {
            let m2752 = match v3.clone() {
                Adt5::C0(_, _, h2751, _, _, _, _, _) => {
                    h2751
                }
            };
            match m2752 {
                Adt6::C0(h2750, _, _, _) => {
                    h2750
                }
            }
        };
        let a2755 = 0u64;
        nat_quot(a2753, a2754, a2755)
    };
    let v17: Adt12 = {
        let m2756 = v8;
        match m2756 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2757 = v10;
                match m2757 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2758 = v12;
                        match m2758 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2767 = {
                                        let a2762 = f71(v1, v14, v15, v16, 0u64)?;
                                        let a2763 = {
                                            let a2760 = v7;
                                            let a2761 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2759, _, _, _) => {
                                                    h2759
                                                }
                                            };
                                            nat_mul(a2760, a2761)?
                                        };
                                        nat_add(a2762, a2763)?
                                    };
                                    let a2768 = {
                                        let a2765 = v13;
                                        let a2766 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2764, _, _, _, _) => {
                                                h2764
                                            }
                                        };
                                        nat_mul(a2765, a2766)?
                                    };
                                    nat_add(a2767, a2768)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2769, _, _) => {
            h2769
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2770) => {
            h2770
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f146(v0, v2)?), {
        let a2772 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2771, _, _) => {
                h2771
            }
        };
        let a2773 = 0u64;
        if nat_eq(a2772, a2773) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2775 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2774) => {
                h2774
            }
        };
        let a2776 = 0u64;
        if nat_eq(a2775, a2776) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f89(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2789 = f175(v0, v1, match v2.clone() {
        Adt3::C0(h2777, _, _, _, _, _, _, _) => {
            h2777
        }
    }, match v2.clone() {
        Adt3::C0(_, h2778, _, _, _, _, _, _) => {
            h2778
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2779, _, _, _, _, _) => {
            h2779
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2780, _, _, _, _) => {
            h2780
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2781, _, _, _) => {
            h2781
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2782, _, _) => {
            h2782
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2783, _) => {
            h2783
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2784) => {
            h2784
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2785, _, _, _) => {
            h2785
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2786, _, _) => {
            h2786
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2787, _) => {
            h2787
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2788) => {
            h2788
        }
    }, v4)?;
    match m2789 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2792 = v6;
        let a2793 = {
            let a2790 = 3u64;
            let a2791 = v5;
            nat_add(a2790, a2791)?
        };
        nat_sub(a2792, a2793)
    };
    let a2795 = v0;
    let a2796 = f36(v3, match v1.clone() {
        Adt3::C0(h2794, _, _, _, _, _, _, _) => {
            h2794
        }
    }, v5, v8)?;
    if nat_lt(a2795, a2796) {
        let v9: u64 = f104(match v1.clone() {
            Adt3::C0(h2797, _, _, _, _, _, _, _) => {
                h2797
            }
        }, v5, v8)?;
        let v10: u64 = f105(v3, match v1.clone() {
            Adt3::C0(h2798, _, _, _, _, _, _, _) => {
                h2798
            }
        }, v5)?;
        let a2799 = v0;
        let a2800 = v9;
        if nat_lt(a2799, a2800) {
            f176(v0, v2, f30(v3), {
                let a2801 = 3u64;
                let a2802 = v5;
                nat_add(a2801, a2802)?
            }, v7)
        } else {
            let a2805 = v0;
            let a2806 = {
                let a2803 = v9;
                let a2804 = v10;
                nat_add(a2803, a2804)?
            };
            if nat_lt(a2805, a2806) {
                let v11: u64 = {
                    let a2807 = v0;
                    let a2808 = v9;
                    nat_sub(a2807, a2808)
                };
                let v12: u64 = f177(v3, match v1.clone() {
                    Adt3::C0(h2809, _, _, _, _, _, _, _) => {
                        h2809
                    }
                })?;
                f178({
                    let a2810 = v11;
                    let a2811 = v12;
                    let a2812 = 0u64;
                    nat_rem(a2810, a2811, a2812)
                }, {
                    let a2813 = v11;
                    let a2814 = v12;
                    let a2815 = 0u64;
                    nat_quot(a2813, a2814, a2815)
                }, v1, v3, v4, v7)
            } else {
                f179({
                    let a2818 = v0;
                    let a2819 = {
                        let a2816 = v9;
                        let a2817 = v10;
                        nat_add(a2816, a2817)?
                    };
                    nat_sub(a2818, a2819)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2820 = v0;
    let a2821 = f19(v1)?;
    if nat_lt(a2820, a2821) {
        let a2822 = v0;
        let a2823 = 4u64;
        if nat_lt(a2822, a2823) {
            let a2824 = v0;
            let a2825 = 0u64;
            if nat_eq(a2824, a2825) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2826 = v0;
                let a2827 = 1u64;
                if nat_eq(a2826, a2827) {
                    let a2828 = v5;
                    let a2829 = 0u64;
                    if nat_eq(a2828, a2829) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2830 = v0;
                    let a2831 = 2u64;
                    if nat_eq(a2830, a2831) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2832 = v0;
            let a2833 = 4u64;
            if if nat_eq(a2832, a2833) {
                let a2834 = f180(v5)?;
                let a2835 = v1;
                !nat_eq(a2834, a2835)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2838 = {
                        let a2836 = v0;
                        let a2837 = 4u64;
                        nat_sub(a2836, a2837)
                    };
                    let a2839 = 3u64;
                    let a2840 = 0u64;
                    nat_quot(a2838, a2839, a2840)
                };
                let v7: u64 = {
                    let a2843 = {
                        let a2841 = v0;
                        let a2842 = 4u64;
                        nat_sub(a2841, a2842)
                    };
                    let a2844 = 3u64;
                    let a2845 = 0u64;
                    nat_rem(a2843, a2844, a2845)
                };
                let a2846 = v7;
                let a2847 = 0u64;
                if nat_eq(a2846, a2847) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2848 = v7;
                    let a2849 = 1u64;
                    if nat_eq(a2848, a2849) {
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

pub fn f92(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2850 = v1;
        let a2851 = 5u64;
        nat_eq(a2850, a2851)
    } else {
        false
    } {
        let a2852 = v2;
        let a2853 = 16u64;
        nat_eq(a2852, a2853)
    } else {
        false
    } {
        let a2854 = v3;
        let a2855 = 0u64;
        nat_eq(a2854, a2855)
    } else {
        false
    } {
        let a2856 = v4;
        let a2857 = 0u64;
        nat_eq(a2856, a2857)
    } else {
        false
    }
}

pub fn f93(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2858 = v1;
    let a2859 = 0u64;
    if nat_eq(a2858, a2859) {
        if v0 {
            let a2860 = v3;
            let a2861 = 0u64;
            if if if nat_eq(a2860, a2861) {
                let a2862 = v4;
                let a2863 = 43u64;
                nat_eq(a2862, a2863)
            } else {
                false
            } {
                true
            } else {
                let a2864 = v3;
                let a2865 = 43u64;
                if nat_eq(a2864, a2865) {
                    let a2866 = v4;
                    let a2867 = 0u64;
                    nat_eq(a2866, a2867)
                } else {
                    false
                }
            } {
                f163(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2868 = v1;
        let a2869 = 1u64;
        if nat_eq(a2868, a2869) {
            Adt11::C1(v2)
        } else {
            let a2870 = v1;
            let a2871 = 2u64;
            if nat_eq(a2870, a2871) {
                Adt11::C5(v2)
            } else {
                let a2872 = v1;
                let a2873 = 3u64;
                if nat_eq(a2872, a2873) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2874 = v1;
                    let a2875 = 4u64;
                    if nat_eq(a2874, a2875) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f92(v0, v1, v2, v3, v4) {
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

pub fn f94(v0: u64, v1: u64) -> Adt12 {
    let a2876 = v0;
    let a2877 = 0u64;
    if nat_eq(a2876, a2877) {
        Adt12::C0(0u64, v1)
    } else {
        let a2878 = v0;
        let a2879 = 1u64;
        if nat_eq(a2878, a2879) {
            Adt12::C1(v1)
        } else {
            let a2880 = v0;
            let a2881 = 2u64;
            if nat_eq(a2880, a2881) {
                Adt12::C2(v1)
            } else {
                let a2882 = v0;
                let a2883 = 3u64;
                if nat_eq(a2882, a2883) {
                    Adt12::C3(v1)
                } else {
                    let a2884 = v0;
                    let a2885 = 13u64;
                    if nat_lt(a2884, a2885) {
                        Adt12::C0({
                            let a2886 = v0;
                            let a2887 = 3u64;
                            nat_sub(a2886, a2887)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f95(v0: u64, v1: u64) -> Adt13 {
    let a2888 = v0;
    let a2889 = 1u64;
    if nat_eq(a2888, a2889) {
        Adt13::C0(v1)
    } else {
        let a2890 = v0;
        let a2891 = 3u64;
        if if nat_eq(a2890, a2891) {
            true
        } else {
            let a2892 = v0;
            let a2893 = 6u64;
            nat_eq(a2892, a2893)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f96(v0: Adt6, v1: u64) -> R<u64> {
    let a2895 = 0u64;
    let a2896 = match v0.clone() {
        Adt6::C0(h2894, _, _, _) => {
            h2894
        }
    };
    if if if if if nat_lt(a2895, a2896) {
        let a2898 = 0u64;
        let a2899 = match v0.clone() {
            Adt6::C0(_, h2897, _, _) => {
                h2897
            }
        };
        nat_lt(a2898, a2899)
    } else {
        false
    } {
        let a2901 = 0u64;
        let a2902 = match v0.clone() {
            Adt6::C0(_, _, _, h2900) => {
                h2900
            }
        };
        nat_lt(a2901, a2902)
    } else {
        false
    } {
        let a2904 = match v0.clone() {
            Adt6::C0(_, h2903, _, _) => {
                h2903
            }
        };
        let a2905 = 2147483648u64;
        nat_lt(a2904, a2905)
    } else {
        false
    } {
        let a2907 = match v0.clone() {
            Adt6::C0(_, _, _, h2906) => {
                h2906
            }
        };
        let a2908 = 2147483648u64;
        nat_lt(a2907, a2908)
    } else {
        false
    } {
        let v2: u64 = {
            let a2912 = v1;
            let a2913 = {
                let a2910 = 2u64;
                let a2911 = f140(match v0.clone() {
                    Adt6::C0(_, _, h2909, _) => {
                        h2909
                    }
                });
                nat_mul(a2910, a2911)?
            };
            nat_add(a2912, a2913)?
        };
        let v3: u64 = {
            let a2925 = {
                let a2920 = 1u64;
                let a2921 = {
                    let a2918 = match v0.clone() {
                        Adt6::C0(_, _, _, h2914) => {
                            h2914
                        }
                    };
                    let a2919 = {
                        let a2916 = match v0.clone() {
                            Adt6::C0(h2915, _, _, _) => {
                                h2915
                            }
                        };
                        let a2917 = 1u64;
                        nat_sub(a2916, a2917)
                    };
                    nat_mul(a2918, a2919)?
                };
                nat_add(a2920, a2921)?
            };
            let a2926 = {
                let a2923 = 2u64;
                let a2924 = f139(match v0.clone() {
                    Adt6::C0(_, _, h2922, _) => {
                        h2922
                    }
                });
                nat_mul(a2923, a2924)?
            };
            nat_add(a2925, a2926)?
        };
        let a2927 = v3;
        let a2928 = v2;
        if nat_le(a2927, a2928) {
            let a2935 = 1u64;
            let a2936 = {
                let a2932 = {
                    let a2929 = v2;
                    let a2930 = v3;
                    nat_sub(a2929, a2930)
                };
                let a2933 = match v0 {
                    Adt6::C0(_, h2931, _, _) => {
                        h2931
                    }
                };
                let a2934 = 0u64;
                nat_quot(a2932, a2933, a2934)
            };
            nat_add(a2935, a2936)
        } else {
            let a2940 = {
                let a2937 = v3;
                let a2938 = v2;
                nat_sub(a2937, a2938)
            };
            let a2941 = match v0 {
                Adt6::C0(_, h2939, _, _) => {
                    h2939
                }
            };
            if nat_lt(a2940, a2941) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f97(v0: u64) -> u64 {
    let v1: u64 = {
        let a2942 = v0;
        let a2943 = 4294967296u64;
        let a2944 = 0u64;
        nat_rem(a2942, a2943, a2944)
    };
    let a2945 = v1;
    let a2946 = 2147483648u64;
    if nat_lt(a2945, a2946) {
        v1
    } else {
        0u64
    }
}

pub fn f98(v0: u64) -> R<u64> {
    let a2949 = 2u64;
    let a2950 = {
        let a2947 = 2u64;
        let a2948 = v0;
        nat_mul(a2947, a2948)?
    };
    nat_add(a2949, a2950)
}

pub fn f99(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2952 = match v1.clone() {
        Adt14::C0(h2951, _, _, _, _, _, _, _, _, _, _) => {
            h2951
        }
    };
    let a2953 = 0u64;
    if if if if nat_eq(a2952, a2953) {
        let a2955 = match v2.clone() {
            Adt14::C0(h2954, _, _, _, _, _, _, _, _, _, _) => {
                h2954
            }
        };
        let a2956 = 0u64;
        nat_eq(a2955, a2956)
    } else {
        false
    } {
        f181(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f182(match v1.clone() {
            Adt14::C0(_, h2957, _, _, _, _, _, _, _, _, _) => {
                h2957
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2958, _, _, _, _, _, _, _, _) => {
                h2958
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2959, _, _, _, _, _, _, _) => {
                h2959
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2960, _, _, _, _, _, _) => {
                h2960
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f183(v1.clone(), v2.clone())
            } else {
                false
            } {
                f184(v1.clone())
            } else {
                false
            } {
                f184(v2.clone())
            } else {
                false
            } {
                if f185(v1.clone(), v2.clone())? {
                    true
                } else {
                    f186(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f182(match v1.clone() {
            Adt14::C0(_, h2961, _, _, _, _, _, _, _, _, _) => {
                h2961
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2962, _, _, _, _, _, _, _, _) => {
                h2962
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2963, _, _, _, _, _, _, _) => {
                h2963
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2964, _, _, _, _, _, _) => {
                h2964
            }
        }) {
            0u64
        } else {
            let a2970 = match v1.clone() {
                Adt14::C0(_, _, h2965, _, _, _, _, _, _, _, _) => {
                    h2965
                }
            };
            let a2971 = {
                let a2968 = match v1.clone() {
                    Adt14::C0(_, _, _, h2966, _, _, _, _, _, _, _) => {
                        h2966
                    }
                };
                let a2969 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2967, _, _, _, _, _, _) => {
                        h2967
                    }
                };
                nat_mul(a2968, a2969)?
            };
            nat_mul(a2970, a2971)?
        };
        let v4: u64 = {
            let a2972 = v3;
            let a2973 = 0u64;
            if if nat_eq(a2972, a2973) {
                true
            } else {
                f187(v2.clone())?
            } {
                0u64
            } else {
                let a2975 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2974, _) => {
                        h2974
                    }
                };
                let a2976 = 4u64;
                let a2977 = 0u64;
                nat_quot(a2975, a2976, a2977)
            }
        };
        Ok(Adt15::C1(f188(f189(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2978, _, _, _, _, _) => {
                h2978
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2979, _, _, _, _, _) => {
                h2979
            }
        }), match v1 {
            Adt14::C0(_, h2980, _, _, _, _, _, _, _, _, _) => {
                h2980
            }
        }, v3, v4)?, 4u64, 64u64, v3, f190(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f100(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f191(v3.clone());
    let a2982 = match v2.clone() {
        Adt14::C0(h2981, _, _, _, _, _, _, _, _, _, _) => {
            h2981
        }
    };
    let a2983 = 0u64;
    if if if if nat_eq(a2982, a2983) {
        let a2985 = match v3.clone() {
            Adt14::C0(h2984, _, _, _, _, _, _, _, _, _, _) => {
                h2984
            }
        };
        let a2986 = 0u64;
        nat_eq(a2985, a2986)
    } else {
        false
    } {
        f181(v2.clone(), v3)
    } else {
        false
    } {
        if f182(match v2.clone() {
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
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2990, _, _, _, _, _, _) => {
                h2990
            }
        }) {
            true
        } else {
            let a2992 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2991, _, _, _, _, _) => {
                    h2991
                }
            };
            let a2993 = 4u64;
            if if if if if nat_eq(a2992, a2993) {
                f192(match v2.clone() {
                    Adt14::C0(_, h2994, _, _, _, _, _, _, _, _, _) => {
                        h2994
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2995, _, _, _, _, _, _, _, _) => {
                        h2995
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2996, _, _, _, _, _, _, _) => {
                        h2996
                    }
                }, v0)?
            } else {
                false
            } {
                f184(v2.clone())
            } else {
                false
            } {
                f184(v4.clone())
            } else {
                false
            } {
                let a2998 = f193(v4.clone())?;
                let a2999 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2997, _) => {
                        h2997
                    }
                };
                nat_le(a2998, a2999)
            } else {
                false
            } {
                if f185(v2.clone(), v4.clone())? {
                    true
                } else {
                    f186(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f182(match v2.clone() {
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
            f187(v4.clone())?
        } {
            0u64
        } else {
            let a3005 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3004, _) => {
                    h3004
                }
            };
            let a3006 = 4u64;
            let a3007 = 0u64;
            nat_quot(a3005, a3006, a3007)
        };
        Ok(Adt15::C1(f194(match v2.clone() {
            Adt14::C0(_, h3008, _, _, _, _, _, _, _, _, _) => {
                h3008
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3009, _, _, _, _, _, _, _, _) => {
                h3009
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3010, _, _, _, _, _, _, _) => {
                h3010
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3011, _, _, _, _, _, _) => {
                h3011
            }
        }, v0, v5)?, 4u64, 65u64, v0, f190(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f101(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f191(v4.clone());
    if if if if if if if v1 {
        let a3013 = match v2.clone() {
            Adt14::C0(h3012, _, _, _, _, _, _, _, _, _, _) => {
                h3012
            }
        };
        let a3014 = 0u64;
        nat_eq(a3013, a3014)
    } else {
        false
    } {
        let a3016 = match v3.clone() {
            Adt14::C0(h3015, _, _, _, _, _, _, _, _, _, _) => {
                h3015
            }
        };
        let a3017 = 0u64;
        nat_eq(a3016, a3017)
    } else {
        false
    } {
        let a3019 = match v4.clone() {
            Adt14::C0(h3018, _, _, _, _, _, _, _, _, _, _) => {
                h3018
            }
        };
        let a3020 = 0u64;
        nat_eq(a3019, a3020)
    } else {
        false
    } {
        f181(v2.clone(), v3.clone())
    } else {
        false
    } {
        f181(v2.clone(), v4)
    } else {
        false
    } {
        if f182(match v2.clone() {
            Adt14::C0(_, h3021, _, _, _, _, _, _, _, _, _) => {
                h3021
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3022, _, _, _, _, _, _, _, _) => {
                h3022
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3023, _, _, _, _, _, _, _) => {
                h3023
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3024, _, _, _, _, _, _) => {
                h3024
            }
        }) {
            true
        } else {
            let a3026 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3025, _, _, _, _, _) => {
                    h3025
                }
            };
            let a3027 = 4u64;
            if if if if if if if nat_eq(a3026, a3027) {
                let a3029 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3028, _, _, _, _, _) => {
                        h3028
                    }
                };
                let a3030 = 4u64;
                nat_eq(a3029, a3030)
            } else {
                false
            } {
                f184(v2.clone())
            } else {
                false
            } {
                f184(v3.clone())
            } else {
                false
            } {
                f184(v5.clone())
            } else {
                false
            } {
                let a3032 = f193(v5.clone())?;
                let a3033 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3031, _) => {
                        h3031
                    }
                };
                nat_le(a3032, a3033)
            } else {
                false
            } {
                if f185(v2.clone(), v5.clone())? {
                    true
                } else {
                    f186(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f185(v3.clone(), v5.clone())? {
                    true
                } else {
                    f186(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f182(match v2.clone() {
            Adt14::C0(_, h3034, _, _, _, _, _, _, _, _, _) => {
                h3034
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3035, _, _, _, _, _, _, _, _) => {
                h3035
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3036, _, _, _, _, _, _, _) => {
                h3036
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3037, _, _, _, _, _, _) => {
                h3037
            }
        }) {
            0u64
        } else {
            let a3043 = match v2.clone() {
                Adt14::C0(_, _, h3038, _, _, _, _, _, _, _, _) => {
                    h3038
                }
            };
            let a3044 = {
                let a3041 = match v2.clone() {
                    Adt14::C0(_, _, _, h3039, _, _, _, _, _, _, _) => {
                        h3039
                    }
                };
                let a3042 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3040, _, _, _, _, _, _) => {
                        h3040
                    }
                };
                nat_mul(a3041, a3042)?
            };
            nat_mul(a3043, a3044)?
        };
        let v7: u64 = {
            let a3045 = v6;
            let a3046 = 0u64;
            if if nat_eq(a3045, a3046) {
                true
            } else {
                f187(v5.clone())?
            } {
                0u64
            } else {
                let a3048 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3047, _) => {
                        h3047
                    }
                };
                let a3049 = 4u64;
                let a3050 = 0u64;
                nat_quot(a3048, a3049, a3050)
            }
        };
        Ok(Adt15::C1(f195(match v2 {
            Adt14::C0(_, h3051, _, _, _, _, _, _, _, _, _) => {
                h3051
            }
        }, v6, v7)?, 4u64, 66u64, v6, f190(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f196(v0, v3, v6) {
        f196(v1, v4, v6)
    } else {
        false
    } {
        f196(v2, v5, v6)
    } else {
        false
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f197(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f197(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f197(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f197(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f197(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f197(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3052 = v2;
    let a3053 = 0u64;
    if nat_eq(a3052, a3053) {
        Ok(0u64)
    } else {
        let a3056 = 2u64;
        let a3057 = {
            let a3054 = v0;
            let a3055 = v1;
            nat_mul(a3054, a3055)?
        };
        nat_mul(a3056, a3057)
    }
}

pub fn f105(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3058 = v2;
    let a3059 = f177(v0, v1)?;
    nat_mul(a3058, a3059)
}

pub fn f106(v0: u64) -> u64 {
    let a3060 = v0;
    let a3061 = 30u64;
    if nat_eq(a3060, a3061) {
        3u64
    } else {
        1u64
    }
}

pub fn f107(v0: u64) -> R<u64> {
    let a3064 = 5u64;
    let a3065 = {
        let a3062 = 4u64;
        let a3063 = v0;
        nat_mul(a3062, a3063)?
    };
    nat_add(a3064, a3065)
}

pub fn f108(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a3071 = match v0.clone() {
            Adt2::C0(_, _, h3066, _, _, _, _, _, _, _, _) => {
                h3066
            }
        };
        let a3072 = {
            let a3069 = match v0.clone() {
                Adt2::C0(_, _, _, h3067, _, _, _, _, _, _, _) => {
                    h3067
                }
            };
            let a3070 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3068, _, _, _, _, _, _) => {
                    h3068
                }
            };
            nat_mul(a3069, a3070)?
        };
        nat_mul(a3071, a3072)?
    };
    let v2: u64 = f16(v0.clone(), 1u64)?;
    let a3074 = match v0.clone() {
        Adt2::C0(h3073, _, _, _, _, _, _, _, _, _, _) => {
            h3073
        }
    };
    let a3075 = 0u64;
    if if if if if if if nat_eq(a3074, a3075) {
        let a3077 = match v0.clone() {
            Adt2::C0(_, h3076, _, _, _, _, _, _, _, _, _) => {
                h3076
            }
        };
        let a3078 = 2147483647u64;
        nat_le(a3077, a3078)
    } else {
        false
    } {
        let a3079 = v1;
        let a3080 = 2147483647u64;
        nat_le(a3079, a3080)
    } else {
        false
    } {
        let a3082 = match v0.clone() {
            Adt2::C0(_, h3081, _, _, _, _, _, _, _, _, _) => {
                h3081
            }
        };
        let a3083 = 1u64;
        if nat_eq(a3082, a3083) {
            true
        } else {
            let a3085 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3084, _, _, _, _, _) => {
                    h3084
                }
            };
            let a3086 = 4u64;
            nat_eq(a3085, a3086)
        }
    } else {
        false
    } {
        let a3087 = v1;
        let a3088 = 1u64;
        if nat_le(a3087, a3088) {
            true
        } else {
            let a3093 = {
                let a3090 = match v0.clone() {
                    Adt2::C0(_, h3089, _, _, _, _, _, _, _, _, _) => {
                        h3089
                    }
                };
                let a3091 = 4u64;
                nat_mul(a3090, a3091)?
            };
            let a3094 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3092, _, _, _, _) => {
                    h3092
                }
            };
            nat_le(a3093, a3094)
        }
    } else {
        false
    } {
        let a3096 = match v0.clone() {
            Adt2::C0(_, _, _, h3095, _, _, _, _, _, _, _) => {
                h3095
            }
        };
        let a3097 = 1u64;
        if nat_eq(a3096, a3097) {
            true
        } else {
            let a3103 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h3098, _, _, _) => {
                    h3098
                }
            };
            let a3104 = {
                let a3101 = match v0.clone() {
                    Adt2::C0(_, _, h3099, _, _, _, _, _, _, _, _) => {
                        h3099
                    }
                };
                let a3102 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3100, _, _, _, _) => {
                        h3100
                    }
                };
                nat_mul(a3101, a3102)?
            };
            nat_eq(a3103, a3104)
        }
    } else {
        false
    } {
        let a3106 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3105, _, _, _, _, _, _) => {
                h3105
            }
        };
        let a3107 = 1u64;
        if nat_eq(a3106, a3107) {
            true
        } else {
            let a3116 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h3108, _, _) => {
                    h3108
                }
            };
            let a3117 = {
                let a3114 = match v0.clone() {
                    Adt2::C0(_, _, _, h3109, _, _, _, _, _, _, _) => {
                        h3109
                    }
                };
                let a3115 = {
                    let a3112 = match v0.clone() {
                        Adt2::C0(_, _, h3110, _, _, _, _, _, _, _, _) => {
                            h3110
                        }
                    };
                    let a3113 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3111, _, _, _, _) => {
                            h3111
                        }
                    };
                    nat_mul(a3112, a3113)?
                };
                nat_mul(a3114, a3115)?
            };
            nat_eq(a3116, a3117)
        }
    } else {
        false
    } {
        let a3118 = v2;
        let a3119 = 0u64;
        if nat_eq(a3118, a3119) {
            Ok(true)
        } else {
            let a3125 = {
                let a3122 = f146({
                    let a3120 = v2;
                    let a3121 = 1u64;
                    nat_sub(a3120, a3121)
                }, f20(v0.clone(), 1u64))?;
                let a3123 = 4u64;
                nat_add(a3122, a3123)?
            };
            let a3126 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3124, _) => {
                    h3124
                }
            };
            Ok(nat_le(a3125, a3126))
        }
    } else {
        Ok(false)
    }
}

pub fn f109(v0: Adt2, v1: u64) -> R<bool> {
    let a3128 = match v0.clone() {
        Adt2::C0(h3127, _, _, _, _, _, _, _, _, _, _) => {
            h3127
        }
    };
    let a3129 = 0u64;
    if if nat_eq(a3128, a3129) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a3130 = v1;
        let a3131 = 1u64;
        if nat_le(a3130, a3131) {
            Ok(true)
        } else {
            let a3136 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3132, _, _, _, _) => {
                    h3132
                }
            };
            let a3137 = {
                let a3134 = 4u64;
                let a3135 = match v0 {
                    Adt2::C0(_, h3133, _, _, _, _, _, _, _, _, _) => {
                        h3133
                    }
                };
                nat_mul(a3134, a3135)?
            };
            Ok(nat_eq(a3136, a3137))
        }
    } else {
        Ok(false)
    }
}

pub fn f110(v0: u64) -> R<u64> {
    let a3140 = 2u64;
    let a3141 = {
        let a3138 = 6u64;
        let a3139 = v0;
        nat_mul(a3138, a3139)?
    };
    nat_add(a3140, a3141)
}

pub fn f111(v0: u64) -> bool {
    let a3142 = v0;
    let a3143 = 0u64;
    if if nat_eq(a3142, a3143) {
        true
    } else {
        let a3144 = v0;
        let a3145 = 1u64;
        nat_eq(a3144, a3145)
    } {
        true
    } else {
        let a3146 = v0;
        let a3147 = 30u64;
        nat_eq(a3146, a3147)
    }
}

pub fn f112(v0: Adt2, v1: u64) -> R<bool> {
    let a3149 = match v0.clone() {
        Adt2::C0(_, _, _, h3148, _, _, _, _, _, _, _) => {
            h3148
        }
    };
    let a3150 = 1u64;
    if if if nat_eq(a3149, a3150) {
        let a3152 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3151, _, _, _, _, _, _) => {
                h3151
            }
        };
        let a3153 = 1u64;
        nat_eq(a3152, a3153)
    } else {
        false
    } {
        let a3155 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h3154, _, _, _, _, _) => {
                h3154
            }
        };
        let a3156 = v1;
        nat_eq(a3155, a3156)
    } else {
        false
    } {
        let a3158 = match v0.clone() {
            Adt2::C0(_, _, h3157, _, _, _, _, _, _, _, _) => {
                h3157
            }
        };
        let a3159 = 1u64;
        if nat_eq(a3158, a3159) {
            Ok(true)
        } else {
            let a3164 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3160, _, _, _, _) => {
                    h3160
                }
            };
            let a3165 = {
                let a3162 = match v0 {
                    Adt2::C0(_, h3161, _, _, _, _, _, _, _, _, _) => {
                        h3161
                    }
                };
                let a3163 = v1;
                nat_mul(a3162, a3163)?
            };
            Ok(nat_eq(a3164, a3165))
        }
    } else {
        Ok(false)
    }
}

pub fn f113(v0: Adt6) -> R<u64> {
    let a3178 = 2u64;
    let a3179 = {
        let a3176 = 2u64;
        let a3177 = {
            let a3173 = {
                let a3170 = match v0.clone() {
                    Adt6::C0(h3166, _, _, _) => {
                        h3166
                    }
                };
                let a3171 = {
                    let a3168 = match v0.clone() {
                        Adt6::C0(_, h3167, _, _) => {
                            h3167
                        }
                    };
                    let a3169 = 1u64;
                    nat_sub(a3168, a3169)
                };
                nat_add(a3170, a3171)?
            };
            let a3174 = match v0 {
                Adt6::C0(_, h3172, _, _) => {
                    h3172
                }
            };
            let a3175 = 0u64;
            nat_quot(a3173, a3174, a3175)
        };
        nat_mul(a3176, a3177)?
    };
    nat_add(a3178, a3179)
}

pub fn f114(v0: u64) -> R<u64> {
    let a3182 = 2u64;
    let a3183 = {
        let a3180 = v0;
        let a3181 = 2u64;
        nat_mul(a3180, a3181)?
    };
    nat_add(a3182, a3183)
}

pub fn f115(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3187 = {
            let a3184 = v2;
            let a3185 = 1u64;
            nat_sub(a3184, a3185)
        };
        let a3188 = match v0.clone() {
            Adt6::C0(_, h3186, _, _) => {
                h3186
            }
        };
        nat_mul(a3187, a3188)?
    };
    let a3190 = 0u64;
    let a3191 = match v0.clone() {
        Adt6::C0(h3189, _, _, _) => {
            h3189
        }
    };
    if if if if if if if if nat_lt(a3190, a3191) {
        let a3193 = match v0.clone() {
            Adt6::C0(h3192, _, _, _) => {
                h3192
            }
        };
        let a3194 = 2147483648u64;
        nat_lt(a3193, a3194)
    } else {
        false
    } {
        let a3196 = 0u64;
        let a3197 = match v0.clone() {
            Adt6::C0(_, h3195, _, _) => {
                h3195
            }
        };
        nat_lt(a3196, a3197)
    } else {
        false
    } {
        let a3199 = match v0.clone() {
            Adt6::C0(_, h3198, _, _) => {
                h3198
            }
        };
        let a3200 = 2147483648u64;
        nat_lt(a3199, a3200)
    } else {
        false
    } {
        let a3201 = v1;
        let a3202 = 2147483648u64;
        nat_lt(a3201, a3202)
    } else {
        false
    } {
        let a3203 = 0u64;
        let a3204 = v2;
        nat_lt(a3203, a3204)
    } else {
        false
    } {
        let a3205 = v3;
        let a3206 = 2147483648u64;
        nat_lt(a3205, a3206)
    } else {
        false
    } {
        let a3218 = {
            let a3213 = {
                let a3210 = v3;
                let a3211 = {
                    let a3208 = match v0.clone() {
                        Adt6::C0(h3207, _, _, _) => {
                            h3207
                        }
                    };
                    let a3209 = 1u64;
                    nat_sub(a3208, a3209)
                };
                nat_add(a3210, a3211)?
            };
            let a3214 = f139(match v0.clone() {
                Adt6::C0(_, _, h3212, _) => {
                    h3212
                }
            });
            nat_add(a3213, a3214)?
        };
        let a3219 = {
            let a3216 = 2147483647u64;
            let a3217 = f140(match v0.clone() {
                Adt6::C0(_, _, h3215, _) => {
                    h3215
                }
            });
            nat_add(a3216, a3217)?
        };
        nat_le(a3218, a3219)
    } else {
        false
    } {
        let a3221 = match v0 {
            Adt6::C0(_, _, h3220, _) => {
                h3220
            }
        };
        let a3222 = 2147483648u64;
        Ok(!nat_eq(a3221, a3222))
    } else {
        Ok(false)
    }
}

pub fn f116(v0: Adt7) -> R<u64> {
    let a3234 = if match v0.clone() {
        Adt7::C0(_, _, _, h3223, _) => {
            h3223
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3235 = {
        let a3232 = {
            let a3230 = {
                let m3226 = match v0.clone() {
                    Adt7::C0(h3225, _, _, _, _) => {
                        h3225
                    }
                };
                match m3226 {
                    Adt6::C0(h3224, _, _, _) => {
                        h3224
                    }
                }
            };
            let a3231 = {
                let m3229 = match v0.clone() {
                    Adt7::C0(_, h3228, _, _, _) => {
                        h3228
                    }
                };
                match m3229 {
                    Adt6::C0(h3227, _, _, _) => {
                        h3227
                    }
                }
            };
            nat_mul(a3230, a3231)?
        };
        let a3233 = f172(v0)?;
        nat_mul(a3232, a3233)?
    };
    nat_add(a3234, a3235)
}

pub fn f117(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f198(match v0.clone() {
        Adt2::C0(h3236, _, _, _, _, _, _, _, _, _, _) => {
            h3236
        }
    }, match v0.clone() {
        Adt2::C0(_, h3237, _, _, _, _, _, _, _, _, _) => {
            h3237
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3238, _, _, _, _, _, _, _, _) => {
            h3238
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3239, _, _, _, _, _, _, _) => {
            h3239
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3240, _, _, _, _, _, _) => {
            h3240
        }
    }, match v1.clone() {
        Adt2::C0(h3241, _, _, _, _, _, _, _, _, _, _) => {
            h3241
        }
    }, match v1.clone() {
        Adt2::C0(_, h3242, _, _, _, _, _, _, _, _, _) => {
            h3242
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3243, _, _, _, _, _, _, _, _) => {
            h3243
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3244, _, _, _, _, _, _, _) => {
            h3244
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3245, _, _, _, _, _, _) => {
            h3245
        }
    }, match v2.clone() {
        Adt2::C0(h3246, _, _, _, _, _, _, _, _, _, _) => {
            h3246
        }
    }, match v2.clone() {
        Adt2::C0(_, h3247, _, _, _, _, _, _, _, _, _) => {
            h3247
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3248, _, _, _, _, _, _, _, _) => {
            h3248
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3249, _, _, _, _, _, _, _) => {
            h3249
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3250, _, _, _, _, _, _) => {
            h3250
        }
    }) {
        let m3252 = f121(match v0.clone() {
            Adt2::C0(h3251, _, _, _, _, _, _, _, _, _, _) => {
                h3251
            }
        });
        match m3252 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3258 = {
                    let a3255 = match v0.clone() {
                        Adt2::C0(_, h3253, _, _, _, _, _, _, _, _, _) => {
                            h3253
                        }
                    };
                    let a3256 = match v3.clone() {
                        Adt18::C0(_, h3254, _) => {
                            h3254
                        }
                    };
                    let a3257 = 0u64;
                    nat_rem(a3255, a3256, a3257)
                };
                let a3259 = 0u64;
                if if nat_eq(a3258, a3259) {
                    if f199(match v0.clone() {
                        Adt2::C0(_, h3260, _, _, _, _, _, _, _, _, _) => {
                            h3260
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3261, _, _, _, _, _, _, _, _, _) => {
                            h3261
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3262, _, _, _, _, _, _, _, _) => {
                            h3262
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3263, _, _, _, _, _, _, _) => {
                            h3263
                        }
                    }) {
                        true
                    } else {
                        let a3265 = 4u64;
                        let a3266 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3264, _, _, _, _, _) => {
                                h3264
                            }
                        };
                        if if nat_le(a3265, a3266) {
                            let a3268 = match v3.clone() {
                                Adt18::C0(_, h3267, _) => {
                                    h3267
                                }
                            };
                            let a3269 = 1u64;
                            if nat_eq(a3268, a3269) {
                                true
                            } else {
                                let a3272 = match v3.clone() {
                                    Adt18::C0(_, _, h3270) => {
                                        h3270
                                    }
                                };
                                let a3273 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3271, _, _, _, _, _) => {
                                        h3271
                                    }
                                };
                                nat_le(a3272, a3273)
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
                        Adt18::C0(_, h3274, _) => {
                            h3274
                        }
                    });
                    Ok(Adt10::C4(if f199(match v0.clone() {
                        Adt2::C0(_, h3275, _, _, _, _, _, _, _, _, _) => {
                            h3275
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3276, _, _, _, _, _, _, _, _, _) => {
                            h3276
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3277, _, _, _, _, _, _, _, _) => {
                            h3277
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3278, _, _, _, _, _, _, _) => {
                            h3278
                        }
                    }) {
                        0u64
                    } else {
                        f166(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3279, _, _, _, _, _, _, _, _, _, _) => {
                            h3279
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

pub fn f118(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f119(match v0.clone() {
        Adt2::C0(h3280, _, _, _, _, _, _, _, _, _, _) => {
            h3280
        }
    }) {
        let a3286 = {
            let a3283 = match v0.clone() {
                Adt2::C0(_, h3281, _, _, _, _, _, _, _, _, _) => {
                    h3281
                }
            };
            let a3284 = f200(match v0.clone() {
                Adt2::C0(h3282, _, _, _, _, _, _, _, _, _, _) => {
                    h3282
                }
            });
            let a3285 = 1u64;
            nat_rem(a3283, a3284, a3285)
        };
        let a3287 = 0u64;
        nat_eq(a3286, a3287)
    } else {
        false
    } {
        let a3289 = match v1.clone() {
            Adt2::C0(h3288, _, _, _, _, _, _, _, _, _, _) => {
                h3288
            }
        };
        let a3290 = 26u64;
        nat_eq(a3289, a3290)
    } else {
        false
    } {
        let a3292 = match v2.clone() {
            Adt2::C0(h3291, _, _, _, _, _, _, _, _, _, _) => {
                h3291
            }
        };
        let a3293 = 0u64;
        nat_eq(a3292, a3293)
    } else {
        false
    } {
        let a3296 = match v0.clone() {
            Adt2::C0(_, h3294, _, _, _, _, _, _, _, _, _) => {
                h3294
            }
        };
        let a3297 = match v2.clone() {
            Adt2::C0(_, h3295, _, _, _, _, _, _, _, _, _) => {
                h3295
            }
        };
        nat_eq(a3296, a3297)
    } else {
        false
    } {
        let a3300 = match v1.clone() {
            Adt2::C0(_, h3298, _, _, _, _, _, _, _, _, _) => {
                h3298
            }
        };
        let a3301 = match v0.clone() {
            Adt2::C0(_, _, h3299, _, _, _, _, _, _, _, _) => {
                h3299
            }
        };
        nat_le(a3300, a3301)
    } else {
        false
    } {
        let a3303 = match v0.clone() {
            Adt2::C0(_, _, _, h3302, _, _, _, _, _, _, _) => {
                h3302
            }
        };
        let a3304 = 1u64;
        nat_eq(a3303, a3304)
    } else {
        false
    } {
        let a3306 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3305, _, _, _, _, _, _) => {
                h3305
            }
        };
        let a3307 = 1u64;
        nat_eq(a3306, a3307)
    } else {
        false
    } {
        let a3309 = match v2.clone() {
            Adt2::C0(_, _, _, h3308, _, _, _, _, _, _, _) => {
                h3308
            }
        };
        let a3310 = 1u64;
        nat_eq(a3309, a3310)
    } else {
        false
    } {
        let a3312 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3311, _, _, _, _, _, _) => {
                h3311
            }
        };
        let a3313 = 1u64;
        nat_eq(a3312, a3313)
    } else {
        false
    } {
        let a3315 = match v1.clone() {
            Adt2::C0(_, _, h3314, _, _, _, _, _, _, _, _) => {
                h3314
            }
        };
        let a3316 = 1u64;
        nat_eq(a3315, a3316)
    } else {
        false
    } {
        let a3318 = match v1.clone() {
            Adt2::C0(_, _, _, h3317, _, _, _, _, _, _, _) => {
                h3317
            }
        };
        let a3319 = 1u64;
        nat_eq(a3318, a3319)
    } else {
        false
    } {
        let a3321 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3320, _, _, _, _, _, _) => {
                h3320
            }
        };
        let a3322 = 1u64;
        nat_eq(a3321, a3322)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3323, _, _, _, _, _, _, _, _, _) => {
                h3323
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3324, _, _, _, _, _, _, _, _) => {
                h3324
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3325, _, _, _, _, _, _, _) => {
                h3325
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3326, _, _, _, _, _, _) => {
                h3326
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3328 = match v1.clone() {
                Adt2::C0(_, h3327, _, _, _, _, _, _, _, _, _) => {
                    h3327
                }
            };
            let a3329 = 0u64;
            if nat_eq(a3328, a3329) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3331 = match v0.clone() {
                    Adt2::C0(_, h3330, _, _, _, _, _, _, _, _, _) => {
                        h3330
                    }
                };
                let a3332 = 2147483647u64;
                if if if if if if nat_le(a3331, a3332) {
                    let a3334 = match v1.clone() {
                        Adt2::C0(_, h3333, _, _, _, _, _, _, _, _, _) => {
                            h3333
                        }
                    };
                    let a3335 = 2147483647u64;
                    nat_le(a3334, a3335)
                } else {
                    false
                } {
                    let a3337 = 4u64;
                    let a3338 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3336, _, _, _, _, _) => {
                            h3336
                        }
                    };
                    nat_le(a3337, a3338)
                } else {
                    false
                } {
                    let a3344 = {
                        let a3341 = match v2.clone() {
                            Adt2::C0(_, h3339, _, _, _, _, _, _, _, _, _) => {
                                h3339
                            }
                        };
                        let a3342 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3340, _, _, _, _, _) => {
                                h3340
                            }
                        };
                        nat_mul(a3341, a3342)?
                    };
                    let a3345 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3343, _, _, _, _) => {
                            h3343
                        }
                    };
                    nat_le(a3344, a3345)
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
                        let a3358 = {
                            let a3349 = {
                                let a3347 = 2u64;
                                let a3348 = match v1.clone() {
                                    Adt2::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                                        h3346
                                    }
                                };
                                nat_mul(a3347, a3348)?
                            };
                            let a3350 = f16(v2.clone(), 1u64)?;
                            nat_add(a3349, a3350)?
                        };
                        let a3359 = {
                            let a3356 = match v1.clone() {
                                Adt2::C0(_, h3351, _, _, _, _, _, _, _, _, _) => {
                                    h3351
                                }
                            };
                            let a3357 = {
                                let a3354 = 1u64;
                                let a3355 = f164(match v0.clone() {
                                    Adt2::C0(h3352, _, _, _, _, _, _, _, _, _, _) => {
                                        h3352
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3353, _, _, _, _, _, _, _, _, _) => {
                                        h3353
                                    }
                                })?;
                                nat_add(a3354, a3355)?
                            };
                            nat_mul(a3356, a3357)?
                        };
                        nat_add(a3358, a3359)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3360, _, _, _, _, _, _, _, _, _) => {
                            h3360
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3361, _, _, _, _, _, _, _, _, _) => {
                            h3361
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3362, _, _, _, _, _) => {
                            h3362
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3363, _, _, _, _) => {
                            h3363
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3364, _, _, _) => {
                            h3364
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3365, _, _) => {
                            h3365
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3366, _, _, _, _, _, _, _, _, _, _) => {
                            h3366
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

pub fn f119(v0: u64) -> bool {
    let a3367 = v0;
    let a3368 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3367, a3368) {
        true
    } else {
        let a3369 = v0;
        let a3370 = 1u64;
        nat_eq(a3369, a3370)
    } {
        true
    } else {
        let a3371 = v0;
        let a3372 = 2u64;
        nat_eq(a3371, a3372)
    } {
        true
    } else {
        let a3373 = v0;
        let a3374 = 3u64;
        nat_eq(a3373, a3374)
    } {
        true
    } else {
        let a3375 = v0;
        let a3376 = 6u64;
        nat_eq(a3375, a3376)
    } {
        true
    } else {
        let a3377 = v0;
        let a3378 = 7u64;
        nat_eq(a3377, a3378)
    } {
        true
    } else {
        let a3379 = v0;
        let a3380 = 8u64;
        nat_eq(a3379, a3380)
    } {
        true
    } else {
        let a3381 = v0;
        let a3382 = 10u64;
        nat_eq(a3381, a3382)
    } {
        true
    } else {
        let a3383 = v0;
        let a3384 = 11u64;
        nat_eq(a3383, a3384)
    } {
        true
    } else {
        let a3385 = v0;
        let a3386 = 12u64;
        nat_eq(a3385, a3386)
    } {
        true
    } else {
        let a3387 = v0;
        let a3388 = 13u64;
        nat_eq(a3387, a3388)
    } {
        true
    } else {
        let a3389 = v0;
        let a3390 = 14u64;
        nat_eq(a3389, a3390)
    } {
        true
    } else {
        let a3391 = v0;
        let a3392 = 16u64;
        nat_eq(a3391, a3392)
    } {
        true
    } else {
        let a3393 = v0;
        let a3394 = 17u64;
        nat_eq(a3393, a3394)
    } {
        true
    } else {
        let a3395 = v0;
        let a3396 = 18u64;
        nat_eq(a3395, a3396)
    } {
        true
    } else {
        let a3397 = v0;
        let a3398 = 19u64;
        nat_eq(a3397, a3398)
    } {
        true
    } else {
        let a3399 = v0;
        let a3400 = 20u64;
        nat_eq(a3399, a3400)
    } {
        true
    } else {
        let a3401 = v0;
        let a3402 = 21u64;
        nat_eq(a3401, a3402)
    } {
        true
    } else {
        let a3403 = v0;
        let a3404 = 22u64;
        nat_eq(a3403, a3404)
    } {
        true
    } else {
        let a3405 = v0;
        let a3406 = 23u64;
        nat_eq(a3405, a3406)
    } {
        true
    } else {
        let a3407 = v0;
        let a3408 = 26u64;
        nat_eq(a3407, a3408)
    } {
        true
    } else {
        let a3409 = v0;
        let a3410 = 29u64;
        nat_eq(a3409, a3410)
    } {
        true
    } else {
        let a3411 = v0;
        let a3412 = 30u64;
        nat_eq(a3411, a3412)
    } {
        true
    } else {
        let a3413 = v0;
        let a3414 = 34u64;
        nat_eq(a3413, a3414)
    } {
        true
    } else {
        let a3415 = v0;
        let a3416 = 35u64;
        nat_eq(a3415, a3416)
    } {
        true
    } else {
        let a3417 = v0;
        let a3418 = 39u64;
        nat_eq(a3417, a3418)
    } {
        true
    } else {
        let a3419 = v0;
        let a3420 = 40u64;
        nat_eq(a3419, a3420)
    } {
        true
    } else {
        let a3421 = v0;
        let a3422 = 41u64;
        nat_eq(a3421, a3422)
    } {
        true
    } else {
        let a3423 = v0;
        let a3424 = 42u64;
        nat_eq(a3423, a3424)
    } {
        true
    } else {
        let a3425 = v0;
        let a3426 = 43u64;
        nat_eq(a3425, a3426)
    }
}

pub fn f120(v0: u64) -> bool {
    if f111(v0) {
        true
    } else {
        let a3427 = v0;
        let a3428 = 43u64;
        nat_eq(a3427, a3428)
    }
}

pub fn f121(v0: u64) -> Adt17 {
    let a3429 = v0;
    let a3430 = 0u64;
    if nat_eq(a3429, a3430) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3431 = v0;
        let a3432 = 1u64;
        if nat_eq(a3431, a3432) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3433 = v0;
            let a3434 = 2u64;
            if nat_eq(a3433, a3434) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3435 = v0;
                let a3436 = 3u64;
                if nat_eq(a3435, a3436) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3437 = v0;
                    let a3438 = 6u64;
                    if nat_eq(a3437, a3438) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3439 = v0;
                        let a3440 = 7u64;
                        if nat_eq(a3439, a3440) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3441 = v0;
                            let a3442 = 8u64;
                            if nat_eq(a3441, a3442) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3443 = v0;
                                let a3444 = 9u64;
                                if nat_eq(a3443, a3444) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3445 = v0;
                                    let a3446 = 10u64;
                                    if nat_eq(a3445, a3446) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3447 = v0;
                                        let a3448 = 11u64;
                                        if nat_eq(a3447, a3448) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3449 = v0;
                                            let a3450 = 12u64;
                                            if nat_eq(a3449, a3450) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3451 = v0;
                                                let a3452 = 13u64;
                                                if nat_eq(a3451, a3452) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3453 = v0;
                                                    let a3454 = 14u64;
                                                    if nat_eq(a3453, a3454) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3455 = v0;
                                                        let a3456 = 15u64;
                                                        if nat_eq(a3455, a3456) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3457 = v0;
                                                            let a3458 = 16u64;
                                                            if nat_eq(a3457, a3458) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3459 = v0;
                                                                let a3460 = 17u64;
                                                                if nat_eq(a3459, a3460) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3461 = v0;
                                                                    let a3462 = 18u64;
                                                                    if nat_eq(a3461, a3462) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3463 = v0;
                                                                        let a3464 = 19u64;
                                                                        if nat_eq(a3463, a3464) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3465 = v0;
                                                                            let a3466 = 20u64;
                                                                            if nat_eq(a3465, a3466) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3467 = v0;
                                                                                let a3468 = 21u64;
                                                                                if nat_eq(a3467, a3468) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3469 = v0;
                                                                                    let a3470 = 22u64;
                                                                                    if nat_eq(a3469, a3470) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3471 = v0;
                                                                                        let a3472 = 23u64;
                                                                                        if nat_eq(a3471, a3472) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3473 = v0;
                                                                                            let a3474 = 24u64;
                                                                                            if nat_eq(a3473, a3474) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3475 = v0;
                                                                                                let a3476 = 25u64;
                                                                                                if nat_eq(a3475, a3476) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3477 = v0;
                                                                                                    let a3478 = 26u64;
                                                                                                    if nat_eq(a3477, a3478) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3479 = v0;
                                                                                                        let a3480 = 27u64;
                                                                                                        if nat_eq(a3479, a3480) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3481 = v0;
                                                                                                            let a3482 = 28u64;
                                                                                                            if nat_eq(a3481, a3482) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3483 = v0;
                                                                                                                let a3484 = 29u64;
                                                                                                                if nat_eq(a3483, a3484) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3485 = v0;
                                                                                                                    let a3486 = 30u64;
                                                                                                                    if nat_eq(a3485, a3486) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3487 = v0;
                                                                                                                        let a3488 = 34u64;
                                                                                                                        if nat_eq(a3487, a3488) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3489 = v0;
                                                                                                                            let a3490 = 35u64;
                                                                                                                            if nat_eq(a3489, a3490) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3491 = v0;
                                                                                                                                let a3492 = 39u64;
                                                                                                                                if nat_eq(a3491, a3492) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3493 = v0;
                                                                                                                                    let a3494 = 40u64;
                                                                                                                                    if nat_eq(a3493, a3494) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3495 = v0;
                                                                                                                                        let a3496 = 41u64;
                                                                                                                                        if nat_eq(a3495, a3496) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3497 = v0;
                                                                                                                                            let a3498 = 42u64;
                                                                                                                                            if nat_eq(a3497, a3498) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3499 = v0;
                                                                                                                                                let a3500 = 43u64;
                                                                                                                                                if nat_eq(a3499, a3500) {
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

pub fn f122(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3503 = match v1.clone() {
        Adt2::C0(h3501, _, _, _, _, _, _, _, _, _, _) => {
            h3501
        }
    };
    let a3504 = match v3.clone() {
        Adt2::C0(h3502, _, _, _, _, _, _, _, _, _, _) => {
            h3502
        }
    };
    if if nat_eq(a3503, a3504) {
        let a3505 = v0;
        let a3506 = 33u64;
        if nat_eq(a3505, a3506) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3508 = f121(match v1.clone() {
            Adt2::C0(h3507, _, _, _, _, _, _, _, _, _, _) => {
                h3507
            }
        });
        match m3508 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3509, _) => {
                        h3509
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3514 = {
                    let a3511 = match v1.clone() {
                        Adt2::C0(_, h3510, _, _, _, _, _, _, _, _, _) => {
                            h3510
                        }
                    };
                    let a3512 = v5;
                    let a3513 = 1u64;
                    nat_rem(a3511, a3512, a3513)
                };
                let a3515 = 0u64;
                if if if nat_eq(a3514, a3515) {
                    let a3520 = {
                        let a3517 = match v3.clone() {
                            Adt2::C0(_, h3516, _, _, _, _, _, _, _, _, _) => {
                                h3516
                            }
                        };
                        let a3518 = v5;
                        let a3519 = 1u64;
                        nat_rem(a3517, a3518, a3519)
                    };
                    let a3521 = 0u64;
                    nat_eq(a3520, a3521)
                } else {
                    false
                } {
                    let a3522 = v6;
                    let a3523 = f16(v3.clone(), v5)?;
                    nat_eq(a3522, a3523)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3524) => {
                            h3524
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

pub fn f123(v0: u64, v1: u64) -> u64 {
    let a3525 = v0;
    let a3526 = 0u64;
    if if nat_eq(a3525, a3526) {
        true
    } else {
        let a3527 = v1;
        let a3528 = 0u64;
        nat_eq(a3527, a3528)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f124(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3532 = {
            let a3529 = v0;
            let a3530 = v4;
            let a3531 = 1u64;
            nat_rem(a3529, a3530, a3531)
        };
        let a3533 = 0u64;
        if if if nat_eq(a3532, a3533) {
            let a3537 = {
                let a3534 = v1;
                let a3535 = v5;
                let a3536 = 1u64;
                nat_rem(a3534, a3535, a3536)
            };
            let a3538 = 0u64;
            nat_eq(a3537, a3538)
        } else {
            false
        } {
            let a3542 = {
                let a3539 = v2;
                let a3540 = v6;
                let a3541 = 1u64;
                nat_rem(a3539, a3540, a3541)
            };
            let a3543 = 0u64;
            nat_eq(a3542, a3543)
        } else {
            false
        } {
            let a3547 = {
                let a3544 = v3;
                let a3545 = v7;
                let a3546 = 1u64;
                nat_rem(a3544, a3545, a3546)
            };
            let a3548 = 0u64;
            nat_eq(a3547, a3548)
        } else {
            false
        }
    }
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3549 = v0;
    let a3550 = v1;
    if nat_eq(a3549, a3550) {
        let a3553 = v4;
        let a3554 = {
            let a3551 = v2;
            let a3552 = v3;
            nat_add(a3551, a3552)?
        };
        Ok(nat_eq(a3553, a3554))
    } else {
        let a3555 = v2;
        let a3556 = v3;
        if nat_eq(a3555, a3556) {
            let a3557 = v4;
            let a3558 = v2;
            Ok(nat_eq(a3557, a3558))
        } else {
            Ok(false)
        }
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3561 = {
            let a3559 = v0;
            let a3560 = f140(v1);
            nat_add(a3559, a3560)?
        };
        let a3562 = f140(v2);
        nat_add(a3561, a3562)?
    };
    let v5: u64 = {
        let a3563 = f139(v1);
        let a3564 = f139(v2);
        nat_add(a3563, a3564)?
    };
    let a3565 = v1;
    let a3566 = 4294967295u64;
    if if nat_le(a3565, a3566) {
        let a3567 = v2;
        let a3568 = 4294967295u64;
        nat_le(a3567, a3568)
    } else {
        false
    } {
        let a3569 = v5;
        let a3570 = v4;
        if nat_le(a3569, a3570) {
            let a3573 = v3;
            let a3574 = {
                let a3571 = v4;
                let a3572 = v5;
                nat_sub(a3571, a3572)
            };
            Ok(nat_eq(a3573, a3574))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f127(v0: u64, v1: u64) -> R<bool> {
    let a3575 = v1;
    let a3576 = 4294967295u64;
    if nat_le(a3575, a3576) {
        let a3579 = {
            let a3577 = f140(v1);
            let a3578 = f139(v1);
            nat_add(a3577, a3578)?
        };
        let a3580 = v0;
        Ok(nat_lt(a3579, a3580))
    } else {
        Ok(false)
    }
}

pub fn f128(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f201(v0.clone(), false)?;
    let v6: u64 = f201(v0, true)?;
    let a3581 = v6;
    let a3582 = v1;
    if if nat_le(a3581, a3582) {
        let a3589 = {
            let a3587 = {
                let a3583 = v1;
                let a3584 = v5;
                nat_add(a3583, a3584)?
            };
            let a3588 = {
                let a3585 = v3;
                let a3586 = 0u64;
                if nat_eq(a3585, a3586) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3587, a3588)?
        };
        let a3590 = v2;
        nat_le(a3589, a3590)
    } else {
        false
    } {
        if v4 {
            let a3595 = {
                let a3593 = {
                    let a3591 = v1;
                    let a3592 = v5;
                    nat_add(a3591, a3592)?
                };
                let a3594 = v6;
                nat_sub(a3593, a3594)
            };
            let a3596 = v2;
            Ok(nat_lt(a3595, a3596))
        } else {
            let a3601 = {
                let a3599 = {
                    let a3597 = v1;
                    let a3598 = v5;
                    nat_add(a3597, a3598)?
                };
                let a3600 = v6;
                nat_sub(a3599, a3600)
            };
            let a3602 = v2;
            Ok(nat_le(a3601, a3602))
        }
    } else {
        Ok(false)
    }
}

pub fn f129(v0: u64, v1: u64) -> bool {
    let a3603 = v0;
    let a3604 = 0u64;
    if if nat_eq(a3603, a3604) {
        true
    } else {
        let a3605 = v0;
        let a3606 = 1u64;
        nat_eq(a3605, a3606)
    } {
        let a3607 = v1;
        let a3608 = 4294967296u64;
        nat_lt(a3607, a3608)
    } else {
        false
    }
}

pub fn f130(v0: u64, v1: u64) -> R<u64> {
    let a3613 = {
        let a3611 = v0;
        let a3612 = {
            let a3609 = v1;
            let a3610 = 1u64;
            nat_sub(a3609, a3610)
        };
        nat_add(a3611, a3612)?
    };
    let a3614 = v1;
    let a3615 = 0u64;
    Ok(nat_quot(a3613, a3614, a3615))
}

pub fn f131(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3616, _, _, _, _, _, _, _, _, _) => {
            h3616
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3617, _, _, _, _, _, _, _, _) => {
            h3617
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3618, _, _, _, _, _, _, _) => {
            h3618
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3619, _, _, _, _, _, _) => {
            h3619
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3622 = match v1.clone() {
                Adt2::C0(_, h3620, _, _, _, _, _, _, _, _, _) => {
                    h3620
                }
            };
            let a3623 = match v1.clone() {
                Adt2::C0(_, _, h3621, _, _, _, _, _, _, _, _) => {
                    h3621
                }
            };
            nat_mul(a3622, a3623)?
        };
        let v5: u64 = {
            let a3625 = v4;
            let a3626 = match v1.clone() {
                Adt2::C0(_, _, _, h3624, _, _, _, _, _, _, _) => {
                    h3624
                }
            };
            nat_mul(a3625, a3626)?
        };
        let a3639 = {
            let a3637 = {
                let a3628 = match v0.clone() {
                    Adt2::C0(_, h3627, _, _, _, _, _, _, _, _, _) => {
                        h3627
                    }
                };
                let a3629 = 1u64;
                nat_sub(a3628, a3629)
            };
            let a3638 = {
                let a3635 = f202(match v0.clone() {
                    Adt2::C0(_, _, h3630, _, _, _, _, _, _, _, _) => {
                        h3630
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3631, _, _, _, _, _, _, _, _, _) => {
                        h3631
                    }
                })?;
                let a3636 = f202(match v0 {
                    Adt2::C0(_, _, _, h3632, _, _, _, _, _, _, _) => {
                        h3632
                    }
                }, v2, {
                    let a3633 = v3;
                    let a3634 = v5;
                    nat_mul(a3633, a3634)?
                }, v4)?;
                nat_add(a3635, a3636)?
            };
            nat_add(a3637, a3638)?
        };
        let a3640 = f16(v1, 1u64)?;
        Ok(nat_lt(a3639, a3640))
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3641 = v0;
    let a3642 = 2147483648u64;
    if if if if nat_lt(a3641, a3642) {
        let a3643 = v1;
        let a3644 = 2147483648u64;
        nat_lt(a3643, a3644)
    } else {
        false
    } {
        let a3647 = {
            let a3645 = v0;
            let a3646 = v1;
            nat_mul(a3645, a3646)?
        };
        let a3648 = 2147483648u64;
        nat_lt(a3647, a3648)
    } else {
        false
    } {
        let a3653 = {
            let a3651 = {
                let a3649 = v0;
                let a3650 = 1u64;
                nat_sub(a3649, a3650)
            };
            let a3652 = v2;
            nat_mul(a3651, a3652)?
        };
        let a3654 = 2147483648u64;
        nat_lt(a3653, a3654)
    } else {
        false
    } {
        let a3659 = {
            let a3657 = {
                let a3655 = v1;
                let a3656 = 1u64;
                nat_sub(a3655, a3656)
            };
            let a3658 = v2;
            nat_mul(a3657, a3658)?
        };
        let a3660 = 2147483648u64;
        Ok(nat_lt(a3659, a3660))
    } else {
        Ok(false)
    }
}

pub fn f133(v0: u64) -> R<u64> {
    let a3663 = 3u64;
    let a3664 = {
        let a3661 = v0;
        let a3662 = 3u64;
        nat_mul(a3661, a3662)?
    };
    nat_add(a3663, a3664)
}

pub fn f134(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3665, _, _, _, _, _, _, _) => {
            h3665
        }
    }, match v0.clone() {
        Adt3::C0(_, h3666, _, _, _, _, _, _) => {
            h3666
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3667, _, _, _, _, _) => {
            h3667
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3668, _, _, _, _) => {
            h3668
        }
    }) {
        Ok(0u64)
    } else {
        let a3677 = match v0.clone() {
            Adt3::C0(h3669, _, _, _, _, _, _, _) => {
                h3669
            }
        };
        let a3678 = {
            let a3675 = match v0.clone() {
                Adt3::C0(_, h3670, _, _, _, _, _, _) => {
                    h3670
                }
            };
            let a3676 = {
                let a3673 = match v0.clone() {
                    Adt3::C0(_, _, h3671, _, _, _, _, _) => {
                        h3671
                    }
                };
                let a3674 = match v0 {
                    Adt3::C0(_, _, _, h3672, _, _, _, _) => {
                        h3672
                    }
                };
                nat_mul(a3673, a3674)?
            };
            nat_mul(a3675, a3676)?
        };
        nat_mul(a3677, a3678)
    }
}

pub fn f135(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3679 = v1;
    let a3680 = 13u64;
    if if nat_eq(a3679, a3680) {
        f34(match v0.clone() {
            Adt3::C0(h3681, _, _, _, _, _, _, _) => {
                h3681
            }
        }, match v0.clone() {
            Adt3::C0(_, h3682, _, _, _, _, _, _) => {
                h3682
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3683, _, _, _, _, _) => {
                h3683
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3684, _, _, _, _) => {
                h3684
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3690 = match v0.clone() {
                Adt3::C0(_, h3685, _, _, _, _, _, _) => {
                    h3685
                }
            };
            let a3691 = {
                let a3688 = match v0.clone() {
                    Adt3::C0(_, _, h3686, _, _, _, _, _) => {
                        h3686
                    }
                };
                let a3689 = match v0.clone() {
                    Adt3::C0(_, _, _, h3687, _, _, _, _) => {
                        h3687
                    }
                };
                nat_mul(a3688, a3689)?
            };
            nat_mul(a3690, a3691)?
        };
        let a3692 = v1;
        let a3693 = 13u64;
        if nat_eq(a3692, a3693) {
            let a3702 = 2u64;
            let a3703 = if f203(v0.clone(), v2) {
                let a3695 = v3;
                let a3696 = f204(match v0 {
                    Adt3::C0(h3694, _, _, _, _, _, _, _) => {
                        h3694
                    }
                }, false)?;
                nat_mul(a3695, a3696)?
            } else {
                let a3700 = v3;
                let a3701 = {
                    let a3698 = match v0 {
                        Adt3::C0(h3697, _, _, _, _, _, _, _) => {
                            h3697
                        }
                    };
                    let a3699 = 2u64;
                    nat_mul(a3698, a3699)?
                };
                nat_mul(a3700, a3701)?
            };
            nat_add(a3702, a3703)
        } else {
            let a3707 = v3;
            let a3708 = f204(match v0 {
                Adt3::C0(h3704, _, _, _, _, _, _, _) => {
                    h3704
                }
            }, {
                let a3705 = v1;
                let a3706 = 16u64;
                nat_eq(a3705, a3706)
            })?;
            nat_mul(a3707, a3708)
        }
    }
}

pub fn f136(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3709 = v0;
    let a3710 = 30u64;
    if if nat_eq(a3709, a3710) {
        let a3713 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3711, _, _, _, _) => {
                h3711
            }
        };
        let a3714 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3712, _, _, _, _, _) => {
                h3712
            }
        };
        nat_lt(a3713, a3714)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3715 = v3;
        match m3715 {
            None => {
                f205(v0, v1, match v2.clone() {
                    Adt2::C0(h3716, _, _, _, _, _, _, _, _, _, _) => {
                        h3716
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3717, _, _, _, _, _, _, _, _, _, _) => {
                        h3717
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3718, _, _, _, _, _, _, _, _, _) => {
                        h3718
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3719, _, _, _, _, _, _, _, _) => {
                        h3719
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3720, _, _, _, _, _, _, _) => {
                        h3720
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3721, _, _, _, _, _, _) => {
                        h3721
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3722, _, _, _, _, _, _, _, _, _) => {
                        h3722
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3723, _, _, _, _, _, _, _, _) => {
                        h3723
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3724, _, _, _, _, _, _, _) => {
                        h3724
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3725, _, _, _, _, _, _) => {
                        h3725
                    }
                })
            }
            Some(v5) => {
                f205(v0, v1, match v2.clone() {
                    Adt2::C0(h3726, _, _, _, _, _, _, _, _, _, _) => {
                        h3726
                    }
                }, match v5.clone() {
                    Adt2::C0(h3727, _, _, _, _, _, _, _, _, _, _) => {
                        h3727
                    }
                }, match v4.clone() {
                    Adt2::C0(h3728, _, _, _, _, _, _, _, _, _, _) => {
                        h3728
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3729, _, _, _, _, _, _, _, _, _) => {
                        h3729
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3730, _, _, _, _, _, _, _, _) => {
                        h3730
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3731, _, _, _, _, _, _, _) => {
                        h3731
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3732, _, _, _, _, _, _) => {
                        h3732
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3733, _, _, _, _, _, _, _, _, _) => {
                        h3733
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3734, _, _, _, _, _, _, _, _) => {
                        h3734
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3735, _, _, _, _, _, _, _) => {
                        h3735
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3736, _, _, _, _, _, _) => {
                        h3736
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3737, _, _, _, _, _, _, _, _, _) => {
                        h3737
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3738, _, _, _, _, _, _, _, _) => {
                        h3738
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3739, _, _, _, _, _, _, _) => {
                        h3739
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3740, _, _, _, _, _, _) => {
                        h3740
                    }
                })
            }
        }
    }
}

pub fn f137(v0: u64, v1: u64) -> u64 {
    let a3741 = v0;
    let a3742 = 23u64;
    if nat_eq(a3741, a3742) {
        let a3743 = v1;
        let a3744 = 0u64;
        if nat_eq(a3743, a3744) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3757 = {
        let a3753 = {
            let a3749 = {
                let a3745 = v0;
                let a3746 = v4;
                nat_mul(a3745, a3746)?
            };
            let a3750 = {
                let a3747 = v1;
                let a3748 = v5;
                nat_mul(a3747, a3748)?
            };
            nat_add(a3749, a3750)?
        };
        let a3754 = {
            let a3751 = v2;
            let a3752 = v6;
            nat_mul(a3751, a3752)?
        };
        nat_add(a3753, a3754)?
    };
    let a3758 = {
        let a3755 = v3;
        let a3756 = v7;
        nat_mul(a3755, a3756)?
    };
    nat_add(a3757, a3758)
}

pub fn f139(v0: u64) -> u64 {
    let a3759 = v0;
    let a3760 = 2147483648u64;
    if nat_lt(a3759, a3760) {
        0u64
    } else {
        let a3761 = 4294967296u64;
        let a3762 = v0;
        nat_sub(a3761, a3762)
    }
}

pub fn f140(v0: u64) -> u64 {
    let a3763 = v0;
    let a3764 = 2147483648u64;
    if nat_lt(a3763, a3764) {
        v0
    } else {
        0u64
    }
}

pub fn f141(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3765 = v7;
        let a3766 = 6u64;
        if nat_eq(a3765, a3766) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f94(v3, v4), f94(v5, v6), f95(v7, v8), f206(v1), f206(v9))
}

pub fn f142(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3767 = v0;
    match m3767 {
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
        Adt4::C13(v34, v35, v36, v37) => {
            Adt11::C4(v34, v35, v36, v37)
        }
        Adt4::C14(v38, _, _) => {
            let a3768 = 13u64;
            let a3769 = v38;
            if nat_le(a3768, a3769) {
                Adt11::C7(v38, v1, v2)
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
        Adt4::C27(v58) => {
            Adt11::C2(match v58.clone() {
                Adt5::C0(_, _, _, _, _, h3770, _, _) => {
                    h3770
                }
            }, match v58 {
                Adt5::C0(_, _, _, _, _, _, _, h3771) => {
                    h3771
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
        Adt4::C33(_) => {
            Adt11::C6
        }
    }
}

pub fn f143(v0: Adt4) -> u64 {
    let m3772 = v0;
    match m3772 {
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
        Adt4::C14(v36, _, _) => {
            v36
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
        Adt4::C33(_) => {
            0u64
        }
    }
}

pub fn f144(v0: Adt4, v1: u64) -> u64 {
    let m3773 = v0;
    match m3773 {
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
        Adt4::C16(_, v41) => {
            v41
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v44, _) => {
            let a3774 = v1;
            let a3775 = 6u64;
            if nat_eq(a3774, a3775) {
                v44
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
        Adt4::C26(v58, _, _, _) => {
            let a3784 = v1;
            let a3785 = 6u64;
            if if if nat_eq(a3784, a3785) {
                true
            } else {
                let a3786 = v1;
                let a3787 = 7u64;
                nat_eq(a3786, a3787)
            } {
                let a3788 = v58;
                let a3789 = 30u64;
                nat_eq(a3788, a3789)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v57) => {
            match v57 {
                Adt5::C0(_, _, _, _, _, _, _, h3783) => {
                    h3783
                }
            }
        }
        Adt4::C28(v53) => {
            let a3776 = v1;
            let a3777 = 6u64;
            if nat_eq(a3776, a3777) {
                match v53 {
                    Adt7::C0(_, _, _, _, h3778) => {
                        h3778
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v56) => {
            let a3779 = v1;
            let a3780 = 6u64;
            if if nat_eq(a3779, a3780) {
                true
            } else {
                let a3781 = v1;
                let a3782 = 7u64;
                nat_eq(a3781, a3782)
            } {
                v56
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
        Adt4::C33(_) => {
            0u64
        }
    }
}

pub fn f145(v0: Adt4, v1: u64) -> Adt11 {
    let a3790 = v1;
    let a3791 = 4u64;
    if nat_lt(a3790, a3791) {
        Adt11::C0(v1)
    } else {
        let a3792 = v1;
        let a3793 = 4u64;
        if nat_eq(a3792, a3793) {
            Adt11::C1(0u64)
        } else {
            let a3794 = v1;
            let a3795 = 5u64;
            if nat_eq(a3794, a3795) {
                Adt11::C1(3u64)
            } else {
                let a3796 = v1;
                let a3797 = 6u64;
                if nat_eq(a3796, a3797) {
                    Adt11::C2(f144(v0, v1), 0u64, false)
                } else {
                    let a3798 = v1;
                    let a3799 = 7u64;
                    if nat_eq(a3798, a3799) {
                        Adt11::C2(0u64, f144(v0, v1), false)
                    } else {
                        let a3800 = v1;
                        let a3801 = 16u64;
                        if nat_lt(a3800, a3801) {
                            Adt11::C1({
                                let a3802 = v1;
                                let a3803 = 8u64;
                                nat_sub(a3802, a3803)
                            })
                        } else {
                            let a3804 = v1;
                            let a3805 = 20u64;
                            if nat_lt(a3804, a3805) {
                                Adt11::C5({
                                    let a3806 = v1;
                                    let a3807 = 16u64;
                                    nat_sub(a3806, a3807)
                                })
                            } else {
                                let a3808 = v1;
                                let a3809 = 20u64;
                                if nat_eq(a3808, a3809) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3810 = v1;
                                    let a3811 = 21u64;
                                    if nat_eq(a3810, a3811) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3812 = v1;
                                        let a3813 = 22u64;
                                        if nat_eq(a3812, a3813) {
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

pub fn f146(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3815 = v0;
        let a3816 = match v1.clone() {
            Adt3::C0(h3814, _, _, _, _, _, _, _) => {
                h3814
            }
        };
        let a3817 = 0u64;
        nat_quot(a3815, a3816, a3817)
    };
    let v3: u64 = {
        let a3819 = v2;
        let a3820 = match v1.clone() {
            Adt3::C0(_, h3818, _, _, _, _, _, _) => {
                h3818
            }
        };
        let a3821 = 0u64;
        nat_quot(a3819, a3820, a3821)
    };
    f138({
        let a3823 = v0;
        let a3824 = match v1.clone() {
            Adt3::C0(h3822, _, _, _, _, _, _, _) => {
                h3822
            }
        };
        let a3825 = 0u64;
        nat_rem(a3823, a3824, a3825)
    }, {
        let a3827 = v2;
        let a3828 = match v1.clone() {
            Adt3::C0(_, h3826, _, _, _, _, _, _) => {
                h3826
            }
        };
        let a3829 = 0u64;
        nat_rem(a3827, a3828, a3829)
    }, {
        let a3831 = v3;
        let a3832 = match v1.clone() {
            Adt3::C0(_, _, h3830, _, _, _, _, _) => {
                h3830
            }
        };
        let a3833 = 0u64;
        nat_rem(a3831, a3832, a3833)
    }, {
        let a3835 = v3;
        let a3836 = match v1.clone() {
            Adt3::C0(_, _, h3834, _, _, _, _, _) => {
                h3834
            }
        };
        let a3837 = 0u64;
        nat_quot(a3835, a3836, a3837)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3838, _, _, _) => {
            h3838
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3839, _, _) => {
            h3839
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3840, _) => {
            h3840
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3841) => {
            h3841
        }
    })
}

pub fn f147(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3859 = {
        let a3854 = f207(v2, {
            let a3845 = {
                let a3842 = v0;
                let a3843 = v1;
                let a3844 = 0u64;
                nat_quot(a3842, a3843, a3844)
            };
            let a3846 = v2;
            let a3847 = 0u64;
            nat_rem(a3845, a3846, a3847)
        }, {
            let a3851 = {
                let a3848 = v0;
                let a3849 = v1;
                let a3850 = 0u64;
                nat_quot(a3848, a3849, a3850)
            };
            let a3852 = v2;
            let a3853 = 0u64;
            nat_quot(a3851, a3852, a3853)
        })?;
        let a3855 = v1;
        nat_mul(a3854, a3855)?
    };
    let a3860 = {
        let a3856 = v0;
        let a3857 = v1;
        let a3858 = 0u64;
        nat_rem(a3856, a3857, a3858)
    };
    nat_add(a3859, a3860)
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3862 = v0;
        let a3863 = match v2.clone() {
            Adt3::C0(h3861, _, _, _, _, _, _, _) => {
                h3861
            }
        };
        let a3864 = 0u64;
        nat_quot(a3862, a3863, a3864)
    };
    let v4: u64 = {
        let a3866 = v3;
        let a3867 = match v2.clone() {
            Adt3::C0(_, h3865, _, _, _, _, _, _) => {
                h3865
            }
        };
        let a3868 = 0u64;
        nat_quot(a3866, a3867, a3868)
    };
    f208({
        let a3870 = v0;
        let a3871 = match v2.clone() {
            Adt3::C0(h3869, _, _, _, _, _, _, _) => {
                h3869
            }
        };
        let a3872 = 0u64;
        nat_rem(a3870, a3871, a3872)
    }, {
        let a3874 = v3;
        let a3875 = match v2.clone() {
            Adt3::C0(_, h3873, _, _, _, _, _, _) => {
                h3873
            }
        };
        let a3876 = 0u64;
        nat_rem(a3874, a3875, a3876)
    }, {
        let a3878 = v4;
        let a3879 = match v2.clone() {
            Adt3::C0(_, _, h3877, _, _, _, _, _) => {
                h3877
            }
        };
        let a3880 = 0u64;
        nat_rem(a3878, a3879, a3880)
    }, {
        let a3882 = v4;
        let a3883 = match v2 {
            Adt3::C0(_, _, h3881, _, _, _, _, _) => {
                h3881
            }
        };
        let a3884 = 0u64;
        nat_quot(a3882, a3883, a3884)
    }, match v1.clone() {
        Adt3::C0(h3885, _, _, _, _, _, _, _) => {
            h3885
        }
    }, match v1.clone() {
        Adt3::C0(_, h3886, _, _, _, _, _, _) => {
            h3886
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3887, _, _, _, _, _) => {
            h3887
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3888, _, _, _, _) => {
            h3888
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3889, _, _, _) => {
            h3889
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3890, _, _) => {
            h3890
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3891, _) => {
            h3891
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3892) => {
            h3892
        }
    })
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3894 = v0;
        let a3895 = match v3.clone() {
            Adt3::C0(h3893, _, _, _, _, _, _, _) => {
                h3893
            }
        };
        let a3896 = 0u64;
        nat_quot(a3894, a3895, a3896)
    };
    let v6: u64 = {
        let a3898 = v5;
        let a3899 = match v3.clone() {
            Adt3::C0(_, h3897, _, _, _, _, _, _) => {
                h3897
            }
        };
        let a3900 = 0u64;
        nat_quot(a3898, a3899, a3900)
    };
    let v7: u64 = {
        let a3902 = v0;
        let a3903 = match v3.clone() {
            Adt3::C0(h3901, _, _, _, _, _, _, _) => {
                h3901
            }
        };
        let a3904 = 0u64;
        nat_rem(a3902, a3903, a3904)
    };
    let v8: u64 = {
        let a3906 = v5;
        let a3907 = match v3.clone() {
            Adt3::C0(_, h3905, _, _, _, _, _, _) => {
                h3905
            }
        };
        let a3908 = 0u64;
        nat_rem(a3906, a3907, a3908)
    };
    let v9: u64 = {
        let a3910 = v6;
        let a3911 = match v3.clone() {
            Adt3::C0(_, _, h3909, _, _, _, _, _) => {
                h3909
            }
        };
        let a3912 = 0u64;
        nat_rem(a3910, a3911, a3912)
    };
    let v10: u64 = {
        let a3914 = v6;
        let a3915 = match v3.clone() {
            Adt3::C0(_, _, h3913, _, _, _, _, _) => {
                h3913
            }
        };
        let a3916 = 0u64;
        nat_quot(a3914, a3915, a3916)
    };
    let v11: u64 = {
        let a3917 = v4;
        let a3918 = 0u64;
        if nat_eq(a3917, a3918) {
            match v1.clone() {
                Adt3::C0(h3919, _, _, _, _, _, _, _) => {
                    h3919
                }
            }
        } else {
            let a3920 = v4;
            let a3921 = 1u64;
            if nat_eq(a3920, a3921) {
                match v1.clone() {
                    Adt3::C0(_, h3922, _, _, _, _, _, _) => {
                        h3922
                    }
                }
            } else {
                let a3923 = v4;
                let a3924 = 2u64;
                if nat_eq(a3923, a3924) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3925, _, _, _, _, _) => {
                            h3925
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3926, _, _, _, _) => {
                            h3926
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3933 = v11;
        let a3934 = {
            let a3927 = v4;
            let a3928 = 0u64;
            if nat_eq(a3927, a3928) {
                v7
            } else {
                let a3929 = v4;
                let a3930 = 1u64;
                if nat_eq(a3929, a3930) {
                    v8
                } else {
                    let a3931 = v4;
                    let a3932 = 2u64;
                    if nat_eq(a3931, a3932) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3933, a3934)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f138(if if v12 {
        let a3935 = v4;
        let a3936 = 0u64;
        nat_eq(a3935, a3936)
    } else {
        false
    } {
        let a3937 = v7;
        let a3938 = v11;
        nat_sub(a3937, a3938)
    } else {
        v7
    }, if if v12 {
        let a3939 = v4;
        let a3940 = 1u64;
        nat_eq(a3939, a3940)
    } else {
        false
    } {
        let a3941 = v8;
        let a3942 = v11;
        nat_sub(a3941, a3942)
    } else {
        v8
    }, if if v12 {
        let a3943 = v4;
        let a3944 = 2u64;
        nat_eq(a3943, a3944)
    } else {
        false
    } {
        let a3945 = v9;
        let a3946 = v11;
        nat_sub(a3945, a3946)
    } else {
        v9
    }, if if v12 {
        let a3947 = v4;
        let a3948 = 3u64;
        nat_eq(a3947, a3948)
    } else {
        false
    } {
        let a3949 = v10;
        let a3950 = v11;
        nat_sub(a3949, a3950)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3951, _, _, _) => {
            h3951
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3952, _, _) => {
            h3952
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3953, _) => {
            h3953
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3954) => {
            h3954
        }
    })?, f146(v0, v3)?))
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f72(v0, v2.clone(), 0u64);
    let v9: u64 = f72(v0, v2.clone(), 1u64);
    let v10: u64 = f72(v0, v2.clone(), 2u64);
    let v11: u64 = f72(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f71(v1.clone(), f209(v8, v3, match v1.clone() {
            Adt3::C0(h3955, _, _, _, _, _, _, _) => {
                h3955
            }
        })?, f209(v9, v4, match v1.clone() {
            Adt3::C0(_, h3956, _, _, _, _, _, _) => {
                h3956
            }
        })?, f209(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3957, _, _, _, _, _) => {
                h3957
            }
        })?, f209(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3958, _, _, _, _) => {
                h3958
            }
        })?)?, f146(v0, v2)?))
    } else {
        if if if if f210(v8, v3, match v1.clone() {
            Adt3::C0(h3959, _, _, _, _, _, _, _) => {
                h3959
            }
        })? {
            f210(v9, v4, match v1.clone() {
                Adt3::C0(_, h3960, _, _, _, _, _, _) => {
                    h3960
                }
            })?
        } else {
            false
        } {
            f210(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3961, _, _, _, _, _) => {
                    h3961
                }
            })?
        } else {
            false
        } {
            f210(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3962, _, _, _, _) => {
                    h3962
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f71(v1, f211(v8, v3)?, f211(v9, v4)?, f211(v10, v5)?, f211(v11, v6)?)?, f146(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f146(v0, v2)?))
        }
    }
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f72(v0, v2.clone(), 0u64);
    let v5: u64 = f72(v0, v2.clone(), 1u64);
    let v6: u64 = f72(v0, v2.clone(), 2u64);
    let v7: u64 = f72(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f71(v1.clone(), f212(v4, v3, match v1 {
        Adt3::C0(h3963, _, _, _, _, _, _, _) => {
            h3963
        }
    })?, v5, v6, v7)?, f146(v0, v2)?))
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3964 = v0;
    let a3965 = v3;
    if nat_lt(a3964, a3965) {
        Ok(Adt9::C1(false, {
            let a3966 = v0;
            let a3967 = 4u64;
            nat_mul(a3966, a3967)?
        }, {
            let a3968 = v0;
            let a3969 = 4u64;
            nat_mul(a3968, a3969)?
        }))
    } else {
        let v6: u64 = {
            let a3970 = v0;
            let a3971 = v3;
            nat_sub(a3970, a3971)
        };
        let v7: u64 = f146(v6, v1)?;
        let v8: u64 = f213(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3972 = v0;
    let a3973 = v3;
    if nat_lt(a3972, a3973) {
        Ok(Adt9::C1(true, 0u64, f146(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3974 = v0;
            let a3975 = v3;
            nat_sub(a3974, a3975)
        };
        let v5: u64 = f148(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f146(v4, v1)?, 3u64, v5))
    }
}

pub fn f154(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3977 = v1;
        let a3978 = match v3.clone() {
            Adt3::C0(h3976, _, _, _, _, _, _, _) => {
                h3976
            }
        };
        let a3979 = 0u64;
        nat_rem(a3977, a3978, a3979)
    };
    let v5: u64 = {
        let a3985 = {
            let a3981 = v1;
            let a3982 = match v3.clone() {
                Adt3::C0(h3980, _, _, _, _, _, _, _) => {
                    h3980
                }
            };
            let a3983 = 0u64;
            nat_quot(a3981, a3982, a3983)
        };
        let a3986 = match v3.clone() {
            Adt3::C0(_, h3984, _, _, _, _, _, _) => {
                h3984
            }
        };
        let a3987 = 0u64;
        nat_rem(a3985, a3986, a3987)
    };
    let v6: bool = f214(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f148(v1, v2, v3.clone())?
    } else {
        0u64
    }, f146(v1, v3)?))
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3988 = v3;
        let a3989 = 8u64;
        nat_add(a3988, a3989)?
    }, 0u64, f146(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f148(v0, v5, v2.clone())?
    }, 3u64, f146(v0, v2)?))
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3990 = v4;
    let a3991 = 0u64;
    if if nat_eq(a3990, a3991) {
        f52(v3)
    } else {
        false
    } {
        let m3992 = f121(v3);
        match m3992 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h3993, _) => {
                        h3993
                    }
                };
                Ok(Adt9::C5(v3, f146(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h3994) => {
                        h3994
                    }
                }, f146({
                    let a3995 = v0;
                    let a3996 = v6;
                    nat_mul(a3995, a3996)?
                }, v2.clone())?, {
                    let a4002 = {
                        let a4000 = {
                            let a3997 = v6;
                            let a3998 = 1u64;
                            nat_sub(a3997, a3998)
                        };
                        let a4001 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3999, _, _, _) => {
                                h3999
                            }
                        };
                        nat_mul(a4000, a4001)?
                    };
                    let a4003 = 4u64;
                    nat_add(a4002, a4003)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4004, _, _, _) => {
                        h4004
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f146(v0, v1)?, f146(v0, v2)?))
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4006 = v0;
        let a4007 = match v2.clone() {
            Adt3::C0(h4005, _, _, _, _, _, _, _) => {
                h4005
            }
        };
        let a4008 = 0u64;
        nat_rem(a4006, a4007, a4008)
    };
    let v7: u64 = {
        let a4010 = v0;
        let a4011 = match v2.clone() {
            Adt3::C0(h4009, _, _, _, _, _, _, _) => {
                h4009
            }
        };
        let a4012 = 0u64;
        nat_quot(a4010, a4011, a4012)
    };
    let v8: u64 = {
        let a4014 = v7;
        let a4015 = match v2.clone() {
            Adt3::C0(_, h4013, _, _, _, _, _, _) => {
                h4013
            }
        };
        let a4016 = 0u64;
        nat_rem(a4014, a4015, a4016)
    };
    let v9: u64 = {
        let a4018 = v7;
        let a4019 = match v2.clone() {
            Adt3::C0(_, h4017, _, _, _, _, _, _) => {
                h4017
            }
        };
        let a4020 = 0u64;
        nat_quot(a4018, a4019, a4020)
    };
    let v10: u64 = {
        let a4022 = v9;
        let a4023 = match v2.clone() {
            Adt3::C0(_, _, h4021, _, _, _, _, _) => {
                h4021
            }
        };
        let a4024 = 0u64;
        nat_rem(a4022, a4023, a4024)
    };
    let v11: u64 = {
        let a4026 = v9;
        let a4027 = match v2.clone() {
            Adt3::C0(_, _, h4025, _, _, _, _, _) => {
                h4025
            }
        };
        let a4028 = 0u64;
        nat_quot(a4026, a4027, a4028)
    };
    let v12: u64 = f146(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4037 = {
                let a4032 = {
                    let a4029 = v10;
                    let a4030 = v4;
                    let a4031 = 0u64;
                    nat_quot(a4029, a4030, a4031)
                };
                let a4033 = v5;
                nat_mul(a4032, a4033)?
            };
            let a4038 = {
                let a4034 = v8;
                let a4035 = v4;
                let a4036 = 0u64;
                nat_quot(a4034, a4035, a4036)
            };
            nat_add(a4037, a4038)?
        };
        let v14: u64 = {
            let a4058 = v6;
            let a4059 = {
                let a4056 = match v1.clone() {
                    Adt3::C0(h4039, _, _, _, _, _, _, _) => {
                        h4039
                    }
                };
                let a4057 = {
                    let a4054 = {
                        let a4040 = v8;
                        let a4041 = v4;
                        let a4042 = 0u64;
                        nat_rem(a4040, a4041, a4042)
                    };
                    let a4055 = {
                        let a4052 = match v1.clone() {
                            Adt3::C0(_, h4043, _, _, _, _, _, _) => {
                                h4043
                            }
                        };
                        let a4053 = {
                            let a4050 = {
                                let a4044 = v10;
                                let a4045 = v4;
                                let a4046 = 0u64;
                                nat_rem(a4044, a4045, a4046)
                            };
                            let a4051 = {
                                let a4048 = match v1.clone() {
                                    Adt3::C0(_, _, h4047, _, _, _, _, _) => {
                                        h4047
                                    }
                                };
                                let a4049 = v13;
                                nat_mul(a4048, a4049)?
                            };
                            nat_add(a4050, a4051)?
                        };
                        nat_mul(a4052, a4053)?
                    };
                    nat_add(a4054, a4055)?
                };
                nat_mul(a4056, a4057)?
            };
            nat_add(a4058, a4059)?
        };
        Ok(Adt9::C1(false, f146(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4065 = {
                let a4063 = {
                    let a4060 = v11;
                    let a4061 = v5;
                    let a4062 = 0u64;
                    nat_rem(a4060, a4061, a4062)
                };
                let a4064 = v4;
                nat_mul(a4063, a4064)?
            };
            let a4066 = v8;
            nat_add(a4065, a4066)?
        };
        let v16: u64 = {
            let a4072 = {
                let a4070 = {
                    let a4067 = v11;
                    let a4068 = v5;
                    let a4069 = 0u64;
                    nat_quot(a4067, a4068, a4069)
                };
                let a4071 = v4;
                nat_mul(a4070, a4071)?
            };
            let a4073 = v10;
            nat_add(a4072, a4073)?
        };
        let a4075 = v15;
        let a4076 = match v1.clone() {
            Adt3::C0(_, h4074, _, _, _, _, _, _) => {
                h4074
            }
        };
        if if nat_lt(a4075, a4076) {
            let a4078 = v16;
            let a4079 = match v1.clone() {
                Adt3::C0(_, _, h4077, _, _, _, _, _) => {
                    h4077
                }
            };
            nat_lt(a4078, a4079)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f146({
                let a4088 = v6;
                let a4089 = {
                    let a4086 = match v1.clone() {
                        Adt3::C0(h4080, _, _, _, _, _, _, _) => {
                            h4080
                        }
                    };
                    let a4087 = {
                        let a4084 = v15;
                        let a4085 = {
                            let a4082 = match v1.clone() {
                                Adt3::C0(_, h4081, _, _, _, _, _, _) => {
                                    h4081
                                }
                            };
                            let a4083 = v16;
                            nat_mul(a4082, a4083)?
                        };
                        nat_add(a4084, a4085)?
                    };
                    nat_mul(a4086, a4087)?
                };
                nat_add(a4088, a4089)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f137(23u64, v4);
    let v6: u64 = {
        let a4090 = v0;
        let a4091 = v5;
        let a4092 = 0u64;
        nat_rem(a4090, a4091, a4092)
    };
    let v7: u64 = {
        let a4093 = v0;
        let a4094 = v5;
        let a4095 = 0u64;
        nat_quot(a4093, a4094, a4095)
    };
    let v8: Adt13 = Adt13::C1(f146(v7, v2)?);
    let a4096 = v6;
    let a4097 = 0u64;
    if nat_eq(a4096, a4097) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f146(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f206(v3), 4u64))
    } else {
        let a4098 = v6;
        let a4099 = 1u64;
        if nat_eq(a4098, a4099) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4100 = v4;
                let a4101 = 0u64;
                if nat_eq(a4100, a4101) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f206(v4)))
        }
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f146(v0, v2.clone())?;
    let v4: u64 = f146(v0, v1.clone())?;
    let a4106 = {
        let a4103 = v0;
        let a4104 = match v1 {
            Adt3::C0(h4102, _, _, _, _, _, _, _) => {
                h4102
            }
        };
        let a4105 = 0u64;
        nat_rem(a4103, a4104, a4105)
    };
    let a4107 = 0u64;
    if nat_eq(a4106, a4107) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f146({
            let a4108 = v0;
            let a4109 = 1u64;
            nat_sub(a4108, a4109)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4110 = v3;
    let a4111 = 13u64;
    if nat_eq(a4110, a4111) {
        f215(v0, v1, v4)
    } else {
        let v5: u64 = f204(match v1.clone() {
            Adt3::C0(h4112, _, _, _, _, _, _, _) => {
                h4112
            }
        }, {
            let a4113 = v3;
            let a4114 = 16u64;
            nat_eq(a4113, a4114)
        })?;
        let v6: u64 = {
            let a4115 = v0;
            let a4116 = v5;
            let a4117 = 0u64;
            nat_quot(a4115, a4116, a4117)
        };
        let v7: u64 = {
            let a4118 = v0;
            let a4119 = v5;
            let a4120 = 0u64;
            nat_rem(a4118, a4119, a4120)
        };
        let v8: u64 = {
            let a4124 = 1u64;
            let a4125 = {
                let a4122 = match v1.clone() {
                    Adt3::C0(h4121, _, _, _, _, _, _, _) => {
                        h4121
                    }
                };
                let a4123 = 2u64;
                nat_mul(a4122, a4123)?
            };
            nat_add(a4124, a4125)?
        };
        let a4126 = v7;
        let a4127 = v8;
        if nat_lt(a4126, a4127) {
            f216(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f146(v6, v2)?;
            let a4128 = v3;
            let a4129 = 14u64;
            if nat_eq(a4128, a4129) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4130 = v7;
                let a4131 = v8;
                if nat_eq(a4130, a4131) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4134 = v7;
                    let a4135 = {
                        let a4132 = v8;
                        let a4133 = 1u64;
                        nat_add(a4132, a4133)?
                    };
                    if nat_eq(a4134, a4135) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4136, _, _, _, _, _, _, _) => {
                                h4136
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

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4137 = v0;
    let a4138 = 0u64;
    if nat_eq(a4137, a4138) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4145 = {
            let a4139 = v0;
            let a4140 = 1u64;
            nat_add(a4139, a4140)?
        };
        let a4146 = {
            let a4143 = 2u64;
            let a4144 = {
                let a4141 = v3;
                let a4142 = 2u64;
                nat_mul(a4141, a4142)?
            };
            nat_add(a4143, a4144)?
        };
        if nat_eq(a4145, a4146) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4147 = v0;
                let a4148 = 1u64;
                nat_sub(a4147, a4148)
            };
            let v5: u64 = {
                let a4149 = v4;
                let a4150 = 2u64;
                let a4151 = 0u64;
                nat_quot(a4149, a4150, a4151)
            };
            let a4155 = {
                let a4152 = v4;
                let a4153 = 2u64;
                let a4154 = 0u64;
                nat_rem(a4152, a4153, a4154)
            };
            let a4156 = 0u64;
            if nat_eq(a4155, a4156) {
                Ok(Adt9::C4(17u64, 0u64, f146(v5, v1)?, 4u64, f146(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f162(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4157 = v0;
    let a4158 = f27(v1, v2, v3)?;
    if nat_lt(a4157, a4158) {
        let a4159 = v0;
        let a4160 = v1;
        if nat_lt(a4159, a4160) {
            Ok(Adt9::C1(false, {
                let a4161 = 4u64;
                let a4162 = v0;
                nat_mul(a4161, a4162)?
            }, {
                let a4163 = 4u64;
                let a4164 = v0;
                nat_mul(a4163, a4164)?
            }))
        } else {
            let v4: u64 = {
                let a4165 = v0;
                let a4166 = v1;
                nat_sub(a4165, a4166)
            };
            let v5: u64 = f217(v4, v2)?;
            let a4167 = v5;
            let a4168 = 2u64;
            if nat_lt(a4167, a4168) {
                Ok(Adt9::C4(6u64, {
                    let a4169 = v5;
                    let a4170 = 0u64;
                    if nat_eq(a4169, a4170) {
                        4u64
                    } else {
                        5u64
                    }
                }, f218(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f219(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f220(v4, v2)? {
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

pub fn f163(v0: u64, v1: u64) -> Adt11 {
    let a4171 = v0;
    let a4172 = 43u64;
    if nat_eq(a4171, a4172) {
        Adt11::C8(3u64)
    } else {
        let a4173 = v1;
        let a4174 = 43u64;
        if nat_eq(a4173, a4174) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f164(v0: u64, v1: u64) -> R<u64> {
    if f111(v0) {
        let a4175 = 2u64;
        let a4176 = v1;
        nat_mul(a4175, a4176)
    } else {
        let a4177 = v1;
        let a4178 = f200(v0);
        let a4179 = 0u64;
        Ok(nat_quot(a4177, a4178, a4179))
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4180 = f121(v0);
    match m4180 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f138(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4181, _, _, _) => {
                    h4181
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4182, _, _) => {
                    h4182
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4183, _) => {
                    h4183
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4184) => {
                    h4184
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4185) => {
                    h4185
                }
            }, f138({
                let a4187 = v2;
                let a4188 = match v6.clone() {
                    Adt18::C0(_, h4186, _) => {
                        h4186
                    }
                };
                nat_mul(a4187, a4188)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4189, _, _, _) => {
                    h4189
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4190, _, _) => {
                    h4190
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4191, _) => {
                    h4191
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4192) => {
                    h4192
                }
            })?, match v6 {
                Adt18::C0(_, h4193, _) => {
                    h4193
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4194, _, _, _) => {
                    h4194
                }
            }))
        }
    }
}

pub fn f166(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4202 = {
        let a4195 = 2u64;
        let a4196 = f134(v1.clone())?;
        nat_mul(a4195, a4196)?
    };
    let a4203 = {
        let a4200 = f134(v1)?;
        let a4201 = {
            let a4198 = 1u64;
            let a4199 = match v0 {
                Adt3::C0(h4197, _, _, _, _, _, _, _) => {
                    h4197
                }
            };
            nat_add(a4198, a4199)?
        };
        nat_mul(a4200, a4201)?
    };
    nat_add(a4202, a4203)
}

pub fn f167(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f146(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f168(v0: u64, v1: u64) -> bool {
    let a4204 = v1;
    let a4205 = 2147483647u64;
    if nat_le(a4204, a4205) {
        let a4206 = v1;
        let a4207 = v0;
        nat_lt(a4206, a4207)
    } else {
        false
    }
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4208 = f121(v0);
    match m4208 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4210 = v2;
                let a4211 = match v7.clone() {
                    Adt18::C0(_, h4209, _) => {
                        h4209
                    }
                };
                nat_mul(a4210, a4211)?
            };
            let m4224 = f221(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4212, _, _, _, _, _, _, _) => {
                    h4212
                }
            }, match v5.clone() {
                Adt3::C0(_, h4213, _, _, _, _, _, _) => {
                    h4213
                }
            }, match v5 {
                Adt3::C0(_, _, h4214, _, _, _, _, _) => {
                    h4214
                }
            }, {
                let a4217 = match v3.clone() {
                    Adt3::C0(h4215, _, _, _, _, _, _, _) => {
                        h4215
                    }
                };
                let a4218 = match v7.clone() {
                    Adt18::C0(_, h4216, _) => {
                        h4216
                    }
                };
                nat_mul(a4217, a4218)?
            }, match v3.clone() {
                Adt3::C0(_, h4219, _, _, _, _, _, _) => {
                    h4219
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4220, _, _, _) => {
                    h4220
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4221, _, _) => {
                    h4221
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4222, _) => {
                    h4222
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4223) => {
                    h4223
                }
            })?;
            match m4224 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4225 = v0;
                    let a4226 = 0u64;
                    if if nat_eq(a4225, a4226) {
                        true
                    } else {
                        let a4227 = v0;
                        let a4228 = 26u64;
                        nat_eq(a4227, a4228)
                    } {
                        Ok(Adt9::C1(false, v9, f146({
                            let a4232 = {
                                let a4230 = v1;
                                let a4231 = match v4.clone() {
                                    Adt3::C0(h4229, _, _, _, _, _, _, _) => {
                                        h4229
                                    }
                                };
                                nat_mul(a4230, a4231)?
                            };
                            let a4233 = v8;
                            nat_add(a4232, a4233)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4234) => {
                                h4234
                            }
                        }, f146({
                            let a4238 = {
                                let a4236 = v1;
                                let a4237 = match v4.clone() {
                                    Adt3::C0(h4235, _, _, _, _, _, _, _) => {
                                        h4235
                                    }
                                };
                                nat_mul(a4236, a4237)?
                            };
                            let a4239 = v8;
                            nat_add(a4238, a4239)?
                        }, v4.clone())?, {
                            let a4246 = {
                                let a4244 = {
                                    let a4241 = match v7 {
                                        Adt18::C0(_, h4240, _) => {
                                            h4240
                                        }
                                    };
                                    let a4242 = 1u64;
                                    nat_sub(a4241, a4242)
                                };
                                let a4245 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4243, _, _, _) => {
                                        h4243
                                    }
                                };
                                nat_mul(a4244, a4245)?
                            };
                            let a4247 = 4u64;
                            nat_add(a4246, a4247)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4248, _, _, _) => {
                                h4248
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4249 = v1;
    let a4250 = 0u64;
    if if nat_eq(a4249, a4250) {
        true
    } else {
        let a4251 = v2;
        let a4252 = 0u64;
        nat_eq(a4251, a4252)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f171(v0: u64, v1: u64) -> bool {
    let a4253 = v0;
    let a4254 = 4294967295u64;
    if nat_eq(a4253, a4254) {
        true
    } else {
        let a4255 = v0;
        let a4256 = 2147483648u64;
        if nat_lt(a4255, a4256) {
            let a4257 = v0;
            let a4258 = v1;
            nat_lt(a4257, a4258)
        } else {
            false
        }
    }
}

pub fn f172(v0: Adt7) -> R<u64> {
    let a4263 = if match v0.clone() {
        Adt7::C0(_, _, _, h4259, _) => {
            h4259
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4264 = {
        let a4261 = match v0 {
            Adt7::C0(_, _, _, _, h4260) => {
                h4260
            }
        };
        let a4262 = 1u64;
        if nat_eq(a4261, a4262) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4263, a4264)
}

pub fn f173(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4269 = {
            let a4266 = v2;
            let a4267 = match v0.clone() {
                Adt6::C0(_, h4265, _, _) => {
                    h4265
                }
            };
            nat_mul(a4266, a4267)?
        };
        let a4270 = f139(match v0.clone() {
            Adt6::C0(_, _, h4268, _) => {
                h4268
            }
        });
        nat_add(a4269, a4270)?
    };
    let v4: u64 = f140(match v0.clone() {
        Adt6::C0(_, _, h4271, _) => {
            h4271
        }
    });
    let v5: u64 = {
        let a4275 = {
            let a4273 = v3;
            let a4274 = match v0 {
                Adt6::C0(h4272, _, _, _) => {
                    h4272
                }
            };
            nat_add(a4273, a4274)?
        };
        let a4276 = v4;
        nat_sub(a4275, a4276)
    };
    let a4281 = {
        let a4277 = v5;
        let a4278 = v1;
        if nat_lt(a4277, a4278) {
            v5
        } else {
            v1
        }
    };
    let a4282 = {
        let a4279 = v3;
        let a4280 = v4;
        nat_sub(a4279, a4280)
    };
    Ok(nat_sub(a4281, a4282))
}

pub fn f174(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4291 = f74(match v0.clone() {
        Adt7::C0(h4283, _, _, _, _) => {
            h4283
        }
    }, match v1.clone() {
        Adt3::C0(h4284, _, _, _, _, _, _, _) => {
            h4284
        }
    }, f72(v3, v2.clone(), 0u64), {
        let a4288 = v4;
        let a4289 = {
            let m4287 = match v0.clone() {
                Adt7::C0(h4286, _, _, _, _) => {
                    h4286
                }
            };
            match m4287 {
                Adt6::C0(h4285, _, _, _) => {
                    h4285
                }
            }
        };
        let a4290 = 0u64;
        nat_rem(a4288, a4289, a4290)
    })?;
    match m4291 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4300 = f74(match v0.clone() {
                Adt7::C0(_, h4292, _, _, _) => {
                    h4292
                }
            }, match v1.clone() {
                Adt3::C0(_, h4293, _, _, _, _, _, _) => {
                    h4293
                }
            }, f72(v3, v2.clone(), 1u64), {
                let a4297 = v4;
                let a4298 = {
                    let m4296 = match v0 {
                        Adt7::C0(h4295, _, _, _, _) => {
                            h4295
                        }
                    };
                    match m4296 {
                        Adt6::C0(h4294, _, _, _) => {
                            h4294
                        }
                    }
                };
                let a4299 = 0u64;
                nat_quot(a4297, a4298, a4299)
            })?;
            match m4300 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f71(v1, v5, v6, f72(v3, v2.clone(), 2u64), f72(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f175(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4301 = v0;
    match m4301 {
        Adt16::C0(v19) => {
            f222(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4302 = v20;
            let a4303 = 66u64;
            if nat_eq(a4302, a4303) {
                f223(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f224(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f176(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4310 = v3;
        let a4311 = {
            let a4307 = f146({
                let a4304 = v0;
                let a4305 = 2u64;
                let a4306 = 0u64;
                nat_quot(a4304, a4305, a4306)
            }, v1)?;
            let a4308 = v2;
            let a4309 = 0u64;
            nat_quot(a4307, a4308, a4309)
        };
        nat_add(a4310, a4311)?
    };
    let a4315 = {
        let a4312 = v0;
        let a4313 = 2u64;
        let a4314 = 0u64;
        nat_rem(a4312, a4313, a4314)
    };
    let a4316 = 0u64;
    if nat_eq(a4315, a4316) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4317 = v4;
        let a4318 = 0u64;
        if nat_eq(a4317, a4318) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f177(v0: u64, v1: u64) -> R<u64> {
    let a4321 = 8u64;
    let a4322 = {
        let a4319 = f225(v0);
        let a4320 = v1;
        nat_mul(a4319, a4320)?
    };
    nat_add(a4321, a4322)
}

pub fn f178(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4323, _, _, _, _, _, _, _) => {
            h4323
        }
    };
    let v7: u64 = f225(v3);
    let v8: u64 = {
        let a4326 = 1u64;
        let a4327 = {
            let a4324 = v6;
            let a4325 = v7;
            nat_mul(a4324, a4325)?
        };
        nat_add(a4326, a4327)?
    };
    let a4328 = v0;
    let a4329 = 0u64;
    if nat_eq(a4328, a4329) {
        Ok(f226(0u64))
    } else {
        let a4330 = v0;
        let a4331 = v8;
        if nat_lt(a4330, a4331) {
            let v9: u64 = {
                let a4332 = v0;
                let a4333 = 1u64;
                nat_sub(a4332, a4333)
            };
            let v10: u64 = {
                let a4334 = v9;
                let a4335 = v7;
                let a4336 = 0u64;
                nat_rem(a4334, a4335, a4336)
            };
            let v11: u64 = f146({
                let a4342 = {
                    let a4337 = v1;
                    let a4338 = v6;
                    nat_mul(a4337, a4338)?
                };
                let a4343 = {
                    let a4339 = v9;
                    let a4340 = v7;
                    let a4341 = 0u64;
                    nat_quot(a4339, a4340, a4341)
                };
                nat_add(a4342, a4343)?
            }, v2)?;
            let a4344 = v3;
            let a4345 = 30u64;
            if nat_eq(a4344, a4345) {
                let a4346 = v10;
                let a4347 = 0u64;
                if nat_eq(a4346, a4347) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4348 = v10;
                    let a4349 = 1u64;
                    if nat_eq(a4348, a4349) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4350 = v10;
                        let a4351 = 2u64;
                        if nat_eq(a4350, a4351) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4352 = v10;
                let a4353 = 0u64;
                if nat_eq(a4352, a4353) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4354 = v10;
                    let a4355 = 1u64;
                    if nat_eq(a4354, a4355) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4356 = v0;
                let a4357 = v8;
                nat_sub(a4356, a4357)
            };
            let a4358 = v12;
            let a4359 = 0u64;
            if nat_eq(a4358, a4359) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4360 = v12;
                let a4361 = 1u64;
                if nat_eq(a4360, a4361) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4362 = v12;
                    let a4363 = 2u64;
                    if nat_eq(a4362, a4363) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4364 = v12;
                        let a4365 = 3u64;
                        if nat_eq(a4364, a4365) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4366 = v12;
                            let a4367 = 4u64;
                            if nat_eq(a4366, a4367) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4368 = v12;
                                let a4369 = 5u64;
                                if nat_eq(a4368, a4369) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4370 = v12;
                                    let a4371 = 6u64;
                                    if nat_eq(a4370, a4371) {
                                        if f227(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4372 = 3u64;
                                                let a4373 = v1;
                                                nat_add(a4372, a4373)?
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

pub fn f179(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4374 = v0;
        let a4375 = f106(v3);
        let a4376 = 0u64;
        nat_quot(a4374, a4375, a4376)
    };
    let v5: u64 = {
        let a4381 = 3u64;
        let a4382 = {
            let a4378 = v4;
            let a4379 = match v1.clone() {
                Adt3::C0(h4377, _, _, _, _, _, _, _) => {
                    h4377
                }
            };
            let a4380 = 0u64;
            nat_quot(a4378, a4379, a4380)
        };
        nat_add(a4381, a4382)?
    };
    let v6: u64 = f146(v4, v1)?;
    let v7: u64 = f146(v4, v2)?;
    let a4383 = v3;
    let a4384 = 30u64;
    if nat_eq(a4383, a4384) {
        let a4388 = {
            let a4385 = v0;
            let a4386 = 3u64;
            let a4387 = 0u64;
            nat_rem(a4385, a4386, a4387)
        };
        let a4389 = 0u64;
        if nat_eq(a4388, a4389) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4393 = {
                let a4390 = v0;
                let a4391 = 3u64;
                let a4392 = 0u64;
                nat_rem(a4390, a4391, a4392)
            };
            let a4394 = 1u64;
            if nat_eq(a4393, a4394) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f180(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4395 = v0;
        let a4396 = 8388608u64;
        let a4397 = 0u64;
        nat_quot(a4395, a4396, a4397)
    };
    let a4398 = v0;
    let a4399 = 2147483648u64;
    if if if nat_lt(a4398, a4399) {
        let a4400 = 127u64;
        let a4401 = v1;
        nat_le(a4400, a4401)
    } else {
        false
    } {
        let a4402 = v1;
        let a4403 = 190u64;
        nat_lt(a4402, a4403)
    } else {
        false
    } {
        let v2: u64 = {
            let a4407 = 8388608u64;
            let a4408 = {
                let a4404 = v0;
                let a4405 = 8388608u64;
                let a4406 = 0u64;
                nat_rem(a4404, a4405, a4406)
            };
            nat_add(a4407, a4408)?
        };
        let a4409 = v1;
        let a4410 = 150u64;
        if nat_lt(a4409, a4410) {
            let v3: u64 = f228({
                let a4411 = 150u64;
                let a4412 = v1;
                nat_sub(a4411, a4412)
            });
            let a4416 = {
                let a4413 = v2;
                let a4414 = v3;
                let a4415 = 0u64;
                nat_rem(a4413, a4414, a4415)
            };
            let a4417 = 0u64;
            if nat_eq(a4416, a4417) {
                let a4418 = v2;
                let a4419 = v3;
                let a4420 = 0u64;
                Ok(nat_quot(a4418, a4419, a4420))
            } else {
                Ok(0u64)
            }
        } else {
            let a4423 = f228({
                let a4421 = v1;
                let a4422 = 150u64;
                nat_sub(a4421, a4422)
            });
            let a4424 = v2;
            nat_mul(a4423, a4424)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f181(v0: Adt14, v1: Adt14) -> bool {
    let a4427 = match v0.clone() {
        Adt14::C0(_, h4425, _, _, _, _, _, _, _, _, _) => {
            h4425
        }
    };
    let a4428 = match v1.clone() {
        Adt14::C0(_, h4426, _, _, _, _, _, _, _, _, _) => {
            h4426
        }
    };
    if if if nat_eq(a4427, a4428) {
        let a4431 = match v0.clone() {
            Adt14::C0(_, _, h4429, _, _, _, _, _, _, _, _) => {
                h4429
            }
        };
        let a4432 = match v1.clone() {
            Adt14::C0(_, _, h4430, _, _, _, _, _, _, _, _) => {
                h4430
            }
        };
        nat_eq(a4431, a4432)
    } else {
        false
    } {
        let a4435 = match v0.clone() {
            Adt14::C0(_, _, _, h4433, _, _, _, _, _, _, _) => {
                h4433
            }
        };
        let a4436 = match v1.clone() {
            Adt14::C0(_, _, _, h4434, _, _, _, _, _, _, _) => {
                h4434
            }
        };
        nat_eq(a4435, a4436)
    } else {
        false
    } {
        let a4439 = match v0 {
            Adt14::C0(_, _, _, _, h4437, _, _, _, _, _, _) => {
                h4437
            }
        };
        let a4440 = match v1 {
            Adt14::C0(_, _, _, _, h4438, _, _, _, _, _, _) => {
                h4438
            }
        };
        nat_eq(a4439, a4440)
    } else {
        false
    }
}

pub fn f182(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4441 = v0;
    let a4442 = 0u64;
    if if if nat_eq(a4441, a4442) {
        true
    } else {
        let a4443 = v1;
        let a4444 = 0u64;
        nat_eq(a4443, a4444)
    } {
        true
    } else {
        let a4445 = v2;
        let a4446 = 0u64;
        nat_eq(a4445, a4446)
    } {
        true
    } else {
        let a4447 = v3;
        let a4448 = 0u64;
        nat_eq(a4447, a4448)
    }
}

pub fn f183(v0: Adt14, v1: Adt14) -> bool {
    if f189(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4449, _, _, _, _, _) => {
            h4449
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4450, _, _, _, _, _) => {
            h4450
        }
    }) {
        let a4452 = match v0 {
            Adt14::C0(_, h4451, _, _, _, _, _, _, _, _, _) => {
                h4451
            }
        };
        let a4453 = 2147483647u64;
        nat_le(a4452, a4453)
    } else {
        true
    }
}

pub fn f184(v0: Adt14) -> bool {
    let a4458 = {
        let a4455 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4454) => {
                h4454
            }
        };
        let a4456 = 4u64;
        let a4457 = 0u64;
        nat_rem(a4455, a4456, a4457)
    };
    let a4459 = 0u64;
    if if nat_eq(a4458, a4459) {
        f229(match v0.clone() {
            Adt14::C0(_, h4460, _, _, _, _, _, _, _, _, _) => {
                h4460
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4461, _, _, _, _, _) => {
                h4461
            }
        })
    } else {
        false
    } {
        f102(match v0.clone() {
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
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4465, _, _, _, _) => {
                h4465
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4466, _, _, _) => {
                h4466
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4467, _, _) => {
                h4467
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f185(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4469 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4468, _) => {
            h4468
        }
    };
    let a4470 = 0u64;
    if nat_eq(a4469, a4470) {
        Ok(true)
    } else {
        let a4472 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4471, _) => {
                h4471
            }
        };
        let a4473 = 0u64;
        if nat_eq(a4472, a4473) {
            Ok(true)
        } else {
            let a4475 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4474) => {
                    h4474
                }
            };
            let a4476 = 0u64;
            if nat_eq(a4475, a4476) {
                Ok(true)
            } else {
                let a4478 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4477) => {
                        h4477
                    }
                };
                let a4479 = 0u64;
                if nat_eq(a4478, a4479) {
                    Ok(true)
                } else {
                    let a4485 = {
                        let a4482 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4480) => {
                                h4480
                            }
                        };
                        let a4483 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4481, _) => {
                                h4481
                            }
                        };
                        nat_add(a4482, a4483)?
                    };
                    let a4486 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4484) => {
                            h4484
                        }
                    };
                    if nat_le(a4485, a4486) {
                        Ok(true)
                    } else {
                        let a4492 = {
                            let a4489 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4487) => {
                                    h4487
                                }
                            };
                            let a4490 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4488, _) => {
                                    h4488
                                }
                            };
                            nat_add(a4489, a4490)?
                        };
                        let a4493 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4491) => {
                                h4491
                            }
                        };
                        Ok(nat_le(a4492, a4493))
                    }
                }
            }
        }
    }
}

pub fn f186(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f181(v0.clone(), v1.clone()) {
        let a4496 = match v0.clone() {
            Adt14::C0(h4494, _, _, _, _, _, _, _, _, _, _) => {
                h4494
            }
        };
        let a4497 = match v1.clone() {
            Adt14::C0(h4495, _, _, _, _, _, _, _, _, _, _) => {
                h4495
            }
        };
        nat_eq(a4496, a4497)
    } else {
        false
    } {
        let a4500 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4498) => {
                h4498
            }
        };
        let a4501 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4499) => {
                h4499
            }
        };
        nat_eq(a4500, a4501)
    } else {
        false
    } {
        let a4504 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4502, _, _, _, _, _) => {
                h4502
            }
        };
        let a4505 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4503, _, _, _, _, _) => {
                h4503
            }
        };
        nat_eq(a4504, a4505)
    } else {
        false
    } {
        let a4508 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4506, _, _, _, _) => {
                h4506
            }
        };
        let a4509 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4507, _, _, _, _) => {
                h4507
            }
        };
        nat_eq(a4508, a4509)
    } else {
        false
    } {
        let a4512 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4510, _, _, _) => {
                h4510
            }
        };
        let a4513 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4511, _, _, _) => {
                h4511
            }
        };
        nat_eq(a4512, a4513)
    } else {
        false
    } {
        let a4516 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4514, _, _) => {
                h4514
            }
        };
        let a4517 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4515, _, _) => {
                h4515
            }
        };
        nat_eq(a4516, a4517)
    } else {
        false
    }
}

pub fn f187(v0: Adt14) -> R<bool> {
    if f230(match v0.clone() {
        Adt14::C0(_, h4518, _, _, _, _, _, _, _, _, _) => {
            h4518
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4519, _, _, _, _, _) => {
            h4519
        }
    }) {
        f103(f231(match v0.clone() {
            Adt14::C0(_, h4520, _, _, _, _, _, _, _, _, _) => {
                h4520
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4521, _, _, _, _, _) => {
                h4521
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4522, _, _, _, _, _, _, _, _) => {
                h4522
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4523, _, _, _, _, _, _, _) => {
                h4523
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4524, _, _, _, _, _, _) => {
                h4524
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4525, _, _, _, _) => {
                h4525
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4526, _, _, _) => {
                h4526
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4527, _, _) => {
                h4527
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f188(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4528 = v1;
    let a4529 = 0u64;
    if if nat_eq(a4528, a4529) {
        true
    } else {
        let a4530 = v2;
        let a4531 = 0u64;
        nat_eq(a4530, a4531)
    } {
        Ok(0u64)
    } else {
        let a4534 = f104(v1, v2, v3)?;
        let a4535 = {
            let a4532 = v2;
            let a4533 = f232(v0, v1)?;
            nat_mul(a4532, a4533)?
        };
        nat_add(a4534, a4535)
    }
}

pub fn f189(v0: u64, v1: u64) -> bool {
    let a4536 = v0;
    let a4537 = 4u64;
    if nat_eq(a4536, a4537) {
        let a4538 = v1;
        let a4539 = 4u64;
        nat_eq(a4538, a4539)
    } else {
        false
    }
}

pub fn f190(v0: u64) -> R<u64> {
    let a4540 = 6u64;
    let a4541 = v0;
    nat_add(a4540, a4541)
}

pub fn f191(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4542, _, _, _, _, _, _, _, _, _, _) => {
            h4542
        }
    }, match v0.clone() {
        Adt14::C0(_, h4543, _, _, _, _, _, _, _, _, _) => {
            h4543
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4544, _, _, _, _, _, _, _, _) => {
            h4544
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4545, _, _, _, _, _, _, _) => {
            h4545
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4546, _, _, _, _, _, _) => {
            h4546
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4547, _, _, _, _) => {
            h4547
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4548, _, _, _) => {
            h4548
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4549, _, _) => {
            h4549
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4550, _) => {
            h4550
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4551) => {
            h4551
        }
    })
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4552 = v0;
    let a4553 = 2147483647u64;
    if if if nat_le(a4552, a4553) {
        let a4554 = 0u64;
        let a4555 = v3;
        if nat_lt(a4554, a4555) {
            let a4556 = v3;
            let a4557 = 2147483647u64;
            nat_le(a4556, a4557)
        } else {
            false
        }
    } else {
        false
    } {
        let a4562 = {
            let a4560 = {
                let a4558 = v2;
                let a4559 = v3;
                nat_add(a4558, a4559)?
            };
            let a4561 = 1u64;
            nat_sub(a4560, a4561)
        };
        let a4563 = 2147483647u64;
        nat_le(a4562, a4563)
    } else {
        false
    } {
        let a4568 = {
            let a4566 = {
                let a4564 = v0;
                let a4565 = v1;
                nat_mul(a4564, a4565)?
            };
            let a4567 = f233(v2, v3)?;
            nat_mul(a4566, a4567)?
        };
        let a4569 = 9223372036854775807u64;
        Ok(nat_le(a4568, a4569))
    } else {
        Ok(false)
    }
}

pub fn f193(v0: Adt14) -> R<u64> {
    let a4600 = {
        let a4592 = {
            let a4584 = {
                let a4576 = 4u64;
                let a4577 = {
                    let a4574 = {
                        let a4571 = match v0.clone() {
                            Adt14::C0(_, h4570, _, _, _, _, _, _, _, _, _) => {
                                h4570
                            }
                        };
                        let a4572 = 1u64;
                        nat_sub(a4571, a4572)
                    };
                    let a4575 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4573, _, _, _, _, _) => {
                            h4573
                        }
                    };
                    nat_mul(a4574, a4575)?
                };
                nat_add(a4576, a4577)?
            };
            let a4585 = {
                let a4582 = {
                    let a4579 = match v0.clone() {
                        Adt14::C0(_, _, h4578, _, _, _, _, _, _, _, _) => {
                            h4578
                        }
                    };
                    let a4580 = 1u64;
                    nat_sub(a4579, a4580)
                };
                let a4583 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4581, _, _, _, _) => {
                        h4581
                    }
                };
                nat_mul(a4582, a4583)?
            };
            nat_add(a4584, a4585)?
        };
        let a4593 = {
            let a4590 = {
                let a4587 = match v0.clone() {
                    Adt14::C0(_, _, _, h4586, _, _, _, _, _, _, _) => {
                        h4586
                    }
                };
                let a4588 = 1u64;
                nat_sub(a4587, a4588)
            };
            let a4591 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4589, _, _, _) => {
                    h4589
                }
            };
            nat_mul(a4590, a4591)?
        };
        nat_add(a4592, a4593)?
    };
    let a4601 = {
        let a4598 = {
            let a4595 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4594, _, _, _, _, _, _) => {
                    h4594
                }
            };
            let a4596 = 1u64;
            nat_sub(a4595, a4596)
        };
        let a4599 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4597, _, _) => {
                h4597
            }
        };
        nat_mul(a4598, a4599)?
    };
    nat_add(a4600, a4601)
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4602 = v0;
    let a4603 = 0u64;
    if if if if nat_eq(a4602, a4603) {
        true
    } else {
        let a4604 = v1;
        let a4605 = 0u64;
        nat_eq(a4604, a4605)
    } {
        true
    } else {
        let a4606 = v2;
        let a4607 = 0u64;
        nat_eq(a4606, a4607)
    } {
        true
    } else {
        let a4608 = v3;
        let a4609 = 0u64;
        nat_eq(a4608, a4609)
    } {
        Ok(0u64)
    } else {
        let a4610 = f234(v0, v1, v2, v3, v5)?;
        let a4611 = f235(v0, v1, v2, v3, v4)?;
        nat_add(a4610, a4611)
    }
}

pub fn f195(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4612 = v0;
    let a4613 = 0u64;
    if if nat_eq(a4612, a4613) {
        true
    } else {
        let a4614 = v1;
        let a4615 = 0u64;
        nat_eq(a4614, a4615)
    } {
        Ok(0u64)
    } else {
        let a4618 = f104(v0, v1, v2)?;
        let a4619 = {
            let a4616 = v1;
            let a4617 = f236(v0)?;
            nat_mul(a4616, a4617)?
        };
        nat_add(a4618, a4619)
    }
}

pub fn f196(v0: u64, v1: u64, v2: u64) -> bool {
    let a4620 = v0;
    let a4621 = 1u64;
    if nat_le(a4620, a4621) {
        true
    } else {
        let a4625 = {
            let a4622 = v1;
            let a4623 = v2;
            let a4624 = 1u64;
            nat_rem(a4622, a4623, a4624)
        };
        let a4626 = 0u64;
        nat_eq(a4625, a4626)
    }
}

pub fn f197(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4627 = v1;
    let a4628 = 1u64;
    if if if nat_le(a4627, a4628) {
        true
    } else {
        let a4629 = v0;
        let a4630 = v2;
        nat_le(a4629, a4630)
    } {
        let a4631 = v3;
        let a4632 = 1u64;
        if nat_le(a4631, a4632) {
            true
        } else {
            let a4639 = {
                let a4637 = v0;
                let a4638 = {
                    let a4635 = {
                        let a4633 = v1;
                        let a4634 = 1u64;
                        nat_sub(a4633, a4634)
                    };
                    let a4636 = v2;
                    nat_mul(a4635, a4636)?
                };
                nat_add(a4637, a4638)?
            };
            let a4640 = v4;
            nat_le(a4639, a4640)
        }
    } else {
        false
    } {
        let a4641 = v5;
        let a4642 = 1u64;
        if nat_le(a4641, a4642) {
            Ok(true)
        } else {
            let a4655 = {
                let a4653 = {
                    let a4647 = v0;
                    let a4648 = {
                        let a4645 = {
                            let a4643 = v1;
                            let a4644 = 1u64;
                            nat_sub(a4643, a4644)
                        };
                        let a4646 = v2;
                        nat_mul(a4645, a4646)?
                    };
                    nat_add(a4647, a4648)?
                };
                let a4654 = {
                    let a4651 = {
                        let a4649 = v3;
                        let a4650 = 1u64;
                        nat_sub(a4649, a4650)
                    };
                    let a4652 = v4;
                    nat_mul(a4651, a4652)?
                };
                nat_add(a4653, a4654)?
            };
            let a4656 = v6;
            Ok(nat_le(a4655, a4656))
        }
    } else {
        Ok(false)
    }
}

pub fn f198(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f237(v0, v1, v6, v7, v8) {
        let a4657 = v5;
        let a4658 = 26u64;
        nat_eq(a4657, a4658)
    } else {
        false
    } {
        let a4659 = v3;
        let a4660 = v7;
        nat_eq(a4659, a4660)
    } else {
        false
    } {
        let a4661 = v4;
        let a4662 = v8;
        nat_eq(a4661, a4662)
    } else {
        false
    } {
        let a4663 = v9;
        let a4664 = 1u64;
        nat_eq(a4663, a4664)
    } else {
        false
    } {
        let a4665 = v10;
        let a4666 = f238(v0);
        nat_eq(a4665, a4666)
    } else {
        false
    } {
        let a4667 = v11;
        let a4668 = v1;
        nat_eq(a4667, a4668)
    } else {
        false
    } {
        let a4669 = v12;
        let a4670 = v6;
        nat_eq(a4669, a4670)
    } else {
        false
    } {
        let a4671 = v13;
        let a4672 = v7;
        nat_eq(a4671, a4672)
    } else {
        false
    } {
        let a4673 = v14;
        let a4674 = v8;
        nat_eq(a4673, a4674)
    } else {
        false
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4675 = v0;
    let a4676 = 0u64;
    if if if nat_eq(a4675, a4676) {
        true
    } else {
        let a4677 = v1;
        let a4678 = 0u64;
        nat_eq(a4677, a4678)
    } {
        true
    } else {
        let a4679 = v2;
        let a4680 = 0u64;
        nat_eq(a4679, a4680)
    } {
        true
    } else {
        let a4681 = v3;
        let a4682 = 0u64;
        nat_eq(a4681, a4682)
    }
}

pub fn f200(v0: u64) -> u64 {
    let m4683 = f121(v0);
    match m4683 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h4684, _) => {
                    h4684
                }
            }
        }
    }
}

pub fn f201(v0: Adt3, v1: bool) -> R<u64> {
    f239(v0.clone(), {
        let a4686 = match v0.clone() {
            Adt3::C0(h4685, _, _, _, _, _, _, _) => {
                h4685
            }
        };
        let a4687 = 1u64;
        nat_sub(a4686, a4687)
    }, {
        let a4689 = match v0.clone() {
            Adt3::C0(_, h4688, _, _, _, _, _, _) => {
                h4688
            }
        };
        let a4690 = 1u64;
        nat_sub(a4689, a4690)
    }, {
        let a4692 = match v0.clone() {
            Adt3::C0(_, _, h4691, _, _, _, _, _) => {
                h4691
            }
        };
        let a4693 = 1u64;
        nat_sub(a4692, a4693)
    }, {
        let a4695 = match v0 {
            Adt3::C0(_, _, _, h4694, _, _, _, _) => {
                h4694
            }
        };
        let a4696 = 1u64;
        nat_sub(a4695, a4696)
    }, v1)
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4697 = v0;
    let a4698 = 0u64;
    if nat_eq(a4697, a4698) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4699 = v0;
            let a4700 = 1u64;
            nat_sub(a4699, a4700)
        };
        let v5: u64 = {
            let a4701 = v4;
            let a4702 = v1;
            let a4703 = 0u64;
            nat_quot(a4701, a4702, a4703)
        };
        let v6: u64 = {
            let a4711 = {
                let a4704 = v5;
                let a4705 = v2;
                nat_mul(a4704, a4705)?
            };
            let a4712 = {
                let a4709 = {
                    let a4706 = v4;
                    let a4707 = v1;
                    let a4708 = 0u64;
                    nat_rem(a4706, a4707, a4708)
                };
                let a4710 = v3;
                nat_mul(a4709, a4710)?
            };
            nat_add(a4711, a4712)?
        };
        let a4713 = v5;
        let a4714 = 0u64;
        if nat_eq(a4713, a4714) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4723 = {
                    let a4717 = {
                        let a4715 = v5;
                        let a4716 = 1u64;
                        nat_sub(a4715, a4716)
                    };
                    let a4718 = v2;
                    nat_mul(a4717, a4718)?
                };
                let a4724 = {
                    let a4721 = {
                        let a4719 = v1;
                        let a4720 = 1u64;
                        nat_sub(a4719, a4720)
                    };
                    let a4722 = v3;
                    nat_mul(a4721, a4722)?
                };
                nat_add(a4723, a4724)?
            };
            let a4725 = v6;
            let a4726 = v7;
            if nat_lt(a4725, a4726) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f203(v0: Adt3, v1: u64) -> bool {
    let a4727 = v1;
    let a4728 = 0u64;
    if !nat_eq(a4727, a4728) {
        true
    } else {
        let a4730 = match v0 {
            Adt3::C0(_, _, _, _, h4729, _, _, _) => {
                h4729
            }
        };
        let a4731 = 4u64;
        nat_eq(a4730, a4731)
    }
}

pub fn f204(v0: u64, v1: bool) -> R<u64> {
    let a4734 = {
        let a4732 = v0;
        let a4733 = 2u64;
        nat_mul(a4732, a4733)?
    };
    let a4735 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4734, a4735)
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4736 = f240(v0, v1);
    match m4736 {
        None => {
            if v5 {
                let m4737 = f241(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4737 {
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
                let a4738 = 4u64;
                let a4739 = v28;
                nat_le(a4738, a4739)
            };
            let a4740 = v2;
            let a4741 = 0u64;
            if if if if if nat_eq(a4740, a4741) {
                let a4742 = v4;
                let a4743 = 0u64;
                nat_eq(a4742, a4743)
            } else {
                false
            } {
                let a4744 = v6;
                let a4745 = v14;
                if nat_eq(a4744, a4745) {
                    let a4746 = v7;
                    let a4747 = v15;
                    nat_eq(a4746, a4747)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4748 = v8;
                let a4749 = v16;
                if nat_eq(a4748, a4749) {
                    let a4750 = v9;
                    let a4751 = v17;
                    nat_eq(a4750, a4751)
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
                        let a4752 = v3;
                        let a4753 = 0u64;
                        nat_eq(a4752, a4753)
                    } else {
                        false
                    } {
                        f124(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4758 = {
                        let a4754 = v6;
                        let a4755 = v7;
                        nat_mul(a4754, a4755)?
                    };
                    let a4759 = {
                        let a4756 = v8;
                        let a4757 = v9;
                        nat_mul(a4756, a4757)?
                    };
                    nat_mul(a4758, a4759)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f206(v0: u64) -> u64 {
    let m4760 = f121(v0);
    match m4760 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4762 = match v1.clone() {
                Adt18::C0(_, h4761, _) => {
                    h4761
                }
            };
            let a4763 = 1u64;
            if if nat_eq(a4762, a4763) {
                let a4765 = match v1.clone() {
                    Adt18::C0(_, _, h4764) => {
                        h4764
                    }
                };
                let a4766 = 1u64;
                if if if nat_eq(a4765, a4766) {
                    true
                } else {
                    let a4768 = match v1.clone() {
                        Adt18::C0(_, _, h4767) => {
                            h4767
                        }
                    };
                    let a4769 = 2u64;
                    nat_eq(a4768, a4769)
                } {
                    true
                } else {
                    let a4771 = match v1.clone() {
                        Adt18::C0(_, _, h4770) => {
                            h4770
                        }
                    };
                    let a4772 = 4u64;
                    nat_eq(a4771, a4772)
                } {
                    true
                } else {
                    let a4774 = match v1.clone() {
                        Adt18::C0(_, _, h4773) => {
                            h4773
                        }
                    };
                    let a4775 = 8u64;
                    nat_eq(a4774, a4775)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4776) => {
                        h4776
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4781 = {
        let a4779 = {
            let a4777 = v0;
            let a4778 = v1;
            nat_sub(a4777, a4778)
        };
        let a4780 = 1u64;
        nat_sub(a4779, a4780)
    };
    let a4782 = v2;
    nat_add(a4781, a4782)
}

pub fn f208(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f138({
        let a4783 = v0;
        let a4784 = v4;
        let a4785 = 0u64;
        nat_rem(a4783, a4784, a4785)
    }, {
        let a4786 = v1;
        let a4787 = v5;
        let a4788 = 0u64;
        nat_rem(a4786, a4787, a4788)
    }, {
        let a4789 = v2;
        let a4790 = v6;
        let a4791 = 0u64;
        nat_rem(a4789, a4790, a4791)
    }, {
        let a4792 = v3;
        let a4793 = v7;
        let a4794 = 0u64;
        nat_rem(a4792, a4793, a4794)
    }, v8, v9, v10, v11)
}

pub fn f209(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4795 = v1;
    let a4796 = 2147483648u64;
    if nat_lt(a4795, a4796) {
        let a4807 = {
            let a4805 = {
                let a4800 = {
                    let a4797 = v0;
                    let a4798 = v2;
                    let a4799 = 0u64;
                    nat_rem(a4797, a4798, a4799)
                };
                let a4801 = v2;
                nat_add(a4800, a4801)?
            };
            let a4806 = {
                let a4802 = v1;
                let a4803 = v2;
                let a4804 = 0u64;
                nat_rem(a4802, a4803, a4804)
            };
            nat_sub(a4805, a4806)
        };
        let a4808 = v2;
        let a4809 = 0u64;
        Ok(nat_rem(a4807, a4808, a4809))
    } else {
        let a4812 = {
            let a4810 = v0;
            let a4811 = f139(v1);
            nat_add(a4810, a4811)?
        };
        let a4813 = v2;
        let a4814 = 0u64;
        Ok(nat_rem(a4812, a4813, a4814))
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4817 = f140(v1);
    let a4818 = {
        let a4815 = v0;
        let a4816 = f139(v1);
        nat_add(a4815, a4816)?
    };
    if nat_le(a4817, a4818) {
        let a4819 = f211(v0, v1)?;
        let a4820 = v2;
        Ok(nat_lt(a4819, a4820))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: u64, v1: u64) -> R<u64> {
    let a4823 = {
        let a4821 = v0;
        let a4822 = f139(v1);
        nat_add(a4821, a4822)?
    };
    let a4824 = f140(v1);
    Ok(nat_sub(a4823, a4824))
}

pub fn f212(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4825 = v0;
    let a4826 = v1;
    if nat_lt(a4825, a4826) {
        let a4827 = v1;
        let a4828 = v0;
        Ok(nat_sub(a4827, a4828))
    } else {
        let a4831 = v0;
        let a4832 = {
            let a4829 = v1;
            let a4830 = v2;
            nat_add(a4829, a4830)?
        };
        if nat_lt(a4831, a4832) {
            let a4833 = v0;
            let a4834 = v1;
            Ok(nat_sub(a4833, a4834))
        } else {
            let a4841 = {
                let a4837 = 2u64;
                let a4838 = {
                    let a4835 = v2;
                    let a4836 = 1u64;
                    nat_sub(a4835, a4836)
                };
                nat_mul(a4837, a4838)?
            };
            let a4842 = {
                let a4839 = v0;
                let a4840 = v1;
                nat_sub(a4839, a4840)
            };
            Ok(nat_sub(a4841, a4842))
        }
    }
}

pub fn f213(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4845 = {
        let a4843 = v2;
        let a4844 = f239(v1.clone(), f72(v0, v1.clone(), 0u64), f72(v0, v1.clone(), 1u64), f72(v0, v1.clone(), 2u64), f72(v0, v1.clone(), 3u64), false)?;
        nat_add(a4843, a4844)?
    };
    let a4846 = f239(v1.clone(), f72(v0, v1.clone(), 0u64), f72(v0, v1.clone(), 1u64), f72(v0, v1.clone(), 2u64), f72(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4845, a4846))
}

pub fn f214(v0: u64, v1: u64, v2: u64) -> bool {
    let a4847 = v0;
    let a4848 = 0u64;
    if nat_eq(a4847, a4848) {
        let a4849 = v2;
        let a4850 = v1;
        nat_le(a4849, a4850)
    } else {
        let a4851 = v0;
        let a4852 = 1u64;
        if nat_eq(a4851, a4852) {
            let a4853 = v2;
            let a4854 = v1;
            nat_lt(a4853, a4854)
        } else {
            let a4855 = v0;
            let a4856 = 2u64;
            if nat_eq(a4855, a4856) {
                let a4857 = v1;
                let a4858 = v2;
                nat_le(a4857, a4858)
            } else {
                let a4859 = v0;
                let a4860 = 3u64;
                if nat_eq(a4859, a4860) {
                    let a4861 = v1;
                    let a4862 = v2;
                    nat_lt(a4861, a4862)
                } else {
                    let a4863 = v1;
                    let a4864 = v2;
                    nat_eq(a4863, a4864)
                }
            }
        }
    }
}

pub fn f215(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4865 = v0;
    let a4866 = 0u64;
    if nat_eq(a4865, a4866) {
        Ok(f226(0u64))
    } else {
        let a4869 = {
            let a4867 = v0;
            let a4868 = 1u64;
            nat_add(a4867, a4868)?
        };
        let a4870 = f135(v1.clone(), 13u64, v2)?;
        if nat_eq(a4869, a4870) {
            Ok(Adt9::C4({
                let a4871 = v2;
                let a4872 = 0u64;
                if nat_eq(a4871, a4872) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4873 = v0;
                let a4874 = 1u64;
                nat_sub(a4873, a4874)
            };
            if f203(v1.clone(), v2) {
                let v4: u64 = f204(match v1.clone() {
                    Adt3::C0(h4875, _, _, _, _, _, _, _) => {
                        h4875
                    }
                }, false)?;
                let v5: u64 = {
                    let a4876 = v3;
                    let a4877 = v4;
                    let a4878 = 0u64;
                    nat_quot(a4876, a4877, a4878)
                };
                let v6: u64 = {
                    let a4879 = v3;
                    let a4880 = v4;
                    let a4881 = 0u64;
                    nat_rem(a4879, a4880, a4881)
                };
                let a4884 = {
                    let a4882 = v6;
                    let a4883 = 1u64;
                    nat_add(a4882, a4883)?
                };
                let a4885 = v4;
                if nat_eq(a4884, a4885) {
                    Ok(Adt9::C4({
                        let a4886 = v2;
                        let a4887 = 0u64;
                        if nat_eq(a4886, a4887) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f216(v6, v5, v1, v2)
                }
            } else {
                f242({
                    let a4888 = v3;
                    let a4889 = 2u64;
                    let a4890 = 0u64;
                    nat_quot(a4888, a4889, a4890)
                }, {
                    let a4891 = v3;
                    let a4892 = 2u64;
                    let a4893 = 0u64;
                    nat_rem(a4891, a4892, a4893)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f216(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4894 = v0;
    let a4895 = 0u64;
    if nat_eq(a4894, a4895) {
        Ok(f226(1u64))
    } else {
        f242({
            let a4904 = {
                let a4897 = v1;
                let a4898 = match v2.clone() {
                    Adt3::C0(h4896, _, _, _, _, _, _, _) => {
                        h4896
                    }
                };
                nat_mul(a4897, a4898)?
            };
            let a4905 = {
                let a4901 = {
                    let a4899 = v0;
                    let a4900 = 1u64;
                    nat_sub(a4899, a4900)
                };
                let a4902 = 2u64;
                let a4903 = 0u64;
                nat_quot(a4901, a4902, a4903)
            };
            nat_add(a4904, a4905)?
        }, {
            let a4908 = {
                let a4906 = v0;
                let a4907 = 1u64;
                nat_sub(a4906, a4907)
            };
            let a4909 = 2u64;
            let a4910 = 0u64;
            nat_rem(a4908, a4909, a4910)
        }, v2, v3, 1u64)
    }
}

pub fn f217(v0: u64, v1: u64) -> R<u64> {
    let a4911 = v0;
    let a4912 = f98(v1)?;
    let a4913 = 0u64;
    Ok(nat_rem(a4911, a4912, a4913))
}

pub fn f218(v0: u64, v1: u64) -> R<u64> {
    let a4914 = 4u64;
    let a4915 = f243(v0, v1)?;
    nat_mul(a4914, a4915)
}

pub fn f219(v0: u64, v1: u64) -> R<u64> {
    let a4916 = 4u64;
    let a4917 = f244(v0, v1)?;
    nat_mul(a4916, a4917)
}

pub fn f220(v0: u64, v1: u64) -> R<bool> {
    let a4923 = {
        let a4920 = {
            let a4918 = f217(v0, v1)?;
            let a4919 = 2u64;
            nat_sub(a4918, a4919)
        };
        let a4921 = 2u64;
        let a4922 = 0u64;
        nat_rem(a4920, a4921, a4922)
    };
    let a4924 = 0u64;
    Ok(nat_eq(a4923, a4924))
}

pub fn f221(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4925 = f121(v0);
    match m4925 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f119(v0) {
                let a4926 = v1;
                let a4927 = v7;
                nat_lt(a4926, a4927)
            } else {
                false
            } {
                f168(v8, v2)
            } else {
                false
            } {
                let a4928 = v3;
                let a4929 = f245(v4, v5, v6)?;
                nat_lt(a4928, a4929)
            } else {
                false
            } {
                let a4934 = {
                    let a4931 = v7;
                    let a4932 = match v13.clone() {
                        Adt18::C0(_, h4930, _) => {
                            h4930
                        }
                    };
                    let a4933 = 0u64;
                    nat_rem(a4931, a4932, a4933)
                };
                let a4935 = 0u64;
                nat_eq(a4934, a4935)
            } else {
                false
            } {
                Ok(Some({
                    let a4964 = {
                        let a4954 = {
                            let a4944 = {
                                let a4940 = {
                                    let a4937 = v1;
                                    let a4938 = match v13 {
                                        Adt18::C0(_, h4936, _) => {
                                            h4936
                                        }
                                    };
                                    let a4939 = 0u64;
                                    nat_quot(a4937, a4938, a4939)
                                };
                                let a4941 = v9;
                                nat_mul(a4940, a4941)?
                            };
                            let a4945 = {
                                let a4942 = v2;
                                let a4943 = v10;
                                nat_mul(a4942, a4943)?
                            };
                            nat_add(a4944, a4945)?
                        };
                        let a4955 = {
                            let a4952 = {
                                let a4949 = {
                                    let a4946 = v3;
                                    let a4947 = v4;
                                    let a4948 = 0u64;
                                    nat_quot(a4946, a4947, a4948)
                                };
                                let a4950 = v5;
                                let a4951 = 0u64;
                                nat_rem(a4949, a4950, a4951)
                            };
                            let a4953 = v11;
                            nat_mul(a4952, a4953)?
                        };
                        nat_add(a4954, a4955)?
                    };
                    let a4965 = {
                        let a4962 = {
                            let a4959 = {
                                let a4956 = v3;
                                let a4957 = v4;
                                let a4958 = 0u64;
                                nat_quot(a4956, a4957, a4958)
                            };
                            let a4960 = v5;
                            let a4961 = 0u64;
                            nat_quot(a4959, a4960, a4961)
                        };
                        let a4963 = v12;
                        nat_mul(a4962, a4963)?
                    };
                    nat_add(a4964, a4965)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f222(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f246(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f223(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f247(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a4966 = v0;
    let a4967 = 64u64;
    if nat_eq(a4966, a4967) {
        f248(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a4968 = v0;
        let a4969 = 65u64;
        if nat_eq(a4968, a4969) {
            f249(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f225(v0: u64) -> u64 {
    let a4970 = v0;
    let a4971 = 30u64;
    if nat_eq(a4970, a4971) {
        4u64
    } else {
        3u64
    }
}

pub fn f226(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f227(v0: u64) -> bool {
    let a4972 = 0u64;
    let a4973 = v0;
    if nat_lt(a4972, a4973) {
        let a4974 = v0;
        let a4975 = 2139095040u64;
        nat_le(a4974, a4975)
    } else {
        false
    }
}

pub fn f228(v0: u64) -> u64 {
    let a4976 = v0;
    let a4977 = 40u64;
    if nat_lt(a4976, a4977) {
        let a4978 = v0;
        let a4979 = 20u64;
        if nat_lt(a4978, a4979) {
            let a4980 = v0;
            let a4981 = 10u64;
            if nat_lt(a4980, a4981) {
                let a4982 = v0;
                let a4983 = 5u64;
                if nat_lt(a4982, a4983) {
                    let a4984 = v0;
                    let a4985 = 2u64;
                    if nat_lt(a4984, a4985) {
                        let a4986 = v0;
                        let a4987 = 1u64;
                        if nat_lt(a4986, a4987) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4988 = v0;
                        let a4989 = 3u64;
                        if nat_lt(a4988, a4989) {
                            4u64
                        } else {
                            let a4990 = v0;
                            let a4991 = 4u64;
                            if nat_lt(a4990, a4991) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4992 = v0;
                    let a4993 = 7u64;
                    if nat_lt(a4992, a4993) {
                        let a4994 = v0;
                        let a4995 = 6u64;
                        if nat_lt(a4994, a4995) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4996 = v0;
                        let a4997 = 8u64;
                        if nat_lt(a4996, a4997) {
                            128u64
                        } else {
                            let a4998 = v0;
                            let a4999 = 9u64;
                            if nat_lt(a4998, a4999) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5000 = v0;
                let a5001 = 15u64;
                if nat_lt(a5000, a5001) {
                    let a5002 = v0;
                    let a5003 = 12u64;
                    if nat_lt(a5002, a5003) {
                        let a5004 = v0;
                        let a5005 = 11u64;
                        if nat_lt(a5004, a5005) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5006 = v0;
                        let a5007 = 13u64;
                        if nat_lt(a5006, a5007) {
                            4096u64
                        } else {
                            let a5008 = v0;
                            let a5009 = 14u64;
                            if nat_lt(a5008, a5009) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5010 = v0;
                    let a5011 = 17u64;
                    if nat_lt(a5010, a5011) {
                        let a5012 = v0;
                        let a5013 = 16u64;
                        if nat_lt(a5012, a5013) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5014 = v0;
                        let a5015 = 18u64;
                        if nat_lt(a5014, a5015) {
                            131072u64
                        } else {
                            let a5016 = v0;
                            let a5017 = 19u64;
                            if nat_lt(a5016, a5017) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5018 = v0;
            let a5019 = 30u64;
            if nat_lt(a5018, a5019) {
                let a5020 = v0;
                let a5021 = 25u64;
                if nat_lt(a5020, a5021) {
                    let a5022 = v0;
                    let a5023 = 22u64;
                    if nat_lt(a5022, a5023) {
                        let a5024 = v0;
                        let a5025 = 21u64;
                        if nat_lt(a5024, a5025) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5026 = v0;
                        let a5027 = 23u64;
                        if nat_lt(a5026, a5027) {
                            4194304u64
                        } else {
                            let a5028 = v0;
                            let a5029 = 24u64;
                            if nat_lt(a5028, a5029) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5030 = v0;
                    let a5031 = 27u64;
                    if nat_lt(a5030, a5031) {
                        let a5032 = v0;
                        let a5033 = 26u64;
                        if nat_lt(a5032, a5033) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5034 = v0;
                        let a5035 = 28u64;
                        if nat_lt(a5034, a5035) {
                            134217728u64
                        } else {
                            let a5036 = v0;
                            let a5037 = 29u64;
                            if nat_lt(a5036, a5037) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5038 = v0;
                let a5039 = 35u64;
                if nat_lt(a5038, a5039) {
                    let a5040 = v0;
                    let a5041 = 32u64;
                    if nat_lt(a5040, a5041) {
                        let a5042 = v0;
                        let a5043 = 31u64;
                        if nat_lt(a5042, a5043) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5044 = v0;
                        let a5045 = 33u64;
                        if nat_lt(a5044, a5045) {
                            4294967296u64
                        } else {
                            let a5046 = v0;
                            let a5047 = 34u64;
                            if nat_lt(a5046, a5047) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5048 = v0;
                    let a5049 = 37u64;
                    if nat_lt(a5048, a5049) {
                        let a5050 = v0;
                        let a5051 = 36u64;
                        if nat_lt(a5050, a5051) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5052 = v0;
                        let a5053 = 38u64;
                        if nat_lt(a5052, a5053) {
                            137438953472u64
                        } else {
                            let a5054 = v0;
                            let a5055 = 39u64;
                            if nat_lt(a5054, a5055) {
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

pub fn f229(v0: u64, v1: u64) -> bool {
    let a5056 = v0;
    let a5057 = 1u64;
    if nat_le(a5056, a5057) {
        true
    } else {
        let a5061 = {
            let a5058 = v1;
            let a5059 = 4u64;
            let a5060 = 0u64;
            nat_rem(a5058, a5059, a5060)
        };
        let a5062 = 0u64;
        nat_eq(a5061, a5062)
    }
}

pub fn f230(v0: u64, v1: u64) -> bool {
    let a5063 = v0;
    let a5064 = 1u64;
    if nat_le(a5063, a5064) {
        true
    } else {
        let a5065 = 4u64;
        let a5066 = v1;
        nat_le(a5065, a5066)
    }
}

pub fn f231(v0: u64, v1: u64) -> R<u64> {
    let a5067 = v0;
    let a5068 = 0u64;
    if nat_eq(a5067, a5068) {
        Ok(0u64)
    } else {
        let a5073 = 4u64;
        let a5074 = {
            let a5071 = {
                let a5069 = v0;
                let a5070 = 1u64;
                nat_sub(a5069, a5070)
            };
            let a5072 = v1;
            nat_mul(a5071, a5072)?
        };
        nat_add(a5073, a5074)
    }
}

pub fn f232(v0: bool, v1: u64) -> R<u64> {
    let a5079 = {
        let a5077 = {
            let a5075 = f250(v0, v1)?;
            let a5076 = f251(v0, v1)?;
            nat_add(a5075, a5076)?
        };
        let a5078 = f252(v0);
        nat_add(a5077, a5078)?
    };
    let a5080 = v1;
    nat_add(a5079, a5080)
}

pub fn f233(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f253(v0, v1)?;
    let v3: u64 = {
        let a5085 = {
            let a5083 = {
                let a5081 = v1;
                let a5082 = 1u64;
                nat_sub(a5081, a5082)
            };
            let a5084 = v2;
            nat_mul(a5083, a5084)?
        };
        let a5086 = v0;
        nat_sub(a5085, a5086)
    };
    let a5087 = v2;
    let a5088 = v3;
    if nat_lt(a5087, a5088) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f234(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5089 = v4;
    let a5090 = 0u64;
    if nat_eq(a5089, a5090) {
        Ok(0u64)
    } else {
        let a5097 = 2u64;
        let a5098 = {
            let a5095 = {
                let a5091 = v0;
                let a5092 = v1;
                nat_mul(a5091, a5092)?
            };
            let a5096 = {
                let a5093 = v2;
                let a5094 = v3;
                nat_mul(a5093, a5094)?
            };
            nat_mul(a5095, a5096)?
        };
        nat_mul(a5097, a5098)
    }
}

pub fn f235(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5103 = v3;
    let a5104 = {
        let a5101 = {
            let a5099 = f254(v2, v4)?;
            let a5100 = f255(v0, v1, f253(v2, v4)?)?;
            nat_mul(a5099, a5100)?
        };
        let a5102 = f255(v0, v1, f256(v2, v4)?)?;
        nat_add(a5101, a5102)?
    };
    nat_mul(a5103, a5104)
}

pub fn f236(v0: u64) -> R<u64> {
    let a5107 = 13u64;
    let a5108 = {
        let a5105 = 9u64;
        let a5106 = v0;
        nat_mul(a5105, a5106)?
    };
    nat_add(a5107, a5108)
}

pub fn f237(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f199(v1, v2, v3, v4) {
        f257(v0)
    } else {
        f119(v0)
    }
}

pub fn f238(v0: u64) -> u64 {
    let a5109 = v0;
    let a5110 = 26u64;
    if nat_eq(a5109, a5110) {
        26u64
    } else {
        0u64
    }
}

pub fn f239(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5131 = {
        let a5125 = {
            let a5119 = {
                let a5113 = v1;
                let a5114 = if v5 {
                    f139(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5111, _, _, _) => {
                            h5111
                        }
                    })
                } else {
                    f140(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5112, _, _, _) => {
                            h5112
                        }
                    })
                };
                nat_mul(a5113, a5114)?
            };
            let a5120 = {
                let a5117 = v2;
                let a5118 = if v5 {
                    f139(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5115, _, _) => {
                            h5115
                        }
                    })
                } else {
                    f140(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5116, _, _) => {
                            h5116
                        }
                    })
                };
                nat_mul(a5117, a5118)?
            };
            nat_add(a5119, a5120)?
        };
        let a5126 = {
            let a5123 = v3;
            let a5124 = if v5 {
                f139(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5121, _) => {
                        h5121
                    }
                })
            } else {
                f140(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5122, _) => {
                        h5122
                    }
                })
            };
            nat_mul(a5123, a5124)?
        };
        nat_add(a5125, a5126)?
    };
    let a5132 = {
        let a5129 = v4;
        let a5130 = if v5 {
            f139(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5127) => {
                    h5127
                }
            })
        } else {
            f140(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5128) => {
                    h5128
                }
            })
        };
        nat_mul(a5129, a5130)?
    };
    nat_add(a5131, a5132)
}

pub fn f240(v0: u64, v1: u64) -> Option<u64> {
    let a5133 = v0;
    let a5134 = 2u64;
    if nat_eq(a5133, a5134) {
        Some(0u64)
    } else {
        let a5135 = v0;
        let a5136 = 5u64;
        if nat_eq(a5135, a5136) {
            Some(1u64)
        } else {
            let a5137 = v0;
            let a5138 = 6u64;
            if nat_eq(a5137, a5138) {
                Some(2u64)
            } else {
                let a5139 = v0;
                let a5140 = 7u64;
                if nat_eq(a5139, a5140) {
                    Some(3u64)
                } else {
                    let a5141 = v0;
                    let a5142 = 9u64;
                    if nat_eq(a5141, a5142) {
                        Some(4u64)
                    } else {
                        let a5143 = v0;
                        let a5144 = 8u64;
                        if nat_eq(a5143, a5144) {
                            Some(5u64)
                        } else {
                            let a5145 = v0;
                            let a5146 = 89u64;
                            if nat_eq(a5145, a5146) {
                                let a5147 = v1;
                                let a5148 = 2u64;
                                if nat_eq(a5147, a5148) {
                                    Some(6u64)
                                } else {
                                    let a5149 = v1;
                                    let a5150 = 0u64;
                                    if nat_eq(a5149, a5150) {
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

pub fn f241(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5151 = v0;
    let a5152 = 30u64;
    if nat_eq(a5151, a5152) {
        let a5153 = v2;
        let a5154 = 0u64;
        if if if nat_eq(a5153, a5154) {
            true
        } else {
            let a5155 = v2;
            let a5156 = 1u64;
            nat_eq(a5155, a5156)
        } {
            let a5157 = v1;
            let a5158 = 0u64;
            if nat_eq(a5157, a5158) {
                true
            } else {
                f258(v1, v4)
            }
        } else {
            false
        } {
            f259(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f260(v1) {
            f260(v2)
        } else {
            false
        } {
            let a5159 = v1;
            let a5160 = 0u64;
            if if nat_eq(a5159, a5160) {
                true
            } else {
                let a5161 = v2;
                let a5162 = 0u64;
                nat_eq(a5161, a5162)
            } {
                true
            } else {
                let a5163 = v1;
                let a5164 = v2;
                nat_eq(a5163, a5164)
            }
        } else {
            false
        } {
            true
        } else {
            let a5165 = v1;
            let a5166 = 43u64;
            if nat_eq(a5165, a5166) {
                f260(v2)
            } else {
                false
            }
        } {
            f259(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f242(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5167 = v1;
    let a5168 = 0u64;
    if nat_eq(a5167, a5168) {
        Ok(Adt9::C4({
            let a5169 = v3;
            let a5170 = 0u64;
            if nat_eq(a5169, a5170) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f146(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5171 = v3;
            let a5172 = 0u64;
            if nat_eq(a5171, a5172) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f243(v0: u64, v1: u64) -> R<u64> {
    let a5173 = v0;
    let a5174 = f98(v1)?;
    let a5175 = 0u64;
    Ok(nat_quot(a5173, a5174, a5175))
}

pub fn f244(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f243(v0, v1)?;
    let v3: u64 = {
        let a5178 = {
            let a5176 = f217(v0, v1)?;
            let a5177 = 2u64;
            nat_sub(a5176, a5177)
        };
        let a5179 = 2u64;
        let a5180 = 0u64;
        nat_quot(a5178, a5179, a5180)
    };
    let v4: u64 = {
        let a5181 = v2;
        let a5182 = v1;
        nat_mul(a5181, a5182)?
    };
    if f220(v0, v1)? {
        let a5183 = v4;
        let a5184 = v3;
        nat_add(a5183, a5184)
    } else {
        let a5196 = {
            let a5192 = v4;
            let a5193 = {
                let a5190 = {
                    let a5185 = v1;
                    let a5186 = 1u64;
                    nat_sub(a5185, a5186)
                };
                let a5191 = {
                    let a5187 = v2;
                    let a5188 = v1;
                    let a5189 = 0u64;
                    nat_rem(a5187, a5188, a5189)
                };
                nat_mul(a5190, a5191)?
            };
            nat_sub(a5192, a5193)
        };
        let a5197 = {
            let a5194 = v3;
            let a5195 = v1;
            nat_mul(a5194, a5195)?
        };
        nat_add(a5196, a5197)
    }
}

pub fn f245(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5198 = v0;
    let a5199 = 0u64;
    if if if nat_eq(a5198, a5199) {
        true
    } else {
        let a5200 = v1;
        let a5201 = 0u64;
        nat_eq(a5200, a5201)
    } {
        true
    } else {
        let a5202 = v2;
        let a5203 = 0u64;
        nat_eq(a5202, a5203)
    } {
        Ok(0u64)
    } else {
        let a5206 = {
            let a5204 = v0;
            let a5205 = v1;
            nat_mul(a5204, a5205)?
        };
        let a5207 = v2;
        nat_mul(a5206, a5207)
    }
}

pub fn f246(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f107(match v1.clone() {
        Adt20::C0(h5208, _, _, _, _, _, _, _) => {
            h5208
        }
    })?;
    let v6: u64 = {
        let a5214 = match v1.clone() {
            Adt20::C0(_, h5209, _, _, _, _, _, _) => {
                h5209
            }
        };
        let a5215 = {
            let a5212 = match v1.clone() {
                Adt20::C0(_, _, h5210, _, _, _, _, _) => {
                    h5210
                }
            };
            let a5213 = match v1.clone() {
                Adt20::C0(_, _, _, h5211, _, _, _, _) => {
                    h5211
                }
            };
            nat_mul(a5212, a5213)?
        };
        nat_mul(a5214, a5215)?
    };
    let a5217 = v0;
    let a5218 = f39(match v1.clone() {
        Adt20::C0(h5216, _, _, _, _, _, _, _) => {
            h5216
        }
    }, v6)?;
    if nat_lt(a5217, a5218) {
        f261({
            let a5219 = v0;
            let a5220 = v5;
            let a5221 = 0u64;
            nat_rem(a5219, a5220, a5221)
        }, {
            let a5222 = v0;
            let a5223 = v5;
            let a5224 = 0u64;
            nat_quot(a5222, a5223, a5224)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f247(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5225 = v6;
        let a5226 = 6u64;
        nat_sub(a5225, a5226)
    };
    let v9: Adt20 = f262(v3);
    let a5228 = v0;
    let a5229 = f195(match v1.clone() {
        Adt20::C0(h5227, _, _, _, _, _, _, _) => {
            h5227
        }
    }, v5, v8)?;
    if nat_lt(a5228, a5229) {
        let v10: u64 = f104(match v1.clone() {
            Adt20::C0(h5230, _, _, _, _, _, _, _) => {
                h5230
            }
        }, v5, v8)?;
        let a5231 = v0;
        let a5232 = v10;
        if nat_lt(a5231, a5232) {
            f263(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5233 = v0;
                let a5234 = v10;
                nat_sub(a5233, a5234)
            };
            let v12: u64 = f236(match v1.clone() {
                Adt20::C0(h5235, _, _, _, _, _, _, _) => {
                    h5235
                }
            })?;
            f264({
                let a5236 = v11;
                let a5237 = v12;
                let a5238 = 0u64;
                nat_rem(a5236, a5237, a5238)
            }, {
                let a5239 = v11;
                let a5240 = v12;
                let a5241 = 0u64;
                nat_quot(a5239, a5240, a5241)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f248(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5242 = v5;
        let a5243 = 6u64;
        nat_sub(a5242, a5243)
    };
    let v8: bool = f189(match v1.clone() {
        Adt20::C0(_, _, _, _, h5244, _, _, _) => {
            h5244
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5245, _, _, _) => {
            h5245
        }
    });
    let a5247 = v0;
    let a5248 = f188(v8, match v1.clone() {
        Adt20::C0(h5246, _, _, _, _, _, _, _) => {
            h5246
        }
    }, v4, v7)?;
    if nat_lt(a5247, a5248) {
        let v9: u64 = f104(match v1.clone() {
            Adt20::C0(h5249, _, _, _, _, _, _, _) => {
                h5249
            }
        }, v4, v7)?;
        let a5250 = v0;
        let a5251 = v9;
        if nat_lt(a5250, a5251) {
            f263(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5252 = v0;
                let a5253 = v9;
                nat_sub(a5252, a5253)
            };
            let v11: u64 = f232(v8, match v1.clone() {
                Adt20::C0(h5254, _, _, _, _, _, _, _) => {
                    h5254
                }
            })?;
            f265({
                let a5255 = v10;
                let a5256 = v11;
                let a5257 = 0u64;
                nat_rem(a5255, a5256, a5257)
            }, {
                let a5258 = v10;
                let a5259 = v11;
                let a5260 = 0u64;
                nat_quot(a5258, a5259, a5260)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f249(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5261 = v5;
        let a5262 = 6u64;
        nat_sub(a5261, a5262)
    };
    let v8: Adt20 = f262(v2);
    let v9: u64 = f234(match v1.clone() {
        Adt20::C0(h5263, _, _, _, _, _, _, _) => {
            h5263
        }
    }, match v1.clone() {
        Adt20::C0(_, h5264, _, _, _, _, _, _) => {
            h5264
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5265, _, _, _, _, _) => {
            h5265
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5266, _, _, _, _) => {
            h5266
        }
    }, v7)?;
    let a5271 = v0;
    let a5272 = f194(match v1.clone() {
        Adt20::C0(h5267, _, _, _, _, _, _, _) => {
            h5267
        }
    }, match v1.clone() {
        Adt20::C0(_, h5268, _, _, _, _, _, _) => {
            h5268
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5269, _, _, _, _, _) => {
            h5269
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5270, _, _, _, _) => {
            h5270
        }
    }, v4, v7)?;
    if nat_lt(a5271, a5272) {
        let a5273 = v0;
        let a5274 = v9;
        if nat_lt(a5273, a5274) {
            f263(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f266({
                let a5275 = v0;
                let a5276 = v9;
                nat_sub(a5275, a5276)
            }, match v1.clone() {
                Adt20::C0(h5277, _, _, _, _, _, _, _) => {
                    h5277
                }
            }, match v1.clone() {
                Adt20::C0(_, h5278, _, _, _, _, _, _) => {
                    h5278
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5279, _, _, _, _, _) => {
                    h5279
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5280, _, _, _, _) => {
                    h5280
                }
            }, v4)?;
            f267(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f250(v0: bool, v1: u64) -> R<u64> {
    let a5283 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5284 = {
        let a5281 = f268(v0);
        let a5282 = v1;
        nat_mul(a5281, a5282)?
    };
    nat_add(a5283, a5284)
}

pub fn f251(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5292 = {
            let a5285 = 13u64;
            let a5286 = f269(v1);
            nat_mul(a5285, a5286)?
        };
        let a5293 = {
            let a5290 = 4u64;
            let a5291 = {
                let a5287 = v1;
                let a5288 = 4u64;
                let a5289 = 0u64;
                nat_rem(a5287, a5288, a5289)
            };
            nat_mul(a5290, a5291)?
        };
        nat_add(a5292, a5293)
    } else {
        let a5294 = 3u64;
        let a5295 = v1;
        nat_mul(a5294, a5295)
    }
}

pub fn f252(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f253(v0: u64, v1: u64) -> R<u64> {
    let a5300 = {
        let a5298 = {
            let a5296 = v0;
            let a5297 = v1;
            nat_add(a5296, a5297)?
        };
        let a5299 = 1u64;
        nat_sub(a5298, a5299)
    };
    let a5301 = v1;
    let a5302 = 0u64;
    Ok(nat_quot(a5300, a5301, a5302))
}

pub fn f254(v0: u64, v1: u64) -> R<u64> {
    let a5303 = v0;
    let a5304 = f253(v0, v1)?;
    let a5305 = 0u64;
    Ok(nat_quot(a5303, a5304, a5305))
}

pub fn f255(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5306 = v2;
    let a5307 = 0u64;
    if nat_eq(a5306, a5307) {
        Ok(0u64)
    } else {
        let a5312 = 11u64;
        let a5313 = {
            let a5310 = {
                let a5308 = v1;
                let a5309 = v2;
                nat_mul(a5308, a5309)?
            };
            let a5311 = f270(v0)?;
            nat_mul(a5310, a5311)?
        };
        nat_add(a5312, a5313)
    }
}

pub fn f256(v0: u64, v1: u64) -> R<u64> {
    let a5314 = v0;
    let a5315 = f253(v0, v1)?;
    let a5316 = 0u64;
    Ok(nat_rem(a5314, a5315, a5316))
}

pub fn f257(v0: u64) -> bool {
    let m5317 = f121(v0);
    match m5317 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f258(v0: u64, v1: u64) -> bool {
    let a5318 = v0;
    let a5319 = 1u64;
    if nat_eq(a5318, a5319) {
        true
    } else {
        let v2: u64 = f271(v0);
        let a5320 = 0u64;
        let a5321 = v2;
        if nat_lt(a5320, a5321) {
            let a5325 = {
                let a5322 = v1;
                let a5323 = v2;
                let a5324 = 1u64;
                nat_rem(a5322, a5323, a5324)
            };
            let a5326 = 0u64;
            nat_eq(a5325, a5326)
        } else {
            false
        }
    }
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5327 = v0;
    let a5328 = 28u64;
    if if if if nat_eq(a5327, a5328) {
        let a5329 = v1;
        let a5330 = 0u64;
        nat_eq(a5329, a5330)
    } else {
        false
    } {
        let a5331 = v2;
        let a5332 = 0u64;
        nat_eq(a5331, a5332)
    } else {
        false
    } {
        let a5333 = v3;
        let a5334 = 0u64;
        nat_eq(a5333, a5334)
    } else {
        false
    } {
        let a5335 = v4;
        let a5336 = v8;
        if if if if if if if if nat_eq(a5335, a5336) {
            let a5337 = v5;
            let a5338 = v12;
            nat_eq(a5337, a5338)
        } else {
            false
        } {
            let a5339 = v9;
            let a5340 = v13;
            nat_eq(a5339, a5340)
        } else {
            false
        } {
            let a5341 = v10;
            let a5342 = v14;
            nat_eq(a5341, a5342)
        } else {
            false
        } {
            let a5343 = v11;
            let a5344 = v15;
            nat_eq(a5343, a5344)
        } else {
            false
        } {
            let a5345 = 0u64;
            let a5346 = v6;
            nat_lt(a5345, a5346)
        } else {
            false
        } {
            let a5347 = 0u64;
            let a5348 = v7;
            nat_lt(a5347, a5348)
        } else {
            false
        } {
            let a5352 = {
                let a5349 = v10;
                let a5350 = v6;
                let a5351 = 1u64;
                nat_rem(a5349, a5350, a5351)
            };
            let a5353 = 0u64;
            if nat_eq(a5352, a5353) {
                let a5357 = {
                    let a5354 = v11;
                    let a5355 = v7;
                    let a5356 = 1u64;
                    nat_rem(a5354, a5355, a5356)
                };
                let a5358 = 0u64;
                nat_eq(a5357, a5358)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5359 = v10;
                let a5360 = v6;
                let a5361 = 0u64;
                nat_quot(a5359, a5360, a5361)
            }, {
                let a5362 = v11;
                let a5363 = v7;
                let a5364 = 0u64;
                nat_quot(a5362, a5363, a5364)
            }, {
                let a5365 = v5;
                let a5366 = v4;
                nat_mul(a5365, a5366)?
            }, {
                let a5367 = v4;
                let a5368 = v9;
                nat_mul(a5367, a5368)?
            }, {
                let a5369 = v5;
                let a5370 = v9;
                nat_mul(a5369, a5370)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f260(v0: u64) -> bool {
    let a5371 = v0;
    let a5372 = 0u64;
    if if nat_eq(a5371, a5372) {
        true
    } else {
        let a5373 = v0;
        let a5374 = 1u64;
        nat_eq(a5373, a5374)
    } {
        true
    } else {
        let a5375 = v0;
        let a5376 = 30u64;
        nat_eq(a5375, a5376)
    }
}

pub fn f261(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5377, _, _, _, _, _, _, _) => {
            h5377
        }
    };
    let v7: u64 = {
        let a5380 = 1u64;
        let a5381 = {
            let a5378 = 3u64;
            let a5379 = v6;
            nat_mul(a5378, a5379)?
        };
        nat_add(a5380, a5381)?
    };
    let a5382 = v0;
    let a5383 = 0u64;
    if nat_eq(a5382, a5383) {
        Ok(f272(0u64))
    } else {
        let a5384 = v0;
        let a5385 = v7;
        if nat_lt(a5384, a5385) {
            let v8: u64 = {
                let a5386 = v0;
                let a5387 = 1u64;
                nat_sub(a5386, a5387)
            };
            let v9: u64 = {
                let a5393 = {
                    let a5388 = v1;
                    let a5389 = v6;
                    nat_mul(a5388, a5389)?
                };
                let a5394 = {
                    let a5390 = v8;
                    let a5391 = 3u64;
                    let a5392 = 0u64;
                    nat_quot(a5390, a5391, a5392)
                };
                nat_add(a5393, a5394)?
            };
            let v10: u64 = f273(v9, v2)?;
            let v11: u64 = {
                let a5395 = v8;
                let a5396 = 3u64;
                let a5397 = 0u64;
                nat_rem(a5395, a5396, a5397)
            };
            let a5398 = v11;
            let a5399 = 0u64;
            if nat_eq(a5398, a5399) {
                Ok(f274(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5400 = v11;
                let a5401 = 1u64;
                if nat_eq(a5400, a5401) {
                    Ok(f274(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f274(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5402 = v0;
            let a5403 = v7;
            if nat_eq(a5402, a5403) {
                Ok(f274(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5406 = v0;
                let a5407 = {
                    let a5404 = v7;
                    let a5405 = 1u64;
                    nat_add(a5404, a5405)?
                };
                if nat_eq(a5406, a5407) {
                    Ok(f274(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5410 = v0;
                    let a5411 = {
                        let a5408 = v7;
                        let a5409 = 2u64;
                        nat_add(a5408, a5409)?
                    };
                    if nat_eq(a5410, a5411) {
                        Ok(f274(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5414 = v0;
                        let a5415 = {
                            let a5412 = v7;
                            let a5413 = 3u64;
                            nat_add(a5412, a5413)?
                        };
                        if nat_eq(a5414, a5415) {
                            let a5416 = v5;
                            let a5417 = 1u64;
                            if if nat_eq(a5416, a5417) {
                                true
                            } else {
                                let a5418 = v5;
                                let a5419 = 2u64;
                                nat_eq(a5418, a5419)
                            } {
                                Ok(f274(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f274(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5426 = {
                                    let a5420 = v1;
                                    let a5421 = v6;
                                    nat_mul(a5420, a5421)?
                                };
                                let a5427 = {
                                    let a5424 = v0;
                                    let a5425 = {
                                        let a5422 = v7;
                                        let a5423 = 4u64;
                                        nat_add(a5422, a5423)?
                                    };
                                    nat_sub(a5424, a5425)
                                };
                                nat_add(a5426, a5427)?
                            };
                            Ok(f274(10u64, 0u64, f273(v12, v2)?, 1u64, 2u64, 3u64, f273(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f262(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5428, _, _, _, _, _, _, _) => {
            h5428
        }
    }, match v0.clone() {
        Adt20::C0(_, h5429, _, _, _, _, _, _) => {
            h5429
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5430, _, _, _, _, _) => {
            h5430
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5431, _, _, _, _) => {
            h5431
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5432, _, _) => {
            h5432
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5433, _) => {
            h5433
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5434) => {
            h5434
        }
    })
}

pub fn f263(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5441 = v3;
        let a5442 = {
            let a5438 = f273({
                let a5435 = v0;
                let a5436 = 2u64;
                let a5437 = 0u64;
                nat_quot(a5435, a5436, a5437)
            }, v1)?;
            let a5439 = v2;
            let a5440 = 0u64;
            nat_quot(a5438, a5439, a5440)
        };
        nat_add(a5441, a5442)?
    };
    let a5446 = {
        let a5443 = v0;
        let a5444 = 2u64;
        let a5445 = 0u64;
        nat_rem(a5443, a5444, a5445)
    };
    let a5447 = 0u64;
    if nat_eq(a5446, a5447) {
        Ok(f274(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5448 = v4;
        let a5449 = 0u64;
        if nat_eq(a5448, a5449) {
            Ok(f274(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f264(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f275(match v2.clone() {
        Adt20::C0(h5450, _, _, _, _, _, _, _) => {
            h5450
        }
    })?;
    let v7: u64 = f276(match v2.clone() {
        Adt20::C0(h5451, _, _, _, _, _, _, _) => {
            h5451
        }
    })?;
    let a5452 = v0;
    let a5453 = 2u64;
    if nat_lt(a5452, a5453) {
        Ok(f272(v0))
    } else {
        let a5454 = v0;
        let a5455 = v6;
        if nat_lt(a5454, a5455) {
            f277({
                let a5456 = v0;
                let a5457 = 2u64;
                nat_sub(a5456, a5457)
            }, v1, v2, v3)
        } else {
            let a5458 = v0;
            let a5459 = v7;
            if nat_lt(a5458, a5459) {
                Ok(f278({
                    let a5460 = v0;
                    let a5461 = v6;
                    nat_sub(a5460, a5461)
                }, match v2 {
                    Adt20::C0(h5462, _, _, _, _, _, _, _) => {
                        h5462
                    }
                }, v5))
            } else {
                let a5464 = v0;
                let a5465 = f236(match v2.clone() {
                    Adt20::C0(h5463, _, _, _, _, _, _, _) => {
                        h5463
                    }
                })?;
                if nat_lt(a5464, a5465) {
                    f279({
                        let a5466 = v0;
                        let a5467 = v7;
                        nat_sub(a5466, a5467)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f265(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f250(v5, match v2.clone() {
        Adt20::C0(h5468, _, _, _, _, _, _, _) => {
            h5468
        }
    })?;
    let v7: u64 = {
        let a5470 = v6;
        let a5471 = f251(v5, match v2.clone() {
            Adt20::C0(h5469, _, _, _, _, _, _, _) => {
                h5469
            }
        })?;
        nat_add(a5470, a5471)?
    };
    let v8: u64 = {
        let a5472 = v7;
        let a5473 = f252(v5);
        nat_add(a5472, a5473)?
    };
    let a5474 = v0;
    let a5475 = v6;
    if nat_lt(a5474, a5475) {
        f280(v0, v1, v2, v5)
    } else {
        let a5476 = v0;
        let a5477 = v7;
        if nat_lt(a5476, a5477) {
            f281({
                let a5478 = v0;
                let a5479 = v6;
                nat_sub(a5478, a5479)
            }, v1, v2, v3, v5)
        } else {
            let a5480 = v0;
            let a5481 = v8;
            if nat_lt(a5480, a5481) {
                Ok(f282({
                    let a5482 = v0;
                    let a5483 = v7;
                    nat_sub(a5482, a5483)
                }, match v2 {
                    Adt20::C0(h5484, _, _, _, _, _, _, _) => {
                        h5484
                    }
                }, v4, v5))
            } else {
                let a5488 = v0;
                let a5489 = {
                    let a5486 = v8;
                    let a5487 = match v2.clone() {
                        Adt20::C0(h5485, _, _, _, _, _, _, _) => {
                            h5485
                        }
                    };
                    nat_add(a5486, a5487)?
                };
                if nat_lt(a5488, a5489) {
                    let v9: u64 = f273({
                        let a5495 = {
                            let a5491 = v1;
                            let a5492 = match v2 {
                                Adt20::C0(h5490, _, _, _, _, _, _, _) => {
                                    h5490
                                }
                            };
                            nat_mul(a5491, a5492)?
                        };
                        let a5496 = {
                            let a5493 = v0;
                            let a5494 = v8;
                            nat_sub(a5493, a5494)
                        };
                        nat_add(a5495, a5496)?
                    }, v3)?;
                    Ok(f274(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f266(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f253(v3, v5)?;
    let v7: u64 = f255(v1, v2, v6)?;
    let v8: u64 = {
        let a5499 = {
            let a5497 = f254(v3, v5)?;
            let a5498 = v4;
            nat_mul(a5497, a5498)?
        };
        let a5500 = v7;
        nat_mul(a5499, a5500)?
    };
    let a5501 = v0;
    let a5502 = v8;
    if nat_lt(a5501, a5502) {
        let v9: u64 = {
            let a5503 = v0;
            let a5504 = v7;
            let a5505 = 0u64;
            nat_quot(a5503, a5504, a5505)
        };
        Ok(Adt22::C0({
            let a5509 = {
                let a5506 = v9;
                let a5507 = v4;
                let a5508 = 0u64;
                nat_quot(a5506, a5507, a5508)
            };
            let a5510 = v6;
            nat_mul(a5509, a5510)?
        }, v6, {
            let a5511 = v9;
            let a5512 = v4;
            let a5513 = 0u64;
            nat_rem(a5511, a5512, a5513)
        }, {
            let a5514 = v0;
            let a5515 = v7;
            let a5516 = 0u64;
            nat_rem(a5514, a5515, a5516)
        }))
    } else {
        let v10: u64 = {
            let a5517 = v0;
            let a5518 = v8;
            nat_sub(a5517, a5518)
        };
        let v11: u64 = f255(v1, v2, f256(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5519 = f254(v3, v5)?;
            let a5520 = v6;
            nat_mul(a5519, a5520)?
        }, f256(v3, v5)?, {
            let a5521 = v10;
            let a5522 = v11;
            let a5523 = 0u64;
            nat_quot(a5521, a5522, a5523)
        }, {
            let a5524 = v10;
            let a5525 = v11;
            let a5526 = 0u64;
            nat_rem(a5524, a5525, a5526)
        }))
    }
}

pub fn f267(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5527) => {
            h5527
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5528, _, _, _, _, _, _, _) => {
            h5528
        }
    };
    let v6: u64 = {
        let a5531 = match v0.clone() {
            Adt22::C0(_, h5529, _, _) => {
                h5529
            }
        };
        let a5532 = match v1.clone() {
            Adt20::C0(_, h5530, _, _, _, _, _, _) => {
                h5530
            }
        };
        nat_mul(a5531, a5532)?
    };
    let v7: u64 = {
        let a5533 = v6;
        let a5534 = v5;
        nat_mul(a5533, a5534)?
    };
    let v8: u64 = {
        let a5537 = {
            let a5535 = 2u64;
            let a5536 = v5;
            nat_mul(a5535, a5536)?
        };
        let a5538 = 2u64;
        nat_add(a5537, a5538)?
    };
    let v9: u64 = {
        let a5541 = {
            let a5539 = 4u64;
            let a5540 = v5;
            nat_mul(a5539, a5540)?
        };
        let a5542 = 2u64;
        nat_add(a5541, a5542)?
    };
    let v10: u64 = {
        let a5545 = 1u64;
        let a5546 = {
            let a5543 = v6;
            let a5544 = v8;
            nat_mul(a5543, a5544)?
        };
        nat_add(a5545, a5546)?
    };
    let v11: u64 = {
        let a5547 = v10;
        let a5548 = 4u64;
        nat_add(a5547, a5548)?
    };
    let v12: u64 = {
        let a5551 = v11;
        let a5552 = {
            let a5549 = v6;
            let a5550 = v9;
            nat_mul(a5549, a5550)?
        };
        nat_add(a5551, a5552)?
    };
    let v13: u64 = {
        let a5553 = v12;
        let a5554 = 6u64;
        nat_add(a5553, a5554)?
    };
    let a5555 = v4;
    let a5556 = 0u64;
    if nat_eq(a5555, a5556) {
        Ok(f272(0u64))
    } else {
        let a5557 = v4;
        let a5558 = v10;
        if nat_lt(a5557, a5558) {
            let v14: u64 = {
                let a5559 = v4;
                let a5560 = 1u64;
                nat_sub(a5559, a5560)
            };
            f283(false, {
                let a5561 = v14;
                let a5562 = v8;
                let a5563 = 0u64;
                nat_rem(a5561, a5562, a5563)
            }, {
                let a5564 = v14;
                let a5565 = v8;
                let a5566 = 0u64;
                nat_quot(a5564, a5565, a5566)
            }, v0, v1, v2)
        } else {
            let a5567 = v4;
            let a5568 = v11;
            if nat_lt(a5567, a5568) {
                Ok(f284({
                    let a5569 = v4;
                    let a5570 = v10;
                    nat_sub(a5569, a5570)
                }, v7))
            } else {
                let a5571 = v4;
                let a5572 = v12;
                if nat_lt(a5571, a5572) {
                    let v15: u64 = {
                        let a5573 = v4;
                        let a5574 = v11;
                        nat_sub(a5573, a5574)
                    };
                    f283(true, {
                        let a5575 = v15;
                        let a5576 = v9;
                        let a5577 = 0u64;
                        nat_rem(a5575, a5576, a5577)
                    }, {
                        let a5578 = v15;
                        let a5579 = v9;
                        let a5580 = 0u64;
                        nat_quot(a5578, a5579, a5580)
                    }, v0, v1, v2)
                } else {
                    let a5581 = v4;
                    let a5582 = v13;
                    if nat_lt(a5581, a5582) {
                        Ok(f285({
                            let a5583 = v4;
                            let a5584 = v12;
                            nat_sub(a5583, a5584)
                        }, v7, v3))
                    } else {
                        let a5587 = v4;
                        let a5588 = {
                            let a5585 = v13;
                            let a5586 = v7;
                            nat_add(a5585, a5586)?
                        };
                        if nat_lt(a5587, a5588) {
                            let v16: u64 = {
                                let a5589 = v4;
                                let a5590 = v13;
                                nat_sub(a5589, a5590)
                            };
                            let v17: u64 = {
                                let a5591 = v16;
                                let a5592 = v5;
                                let a5593 = 0u64;
                                nat_quot(a5591, a5592, a5593)
                            };
                            let v18: u64 = {
                                let a5599 = {
                                    let a5594 = f286(v17, v0, v1)?;
                                    let a5595 = v5;
                                    nat_mul(a5594, a5595)?
                                };
                                let a5600 = {
                                    let a5596 = v16;
                                    let a5597 = v5;
                                    let a5598 = 0u64;
                                    nat_rem(a5596, a5597, a5598)
                                };
                                nat_add(a5599, a5600)?
                            };
                            let v19: u64 = f273(v18, v2)?;
                            Ok(f274(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f268(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f269(v0: u64) -> u64 {
    let a5601 = v0;
    let a5602 = 4u64;
    let a5603 = 0u64;
    nat_quot(a5601, a5602, a5603)
}

pub fn f270(v0: u64) -> R<u64> {
    let a5606 = {
        let a5604 = 7u64;
        let a5605 = v0;
        nat_mul(a5604, a5605)?
    };
    let a5607 = 4u64;
    nat_add(a5606, a5607)
}

pub fn f271(v0: u64) -> u64 {
    if f119(v0) {
        let m5608 = f121(v0);
        match m5608 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5610 = 1u64;
                let a5611 = match v1.clone() {
                    Adt18::C0(_, h5609, _) => {
                        h5609
                    }
                };
                if if nat_lt(a5610, a5611) {
                    true
                } else {
                    let a5612 = v0;
                    let a5613 = 43u64;
                    nat_eq(a5612, a5613)
                } {
                    match v1 {
                        Adt18::C0(_, h5614, _) => {
                            h5614
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

pub fn f272(v0: u64) -> Adt19 {
    f274(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f273(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5616 = v0;
        let a5617 = match v1.clone() {
            Adt20::C0(h5615, _, _, _, _, _, _, _) => {
                h5615
            }
        };
        let a5618 = 0u64;
        nat_quot(a5616, a5617, a5618)
    };
    let v3: u64 = {
        let a5620 = v2;
        let a5621 = match v1.clone() {
            Adt20::C0(_, h5619, _, _, _, _, _, _) => {
                h5619
            }
        };
        let a5622 = 0u64;
        nat_quot(a5620, a5621, a5622)
    };
    f138({
        let a5624 = v0;
        let a5625 = match v1.clone() {
            Adt20::C0(h5623, _, _, _, _, _, _, _) => {
                h5623
            }
        };
        let a5626 = 0u64;
        nat_rem(a5624, a5625, a5626)
    }, {
        let a5628 = v2;
        let a5629 = match v1.clone() {
            Adt20::C0(_, h5627, _, _, _, _, _, _) => {
                h5627
            }
        };
        let a5630 = 0u64;
        nat_rem(a5628, a5629, a5630)
    }, {
        let a5632 = v3;
        let a5633 = match v1.clone() {
            Adt20::C0(_, _, h5631, _, _, _, _, _) => {
                h5631
            }
        };
        let a5634 = 0u64;
        nat_rem(a5632, a5633, a5634)
    }, {
        let a5636 = v3;
        let a5637 = match v1.clone() {
            Adt20::C0(_, _, h5635, _, _, _, _, _) => {
                h5635
            }
        };
        let a5638 = 0u64;
        nat_quot(a5636, a5637, a5638)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5639, _, _, _) => {
            h5639
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5640, _, _) => {
            h5640
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5641, _) => {
            h5641
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5642) => {
            h5642
        }
    })
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5643 = v0;
    let a5644 = 4u64;
    if if if nat_le(a5643, a5644) {
        true
    } else {
        let a5645 = 9u64;
        let a5646 = v0;
        if nat_le(a5645, a5646) {
            let a5647 = v0;
            let a5648 = 12u64;
            nat_le(a5647, a5648)
        } else {
            false
        }
    } {
        true
    } else {
        let a5649 = v0;
        let a5650 = 16u64;
        if if nat_eq(a5649, a5650) {
            true
        } else {
            let a5651 = v0;
            let a5652 = 19u64;
            nat_eq(a5651, a5652)
        } {
            true
        } else {
            let a5653 = v0;
            let a5654 = 21u64;
            if nat_eq(a5653, a5654) {
                true
            } else {
                let a5655 = v0;
                let a5656 = 22u64;
                nat_eq(a5655, a5656)
            }
        }
    } {
        Adt19::C1({
            let a5657 = v0;
            let a5658 = 4u64;
            if if nat_lt(a5657, a5658) {
                true
            } else {
                let a5659 = 21u64;
                let a5660 = v0;
                nat_le(a5659, a5660)
            } {
                0u64
            } else {
                let a5661 = v0;
                let a5662 = 16u64;
                if if nat_eq(a5661, a5662) {
                    true
                } else {
                    let a5663 = v0;
                    let a5664 = 19u64;
                    nat_eq(a5663, a5664)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5665 = v0;
            let a5666 = 4u64;
            if nat_lt(a5665, a5666) {
                v0
            } else {
                let a5667 = v0;
                let a5668 = 4u64;
                if if nat_eq(a5667, a5668) {
                    true
                } else {
                    let a5669 = v0;
                    let a5670 = 16u64;
                    nat_eq(a5669, a5670)
                } {
                    0u64
                } else {
                    let a5671 = v0;
                    let a5672 = 19u64;
                    if nat_eq(a5671, a5672) {
                        3u64
                    } else {
                        let a5673 = 21u64;
                        let a5674 = v0;
                        if nat_le(a5673, a5674) {
                            let a5675 = v0;
                            let a5676 = 17u64;
                            nat_sub(a5675, a5676)
                        } else {
                            let a5677 = v0;
                            let a5678 = 8u64;
                            nat_sub(a5677, a5678)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f275(v0: u64) -> R<u64> {
    let a5681 = 2u64;
    let a5682 = {
        let a5679 = 6u64;
        let a5680 = v0;
        nat_mul(a5679, a5680)?
    };
    nat_add(a5681, a5682)
}

pub fn f276(v0: u64) -> R<u64> {
    let a5683 = f275(v0)?;
    let a5684 = 11u64;
    nat_add(a5683, a5684)
}

pub fn f277(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5685 = v0;
        let a5686 = 6u64;
        let a5687 = 0u64;
        nat_rem(a5685, a5686, a5687)
    };
    let v5: u64 = {
        let a5694 = {
            let a5689 = v1;
            let a5690 = match v2.clone() {
                Adt20::C0(h5688, _, _, _, _, _, _, _) => {
                    h5688
                }
            };
            nat_mul(a5689, a5690)?
        };
        let a5695 = {
            let a5691 = v0;
            let a5692 = 6u64;
            let a5693 = 0u64;
            nat_quot(a5691, a5692, a5693)
        };
        nat_add(a5694, a5695)?
    };
    let v6: u64 = f273(v5, v3)?;
    let v7: u64 = f273(v5, v2)?;
    let a5696 = v4;
    let a5697 = 0u64;
    if nat_eq(a5696, a5697) {
        Ok(f274(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a5698 = v4;
        let a5699 = 1u64;
        if nat_eq(a5698, a5699) {
            Ok(f274(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a5700 = v4;
            let a5701 = 2u64;
            if nat_eq(a5700, a5701) {
                Ok(f274(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a5702 = v4;
                let a5703 = 3u64;
                if nat_eq(a5702, a5703) {
                    Ok(f274(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a5704 = v4;
                    let a5705 = 4u64;
                    if nat_eq(a5704, a5705) {
                        Ok(f274(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f274(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f278(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5706 = v0;
    let a5707 = 0u64;
    if nat_eq(a5706, a5707) {
        f274(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a5708 = v0;
        let a5709 = 1u64;
        if nat_eq(a5708, a5709) {
            f274(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a5710 = v0;
            let a5711 = 2u64;
            if nat_eq(a5710, a5711) {
                f274(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a5712 = v0;
                let a5713 = 3u64;
                if nat_eq(a5712, a5713) {
                    f274(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a5714 = v0;
                    let a5715 = 4u64;
                    if nat_eq(a5714, a5715) {
                        f274(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a5716 = v0;
                        let a5717 = 5u64;
                        if nat_eq(a5716, a5717) {
                            f274(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a5718 = v0;
                            let a5719 = 6u64;
                            if nat_eq(a5718, a5719) {
                                f274(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a5720 = v0;
                                let a5721 = 7u64;
                                if nat_eq(a5720, a5721) {
                                    f274(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a5722 = v0;
                                    let a5723 = 8u64;
                                    if nat_eq(a5722, a5723) {
                                        f274(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a5724 = v0;
                                        let a5725 = 9u64;
                                        if nat_eq(a5724, a5725) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a5726 = v0;
                                            let a5727 = 10u64;
                                            if nat_eq(a5726, a5727) {
                                                f274(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f279(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5728 = v0;
        let a5729 = 3u64;
        let a5730 = 0u64;
        nat_rem(a5728, a5729, a5730)
    };
    let v6: u64 = {
        let a5737 = {
            let a5732 = v1;
            let a5733 = match v2.clone() {
                Adt20::C0(h5731, _, _, _, _, _, _, _) => {
                    h5731
                }
            };
            nat_mul(a5732, a5733)?
        };
        let a5738 = {
            let a5734 = v0;
            let a5735 = 3u64;
            let a5736 = 0u64;
            nat_quot(a5734, a5735, a5736)
        };
        nat_add(a5737, a5738)?
    };
    let v7: u64 = f273(v6, v3)?;
    let v8: u64 = f273(v6, v2)?;
    let v9: u64 = f273(v6, v4)?;
    let a5739 = v5;
    let a5740 = 0u64;
    if nat_eq(a5739, a5740) {
        Ok(f274(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a5741 = v5;
        let a5742 = 1u64;
        if nat_eq(a5741, a5742) {
            Ok(f274(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f274(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f280(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h5743, _, _, _, _, _, _, _) => {
            h5743
        }
    };
    let v5: u64 = {
        let a5746 = 1u64;
        let a5747 = {
            let a5744 = v4;
            let a5745 = f268(v3);
            nat_mul(a5744, a5745)?
        };
        nat_add(a5746, a5747)?
    };
    let a5748 = v0;
    let a5749 = 0u64;
    if nat_eq(a5748, a5749) {
        Ok(f272(0u64))
    } else {
        let a5750 = v0;
        let a5751 = v5;
        if nat_lt(a5750, a5751) {
            let v6: u64 = {
                let a5752 = v0;
                let a5753 = 1u64;
                nat_sub(a5752, a5753)
            };
            let v7: u64 = f273({
                let a5759 = {
                    let a5754 = v1;
                    let a5755 = v4;
                    nat_mul(a5754, a5755)?
                };
                let a5760 = {
                    let a5756 = v6;
                    let a5757 = f268(v3);
                    let a5758 = 0u64;
                    nat_quot(a5756, a5757, a5758)
                };
                nat_add(a5759, a5760)?
            }, v2)?;
            if v3 {
                let a5764 = {
                    let a5761 = v6;
                    let a5762 = 2u64;
                    let a5763 = 0u64;
                    nat_rem(a5761, a5762, a5763)
                };
                let a5765 = 0u64;
                if nat_eq(a5764, a5765) {
                    Ok(f274(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f274(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f274(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a5766 = v0;
                let a5767 = v5;
                nat_sub(a5766, a5767)
            };
            let a5768 = v8;
            let a5769 = 0u64;
            if nat_eq(a5768, a5769) {
                Ok(f274(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a5770 = v8;
                    let a5771 = 1u64;
                    nat_eq(a5770, a5771)
                } else {
                    false
                } {
                    Ok(f274(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a5772 = v8;
                    let a5773 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a5772, a5773) {
                        Ok(f274(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a5774 = v8;
                        let a5775 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a5774, a5775) {
                            Ok(f272(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f281(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a5777 = 13u64;
        let a5778 = f269(match v2.clone() {
            Adt20::C0(h5776, _, _, _, _, _, _, _) => {
                h5776
            }
        });
        nat_mul(a5777, a5778)?
    } else {
        0u64
    };
    let a5779 = v0;
    let a5780 = v5;
    if nat_lt(a5779, a5780) {
        f287({
            let a5781 = v0;
            let a5782 = 13u64;
            let a5783 = 0u64;
            nat_rem(a5781, a5782, a5783)
        }, v1, {
            let a5784 = v0;
            let a5785 = 13u64;
            let a5786 = 0u64;
            nat_quot(a5784, a5785, a5786)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a5787 = v0;
            let a5788 = v5;
            nat_sub(a5787, a5788)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f288({
            let a5789 = v6;
            let a5790 = v7;
            let a5791 = 0u64;
            nat_rem(a5789, a5790, a5791)
        }, {
            let a5801 = {
                let a5796 = {
                    let a5793 = v1;
                    let a5794 = match v2.clone() {
                        Adt20::C0(h5792, _, _, _, _, _, _, _) => {
                            h5792
                        }
                    };
                    nat_mul(a5793, a5794)?
                };
                let a5797 = if v4 {
                    f289(match v2.clone() {
                        Adt20::C0(h5795, _, _, _, _, _, _, _) => {
                            h5795
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a5796, a5797)?
            };
            let a5802 = {
                let a5798 = v6;
                let a5799 = v7;
                let a5800 = 0u64;
                nat_quot(a5798, a5799, a5800)
            };
            nat_add(a5801, a5802)?
        }, v2, v3, v4)
    }
}

pub fn f282(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a5803 = v0;
    let a5804 = 0u64;
    if nat_eq(a5803, a5804) {
        f274(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a5805 = v0;
        let a5806 = 1u64;
        if nat_eq(a5805, a5806) {
            f274(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a5807 = v0;
                let a5808 = 2u64;
                nat_eq(a5807, a5808)
            } else {
                false
            } {
                f274(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a5809 = v0;
                let a5810 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a5809, a5810) {
                    f274(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5811 = v0;
                    let a5812 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a5811, a5812) {
                        f274(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5813 = v0;
                        let a5814 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a5813, a5814) {
                            f274(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f283(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a5818 = 1u64;
        let a5819 = {
            let a5816 = match v4.clone() {
                Adt20::C0(h5815, _, _, _, _, _, _, _) => {
                    h5815
                }
            };
            let a5817 = v6;
            nat_mul(a5816, a5817)?
        };
        nat_add(a5818, a5819)?
    };
    let a5820 = v1;
    let a5821 = 0u64;
    if nat_eq(a5820, a5821) {
        Ok(f272(1u64))
    } else {
        let a5822 = v1;
        let a5823 = v7;
        if nat_lt(a5822, a5823) {
            let v8: u64 = {
                let a5824 = v1;
                let a5825 = 1u64;
                nat_sub(a5824, a5825)
            };
            let v9: u64 = {
                let a5832 = {
                    let a5827 = f286(v2, v3, v4.clone())?;
                    let a5828 = match v4.clone() {
                        Adt20::C0(h5826, _, _, _, _, _, _, _) => {
                            h5826
                        }
                    };
                    nat_mul(a5827, a5828)?
                };
                let a5833 = {
                    let a5829 = v8;
                    let a5830 = v6;
                    let a5831 = 0u64;
                    nat_quot(a5829, a5830, a5831)
                };
                nat_add(a5832, a5833)?
            };
            let v10: u64 = f273(v9, v4)?;
            let v11: u64 = f273(v9, v5)?;
            let v12: u64 = {
                let a5834 = v8;
                let a5835 = v6;
                let a5836 = 0u64;
                nat_rem(a5834, a5835, a5836)
            };
            if v0 {
                let a5837 = v12;
                let a5838 = 0u64;
                if nat_eq(a5837, a5838) {
                    Ok(f274(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a5839 = v12;
                    let a5840 = 1u64;
                    if nat_eq(a5839, a5840) {
                        Ok(f274(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a5841 = v12;
                        let a5842 = 2u64;
                        if nat_eq(a5841, a5842) {
                            Ok(f274(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f274(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a5843 = v12;
                let a5844 = 0u64;
                if nat_eq(a5843, a5844) {
                    Ok(f274(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f274(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a5845 = v1;
            let a5846 = v7;
            if nat_eq(a5845, a5846) {
                Ok(f274(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f284(v0: u64, v1: u64) -> Adt19 {
    let a5847 = v0;
    let a5848 = 0u64;
    if nat_eq(a5847, a5848) {
        f274(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5849 = v0;
        let a5850 = 1u64;
        if nat_eq(a5849, a5850) {
            f274(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a5851 = v0;
            let a5852 = 2u64;
            if nat_eq(a5851, a5852) {
                f274(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a5853 = v0;
                let a5854 = 3u64;
                if nat_eq(a5853, a5854) {
                    f272(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f285(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a5855 = v0;
    let a5856 = 0u64;
    if nat_eq(a5855, a5856) {
        f274(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a5857 = v0;
        let a5858 = 1u64;
        if nat_eq(a5857, a5858) {
            f274(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a5859 = v0;
            let a5860 = 2u64;
            if nat_eq(a5859, a5860) {
                f274(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a5861 = v0;
                let a5862 = 3u64;
                if nat_eq(a5861, a5862) {
                    f274(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a5863 = v0;
                    let a5864 = 4u64;
                    if nat_eq(a5863, a5864) {
                        f274(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a5865 = v0;
                        let a5866 = 5u64;
                        if nat_eq(a5865, a5866) {
                            f274(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f286(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a5877 = {
        let a5875 = {
            let a5872 = {
                let a5869 = match v1.clone() {
                    Adt22::C0(_, _, h5867, _) => {
                        h5867
                    }
                };
                let a5870 = match v2.clone() {
                    Adt20::C0(_, _, h5868, _, _, _, _, _) => {
                        h5868
                    }
                };
                nat_mul(a5869, a5870)?
            };
            let a5873 = match v1 {
                Adt22::C0(h5871, _, _, _) => {
                    h5871
                }
            };
            nat_add(a5872, a5873)?
        };
        let a5876 = match v2 {
            Adt20::C0(_, h5874, _, _, _, _, _, _) => {
                h5874
            }
        };
        nat_mul(a5875, a5876)?
    };
    let a5878 = v0;
    nat_add(a5877, a5878)
}

pub fn f287(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a5886 = {
            let a5884 = {
                let a5880 = v1;
                let a5881 = match v3.clone() {
                    Adt20::C0(h5879, _, _, _, _, _, _, _) => {
                        h5879
                    }
                };
                nat_mul(a5880, a5881)?
            };
            let a5885 = {
                let a5882 = v2;
                let a5883 = 4u64;
                nat_mul(a5882, a5883)?
            };
            nat_add(a5884, a5885)?
        };
        let a5887 = f290(v0);
        nat_add(a5886, a5887)?
    };
    let v6: u64 = f273(v5, v3)?;
    let v7: u64 = f273(v5, v4)?;
    let a5888 = v0;
    let a5889 = 0u64;
    if if if if nat_eq(a5888, a5889) {
        true
    } else {
        let a5890 = v0;
        let a5891 = 2u64;
        nat_eq(a5890, a5891)
    } {
        true
    } else {
        let a5892 = v0;
        let a5893 = 5u64;
        nat_eq(a5892, a5893)
    } {
        true
    } else {
        let a5894 = v0;
        let a5895 = 7u64;
        nat_eq(a5894, a5895)
    } {
        Ok(f274(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a5896 = v0;
        let a5897 = 1u64;
        if if if if nat_eq(a5896, a5897) {
            true
        } else {
            let a5898 = v0;
            let a5899 = 3u64;
            nat_eq(a5898, a5899)
        } {
            true
        } else {
            let a5900 = v0;
            let a5901 = 6u64;
            nat_eq(a5900, a5901)
        } {
            true
        } else {
            let a5902 = v0;
            let a5903 = 8u64;
            nat_eq(a5902, a5903)
        } {
            Ok(f274(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a5904 = v0;
                let a5905 = 1u64;
                if nat_eq(a5904, a5905) {
                    4u64
                } else {
                    let a5906 = v0;
                    let a5907 = 6u64;
                    if nat_eq(a5906, a5907) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a5908 = v0;
            let a5909 = 4u64;
            if nat_eq(a5908, a5909) {
                Ok(f274(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a5910 = v0;
                let a5911 = 9u64;
                if nat_eq(a5910, a5911) {
                    Ok(f274(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a5912 = v0;
                    let a5913 = 10u64;
                    if nat_eq(a5912, a5913) {
                        Ok(f274(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a5914 = v0;
                        let a5915 = 11u64;
                        if nat_eq(a5914, a5915) {
                            Ok(f274(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a5916 = v0;
                            let a5917 = 12u64;
                            if nat_eq(a5916, a5917) {
                                Ok(f274(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f288(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f273(v1, v2)?;
    let v6: u64 = f273(v1, v3)?;
    let a5918 = v0;
    let a5919 = 0u64;
    if nat_eq(a5918, a5919) {
        Ok(f274(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a5920 = v0;
        let a5921 = 1u64;
        if nat_eq(a5920, a5921) {
            Ok(f274(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a5922 = v0;
                let a5923 = 2u64;
                if nat_eq(a5922, a5923) {
                    Ok(f274(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5924 = v0;
                    let a5925 = 3u64;
                    if nat_eq(a5924, a5925) {
                        Ok(f274(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a5926 = v0;
                let a5927 = 2u64;
                if nat_eq(a5926, a5927) {
                    Ok(f274(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f289(v0: u64) -> R<u64> {
    let a5928 = 4u64;
    let a5929 = f269(v0);
    nat_mul(a5928, a5929)
}

pub fn f290(v0: u64) -> u64 {
    let a5930 = v0;
    let a5931 = 5u64;
    if nat_lt(a5930, a5931) {
        let a5932 = v0;
        let a5933 = 2u64;
        if nat_lt(a5932, a5933) {
            0u64
        } else {
            1u64
        }
    } else {
        let a5934 = v0;
        let a5935 = 7u64;
        if nat_lt(a5934, a5935) {
            2u64
        } else {
            3u64
        }
    }
}
