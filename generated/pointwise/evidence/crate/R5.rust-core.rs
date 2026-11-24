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
    C34(Adt3, Adt6, Adt6, u64),
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
        let a465 = 55u64;
        if nat_eq(a464, a465) {
            let m466 = v3;
            match m466 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f41(v1, v2, v6, v4)?
                }
            }
        } else {
            let a467 = v0;
            let a468 = 22u64;
            if nat_eq(a467, a468) {
                let m469 = v3;
                match m469 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f42(v2, v7, v4)?
                    }
                }
            } else {
                let a470 = v0;
                let a471 = 44u64;
                if nat_eq(a470, a471) {
                    f43(v1, v2, v3, v4)?
                } else {
                    let a472 = v0;
                    let a473 = 52u64;
                    if nat_eq(a472, a473) {
                        f44(v1, v2, v4)?
                    } else {
                        let a474 = v0;
                        let a475 = 73u64;
                        if nat_eq(a474, a475) {
                            f45(v2, v3, v4)?
                        } else {
                            let a476 = v0;
                            let a477 = 57u64;
                            if if nat_eq(a476, a477) {
                                true
                            } else {
                                let a478 = v0;
                                let a479 = 58u64;
                                nat_eq(a478, a479)
                            } {
                                let m480 = v3;
                                match m480 {
                                    None => {
                                        f46({
                                            let a481 = v0;
                                            let a482 = 58u64;
                                            nat_eq(a481, a482)
                                        }, v1, v2, v4)?
                                    }
                                    Some(_) => {
                                        Adt10::C1
                                    }
                                }
                            } else {
                                let a483 = v0;
                                let a484 = 49u64;
                                if nat_eq(a483, a484) {
                                    f47(v1, v2, v3, v4)?
                                } else {
                                    let a485 = v0;
                                    let a486 = 51u64;
                                    if nat_eq(a485, a486) {
                                        f48(v1, v2, v3, v4)?
                                    } else {
                                        let a487 = v0;
                                        let a488 = 39u64;
                                        if nat_eq(a487, a488) {
                                            f49(v2, v3, v4)?
                                        } else {
                                            let a489 = v0;
                                            let a490 = 77u64;
                                            if nat_eq(a489, a490) {
                                                f50(v2, v4)?
                                            } else {
                                                let a491 = v0;
                                                let a492 = 40u64;
                                                if nat_eq(a491, a492) {
                                                    f51(v2, v3, v4)?
                                                } else {
                                                    if f52(v0) {
                                                        let a493 = v0;
                                                        let a494 = 33u64;
                                                        if if if nat_eq(a493, a494) {
                                                            let a496 = match v4.clone() {
                                                                Adt2::C0(h495, _, _, _, _, _, _, _, _, _, _) => {
                                                                    h495
                                                                }
                                                            };
                                                            let a497 = 0u64;
                                                            nat_eq(a496, a497)
                                                        } else {
                                                            false
                                                        } {
                                                            f53(match v2.clone() {
                                                                Adt2::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
                                                                    h498
                                                                }
                                                            })
                                                        } else {
                                                            false
                                                        } {
                                                            f54(v2, v3, v4)?
                                                        } else {
                                                            f55(v0, v2, {
                                                                let m499 = v3;
                                                                match m499 {
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
                                                        let a500 = v0;
                                                        let a501 = 19u64;
                                                        if nat_eq(a500, a501) {
                                                            f56(v2, v4)?
                                                        } else {
                                                            let a502 = v0;
                                                            let a503 = 21u64;
                                                            if nat_eq(a502, a503) {
                                                                f57(v5, v2, v3, v4)?
                                                            } else {
                                                                let a504 = v0;
                                                                let a505 = 61u64;
                                                                if if if nat_eq(a504, a505) {
                                                                    true
                                                                } else {
                                                                    let a506 = v0;
                                                                    let a507 = 62u64;
                                                                    nat_eq(a506, a507)
                                                                } {
                                                                    true
                                                                } else {
                                                                    let a508 = v0;
                                                                    let a509 = 63u64;
                                                                    nat_eq(a508, a509)
                                                                } {
                                                                    f58(v0, v1, v2, v4)?
                                                                } else {
                                                                    let a510 = v0;
                                                                    let a511 = 4u64;
                                                                    if if nat_eq(a510, a511) {
                                                                        true
                                                                    } else {
                                                                        let a512 = v0;
                                                                        let a513 = 32u64;
                                                                        nat_eq(a512, a513)
                                                                    } {
                                                                        f59(v0, v1, v2, v3, v4)?
                                                                    } else {
                                                                        let a514 = v0;
                                                                        let a515 = 20u64;
                                                                        if nat_eq(a514, a515) {
                                                                            f60(v2, v4)?
                                                                        } else {
                                                                            let a516 = v0;
                                                                            let a517 = 42u64;
                                                                            if nat_eq(a516, a517) {
                                                                                f61(4u64, v2, v4)?
                                                                            } else {
                                                                                let a518 = v0;
                                                                                let a519 = 69u64;
                                                                                if nat_eq(a518, a519) {
                                                                                    let a520 = v5;
                                                                                    let a521 = 4u64;
                                                                                    if nat_lt(a520, a521) {
                                                                                        f61(v5, v2, v4)?
                                                                                    } else {
                                                                                        Adt10::C1
                                                                                    }
                                                                                } else {
                                                                                    let a522 = v0;
                                                                                    let a523 = 70u64;
                                                                                    if nat_eq(a522, a523) {
                                                                                        f62(v5, v2, v4)?
                                                                                    } else {
                                                                                        let a524 = v0;
                                                                                        let a525 = 75u64;
                                                                                        if if nat_eq(a524, a525) {
                                                                                            true
                                                                                        } else {
                                                                                            let a526 = v0;
                                                                                            let a527 = 76u64;
                                                                                            nat_eq(a526, a527)
                                                                                        } {
                                                                                            f63({
                                                                                                let a528 = v0;
                                                                                                let a529 = 76u64;
                                                                                                nat_eq(a528, a529)
                                                                                            }, v1, v2, v4)?
                                                                                        } else {
                                                                                            let a530 = v0;
                                                                                            let a531 = 31u64;
                                                                                            if if if nat_eq(a530, a531) {
                                                                                                true
                                                                                            } else {
                                                                                                let a532 = v0;
                                                                                                let a533 = 47u64;
                                                                                                nat_eq(a532, a533)
                                                                                            } {
                                                                                                true
                                                                                            } else {
                                                                                                let a534 = v0;
                                                                                                let a535 = 68u64;
                                                                                                nat_eq(a534, a535)
                                                                                            } {
                                                                                                f64(v0, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a536 = v0;
                                                                                                let a537 = 15u64;
                                                                                                if nat_eq(a536, a537) {
                                                                                                    f65(v2, v4)?
                                                                                                } else {
                                                                                                    let a538 = v0;
                                                                                                    let a539 = 17u64;
                                                                                                    if nat_eq(a538, a539) {
                                                                                                        f66(v2, v4)?
                                                                                                    } else {
                                                                                                        let a540 = v0;
                                                                                                        let a541 = 18u64;
                                                                                                        if nat_eq(a540, a541) {
                                                                                                            f67(v2, v3, v4)?
                                                                                                        } else {
                                                                                                            let a542 = v0;
                                                                                                            let a543 = 13u64;
                                                                                                            if if if nat_eq(a542, a543) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a544 = v0;
                                                                                                                let a545 = 14u64;
                                                                                                                nat_eq(a544, a545)
                                                                                                            } {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a546 = v0;
                                                                                                                let a547 = 16u64;
                                                                                                                nat_eq(a546, a547)
                                                                                                            } {
                                                                                                                f68(v0, v2, v4)?
                                                                                                            } else {
                                                                                                                let m548 = f69(v0, v5)?;
                                                                                                                match m548 {
                                                                                                                    None => {
                                                                                                                        f70(v0, v5, v2, v3, v4)?
                                                                                                                    }
                                                                                                                    Some(v10) => {
                                                                                                                        f71(v10, v2, v4)?
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h549, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h549
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h550, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h550
        }
    };
    let v6: u64 = {
        let a551 = v3;
        let a552 = v5;
        let a553 = 0u64;
        nat_rem(a551, a552, a553)
    };
    let a554 = v1;
    let a555 = 0u64;
    if nat_eq(a554, a555) {
        let a557 = v3;
        let a558 = match v0.clone() {
            Adt8::C0(_, _, _, h556, _, _, _, _, _, _, _, _, _, _, _) => {
                h556
            }
        };
        if nat_lt(a557, a558) {
            Ok(Adt9::C8(Some(f72(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h559, _, _, _, _, _, _) => {
                    h559
                }
            }, {
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
                nat_rem(a563, a564, a565)
            }, {
                let a575 = {
                    let a569 = v6;
                    let a570 = {
                        let m568 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h567, _, _, _, _, _, _) => {
                                h567
                            }
                        };
                        match m568 {
                            Adt3::C0(h566, _, _, _, _, _, _, _) => {
                                h566
                            }
                        }
                    };
                    let a571 = 0u64;
                    nat_quot(a569, a570, a571)
                };
                let a576 = {
                    let m574 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h573, _, _, _, _, _, _) => {
                            h573
                        }
                    };
                    match m574 {
                        Adt3::C0(_, h572, _, _, _, _, _, _) => {
                            h572
                        }
                    }
                };
                let a577 = 0u64;
                nat_rem(a575, a576, a577)
            }, {
                let a587 = {
                    let a581 = v6;
                    let a582 = {
                        let m580 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h579, _, _, _, _, _, _) => {
                                h579
                            }
                        };
                        match m580 {
                            Adt3::C0(h578, _, _, _, _, _, _, _) => {
                                h578
                            }
                        }
                    };
                    let a583 = 0u64;
                    nat_quot(a581, a582, a583)
                };
                let a588 = {
                    let m586 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h585, _, _, _, _, _, _) => {
                            h585
                        }
                    };
                    match m586 {
                        Adt3::C0(_, h584, _, _, _, _, _, _) => {
                            h584
                        }
                    }
                };
                let a589 = 0u64;
                nat_quot(a587, a588, a589)
            }, {
                let a590 = v3;
                let a591 = v5;
                let a592 = 0u64;
                nat_quot(a590, a591, a592)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a598 = v2;
            let a599 = {
                let a595 = v3;
                let a596 = {
                    let a593 = v1;
                    let a594 = 1u64;
                    if nat_eq(a593, a594) {
                        v4
                    } else {
                        v5
                    }
                };
                let a597 = 0u64;
                nat_quot(a595, a596, a597)
            };
            nat_add(a598, a599)?
        };
        let a601 = v7;
        let a602 = match v0.clone() {
            Adt8::C0(h600, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h600
            }
        };
        if nat_lt(a601, a602) {
            let v8: u64 = {
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
                nat_rem(a606, a607, a608)
            };
            let v9: u64 = {
                let a612 = v7;
                let a613 = {
                    let m611 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h610, _, _, _, _, _) => {
                            h610
                        }
                    };
                    match m611 {
                        Adt3::C0(h609, _, _, _, _, _, _, _) => {
                            h609
                        }
                    }
                };
                let a614 = 0u64;
                nat_quot(a612, a613, a614)
            };
            let v10: u64 = {
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
                nat_rem(a618, a619, a620)
            };
            let v11: u64 = {
                let a624 = v9;
                let a625 = {
                    let m623 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h622, _, _, _, _, _) => {
                            h622
                        }
                    };
                    match m623 {
                        Adt3::C0(_, h621, _, _, _, _, _, _) => {
                            h621
                        }
                    }
                };
                let a626 = 0u64;
                nat_quot(a624, a625, a626)
            };
            let a627 = v1;
            let a628 = 1u64;
            if nat_eq(a627, a628) {
                Ok(Adt9::C8(Some(f72(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h629, _, _, _, _, _) => {
                        h629
                    }
                }, v8, v10, {
                    let a630 = v3;
                    let a631 = v4;
                    let a632 = 0u64;
                    nat_rem(a630, a631, a632)
                }, v11)?)))
            } else {
                let a633 = v1;
                let a634 = 2u64;
                if nat_eq(a633, a634) {
                    Ok(Adt9::C8(Some({
                        let a680 = f72(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h635, _, _, _, _) => {
                                h635
                            }
                        }, {
                            let a647 = {
                                let a639 = v8;
                                let a640 = {
                                    let m638 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h637, _, _, _) => {
                                            h637
                                        }
                                    };
                                    match m638 {
                                        Adt6::C0(_, h636, _, _) => {
                                            h636
                                        }
                                    }
                                };
                                nat_mul(a639, a640)?
                            };
                            let a648 = {
                                let a644 = v6;
                                let a645 = {
                                    let m643 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h642, _, _, _, _, _, _) => {
                                            h642
                                        }
                                    };
                                    match m643 {
                                        Adt3::C0(h641, _, _, _, _, _, _, _) => {
                                            h641
                                        }
                                    }
                                };
                                let a646 = 0u64;
                                nat_rem(a644, a645, a646)
                            };
                            nat_add(a647, a648)?
                        }, {
                            let a666 = {
                                let a652 = v10;
                                let a653 = {
                                    let m651 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h650, _, _, _) => {
                                            h650
                                        }
                                    };
                                    match m651 {
                                        Adt6::C0(_, h649, _, _) => {
                                            h649
                                        }
                                    }
                                };
                                nat_mul(a652, a653)?
                            };
                            let a667 = {
                                let a663 = {
                                    let a657 = v6;
                                    let a658 = {
                                        let m656 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h655, _, _, _, _, _, _) => {
                                                h655
                                            }
                                        };
                                        match m656 {
                                            Adt3::C0(h654, _, _, _, _, _, _, _) => {
                                                h654
                                            }
                                        }
                                    };
                                    let a659 = 0u64;
                                    nat_quot(a657, a658, a659)
                                };
                                let a664 = {
                                    let m662 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h661, _, _, _, _, _, _) => {
                                            h661
                                        }
                                    };
                                    match m662 {
                                        Adt3::C0(_, h660, _, _, _, _, _, _) => {
                                            h660
                                        }
                                    }
                                };
                                let a665 = 0u64;
                                nat_rem(a663, a664, a665)
                            };
                            nat_add(a666, a667)?
                        }, {
                            let a677 = {
                                let a671 = v6;
                                let a672 = {
                                    let m670 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h669, _, _, _, _, _, _) => {
                                            h669
                                        }
                                    };
                                    match m670 {
                                        Adt3::C0(h668, _, _, _, _, _, _, _) => {
                                            h668
                                        }
                                    }
                                };
                                let a673 = 0u64;
                                nat_quot(a671, a672, a673)
                            };
                            let a678 = {
                                let m676 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h675, _, _, _, _, _, _) => {
                                        h675
                                    }
                                };
                                match m676 {
                                    Adt3::C0(_, h674, _, _, _, _, _, _) => {
                                        h674
                                    }
                                }
                            };
                            let a679 = 0u64;
                            nat_quot(a677, a678, a679)
                        }, v11)?;
                        let a681 = 4u64;
                        let a682 = 0u64;
                        nat_quot(a680, a681, a682)
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
    let a683 = v1;
    let a684 = 0u64;
    if nat_eq(a683, a684) {
        let a686 = v3;
        let a687 = match v0.clone() {
            Adt8::C0(_, _, _, h685, _, _, _, _, _, _, _, _, _, _, _) => {
                h685
            }
        };
        if nat_lt(a686, a687) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h688, _, _, _, _, _, _) => {
                    h688
                }
            };
            let v5: u64 = {
                let a690 = v3;
                let a691 = match v0.clone() {
                    Adt8::C0(_, _, h689, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h689
                    }
                };
                let a692 = 0u64;
                nat_quot(a690, a691, a692)
            };
            Ok(Adt9::C8(Some(f72(v4.clone(), f73(v5, v4.clone(), 0u64), f73(v5, v4.clone(), 1u64), f73(v5, v4, 2u64), f74(v0.clone(), v5, {
                let a694 = v3;
                let a695 = match v0 {
                    Adt8::C0(_, _, h693, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h693
                    }
                };
                let a696 = 0u64;
                nat_rem(a694, a695, a696)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h697, _, _, _, _) => {
                h697
            }
        };
        let v7: u64 = {
            let a698 = v1;
            let a699 = 1u64;
            if nat_eq(a698, a699) {
                match v0.clone() {
                    Adt8::C0(_, h700, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h700
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h701, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h701
                    }
                }
            }
        };
        let v8: u64 = {
            let a705 = v2;
            let a706 = {
                let a702 = v3;
                let a703 = v7;
                let a704 = 0u64;
                nat_quot(a702, a703, a704)
            };
            nat_add(a705, a706)?
        };
        let v9: u64 = {
            let a707 = v3;
            let a708 = v7;
            let a709 = 0u64;
            nat_rem(a707, a708, a709)
        };
        let v10: u64 = {
            let a711 = v8;
            let a712 = match v6.clone() {
                Adt3::C0(h710, _, _, _, _, _, _, _) => {
                    h710
                }
            };
            let a713 = 0u64;
            nat_rem(a711, a712, a713)
        };
        let v11: u64 = {
            let a715 = v8;
            let a716 = match v6.clone() {
                Adt3::C0(h714, _, _, _, _, _, _, _) => {
                    h714
                }
            };
            let a717 = 0u64;
            nat_quot(a715, a716, a717)
        };
        let v12: u64 = {
            let a719 = v11;
            let a720 = match v6.clone() {
                Adt3::C0(_, h718, _, _, _, _, _, _) => {
                    h718
                }
            };
            let a721 = 0u64;
            nat_rem(a719, a720, a721)
        };
        let v13: u64 = {
            let a723 = v11;
            let a724 = match v6.clone() {
                Adt3::C0(_, h722, _, _, _, _, _, _) => {
                    h722
                }
            };
            let a725 = 0u64;
            nat_quot(a723, a724, a725)
        };
        let a727 = v8;
        let a728 = match v0.clone() {
            Adt8::C0(h726, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h726
            }
        };
        if nat_lt(a727, a728) {
            let a729 = v1;
            let a730 = 1u64;
            if nat_eq(a729, a730) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h731, _, _, _, _, _, _) => {
                        h731
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h732, _, _, _, _, _) => {
                        h732
                    }
                };
                let m735 = f75(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h733, _, _, _) => {
                        h733
                    }
                }, match v15.clone() {
                    Adt3::C0(h734, _, _, _, _, _, _, _) => {
                        h734
                    }
                }, v10, f73(v9, v14.clone(), 0u64))?;
                match m735 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m738 = f75(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h736, _, _) => {
                                h736
                            }
                        }, match v15 {
                            Adt3::C0(_, h737, _, _, _, _, _, _) => {
                                h737
                            }
                        }, v12, f73(v9, v14, 1u64))?;
                        match m738 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f76(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a739 = v1;
                let a740 = 2u64;
                if nat_eq(a739, a740) {
                    Ok(Adt9::C8(Some({
                        let a742 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h741) => {
                                h741
                            }
                        };
                        let a743 = 0u64;
                        if nat_eq(a742, a743) {
                            f72(v6, v10, v12, v9, v13)?
                        } else {
                            f72(v6.clone(), v10, v12, {
                                let a745 = v13;
                                let a746 = match v6.clone() {
                                    Adt3::C0(_, _, h744, _, _, _, _, _) => {
                                        h744
                                    }
                                };
                                let a747 = 0u64;
                                nat_rem(a745, a746, a747)
                            }, {
                                let a755 = {
                                    let a753 = {
                                        let a749 = v13;
                                        let a750 = match v6 {
                                            Adt3::C0(_, _, h748, _, _, _, _, _) => {
                                                h748
                                            }
                                        };
                                        let a751 = 0u64;
                                        nat_quot(a749, a750, a751)
                                    };
                                    let a754 = match v0 {
                                        Adt8::C0(_, _, h752, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h752
                                        }
                                    };
                                    nat_mul(a753, a754)?
                                };
                                let a756 = v9;
                                nat_add(a755, a756)?
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
    Ok(f77(v3.clone(), {
        let m757 = v3.clone();
        match m757 {
            Adt4::C0 => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f83(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C19(v57, v58) => {
                f84(v57, v0, v1, v2, v58, v4)?
            }
            Adt4::C20(v59, v60) => {
                f85(v59, v0, v1, v2, v60, v4)?
            }
            Adt4::C21 => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C22(v61, v62) => {
                f86(v0, v1, v2, v61, v62, v4)?
            }
            Adt4::C23(v76, v77, v78, v79) => {
                f93(v0, v79, v76, v77, v78, v4)?
            }
            Adt4::C24(v75) => {
                if v5 {
                    f91(Adt16::C0(v75), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f78(v0, v1, v2, v3)?
            }
            Adt4::C26(v71, v72, v73, v74) => {
                let a759 = v71;
                let a760 = 64u64;
                if if if nat_eq(a759, a760) {
                    true
                } else {
                    let a761 = v71;
                    let a762 = 65u64;
                    nat_eq(a761, a762)
                } {
                    true
                } else {
                    let a763 = v71;
                    let a764 = 66u64;
                    nat_eq(a763, a764)
                } {
                    if v5 {
                        f91(Adt16::C1(v71, v72, v73, v74), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a765 = v71;
                    let a766 = 0u64;
                    if if nat_eq(a765, a766) {
                        true
                    } else {
                        let a767 = v71;
                        let a768 = 30u64;
                        nat_eq(a767, a768)
                    } {
                        f92(v0, v1, v2, v71, v72, v73, v74, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v70) => {
                f90(v0, v1, v2, v70)?
            }
            Adt4::C28(v66) => {
                f87(v0, v1, v2, v66, v4)?
            }
            Adt4::C29(v67) => {
                f88(v0, v1, v2, v67)?
            }
            Adt4::C30(v68, _) => {
                f89(v0, v1, v2, v68)?
            }
            Adt4::C31(v33, v34) => {
                f79(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f80(v0, match v1 {
                    Adt3::C0(h758, _, _, _, _, _, _, _) => {
                        h758
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f81(v0, v1, v36, v2)?
            }
            Adt4::C34(v37, v38, v39, v40) => {
                f82(v0, v1, v37, v2, v38, v39, v40)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m769 = v0;
    match m769 {
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
        Adt4::C25(v53, _, v55) => {
            Ok(Adt9::C6({
                let a774 = {
                    let a770 = 0u64;
                    let a771 = v53;
                    if nat_lt(a770, a771) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a775 = {
                    let a772 = 0u64;
                    let a773 = v55;
                    if nat_lt(a772, a773) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a774, a775)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v61, _, _, _) => {
            Ok(Adt9::C6({
                let a776 = v61;
                let a777 = 66u64;
                if nat_eq(a776, a777) {
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
        Adt4::C34(_, _, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a778 = v1;
    let a779 = 5u64;
    if if if if if nat_lt(a778, a779) {
        true
    } else {
        f94(v0, v1, v2, v3, v4)
    } {
        let a780 = v5;
        let a781 = 5u64;
        nat_lt(a780, a781)
    } else {
        false
    } {
        let a782 = v7;
        let a783 = 5u64;
        nat_lt(a782, a783)
    } else {
        false
    } {
        let a784 = v9;
        let a785 = 2u64;
        if nat_lt(a784, a785) {
            true
        } else {
            let a786 = v9;
            let a787 = 3u64;
            nat_eq(a786, a787)
        }
    } else {
        false
    } {
        Adt9::C7(f95(v0, v1, v2, v3, v4), f96(v5, v6), f96(v7, v8), f97({
            let a788 = v9;
            let a789 = 0u64;
            if nat_eq(a788, a789) {
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
    let a801 = {
        let a795 = {
            let a791 = match v0.clone() {
                Adt2::C0(_, h790, _, _, _, _, _, _, _, _, _) => {
                    h790
                }
            };
            let a792 = v1;
            let a793 = 0u64;
            nat_quot(a791, a792, a793)
        };
        let a796 = match v0.clone() {
            Adt2::C0(_, _, h794, _, _, _, _, _, _, _, _) => {
                h794
            }
        };
        nat_mul(a795, a796)?
    };
    let a802 = {
        let a799 = match v0.clone() {
            Adt2::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
                h797
            }
        };
        let a800 = match v0 {
            Adt2::C0(_, _, _, _, h798, _, _, _, _, _, _) => {
                h798
            }
        };
        nat_mul(a799, a800)?
    };
    nat_mul(a801, a802)
}

pub fn f17(v0: u64) -> bool {
    let a803 = v0;
    let a804 = 4294967296u64;
    if nat_lt(a803, a804) {
        let a808 = {
            let a805 = v0;
            let a806 = 2147483648u64;
            let a807 = 0u64;
            nat_rem(a805, a806, a807)
        };
        let a809 = 2139095040u64;
        nat_lt(a808, a809)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a810 = 4u64;
    let a811 = v0;
    nat_mul(a810, a811)
}

pub fn f19(v0: u64) -> R<u64> {
    let a812 = v0;
    let a813 = 0u64;
    if nat_eq(a812, a813) {
        Ok(0u64)
    } else {
        let a816 = 4u64;
        let a817 = {
            let a814 = 3u64;
            let a815 = v0;
            nat_mul(a814, a815)?
        };
        nat_add(a816, a817)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a819 = match v0.clone() {
            Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                h818
            }
        };
        let a820 = v1;
        let a821 = 0u64;
        nat_quot(a819, a820, a821)
    }, match v0.clone() {
        Adt2::C0(_, _, h822, _, _, _, _, _, _, _, _) => {
            h822
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h823, _, _, _, _, _, _, _) => {
            h823
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h824, _, _, _, _, _, _) => {
            h824
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h825, _, _, _, _, _) => {
            h825
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h826, _, _, _, _) => {
            h826
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h827, _, _, _) => {
            h827
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h828, _, _) => {
            h828
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h829, _, _, _, _, _, _, _, _, _) => {
                h829
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h830, _, _, _, _, _, _, _, _) => {
                h830
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h831, _, _, _, _, _, _, _) => {
                h831
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h832, _, _, _, _, _) => {
                h832
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h833, _, _, _, _) => {
                h833
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h834, _, _, _) => {
                h834
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a836 = match v0.clone() {
        Adt2::C0(_, h835, _, _, _, _, _, _, _, _, _) => {
            h835
        }
    };
    let a837 = 1u64;
    if if if if nat_eq(a836, a837) {
        true
    } else {
        let a839 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h838, _, _, _, _, _) => {
                h838
            }
        };
        let a840 = 4u64;
        nat_eq(a839, a840)
    } {
        let a842 = match v0.clone() {
            Adt2::C0(_, _, h841, _, _, _, _, _, _, _, _) => {
                h841
            }
        };
        let a843 = 1u64;
        if nat_eq(a842, a843) {
            true
        } else {
            let a848 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h844, _, _, _, _) => {
                    h844
                }
            };
            let a849 = {
                let a846 = match v0.clone() {
                    Adt2::C0(_, h845, _, _, _, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a847 = 4u64;
                nat_mul(a846, a847)?
            };
            nat_eq(a848, a849)
        }
    } else {
        false
    } {
        let a851 = match v0.clone() {
            Adt2::C0(_, _, _, h850, _, _, _, _, _, _, _) => {
                h850
            }
        };
        let a852 = 1u64;
        if nat_eq(a851, a852) {
            true
        } else {
            let a860 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h853, _, _, _) => {
                    h853
                }
            };
            let a861 = {
                let a858 = {
                    let a856 = match v0.clone() {
                        Adt2::C0(_, h854, _, _, _, _, _, _, _, _, _) => {
                            h854
                        }
                    };
                    let a857 = match v0.clone() {
                        Adt2::C0(_, _, h855, _, _, _, _, _, _, _, _) => {
                            h855
                        }
                    };
                    nat_mul(a856, a857)?
                };
                let a859 = 4u64;
                nat_mul(a858, a859)?
            };
            nat_eq(a860, a861)
        }
    } else {
        false
    } {
        let a863 = match v0.clone() {
            Adt2::C0(_, _, _, _, h862, _, _, _, _, _, _) => {
                h862
            }
        };
        let a864 = 1u64;
        if nat_eq(a863, a864) {
            Ok(true)
        } else {
            let a875 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h865, _, _) => {
                    h865
                }
            };
            let a876 = {
                let a873 = {
                    let a871 = {
                        let a868 = match v0.clone() {
                            Adt2::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                                h866
                            }
                        };
                        let a869 = match v0.clone() {
                            Adt2::C0(_, _, h867, _, _, _, _, _, _, _, _) => {
                                h867
                            }
                        };
                        nat_mul(a868, a869)?
                    };
                    let a872 = match v0 {
                        Adt2::C0(_, _, _, h870, _, _, _, _, _, _, _) => {
                            h870
                        }
                    };
                    nat_mul(a871, a872)?
                };
                let a874 = 4u64;
                nat_mul(a873, a874)?
            };
            Ok(nat_eq(a875, a876))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a878 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h877, _) => {
            h877
        }
    };
    let a879 = 0u64;
    if nat_eq(a878, a879) {
        Ok(true)
    } else {
        let a881 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h880, _) => {
                h880
            }
        };
        let a882 = 0u64;
        if nat_eq(a881, a882) {
            Ok(true)
        } else {
            let a884 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h883) => {
                    h883
                }
            };
            let a885 = 0u64;
            if nat_eq(a884, a885) {
                Ok(true)
            } else {
                let a887 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h886) => {
                        h886
                    }
                };
                let a888 = 0u64;
                if nat_eq(a887, a888) {
                    Ok(true)
                } else {
                    let a894 = {
                        let a891 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h889) => {
                                h889
                            }
                        };
                        let a892 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h890, _) => {
                                h890
                            }
                        };
                        nat_add(a891, a892)?
                    };
                    let a895 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h893) => {
                            h893
                        }
                    };
                    if nat_le(a894, a895) {
                        Ok(true)
                    } else {
                        let a901 = {
                            let a898 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h896) => {
                                    h896
                                }
                            };
                            let a899 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h897, _) => {
                                    h897
                                }
                            };
                            nat_add(a898, a899)?
                        };
                        let a902 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h900) => {
                                h900
                            }
                        };
                        Ok(nat_le(a901, a902))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a904 = match v0.clone() {
        Adt6::C0(h903, _, _, _) => {
            h903
        }
    };
    let a905 = 0u64;
    if nat_eq(a904, a905) {
        f98(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h906, _, _) => {
                h906
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h907, _) => {
                h907
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h908) => {
                h908
            }
        }), {
            let a910 = v1;
            let a911 = match v0 {
                Adt6::C0(_, _, _, h909) => {
                    h909
                }
            };
            nat_add(a910, a911)?
        })
    } else {
        f98(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a914 = match v0.clone() {
        Adt2::C0(_, h912, _, _, _, _, _, _, _, _, _) => {
            h912
        }
    };
    let a915 = match v1.clone() {
        Adt2::C0(_, h913, _, _, _, _, _, _, _, _, _) => {
            h913
        }
    };
    if if if nat_eq(a914, a915) {
        let a918 = match v0.clone() {
            Adt2::C0(_, _, h916, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = match v1.clone() {
            Adt2::C0(_, _, h917, _, _, _, _, _, _, _, _) => {
                h917
            }
        };
        nat_eq(a918, a919)
    } else {
        false
    } {
        let a922 = match v0.clone() {
            Adt2::C0(_, _, _, h920, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = match v1.clone() {
            Adt2::C0(_, _, _, h921, _, _, _, _, _, _, _) => {
                h921
            }
        };
        nat_eq(a922, a923)
    } else {
        false
    } {
        let a926 = match v0 {
            Adt2::C0(_, _, _, _, h924, _, _, _, _, _, _) => {
                h924
            }
        };
        let a927 = match v1 {
            Adt2::C0(_, _, _, _, h925, _, _, _, _, _, _) => {
                h925
            }
        };
        nat_eq(a926, a927)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a932 = {
        let a928 = v0;
        let a929 = v1;
        nat_mul(a928, a929)?
    };
    let a933 = {
        let a930 = v2;
        let a931 = f99(v3);
        nat_mul(a930, a931)?
    };
    nat_mul(a932, a933)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a936 = v0;
    let a937 = {
        let a934 = f100(v1)?;
        let a935 = v2;
        nat_mul(a934, a935)?
    };
    nat_add(a936, a937)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h938, _, _, _, _, _, _, _, _, _, _) => {
            h938
        }
    }, match v0.clone() {
        Adt2::C0(_, h939, _, _, _, _, _, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h940, _, _, _, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h941, _, _, _, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h942, _, _, _, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h943, _, _, _, _, _) => {
            h943
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h944, _, _, _, _) => {
            h944
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h945, _, _, _) => {
            h945
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h946, _, _) => {
            h946
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h947, _) => {
            h947
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h948) => {
            h948
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a949 = v0;
    let a950 = 23u64;
    if nat_eq(a949, a950) {
        f101(v1, v4, v6)
    } else {
        let a951 = v0;
        let a952 = 26u64;
        if nat_eq(a951, a952) {
            f102(v1, v2, v4, v6)
        } else {
            let a953 = v0;
            let a954 = 25u64;
            if nat_eq(a953, a954) {
                f103(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a955 = v0;
    let a956 = 30u64;
    if nat_eq(a955, a956) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a957 = v0;
    let a958 = 2139095040u64;
    if nat_le(a957, a958) {
        true
    } else {
        let a959 = v0;
        let a960 = 2147483648u64;
        nat_eq(a959, a960)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a962 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h961, _, _, _, _, _) => {
            h961
        }
    };
    let a963 = v1;
    if if nat_eq(a962, a963) {
        let a968 = {
            let a965 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h964) => {
                    h964
                }
            };
            let a966 = v1;
            let a967 = 0u64;
            nat_rem(a965, a966, a967)
        };
        let a969 = 0u64;
        nat_eq(a968, a969)
    } else {
        false
    } {
        f104(match v0.clone() {
            Adt2::C0(_, _, h970, _, _, _, _, _, _, _, _) => {
                h970
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h971, _, _, _, _, _, _, _) => {
                h971
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h972, _, _, _, _, _, _) => {
                h972
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h973, _, _, _, _) => {
                h973
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h974, _, _, _) => {
                h974
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h975, _, _) => {
                h975
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a978 = match v0.clone() {
            Adt2::C0(h976, _, _, _, _, _, _, _, _, _, _) => {
                h976
            }
        };
        let a979 = match v1.clone() {
            Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        nat_eq(a978, a979)
    } else {
        false
    } {
        let a982 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h980) => {
                h980
            }
        };
        let a983 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h981) => {
                h981
            }
        };
        nat_eq(a982, a983)
    } else {
        false
    } {
        let a986 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h984, _, _, _, _, _) => {
                h984
            }
        };
        let a987 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h985, _, _, _, _, _) => {
                h985
            }
        };
        nat_eq(a986, a987)
    } else {
        false
    } {
        let a990 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h988, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h989, _, _, _, _) => {
                h989
            }
        };
        nat_eq(a990, a991)
    } else {
        false
    } {
        let a994 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h992, _, _, _) => {
                h992
            }
        };
        let a995 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h993, _, _, _) => {
                h993
            }
        };
        nat_eq(a994, a995)
    } else {
        false
    } {
        let a998 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h996, _, _) => {
                h996
            }
        };
        let a999 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h997, _, _) => {
                h997
            }
        };
        nat_eq(a998, a999)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1000 = v0;
    let a1001 = 0u64;
    if if if nat_eq(a1000, a1001) {
        true
    } else {
        let a1002 = v1;
        let a1003 = 0u64;
        nat_eq(a1002, a1003)
    } {
        true
    } else {
        let a1004 = v2;
        let a1005 = 0u64;
        nat_eq(a1004, a1005)
    } {
        true
    } else {
        let a1006 = v3;
        let a1007 = 0u64;
        nat_eq(a1006, a1007)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f105({
        let a1009 = match v0.clone() {
            Adt2::C0(_, h1008, _, _, _, _, _, _, _, _, _) => {
                h1008
            }
        };
        let a1010 = v1;
        nat_mul(a1009, a1010)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1011, _, _, _, _, _, _, _, _) => {
            h1011
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1012, _, _, _, _, _, _, _) => {
            h1012
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
            h1013
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1014, _, _, _, _) => {
            h1014
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1015, _, _, _) => {
            h1015
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1016, _, _) => {
            h1016
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1017 = v1;
    let a1018 = 0u64;
    if if nat_eq(a1017, a1018) {
        true
    } else {
        let a1019 = v2;
        let a1020 = 0u64;
        nat_eq(a1019, a1020)
    } {
        Ok(0u64)
    } else {
        let a1027 = {
            let a1021 = f106(v1, v2, v3)?;
            let a1022 = f107(v0, v1, v2)?;
            nat_add(a1021, a1022)?
        };
        let a1028 = {
            let a1025 = f108(v0);
            let a1026 = {
                let a1023 = v1;
                let a1024 = v2;
                nat_mul(a1023, a1024)?
            };
            nat_mul(a1025, a1026)?
        };
        nat_add(a1027, a1028)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1029 = v0;
    let a1030 = 0u64;
    if if nat_eq(a1029, a1030) {
        true
    } else {
        let a1031 = v1;
        let a1032 = 0u64;
        nat_eq(a1031, a1032)
    } {
        Ok(3u64)
    } else {
        let a1035 = {
            let a1033 = 3u64;
            let a1034 = v1;
            nat_add(a1033, a1034)?
        };
        let a1036 = v2;
        nat_add(a1035, a1036)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1037 = v0;
    let a1038 = 2139095040u64;
    if nat_le(a1037, a1038) {
        true
    } else {
        let a1039 = v0;
        let a1040 = 2147483648u64;
        nat_eq(a1039, a1040)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1041 = v0;
    let a1042 = 0u64;
    if nat_eq(a1041, a1042) {
        Ok(0u64)
    } else {
        let a1043 = v1;
        let a1044 = f109(v0)?;
        nat_mul(a1043, a1044)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1045 = v0;
    let a1046 = 0u64;
    if if if if nat_eq(a1045, a1046) {
        true
    } else {
        let a1047 = v1;
        let a1048 = 0u64;
        nat_eq(a1047, a1048)
    } {
        true
    } else {
        let a1049 = v2;
        let a1050 = 0u64;
        nat_eq(a1049, a1050)
    } {
        true
    } else {
        let a1051 = v3;
        let a1052 = 0u64;
        nat_eq(a1051, a1052)
    } {
        Ok(0u64)
    } else {
        let a1065 = {
            let a1055 = 2u64;
            let a1056 = {
                let a1053 = v1;
                let a1054 = v2;
                nat_mul(a1053, a1054)?
            };
            nat_mul(a1055, a1056)?
        };
        let a1066 = {
            let a1063 = {
                let a1059 = {
                    let a1057 = v1;
                    let a1058 = v2;
                    nat_mul(a1057, a1058)?
                };
                let a1060 = v3;
                nat_mul(a1059, a1060)?
            };
            let a1064 = {
                let a1061 = 1u64;
                let a1062 = v0;
                nat_add(a1061, a1062)?
            };
            nat_mul(a1063, a1064)?
        };
        nat_add(a1065, a1066)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h1067, _, _, _, _, _, _, _, _, _) => {
            h1067
        }
    }, match v0.clone() {
        Adt1::C0(h1068, _, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1069, _, _, _, _, _, _, _, _, _) => {
            h1069
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1070, _, _, _, _, _, _, _) => {
            h1070
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h1071, _, _, _, _, _, _, _, _) => {
            h1071
        }
    }, match v0.clone() {
        Adt1::C0(_, h1072, _, _, _, _, _, _, _, _, _, _) => {
            h1072
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1073, _, _, _, _, _, _, _, _) => {
            h1073
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h1074, _, _, _, _, _, _) => {
            h1074
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
            h1075
        }
    };
    let v7: bool = {
        let a1078 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1076, _, _, _) => {
                h1076
            }
        };
        let a1079 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1077, _, _, _, _, _) => {
                h1077
            }
        };
        nat_lt(a1078, a1079)
    };
    let a1080 = v6;
    let a1081 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a1080, a1081) {
        true
    } else {
        let a1082 = v6;
        let a1083 = 1u64;
        nat_eq(a1082, a1083)
    } {
        let a1085 = match v2.clone() {
            Adt2::C0(h1084, _, _, _, _, _, _, _, _, _, _) => {
                h1084
            }
        };
        let a1086 = 0u64;
        nat_eq(a1085, a1086)
    } else {
        false
    } {
        let a1088 = match v3.clone() {
            Adt2::C0(h1087, _, _, _, _, _, _, _, _, _, _) => {
                h1087
            }
        };
        let a1089 = 0u64;
        nat_eq(a1088, a1089)
    } else {
        false
    } {
        let a1091 = match v1.clone() {
            Adt2::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1092 = 1u64;
        nat_eq(a1091, a1092)
    } else {
        false
    } {
        let a1095 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1093, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1096 = match v2.clone() {
            Adt2::C0(_, _, _, h1094, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        nat_eq(a1095, a1096)
    } else {
        false
    } {
        let a1099 = match v3.clone() {
            Adt2::C0(_, _, _, h1097, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1100 = match v2.clone() {
            Adt2::C0(_, _, _, h1098, _, _, _, _, _, _, _) => {
                h1098
            }
        };
        nat_eq(a1099, a1100)
    } else {
        false
    } {
        let a1103 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1101, _, _, _, _, _, _) => {
                h1101
            }
        };
        let a1104 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1102, _, _, _, _, _, _) => {
                h1102
            }
        };
        nat_eq(a1103, a1104)
    } else {
        false
    } {
        let a1106 = 0u64;
        let a1107 = match v4.clone() {
            Adt6::C0(h1105, _, _, _) => {
                h1105
            }
        };
        if if if if if nat_lt(a1106, a1107) {
            let a1109 = 0u64;
            let a1110 = match v4.clone() {
                Adt6::C0(_, h1108, _, _) => {
                    h1108
                }
            };
            nat_lt(a1109, a1110)
        } else {
            false
        } {
            let a1112 = match v4.clone() {
                Adt6::C0(_, h1111, _, _) => {
                    h1111
                }
            };
            let a1113 = 2147483648u64;
            nat_lt(a1112, a1113)
        } else {
            false
        } {
            let a1115 = 0u64;
            let a1116 = match v4.clone() {
                Adt6::C0(_, _, _, h1114) => {
                    h1114
                }
            };
            nat_lt(a1115, a1116)
        } else {
            false
        } {
            let a1118 = match v4.clone() {
                Adt6::C0(_, _, _, h1117) => {
                    h1117
                }
            };
            let a1119 = 2147483648u64;
            nat_lt(a1118, a1119)
        } else {
            false
        } {
            let a1122 = match v3.clone() {
                Adt2::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                    h1120
                }
            };
            let a1123 = f98(v4.clone(), match v2.clone() {
                Adt2::C0(_, h1121, _, _, _, _, _, _, _, _, _) => {
                    h1121
                }
            })?;
            nat_eq(a1122, a1123)
        } else {
            false
        }
    } else {
        false
    } {
        let a1125 = 0u64;
        let a1126 = match v5.clone() {
            Adt6::C0(h1124, _, _, _) => {
                h1124
            }
        };
        if if if if if nat_lt(a1125, a1126) {
            let a1128 = 0u64;
            let a1129 = match v5.clone() {
                Adt6::C0(_, h1127, _, _) => {
                    h1127
                }
            };
            nat_lt(a1128, a1129)
        } else {
            false
        } {
            let a1131 = match v5.clone() {
                Adt6::C0(_, h1130, _, _) => {
                    h1130
                }
            };
            let a1132 = 2147483648u64;
            nat_lt(a1131, a1132)
        } else {
            false
        } {
            let a1134 = 0u64;
            let a1135 = match v5.clone() {
                Adt6::C0(_, _, _, h1133) => {
                    h1133
                }
            };
            nat_lt(a1134, a1135)
        } else {
            false
        } {
            let a1137 = match v5.clone() {
                Adt6::C0(_, _, _, h1136) => {
                    h1136
                }
            };
            let a1138 = 2147483648u64;
            nat_lt(a1137, a1138)
        } else {
            false
        } {
            let a1141 = match v3.clone() {
                Adt2::C0(_, _, h1139, _, _, _, _, _, _, _, _) => {
                    h1139
                }
            };
            let a1142 = f98(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1140, _, _, _, _, _, _, _, _) => {
                    h1140
                }
            })?;
            nat_eq(a1141, a1142)
        } else {
            false
        }
    } else {
        false
    } {
        f110(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f110(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f110(v1.clone(), {
            let a1143 = v6;
            let a1144 = 0u64;
            if nat_eq(a1143, a1144) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f23(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f23(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a1153 = {
                let a1151 = {
                    let a1149 = 3u64;
                    let a1150 = {
                        let a1147 = match v4.clone() {
                            Adt6::C0(h1145, _, _, _) => {
                                h1145
                            }
                        };
                        let a1148 = match v5.clone() {
                            Adt6::C0(h1146, _, _, _) => {
                                h1146
                            }
                        };
                        nat_mul(a1147, a1148)?
                    };
                    nat_mul(a1149, a1150)?
                };
                let a1152 = 2u64;
                nat_add(a1151, a1152)?
            };
            let a1154 = f16(v3.clone(), 1u64)?;
            nat_mul(a1153, a1154)?
        }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C34(f20(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f42(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f25(v0.clone(), v1.clone()) {
        f25(v0.clone(), v2.clone())
    } else {
        false
    } {
        f111(v0.clone())?
    } else {
        false
    } {
        f111(v1.clone())?
    } else {
        false
    } {
        f111(v2.clone())?
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
            let a1155 = 6u64;
            let a1156 = f16(v0.clone(), 1u64)?;
            nat_mul(a1155, a1156)?
        }, 2u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C33(f20(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f43(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1157 = v2;
    match m1157 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h1158, _, _, _, _, _, _, _, _, _) => {
                    h1158
                }
            };
            let v6: u64 = {
                let a1164 = match v1.clone() {
                    Adt2::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                        h1159
                    }
                };
                let a1165 = {
                    let a1162 = match v1.clone() {
                        Adt2::C0(_, _, _, h1160, _, _, _, _, _, _, _) => {
                            h1160
                        }
                    };
                    let a1163 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1161, _, _, _, _, _, _) => {
                            h1161
                        }
                    };
                    nat_mul(a1162, a1163)?
                };
                nat_mul(a1164, a1165)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f112(v1.clone(), v6)?
            } else {
                false
            } {
                f112(v4.clone(), v6)?
            } else {
                false
            } {
                f112(v3.clone(), v6)?
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
                let a1166 = v5;
                let a1167 = 2147483647u64;
                nat_le(a1166, a1167)
            } else {
                false
            } {
                let a1168 = v6;
                let a1169 = 2147483647u64;
                nat_le(a1168, a1169)
            } else {
                false
            } {
                let a1171 = match v0.clone() {
                    Adt1::C0(h1170, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1170
                    }
                };
                let a1172 = 4294967295u64;
                if nat_le(a1171, a1172) {
                    let a1174 = match v0.clone() {
                        Adt1::C0(_, h1173, _, _, _, _, _, _, _, _, _, _) => {
                            h1173
                        }
                    };
                    let a1175 = 4294967295u64;
                    nat_le(a1174, a1175)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1176 = v5;
                    let a1177 = 0u64;
                    if nat_eq(a1176, a1177) {
                        0u64
                    } else {
                        let a1178 = v6;
                        let a1179 = f113(v5)?;
                        nat_mul(a1178, a1179)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h1180, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1180
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f44(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h1181, _, _, _, _, _, _, _, _, _, _, _) => {
            h1181
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1182, _, _, _, _, _, _, _, _, _, _) => {
            h1182
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1183, _, _, _, _, _, _, _, _, _) => {
            h1183
        }
    };
    let v6: u64 = {
        let a1185 = match v1.clone() {
            Adt2::C0(_, h1184, _, _, _, _, _, _, _, _, _) => {
                h1184
            }
        };
        let a1186 = v4;
        let a1187 = 0u64;
        nat_quot(a1185, a1186, a1187)
    };
    let v7: u64 = {
        let a1189 = match v1.clone() {
            Adt2::C0(h1188, _, _, _, _, _, _, _, _, _, _) => {
                h1188
            }
        };
        let a1190 = 0u64;
        if nat_eq(a1189, a1190) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f114(match v1.clone() {
        Adt2::C0(h1191, _, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    }) {
        let a1194 = match v1.clone() {
            Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        let a1195 = match v2.clone() {
            Adt2::C0(h1193, _, _, _, _, _, _, _, _, _, _) => {
                h1193
            }
        };
        nat_eq(a1194, a1195)
    } else {
        false
    } {
        let a1196 = 0u64;
        let a1197 = v3;
        nat_lt(a1196, a1197)
    } else {
        false
    } {
        let a1198 = v3;
        let a1199 = 2147483648u64;
        nat_lt(a1198, a1199)
    } else {
        false
    } {
        let a1200 = 0u64;
        let a1201 = v4;
        nat_lt(a1200, a1201)
    } else {
        false
    } {
        let a1202 = v4;
        let a1203 = 2147483648u64;
        nat_lt(a1202, a1203)
    } else {
        false
    } {
        let a1204 = v5;
        let a1205 = 2147483648u64;
        nat_lt(a1204, a1205)
    } else {
        false
    } {
        let a1206 = 0u64;
        let a1207 = v6;
        nat_lt(a1206, a1207)
    } else {
        false
    } {
        let a1209 = 0u64;
        let a1210 = match v2.clone() {
            Adt2::C0(_, h1208, _, _, _, _, _, _, _, _, _) => {
                h1208
            }
        };
        nat_lt(a1209, a1210)
    } else {
        false
    } {
        let a1214 = match v1.clone() {
            Adt2::C0(_, h1211, _, _, _, _, _, _, _, _, _) => {
                h1211
            }
        };
        let a1215 = {
            let a1212 = v6;
            let a1213 = v4;
            nat_mul(a1212, a1213)?
        };
        nat_eq(a1214, a1215)
    } else {
        false
    } {
        let a1217 = match v2.clone() {
            Adt2::C0(_, _, h1216, _, _, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1218 = v4;
        nat_eq(a1217, a1218)
    } else {
        false
    } {
        let a1231 = {
            let a1224 = {
                let a1222 = match v2.clone() {
                    Adt2::C0(_, h1219, _, _, _, _, _, _, _, _, _) => {
                        h1219
                    }
                };
                let a1223 = {
                    let a1220 = v5;
                    let a1221 = 2u64;
                    nat_mul(a1220, a1221)?
                };
                nat_add(a1222, a1223)?
            };
            let a1225 = v3;
            nat_add(a1224, a1225)?
        };
        let a1232 = {
            let a1229 = {
                let a1227 = match v1.clone() {
                    Adt2::C0(_, _, h1226, _, _, _, _, _, _, _, _) => {
                        h1226
                    }
                };
                let a1228 = v3;
                nat_mul(a1227, a1228)?
            };
            let a1230 = v6;
            nat_add(a1229, a1230)?
        };
        nat_eq(a1231, a1232)
    } else {
        false
    } {
        f115(v1.clone(), v7)?
    } else {
        false
    } {
        f115(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1233 = f16(v2.clone(), 1u64)?;
            let a1234 = f116(v8.clone())?;
            nat_mul(a1233, a1234)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1235, _, _, _, _, _, _, _, _, _, _) => {
                h1235
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f45(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1236 = v1;
    match m1236 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1238 = match v0.clone() {
                Adt2::C0(h1237, _, _, _, _, _, _, _, _, _, _) => {
                    h1237
                }
            };
            let a1239 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1238, a1239) {
                let a1241 = match v3.clone() {
                    Adt2::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                        h1240
                    }
                };
                let a1242 = 0u64;
                nat_eq(a1241, a1242)
            } else {
                false
            } {
                let a1244 = match v2.clone() {
                    Adt2::C0(h1243, _, _, _, _, _, _, _, _, _, _) => {
                        h1243
                    }
                };
                let a1245 = 0u64;
                nat_eq(a1244, a1245)
            } else {
                false
            } {
                let a1247 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1246, _, _, _, _, _, _) => {
                        h1246
                    }
                };
                let a1248 = 1u64;
                nat_eq(a1247, a1248)
            } else {
                false
            } {
                let a1250 = match v3.clone() {
                    Adt2::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                        h1249
                    }
                };
                let a1251 = 1u64;
                nat_eq(a1250, a1251)
            } else {
                false
            } {
                let a1253 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1252, _, _, _, _, _, _) => {
                        h1252
                    }
                };
                let a1254 = 1u64;
                nat_eq(a1253, a1254)
            } else {
                false
            } {
                let a1257 = match v0.clone() {
                    Adt2::C0(_, _, h1255, _, _, _, _, _, _, _, _) => {
                        h1255
                    }
                };
                let a1258 = match v3.clone() {
                    Adt2::C0(_, _, h1256, _, _, _, _, _, _, _, _) => {
                        h1256
                    }
                };
                nat_eq(a1257, a1258)
            } else {
                false
            } {
                let a1263 = match v3.clone() {
                    Adt2::C0(_, h1259, _, _, _, _, _, _, _, _, _) => {
                        h1259
                    }
                };
                let a1264 = {
                    let a1261 = match v0.clone() {
                        Adt2::C0(_, h1260, _, _, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    };
                    let a1262 = 1u64;
                    nat_add(a1261, a1262)?
                };
                nat_le(a1263, a1264)
            } else {
                false
            } {
                let a1267 = match v2.clone() {
                    Adt2::C0(_, h1265, _, _, _, _, _, _, _, _, _) => {
                        h1265
                    }
                };
                let a1268 = match v0.clone() {
                    Adt2::C0(_, _, h1266, _, _, _, _, _, _, _, _) => {
                        h1266
                    }
                };
                nat_eq(a1267, a1268)
            } else {
                false
            } {
                let a1276 = match v2.clone() {
                    Adt2::C0(_, _, h1269, _, _, _, _, _, _, _, _) => {
                        h1269
                    }
                };
                let a1277 = {
                    let a1274 = {
                        let a1271 = match v0.clone() {
                            Adt2::C0(_, h1270, _, _, _, _, _, _, _, _, _) => {
                                h1270
                            }
                        };
                        let a1272 = 1u64;
                        nat_add(a1271, a1272)?
                    };
                    let a1275 = match v3.clone() {
                        Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                            h1273
                        }
                    };
                    nat_sub(a1274, a1275)
                };
                nat_eq(a1276, a1277)
            } else {
                false
            } {
                let a1280 = match v2.clone() {
                    Adt2::C0(_, _, _, h1278, _, _, _, _, _, _, _) => {
                        h1278
                    }
                };
                let a1281 = match v0.clone() {
                    Adt2::C0(_, _, _, h1279, _, _, _, _, _, _, _) => {
                        h1279
                    }
                };
                nat_eq(a1280, a1281)
            } else {
                false
            } {
                let a1283 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1282, _, _, _, _, _, _) => {
                        h1282
                    }
                };
                let a1284 = 1u64;
                nat_eq(a1283, a1284)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1286 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1285, _, _, _, _, _) => {
                        h1285
                    }
                };
                let a1287 = 4u64;
                nat_eq(a1286, a1287)
            } else {
                false
            } {
                let a1289 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1288, _, _, _, _, _) => {
                        h1288
                    }
                };
                let a1290 = 4u64;
                nat_eq(a1289, a1290)
            } else {
                false
            } {
                let a1295 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1291, _, _, _, _) => {
                        h1291
                    }
                };
                let a1296 = {
                    let a1293 = match v0.clone() {
                        Adt2::C0(_, h1292, _, _, _, _, _, _, _, _, _) => {
                            h1292
                        }
                    };
                    let a1294 = 4u64;
                    nat_mul(a1293, a1294)?
                };
                nat_eq(a1295, a1296)
            } else {
                false
            } {
                let a1301 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1297, _, _, _, _) => {
                        h1297
                    }
                };
                let a1302 = {
                    let a1299 = match v3.clone() {
                        Adt2::C0(_, h1298, _, _, _, _, _, _, _, _, _) => {
                            h1298
                        }
                    };
                    let a1300 = 4u64;
                    nat_mul(a1299, a1300)?
                };
                nat_eq(a1301, a1302)
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
                    let a1304 = f16(v2.clone(), 1u64)?;
                    let a1305 = f117(match v3.clone() {
                        Adt2::C0(_, h1303, _, _, _, _, _, _, _, _, _) => {
                            h1303
                        }
                    })?;
                    nat_mul(a1304, a1305)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1306, _, _, _, _, _, _, _, _, _, _) => {
            h1306
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1307, _, _, _, _, _, _, _, _) => {
                h1307
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1308, _, _, _, _, _, _, _, _, _) => {
                h1308
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1309, _, _, _, _, _, _) => {
                h1309
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1310, _, _, _, _, _, _, _, _) => {
                h1310
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1311, _, _, _, _, _, _, _, _, _) => {
                h1311
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1312, _, _, _, _, _, _, _) => {
                h1312
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1313, _, _, _, _, _) => {
                h1313
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1315 = match v1.clone() {
        Adt1::C0(h1314, _, _, _, _, _, _, _, _, _, _, _) => {
            h1314
        }
    };
    let a1316 = 1u64;
    if if if if if if if if if if if nat_le(a1315, a1316) {
        let a1318 = match v2.clone() {
            Adt2::C0(h1317, _, _, _, _, _, _, _, _, _, _) => {
                h1317
            }
        };
        let a1319 = 1u64;
        nat_le(a1318, a1319)
    } else {
        false
    } {
        let a1321 = match v3.clone() {
            Adt2::C0(h1320, _, _, _, _, _, _, _, _, _, _) => {
                h1320
            }
        };
        let a1322 = 0u64;
        nat_eq(a1321, a1322)
    } else {
        false
    } {
        let a1327 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1323, _, _, _, _, _) => {
                h1323
            }
        };
        let a1328 = {
            let a1325 = match v2.clone() {
                Adt2::C0(h1324, _, _, _, _, _, _, _, _, _, _) => {
                    h1324
                }
            };
            let a1326 = 0u64;
            if nat_eq(a1325, a1326) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1327, a1328)
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
        f118(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1329, _, _, _, _, _, _, _, _, _) => {
                h1329
            }
        }, match v3.clone() {
            Adt2::C0(_, h1330, _, _, _, _, _, _, _, _, _) => {
                h1330
            }
        })?
    } else {
        false
    } {
        if v0 {
            f118(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1331, _, _, _, _, _, _, _, _) => {
                    h1331
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1332, _, _, _, _, _, _, _, _) => {
                    h1332
                }
            })?
        } else {
            let a1335 = match v2.clone() {
                Adt2::C0(_, _, h1333, _, _, _, _, _, _, _, _) => {
                    h1333
                }
            };
            let a1336 = match v3.clone() {
                Adt2::C0(_, _, h1334, _, _, _, _, _, _, _, _) => {
                    h1334
                }
            };
            nat_eq(a1335, a1336)
        }
    } else {
        false
    } {
        let a1339 = match v2.clone() {
            Adt2::C0(_, _, _, h1337, _, _, _, _, _, _, _) => {
                h1337
            }
        };
        let a1340 = match v3.clone() {
            Adt2::C0(_, _, _, h1338, _, _, _, _, _, _, _) => {
                h1338
            }
        };
        nat_eq(a1339, a1340)
    } else {
        false
    } {
        let a1343 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1341, _, _, _, _, _, _) => {
                h1341
            }
        };
        let a1344 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1342, _, _, _, _, _, _) => {
                h1342
            }
        };
        nat_eq(a1343, a1344)
    } else {
        false
    } {
        let a1349 = {
            let a1347 = match v4.clone() {
                Adt6::C0(h1345, _, _, _) => {
                    h1345
                }
            };
            let a1348 = match v5.clone() {
                Adt6::C0(h1346, _, _, _) => {
                    h1346
                }
            };
            nat_mul(a1347, a1348)?
        };
        let a1350 = 2147483648u64;
        nat_lt(a1349, a1350)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1352 = match v1 {
                Adt1::C0(h1351, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1351
                }
            };
            let a1353 = 0u64;
            nat_eq(a1352, a1353)
        }, match v2.clone() {
            Adt2::C0(h1354, _, _, _, _, _, _, _, _, _, _) => {
                h1354
            }
        });
        Ok(Adt10::C4({
            let a1355 = f16(v3.clone(), 1u64)?;
            let a1356 = f119(v6.clone())?;
            nat_mul(a1355, a1356)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f47(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1357 = v2;
    match m1357 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1359 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1358, _, _, _, _, _) => {
                        h1358
                    }
                };
                let a1360 = 1u64;
                nat_eq(a1359, a1360)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1361, _, _, _, _, _, _, _, _, _) => {
                    h1361
                }
            }, match v0.clone() {
                Adt1::C0(h1362, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1362
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1363, _, _, _, _, _, _, _, _, _) => {
                    h1363
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1364, _, _, _, _, _, _, _) => {
                    h1364
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1365, _, _, _, _, _, _, _, _) => {
                        h1365
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1366, _, _, _, _, _, _, _, _, _, _) => {
                        h1366
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1367, _, _, _, _, _, _, _, _) => {
                        h1367
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1368, _, _, _, _, _, _) => {
                        h1368
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1369, _, _, _, _, _, _, _) => {
                        h1369
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1370, _, _, _, _, _, _, _, _) => {
                        h1370
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1371, _, _, _, _, _, _) => {
                        h1371
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1372, _, _, _, _, _, _, _) => {
                        h1372
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1373, _, _, _, _, _, _, _, _) => {
                        h1373
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1374, _, _, _, _, _, _, _, _, _, _) => {
                    h1374
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1375, _, _, _, _, _, _, _, _, _, _) => {
                    h1375
                }
            };
            let v14: u64 = {
                let a1376 = v12;
                let a1377 = 0u64;
                if nat_eq(a1376, a1377) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1379 = match v4.clone() {
                    Adt2::C0(_, h1378, _, _, _, _, _, _, _, _, _) => {
                        h1378
                    }
                };
                let a1380 = v14;
                nat_mul(a1379, a1380)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1381, _, _, _) => {
                        h1381
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1382, _, _, _, _) => {
                        h1382
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1383, _, _) => {
                        h1383
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1384, _, _, _) => {
                        h1384
                    }
                }
            };
            let a1386 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1385, _, _, _, _, _) => {
                    h1385
                }
            };
            let a1387 = 1u64;
            if if if if if if if if if if nat_le(a1386, a1387) {
                let a1388 = v12;
                let a1389 = 0u64;
                if nat_eq(a1388, a1389) {
                    true
                } else {
                    let a1390 = v12;
                    let a1391 = 1u64;
                    if nat_eq(a1390, a1391) {
                        let a1392 = v13;
                        let a1393 = 1u64;
                        nat_eq(a1392, a1393)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1394 = v13;
                let a1395 = 0u64;
                if nat_eq(a1394, a1395) {
                    true
                } else {
                    let a1396 = v13;
                    let a1397 = 1u64;
                    nat_eq(a1396, a1397)
                }
            } else {
                false
            } {
                let a1399 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1398, _, _, _, _, _) => {
                        h1398
                    }
                };
                let a1400 = v14;
                nat_eq(a1399, a1400)
            } else {
                false
            } {
                let a1408 = match v3.clone() {
                    Adt2::C0(_, h1401, _, _, _, _, _, _, _, _, _) => {
                        h1401
                    }
                };
                let a1409 = {
                    let a1406 = v9;
                    let a1407 = {
                        let a1404 = match v6.clone() {
                            Adt6::C0(h1402, _, _, _) => {
                                h1402
                            }
                        };
                        let a1405 = match v7.clone() {
                            Adt6::C0(h1403, _, _, _) => {
                                h1403
                            }
                        };
                        nat_mul(a1404, a1405)?
                    };
                    nat_mul(a1406, a1407)?
                };
                nat_eq(a1408, a1409)
            } else {
                false
            } {
                let a1412 = match v3.clone() {
                    Adt2::C0(_, _, h1410, _, _, _, _, _, _, _, _) => {
                        h1410
                    }
                };
                let a1413 = f98(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1411, _, _, _, _, _, _, _, _, _) => {
                        h1411
                    }
                })?;
                nat_eq(a1412, a1413)
            } else {
                false
            } {
                let a1415 = 0u64;
                let a1416 = match v3.clone() {
                    Adt2::C0(_, _, h1414, _, _, _, _, _, _, _, _) => {
                        h1414
                    }
                };
                nat_lt(a1415, a1416)
            } else {
                false
            } {
                if v5 {
                    let a1418 = match v1 {
                        Adt2::C0(_, _, _, h1417, _, _, _, _, _, _, _) => {
                            h1417
                        }
                    };
                    let a1419 = v9;
                    if if if nat_eq(a1418, a1419) {
                        let a1421 = match v3.clone() {
                            Adt2::C0(_, _, _, h1420, _, _, _, _, _, _, _) => {
                                h1420
                            }
                        };
                        let a1422 = f98(v7.clone(), v11)?;
                        nat_eq(a1421, a1422)
                    } else {
                        false
                    } {
                        let a1424 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1423, _, _, _, _, _, _) => {
                                h1423
                            }
                        };
                        let a1425 = v10;
                        nat_eq(a1424, a1425)
                    } else {
                        false
                    } {
                        let a1427 = 0u64;
                        let a1428 = match v3.clone() {
                            Adt2::C0(_, _, _, h1426, _, _, _, _, _, _, _) => {
                                h1426
                            }
                        };
                        nat_lt(a1427, a1428)
                    } else {
                        false
                    }
                } else {
                    let a1430 = match v1 {
                        Adt2::C0(_, _, h1429, _, _, _, _, _, _, _, _) => {
                            h1429
                        }
                    };
                    let a1431 = v9;
                    if if if nat_eq(a1430, a1431) {
                        let a1433 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1432, _, _, _, _, _, _) => {
                                h1432
                            }
                        };
                        let a1434 = 1u64;
                        nat_eq(a1433, a1434)
                    } else {
                        false
                    } {
                        let a1436 = match v3.clone() {
                            Adt2::C0(_, _, _, h1435, _, _, _, _, _, _, _) => {
                                h1435
                            }
                        };
                        let a1437 = v10;
                        nat_eq(a1436, a1437)
                    } else {
                        false
                    } {
                        let a1439 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1438, _, _, _, _, _, _) => {
                                h1438
                            }
                        };
                        let a1440 = 1u64;
                        nat_eq(a1439, a1440)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1441, _, _, _, _, _, _, _, _, _) => {
                        h1441
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1461 = {
                        let a1458 = {
                            let a1456 = {
                                let a1450 = {
                                    let a1444 = {
                                        let a1442 = v10;
                                        let a1443 = 1u64;
                                        nat_sub(a1442, a1443)
                                    };
                                    let a1445 = v17;
                                    nat_mul(a1444, a1445)?
                                };
                                let a1451 = {
                                    let a1448 = {
                                        let a1446 = v9;
                                        let a1447 = 1u64;
                                        nat_sub(a1446, a1447)
                                    };
                                    let a1449 = v16;
                                    nat_mul(a1448, a1449)?
                                };
                                nat_add(a1450, a1451)?
                            };
                            let a1457 = {
                                let a1454 = {
                                    let a1452 = v11;
                                    let a1453 = 1u64;
                                    nat_sub(a1452, a1453)
                                };
                                let a1455 = v15;
                                nat_mul(a1454, a1455)?
                            };
                            nat_add(a1456, a1457)?
                        };
                        let a1459 = v15;
                        nat_add(a1458, a1459)?
                    };
                    let a1462 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1460, _) => {
                            h1460
                        }
                    };
                    nat_le(a1461, a1462)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1463 = v13;
                    let a1464 = 0u64;
                    if nat_eq(a1463, a1464) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1465, _, _, _, _, _, _, _, _, _) => {
                        h1465
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1466, _, _, _, _, _, _, _, _, _) => {
                            h1466
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1467, _, _, _, _, _, _, _, _) => {
                            h1467
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1468, _, _, _, _, _, _, _) => {
                            h1468
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1469, _, _, _, _, _) => {
                            h1469
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1470, _, _, _, _) => {
                            h1470
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1471, _, _, _) => {
                            h1471
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f48(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1472 = v2;
    match m1472 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1473, _, _, _, _, _, _, _, _, _) => {
                    h1473
                }
            }, match v0.clone() {
                Adt1::C0(h1474, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1474
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1475, _, _, _, _, _, _, _, _) => {
                    h1475
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1476, _, _, _, _, _) => {
                    h1476
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1477, _, _, _, _, _, _, _, _) => {
                    h1477
                }
            }, match v0.clone() {
                Adt1::C0(_, h1478, _, _, _, _, _, _, _, _, _, _) => {
                    h1478
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1479, _, _, _, _, _, _, _) => {
                    h1479
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1480, _, _, _, _) => {
                    h1480
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1481, _, _, _, _, _, _, _) => {
                    h1481
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1482, _, _, _, _, _, _, _, _, _) => {
                    h1482
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1483, _, _, _, _, _, _) => {
                    h1483
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1484, _, _, _) => {
                    h1484
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1485, _, _) => {
                    h1485
                }
            };
            let v9: u64 = f98(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1486, _, _, _, _, _, _, _) => {
                    h1486
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1487, _, _, _, _, _, _, _, _, _, _) => {
                    h1487
                }
            };
            let v11: u64 = {
                let a1495 = v8;
                let a1496 = {
                    let a1493 = match v5.clone() {
                        Adt6::C0(h1488, _, _, _) => {
                            h1488
                        }
                    };
                    let a1494 = {
                        let a1491 = match v6.clone() {
                            Adt6::C0(h1489, _, _, _) => {
                                h1489
                            }
                        };
                        let a1492 = match v7.clone() {
                            Adt6::C0(h1490, _, _, _) => {
                                h1490
                            }
                        };
                        nat_mul(a1491, a1492)?
                    };
                    nat_mul(a1493, a1494)?
                };
                nat_mul(a1495, a1496)?
            };
            let a1498 = match v4.clone() {
                Adt2::C0(h1497, _, _, _, _, _, _, _, _, _, _) => {
                    h1497
                }
            };
            let a1499 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1498, a1499) {
                let a1500 = v10;
                let a1501 = 0u64;
                if nat_eq(a1500, a1501) {
                    true
                } else {
                    let a1502 = v10;
                    let a1503 = 1u64;
                    nat_eq(a1502, a1503)
                }
            } else {
                false
            } {
                let a1504 = 0u64;
                let a1505 = v8;
                nat_lt(a1504, a1505)
            } else {
                false
            } {
                let a1506 = v8;
                let a1507 = 2147483648u64;
                nat_lt(a1506, a1507)
            } else {
                false
            } {
                let a1509 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1508, _, _, _, _, _) => {
                        h1508
                    }
                };
                let a1510 = 4u64;
                nat_eq(a1509, a1510)
            } else {
                false
            } {
                let a1511 = 0u64;
                let a1512 = v9;
                nat_lt(a1511, a1512)
            } else {
                false
            } {
                let a1514 = 0u64;
                let a1515 = match v3.clone() {
                    Adt2::C0(_, _, h1513, _, _, _, _, _, _, _, _) => {
                        h1513
                    }
                };
                nat_lt(a1514, a1515)
            } else {
                false
            } {
                let a1517 = 0u64;
                let a1518 = match v3.clone() {
                    Adt2::C0(_, _, _, h1516, _, _, _, _, _, _, _) => {
                        h1516
                    }
                };
                nat_lt(a1517, a1518)
            } else {
                false
            } {
                let a1520 = match v3.clone() {
                    Adt2::C0(_, h1519, _, _, _, _, _, _, _, _, _) => {
                        h1519
                    }
                };
                let a1521 = v11;
                nat_eq(a1520, a1521)
            } else {
                false
            } {
                let a1524 = match v3.clone() {
                    Adt2::C0(_, _, h1522, _, _, _, _, _, _, _, _) => {
                        h1522
                    }
                };
                let a1525 = f98(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1523, _, _, _, _, _, _, _, _, _) => {
                        h1523
                    }
                })?;
                nat_eq(a1524, a1525)
            } else {
                false
            } {
                let a1528 = match v3.clone() {
                    Adt2::C0(_, _, _, h1526, _, _, _, _, _, _, _) => {
                        h1526
                    }
                };
                let a1529 = f98(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1527, _, _, _, _, _, _, _, _) => {
                        h1527
                    }
                })?;
                nat_eq(a1528, a1529)
            } else {
                false
            } {
                let a1537 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1530, _, _, _, _, _, _) => {
                        h1530
                    }
                };
                let a1538 = {
                    let a1535 = v9;
                    let a1536 = {
                        let a1532 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1531, _, _, _, _, _, _) => {
                                h1531
                            }
                        };
                        let a1533 = v8;
                        let a1534 = 0u64;
                        nat_quot(a1532, a1533, a1534)
                    };
                    nat_mul(a1535, a1536)?
                };
                nat_eq(a1537, a1538)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1539 = v10;
                    let a1540 = 0u64;
                    if nat_eq(a1539, a1540) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1541, _, _) => {
                        h1541
                    }
                }, {
                    let a1543 = v8;
                    let a1544 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1542, _, _) => {
                            h1542
                        }
                    };
                    nat_mul(a1543, a1544)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f49(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1545 = v1;
    match m1545 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f120(v0, v3, v2)
        }
    }
}

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1547 = match v0.clone() {
        Adt2::C0(h1546, _, _, _, _, _, _, _, _, _, _) => {
            h1546
        }
    };
    let a1548 = 1u64;
    if if nat_eq(a1547, a1548) {
        true
    } else {
        let a1550 = match v0.clone() {
            Adt2::C0(h1549, _, _, _, _, _, _, _, _, _, _) => {
                h1549
            }
        };
        let a1551 = 30u64;
        nat_eq(a1550, a1551)
    } {
        let a1553 = match v1.clone() {
            Adt2::C0(h1552, _, _, _, _, _, _, _, _, _, _) => {
                h1552
            }
        };
        let a1554 = 1u64;
        if if if if if if if if if if if nat_eq(a1553, a1554) {
            let a1557 = match v0.clone() {
                Adt2::C0(_, h1555, _, _, _, _, _, _, _, _, _) => {
                    h1555
                }
            };
            let a1558 = match v1.clone() {
                Adt2::C0(_, h1556, _, _, _, _, _, _, _, _, _) => {
                    h1556
                }
            };
            nat_eq(a1557, a1558)
        } else {
            false
        } {
            let a1560 = 0u64;
            let a1561 = match v1.clone() {
                Adt2::C0(_, _, h1559, _, _, _, _, _, _, _, _) => {
                    h1559
                }
            };
            nat_lt(a1560, a1561)
        } else {
            false
        } {
            let a1564 = match v1.clone() {
                Adt2::C0(_, _, h1562, _, _, _, _, _, _, _, _) => {
                    h1562
                }
            };
            let a1565 = match v1.clone() {
                Adt2::C0(_, _, _, h1563, _, _, _, _, _, _, _) => {
                    h1563
                }
            };
            nat_eq(a1564, a1565)
        } else {
            false
        } {
            let a1567 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1566, _, _, _, _, _, _) => {
                    h1566
                }
            };
            let a1568 = 1u64;
            nat_eq(a1567, a1568)
        } else {
            false
        } {
            let a1576 = match v0.clone() {
                Adt2::C0(_, _, h1569, _, _, _, _, _, _, _, _) => {
                    h1569
                }
            };
            let a1577 = {
                let a1574 = {
                    let a1572 = match v1.clone() {
                        Adt2::C0(_, _, h1570, _, _, _, _, _, _, _, _) => {
                            h1570
                        }
                    };
                    let a1573 = match v1.clone() {
                        Adt2::C0(_, _, h1571, _, _, _, _, _, _, _, _) => {
                            h1571
                        }
                    };
                    nat_add(a1572, a1573)?
                };
                let a1575 = 1u64;
                nat_sub(a1574, a1575)
            };
            nat_eq(a1576, a1577)
        } else {
            false
        } {
            let a1579 = 0u64;
            let a1580 = match v0.clone() {
                Adt2::C0(_, _, _, h1578, _, _, _, _, _, _, _) => {
                    h1578
                }
            };
            nat_lt(a1579, a1580)
        } else {
            false
        } {
            let a1582 = 0u64;
            let a1583 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1581, _, _, _, _, _, _) => {
                    h1581
                }
            };
            nat_lt(a1582, a1583)
        } else {
            false
        } {
            let a1585 = 2u64;
            let a1586 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1584, _, _, _, _, _) => {
                    h1584
                }
            };
            nat_le(a1585, a1586)
        } else {
            false
        } {
            let a1588 = 2u64;
            let a1589 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1587, _, _, _, _, _) => {
                    h1587
                }
            };
            nat_le(a1588, a1589)
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

pub fn f51(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1590 = v1;
    match m1590 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f121(v0, v3, v2)
        }
    }
}

pub fn f52(v0: u64) -> bool {
    let a1591 = v0;
    let a1592 = 1u64;
    if if nat_eq(a1591, a1592) {
        true
    } else {
        let a1593 = v0;
        let a1594 = 33u64;
        nat_eq(a1593, a1594)
    } {
        true
    } else {
        let a1595 = v0;
        let a1596 = 34u64;
        nat_eq(a1595, a1596)
    }
}

pub fn f53(v0: u64) -> bool {
    if f122(v0) {
        !f123(v0)
    } else {
        false
    }
}

pub fn f54(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1597 = v1;
    match m1597 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1599 = f124(match v0.clone() {
                Adt2::C0(h1598, _, _, _, _, _, _, _, _, _, _) => {
                    h1598
                }
            });
            match m1599 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1600, _) => {
                            h1600
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1601 = f16(v0.clone(), 1u64)?;
                        let a1602 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1601, a1602)
                    } else {
                        false
                    } {
                        let a1607 = {
                            let a1604 = match v0.clone() {
                                Adt2::C0(_, h1603, _, _, _, _, _, _, _, _, _) => {
                                    h1603
                                }
                            };
                            let a1605 = v5;
                            let a1606 = 1u64;
                            nat_rem(a1604, a1605, a1606)
                        };
                        let a1608 = 0u64;
                        if nat_eq(a1607, a1608) {
                            let a1611 = match v4 {
                                Adt18::C0(_, _, h1609) => {
                                    h1609
                                }
                            };
                            let a1612 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1610, _, _, _, _, _) => {
                                    h1610
                                }
                            };
                            nat_le(a1611, a1612)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1613 = v5;
                        let a1614 = 1u64;
                        if nat_eq(a1613, a1614) {
                            let a1616 = 4u64;
                            let a1617 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1615, _, _, _, _, _) => {
                                    h1615
                                }
                            };
                            nat_le(a1616, a1617)
                        } else {
                            let a1619 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1618, _, _, _, _, _) => {
                                    h1618
                                }
                            };
                            let a1620 = 4u64;
                            nat_eq(a1619, a1620)
                        }
                    } else {
                        false
                    } {
                        let a1625 = {
                            let a1622 = match v2.clone() {
                                Adt2::C0(_, h1621, _, _, _, _, _, _, _, _, _) => {
                                    h1621
                                }
                            };
                            let a1623 = v5;
                            let a1624 = 1u64;
                            nat_rem(a1622, a1623, a1624)
                        };
                        let a1626 = 0u64;
                        if nat_eq(a1625, a1626) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1627, _, _, _, _, _, _, _, _, _, _) => {
                                h1627
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

pub fn f55(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1630 = match v1.clone() {
        Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
            h1628
        }
    };
    let a1631 = match v3.clone() {
        Adt2::C0(h1629, _, _, _, _, _, _, _, _, _, _) => {
            h1629
        }
    };
    if nat_eq(a1630, a1631) {
        f125(v0, v1, v2, v3)
    } else {
        if if if if f123(match v1.clone() {
            Adt2::C0(h1632, _, _, _, _, _, _, _, _, _, _) => {
                h1632
            }
        }) {
            f123(match v3.clone() {
                Adt2::C0(h1633, _, _, _, _, _, _, _, _, _, _) => {
                    h1633
                }
            })
        } else {
            false
        } {
            let a1634 = v0;
            let a1635 = 33u64;
            if nat_eq(a1634, a1635) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1636 = f16(v1.clone(), 1u64)?;
            let a1637 = f16(v3.clone(), 1u64)?;
            nat_eq(a1636, a1637)
        } else {
            false
        } {
            let m1639 = f124(match v3.clone() {
                Adt2::C0(h1638, _, _, _, _, _, _, _, _, _, _) => {
                    h1638
                }
            });
            match m1639 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1640, _, _, _, _, _, _, _, _, _, _) => {
                            h1640
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1641, _, _, _, _, _, _, _, _, _, _) => {
                            h1641
                        }
                    };
                    let v7: bool = {
                        let a1642 = v5;
                        let a1643 = 43u64;
                        if nat_eq(a1642, a1643) {
                            true
                        } else {
                            let a1644 = v6;
                            let a1645 = 43u64;
                            nat_eq(a1644, a1645)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1646 = f16(v3.clone(), 1u64)?;
                        let a1647 = if v7 {
                            f126(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1646, a1647)?
                    }, match v4 {
                        Adt18::C0(_, _, h1648) => {
                            h1648
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

pub fn f56(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1650 = match v0.clone() {
        Adt2::C0(h1649, _, _, _, _, _, _, _, _, _, _) => {
            h1649
        }
    };
    let a1651 = 0u64;
    if if if if nat_eq(a1650, a1651) {
        true
    } else {
        let a1653 = match v0.clone() {
            Adt2::C0(h1652, _, _, _, _, _, _, _, _, _, _) => {
                h1652
            }
        };
        let a1654 = 1u64;
        nat_eq(a1653, a1654)
    } {
        true
    } else {
        let a1656 = match v0.clone() {
            Adt2::C0(h1655, _, _, _, _, _, _, _, _, _, _) => {
                h1655
            }
        };
        let a1657 = 25u64;
        if nat_eq(a1656, a1657) {
            true
        } else {
            let a1659 = match v0.clone() {
                Adt2::C0(h1658, _, _, _, _, _, _, _, _, _, _) => {
                    h1658
                }
            };
            let a1660 = 26u64;
            nat_eq(a1659, a1660)
        }
    } {
        true
    } else {
        let a1662 = match v0.clone() {
            Adt2::C0(h1661, _, _, _, _, _, _, _, _, _, _) => {
                h1661
            }
        };
        let a1663 = 30u64;
        nat_eq(a1662, a1663)
    } {
        let a1666 = match v0.clone() {
            Adt2::C0(h1664, _, _, _, _, _, _, _, _, _, _) => {
                h1664
            }
        };
        let a1667 = match v1.clone() {
            Adt2::C0(h1665, _, _, _, _, _, _, _, _, _, _) => {
                h1665
            }
        };
        if if nat_eq(a1666, a1667) {
            f127(match v1.clone() {
                Adt2::C0(_, h1668, _, _, _, _, _, _, _, _, _) => {
                    h1668
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1669, _, _, _, _, _, _, _, _) => {
                    h1669
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1670, _, _, _, _, _, _, _) => {
                    h1670
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1671, _, _, _, _, _, _) => {
                    h1671
                }
            }, match v0.clone() {
                Adt2::C0(_, h1672, _, _, _, _, _, _, _, _, _) => {
                    h1672
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1673, _, _, _, _, _, _, _, _) => {
                    h1673
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1674, _, _, _, _, _, _, _) => {
                    h1674
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1675, _, _, _, _, _, _) => {
                    h1675
                }
            })
        } else {
            false
        } {
            let m1677 = f124(match v0.clone() {
                Adt2::C0(h1676, _, _, _, _, _, _, _, _, _, _) => {
                    h1676
                }
            });
            match m1677 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1678) => {
                            h1678
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

pub fn f57(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1679 = v2;
    match m1679 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1680 = v0;
            let a1681 = 4u64;
            if if if nat_lt(a1680, a1681) {
                let a1684 = match v1.clone() {
                    Adt2::C0(h1682, _, _, _, _, _, _, _, _, _, _) => {
                        h1682
                    }
                };
                let a1685 = match v4.clone() {
                    Adt2::C0(h1683, _, _, _, _, _, _, _, _, _, _) => {
                        h1683
                    }
                };
                if nat_eq(a1684, a1685) {
                    let a1688 = match v1.clone() {
                        Adt2::C0(h1686, _, _, _, _, _, _, _, _, _, _) => {
                            h1686
                        }
                    };
                    let a1689 = match v3.clone() {
                        Adt2::C0(h1687, _, _, _, _, _, _, _, _, _, _) => {
                            h1687
                        }
                    };
                    nat_eq(a1688, a1689)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f128(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1690, _, _, _, _, _, _, _, _, _) => {
                        h1690
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1691, _, _, _, _, _, _, _, _, _) => {
                        h1691
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1692, _, _, _, _, _, _, _, _, _) => {
                        h1692
                    }
                })? {
                    f128(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1693, _, _, _, _, _, _, _, _) => {
                            h1693
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1694, _, _, _, _, _, _, _, _) => {
                            h1694
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1695, _, _, _, _, _, _, _, _) => {
                            h1695
                        }
                    })?
                } else {
                    false
                } {
                    f128(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1696, _, _, _, _, _, _, _) => {
                            h1696
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1697, _, _, _, _, _, _, _) => {
                            h1697
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1698, _, _, _, _, _, _, _) => {
                            h1698
                        }
                    })?
                } else {
                    false
                } {
                    f128(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1699, _, _, _, _, _, _) => {
                            h1699
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1700, _, _, _, _, _, _) => {
                            h1700
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1701, _, _, _, _, _, _) => {
                            h1701
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1703 = f124(match v1.clone() {
                    Adt2::C0(h1702, _, _, _, _, _, _, _, _, _, _) => {
                        h1702
                    }
                });
                match m1703 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1704, _) => {
                                h1704
                            }
                        };
                        let a1709 = {
                            let a1706 = match v1.clone() {
                                Adt2::C0(_, h1705, _, _, _, _, _, _, _, _, _) => {
                                    h1705
                                }
                            };
                            let a1707 = v6;
                            let a1708 = 1u64;
                            nat_rem(a1706, a1707, a1708)
                        };
                        let a1710 = 0u64;
                        if if if nat_eq(a1709, a1710) {
                            let a1715 = {
                                let a1712 = match v4.clone() {
                                    Adt2::C0(_, h1711, _, _, _, _, _, _, _, _, _) => {
                                        h1711
                                    }
                                };
                                let a1713 = v6;
                                let a1714 = 1u64;
                                nat_rem(a1712, a1713, a1714)
                            };
                            let a1716 = 0u64;
                            nat_eq(a1715, a1716)
                        } else {
                            false
                        } {
                            let a1721 = {
                                let a1718 = match v3.clone() {
                                    Adt2::C0(_, h1717, _, _, _, _, _, _, _, _, _) => {
                                        h1717
                                    }
                                };
                                let a1719 = v6;
                                let a1720 = 1u64;
                                nat_rem(a1718, a1719, a1720)
                            };
                            let a1722 = 0u64;
                            nat_eq(a1721, a1722)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1723) => {
                                    h1723
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

pub fn f58(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1725 = match v2.clone() {
        Adt2::C0(h1724, _, _, _, _, _, _, _, _, _, _) => {
            h1724
        }
    };
    let a1726 = 0u64;
    if if nat_eq(a1725, a1726) {
        let a1728 = match v3.clone() {
            Adt2::C0(h1727, _, _, _, _, _, _, _, _, _, _) => {
                h1727
            }
        };
        let a1729 = 0u64;
        nat_eq(a1728, a1729)
    } else {
        false
    } {
        let a1730 = v0;
        let a1731 = 61u64;
        if nat_eq(a1730, a1731) {
            let v4: bool = {
                let a1733 = 0u64;
                let a1734 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1732, _, _, _) => {
                        h1732
                    }
                };
                nat_lt(a1733, a1734)
            };
            if if if if if if f129(match v2.clone() {
                Adt2::C0(_, h1735, _, _, _, _, _, _, _, _, _) => {
                    h1735
                }
            }, match v1.clone() {
                Adt1::C0(h1736, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1736
                }
            }, match v1.clone() {
                Adt1::C0(_, h1737, _, _, _, _, _, _, _, _, _, _) => {
                    h1737
                }
            }, match v3.clone() {
                Adt2::C0(_, h1738, _, _, _, _, _, _, _, _, _) => {
                    h1738
                }
            })? {
                f129(match v2.clone() {
                    Adt2::C0(_, _, h1739, _, _, _, _, _, _, _, _) => {
                        h1739
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1740, _, _, _, _, _, _, _, _, _) => {
                        h1740
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1741, _, _, _, _, _, _, _, _) => {
                        h1741
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1742, _, _, _, _, _, _, _, _) => {
                        h1742
                    }
                })?
            } else {
                false
            } {
                f129(match v2.clone() {
                    Adt2::C0(_, _, _, h1743, _, _, _, _, _, _, _) => {
                        h1743
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1744, _, _, _, _, _, _, _) => {
                        h1744
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1745, _, _, _, _, _, _) => {
                        h1745
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1746, _, _, _, _, _, _, _) => {
                        h1746
                    }
                })?
            } else {
                false
            } {
                f129(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1747, _, _, _, _, _, _) => {
                        h1747
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1748, _, _, _, _, _) => {
                        h1748
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1749, _, _, _, _) => {
                        h1749
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1750, _, _, _, _, _, _) => {
                        h1750
                    }
                })?
            } else {
                false
            } {
                let a1752 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1751, _, _, _) => {
                        h1751
                    }
                };
                let a1753 = 4294967295u64;
                nat_le(a1752, a1753)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1754, _, _, _, _, _, _, _, _, _) => {
                            h1754
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1755, _, _, _, _, _, _, _, _) => {
                            h1755
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1756, _, _, _, _, _, _, _) => {
                            h1756
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1757, _, _, _, _, _, _) => {
                            h1757
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1758, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1758
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1759, _, _, _, _, _, _, _, _, _) => {
                        h1759
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1760, _, _, _, _, _, _, _) => {
                        h1760
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1761, _, _, _, _, _) => {
                        h1761
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1762 = v0;
            let a1763 = 62u64;
            if nat_eq(a1762, a1763) {
                let a1765 = match v1.clone() {
                    Adt1::C0(h1764, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1764
                    }
                };
                let a1766 = 2147483648u64;
                if if if if if nat_lt(a1765, a1766) {
                    let a1768 = match v1.clone() {
                        Adt1::C0(_, h1767, _, _, _, _, _, _, _, _, _, _) => {
                            h1767
                        }
                    };
                    let a1769 = 2147483648u64;
                    nat_lt(a1768, a1769)
                } else {
                    false
                } {
                    let a1772 = match v1.clone() {
                        Adt1::C0(h1770, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1770
                        }
                    };
                    let a1773 = match v2.clone() {
                        Adt2::C0(_, h1771, _, _, _, _, _, _, _, _, _) => {
                            h1771
                        }
                    };
                    nat_lt(a1772, a1773)
                } else {
                    false
                } {
                    let a1776 = match v1.clone() {
                        Adt1::C0(_, h1774, _, _, _, _, _, _, _, _, _, _) => {
                            h1774
                        }
                    };
                    let a1777 = match v2.clone() {
                        Adt2::C0(_, h1775, _, _, _, _, _, _, _, _, _) => {
                            h1775
                        }
                    };
                    nat_lt(a1776, a1777)
                } else {
                    false
                } {
                    let a1780 = match v2.clone() {
                        Adt2::C0(_, _, h1778, _, _, _, _, _, _, _, _) => {
                            h1778
                        }
                    };
                    let a1781 = match v3.clone() {
                        Adt2::C0(_, _, h1779, _, _, _, _, _, _, _, _) => {
                            h1779
                        }
                    };
                    if if if nat_eq(a1780, a1781) {
                        let a1784 = match v2.clone() {
                            Adt2::C0(_, _, _, h1782, _, _, _, _, _, _, _) => {
                                h1782
                            }
                        };
                        let a1785 = match v3.clone() {
                            Adt2::C0(_, _, _, h1783, _, _, _, _, _, _, _) => {
                                h1783
                            }
                        };
                        nat_eq(a1784, a1785)
                    } else {
                        false
                    } {
                        let a1788 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1786, _, _, _, _, _, _) => {
                                h1786
                            }
                        };
                        let a1789 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1787, _, _, _, _, _, _) => {
                                h1787
                            }
                        };
                        nat_eq(a1788, a1789)
                    } else {
                        false
                    } {
                        let a1798 = match v3.clone() {
                            Adt2::C0(_, h1790, _, _, _, _, _, _, _, _, _) => {
                                h1790
                            }
                        };
                        let a1799 = {
                            let a1796 = {
                                let a1793 = match v2.clone() {
                                    Adt2::C0(_, h1791, _, _, _, _, _, _, _, _, _) => {
                                        h1791
                                    }
                                };
                                let a1794 = match v1.clone() {
                                    Adt1::C0(h1792, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1792
                                    }
                                };
                                nat_add(a1793, a1794)?
                            };
                            let a1797 = match v1.clone() {
                                Adt1::C0(_, h1795, _, _, _, _, _, _, _, _, _, _) => {
                                    h1795
                                }
                            };
                            nat_add(a1796, a1797)?
                        };
                        nat_eq(a1798, a1799)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1800, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1800
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1801 = v0;
                let a1802 = 63u64;
                if nat_eq(a1801, a1802) {
                    if if if if if f130(match v2.clone() {
                        Adt2::C0(_, h1803, _, _, _, _, _, _, _, _, _) => {
                            h1803
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1804, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1804
                        }
                    })? {
                        f130(match v2.clone() {
                            Adt2::C0(_, _, h1805, _, _, _, _, _, _, _, _) => {
                                h1805
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1806, _, _, _, _, _, _, _, _, _, _) => {
                                h1806
                            }
                        })?
                    } else {
                        false
                    } {
                        f130(match v2.clone() {
                            Adt2::C0(_, _, _, h1807, _, _, _, _, _, _, _) => {
                                h1807
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1808, _, _, _, _, _, _, _, _, _) => {
                                h1808
                            }
                        })?
                    } else {
                        false
                    } {
                        f130(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1809, _, _, _, _, _, _) => {
                                h1809
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1810, _, _, _, _, _, _, _, _) => {
                                h1810
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1813 = match v2.clone() {
                            Adt2::C0(_, h1811, _, _, _, _, _, _, _, _, _) => {
                                h1811
                            }
                        };
                        let a1814 = match v3.clone() {
                            Adt2::C0(_, h1812, _, _, _, _, _, _, _, _, _) => {
                                h1812
                            }
                        };
                        if if if nat_eq(a1813, a1814) {
                            let a1817 = match v2.clone() {
                                Adt2::C0(_, _, h1815, _, _, _, _, _, _, _, _) => {
                                    h1815
                                }
                            };
                            let a1818 = match v3.clone() {
                                Adt2::C0(_, _, h1816, _, _, _, _, _, _, _, _) => {
                                    h1816
                                }
                            };
                            nat_eq(a1817, a1818)
                        } else {
                            false
                        } {
                            let a1821 = match v2.clone() {
                                Adt2::C0(_, _, _, h1819, _, _, _, _, _, _, _) => {
                                    h1819
                                }
                            };
                            let a1822 = match v3.clone() {
                                Adt2::C0(_, _, _, h1820, _, _, _, _, _, _, _) => {
                                    h1820
                                }
                            };
                            nat_eq(a1821, a1822)
                        } else {
                            false
                        } {
                            let a1825 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1823, _, _, _, _, _, _) => {
                                    h1823
                                }
                            };
                            let a1826 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1824, _, _, _, _, _, _) => {
                                    h1824
                                }
                            };
                            nat_eq(a1825, a1826)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1827, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1827
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1828, _, _, _, _, _, _, _, _, _, _) => {
                                h1828
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1829, _, _, _, _, _, _, _, _, _) => {
                                h1829
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1830, _, _, _, _, _, _, _, _) => {
                                h1830
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

pub fn f59(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1831 = v3;
    match m1831 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1832 = v0;
                let a1833 = 4u64;
                nat_eq(a1832, a1833)
            };
            let v7: bool = {
                let a1835 = 0u64;
                let a1836 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1834, _, _, _, _, _, _, _) => {
                        h1834
                    }
                };
                nat_lt(a1835, a1836)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1837, _, _, _, _, _, _, _, _, _) => {
                    h1837
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1838, _, _, _, _, _, _, _, _) => {
                    h1838
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1839, _, _, _, _, _, _, _) => {
                    h1839
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1840, _, _, _, _, _, _) => {
                    h1840
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1841, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1841
                }
            }, match v1.clone() {
                Adt1::C0(_, h1842, _, _, _, _, _, _, _, _, _, _) => {
                    h1842
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1843, _, _, _, _, _, _, _, _, _) => {
                    h1843
                }
            });
            if if if if if if if if if if if if v6 {
                let a1845 = match v2.clone() {
                    Adt2::C0(h1844, _, _, _, _, _, _, _, _, _, _) => {
                        h1844
                    }
                };
                let a1846 = 0u64;
                if nat_eq(a1845, a1846) {
                    let a1848 = match v5.clone() {
                        Adt2::C0(h1847, _, _, _, _, _, _, _, _, _, _) => {
                            h1847
                        }
                    };
                    let a1849 = 0u64;
                    nat_eq(a1848, a1849)
                } else {
                    false
                }
            } else {
                let a1851 = match v2.clone() {
                    Adt2::C0(h1850, _, _, _, _, _, _, _, _, _, _) => {
                        h1850
                    }
                };
                let a1852 = 0u64;
                if if nat_eq(a1851, a1852) {
                    true
                } else {
                    let a1854 = match v2.clone() {
                        Adt2::C0(h1853, _, _, _, _, _, _, _, _, _, _) => {
                            h1853
                        }
                    };
                    let a1855 = 26u64;
                    nat_eq(a1854, a1855)
                } {
                    let a1857 = match v5.clone() {
                        Adt2::C0(h1856, _, _, _, _, _, _, _, _, _, _) => {
                            h1856
                        }
                    };
                    let a1858 = 0u64;
                    if nat_eq(a1857, a1858) {
                        true
                    } else {
                        let a1860 = match v5.clone() {
                            Adt2::C0(h1859, _, _, _, _, _, _, _, _, _, _) => {
                                h1859
                            }
                        };
                        let a1861 = 26u64;
                        nat_eq(a1860, a1861)
                    }
                } else {
                    false
                }
            } {
                let a1864 = match v2.clone() {
                    Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                        h1862
                    }
                };
                let a1865 = match v4.clone() {
                    Adt2::C0(h1863, _, _, _, _, _, _, _, _, _, _) => {
                        h1863
                    }
                };
                nat_eq(a1864, a1865)
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
                let a1867 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1866, _, _, _, _, _) => {
                        h1866
                    }
                };
                let a1868 = 4u64;
                nat_eq(a1867, a1868)
            } else {
                false
            } {
                let a1869 = v8;
                let a1870 = f16(v2.clone(), 1u64)?;
                nat_le(a1869, a1870)
            } else {
                false
            } {
                let a1872 = match v1.clone() {
                    Adt1::C0(_, _, _, h1871, _, _, _, _, _, _, _, _) => {
                        h1871
                    }
                };
                let a1873 = 1073741824u64;
                nat_lt(a1872, a1873)
            } else {
                false
            } {
                let a1875 = match v1.clone() {
                    Adt1::C0(h1874, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1874
                    }
                };
                let a1876 = 4294967295u64;
                if if if if nat_le(a1875, a1876) {
                    let a1878 = match v1.clone() {
                        Adt1::C0(_, h1877, _, _, _, _, _, _, _, _, _, _) => {
                            h1877
                        }
                    };
                    let a1879 = 4294967295u64;
                    nat_le(a1878, a1879)
                } else {
                    false
                } {
                    let a1881 = match v1.clone() {
                        Adt1::C0(_, _, h1880, _, _, _, _, _, _, _, _, _) => {
                            h1880
                        }
                    };
                    let a1882 = 4294967295u64;
                    nat_le(a1881, a1882)
                } else {
                    false
                } {
                    let a1884 = match v1.clone() {
                        Adt1::C0(_, _, _, h1883, _, _, _, _, _, _, _, _) => {
                            h1883
                        }
                    };
                    let a1885 = 4294967295u64;
                    nat_le(a1884, a1885)
                } else {
                    false
                } {
                    let a1887 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1886, _, _, _, _, _, _, _) => {
                            h1886
                        }
                    };
                    let a1888 = 4294967295u64;
                    nat_le(a1887, a1888)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1891 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1889) => {
                            h1889
                        }
                    };
                    let a1892 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1890) => {
                            h1890
                        }
                    };
                    nat_eq(a1891, a1892)
                } else {
                    true
                }
            } else {
                false
            } {
                f131(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1893, _, _, _, _, _, _, _, _) => {
                        h1893
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1894, _) => {
                        h1894
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
                    let a1895 = v10;
                    let a1896 = v8;
                    nat_add(a1895, a1896)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1897, _, _, _, _, _, _, _, _) => {
                        h1897
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f60(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1899 = match v0.clone() {
        Adt2::C0(h1898, _, _, _, _, _, _, _, _, _, _) => {
            h1898
        }
    };
    let a1900 = 0u64;
    if if if nat_eq(a1899, a1900) {
        let a1902 = match v1.clone() {
            Adt2::C0(h1901, _, _, _, _, _, _, _, _, _, _) => {
                h1901
            }
        };
        let a1903 = 0u64;
        nat_eq(a1902, a1903)
    } else {
        false
    } {
        f127(match v0.clone() {
            Adt2::C0(_, h1904, _, _, _, _, _, _, _, _, _) => {
                h1904
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1905, _, _, _, _, _, _, _, _) => {
                h1905
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1906, _, _, _, _, _, _, _) => {
                h1906
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1907, _, _, _, _, _, _) => {
                h1907
            }
        }, match v1.clone() {
            Adt2::C0(_, h1908, _, _, _, _, _, _, _, _, _) => {
                h1908
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1909, _, _, _, _, _, _, _, _) => {
                h1909
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1910, _, _, _, _, _, _, _) => {
                h1910
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1911, _, _, _, _, _, _) => {
                h1911
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1912 = v2;
            let a1913 = f16(v0.clone(), 1u64)?;
            nat_add(a1912, a1913)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f61(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1915 = match v1.clone() {
        Adt2::C0(h1914, _, _, _, _, _, _, _, _, _, _) => {
            h1914
        }
    };
    let a1916 = 0u64;
    if if if nat_eq(a1915, a1916) {
        true
    } else {
        let a1918 = match v1.clone() {
            Adt2::C0(h1917, _, _, _, _, _, _, _, _, _, _) => {
                h1917
            }
        };
        let a1919 = 1u64;
        nat_eq(a1918, a1919)
    } {
        true
    } else {
        let a1921 = match v1.clone() {
            Adt2::C0(h1920, _, _, _, _, _, _, _, _, _, _) => {
                h1920
            }
        };
        let a1922 = 30u64;
        nat_eq(a1921, a1922)
    } {
        let a1925 = match v1.clone() {
            Adt2::C0(h1923, _, _, _, _, _, _, _, _, _, _) => {
                h1923
            }
        };
        let a1926 = match v2.clone() {
            Adt2::C0(h1924, _, _, _, _, _, _, _, _, _, _) => {
                h1924
            }
        };
        if if if if if nat_eq(a1925, a1926) {
            let a1929 = match v1.clone() {
                Adt2::C0(_, h1927, _, _, _, _, _, _, _, _, _) => {
                    h1927
                }
            };
            let a1930 = match v2.clone() {
                Adt2::C0(_, h1928, _, _, _, _, _, _, _, _, _) => {
                    h1928
                }
            };
            nat_eq(a1929, a1930)
        } else {
            false
        } {
            let a1933 = match v1.clone() {
                Adt2::C0(_, _, _, h1931, _, _, _, _, _, _, _) => {
                    h1931
                }
            };
            let a1934 = match v2.clone() {
                Adt2::C0(_, _, _, h1932, _, _, _, _, _, _, _) => {
                    h1932
                }
            };
            nat_eq(a1933, a1934)
        } else {
            false
        } {
            let a1937 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1935, _, _, _, _, _, _) => {
                    h1935
                }
            };
            let a1938 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1936, _, _, _, _, _, _) => {
                    h1936
                }
            };
            nat_eq(a1937, a1938)
        } else {
            false
        } {
            let a1939 = v0;
            let a1940 = 4u64;
            if nat_eq(a1939, a1940) {
                let a1942 = match v1.clone() {
                    Adt2::C0(_, _, h1941, _, _, _, _, _, _, _, _) => {
                        h1941
                    }
                };
                let a1943 = 1u64;
                if nat_eq(a1942, a1943) {
                    let a1946 = match v1.clone() {
                        Adt2::C0(_, h1944, _, _, _, _, _, _, _, _, _) => {
                            h1944
                        }
                    };
                    let a1947 = match v2.clone() {
                        Adt2::C0(_, _, h1945, _, _, _, _, _, _, _, _) => {
                            h1945
                        }
                    };
                    nat_eq(a1946, a1947)
                } else {
                    false
                }
            } else {
                let a1948 = v0;
                let a1949 = 4u64;
                if if nat_lt(a1948, a1949) {
                    let a1952 = match v1.clone() {
                        Adt2::C0(_, h1950, _, _, _, _, _, _, _, _, _) => {
                            h1950
                        }
                    };
                    let a1953 = match v1.clone() {
                        Adt2::C0(_, _, h1951, _, _, _, _, _, _, _, _) => {
                            h1951
                        }
                    };
                    nat_eq(a1952, a1953)
                } else {
                    false
                } {
                    let a1956 = match v1.clone() {
                        Adt2::C0(_, _, h1954, _, _, _, _, _, _, _, _) => {
                            h1954
                        }
                    };
                    let a1957 = match v2.clone() {
                        Adt2::C0(_, _, h1955, _, _, _, _, _, _, _, _) => {
                            h1955
                        }
                    };
                    nat_eq(a1956, a1957)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1959 = f124(match v1.clone() {
                Adt2::C0(h1958, _, _, _, _, _, _, _, _, _, _) => {
                    h1958
                }
            });
            match m1959 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1960) => {
                            h1960
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

pub fn f62(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f132(match v1.clone() {
        Adt2::C0(h1961, _, _, _, _, _, _, _, _, _, _) => {
            h1961
        }
    }, v0) {
        let a1964 = match v1.clone() {
            Adt2::C0(h1962, _, _, _, _, _, _, _, _, _, _) => {
                h1962
            }
        };
        let a1965 = match v2.clone() {
            Adt2::C0(h1963, _, _, _, _, _, _, _, _, _, _) => {
                h1963
            }
        };
        nat_eq(a1964, a1965)
    } else {
        false
    } {
        let a1968 = match v1.clone() {
            Adt2::C0(_, h1966, _, _, _, _, _, _, _, _, _) => {
                h1966
            }
        };
        let a1969 = match v2.clone() {
            Adt2::C0(_, h1967, _, _, _, _, _, _, _, _, _) => {
                h1967
            }
        };
        if if if nat_eq(a1968, a1969) {
            let a1972 = match v1.clone() {
                Adt2::C0(_, _, h1970, _, _, _, _, _, _, _, _) => {
                    h1970
                }
            };
            let a1973 = match v2.clone() {
                Adt2::C0(_, _, h1971, _, _, _, _, _, _, _, _) => {
                    h1971
                }
            };
            nat_eq(a1972, a1973)
        } else {
            false
        } {
            let a1976 = match v1.clone() {
                Adt2::C0(_, _, _, h1974, _, _, _, _, _, _, _) => {
                    h1974
                }
            };
            let a1977 = match v2.clone() {
                Adt2::C0(_, _, _, h1975, _, _, _, _, _, _, _) => {
                    h1975
                }
            };
            nat_eq(a1976, a1977)
        } else {
            false
        } {
            let a1980 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1978, _, _, _, _, _, _) => {
                    h1978
                }
            };
            let a1981 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1979, _, _, _, _, _, _) => {
                    h1979
                }
            };
            nat_eq(a1980, a1981)
        } else {
            false
        }
    } else {
        false
    } {
        let m1983 = f124(match v1.clone() {
            Adt2::C0(h1982, _, _, _, _, _, _, _, _, _, _) => {
                h1982
            }
        });
        match m1983 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h1984) => {
                        h1984
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1985, _, _, _, _, _, _, _, _, _, _) => {
                        h1985
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f63(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1986, _, _, _, _, _, _, _, _, _, _, _) => {
                h1986
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1987, _, _, _, _, _, _, _, _, _) => {
                h1987
            }
        }
    };
    let a1988 = 0u64;
    let a1989 = v4;
    if if nat_lt(a1988, a1989) {
        let a1990 = v4;
        let a1991 = 2147483648u64;
        nat_lt(a1990, a1991)
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
        let v7: u64 = f133(match v5.clone() {
            Adt2::C0(_, _, h1992, _, _, _, _, _, _, _, _) => {
                h1992
            }
        }, v4)?;
        let v8: u64 = f133(match v5.clone() {
            Adt2::C0(_, _, _, h1993, _, _, _, _, _, _, _) => {
                h1993
            }
        }, v4)?;
        let a1995 = match v5.clone() {
            Adt2::C0(h1994, _, _, _, _, _, _, _, _, _, _) => {
                h1994
            }
        };
        let a1996 = 0u64;
        if if if if if nat_eq(a1995, a1996) {
            let a1998 = match v6.clone() {
                Adt2::C0(h1997, _, _, _, _, _, _, _, _, _, _) => {
                    h1997
                }
            };
            let a1999 = 0u64;
            nat_eq(a1998, a1999)
        } else {
            false
        } {
            let a2001 = match v5.clone() {
                Adt2::C0(_, _, _, _, h2000, _, _, _, _, _, _) => {
                    h2000
                }
            };
            let a2002 = 1u64;
            nat_eq(a2001, a2002)
        } else {
            false
        } {
            let a2005 = match v5.clone() {
                Adt2::C0(_, h2003, _, _, _, _, _, _, _, _, _) => {
                    h2003
                }
            };
            let a2006 = match v6.clone() {
                Adt2::C0(_, h2004, _, _, _, _, _, _, _, _, _) => {
                    h2004
                }
            };
            nat_eq(a2005, a2006)
        } else {
            false
        } {
            if v0 {
                let a2008 = match v5.clone() {
                    Adt2::C0(_, _, h2007, _, _, _, _, _, _, _, _) => {
                        h2007
                    }
                };
                let a2009 = 2147483648u64;
                if if if nat_lt(a2008, a2009) {
                    let a2011 = match v5.clone() {
                        Adt2::C0(_, _, _, h2010, _, _, _, _, _, _, _) => {
                            h2010
                        }
                    };
                    let a2012 = 2147483648u64;
                    nat_lt(a2011, a2012)
                } else {
                    false
                } {
                    let a2015 = {
                        let a2013 = v7;
                        let a2014 = v8;
                        nat_mul(a2013, a2014)?
                    };
                    let a2016 = 2147483648u64;
                    nat_le(a2015, a2016)
                } else {
                    false
                } {
                    f134(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a2018 = match v1.clone() {
                    Adt1::C0(h2017, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2017
                    }
                };
                let a2019 = v7;
                if if if if nat_eq(a2018, a2019) {
                    let a2021 = match v1 {
                        Adt1::C0(_, h2020, _, _, _, _, _, _, _, _, _, _) => {
                            h2020
                        }
                    };
                    let a2022 = v8;
                    nat_eq(a2021, a2022)
                } else {
                    false
                } {
                    f135(v7, v8, v4)?
                } else {
                    false
                } {
                    let a2024 = match v6.clone() {
                        Adt2::C0(_, _, h2023, _, _, _, _, _, _, _, _) => {
                            h2023
                        }
                    };
                    let a2025 = v4;
                    if nat_eq(a2024, a2025) {
                        let a2027 = match v6.clone() {
                            Adt2::C0(_, _, _, h2026, _, _, _, _, _, _, _) => {
                                h2026
                            }
                        };
                        let a2028 = v4;
                        nat_eq(a2027, a2028)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a2032 = match v6 {
                        Adt2::C0(_, _, _, _, h2029, _, _, _, _, _, _) => {
                            h2029
                        }
                    };
                    let a2033 = {
                        let a2030 = v7;
                        let a2031 = v8;
                        nat_mul(a2030, a2031)?
                    };
                    nat_eq(a2032, a2033)
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

pub fn f64(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a2035 = match v2.clone() {
        Adt2::C0(h2034, _, _, _, _, _, _, _, _, _, _) => {
            h2034
        }
    };
    let a2036 = 0u64;
    if if if if if nat_eq(a2035, a2036) {
        true
    } else {
        let a2038 = match v2.clone() {
            Adt2::C0(h2037, _, _, _, _, _, _, _, _, _, _) => {
                h2037
            }
        };
        let a2039 = 1u64;
        nat_eq(a2038, a2039)
    } {
        true
    } else {
        let a2041 = match v2.clone() {
            Adt2::C0(h2040, _, _, _, _, _, _, _, _, _, _) => {
                h2040
            }
        };
        let a2042 = 30u64;
        nat_eq(a2041, a2042)
    } {
        let a2045 = match v2.clone() {
            Adt2::C0(h2043, _, _, _, _, _, _, _, _, _, _) => {
                h2043
            }
        };
        let a2046 = match v3.clone() {
            Adt2::C0(h2044, _, _, _, _, _, _, _, _, _, _) => {
                h2044
            }
        };
        nat_eq(a2045, a2046)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a2048 = match v1.clone() {
                Adt1::C0(h2047, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2047
                }
            };
            let a2049 = 4294967296u64;
            nat_lt(a2048, a2049)
        } else {
            false
        } {
            let a2051 = match v1.clone() {
                Adt1::C0(_, h2050, _, _, _, _, _, _, _, _, _, _) => {
                    h2050
                }
            };
            let a2052 = 4294967296u64;
            nat_lt(a2051, a2052)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a2054 = match v2.clone() {
                Adt2::C0(h2053, _, _, _, _, _, _, _, _, _, _) => {
                    h2053
                }
            };
            let a2055 = 0u64;
            if nat_eq(a2054, a2055) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a2056 = v0;
            let a2057 = 31u64;
            if nat_eq(a2056, a2057) {
                0u64
            } else {
                let a2058 = v0;
                let a2059 = 47u64;
                if nat_eq(a2058, a2059) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h2060, _, _, _, _, _, _, _, _, _, _) => {
                h2060
            }
        }, match v1.clone() {
            Adt1::C0(h2061, _, _, _, _, _, _, _, _, _, _, _) => {
                h2061
            }
        }, match v1 {
            Adt1::C0(_, h2062, _, _, _, _, _, _, _, _, _, _) => {
                h2062
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f65(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2064 = match v0.clone() {
        Adt2::C0(h2063, _, _, _, _, _, _, _, _, _, _) => {
            h2063
        }
    };
    let a2065 = 0u64;
    if if if nat_eq(a2064, a2065) {
        let a2067 = match v1.clone() {
            Adt2::C0(h2066, _, _, _, _, _, _, _, _, _, _) => {
                h2066
            }
        };
        let a2068 = 0u64;
        nat_eq(a2067, a2068)
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

pub fn f66(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2070 = match v0.clone() {
        Adt2::C0(h2069, _, _, _, _, _, _, _, _, _, _) => {
            h2069
        }
    };
    let a2071 = 0u64;
    if if nat_eq(a2070, a2071) {
        let a2073 = match v1.clone() {
            Adt2::C0(h2072, _, _, _, _, _, _, _, _, _, _) => {
                h2072
            }
        };
        let a2074 = 26u64;
        nat_eq(a2073, a2074)
    } else {
        false
    } {
        let a2076 = match v0.clone() {
            Adt2::C0(_, _, _, h2075, _, _, _, _, _, _, _) => {
                h2075
            }
        };
        let a2077 = 1u64;
        if if if if if if if nat_eq(a2076, a2077) {
            let a2079 = match v0.clone() {
                Adt2::C0(_, _, _, _, h2078, _, _, _, _, _, _) => {
                    h2078
                }
            };
            let a2080 = 1u64;
            nat_eq(a2079, a2080)
        } else {
            false
        } {
            let a2082 = match v0.clone() {
                Adt2::C0(_, h2081, _, _, _, _, _, _, _, _, _) => {
                    h2081
                }
            };
            let a2083 = 2147483647u64;
            nat_le(a2082, a2083)
        } else {
            false
        } {
            let a2086 = match v1.clone() {
                Adt2::C0(_, h2084, _, _, _, _, _, _, _, _, _) => {
                    h2084
                }
            };
            let a2087 = match v0.clone() {
                Adt2::C0(_, _, h2085, _, _, _, _, _, _, _, _) => {
                    h2085
                }
            };
            nat_eq(a2086, a2087)
        } else {
            false
        } {
            let a2089 = match v1.clone() {
                Adt2::C0(_, _, h2088, _, _, _, _, _, _, _, _) => {
                    h2088
                }
            };
            let a2090 = 1u64;
            nat_eq(a2089, a2090)
        } else {
            false
        } {
            let a2092 = match v1.clone() {
                Adt2::C0(_, _, _, h2091, _, _, _, _, _, _, _) => {
                    h2091
                }
            };
            let a2093 = 1u64;
            nat_eq(a2092, a2093)
        } else {
            false
        } {
            let a2095 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2094, _, _, _, _, _, _) => {
                    h2094
                }
            };
            let a2096 = 1u64;
            nat_eq(a2095, a2096)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a2099 = match v0.clone() {
                    Adt2::C0(_, _, h2097, _, _, _, _, _, _, _, _) => {
                        h2097
                    }
                };
                let a2100 = f136(match v0.clone() {
                    Adt2::C0(_, h2098, _, _, _, _, _, _, _, _, _) => {
                        h2098
                    }
                })?;
                nat_mul(a2099, a2100)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f67(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2101 = v1;
    match m2101 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f137(f20(v0.clone(), 1u64))?;
            let a2103 = match v0.clone() {
                Adt2::C0(h2102, _, _, _, _, _, _, _, _, _, _) => {
                    h2102
                }
            };
            let a2104 = 26u64;
            if if if nat_eq(a2103, a2104) {
                let a2106 = match v3.clone() {
                    Adt2::C0(h2105, _, _, _, _, _, _, _, _, _, _) => {
                        h2105
                    }
                };
                let a2107 = 26u64;
                nat_eq(a2106, a2107)
            } else {
                false
            } {
                let a2109 = match v2.clone() {
                    Adt2::C0(h2108, _, _, _, _, _, _, _, _, _, _) => {
                        h2108
                    }
                };
                let a2110 = 27u64;
                nat_eq(a2109, a2110)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a2112 = match v2.clone() {
                        Adt2::C0(_, h2111, _, _, _, _, _, _, _, _, _) => {
                            h2111
                        }
                    };
                    let a2113 = 1u64;
                    nat_eq(a2112, a2113)
                } else {
                    false
                } {
                    let a2115 = match v2.clone() {
                        Adt2::C0(_, _, h2114, _, _, _, _, _, _, _, _) => {
                            h2114
                        }
                    };
                    let a2116 = 1u64;
                    nat_eq(a2115, a2116)
                } else {
                    false
                } {
                    let a2118 = match v2.clone() {
                        Adt2::C0(_, _, _, h2117, _, _, _, _, _, _, _) => {
                            h2117
                        }
                    };
                    let a2119 = 1u64;
                    nat_eq(a2118, a2119)
                } else {
                    false
                } {
                    let a2121 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2120, _, _, _, _, _, _) => {
                            h2120
                        }
                    };
                    let a2122 = 1u64;
                    nat_eq(a2121, a2122)
                } else {
                    false
                } {
                    let a2123 = v4;
                    let a2124 = 9223372036854775808u64;
                    nat_lt(a2123, a2124)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2127 = 2u64;
                        let a2128 = {
                            let a2125 = v4;
                            let a2126 = 2u64;
                            nat_mul(a2125, a2126)?
                        };
                        nat_add(a2127, a2128)?
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

pub fn f68(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2129, _, _, _, _, _, _, _, _, _, _) => {
            h2129
        }
    };
    let a2130 = v3;
    let a2131 = 0u64;
    if if nat_eq(a2130, a2131) {
        true
    } else {
        let a2132 = v0;
        let a2133 = 13u64;
        if nat_eq(a2132, a2133) {
            let a2134 = v3;
            let a2135 = 1u64;
            if nat_eq(a2134, a2135) {
                true
            } else {
                let a2136 = v3;
                let a2137 = 30u64;
                nat_eq(a2136, a2137)
            }
        } else {
            false
        }
    } {
        let a2139 = v3;
        let a2140 = match v2.clone() {
            Adt2::C0(h2138, _, _, _, _, _, _, _, _, _, _) => {
                h2138
            }
        };
        if if nat_eq(a2139, a2140) {
            let a2141 = v0;
            let a2142 = 13u64;
            if nat_eq(a2141, a2142) {
                let a2144 = match v2.clone() {
                    Adt2::C0(_, h2143, _, _, _, _, _, _, _, _, _) => {
                        h2143
                    }
                };
                let a2145 = 1u64;
                if if if nat_eq(a2144, a2145) {
                    let a2147 = match v2.clone() {
                        Adt2::C0(_, _, h2146, _, _, _, _, _, _, _, _) => {
                            h2146
                        }
                    };
                    let a2148 = 1u64;
                    nat_eq(a2147, a2148)
                } else {
                    false
                } {
                    let a2150 = match v2.clone() {
                        Adt2::C0(_, _, _, h2149, _, _, _, _, _, _, _) => {
                            h2149
                        }
                    };
                    let a2151 = 1u64;
                    nat_eq(a2150, a2151)
                } else {
                    false
                } {
                    let a2153 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2152, _, _, _, _, _, _) => {
                            h2152
                        }
                    };
                    let a2154 = 1u64;
                    nat_eq(a2153, a2154)
                } else {
                    false
                }
            } else {
                let a2156 = match v2.clone() {
                    Adt2::C0(_, h2155, _, _, _, _, _, _, _, _, _) => {
                        h2155
                    }
                };
                let a2157 = 1u64;
                if if if nat_eq(a2156, a2157) {
                    let a2160 = match v1.clone() {
                        Adt2::C0(_, _, h2158, _, _, _, _, _, _, _, _) => {
                            h2158
                        }
                    };
                    let a2161 = match v2.clone() {
                        Adt2::C0(_, _, h2159, _, _, _, _, _, _, _, _) => {
                            h2159
                        }
                    };
                    nat_eq(a2160, a2161)
                } else {
                    false
                } {
                    let a2164 = match v1.clone() {
                        Adt2::C0(_, _, _, h2162, _, _, _, _, _, _, _) => {
                            h2162
                        }
                    };
                    let a2165 = match v2.clone() {
                        Adt2::C0(_, _, _, h2163, _, _, _, _, _, _, _) => {
                            h2163
                        }
                    };
                    nat_eq(a2164, a2165)
                } else {
                    false
                } {
                    let a2168 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2166, _, _, _, _, _, _) => {
                            h2166
                        }
                    };
                    let a2169 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2167, _, _, _, _, _, _) => {
                            h2167
                        }
                    };
                    nat_eq(a2168, a2169)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f138(v4.clone(), v0, v3)?, {
                let a2170 = v3;
                let a2171 = 0u64;
                if nat_eq(a2170, a2171) {
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

pub fn f69(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2172 = v0;
    let a2173 = 89u64;
    if if nat_eq(a2172, a2173) {
        let a2174 = v1;
        let a2175 = 14u64;
        nat_eq(a2174, a2175)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2176 = v0;
        let a2177 = 89u64;
        if if nat_eq(a2176, a2177) {
            let a2178 = v1;
            let a2179 = 15u64;
            nat_eq(a2178, a2179)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2180 = v0;
            let a2181 = 89u64;
            if if nat_eq(a2180, a2181) {
                let a2182 = 8u64;
                let a2183 = v1;
                if nat_le(a2182, a2183) {
                    let a2184 = v1;
                    let a2185 = 9u64;
                    nat_le(a2184, a2185)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2186 = v1;
                    let a2187 = 12u64;
                    nat_add(a2186, a2187)?
                }))
            } else {
                let a2188 = v0;
                let a2189 = 89u64;
                if if nat_eq(a2188, a2189) {
                    let a2190 = v1;
                    let a2191 = 4u64;
                    nat_eq(a2190, a2191)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2192 = 10u64;
                    let a2193 = v0;
                    if if nat_le(a2192, a2193) {
                        let a2194 = v0;
                        let a2195 = 12u64;
                        nat_le(a2194, a2195)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2196 = v0;
                            let a2197 = 3u64;
                            nat_add(a2196, a2197)?
                        }))
                    } else {
                        let a2198 = v0;
                        let a2199 = 89u64;
                        if if nat_eq(a2198, a2199) {
                            let a2200 = v1;
                            let a2201 = 7u64;
                            nat_eq(a2200, a2201)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2202 = v0;
                            let a2203 = 89u64;
                            if if nat_eq(a2202, a2203) {
                                let a2204 = v1;
                                let a2205 = 10u64;
                                nat_eq(a2204, a2205)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2206 = v0;
                                let a2207 = 89u64;
                                if if nat_eq(a2206, a2207) {
                                    let a2208 = v1;
                                    let a2209 = 13u64;
                                    nat_eq(a2208, a2209)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2210 = v0;
                                    let a2211 = 8u64;
                                    if nat_eq(a2210, a2211) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2212 = v0;
                                        let a2213 = 9u64;
                                        if nat_eq(a2212, a2213) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2214 = v0;
                                            let a2215 = 89u64;
                                            if nat_eq(a2214, a2215) {
                                                let a2216 = v1;
                                                let a2217 = 0u64;
                                                if nat_eq(a2216, a2217) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2218 = v1;
                                                    let a2219 = 1u64;
                                                    if nat_eq(a2218, a2219) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2220 = v1;
                                                        let a2221 = 2u64;
                                                        if nat_eq(a2220, a2221) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2222 = v1;
                                                            let a2223 = 3u64;
                                                            if nat_eq(a2222, a2223) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2224 = v1;
                                                                let a2225 = 6u64;
                                                                if nat_eq(a2224, a2225) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2226 = v1;
                                                                    let a2227 = 12u64;
                                                                    if nat_eq(a2226, a2227) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2228 = v1;
                                                                        let a2229 = 11u64;
                                                                        if nat_eq(a2228, a2229) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2230 = v1;
                                                                            let a2231 = 18u64;
                                                                            if nat_eq(a2230, a2231) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2232 = v1;
                                                                                let a2233 = 19u64;
                                                                                if nat_eq(a2232, a2233) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2234 = v1;
                                                                                    let a2235 = 20u64;
                                                                                    if nat_eq(a2234, a2235) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2236 = v1;
                                                                                        let a2237 = 21u64;
                                                                                        if nat_eq(a2236, a2237) {
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

pub fn f70(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2238 = f139(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2238 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2239 = v0;
            let a2240 = 2u64;
            if if if if nat_eq(a2239, a2240) {
                true
            } else {
                let a2241 = v0;
                let a2242 = 5u64;
                nat_eq(a2241, a2242)
            } {
                true
            } else {
                let a2243 = v0;
                let a2244 = 6u64;
                nat_eq(a2243, a2244)
            } {
                true
            } else {
                let a2245 = v0;
                let a2246 = 7u64;
                nat_eq(a2245, a2246)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2247 = v3;
                    match m2247 {
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

pub fn f71(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2249 = match v1.clone() {
        Adt2::C0(h2248, _, _, _, _, _, _, _, _, _, _) => {
            h2248
        }
    };
    let a2250 = 0u64;
    if if if if if nat_eq(a2249, a2250) {
        true
    } else {
        let a2252 = match v1.clone() {
            Adt2::C0(h2251, _, _, _, _, _, _, _, _, _, _) => {
                h2251
            }
        };
        let a2253 = 1u64;
        nat_eq(a2252, a2253)
    } {
        true
    } else {
        let a2255 = match v1.clone() {
            Adt2::C0(h2254, _, _, _, _, _, _, _, _, _, _) => {
                h2254
            }
        };
        let a2256 = 30u64;
        nat_eq(a2255, a2256)
    } {
        let a2259 = match v1.clone() {
            Adt2::C0(h2257, _, _, _, _, _, _, _, _, _, _) => {
                h2257
            }
        };
        let a2260 = match v2.clone() {
            Adt2::C0(h2258, _, _, _, _, _, _, _, _, _, _) => {
                h2258
            }
        };
        if nat_eq(a2259, a2260) {
            true
        } else {
            let a2262 = match v2.clone() {
                Adt2::C0(h2261, _, _, _, _, _, _, _, _, _, _) => {
                    h2261
                }
            };
            let a2263 = 0u64;
            nat_eq(a2262, a2263)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2265 = f16(v2.clone(), 1u64)?;
            let a2266 = f140(v0, match v2.clone() {
                Adt2::C0(h2264, _, _, _, _, _, _, _, _, _, _) => {
                    h2264
                }
            });
            nat_mul(a2265, a2266)?
        }, {
            let a2268 = match v2.clone() {
                Adt2::C0(h2267, _, _, _, _, _, _, _, _, _, _) => {
                    h2267
                }
            };
            let a2269 = 0u64;
            if nat_eq(a2268, a2269) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2270, _, _, _, _, _, _, _, _, _, _) => {
                h2270
            }
        }, match v2 {
            Adt2::C0(h2271, _, _, _, _, _, _, _, _, _, _) => {
                h2271
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f72(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f141(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2272, _, _, _) => {
            h2272
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2273, _, _) => {
            h2273
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2274, _) => {
            h2274
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2275) => {
            h2275
        }
    })
}

pub fn f73(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2277 = v0;
        let a2278 = match v1.clone() {
            Adt3::C0(h2276, _, _, _, _, _, _, _) => {
                h2276
            }
        };
        let a2279 = 0u64;
        nat_quot(a2277, a2278, a2279)
    };
    let v4: u64 = {
        let a2281 = v3;
        let a2282 = match v1.clone() {
            Adt3::C0(_, h2280, _, _, _, _, _, _) => {
                h2280
            }
        };
        let a2283 = 0u64;
        nat_quot(a2281, a2282, a2283)
    };
    let a2284 = v2;
    let a2285 = 0u64;
    if nat_eq(a2284, a2285) {
        let a2287 = v0;
        let a2288 = match v1 {
            Adt3::C0(h2286, _, _, _, _, _, _, _) => {
                h2286
            }
        };
        let a2289 = 0u64;
        nat_rem(a2287, a2288, a2289)
    } else {
        let a2290 = v2;
        let a2291 = 1u64;
        if nat_eq(a2290, a2291) {
            let a2293 = v3;
            let a2294 = match v1 {
                Adt3::C0(_, h2292, _, _, _, _, _, _) => {
                    h2292
                }
            };
            let a2295 = 0u64;
            nat_rem(a2293, a2294, a2295)
        } else {
            let a2296 = v2;
            let a2297 = 2u64;
            if nat_eq(a2296, a2297) {
                let a2299 = v4;
                let a2300 = match v1 {
                    Adt3::C0(_, _, h2298, _, _, _, _, _) => {
                        h2298
                    }
                };
                let a2301 = 0u64;
                nat_rem(a2299, a2300, a2301)
            } else {
                let a2303 = v4;
                let a2304 = match v1 {
                    Adt3::C0(_, _, h2302, _, _, _, _, _) => {
                        h2302
                    }
                };
                let a2305 = 0u64;
                nat_quot(a2303, a2304, a2305)
            }
        }
    }
}

pub fn f74(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2307 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2306) => {
            h2306
        }
    };
    let a2308 = 0u64;
    if nat_eq(a2307, a2308) {
        Ok(v2)
    } else {
        let a2313 = {
            let a2310 = v2;
            let a2311 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2309) => {
                    h2309
                }
            };
            nat_mul(a2310, a2311)?
        };
        let a2314 = f73(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2312, _, _, _, _, _, _) => {
                h2312
            }
        }, 3u64);
        nat_add(a2313, a2314)
    }
}

pub fn f75(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2324 = {
            let a2321 = {
                let a2316 = v2;
                let a2317 = match v0.clone() {
                    Adt6::C0(_, h2315, _, _) => {
                        h2315
                    }
                };
                nat_mul(a2316, a2317)?
            };
            let a2322 = {
                let a2319 = v3;
                let a2320 = match v0.clone() {
                    Adt6::C0(_, _, _, h2318) => {
                        h2318
                    }
                };
                nat_mul(a2319, a2320)?
            };
            nat_add(a2321, a2322)?
        };
        let a2325 = f142(match v0.clone() {
            Adt6::C0(_, _, h2323, _) => {
                h2323
            }
        });
        nat_add(a2324, a2325)?
    };
    let v5: u64 = f143(match v0 {
        Adt6::C0(_, _, h2326, _) => {
            h2326
        }
    });
    let a2327 = v5;
    let a2328 = v4;
    if if nat_le(a2327, a2328) {
        let a2331 = {
            let a2329 = v4;
            let a2330 = v5;
            nat_sub(a2329, a2330)
        };
        let a2332 = v1;
        nat_lt(a2331, a2332)
    } else {
        false
    } {
        Ok(Some({
            let a2333 = v4;
            let a2334 = v5;
            nat_sub(a2333, a2334)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f76(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2335, _, _, _, _, _) => {
            h2335
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2336, _, _, _, _, _, _) => {
            h2336
        }
    };
    let v7: u64 = f73(v1, v6.clone(), 2u64);
    let a2338 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2337) => {
            h2337
        }
    };
    let a2339 = 0u64;
    if nat_eq(a2338, a2339) {
        Ok(Adt9::C8(Some(f72(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2342 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2341, _, _, _, _) => {
                    h2341
                }
            };
            match m2342 {
                Adt3::C0(_, _, h2340, _, _, _, _, _) => {
                    h2340
                }
            }
        };
        let m2348 = f75(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2343, _) => {
                h2343
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2344, _, _, _, _, _) => {
                h2344
            }
        }, {
            let a2345 = v2;
            let a2346 = v8;
            let a2347 = 0u64;
            nat_rem(a2345, a2346, a2347)
        }, v7)?;
        match m2348 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2355 = {
                        let a2353 = {
                            let a2349 = v2;
                            let a2350 = v8;
                            let a2351 = 0u64;
                            nat_quot(a2349, a2350, a2351)
                        };
                        let a2354 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2352) => {
                                h2352
                            }
                        };
                        nat_mul(a2353, a2354)?
                    };
                    let a2356 = f73(v1, v6, 3u64);
                    nat_add(a2355, a2356)?
                };
                Ok(Adt9::C8(Some(f72(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f77(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2357 = v1;
    match m2357 {
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
            f144(f145(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f146(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f147(v0.clone(), v24);
            f144(f148(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2358 = v3;
    match m2358 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f149(v0, v1)?, f149(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f151(v0, v1, v2.clone())?, f149(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f152(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f153(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f154(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f153(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f155(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f156(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f157(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f149(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f158(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v36, v37) => {
            f159(v0, v1, v2, v36, v37)
        }
        Adt4::C12(v38, v39, v40) => {
            f160(v0, v1, v2, v38, v39, v40)
        }
        Adt4::C13(_, v42, _, _) => {
            Ok(Adt9::C3(v42, v42, f149(v0, v1)?, f149(v0, v2)?))
        }
        Adt4::C14(v45, v46, v47) => {
            let a2361 = v45;
            let a2362 = 23u64;
            if nat_eq(a2361, a2362) {
                f161(v0, v1, v2, v46, v47)
            } else {
                Ok(Adt9::C3(v46, v47, f149(v0, v1)?, f149(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f162(v0, v1, v2)
        }
        Adt4::C16(v48, v49) => {
            f163(v0, v1, v2, v48, v49)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v50, v51) => {
            f164(v0, v1, v51, v50)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f149(f150(v0, match v2.clone() {
                Adt3::C0(h2359, _, _, _, _, _, _, _) => {
                    h2359
                }
            }, match v2.clone() {
                Adt3::C0(_, h2360, _, _, _, _, _, _) => {
                    h2360
                }
            })?, v1)?, f149(v0, v2)?))
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
        Adt4::C25(v56, v57, v58) => {
            f165(v0, v56, v57, v58)
        }
        Adt4::C26(v64, _, _, _) => {
            let a2363 = v64;
            let a2364 = 64u64;
            if if if nat_eq(a2363, a2364) {
                true
            } else {
                let a2365 = v64;
                let a2366 = 65u64;
                nat_eq(a2365, a2366)
            } {
                true
            } else {
                let a2367 = v64;
                let a2368 = 66u64;
                nat_eq(a2367, a2368)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v63) => {
            f90(v0, v1, v2, v63)
        }
        Adt4::C28(v59) => {
            f87(v0, v1, v2, v59, 0u64)
        }
        Adt4::C29(v60) => {
            f88(v0, v1, v2, v60)
        }
        Adt4::C30(v61, _) => {
            f89(v0, v1, v2, v61)
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
        Adt4::C34(_, _, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f126(v3, v4);
    let v6: u64 = {
        let a2369 = v0;
        let a2370 = v5;
        let a2371 = 0u64;
        nat_quot(a2369, a2370, a2371)
    };
    let a2372 = v5;
    let a2373 = 1u64;
    if nat_eq(a2372, a2373) {
        Ok(f144(f166(v3, v4), v3, v4, 0u64, f149(v6, v1)?, 2u64, 0u64, 3u64, f149(v6, v2)?))
    } else {
        let a2377 = {
            let a2374 = v0;
            let a2375 = 2u64;
            let a2376 = 0u64;
            nat_rem(a2374, a2375, a2376)
        };
        let a2378 = 0u64;
        if nat_eq(a2377, a2378) {
            Ok(f144(f166(v3, 0u64), v3, 0u64, 0u64, f149(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f144(f166(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f149(v6, v2)?))
        }
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f113(v1)?;
    let v4: u64 = {
        let a2379 = v0;
        let a2380 = v3;
        let a2381 = 0u64;
        nat_rem(a2379, a2380, a2381)
    };
    let v5: u64 = {
        let a2384 = {
            let a2382 = v1;
            let a2383 = 3u64;
            nat_mul(a2382, a2383)?
        };
        let a2385 = 1u64;
        nat_add(a2384, a2385)?
    };
    let a2386 = v4;
    let a2387 = 0u64;
    if nat_eq(a2386, a2387) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a2388 = v4;
        let a2389 = v5;
        if nat_eq(a2388, a2389) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a2394 = v4;
                let a2395 = {
                    let a2390 = v4;
                    let a2391 = v5;
                    if nat_lt(a2390, a2391) {
                        1u64
                    } else {
                        let a2392 = v5;
                        let a2393 = 1u64;
                        nat_add(a2392, a2393)?
                    }
                };
                nat_sub(a2394, a2395)
            };
            let v7: u64 = {
                let a2396 = v6;
                let a2397 = 3u64;
                let a2398 = 0u64;
                nat_rem(a2396, a2397, a2398)
            };
            let v8: u64 = {
                let a2409 = 4u64;
                let a2410 = {
                    let a2407 = {
                        let a2402 = {
                            let a2399 = v0;
                            let a2400 = v3;
                            let a2401 = 0u64;
                            nat_quot(a2399, a2400, a2401)
                        };
                        let a2403 = v1;
                        nat_mul(a2402, a2403)?
                    };
                    let a2408 = {
                        let a2404 = v6;
                        let a2405 = 3u64;
                        let a2406 = 0u64;
                        nat_quot(a2404, a2405, a2406)
                    };
                    nat_add(a2407, a2408)?
                };
                nat_mul(a2409, a2410)?
            };
            let a2411 = v4;
            let a2412 = v5;
            if nat_lt(a2411, a2412) {
                let a2413 = v7;
                let a2414 = 0u64;
                if nat_eq(a2413, a2414) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2415 = v7;
                    let a2416 = 1u64;
                    if nat_eq(a2415, a2416) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a2417 = v7;
                let a2418 = 0u64;
                if nat_eq(a2417, a2418) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2419 = v7;
                    let a2420 = 1u64;
                    if nat_eq(a2419, a2420) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2421 = v0;
        let a2422 = 6u64;
        let a2423 = 0u64;
        nat_quot(a2421, a2422, a2423)
    };
    let v5: u64 = {
        let a2424 = v0;
        let a2425 = 6u64;
        let a2426 = 0u64;
        nat_rem(a2424, a2425, a2426)
    };
    let a2427 = v5;
    let a2428 = 0u64;
    if nat_eq(a2427, a2428) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f149(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2429 = v5;
        let a2430 = 1u64;
        if nat_eq(a2429, a2430) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a2431 = v5;
            let a2432 = 2u64;
            if nat_eq(a2431, a2432) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f149(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2433 = v5;
                let a2434 = 3u64;
                if nat_eq(a2433, a2434) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2435 = v5;
                    let a2436 = 4u64;
                    if nat_eq(a2435, a2436) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f149(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f149(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2443 = {
            let a2441 = 3u64;
            let a2442 = {
                let a2439 = match v4.clone() {
                    Adt6::C0(h2437, _, _, _) => {
                        h2437
                    }
                };
                let a2440 = match v5.clone() {
                    Adt6::C0(h2438, _, _, _) => {
                        h2438
                    }
                };
                nat_mul(a2439, a2440)?
            };
            nat_mul(a2441, a2442)?
        };
        let a2444 = 2u64;
        nat_add(a2443, a2444)?
    };
    let v8: u64 = {
        let a2445 = v0;
        let a2446 = v7;
        let a2447 = 0u64;
        nat_quot(a2445, a2446, a2447)
    };
    let v9: u64 = {
        let a2448 = v0;
        let a2449 = v7;
        let a2450 = 0u64;
        nat_rem(a2448, a2449, a2450)
    };
    let a2451 = v9;
    let a2452 = 0u64;
    if nat_eq(a2451, a2452) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2455 = {
            let a2453 = v9;
            let a2454 = 1u64;
            nat_add(a2453, a2454)?
        };
        let a2456 = v7;
        if nat_eq(a2455, a2456) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f149(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2457 = v9;
                let a2458 = 1u64;
                nat_sub(a2457, a2458)
            };
            let v11: u64 = {
                let a2459 = v10;
                let a2460 = 3u64;
                let a2461 = 0u64;
                nat_quot(a2459, a2460, a2461)
            };
            let v12: u64 = {
                let a2463 = v11;
                let a2464 = match v4.clone() {
                    Adt6::C0(h2462, _, _, _) => {
                        h2462
                    }
                };
                let a2465 = 0u64;
                nat_rem(a2463, a2464, a2465)
            };
            let v13: u64 = {
                let a2467 = v11;
                let a2468 = match v4.clone() {
                    Adt6::C0(h2466, _, _, _) => {
                        h2466
                    }
                };
                let a2469 = 0u64;
                nat_quot(a2467, a2468, a2469)
            };
            let v14: u64 = {
                let a2470 = v10;
                let a2471 = 3u64;
                let a2472 = 0u64;
                nat_rem(a2470, a2471, a2472)
            };
            let v15: u64 = f73(v8, v3.clone(), 2u64);
            let m2474 = f75(v4, match v2.clone() {
                Adt3::C0(h2473, _, _, _, _, _, _, _) => {
                    h2473
                }
            }, f73(v8, v3.clone(), 0u64), v12)?;
            match m2474 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2476 = f75(v5, match v2.clone() {
                        Adt3::C0(_, h2475, _, _, _, _, _, _) => {
                            h2475
                        }
                    }, f73(v8, v3.clone(), 1u64), v13)?;
                    match m2476 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2477 = v14;
                            let a2478 = 0u64;
                            if nat_eq(a2477, a2478) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f72(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f167(v6), 4u64))
                            } else {
                                let a2479 = v14;
                                let a2480 = 1u64;
                                if nat_eq(a2479, a2480) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f72(v2, v16, v17, v15, f73(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f136(match v1.clone() {
        Adt3::C0(h2481, _, _, _, _, _, _, _) => {
            h2481
        }
    })?;
    let v5: u64 = {
        let a2482 = v0;
        let a2483 = v4;
        let a2484 = 0u64;
        nat_quot(a2482, a2483, a2484)
    };
    let v6: u64 = {
        let a2485 = v0;
        let a2486 = v4;
        let a2487 = 0u64;
        nat_rem(a2485, a2486, a2487)
    };
    let a2488 = v6;
    let a2489 = 0u64;
    if nat_eq(a2488, a2489) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2490 = v6;
        let a2491 = 1u64;
        if nat_eq(a2490, a2491) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2494 = {
                let a2492 = v6;
                let a2493 = 1u64;
                nat_add(a2492, a2493)?
            };
            let a2495 = v4;
            if nat_eq(a2494, a2495) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f149(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2496 = v6;
                    let a2497 = 2u64;
                    nat_sub(a2496, a2497)
                };
                let v8: u64 = {
                    let a2498 = v7;
                    let a2499 = 3u64;
                    let a2500 = 0u64;
                    nat_quot(a2498, a2499, a2500)
                };
                let v9: u64 = {
                    let a2501 = v7;
                    let a2502 = 3u64;
                    let a2503 = 0u64;
                    nat_rem(a2501, a2502, a2503)
                };
                let v10: u64 = f149({
                    let a2507 = {
                        let a2505 = v5;
                        let a2506 = match v1.clone() {
                            Adt3::C0(h2504, _, _, _, _, _, _, _) => {
                                h2504
                            }
                        };
                        nat_mul(a2505, a2506)?
                    };
                    let a2508 = v8;
                    nat_add(a2507, a2508)?
                }, v1)?;
                let a2509 = v9;
                let a2510 = 0u64;
                if nat_eq(a2509, a2510) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2511 = v9;
                    let a2512 = 1u64;
                    if nat_eq(a2511, a2512) {
                        let a2513 = v3;
                        let a2514 = 2u64;
                        if nat_eq(a2513, a2514) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2515 = v3;
                        let a2516 = 2u64;
                        if if nat_eq(a2515, a2516) {
                            true
                        } else {
                            let a2517 = v3;
                            let a2518 = 3u64;
                            nat_eq(a2517, a2518)
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

pub fn f84(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2520 = 2u64;
        let a2521 = match v2.clone() {
            Adt3::C0(_, h2519, _, _, _, _, _, _) => {
                h2519
            }
        };
        nat_mul(a2520, a2521)?
    };
    let a2522 = v1;
    let a2523 = v6;
    if nat_lt(a2522, a2523) {
        let a2527 = {
            let a2524 = v1;
            let a2525 = 2u64;
            let a2526 = 0u64;
            nat_rem(a2524, a2525, a2526)
        };
        let a2528 = 0u64;
        if nat_eq(a2527, a2528) {
            Ok(Adt9::C4(19u64, 4u64, f149({
                let a2529 = v1;
                let a2530 = 2u64;
                let a2531 = 0u64;
                nat_quot(a2529, a2530, a2531)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2532 = v5;
            let a2533 = 2147483648u64;
            if if nat_lt(a2532, a2533) {
                let a2535 = v5;
                let a2536 = match v3 {
                    Adt3::C0(_, h2534, _, _, _, _, _, _) => {
                        h2534
                    }
                };
                nat_lt(a2535, a2536)
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
            let a2539 = match v3.clone() {
                Adt3::C0(h2537, _, _, _, _, _, _, _) => {
                    h2537
                }
            };
            let a2540 = match v3.clone() {
                Adt3::C0(_, h2538, _, _, _, _, _, _) => {
                    h2538
                }
            };
            nat_mul(a2539, a2540)?
        };
        let a2543 = {
            let a2541 = v1;
            let a2542 = v6;
            nat_sub(a2541, a2542)
        };
        let a2544 = v7;
        if nat_lt(a2543, a2544) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f149({
                let a2545 = v1;
                let a2546 = v6;
                nat_sub(a2545, a2546)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2549 = {
                    let a2547 = v1;
                    let a2548 = v6;
                    nat_sub(a2547, a2548)
                };
                let a2550 = v7;
                nat_sub(a2549, a2550)
            };
            let v9: u64 = {
                let a2552 = 1u64;
                let a2553 = f168(v0, match v2.clone() {
                    Adt3::C0(h2551, _, _, _, _, _, _, _) => {
                        h2551
                    }
                })?;
                nat_add(a2552, a2553)?
            };
            let v10: u64 = {
                let a2554 = v8;
                let a2555 = v9;
                let a2556 = 0u64;
                nat_quot(a2554, a2555, a2556)
            };
            let v11: u64 = {
                let a2557 = v8;
                let a2558 = v9;
                let a2559 = 0u64;
                nat_rem(a2557, a2558, a2559)
            };
            let a2560 = v11;
            let a2561 = 0u64;
            if nat_eq(a2560, a2561) {
                Ok(Adt9::C4(19u64, 4u64, f149(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f114(v0) {
                    let a2565 = {
                        let a2562 = v11;
                        let a2563 = 2u64;
                        let a2564 = 0u64;
                        nat_rem(a2562, a2563, a2564)
                    };
                    let a2566 = 1u64;
                    if nat_eq(a2565, a2566) {
                        Ok(Adt9::C4(6u64, 0u64, f149({
                            let a2575 = {
                                let a2568 = v10;
                                let a2569 = match v2.clone() {
                                    Adt3::C0(h2567, _, _, _, _, _, _, _) => {
                                        h2567
                                    }
                                };
                                nat_mul(a2568, a2569)?
                            };
                            let a2576 = {
                                let a2572 = {
                                    let a2570 = v11;
                                    let a2571 = 1u64;
                                    nat_sub(a2570, a2571)
                                };
                                let a2573 = 2u64;
                                let a2574 = 0u64;
                                nat_quot(a2572, a2573, a2574)
                            };
                            nat_add(a2575, a2576)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f149({
                            let a2585 = {
                                let a2578 = v5;
                                let a2579 = match v2.clone() {
                                    Adt3::C0(h2577, _, _, _, _, _, _, _) => {
                                        h2577
                                    }
                                };
                                nat_mul(a2578, a2579)?
                            };
                            let a2586 = {
                                let a2582 = {
                                    let a2580 = v11;
                                    let a2581 = 1u64;
                                    nat_sub(a2580, a2581)
                                };
                                let a2583 = 2u64;
                                let a2584 = 0u64;
                                nat_quot(a2582, a2583, a2584)
                            };
                            nat_add(a2585, a2586)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f149({
                            let a2595 = {
                                let a2588 = v5;
                                let a2589 = match v2 {
                                    Adt3::C0(h2587, _, _, _, _, _, _, _) => {
                                        h2587
                                    }
                                };
                                nat_mul(a2588, a2589)?
                            };
                            let a2596 = {
                                let a2592 = {
                                    let a2590 = v11;
                                    let a2591 = 1u64;
                                    nat_sub(a2590, a2591)
                                };
                                let a2593 = 2u64;
                                let a2594 = 0u64;
                                nat_quot(a2592, a2593, a2594)
                            };
                            nat_add(a2595, a2596)?
                        }, v3)?))
                    }
                } else {
                    f169(v0, v10, {
                        let a2597 = v11;
                        let a2598 = 1u64;
                        nat_sub(a2597, a2598)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2599 = v1;
    let a2600 = f170(v2.clone(), v4.clone())?;
    if nat_lt(a2599, a2600) {
        let a2603 = v1;
        let a2604 = {
            let a2601 = 2u64;
            let a2602 = f137(v4.clone())?;
            nat_mul(a2601, a2602)?
        };
        if nat_lt(a2603, a2604) {
            let a2608 = {
                let a2605 = v1;
                let a2606 = 2u64;
                let a2607 = 0u64;
                nat_rem(a2605, a2606, a2607)
            };
            let a2609 = 0u64;
            if nat_eq(a2608, a2609) {
                f171({
                    let a2610 = v1;
                    let a2611 = 2u64;
                    let a2612 = 0u64;
                    nat_quot(a2610, a2611, a2612)
                }, v4)
            } else {
                if f172(match v2 {
                    Adt3::C0(_, h2613, _, _, _, _, _, _) => {
                        h2613
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2616 = v1;
                let a2617 = {
                    let a2614 = 2u64;
                    let a2615 = f137(v4.clone())?;
                    nat_mul(a2614, a2615)?
                };
                nat_sub(a2616, a2617)
            };
            let v7: u64 = {
                let a2621 = v6;
                let a2622 = {
                    let a2619 = 1u64;
                    let a2620 = match v2.clone() {
                        Adt3::C0(h2618, _, _, _, _, _, _, _) => {
                            h2618
                        }
                    };
                    nat_add(a2619, a2620)?
                };
                let a2623 = 0u64;
                nat_quot(a2621, a2622, a2623)
            };
            let v8: u64 = {
                let a2627 = v6;
                let a2628 = {
                    let a2625 = 1u64;
                    let a2626 = match v2.clone() {
                        Adt3::C0(h2624, _, _, _, _, _, _, _) => {
                            h2624
                        }
                    };
                    nat_add(a2625, a2626)?
                };
                let a2629 = 0u64;
                nat_rem(a2627, a2628, a2629)
            };
            let a2630 = v8;
            let a2631 = 0u64;
            if nat_eq(a2630, a2631) {
                f171(v7, v4)
            } else {
                f173(v0, v7, {
                    let a2632 = v8;
                    let a2633 = 1u64;
                    nat_sub(a2632, a2633)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2636 = match v4.clone() {
            Adt3::C0(h2634, _, _, _, _, _, _, _) => {
                h2634
            }
        };
        let a2637 = match v4.clone() {
            Adt3::C0(_, h2635, _, _, _, _, _, _) => {
                h2635
            }
        };
        nat_mul(a2636, a2637)?
    };
    let v7: u64 = {
        let a2638 = 2u64;
        let a2639 = v6;
        nat_mul(a2638, a2639)?
    };
    let v8: u64 = f174(match v3.clone() {
        Adt3::C0(_, h2640, _, _, _, _, _, _) => {
            h2640
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2641, _, _, _, _, _) => {
            h2641
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2642, _, _, _, _) => {
            h2642
        }
    });
    let a2643 = v0;
    let a2644 = v7;
    if nat_lt(a2643, a2644) {
        let a2648 = {
            let a2645 = v0;
            let a2646 = 2u64;
            let a2647 = 0u64;
            nat_rem(a2645, a2646, a2647)
        };
        let a2649 = 0u64;
        if nat_eq(a2648, a2649) {
            Ok(Adt9::C4(19u64, 5u64, f149({
                let a2650 = v0;
                let a2651 = 2u64;
                let a2652 = 0u64;
                nat_quot(a2650, a2651, a2652)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f175(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2653 = v0;
            let a2654 = v7;
            nat_sub(a2653, a2654)
        };
        let v10: u64 = {
            let a2658 = v9;
            let a2659 = {
                let a2656 = 1u64;
                let a2657 = match v1.clone() {
                    Adt3::C0(h2655, _, _, _, _, _, _, _) => {
                        h2655
                    }
                };
                nat_add(a2656, a2657)?
            };
            let a2660 = 0u64;
            nat_quot(a2658, a2659, a2660)
        };
        let v11: u64 = {
            let a2664 = v9;
            let a2665 = {
                let a2662 = 1u64;
                let a2663 = match v1.clone() {
                    Adt3::C0(h2661, _, _, _, _, _, _, _) => {
                        h2661
                    }
                };
                nat_add(a2662, a2663)?
            };
            let a2666 = 0u64;
            nat_rem(a2664, a2665, a2666)
        };
        let a2667 = v11;
        let a2668 = 0u64;
        if nat_eq(a2667, a2668) {
            Ok(Adt9::C4(19u64, 5u64, f149({
                let a2669 = v10;
                let a2670 = v6;
                let a2671 = 0u64;
                nat_rem(a2669, a2670, a2671)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f175(v5, v8) {
                let v12: u64 = {
                    let a2677 = {
                        let a2673 = v10;
                        let a2674 = match v1.clone() {
                            Adt3::C0(h2672, _, _, _, _, _, _, _) => {
                                h2672
                            }
                        };
                        nat_mul(a2673, a2674)?
                    };
                    let a2678 = {
                        let a2675 = v11;
                        let a2676 = 1u64;
                        nat_sub(a2675, a2676)
                    };
                    nat_add(a2677, a2678)?
                };
                let a2679 = v5;
                let a2680 = 4294967295u64;
                if nat_eq(a2679, a2680) {
                    Ok(Adt9::C1(false, f149(v12, v1)?, f149(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f149(v12, v1)?, 4u64, f149({
                        let a2686 = {
                            let a2682 = v5;
                            let a2683 = match v3.clone() {
                                Adt3::C0(h2681, _, _, _, _, _, _, _) => {
                                    h2681
                                }
                            };
                            nat_mul(a2682, a2683)?
                        };
                        let a2687 = {
                            let a2684 = v11;
                            let a2685 = 1u64;
                            nat_sub(a2684, a2685)
                        };
                        nat_add(a2686, a2687)?
                    }, v3)?, 3u64, f149(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f87(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f119(v3.clone())?;
    let v6: u64 = {
        let a2688 = v0;
        let a2689 = v5;
        let a2690 = 0u64;
        nat_quot(a2688, a2689, a2690)
    };
    let v7: u64 = {
        let a2691 = v0;
        let a2692 = v5;
        let a2693 = 0u64;
        nat_rem(a2691, a2692, a2693)
    };
    let v8: u64 = f176(v3.clone())?;
    let a2694 = v7;
    let a2695 = 0u64;
    if nat_eq(a2694, a2695) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2696, _) => {
                h2696
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2700 = v5;
        let a2701 = {
            let a2698 = v7;
            let a2699 = if match v3.clone() {
                Adt7::C0(_, _, _, h2697, _) => {
                    h2697
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2698, a2699)?
        };
        if nat_le(a2700, a2701) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2702, _) => {
                    h2702
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f149(v6, v2)?))
            } else {
                let a2705 = {
                    let a2703 = v7;
                    let a2704 = 2u64;
                    nat_add(a2703, a2704)?
                };
                let a2706 = v5;
                if nat_eq(a2705, a2706) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2707, _, _) => {
                            h2707
                        }
                    } {
                        let a2714 = {
                            let m2710 = match v3.clone() {
                                Adt7::C0(h2709, _, _, _, _) => {
                                    h2709
                                }
                            };
                            match m2710 {
                                Adt6::C0(h2708, _, _, _) => {
                                    h2708
                                }
                            }
                        };
                        let a2715 = {
                            let m2713 = match v3 {
                                Adt7::C0(_, h2712, _, _, _) => {
                                    h2712
                                }
                            };
                            match m2713 {
                                Adt6::C0(h2711, _, _, _) => {
                                    h2711
                                }
                            }
                        };
                        nat_mul(a2714, a2715)?
                    } else {
                        f177(match v3 {
                            Adt7::C0(h2716, _, _, _, _) => {
                                h2716
                            }
                        }, match v1 {
                            Adt3::C0(h2717, _, _, _, _, _, _, _) => {
                                h2717
                            }
                        }, f73(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2718 = v9;
                        let a2719 = 0u64;
                        if nat_eq(a2718, a2719) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f149(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2722 = {
                    let a2720 = v7;
                    let a2721 = 1u64;
                    nat_sub(a2720, a2721)
                };
                let a2723 = v8;
                let a2724 = 0u64;
                nat_rem(a2722, a2723, a2724)
            };
            let m2730 = f178(v3.clone(), v1, v2, v6, {
                let a2727 = {
                    let a2725 = v7;
                    let a2726 = 1u64;
                    nat_sub(a2725, a2726)
                };
                let a2728 = v8;
                let a2729 = 0u64;
                nat_quot(a2727, a2728, a2729)
            })?;
            match m2730 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2732 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2731) => {
                            h2731
                        }
                    };
                    let a2733 = 1u64;
                    if if nat_eq(a2732, a2733) {
                        let a2734 = v10;
                        let a2735 = 0u64;
                        nat_eq(a2734, a2735)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2736, _) => {
                                h2736
                            }
                        } {
                            let a2739 = {
                                let a2737 = v10;
                                let a2738 = 1u64;
                                nat_add(a2737, a2738)?
                            };
                            let a2740 = v8;
                            if nat_eq(a2739, a2740) {
                                let a2741 = v4;
                                let a2742 = 2u64;
                                if nat_eq(a2741, a2742) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2744 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2743) => {
                                                h2743
                                            }
                                        };
                                        let a2745 = 1u64;
                                        if nat_eq(a2744, a2745) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2747 = match v3 {
                                            Adt7::C0(_, _, _, _, h2746) => {
                                                h2746
                                            }
                                        };
                                        let a2748 = 1u64;
                                        if nat_eq(a2747, a2748) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2750 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2749) => {
                                            h2749
                                        }
                                    };
                                    let a2751 = 1u64;
                                    if nat_eq(a2750, a2751) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2753 = match v3 {
                                        Adt7::C0(_, _, _, _, h2752) => {
                                            h2752
                                        }
                                    };
                                    let a2754 = 1u64;
                                    if nat_eq(a2753, a2754) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2756 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2755) => {
                                        h2755
                                    }
                                };
                                let a2757 = 1u64;
                                if nat_eq(a2756, a2757) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2759 = match v3 {
                                    Adt7::C0(_, _, _, _, h2758) => {
                                        h2758
                                    }
                                };
                                let a2760 = 1u64;
                                if nat_eq(a2759, a2760) {
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

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f117(match v3.clone() {
        Adt3::C0(h2761, _, _, _, _, _, _, _) => {
            h2761
        }
    })?;
    let v5: u64 = {
        let a2762 = v0;
        let a2763 = v4;
        let a2764 = 0u64;
        nat_rem(a2762, a2763, a2764)
    };
    let v6: u64 = {
        let a2765 = v0;
        let a2766 = v4;
        let a2767 = 0u64;
        nat_quot(a2765, a2766, a2767)
    };
    let a2768 = v5;
    let a2769 = 0u64;
    if nat_eq(a2768, a2769) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2772 = {
            let a2770 = v5;
            let a2771 = 1u64;
            nat_add(a2770, a2771)?
        };
        let a2773 = v4;
        if nat_eq(a2772, a2773) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f149(v6, v2)?))
        } else {
            let a2777 = {
                let a2774 = v5;
                let a2775 = 2u64;
                let a2776 = 0u64;
                nat_rem(a2774, a2775, a2776)
            };
            let a2778 = 0u64;
            if nat_eq(a2777, a2778) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2781 = {
                        let a2779 = v5;
                        let a2780 = 1u64;
                        nat_sub(a2779, a2780)
                    };
                    let a2782 = 2u64;
                    let a2783 = 0u64;
                    nat_quot(a2781, a2782, a2783)
                };
                let v8: u64 = f73(v6, v2.clone(), 0u64);
                let v9: u64 = f73(v6, v2.clone(), 1u64);
                let v10: u64 = f73(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f72(v1, {
                    let a2784 = v9;
                    let a2785 = v7;
                    nat_add(a2784, a2785)?
                }, v8, v10, 0u64)?, 4u64, f72(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f89(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f116(v3.clone())?;
    let v5: u64 = {
        let a2786 = v0;
        let a2787 = v4;
        let a2788 = 0u64;
        nat_rem(a2786, a2787, a2788)
    };
    let v6: u64 = {
        let a2789 = v0;
        let a2790 = v4;
        let a2791 = 0u64;
        nat_quot(a2789, a2790, a2791)
    };
    let a2792 = v5;
    let a2793 = 0u64;
    if nat_eq(a2792, a2793) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2796 = {
            let a2794 = v5;
            let a2795 = 1u64;
            nat_add(a2794, a2795)?
        };
        let a2797 = v4;
        if nat_eq(a2796, a2797) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f149(v6, v2)?))
        } else {
            let a2801 = {
                let a2798 = v5;
                let a2799 = 2u64;
                let a2800 = 0u64;
                nat_rem(a2798, a2799, a2800)
            };
            let a2802 = 0u64;
            if nat_eq(a2801, a2802) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2804 = f73(v6, v2.clone(), 0u64);
                    let a2805 = match v3.clone() {
                        Adt6::C0(_, _, h2803, _) => {
                            h2803
                        }
                    };
                    nat_add(a2804, a2805)?
                };
                let v8: u64 = {
                    let a2813 = {
                        let a2810 = {
                            let a2807 = v7;
                            let a2808 = match v3.clone() {
                                Adt6::C0(_, h2806, _, _) => {
                                    h2806
                                }
                            };
                            nat_add(a2807, a2808)?
                        };
                        let a2811 = match v3.clone() {
                            Adt6::C0(h2809, _, _, _) => {
                                h2809
                            }
                        };
                        nat_sub(a2810, a2811)
                    };
                    let a2814 = match v3.clone() {
                        Adt6::C0(_, h2812, _, _) => {
                            h2812
                        }
                    };
                    let a2815 = 0u64;
                    nat_quot(a2813, a2814, a2815)
                };
                let v9: u64 = {
                    let a2821 = v8;
                    let a2822 = {
                        let a2818 = {
                            let a2816 = v5;
                            let a2817 = 1u64;
                            nat_sub(a2816, a2817)
                        };
                        let a2819 = 2u64;
                        let a2820 = 0u64;
                        nat_quot(a2818, a2819, a2820)
                    };
                    nat_add(a2821, a2822)?
                };
                let v10: u64 = {
                    let a2824 = v9;
                    let a2825 = match v3.clone() {
                        Adt6::C0(_, h2823, _, _) => {
                            h2823
                        }
                    };
                    nat_mul(a2824, a2825)?
                };
                let v11: u64 = {
                    let a2826 = v7;
                    let a2827 = v10;
                    nat_sub(a2826, a2827)
                };
                let a2829 = v9;
                let a2830 = match v1.clone() {
                    Adt3::C0(_, h2828, _, _, _, _, _, _) => {
                        h2828
                    }
                };
                if if if nat_lt(a2829, a2830) {
                    let a2831 = v10;
                    let a2832 = v7;
                    nat_le(a2831, a2832)
                } else {
                    false
                } {
                    let a2834 = v11;
                    let a2835 = match v3.clone() {
                        Adt6::C0(h2833, _, _, _) => {
                            h2833
                        }
                    };
                    nat_lt(a2834, a2835)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f72(v1, {
                        let a2839 = {
                            let a2837 = f73(v6, v2, 1u64);
                            let a2838 = match v3 {
                                Adt6::C0(h2836, _, _, _) => {
                                    h2836
                                }
                            };
                            nat_mul(a2837, a2838)?
                        };
                        let a2840 = v11;
                        nat_add(a2839, a2840)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f73(v0, v2.clone(), 0u64);
    let v5: u64 = f73(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2842 = v5;
        let a2843 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2841, _) => {
                h2841
            }
        };
        let a2844 = 0u64;
        nat_rem(a2842, a2843, a2844)
    };
    let v7: u64 = {
        let a2846 = v5;
        let a2847 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2845, _) => {
                h2845
            }
        };
        let a2848 = 0u64;
        nat_quot(a2846, a2847, a2848)
    };
    let v8: Option<u64> = f75(match v3.clone() {
        Adt5::C0(h2849, _, _, _, _, _, _, _) => {
            h2849
        }
    }, match v1.clone() {
        Adt3::C0(h2850, _, _, _, _, _, _, _) => {
            h2850
        }
    }, f73(v0, v2.clone(), 1u64), {
        let a2854 = v4;
        let a2855 = {
            let m2853 = match v3.clone() {
                Adt5::C0(h2852, _, _, _, _, _, _, _) => {
                    h2852
                }
            };
            match m2853 {
                Adt6::C0(h2851, _, _, _) => {
                    h2851
                }
            }
        };
        let a2856 = 0u64;
        nat_rem(a2854, a2855, a2856)
    })?;
    let v9: u64 = {
        let a2860 = v4;
        let a2861 = {
            let m2859 = match v3.clone() {
                Adt5::C0(h2858, _, _, _, _, _, _, _) => {
                    h2858
                }
            };
            match m2859 {
                Adt6::C0(h2857, _, _, _) => {
                    h2857
                }
            }
        };
        let a2862 = 0u64;
        nat_quot(a2860, a2861, a2862)
    };
    let v10: Option<u64> = f75(match v3.clone() {
        Adt5::C0(_, h2863, _, _, _, _, _, _) => {
            h2863
        }
    }, match v1.clone() {
        Adt3::C0(_, h2864, _, _, _, _, _, _) => {
            h2864
        }
    }, f73(v0, v2.clone(), 2u64), {
        let a2868 = v9;
        let a2869 = {
            let m2867 = match v3.clone() {
                Adt5::C0(_, h2866, _, _, _, _, _, _) => {
                    h2866
                }
            };
            match m2867 {
                Adt6::C0(h2865, _, _, _) => {
                    h2865
                }
            }
        };
        let a2870 = 0u64;
        nat_rem(a2868, a2869, a2870)
    })?;
    let v11: u64 = {
        let a2874 = v9;
        let a2875 = {
            let m2873 = match v3.clone() {
                Adt5::C0(_, h2872, _, _, _, _, _, _) => {
                    h2872
                }
            };
            match m2873 {
                Adt6::C0(h2871, _, _, _) => {
                    h2871
                }
            }
        };
        let a2876 = 0u64;
        nat_quot(a2874, a2875, a2876)
    };
    let v12: Option<u64> = f75(match v3.clone() {
        Adt5::C0(_, _, h2877, _, _, _, _, _) => {
            h2877
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2878, _, _, _, _, _) => {
            h2878
        }
    }, v6, {
        let a2882 = v11;
        let a2883 = {
            let m2881 = match v3.clone() {
                Adt5::C0(_, _, h2880, _, _, _, _, _) => {
                    h2880
                }
            };
            match m2881 {
                Adt6::C0(h2879, _, _, _) => {
                    h2879
                }
            }
        };
        let a2884 = 0u64;
        nat_rem(a2882, a2883, a2884)
    })?;
    let v13: u64 = {
        let a2888 = v11;
        let a2889 = {
            let m2887 = match v3.clone() {
                Adt5::C0(_, _, h2886, _, _, _, _, _) => {
                    h2886
                }
            };
            match m2887 {
                Adt6::C0(h2885, _, _, _) => {
                    h2885
                }
            }
        };
        let a2890 = 0u64;
        nat_quot(a2888, a2889, a2890)
    };
    let v17: Adt12 = {
        let m2891 = v8;
        match m2891 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2892 = v10;
                match m2892 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2893 = v12;
                        match m2893 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2902 = {
                                        let a2897 = f72(v1, v14, v15, v16, 0u64)?;
                                        let a2898 = {
                                            let a2895 = v7;
                                            let a2896 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2894, _, _, _) => {
                                                    h2894
                                                }
                                            };
                                            nat_mul(a2895, a2896)?
                                        };
                                        nat_add(a2897, a2898)?
                                    };
                                    let a2903 = {
                                        let a2900 = v13;
                                        let a2901 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2899, _, _, _, _) => {
                                                h2899
                                            }
                                        };
                                        nat_mul(a2900, a2901)?
                                    };
                                    nat_add(a2902, a2903)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2904, _, _) => {
            h2904
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2905) => {
            h2905
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f149(v0, v2)?), {
        let a2907 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2906, _, _) => {
                h2906
            }
        };
        let a2908 = 0u64;
        if nat_eq(a2907, a2908) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2910 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2909) => {
                h2909
            }
        };
        let a2911 = 0u64;
        if nat_eq(a2910, a2911) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f91(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2924 = f179(v0, v1, match v2.clone() {
        Adt3::C0(h2912, _, _, _, _, _, _, _) => {
            h2912
        }
    }, match v2.clone() {
        Adt3::C0(_, h2913, _, _, _, _, _, _) => {
            h2913
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2914, _, _, _, _, _) => {
            h2914
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2915, _, _, _, _) => {
            h2915
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2916, _, _, _) => {
            h2916
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2917, _, _) => {
            h2917
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2918, _) => {
            h2918
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2919) => {
            h2919
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2920, _, _, _) => {
            h2920
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2921, _, _) => {
            h2921
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2922, _) => {
            h2922
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2923) => {
            h2923
        }
    }, v4)?;
    match m2924 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2927 = v6;
        let a2928 = {
            let a2925 = 3u64;
            let a2926 = v5;
            nat_add(a2925, a2926)?
        };
        nat_sub(a2927, a2928)
    };
    let a2930 = v0;
    let a2931 = f36(v3, match v1.clone() {
        Adt3::C0(h2929, _, _, _, _, _, _, _) => {
            h2929
        }
    }, v5, v8)?;
    if nat_lt(a2930, a2931) {
        let v9: u64 = f106(match v1.clone() {
            Adt3::C0(h2932, _, _, _, _, _, _, _) => {
                h2932
            }
        }, v5, v8)?;
        let v10: u64 = f107(v3, match v1.clone() {
            Adt3::C0(h2933, _, _, _, _, _, _, _) => {
                h2933
            }
        }, v5)?;
        let a2934 = v0;
        let a2935 = v9;
        if nat_lt(a2934, a2935) {
            f180(v0, v2, f30(v3), {
                let a2936 = 3u64;
                let a2937 = v5;
                nat_add(a2936, a2937)?
            }, v7)
        } else {
            let a2940 = v0;
            let a2941 = {
                let a2938 = v9;
                let a2939 = v10;
                nat_add(a2938, a2939)?
            };
            if nat_lt(a2940, a2941) {
                let v11: u64 = {
                    let a2942 = v0;
                    let a2943 = v9;
                    nat_sub(a2942, a2943)
                };
                let v12: u64 = f181(v3, match v1.clone() {
                    Adt3::C0(h2944, _, _, _, _, _, _, _) => {
                        h2944
                    }
                })?;
                f182({
                    let a2945 = v11;
                    let a2946 = v12;
                    let a2947 = 0u64;
                    nat_rem(a2945, a2946, a2947)
                }, {
                    let a2948 = v11;
                    let a2949 = v12;
                    let a2950 = 0u64;
                    nat_quot(a2948, a2949, a2950)
                }, v1, v3, v4, v7)
            } else {
                f183({
                    let a2953 = v0;
                    let a2954 = {
                        let a2951 = v9;
                        let a2952 = v10;
                        nat_add(a2951, a2952)?
                    };
                    nat_sub(a2953, a2954)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2955 = v0;
    let a2956 = f19(v1)?;
    if nat_lt(a2955, a2956) {
        let a2957 = v0;
        let a2958 = 4u64;
        if nat_lt(a2957, a2958) {
            let a2959 = v0;
            let a2960 = 0u64;
            if nat_eq(a2959, a2960) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2961 = v0;
                let a2962 = 1u64;
                if nat_eq(a2961, a2962) {
                    let a2963 = v5;
                    let a2964 = 0u64;
                    if nat_eq(a2963, a2964) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2965 = v0;
                    let a2966 = 2u64;
                    if nat_eq(a2965, a2966) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2967 = v0;
            let a2968 = 4u64;
            if if nat_eq(a2967, a2968) {
                let a2969 = f184(v5)?;
                let a2970 = v1;
                !nat_eq(a2969, a2970)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2973 = {
                        let a2971 = v0;
                        let a2972 = 4u64;
                        nat_sub(a2971, a2972)
                    };
                    let a2974 = 3u64;
                    let a2975 = 0u64;
                    nat_quot(a2973, a2974, a2975)
                };
                let v7: u64 = {
                    let a2978 = {
                        let a2976 = v0;
                        let a2977 = 4u64;
                        nat_sub(a2976, a2977)
                    };
                    let a2979 = 3u64;
                    let a2980 = 0u64;
                    nat_rem(a2978, a2979, a2980)
                };
                let a2981 = v7;
                let a2982 = 0u64;
                if nat_eq(a2981, a2982) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2983 = v7;
                    let a2984 = 1u64;
                    if nat_eq(a2983, a2984) {
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

pub fn f94(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2985 = v1;
        let a2986 = 5u64;
        nat_eq(a2985, a2986)
    } else {
        false
    } {
        let a2987 = v2;
        let a2988 = 16u64;
        nat_eq(a2987, a2988)
    } else {
        false
    } {
        let a2989 = v3;
        let a2990 = 0u64;
        nat_eq(a2989, a2990)
    } else {
        false
    } {
        let a2991 = v4;
        let a2992 = 0u64;
        nat_eq(a2991, a2992)
    } else {
        false
    }
}

pub fn f95(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2993 = v1;
    let a2994 = 0u64;
    if nat_eq(a2993, a2994) {
        if v0 {
            let a2995 = v3;
            let a2996 = 0u64;
            if if if nat_eq(a2995, a2996) {
                let a2997 = v4;
                let a2998 = 43u64;
                nat_eq(a2997, a2998)
            } else {
                false
            } {
                true
            } else {
                let a2999 = v3;
                let a3000 = 43u64;
                if nat_eq(a2999, a3000) {
                    let a3001 = v4;
                    let a3002 = 0u64;
                    nat_eq(a3001, a3002)
                } else {
                    false
                }
            } {
                f166(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3003 = v1;
        let a3004 = 1u64;
        if nat_eq(a3003, a3004) {
            Adt11::C1(v2)
        } else {
            let a3005 = v1;
            let a3006 = 2u64;
            if nat_eq(a3005, a3006) {
                Adt11::C5(v2)
            } else {
                let a3007 = v1;
                let a3008 = 3u64;
                if nat_eq(a3007, a3008) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3009 = v1;
                    let a3010 = 4u64;
                    if nat_eq(a3009, a3010) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f94(v0, v1, v2, v3, v4) {
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

pub fn f96(v0: u64, v1: u64) -> Adt12 {
    let a3011 = v0;
    let a3012 = 0u64;
    if nat_eq(a3011, a3012) {
        Adt12::C0(0u64, v1)
    } else {
        let a3013 = v0;
        let a3014 = 1u64;
        if nat_eq(a3013, a3014) {
            Adt12::C1(v1)
        } else {
            let a3015 = v0;
            let a3016 = 2u64;
            if nat_eq(a3015, a3016) {
                Adt12::C2(v1)
            } else {
                let a3017 = v0;
                let a3018 = 3u64;
                if nat_eq(a3017, a3018) {
                    Adt12::C3(v1)
                } else {
                    let a3019 = v0;
                    let a3020 = 13u64;
                    if nat_lt(a3019, a3020) {
                        Adt12::C0({
                            let a3021 = v0;
                            let a3022 = 3u64;
                            nat_sub(a3021, a3022)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f97(v0: u64, v1: u64) -> Adt13 {
    let a3023 = v0;
    let a3024 = 1u64;
    if nat_eq(a3023, a3024) {
        Adt13::C0(v1)
    } else {
        let a3025 = v0;
        let a3026 = 3u64;
        if if nat_eq(a3025, a3026) {
            true
        } else {
            let a3027 = v0;
            let a3028 = 6u64;
            nat_eq(a3027, a3028)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f98(v0: Adt6, v1: u64) -> R<u64> {
    let a3030 = 0u64;
    let a3031 = match v0.clone() {
        Adt6::C0(h3029, _, _, _) => {
            h3029
        }
    };
    if if if if if nat_lt(a3030, a3031) {
        let a3033 = 0u64;
        let a3034 = match v0.clone() {
            Adt6::C0(_, h3032, _, _) => {
                h3032
            }
        };
        nat_lt(a3033, a3034)
    } else {
        false
    } {
        let a3036 = 0u64;
        let a3037 = match v0.clone() {
            Adt6::C0(_, _, _, h3035) => {
                h3035
            }
        };
        nat_lt(a3036, a3037)
    } else {
        false
    } {
        let a3039 = match v0.clone() {
            Adt6::C0(_, h3038, _, _) => {
                h3038
            }
        };
        let a3040 = 2147483648u64;
        nat_lt(a3039, a3040)
    } else {
        false
    } {
        let a3042 = match v0.clone() {
            Adt6::C0(_, _, _, h3041) => {
                h3041
            }
        };
        let a3043 = 2147483648u64;
        nat_lt(a3042, a3043)
    } else {
        false
    } {
        let v2: u64 = {
            let a3047 = v1;
            let a3048 = {
                let a3045 = 2u64;
                let a3046 = f143(match v0.clone() {
                    Adt6::C0(_, _, h3044, _) => {
                        h3044
                    }
                });
                nat_mul(a3045, a3046)?
            };
            nat_add(a3047, a3048)?
        };
        let v3: u64 = {
            let a3060 = {
                let a3055 = 1u64;
                let a3056 = {
                    let a3053 = match v0.clone() {
                        Adt6::C0(_, _, _, h3049) => {
                            h3049
                        }
                    };
                    let a3054 = {
                        let a3051 = match v0.clone() {
                            Adt6::C0(h3050, _, _, _) => {
                                h3050
                            }
                        };
                        let a3052 = 1u64;
                        nat_sub(a3051, a3052)
                    };
                    nat_mul(a3053, a3054)?
                };
                nat_add(a3055, a3056)?
            };
            let a3061 = {
                let a3058 = 2u64;
                let a3059 = f142(match v0.clone() {
                    Adt6::C0(_, _, h3057, _) => {
                        h3057
                    }
                });
                nat_mul(a3058, a3059)?
            };
            nat_add(a3060, a3061)?
        };
        let a3062 = v3;
        let a3063 = v2;
        if nat_le(a3062, a3063) {
            let a3070 = 1u64;
            let a3071 = {
                let a3067 = {
                    let a3064 = v2;
                    let a3065 = v3;
                    nat_sub(a3064, a3065)
                };
                let a3068 = match v0 {
                    Adt6::C0(_, h3066, _, _) => {
                        h3066
                    }
                };
                let a3069 = 0u64;
                nat_quot(a3067, a3068, a3069)
            };
            nat_add(a3070, a3071)
        } else {
            let a3075 = {
                let a3072 = v3;
                let a3073 = v2;
                nat_sub(a3072, a3073)
            };
            let a3076 = match v0 {
                Adt6::C0(_, h3074, _, _) => {
                    h3074
                }
            };
            if nat_lt(a3075, a3076) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f99(v0: u64) -> u64 {
    let v1: u64 = {
        let a3077 = v0;
        let a3078 = 4294967296u64;
        let a3079 = 0u64;
        nat_rem(a3077, a3078, a3079)
    };
    let a3080 = v1;
    let a3081 = 2147483648u64;
    if nat_lt(a3080, a3081) {
        v1
    } else {
        0u64
    }
}

pub fn f100(v0: u64) -> R<u64> {
    let a3084 = 2u64;
    let a3085 = {
        let a3082 = 2u64;
        let a3083 = v0;
        nat_mul(a3082, a3083)?
    };
    nat_add(a3084, a3085)
}

pub fn f101(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3087 = match v1.clone() {
        Adt14::C0(h3086, _, _, _, _, _, _, _, _, _, _) => {
            h3086
        }
    };
    let a3088 = 0u64;
    if if if if nat_eq(a3087, a3088) {
        let a3090 = match v2.clone() {
            Adt14::C0(h3089, _, _, _, _, _, _, _, _, _, _) => {
                h3089
            }
        };
        let a3091 = 0u64;
        nat_eq(a3090, a3091)
    } else {
        false
    } {
        f185(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f186(match v1.clone() {
            Adt14::C0(_, h3092, _, _, _, _, _, _, _, _, _) => {
                h3092
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3093, _, _, _, _, _, _, _, _) => {
                h3093
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3094, _, _, _, _, _, _, _) => {
                h3094
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3095, _, _, _, _, _, _) => {
                h3095
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f187(v1.clone(), v2.clone())
            } else {
                false
            } {
                f188(v1.clone())
            } else {
                false
            } {
                f188(v2.clone())
            } else {
                false
            } {
                if f189(v1.clone(), v2.clone())? {
                    true
                } else {
                    f190(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f186(match v1.clone() {
            Adt14::C0(_, h3096, _, _, _, _, _, _, _, _, _) => {
                h3096
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3097, _, _, _, _, _, _, _, _) => {
                h3097
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3098, _, _, _, _, _, _, _) => {
                h3098
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3099, _, _, _, _, _, _) => {
                h3099
            }
        }) {
            0u64
        } else {
            let a3105 = match v1.clone() {
                Adt14::C0(_, _, h3100, _, _, _, _, _, _, _, _) => {
                    h3100
                }
            };
            let a3106 = {
                let a3103 = match v1.clone() {
                    Adt14::C0(_, _, _, h3101, _, _, _, _, _, _, _) => {
                        h3101
                    }
                };
                let a3104 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3102, _, _, _, _, _, _) => {
                        h3102
                    }
                };
                nat_mul(a3103, a3104)?
            };
            nat_mul(a3105, a3106)?
        };
        let v4: u64 = {
            let a3107 = v3;
            let a3108 = 0u64;
            if if nat_eq(a3107, a3108) {
                true
            } else {
                f191(v2.clone())?
            } {
                0u64
            } else {
                let a3110 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3109, _) => {
                        h3109
                    }
                };
                let a3111 = 4u64;
                let a3112 = 0u64;
                nat_quot(a3110, a3111, a3112)
            }
        };
        Ok(Adt15::C1(f192(f193(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3113, _, _, _, _, _) => {
                h3113
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3114, _, _, _, _, _) => {
                h3114
            }
        }), match v1 {
            Adt14::C0(_, h3115, _, _, _, _, _, _, _, _, _) => {
                h3115
            }
        }, v3, v4)?, 4u64, 64u64, v3, f194(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f102(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f195(v3.clone());
    let a3117 = match v2.clone() {
        Adt14::C0(h3116, _, _, _, _, _, _, _, _, _, _) => {
            h3116
        }
    };
    let a3118 = 0u64;
    if if if if nat_eq(a3117, a3118) {
        let a3120 = match v3.clone() {
            Adt14::C0(h3119, _, _, _, _, _, _, _, _, _, _) => {
                h3119
            }
        };
        let a3121 = 0u64;
        nat_eq(a3120, a3121)
    } else {
        false
    } {
        f185(v2.clone(), v3)
    } else {
        false
    } {
        if f186(match v2.clone() {
            Adt14::C0(_, h3122, _, _, _, _, _, _, _, _, _) => {
                h3122
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3123, _, _, _, _, _, _, _, _) => {
                h3123
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3124, _, _, _, _, _, _, _) => {
                h3124
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3125, _, _, _, _, _, _) => {
                h3125
            }
        }) {
            true
        } else {
            let a3127 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3126, _, _, _, _, _) => {
                    h3126
                }
            };
            let a3128 = 4u64;
            if if if if if nat_eq(a3127, a3128) {
                f196(match v2.clone() {
                    Adt14::C0(_, h3129, _, _, _, _, _, _, _, _, _) => {
                        h3129
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3130, _, _, _, _, _, _, _, _) => {
                        h3130
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3131, _, _, _, _, _, _, _) => {
                        h3131
                    }
                }, v0)?
            } else {
                false
            } {
                f188(v2.clone())
            } else {
                false
            } {
                f188(v4.clone())
            } else {
                false
            } {
                let a3133 = f197(v4.clone())?;
                let a3134 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3132, _) => {
                        h3132
                    }
                };
                nat_le(a3133, a3134)
            } else {
                false
            } {
                if f189(v2.clone(), v4.clone())? {
                    true
                } else {
                    f190(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f186(match v2.clone() {
            Adt14::C0(_, h3135, _, _, _, _, _, _, _, _, _) => {
                h3135
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3136, _, _, _, _, _, _, _, _) => {
                h3136
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3137, _, _, _, _, _, _, _) => {
                h3137
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3138, _, _, _, _, _, _) => {
                h3138
            }
        }) {
            true
        } else {
            f191(v4.clone())?
        } {
            0u64
        } else {
            let a3140 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3139, _) => {
                    h3139
                }
            };
            let a3141 = 4u64;
            let a3142 = 0u64;
            nat_quot(a3140, a3141, a3142)
        };
        Ok(Adt15::C1(f198(match v2.clone() {
            Adt14::C0(_, h3143, _, _, _, _, _, _, _, _, _) => {
                h3143
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3144, _, _, _, _, _, _, _, _) => {
                h3144
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3145, _, _, _, _, _, _, _) => {
                h3145
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3146, _, _, _, _, _, _) => {
                h3146
            }
        }, v0, v5)?, 4u64, 65u64, v0, f194(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f103(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f195(v4.clone());
    if if if if if if if v1 {
        let a3148 = match v2.clone() {
            Adt14::C0(h3147, _, _, _, _, _, _, _, _, _, _) => {
                h3147
            }
        };
        let a3149 = 0u64;
        nat_eq(a3148, a3149)
    } else {
        false
    } {
        let a3151 = match v3.clone() {
            Adt14::C0(h3150, _, _, _, _, _, _, _, _, _, _) => {
                h3150
            }
        };
        let a3152 = 0u64;
        nat_eq(a3151, a3152)
    } else {
        false
    } {
        let a3154 = match v4.clone() {
            Adt14::C0(h3153, _, _, _, _, _, _, _, _, _, _) => {
                h3153
            }
        };
        let a3155 = 0u64;
        nat_eq(a3154, a3155)
    } else {
        false
    } {
        f185(v2.clone(), v3.clone())
    } else {
        false
    } {
        f185(v2.clone(), v4)
    } else {
        false
    } {
        if f186(match v2.clone() {
            Adt14::C0(_, h3156, _, _, _, _, _, _, _, _, _) => {
                h3156
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3157, _, _, _, _, _, _, _, _) => {
                h3157
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3158, _, _, _, _, _, _, _) => {
                h3158
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3159, _, _, _, _, _, _) => {
                h3159
            }
        }) {
            true
        } else {
            let a3161 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3160, _, _, _, _, _) => {
                    h3160
                }
            };
            let a3162 = 4u64;
            if if if if if if if nat_eq(a3161, a3162) {
                let a3164 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3163, _, _, _, _, _) => {
                        h3163
                    }
                };
                let a3165 = 4u64;
                nat_eq(a3164, a3165)
            } else {
                false
            } {
                f188(v2.clone())
            } else {
                false
            } {
                f188(v3.clone())
            } else {
                false
            } {
                f188(v5.clone())
            } else {
                false
            } {
                let a3167 = f197(v5.clone())?;
                let a3168 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3166, _) => {
                        h3166
                    }
                };
                nat_le(a3167, a3168)
            } else {
                false
            } {
                if f189(v2.clone(), v5.clone())? {
                    true
                } else {
                    f190(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f189(v3.clone(), v5.clone())? {
                    true
                } else {
                    f190(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f186(match v2.clone() {
            Adt14::C0(_, h3169, _, _, _, _, _, _, _, _, _) => {
                h3169
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3170, _, _, _, _, _, _, _, _) => {
                h3170
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3171, _, _, _, _, _, _, _) => {
                h3171
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3172, _, _, _, _, _, _) => {
                h3172
            }
        }) {
            0u64
        } else {
            let a3178 = match v2.clone() {
                Adt14::C0(_, _, h3173, _, _, _, _, _, _, _, _) => {
                    h3173
                }
            };
            let a3179 = {
                let a3176 = match v2.clone() {
                    Adt14::C0(_, _, _, h3174, _, _, _, _, _, _, _) => {
                        h3174
                    }
                };
                let a3177 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3175, _, _, _, _, _, _) => {
                        h3175
                    }
                };
                nat_mul(a3176, a3177)?
            };
            nat_mul(a3178, a3179)?
        };
        let v7: u64 = {
            let a3180 = v6;
            let a3181 = 0u64;
            if if nat_eq(a3180, a3181) {
                true
            } else {
                f191(v5.clone())?
            } {
                0u64
            } else {
                let a3183 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3182, _) => {
                        h3182
                    }
                };
                let a3184 = 4u64;
                let a3185 = 0u64;
                nat_quot(a3183, a3184, a3185)
            }
        };
        Ok(Adt15::C1(f199(match v2 {
            Adt14::C0(_, h3186, _, _, _, _, _, _, _, _, _) => {
                h3186
            }
        }, v6, v7)?, 4u64, 66u64, v6, f194(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f200(v0, v3, v6) {
        f200(v1, v4, v6)
    } else {
        false
    } {
        f200(v2, v5, v6)
    } else {
        false
    }
}

pub fn f105(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f201(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f201(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f201(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f201(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f201(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f201(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f106(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3187 = v2;
    let a3188 = 0u64;
    if nat_eq(a3187, a3188) {
        Ok(0u64)
    } else {
        let a3191 = 2u64;
        let a3192 = {
            let a3189 = v0;
            let a3190 = v1;
            nat_mul(a3189, a3190)?
        };
        nat_mul(a3191, a3192)
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3193 = v2;
    let a3194 = f181(v0, v1)?;
    nat_mul(a3193, a3194)
}

pub fn f108(v0: u64) -> u64 {
    let a3195 = v0;
    let a3196 = 30u64;
    if nat_eq(a3195, a3196) {
        3u64
    } else {
        1u64
    }
}

pub fn f109(v0: u64) -> R<u64> {
    let a3199 = 5u64;
    let a3200 = {
        let a3197 = 4u64;
        let a3198 = v0;
        nat_mul(a3197, a3198)?
    };
    nat_add(a3199, a3200)
}

pub fn f110(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a3202 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3201, _, _, _, _, _, _) => {
                    h3201
                }
            };
            let a3203 = 1u64;
            if if if if if nat_le(a3202, a3203) {
                true
            } else {
                let a3205 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3204, _, _) => {
                        h3204
                    }
                };
                let a3206 = v1;
                nat_eq(a3205, a3206)
            } {
                let a3208 = match v0.clone() {
                    Adt2::C0(_, h3207, _, _, _, _, _, _, _, _, _) => {
                        h3207
                    }
                };
                let a3209 = 1u64;
                if nat_le(a3208, a3209) {
                    true
                } else {
                    let a3214 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3210, _, _, _, _, _) => {
                            h3210
                        }
                    };
                    let a3215 = {
                        let a3212 = v1;
                        let a3213 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h3211, _, _, _, _, _, _) => {
                                h3211
                            }
                        };
                        nat_mul(a3212, a3213)?
                    };
                    nat_eq(a3214, a3215)
                }
            } else {
                false
            } {
                let a3217 = match v0.clone() {
                    Adt2::C0(_, _, h3216, _, _, _, _, _, _, _, _) => {
                        h3216
                    }
                };
                let a3218 = 1u64;
                if nat_le(a3217, a3218) {
                    true
                } else {
                    let a3226 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3219, _, _, _, _) => {
                            h3219
                        }
                    };
                    let a3227 = {
                        let a3224 = {
                            let a3221 = v1;
                            let a3222 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h3220, _, _, _, _, _, _) => {
                                    h3220
                                }
                            };
                            nat_mul(a3221, a3222)?
                        };
                        let a3225 = match v0.clone() {
                            Adt2::C0(_, h3223, _, _, _, _, _, _, _, _, _) => {
                                h3223
                            }
                        };
                        nat_mul(a3224, a3225)?
                    };
                    nat_eq(a3226, a3227)
                }
            } else {
                false
            } {
                let a3229 = match v0.clone() {
                    Adt2::C0(_, _, _, h3228, _, _, _, _, _, _, _) => {
                        h3228
                    }
                };
                let a3230 = 1u64;
                if nat_le(a3229, a3230) {
                    true
                } else {
                    let a3241 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3231, _, _, _) => {
                            h3231
                        }
                    };
                    let a3242 = {
                        let a3239 = {
                            let a3236 = {
                                let a3233 = v1;
                                let a3234 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3232, _, _, _, _, _, _) => {
                                        h3232
                                    }
                                };
                                nat_mul(a3233, a3234)?
                            };
                            let a3237 = match v0.clone() {
                                Adt2::C0(_, h3235, _, _, _, _, _, _, _, _, _) => {
                                    h3235
                                }
                            };
                            nat_mul(a3236, a3237)?
                        };
                        let a3240 = match v0.clone() {
                            Adt2::C0(_, _, h3238, _, _, _, _, _, _, _, _) => {
                                h3238
                            }
                        };
                        nat_mul(a3239, a3240)?
                    };
                    nat_eq(a3241, a3242)
                }
            } else {
                false
            } {
                let a3256 = {
                    let a3253 = {
                        let a3250 = {
                            let a3247 = {
                                let a3244 = v1;
                                let a3245 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3243, _, _, _, _, _, _) => {
                                        h3243
                                    }
                                };
                                nat_mul(a3244, a3245)?
                            };
                            let a3248 = match v0.clone() {
                                Adt2::C0(_, h3246, _, _, _, _, _, _, _, _, _) => {
                                    h3246
                                }
                            };
                            nat_mul(a3247, a3248)?
                        };
                        let a3251 = match v0.clone() {
                            Adt2::C0(_, _, h3249, _, _, _, _, _, _, _, _) => {
                                h3249
                            }
                        };
                        nat_mul(a3250, a3251)?
                    };
                    let a3254 = match v0.clone() {
                        Adt2::C0(_, _, _, h3252, _, _, _, _, _, _, _) => {
                            h3252
                        }
                    };
                    nat_mul(a3253, a3254)?
                };
                let a3257 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3255, _) => {
                        h3255
                    }
                };
                Ok(nat_le(a3256, a3257))
            } else {
                Ok(false)
            }
        } else {
            let a3259 = match v0.clone() {
                Adt2::C0(_, _, _, h3258, _, _, _, _, _, _, _) => {
                    h3258
                }
            };
            let a3260 = 1u64;
            if if if if if nat_le(a3259, a3260) {
                true
            } else {
                let a3262 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3261, _, _, _) => {
                        h3261
                    }
                };
                let a3263 = v1;
                nat_eq(a3262, a3263)
            } {
                let a3265 = match v0.clone() {
                    Adt2::C0(_, h3264, _, _, _, _, _, _, _, _, _) => {
                        h3264
                    }
                };
                let a3266 = 1u64;
                if nat_le(a3265, a3266) {
                    true
                } else {
                    let a3271 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3267, _, _, _, _, _) => {
                            h3267
                        }
                    };
                    let a3272 = {
                        let a3269 = v1;
                        let a3270 = match v0.clone() {
                            Adt2::C0(_, _, _, h3268, _, _, _, _, _, _, _) => {
                                h3268
                            }
                        };
                        nat_mul(a3269, a3270)?
                    };
                    nat_eq(a3271, a3272)
                }
            } else {
                false
            } {
                let a3274 = match v0.clone() {
                    Adt2::C0(_, _, h3273, _, _, _, _, _, _, _, _) => {
                        h3273
                    }
                };
                let a3275 = 1u64;
                if nat_le(a3274, a3275) {
                    true
                } else {
                    let a3283 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3276, _, _, _, _) => {
                            h3276
                        }
                    };
                    let a3284 = {
                        let a3281 = {
                            let a3278 = v1;
                            let a3279 = match v0.clone() {
                                Adt2::C0(_, _, _, h3277, _, _, _, _, _, _, _) => {
                                    h3277
                                }
                            };
                            nat_mul(a3278, a3279)?
                        };
                        let a3282 = match v0.clone() {
                            Adt2::C0(_, h3280, _, _, _, _, _, _, _, _, _) => {
                                h3280
                            }
                        };
                        nat_mul(a3281, a3282)?
                    };
                    nat_eq(a3283, a3284)
                }
            } else {
                false
            } {
                let a3286 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3285, _, _, _, _, _, _) => {
                        h3285
                    }
                };
                let a3287 = 1u64;
                if nat_le(a3286, a3287) {
                    true
                } else {
                    let a3298 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3288, _, _) => {
                            h3288
                        }
                    };
                    let a3299 = {
                        let a3296 = {
                            let a3293 = {
                                let a3290 = v1;
                                let a3291 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3289, _, _, _, _, _, _, _) => {
                                        h3289
                                    }
                                };
                                nat_mul(a3290, a3291)?
                            };
                            let a3294 = match v0.clone() {
                                Adt2::C0(_, h3292, _, _, _, _, _, _, _, _, _) => {
                                    h3292
                                }
                            };
                            nat_mul(a3293, a3294)?
                        };
                        let a3297 = match v0.clone() {
                            Adt2::C0(_, _, h3295, _, _, _, _, _, _, _, _) => {
                                h3295
                            }
                        };
                        nat_mul(a3296, a3297)?
                    };
                    nat_eq(a3298, a3299)
                }
            } else {
                false
            } {
                let a3313 = {
                    let a3310 = {
                        let a3307 = {
                            let a3304 = {
                                let a3301 = v1;
                                let a3302 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3300, _, _, _, _, _, _, _) => {
                                        h3300
                                    }
                                };
                                nat_mul(a3301, a3302)?
                            };
                            let a3305 = match v0.clone() {
                                Adt2::C0(_, h3303, _, _, _, _, _, _, _, _, _) => {
                                    h3303
                                }
                            };
                            nat_mul(a3304, a3305)?
                        };
                        let a3308 = match v0.clone() {
                            Adt2::C0(_, _, h3306, _, _, _, _, _, _, _, _) => {
                                h3306
                            }
                        };
                        nat_mul(a3307, a3308)?
                    };
                    let a3311 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h3309, _, _, _, _, _, _) => {
                            h3309
                        }
                    };
                    nat_mul(a3310, a3311)?
                };
                let a3314 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3312, _) => {
                        h3312
                    }
                };
                Ok(nat_le(a3313, a3314))
            } else {
                Ok(false)
            }
        }
    } else {
        let a3316 = match v0.clone() {
            Adt2::C0(_, h3315, _, _, _, _, _, _, _, _, _) => {
                h3315
            }
        };
        let a3317 = 1u64;
        if if if if if nat_le(a3316, a3317) {
            true
        } else {
            let a3319 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3318, _, _, _, _, _) => {
                    h3318
                }
            };
            let a3320 = v1;
            nat_eq(a3319, a3320)
        } {
            let a3322 = match v0.clone() {
                Adt2::C0(_, _, h3321, _, _, _, _, _, _, _, _) => {
                    h3321
                }
            };
            let a3323 = 1u64;
            if nat_le(a3322, a3323) {
                true
            } else {
                let a3328 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3324, _, _, _, _) => {
                        h3324
                    }
                };
                let a3329 = {
                    let a3326 = v1;
                    let a3327 = match v0.clone() {
                        Adt2::C0(_, h3325, _, _, _, _, _, _, _, _, _) => {
                            h3325
                        }
                    };
                    nat_mul(a3326, a3327)?
                };
                nat_eq(a3328, a3329)
            }
        } else {
            false
        } {
            let a3331 = match v0.clone() {
                Adt2::C0(_, _, _, h3330, _, _, _, _, _, _, _) => {
                    h3330
                }
            };
            let a3332 = 1u64;
            if nat_le(a3331, a3332) {
                true
            } else {
                let a3340 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3333, _, _, _) => {
                        h3333
                    }
                };
                let a3341 = {
                    let a3338 = {
                        let a3335 = v1;
                        let a3336 = match v0.clone() {
                            Adt2::C0(_, h3334, _, _, _, _, _, _, _, _, _) => {
                                h3334
                            }
                        };
                        nat_mul(a3335, a3336)?
                    };
                    let a3339 = match v0.clone() {
                        Adt2::C0(_, _, h3337, _, _, _, _, _, _, _, _) => {
                            h3337
                        }
                    };
                    nat_mul(a3338, a3339)?
                };
                nat_eq(a3340, a3341)
            }
        } else {
            false
        } {
            let a3343 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3342, _, _, _, _, _, _) => {
                    h3342
                }
            };
            let a3344 = 1u64;
            if nat_le(a3343, a3344) {
                true
            } else {
                let a3355 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3345, _, _) => {
                        h3345
                    }
                };
                let a3356 = {
                    let a3353 = {
                        let a3350 = {
                            let a3347 = v1;
                            let a3348 = match v0.clone() {
                                Adt2::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                                    h3346
                                }
                            };
                            nat_mul(a3347, a3348)?
                        };
                        let a3351 = match v0.clone() {
                            Adt2::C0(_, _, h3349, _, _, _, _, _, _, _, _) => {
                                h3349
                            }
                        };
                        nat_mul(a3350, a3351)?
                    };
                    let a3354 = match v0.clone() {
                        Adt2::C0(_, _, _, h3352, _, _, _, _, _, _, _) => {
                            h3352
                        }
                    };
                    nat_mul(a3353, a3354)?
                };
                nat_eq(a3355, a3356)
            }
        } else {
            false
        } {
            let a3370 = {
                let a3367 = {
                    let a3364 = {
                        let a3361 = {
                            let a3358 = v1;
                            let a3359 = match v0.clone() {
                                Adt2::C0(_, h3357, _, _, _, _, _, _, _, _, _) => {
                                    h3357
                                }
                            };
                            nat_mul(a3358, a3359)?
                        };
                        let a3362 = match v0.clone() {
                            Adt2::C0(_, _, h3360, _, _, _, _, _, _, _, _) => {
                                h3360
                            }
                        };
                        nat_mul(a3361, a3362)?
                    };
                    let a3365 = match v0.clone() {
                        Adt2::C0(_, _, _, h3363, _, _, _, _, _, _, _) => {
                            h3363
                        }
                    };
                    nat_mul(a3364, a3365)?
                };
                let a3368 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3366, _, _, _, _, _, _) => {
                        h3366
                    }
                };
                nat_mul(a3367, a3368)?
            };
            let a3371 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3369, _) => {
                    h3369
                }
            };
            Ok(nat_le(a3370, a3371))
        } else {
            Ok(false)
        }
    }
}

pub fn f111(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a3377 = match v0.clone() {
            Adt2::C0(_, _, h3372, _, _, _, _, _, _, _, _) => {
                h3372
            }
        };
        let a3378 = {
            let a3375 = match v0.clone() {
                Adt2::C0(_, _, _, h3373, _, _, _, _, _, _, _) => {
                    h3373
                }
            };
            let a3376 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3374, _, _, _, _, _, _) => {
                    h3374
                }
            };
            nat_mul(a3375, a3376)?
        };
        nat_mul(a3377, a3378)?
    };
    let v2: u64 = f16(v0.clone(), 1u64)?;
    let a3380 = match v0.clone() {
        Adt2::C0(h3379, _, _, _, _, _, _, _, _, _, _) => {
            h3379
        }
    };
    let a3381 = 0u64;
    if if if if if if if nat_eq(a3380, a3381) {
        let a3383 = match v0.clone() {
            Adt2::C0(_, h3382, _, _, _, _, _, _, _, _, _) => {
                h3382
            }
        };
        let a3384 = 2147483647u64;
        nat_le(a3383, a3384)
    } else {
        false
    } {
        let a3385 = v1;
        let a3386 = 2147483647u64;
        nat_le(a3385, a3386)
    } else {
        false
    } {
        let a3388 = match v0.clone() {
            Adt2::C0(_, h3387, _, _, _, _, _, _, _, _, _) => {
                h3387
            }
        };
        let a3389 = 1u64;
        if nat_eq(a3388, a3389) {
            true
        } else {
            let a3391 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3390, _, _, _, _, _) => {
                    h3390
                }
            };
            let a3392 = 4u64;
            nat_eq(a3391, a3392)
        }
    } else {
        false
    } {
        let a3393 = v1;
        let a3394 = 1u64;
        if nat_le(a3393, a3394) {
            true
        } else {
            let a3399 = {
                let a3396 = match v0.clone() {
                    Adt2::C0(_, h3395, _, _, _, _, _, _, _, _, _) => {
                        h3395
                    }
                };
                let a3397 = 4u64;
                nat_mul(a3396, a3397)?
            };
            let a3400 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3398, _, _, _, _) => {
                    h3398
                }
            };
            nat_le(a3399, a3400)
        }
    } else {
        false
    } {
        let a3402 = match v0.clone() {
            Adt2::C0(_, _, _, h3401, _, _, _, _, _, _, _) => {
                h3401
            }
        };
        let a3403 = 1u64;
        if nat_eq(a3402, a3403) {
            true
        } else {
            let a3409 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h3404, _, _, _) => {
                    h3404
                }
            };
            let a3410 = {
                let a3407 = match v0.clone() {
                    Adt2::C0(_, _, h3405, _, _, _, _, _, _, _, _) => {
                        h3405
                    }
                };
                let a3408 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3406, _, _, _, _) => {
                        h3406
                    }
                };
                nat_mul(a3407, a3408)?
            };
            nat_eq(a3409, a3410)
        }
    } else {
        false
    } {
        let a3412 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3411, _, _, _, _, _, _) => {
                h3411
            }
        };
        let a3413 = 1u64;
        if nat_eq(a3412, a3413) {
            true
        } else {
            let a3422 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h3414, _, _) => {
                    h3414
                }
            };
            let a3423 = {
                let a3420 = match v0.clone() {
                    Adt2::C0(_, _, _, h3415, _, _, _, _, _, _, _) => {
                        h3415
                    }
                };
                let a3421 = {
                    let a3418 = match v0.clone() {
                        Adt2::C0(_, _, h3416, _, _, _, _, _, _, _, _) => {
                            h3416
                        }
                    };
                    let a3419 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3417, _, _, _, _) => {
                            h3417
                        }
                    };
                    nat_mul(a3418, a3419)?
                };
                nat_mul(a3420, a3421)?
            };
            nat_eq(a3422, a3423)
        }
    } else {
        false
    } {
        let a3424 = v2;
        let a3425 = 0u64;
        if nat_eq(a3424, a3425) {
            Ok(true)
        } else {
            let a3431 = {
                let a3428 = f149({
                    let a3426 = v2;
                    let a3427 = 1u64;
                    nat_sub(a3426, a3427)
                }, f20(v0.clone(), 1u64))?;
                let a3429 = 4u64;
                nat_add(a3428, a3429)?
            };
            let a3432 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3430, _) => {
                    h3430
                }
            };
            Ok(nat_le(a3431, a3432))
        }
    } else {
        Ok(false)
    }
}

pub fn f112(v0: Adt2, v1: u64) -> R<bool> {
    let a3434 = match v0.clone() {
        Adt2::C0(h3433, _, _, _, _, _, _, _, _, _, _) => {
            h3433
        }
    };
    let a3435 = 0u64;
    if if nat_eq(a3434, a3435) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a3436 = v1;
        let a3437 = 1u64;
        if nat_le(a3436, a3437) {
            Ok(true)
        } else {
            let a3442 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3438, _, _, _, _) => {
                    h3438
                }
            };
            let a3443 = {
                let a3440 = 4u64;
                let a3441 = match v0 {
                    Adt2::C0(_, h3439, _, _, _, _, _, _, _, _, _) => {
                        h3439
                    }
                };
                nat_mul(a3440, a3441)?
            };
            Ok(nat_eq(a3442, a3443))
        }
    } else {
        Ok(false)
    }
}

pub fn f113(v0: u64) -> R<u64> {
    let a3446 = 2u64;
    let a3447 = {
        let a3444 = 6u64;
        let a3445 = v0;
        nat_mul(a3444, a3445)?
    };
    nat_add(a3446, a3447)
}

pub fn f114(v0: u64) -> bool {
    let a3448 = v0;
    let a3449 = 0u64;
    if if nat_eq(a3448, a3449) {
        true
    } else {
        let a3450 = v0;
        let a3451 = 1u64;
        nat_eq(a3450, a3451)
    } {
        true
    } else {
        let a3452 = v0;
        let a3453 = 30u64;
        nat_eq(a3452, a3453)
    }
}

pub fn f115(v0: Adt2, v1: u64) -> R<bool> {
    let a3455 = match v0.clone() {
        Adt2::C0(_, _, _, h3454, _, _, _, _, _, _, _) => {
            h3454
        }
    };
    let a3456 = 1u64;
    if if if nat_eq(a3455, a3456) {
        let a3458 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3457, _, _, _, _, _, _) => {
                h3457
            }
        };
        let a3459 = 1u64;
        nat_eq(a3458, a3459)
    } else {
        false
    } {
        let a3461 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h3460, _, _, _, _, _) => {
                h3460
            }
        };
        let a3462 = v1;
        nat_eq(a3461, a3462)
    } else {
        false
    } {
        let a3464 = match v0.clone() {
            Adt2::C0(_, _, h3463, _, _, _, _, _, _, _, _) => {
                h3463
            }
        };
        let a3465 = 1u64;
        if nat_eq(a3464, a3465) {
            Ok(true)
        } else {
            let a3470 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3466, _, _, _, _) => {
                    h3466
                }
            };
            let a3471 = {
                let a3468 = match v0 {
                    Adt2::C0(_, h3467, _, _, _, _, _, _, _, _, _) => {
                        h3467
                    }
                };
                let a3469 = v1;
                nat_mul(a3468, a3469)?
            };
            Ok(nat_eq(a3470, a3471))
        }
    } else {
        Ok(false)
    }
}

pub fn f116(v0: Adt6) -> R<u64> {
    let a3484 = 2u64;
    let a3485 = {
        let a3482 = 2u64;
        let a3483 = {
            let a3479 = {
                let a3476 = match v0.clone() {
                    Adt6::C0(h3472, _, _, _) => {
                        h3472
                    }
                };
                let a3477 = {
                    let a3474 = match v0.clone() {
                        Adt6::C0(_, h3473, _, _) => {
                            h3473
                        }
                    };
                    let a3475 = 1u64;
                    nat_sub(a3474, a3475)
                };
                nat_add(a3476, a3477)?
            };
            let a3480 = match v0 {
                Adt6::C0(_, h3478, _, _) => {
                    h3478
                }
            };
            let a3481 = 0u64;
            nat_quot(a3479, a3480, a3481)
        };
        nat_mul(a3482, a3483)?
    };
    nat_add(a3484, a3485)
}

pub fn f117(v0: u64) -> R<u64> {
    let a3488 = 2u64;
    let a3489 = {
        let a3486 = v0;
        let a3487 = 2u64;
        nat_mul(a3486, a3487)?
    };
    nat_add(a3488, a3489)
}

pub fn f118(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3493 = {
            let a3490 = v2;
            let a3491 = 1u64;
            nat_sub(a3490, a3491)
        };
        let a3494 = match v0.clone() {
            Adt6::C0(_, h3492, _, _) => {
                h3492
            }
        };
        nat_mul(a3493, a3494)?
    };
    let a3496 = 0u64;
    let a3497 = match v0.clone() {
        Adt6::C0(h3495, _, _, _) => {
            h3495
        }
    };
    if if if if if if if if nat_lt(a3496, a3497) {
        let a3499 = match v0.clone() {
            Adt6::C0(h3498, _, _, _) => {
                h3498
            }
        };
        let a3500 = 2147483648u64;
        nat_lt(a3499, a3500)
    } else {
        false
    } {
        let a3502 = 0u64;
        let a3503 = match v0.clone() {
            Adt6::C0(_, h3501, _, _) => {
                h3501
            }
        };
        nat_lt(a3502, a3503)
    } else {
        false
    } {
        let a3505 = match v0.clone() {
            Adt6::C0(_, h3504, _, _) => {
                h3504
            }
        };
        let a3506 = 2147483648u64;
        nat_lt(a3505, a3506)
    } else {
        false
    } {
        let a3507 = v1;
        let a3508 = 2147483648u64;
        nat_lt(a3507, a3508)
    } else {
        false
    } {
        let a3509 = 0u64;
        let a3510 = v2;
        nat_lt(a3509, a3510)
    } else {
        false
    } {
        let a3511 = v3;
        let a3512 = 2147483648u64;
        nat_lt(a3511, a3512)
    } else {
        false
    } {
        let a3524 = {
            let a3519 = {
                let a3516 = v3;
                let a3517 = {
                    let a3514 = match v0.clone() {
                        Adt6::C0(h3513, _, _, _) => {
                            h3513
                        }
                    };
                    let a3515 = 1u64;
                    nat_sub(a3514, a3515)
                };
                nat_add(a3516, a3517)?
            };
            let a3520 = f142(match v0.clone() {
                Adt6::C0(_, _, h3518, _) => {
                    h3518
                }
            });
            nat_add(a3519, a3520)?
        };
        let a3525 = {
            let a3522 = 2147483647u64;
            let a3523 = f143(match v0.clone() {
                Adt6::C0(_, _, h3521, _) => {
                    h3521
                }
            });
            nat_add(a3522, a3523)?
        };
        nat_le(a3524, a3525)
    } else {
        false
    } {
        let a3527 = match v0 {
            Adt6::C0(_, _, h3526, _) => {
                h3526
            }
        };
        let a3528 = 2147483648u64;
        Ok(!nat_eq(a3527, a3528))
    } else {
        Ok(false)
    }
}

pub fn f119(v0: Adt7) -> R<u64> {
    let a3540 = if match v0.clone() {
        Adt7::C0(_, _, _, h3529, _) => {
            h3529
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3541 = {
        let a3538 = {
            let a3536 = {
                let m3532 = match v0.clone() {
                    Adt7::C0(h3531, _, _, _, _) => {
                        h3531
                    }
                };
                match m3532 {
                    Adt6::C0(h3530, _, _, _) => {
                        h3530
                    }
                }
            };
            let a3537 = {
                let m3535 = match v0.clone() {
                    Adt7::C0(_, h3534, _, _, _) => {
                        h3534
                    }
                };
                match m3535 {
                    Adt6::C0(h3533, _, _, _) => {
                        h3533
                    }
                }
            };
            nat_mul(a3536, a3537)?
        };
        let a3539 = f176(v0)?;
        nat_mul(a3538, a3539)?
    };
    nat_add(a3540, a3541)
}

pub fn f120(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f202(match v0.clone() {
        Adt2::C0(h3542, _, _, _, _, _, _, _, _, _, _) => {
            h3542
        }
    }, match v0.clone() {
        Adt2::C0(_, h3543, _, _, _, _, _, _, _, _, _) => {
            h3543
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3544, _, _, _, _, _, _, _, _) => {
            h3544
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3545, _, _, _, _, _, _, _) => {
            h3545
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3546, _, _, _, _, _, _) => {
            h3546
        }
    }, match v1.clone() {
        Adt2::C0(h3547, _, _, _, _, _, _, _, _, _, _) => {
            h3547
        }
    }, match v1.clone() {
        Adt2::C0(_, h3548, _, _, _, _, _, _, _, _, _) => {
            h3548
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3549, _, _, _, _, _, _, _, _) => {
            h3549
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3550, _, _, _, _, _, _, _) => {
            h3550
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3551, _, _, _, _, _, _) => {
            h3551
        }
    }, match v2.clone() {
        Adt2::C0(h3552, _, _, _, _, _, _, _, _, _, _) => {
            h3552
        }
    }, match v2.clone() {
        Adt2::C0(_, h3553, _, _, _, _, _, _, _, _, _) => {
            h3553
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3554, _, _, _, _, _, _, _, _) => {
            h3554
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3555, _, _, _, _, _, _, _) => {
            h3555
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3556, _, _, _, _, _, _) => {
            h3556
        }
    }) {
        let m3558 = f124(match v0.clone() {
            Adt2::C0(h3557, _, _, _, _, _, _, _, _, _, _) => {
                h3557
            }
        });
        match m3558 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3564 = {
                    let a3561 = match v0.clone() {
                        Adt2::C0(_, h3559, _, _, _, _, _, _, _, _, _) => {
                            h3559
                        }
                    };
                    let a3562 = match v3.clone() {
                        Adt18::C0(_, h3560, _) => {
                            h3560
                        }
                    };
                    let a3563 = 0u64;
                    nat_rem(a3561, a3562, a3563)
                };
                let a3565 = 0u64;
                if if nat_eq(a3564, a3565) {
                    if f203(match v0.clone() {
                        Adt2::C0(_, h3566, _, _, _, _, _, _, _, _, _) => {
                            h3566
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3567, _, _, _, _, _, _, _, _, _) => {
                            h3567
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3568, _, _, _, _, _, _, _, _) => {
                            h3568
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3569, _, _, _, _, _, _, _) => {
                            h3569
                        }
                    }) {
                        true
                    } else {
                        let a3571 = 4u64;
                        let a3572 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3570, _, _, _, _, _) => {
                                h3570
                            }
                        };
                        if if nat_le(a3571, a3572) {
                            let a3574 = match v3.clone() {
                                Adt18::C0(_, h3573, _) => {
                                    h3573
                                }
                            };
                            let a3575 = 1u64;
                            if nat_eq(a3574, a3575) {
                                true
                            } else {
                                let a3578 = match v3.clone() {
                                    Adt18::C0(_, _, h3576) => {
                                        h3576
                                    }
                                };
                                let a3579 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3577, _, _, _, _, _) => {
                                        h3577
                                    }
                                };
                                nat_le(a3578, a3579)
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
                        Adt18::C0(_, h3580, _) => {
                            h3580
                        }
                    });
                    Ok(Adt10::C4(if f203(match v0.clone() {
                        Adt2::C0(_, h3581, _, _, _, _, _, _, _, _, _) => {
                            h3581
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3582, _, _, _, _, _, _, _, _, _) => {
                            h3582
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3583, _, _, _, _, _, _, _, _) => {
                            h3583
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3584, _, _, _, _, _, _, _) => {
                            h3584
                        }
                    }) {
                        0u64
                    } else {
                        f170(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3585, _, _, _, _, _, _, _, _, _, _) => {
                            h3585
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

pub fn f121(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f122(match v0.clone() {
        Adt2::C0(h3586, _, _, _, _, _, _, _, _, _, _) => {
            h3586
        }
    }) {
        let a3592 = {
            let a3589 = match v0.clone() {
                Adt2::C0(_, h3587, _, _, _, _, _, _, _, _, _) => {
                    h3587
                }
            };
            let a3590 = f204(match v0.clone() {
                Adt2::C0(h3588, _, _, _, _, _, _, _, _, _, _) => {
                    h3588
                }
            });
            let a3591 = 1u64;
            nat_rem(a3589, a3590, a3591)
        };
        let a3593 = 0u64;
        nat_eq(a3592, a3593)
    } else {
        false
    } {
        let a3595 = match v1.clone() {
            Adt2::C0(h3594, _, _, _, _, _, _, _, _, _, _) => {
                h3594
            }
        };
        let a3596 = 26u64;
        nat_eq(a3595, a3596)
    } else {
        false
    } {
        let a3598 = match v2.clone() {
            Adt2::C0(h3597, _, _, _, _, _, _, _, _, _, _) => {
                h3597
            }
        };
        let a3599 = 0u64;
        nat_eq(a3598, a3599)
    } else {
        false
    } {
        let a3602 = match v0.clone() {
            Adt2::C0(_, h3600, _, _, _, _, _, _, _, _, _) => {
                h3600
            }
        };
        let a3603 = match v2.clone() {
            Adt2::C0(_, h3601, _, _, _, _, _, _, _, _, _) => {
                h3601
            }
        };
        nat_eq(a3602, a3603)
    } else {
        false
    } {
        let a3606 = match v1.clone() {
            Adt2::C0(_, h3604, _, _, _, _, _, _, _, _, _) => {
                h3604
            }
        };
        let a3607 = match v0.clone() {
            Adt2::C0(_, _, h3605, _, _, _, _, _, _, _, _) => {
                h3605
            }
        };
        nat_le(a3606, a3607)
    } else {
        false
    } {
        let a3609 = match v0.clone() {
            Adt2::C0(_, _, _, h3608, _, _, _, _, _, _, _) => {
                h3608
            }
        };
        let a3610 = 1u64;
        nat_eq(a3609, a3610)
    } else {
        false
    } {
        let a3612 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3611, _, _, _, _, _, _) => {
                h3611
            }
        };
        let a3613 = 1u64;
        nat_eq(a3612, a3613)
    } else {
        false
    } {
        let a3615 = match v2.clone() {
            Adt2::C0(_, _, _, h3614, _, _, _, _, _, _, _) => {
                h3614
            }
        };
        let a3616 = 1u64;
        nat_eq(a3615, a3616)
    } else {
        false
    } {
        let a3618 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3617, _, _, _, _, _, _) => {
                h3617
            }
        };
        let a3619 = 1u64;
        nat_eq(a3618, a3619)
    } else {
        false
    } {
        let a3621 = match v1.clone() {
            Adt2::C0(_, _, h3620, _, _, _, _, _, _, _, _) => {
                h3620
            }
        };
        let a3622 = 1u64;
        nat_eq(a3621, a3622)
    } else {
        false
    } {
        let a3624 = match v1.clone() {
            Adt2::C0(_, _, _, h3623, _, _, _, _, _, _, _) => {
                h3623
            }
        };
        let a3625 = 1u64;
        nat_eq(a3624, a3625)
    } else {
        false
    } {
        let a3627 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3626, _, _, _, _, _, _) => {
                h3626
            }
        };
        let a3628 = 1u64;
        nat_eq(a3627, a3628)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3629, _, _, _, _, _, _, _, _, _) => {
                h3629
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3630, _, _, _, _, _, _, _, _) => {
                h3630
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3631, _, _, _, _, _, _, _) => {
                h3631
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3632, _, _, _, _, _, _) => {
                h3632
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3634 = match v1.clone() {
                Adt2::C0(_, h3633, _, _, _, _, _, _, _, _, _) => {
                    h3633
                }
            };
            let a3635 = 0u64;
            if nat_eq(a3634, a3635) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3637 = match v0.clone() {
                    Adt2::C0(_, h3636, _, _, _, _, _, _, _, _, _) => {
                        h3636
                    }
                };
                let a3638 = 2147483647u64;
                if if if if if if nat_le(a3637, a3638) {
                    let a3640 = match v1.clone() {
                        Adt2::C0(_, h3639, _, _, _, _, _, _, _, _, _) => {
                            h3639
                        }
                    };
                    let a3641 = 2147483647u64;
                    nat_le(a3640, a3641)
                } else {
                    false
                } {
                    let a3643 = 4u64;
                    let a3644 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3642, _, _, _, _, _) => {
                            h3642
                        }
                    };
                    nat_le(a3643, a3644)
                } else {
                    false
                } {
                    let a3650 = {
                        let a3647 = match v2.clone() {
                            Adt2::C0(_, h3645, _, _, _, _, _, _, _, _, _) => {
                                h3645
                            }
                        };
                        let a3648 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3646, _, _, _, _, _) => {
                                h3646
                            }
                        };
                        nat_mul(a3647, a3648)?
                    };
                    let a3651 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3649, _, _, _, _) => {
                            h3649
                        }
                    };
                    nat_le(a3650, a3651)
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
                        let a3664 = {
                            let a3655 = {
                                let a3653 = 2u64;
                                let a3654 = match v1.clone() {
                                    Adt2::C0(_, h3652, _, _, _, _, _, _, _, _, _) => {
                                        h3652
                                    }
                                };
                                nat_mul(a3653, a3654)?
                            };
                            let a3656 = f16(v2.clone(), 1u64)?;
                            nat_add(a3655, a3656)?
                        };
                        let a3665 = {
                            let a3662 = match v1.clone() {
                                Adt2::C0(_, h3657, _, _, _, _, _, _, _, _, _) => {
                                    h3657
                                }
                            };
                            let a3663 = {
                                let a3660 = 1u64;
                                let a3661 = f168(match v0.clone() {
                                    Adt2::C0(h3658, _, _, _, _, _, _, _, _, _, _) => {
                                        h3658
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3659, _, _, _, _, _, _, _, _, _) => {
                                        h3659
                                    }
                                })?;
                                nat_add(a3660, a3661)?
                            };
                            nat_mul(a3662, a3663)?
                        };
                        nat_add(a3664, a3665)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3666, _, _, _, _, _, _, _, _, _) => {
                            h3666
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3667, _, _, _, _, _, _, _, _, _) => {
                            h3667
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3668, _, _, _, _, _) => {
                            h3668
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3669, _, _, _, _) => {
                            h3669
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3670, _, _, _) => {
                            h3670
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3671, _, _) => {
                            h3671
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3672, _, _, _, _, _, _, _, _, _, _) => {
                            h3672
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

pub fn f122(v0: u64) -> bool {
    let a3673 = v0;
    let a3674 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3673, a3674) {
        true
    } else {
        let a3675 = v0;
        let a3676 = 1u64;
        nat_eq(a3675, a3676)
    } {
        true
    } else {
        let a3677 = v0;
        let a3678 = 2u64;
        nat_eq(a3677, a3678)
    } {
        true
    } else {
        let a3679 = v0;
        let a3680 = 3u64;
        nat_eq(a3679, a3680)
    } {
        true
    } else {
        let a3681 = v0;
        let a3682 = 6u64;
        nat_eq(a3681, a3682)
    } {
        true
    } else {
        let a3683 = v0;
        let a3684 = 7u64;
        nat_eq(a3683, a3684)
    } {
        true
    } else {
        let a3685 = v0;
        let a3686 = 8u64;
        nat_eq(a3685, a3686)
    } {
        true
    } else {
        let a3687 = v0;
        let a3688 = 10u64;
        nat_eq(a3687, a3688)
    } {
        true
    } else {
        let a3689 = v0;
        let a3690 = 11u64;
        nat_eq(a3689, a3690)
    } {
        true
    } else {
        let a3691 = v0;
        let a3692 = 12u64;
        nat_eq(a3691, a3692)
    } {
        true
    } else {
        let a3693 = v0;
        let a3694 = 13u64;
        nat_eq(a3693, a3694)
    } {
        true
    } else {
        let a3695 = v0;
        let a3696 = 14u64;
        nat_eq(a3695, a3696)
    } {
        true
    } else {
        let a3697 = v0;
        let a3698 = 16u64;
        nat_eq(a3697, a3698)
    } {
        true
    } else {
        let a3699 = v0;
        let a3700 = 17u64;
        nat_eq(a3699, a3700)
    } {
        true
    } else {
        let a3701 = v0;
        let a3702 = 18u64;
        nat_eq(a3701, a3702)
    } {
        true
    } else {
        let a3703 = v0;
        let a3704 = 19u64;
        nat_eq(a3703, a3704)
    } {
        true
    } else {
        let a3705 = v0;
        let a3706 = 20u64;
        nat_eq(a3705, a3706)
    } {
        true
    } else {
        let a3707 = v0;
        let a3708 = 21u64;
        nat_eq(a3707, a3708)
    } {
        true
    } else {
        let a3709 = v0;
        let a3710 = 22u64;
        nat_eq(a3709, a3710)
    } {
        true
    } else {
        let a3711 = v0;
        let a3712 = 23u64;
        nat_eq(a3711, a3712)
    } {
        true
    } else {
        let a3713 = v0;
        let a3714 = 26u64;
        nat_eq(a3713, a3714)
    } {
        true
    } else {
        let a3715 = v0;
        let a3716 = 29u64;
        nat_eq(a3715, a3716)
    } {
        true
    } else {
        let a3717 = v0;
        let a3718 = 30u64;
        nat_eq(a3717, a3718)
    } {
        true
    } else {
        let a3719 = v0;
        let a3720 = 34u64;
        nat_eq(a3719, a3720)
    } {
        true
    } else {
        let a3721 = v0;
        let a3722 = 35u64;
        nat_eq(a3721, a3722)
    } {
        true
    } else {
        let a3723 = v0;
        let a3724 = 39u64;
        nat_eq(a3723, a3724)
    } {
        true
    } else {
        let a3725 = v0;
        let a3726 = 40u64;
        nat_eq(a3725, a3726)
    } {
        true
    } else {
        let a3727 = v0;
        let a3728 = 41u64;
        nat_eq(a3727, a3728)
    } {
        true
    } else {
        let a3729 = v0;
        let a3730 = 42u64;
        nat_eq(a3729, a3730)
    } {
        true
    } else {
        let a3731 = v0;
        let a3732 = 43u64;
        nat_eq(a3731, a3732)
    }
}

pub fn f123(v0: u64) -> bool {
    if f114(v0) {
        true
    } else {
        let a3733 = v0;
        let a3734 = 43u64;
        nat_eq(a3733, a3734)
    }
}

pub fn f124(v0: u64) -> Adt17 {
    let a3735 = v0;
    let a3736 = 0u64;
    if nat_eq(a3735, a3736) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3737 = v0;
        let a3738 = 1u64;
        if nat_eq(a3737, a3738) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3739 = v0;
            let a3740 = 2u64;
            if nat_eq(a3739, a3740) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3741 = v0;
                let a3742 = 3u64;
                if nat_eq(a3741, a3742) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3743 = v0;
                    let a3744 = 6u64;
                    if nat_eq(a3743, a3744) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3745 = v0;
                        let a3746 = 7u64;
                        if nat_eq(a3745, a3746) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3747 = v0;
                            let a3748 = 8u64;
                            if nat_eq(a3747, a3748) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3749 = v0;
                                let a3750 = 9u64;
                                if nat_eq(a3749, a3750) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3751 = v0;
                                    let a3752 = 10u64;
                                    if nat_eq(a3751, a3752) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3753 = v0;
                                        let a3754 = 11u64;
                                        if nat_eq(a3753, a3754) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3755 = v0;
                                            let a3756 = 12u64;
                                            if nat_eq(a3755, a3756) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3757 = v0;
                                                let a3758 = 13u64;
                                                if nat_eq(a3757, a3758) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3759 = v0;
                                                    let a3760 = 14u64;
                                                    if nat_eq(a3759, a3760) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3761 = v0;
                                                        let a3762 = 15u64;
                                                        if nat_eq(a3761, a3762) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3763 = v0;
                                                            let a3764 = 16u64;
                                                            if nat_eq(a3763, a3764) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3765 = v0;
                                                                let a3766 = 17u64;
                                                                if nat_eq(a3765, a3766) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3767 = v0;
                                                                    let a3768 = 18u64;
                                                                    if nat_eq(a3767, a3768) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3769 = v0;
                                                                        let a3770 = 19u64;
                                                                        if nat_eq(a3769, a3770) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3771 = v0;
                                                                            let a3772 = 20u64;
                                                                            if nat_eq(a3771, a3772) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3773 = v0;
                                                                                let a3774 = 21u64;
                                                                                if nat_eq(a3773, a3774) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3775 = v0;
                                                                                    let a3776 = 22u64;
                                                                                    if nat_eq(a3775, a3776) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3777 = v0;
                                                                                        let a3778 = 23u64;
                                                                                        if nat_eq(a3777, a3778) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3779 = v0;
                                                                                            let a3780 = 24u64;
                                                                                            if nat_eq(a3779, a3780) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3781 = v0;
                                                                                                let a3782 = 25u64;
                                                                                                if nat_eq(a3781, a3782) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3783 = v0;
                                                                                                    let a3784 = 26u64;
                                                                                                    if nat_eq(a3783, a3784) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3785 = v0;
                                                                                                        let a3786 = 27u64;
                                                                                                        if nat_eq(a3785, a3786) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3787 = v0;
                                                                                                            let a3788 = 28u64;
                                                                                                            if nat_eq(a3787, a3788) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3789 = v0;
                                                                                                                let a3790 = 29u64;
                                                                                                                if nat_eq(a3789, a3790) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3791 = v0;
                                                                                                                    let a3792 = 30u64;
                                                                                                                    if nat_eq(a3791, a3792) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3793 = v0;
                                                                                                                        let a3794 = 34u64;
                                                                                                                        if nat_eq(a3793, a3794) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3795 = v0;
                                                                                                                            let a3796 = 35u64;
                                                                                                                            if nat_eq(a3795, a3796) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3797 = v0;
                                                                                                                                let a3798 = 39u64;
                                                                                                                                if nat_eq(a3797, a3798) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3799 = v0;
                                                                                                                                    let a3800 = 40u64;
                                                                                                                                    if nat_eq(a3799, a3800) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3801 = v0;
                                                                                                                                        let a3802 = 41u64;
                                                                                                                                        if nat_eq(a3801, a3802) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3803 = v0;
                                                                                                                                            let a3804 = 42u64;
                                                                                                                                            if nat_eq(a3803, a3804) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3805 = v0;
                                                                                                                                                let a3806 = 43u64;
                                                                                                                                                if nat_eq(a3805, a3806) {
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

pub fn f125(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3809 = match v1.clone() {
        Adt2::C0(h3807, _, _, _, _, _, _, _, _, _, _) => {
            h3807
        }
    };
    let a3810 = match v3.clone() {
        Adt2::C0(h3808, _, _, _, _, _, _, _, _, _, _) => {
            h3808
        }
    };
    if if nat_eq(a3809, a3810) {
        let a3811 = v0;
        let a3812 = 33u64;
        if nat_eq(a3811, a3812) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3814 = f124(match v1.clone() {
            Adt2::C0(h3813, _, _, _, _, _, _, _, _, _, _) => {
                h3813
            }
        });
        match m3814 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3815, _) => {
                        h3815
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3820 = {
                    let a3817 = match v1.clone() {
                        Adt2::C0(_, h3816, _, _, _, _, _, _, _, _, _) => {
                            h3816
                        }
                    };
                    let a3818 = v5;
                    let a3819 = 1u64;
                    nat_rem(a3817, a3818, a3819)
                };
                let a3821 = 0u64;
                if if if nat_eq(a3820, a3821) {
                    let a3826 = {
                        let a3823 = match v3.clone() {
                            Adt2::C0(_, h3822, _, _, _, _, _, _, _, _, _) => {
                                h3822
                            }
                        };
                        let a3824 = v5;
                        let a3825 = 1u64;
                        nat_rem(a3823, a3824, a3825)
                    };
                    let a3827 = 0u64;
                    nat_eq(a3826, a3827)
                } else {
                    false
                } {
                    let a3828 = v6;
                    let a3829 = f16(v3.clone(), v5)?;
                    nat_eq(a3828, a3829)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3830) => {
                            h3830
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

pub fn f126(v0: u64, v1: u64) -> u64 {
    let a3831 = v0;
    let a3832 = 0u64;
    if if nat_eq(a3831, a3832) {
        true
    } else {
        let a3833 = v1;
        let a3834 = 0u64;
        nat_eq(a3833, a3834)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3838 = {
            let a3835 = v0;
            let a3836 = v4;
            let a3837 = 1u64;
            nat_rem(a3835, a3836, a3837)
        };
        let a3839 = 0u64;
        if if if nat_eq(a3838, a3839) {
            let a3843 = {
                let a3840 = v1;
                let a3841 = v5;
                let a3842 = 1u64;
                nat_rem(a3840, a3841, a3842)
            };
            let a3844 = 0u64;
            nat_eq(a3843, a3844)
        } else {
            false
        } {
            let a3848 = {
                let a3845 = v2;
                let a3846 = v6;
                let a3847 = 1u64;
                nat_rem(a3845, a3846, a3847)
            };
            let a3849 = 0u64;
            nat_eq(a3848, a3849)
        } else {
            false
        } {
            let a3853 = {
                let a3850 = v3;
                let a3851 = v7;
                let a3852 = 1u64;
                nat_rem(a3850, a3851, a3852)
            };
            let a3854 = 0u64;
            nat_eq(a3853, a3854)
        } else {
            false
        }
    }
}

pub fn f128(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3855 = v0;
    let a3856 = v1;
    if nat_eq(a3855, a3856) {
        let a3859 = v4;
        let a3860 = {
            let a3857 = v2;
            let a3858 = v3;
            nat_add(a3857, a3858)?
        };
        Ok(nat_eq(a3859, a3860))
    } else {
        let a3861 = v2;
        let a3862 = v3;
        if nat_eq(a3861, a3862) {
            let a3863 = v4;
            let a3864 = v2;
            Ok(nat_eq(a3863, a3864))
        } else {
            Ok(false)
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3867 = {
            let a3865 = v0;
            let a3866 = f143(v1);
            nat_add(a3865, a3866)?
        };
        let a3868 = f143(v2);
        nat_add(a3867, a3868)?
    };
    let v5: u64 = {
        let a3869 = f142(v1);
        let a3870 = f142(v2);
        nat_add(a3869, a3870)?
    };
    let a3871 = v1;
    let a3872 = 4294967295u64;
    if if nat_le(a3871, a3872) {
        let a3873 = v2;
        let a3874 = 4294967295u64;
        nat_le(a3873, a3874)
    } else {
        false
    } {
        let a3875 = v5;
        let a3876 = v4;
        if nat_le(a3875, a3876) {
            let a3879 = v3;
            let a3880 = {
                let a3877 = v4;
                let a3878 = v5;
                nat_sub(a3877, a3878)
            };
            Ok(nat_eq(a3879, a3880))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f130(v0: u64, v1: u64) -> R<bool> {
    let a3881 = v1;
    let a3882 = 4294967295u64;
    if nat_le(a3881, a3882) {
        let a3885 = {
            let a3883 = f143(v1);
            let a3884 = f142(v1);
            nat_add(a3883, a3884)?
        };
        let a3886 = v0;
        Ok(nat_lt(a3885, a3886))
    } else {
        Ok(false)
    }
}

pub fn f131(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f205(v0.clone(), false)?;
    let v6: u64 = f205(v0, true)?;
    let a3887 = v6;
    let a3888 = v1;
    if if nat_le(a3887, a3888) {
        let a3895 = {
            let a3893 = {
                let a3889 = v1;
                let a3890 = v5;
                nat_add(a3889, a3890)?
            };
            let a3894 = {
                let a3891 = v3;
                let a3892 = 0u64;
                if nat_eq(a3891, a3892) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3893, a3894)?
        };
        let a3896 = v2;
        nat_le(a3895, a3896)
    } else {
        false
    } {
        if v4 {
            let a3901 = {
                let a3899 = {
                    let a3897 = v1;
                    let a3898 = v5;
                    nat_add(a3897, a3898)?
                };
                let a3900 = v6;
                nat_sub(a3899, a3900)
            };
            let a3902 = v2;
            Ok(nat_lt(a3901, a3902))
        } else {
            let a3907 = {
                let a3905 = {
                    let a3903 = v1;
                    let a3904 = v5;
                    nat_add(a3903, a3904)?
                };
                let a3906 = v6;
                nat_sub(a3905, a3906)
            };
            let a3908 = v2;
            Ok(nat_le(a3907, a3908))
        }
    } else {
        Ok(false)
    }
}

pub fn f132(v0: u64, v1: u64) -> bool {
    let a3909 = v0;
    let a3910 = 0u64;
    if if nat_eq(a3909, a3910) {
        true
    } else {
        let a3911 = v0;
        let a3912 = 1u64;
        nat_eq(a3911, a3912)
    } {
        let a3913 = v1;
        let a3914 = 4294967296u64;
        nat_lt(a3913, a3914)
    } else {
        false
    }
}

pub fn f133(v0: u64, v1: u64) -> R<u64> {
    let a3919 = {
        let a3917 = v0;
        let a3918 = {
            let a3915 = v1;
            let a3916 = 1u64;
            nat_sub(a3915, a3916)
        };
        nat_add(a3917, a3918)?
    };
    let a3920 = v1;
    let a3921 = 0u64;
    Ok(nat_quot(a3919, a3920, a3921))
}

pub fn f134(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3922, _, _, _, _, _, _, _, _, _) => {
            h3922
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3923, _, _, _, _, _, _, _, _) => {
            h3923
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3924, _, _, _, _, _, _, _) => {
            h3924
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3925, _, _, _, _, _, _) => {
            h3925
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3928 = match v1.clone() {
                Adt2::C0(_, h3926, _, _, _, _, _, _, _, _, _) => {
                    h3926
                }
            };
            let a3929 = match v1.clone() {
                Adt2::C0(_, _, h3927, _, _, _, _, _, _, _, _) => {
                    h3927
                }
            };
            nat_mul(a3928, a3929)?
        };
        let v5: u64 = {
            let a3931 = v4;
            let a3932 = match v1.clone() {
                Adt2::C0(_, _, _, h3930, _, _, _, _, _, _, _) => {
                    h3930
                }
            };
            nat_mul(a3931, a3932)?
        };
        let a3945 = {
            let a3943 = {
                let a3934 = match v0.clone() {
                    Adt2::C0(_, h3933, _, _, _, _, _, _, _, _, _) => {
                        h3933
                    }
                };
                let a3935 = 1u64;
                nat_sub(a3934, a3935)
            };
            let a3944 = {
                let a3941 = f206(match v0.clone() {
                    Adt2::C0(_, _, h3936, _, _, _, _, _, _, _, _) => {
                        h3936
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3937, _, _, _, _, _, _, _, _, _) => {
                        h3937
                    }
                })?;
                let a3942 = f206(match v0 {
                    Adt2::C0(_, _, _, h3938, _, _, _, _, _, _, _) => {
                        h3938
                    }
                }, v2, {
                    let a3939 = v3;
                    let a3940 = v5;
                    nat_mul(a3939, a3940)?
                }, v4)?;
                nat_add(a3941, a3942)?
            };
            nat_add(a3943, a3944)?
        };
        let a3946 = f16(v1, 1u64)?;
        Ok(nat_lt(a3945, a3946))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3947 = v0;
    let a3948 = 2147483648u64;
    if if if if nat_lt(a3947, a3948) {
        let a3949 = v1;
        let a3950 = 2147483648u64;
        nat_lt(a3949, a3950)
    } else {
        false
    } {
        let a3953 = {
            let a3951 = v0;
            let a3952 = v1;
            nat_mul(a3951, a3952)?
        };
        let a3954 = 2147483648u64;
        nat_lt(a3953, a3954)
    } else {
        false
    } {
        let a3959 = {
            let a3957 = {
                let a3955 = v0;
                let a3956 = 1u64;
                nat_sub(a3955, a3956)
            };
            let a3958 = v2;
            nat_mul(a3957, a3958)?
        };
        let a3960 = 2147483648u64;
        nat_lt(a3959, a3960)
    } else {
        false
    } {
        let a3965 = {
            let a3963 = {
                let a3961 = v1;
                let a3962 = 1u64;
                nat_sub(a3961, a3962)
            };
            let a3964 = v2;
            nat_mul(a3963, a3964)?
        };
        let a3966 = 2147483648u64;
        Ok(nat_lt(a3965, a3966))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64) -> R<u64> {
    let a3969 = 3u64;
    let a3970 = {
        let a3967 = v0;
        let a3968 = 3u64;
        nat_mul(a3967, a3968)?
    };
    nat_add(a3969, a3970)
}

pub fn f137(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3971, _, _, _, _, _, _, _) => {
            h3971
        }
    }, match v0.clone() {
        Adt3::C0(_, h3972, _, _, _, _, _, _) => {
            h3972
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3973, _, _, _, _, _) => {
            h3973
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3974, _, _, _, _) => {
            h3974
        }
    }) {
        Ok(0u64)
    } else {
        let a3983 = match v0.clone() {
            Adt3::C0(h3975, _, _, _, _, _, _, _) => {
                h3975
            }
        };
        let a3984 = {
            let a3981 = match v0.clone() {
                Adt3::C0(_, h3976, _, _, _, _, _, _) => {
                    h3976
                }
            };
            let a3982 = {
                let a3979 = match v0.clone() {
                    Adt3::C0(_, _, h3977, _, _, _, _, _) => {
                        h3977
                    }
                };
                let a3980 = match v0 {
                    Adt3::C0(_, _, _, h3978, _, _, _, _) => {
                        h3978
                    }
                };
                nat_mul(a3979, a3980)?
            };
            nat_mul(a3981, a3982)?
        };
        nat_mul(a3983, a3984)
    }
}

pub fn f138(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3985 = v1;
    let a3986 = 13u64;
    if if nat_eq(a3985, a3986) {
        f34(match v0.clone() {
            Adt3::C0(h3987, _, _, _, _, _, _, _) => {
                h3987
            }
        }, match v0.clone() {
            Adt3::C0(_, h3988, _, _, _, _, _, _) => {
                h3988
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3989, _, _, _, _, _) => {
                h3989
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3990, _, _, _, _) => {
                h3990
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3996 = match v0.clone() {
                Adt3::C0(_, h3991, _, _, _, _, _, _) => {
                    h3991
                }
            };
            let a3997 = {
                let a3994 = match v0.clone() {
                    Adt3::C0(_, _, h3992, _, _, _, _, _) => {
                        h3992
                    }
                };
                let a3995 = match v0.clone() {
                    Adt3::C0(_, _, _, h3993, _, _, _, _) => {
                        h3993
                    }
                };
                nat_mul(a3994, a3995)?
            };
            nat_mul(a3996, a3997)?
        };
        let a3998 = v1;
        let a3999 = 13u64;
        if nat_eq(a3998, a3999) {
            let a4008 = 2u64;
            let a4009 = if f207(v0.clone(), v2) {
                let a4001 = v3;
                let a4002 = f208(match v0 {
                    Adt3::C0(h4000, _, _, _, _, _, _, _) => {
                        h4000
                    }
                }, false)?;
                nat_mul(a4001, a4002)?
            } else {
                let a4006 = v3;
                let a4007 = {
                    let a4004 = match v0 {
                        Adt3::C0(h4003, _, _, _, _, _, _, _) => {
                            h4003
                        }
                    };
                    let a4005 = 2u64;
                    nat_mul(a4004, a4005)?
                };
                nat_mul(a4006, a4007)?
            };
            nat_add(a4008, a4009)
        } else {
            let a4013 = v3;
            let a4014 = f208(match v0 {
                Adt3::C0(h4010, _, _, _, _, _, _, _) => {
                    h4010
                }
            }, {
                let a4011 = v1;
                let a4012 = 16u64;
                nat_eq(a4011, a4012)
            })?;
            nat_mul(a4013, a4014)
        }
    }
}

pub fn f139(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a4015 = v0;
    let a4016 = 30u64;
    if if nat_eq(a4015, a4016) {
        let a4019 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h4017, _, _, _, _) => {
                h4017
            }
        };
        let a4020 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h4018, _, _, _, _, _) => {
                h4018
            }
        };
        nat_lt(a4019, a4020)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m4021 = v3;
        match m4021 {
            None => {
                f209(v0, v1, match v2.clone() {
                    Adt2::C0(h4022, _, _, _, _, _, _, _, _, _, _) => {
                        h4022
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h4023, _, _, _, _, _, _, _, _, _, _) => {
                        h4023
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h4024, _, _, _, _, _, _, _, _, _) => {
                        h4024
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4025, _, _, _, _, _, _, _, _) => {
                        h4025
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4026, _, _, _, _, _, _, _) => {
                        h4026
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4027, _, _, _, _, _, _) => {
                        h4027
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h4028, _, _, _, _, _, _, _, _, _) => {
                        h4028
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4029, _, _, _, _, _, _, _, _) => {
                        h4029
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4030, _, _, _, _, _, _, _) => {
                        h4030
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4031, _, _, _, _, _, _) => {
                        h4031
                    }
                })
            }
            Some(v5) => {
                f209(v0, v1, match v2.clone() {
                    Adt2::C0(h4032, _, _, _, _, _, _, _, _, _, _) => {
                        h4032
                    }
                }, match v5.clone() {
                    Adt2::C0(h4033, _, _, _, _, _, _, _, _, _, _) => {
                        h4033
                    }
                }, match v4.clone() {
                    Adt2::C0(h4034, _, _, _, _, _, _, _, _, _, _) => {
                        h4034
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h4035, _, _, _, _, _, _, _, _, _) => {
                        h4035
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4036, _, _, _, _, _, _, _, _) => {
                        h4036
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4037, _, _, _, _, _, _, _) => {
                        h4037
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4038, _, _, _, _, _, _) => {
                        h4038
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h4039, _, _, _, _, _, _, _, _, _) => {
                        h4039
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h4040, _, _, _, _, _, _, _, _) => {
                        h4040
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h4041, _, _, _, _, _, _, _) => {
                        h4041
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h4042, _, _, _, _, _, _) => {
                        h4042
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4043, _, _, _, _, _, _, _, _, _) => {
                        h4043
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4044, _, _, _, _, _, _, _, _) => {
                        h4044
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4045, _, _, _, _, _, _, _) => {
                        h4045
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4046, _, _, _, _, _, _) => {
                        h4046
                    }
                })
            }
        }
    }
}

pub fn f140(v0: u64, v1: u64) -> u64 {
    let a4047 = v0;
    let a4048 = 23u64;
    if nat_eq(a4047, a4048) {
        let a4049 = v1;
        let a4050 = 0u64;
        if nat_eq(a4049, a4050) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f141(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4063 = {
        let a4059 = {
            let a4055 = {
                let a4051 = v0;
                let a4052 = v4;
                nat_mul(a4051, a4052)?
            };
            let a4056 = {
                let a4053 = v1;
                let a4054 = v5;
                nat_mul(a4053, a4054)?
            };
            nat_add(a4055, a4056)?
        };
        let a4060 = {
            let a4057 = v2;
            let a4058 = v6;
            nat_mul(a4057, a4058)?
        };
        nat_add(a4059, a4060)?
    };
    let a4064 = {
        let a4061 = v3;
        let a4062 = v7;
        nat_mul(a4061, a4062)?
    };
    nat_add(a4063, a4064)
}

pub fn f142(v0: u64) -> u64 {
    let a4065 = v0;
    let a4066 = 2147483648u64;
    if nat_lt(a4065, a4066) {
        0u64
    } else {
        let a4067 = 4294967296u64;
        let a4068 = v0;
        nat_sub(a4067, a4068)
    }
}

pub fn f143(v0: u64) -> u64 {
    let a4069 = v0;
    let a4070 = 2147483648u64;
    if nat_lt(a4069, a4070) {
        v0
    } else {
        0u64
    }
}

pub fn f144(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4071 = v7;
        let a4072 = 6u64;
        if nat_eq(a4071, a4072) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f96(v3, v4), f96(v5, v6), f97(v7, v8), f167(v1), f167(v9))
}

pub fn f145(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4073 = v0;
    match m4073 {
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
        Adt4::C13(v38, v39, v40, v41) => {
            Adt11::C4(v38, v39, v40, v41)
        }
        Adt4::C14(v42, _, _) => {
            let a4074 = 13u64;
            let a4075 = v42;
            if nat_le(a4074, a4075) {
                Adt11::C7(v42, v1, v2)
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
        Adt4::C27(v62) => {
            Adt11::C2(match v62.clone() {
                Adt5::C0(_, _, _, _, _, h4076, _, _) => {
                    h4076
                }
            }, match v62 {
                Adt5::C0(_, _, _, _, _, _, _, h4077) => {
                    h4077
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
        Adt4::C34(_, _, _, _) => {
            Adt11::C6
        }
    }
}

pub fn f146(v0: Adt4) -> u64 {
    let m4078 = v0;
    match m4078 {
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
        Adt4::C14(v40, _, _) => {
            v40
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
        Adt4::C34(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f147(v0: Adt4, v1: u64) -> u64 {
    let m4079 = v0;
    match m4079 {
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
        Adt4::C16(_, v45) => {
            v45
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v48, _) => {
            let a4080 = v1;
            let a4081 = 6u64;
            if nat_eq(a4080, a4081) {
                v48
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
        Adt4::C26(v62, _, _, _) => {
            let a4090 = v1;
            let a4091 = 6u64;
            if if if nat_eq(a4090, a4091) {
                true
            } else {
                let a4092 = v1;
                let a4093 = 7u64;
                nat_eq(a4092, a4093)
            } {
                let a4094 = v62;
                let a4095 = 30u64;
                nat_eq(a4094, a4095)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v61) => {
            match v61 {
                Adt5::C0(_, _, _, _, _, _, _, h4089) => {
                    h4089
                }
            }
        }
        Adt4::C28(v57) => {
            let a4082 = v1;
            let a4083 = 6u64;
            if nat_eq(a4082, a4083) {
                match v57 {
                    Adt7::C0(_, _, _, _, h4084) => {
                        h4084
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v60) => {
            let a4085 = v1;
            let a4086 = 6u64;
            if if nat_eq(a4085, a4086) {
                true
            } else {
                let a4087 = v1;
                let a4088 = 7u64;
                nat_eq(a4087, a4088)
            } {
                v60
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
        Adt4::C34(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f148(v0: Adt4, v1: u64) -> Adt11 {
    let a4096 = v1;
    let a4097 = 4u64;
    if nat_lt(a4096, a4097) {
        Adt11::C0(v1)
    } else {
        let a4098 = v1;
        let a4099 = 4u64;
        if nat_eq(a4098, a4099) {
            Adt11::C1(0u64)
        } else {
            let a4100 = v1;
            let a4101 = 5u64;
            if nat_eq(a4100, a4101) {
                Adt11::C1(3u64)
            } else {
                let a4102 = v1;
                let a4103 = 6u64;
                if nat_eq(a4102, a4103) {
                    Adt11::C2(f147(v0, v1), 0u64, false)
                } else {
                    let a4104 = v1;
                    let a4105 = 7u64;
                    if nat_eq(a4104, a4105) {
                        Adt11::C2(0u64, f147(v0, v1), false)
                    } else {
                        let a4106 = v1;
                        let a4107 = 16u64;
                        if nat_lt(a4106, a4107) {
                            Adt11::C1({
                                let a4108 = v1;
                                let a4109 = 8u64;
                                nat_sub(a4108, a4109)
                            })
                        } else {
                            let a4110 = v1;
                            let a4111 = 20u64;
                            if nat_lt(a4110, a4111) {
                                Adt11::C5({
                                    let a4112 = v1;
                                    let a4113 = 16u64;
                                    nat_sub(a4112, a4113)
                                })
                            } else {
                                let a4114 = v1;
                                let a4115 = 20u64;
                                if nat_eq(a4114, a4115) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4116 = v1;
                                    let a4117 = 21u64;
                                    if nat_eq(a4116, a4117) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4118 = v1;
                                        let a4119 = 22u64;
                                        if nat_eq(a4118, a4119) {
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

pub fn f149(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a4121 = v0;
        let a4122 = match v1.clone() {
            Adt3::C0(h4120, _, _, _, _, _, _, _) => {
                h4120
            }
        };
        let a4123 = 0u64;
        nat_quot(a4121, a4122, a4123)
    };
    let v3: u64 = {
        let a4125 = v2;
        let a4126 = match v1.clone() {
            Adt3::C0(_, h4124, _, _, _, _, _, _) => {
                h4124
            }
        };
        let a4127 = 0u64;
        nat_quot(a4125, a4126, a4127)
    };
    f141({
        let a4129 = v0;
        let a4130 = match v1.clone() {
            Adt3::C0(h4128, _, _, _, _, _, _, _) => {
                h4128
            }
        };
        let a4131 = 0u64;
        nat_rem(a4129, a4130, a4131)
    }, {
        let a4133 = v2;
        let a4134 = match v1.clone() {
            Adt3::C0(_, h4132, _, _, _, _, _, _) => {
                h4132
            }
        };
        let a4135 = 0u64;
        nat_rem(a4133, a4134, a4135)
    }, {
        let a4137 = v3;
        let a4138 = match v1.clone() {
            Adt3::C0(_, _, h4136, _, _, _, _, _) => {
                h4136
            }
        };
        let a4139 = 0u64;
        nat_rem(a4137, a4138, a4139)
    }, {
        let a4141 = v3;
        let a4142 = match v1.clone() {
            Adt3::C0(_, _, h4140, _, _, _, _, _) => {
                h4140
            }
        };
        let a4143 = 0u64;
        nat_quot(a4141, a4142, a4143)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4144, _, _, _) => {
            h4144
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4145, _, _) => {
            h4145
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4146, _) => {
            h4146
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4147) => {
            h4147
        }
    })
}

pub fn f150(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4165 = {
        let a4160 = f210(v2, {
            let a4151 = {
                let a4148 = v0;
                let a4149 = v1;
                let a4150 = 0u64;
                nat_quot(a4148, a4149, a4150)
            };
            let a4152 = v2;
            let a4153 = 0u64;
            nat_rem(a4151, a4152, a4153)
        }, {
            let a4157 = {
                let a4154 = v0;
                let a4155 = v1;
                let a4156 = 0u64;
                nat_quot(a4154, a4155, a4156)
            };
            let a4158 = v2;
            let a4159 = 0u64;
            nat_quot(a4157, a4158, a4159)
        })?;
        let a4161 = v1;
        nat_mul(a4160, a4161)?
    };
    let a4166 = {
        let a4162 = v0;
        let a4163 = v1;
        let a4164 = 0u64;
        nat_rem(a4162, a4163, a4164)
    };
    nat_add(a4165, a4166)
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4168 = v0;
        let a4169 = match v2.clone() {
            Adt3::C0(h4167, _, _, _, _, _, _, _) => {
                h4167
            }
        };
        let a4170 = 0u64;
        nat_quot(a4168, a4169, a4170)
    };
    let v4: u64 = {
        let a4172 = v3;
        let a4173 = match v2.clone() {
            Adt3::C0(_, h4171, _, _, _, _, _, _) => {
                h4171
            }
        };
        let a4174 = 0u64;
        nat_quot(a4172, a4173, a4174)
    };
    f211({
        let a4176 = v0;
        let a4177 = match v2.clone() {
            Adt3::C0(h4175, _, _, _, _, _, _, _) => {
                h4175
            }
        };
        let a4178 = 0u64;
        nat_rem(a4176, a4177, a4178)
    }, {
        let a4180 = v3;
        let a4181 = match v2.clone() {
            Adt3::C0(_, h4179, _, _, _, _, _, _) => {
                h4179
            }
        };
        let a4182 = 0u64;
        nat_rem(a4180, a4181, a4182)
    }, {
        let a4184 = v4;
        let a4185 = match v2.clone() {
            Adt3::C0(_, _, h4183, _, _, _, _, _) => {
                h4183
            }
        };
        let a4186 = 0u64;
        nat_rem(a4184, a4185, a4186)
    }, {
        let a4188 = v4;
        let a4189 = match v2 {
            Adt3::C0(_, _, h4187, _, _, _, _, _) => {
                h4187
            }
        };
        let a4190 = 0u64;
        nat_quot(a4188, a4189, a4190)
    }, match v1.clone() {
        Adt3::C0(h4191, _, _, _, _, _, _, _) => {
            h4191
        }
    }, match v1.clone() {
        Adt3::C0(_, h4192, _, _, _, _, _, _) => {
            h4192
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4193, _, _, _, _, _) => {
            h4193
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4194, _, _, _, _) => {
            h4194
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4195, _, _, _) => {
            h4195
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4196, _, _) => {
            h4196
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4197, _) => {
            h4197
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4198) => {
            h4198
        }
    })
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4200 = v0;
        let a4201 = match v3.clone() {
            Adt3::C0(h4199, _, _, _, _, _, _, _) => {
                h4199
            }
        };
        let a4202 = 0u64;
        nat_quot(a4200, a4201, a4202)
    };
    let v6: u64 = {
        let a4204 = v5;
        let a4205 = match v3.clone() {
            Adt3::C0(_, h4203, _, _, _, _, _, _) => {
                h4203
            }
        };
        let a4206 = 0u64;
        nat_quot(a4204, a4205, a4206)
    };
    let v7: u64 = {
        let a4208 = v0;
        let a4209 = match v3.clone() {
            Adt3::C0(h4207, _, _, _, _, _, _, _) => {
                h4207
            }
        };
        let a4210 = 0u64;
        nat_rem(a4208, a4209, a4210)
    };
    let v8: u64 = {
        let a4212 = v5;
        let a4213 = match v3.clone() {
            Adt3::C0(_, h4211, _, _, _, _, _, _) => {
                h4211
            }
        };
        let a4214 = 0u64;
        nat_rem(a4212, a4213, a4214)
    };
    let v9: u64 = {
        let a4216 = v6;
        let a4217 = match v3.clone() {
            Adt3::C0(_, _, h4215, _, _, _, _, _) => {
                h4215
            }
        };
        let a4218 = 0u64;
        nat_rem(a4216, a4217, a4218)
    };
    let v10: u64 = {
        let a4220 = v6;
        let a4221 = match v3.clone() {
            Adt3::C0(_, _, h4219, _, _, _, _, _) => {
                h4219
            }
        };
        let a4222 = 0u64;
        nat_quot(a4220, a4221, a4222)
    };
    let v11: u64 = {
        let a4223 = v4;
        let a4224 = 0u64;
        if nat_eq(a4223, a4224) {
            match v1.clone() {
                Adt3::C0(h4225, _, _, _, _, _, _, _) => {
                    h4225
                }
            }
        } else {
            let a4226 = v4;
            let a4227 = 1u64;
            if nat_eq(a4226, a4227) {
                match v1.clone() {
                    Adt3::C0(_, h4228, _, _, _, _, _, _) => {
                        h4228
                    }
                }
            } else {
                let a4229 = v4;
                let a4230 = 2u64;
                if nat_eq(a4229, a4230) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4231, _, _, _, _, _) => {
                            h4231
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4232, _, _, _, _) => {
                            h4232
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4239 = v11;
        let a4240 = {
            let a4233 = v4;
            let a4234 = 0u64;
            if nat_eq(a4233, a4234) {
                v7
            } else {
                let a4235 = v4;
                let a4236 = 1u64;
                if nat_eq(a4235, a4236) {
                    v8
                } else {
                    let a4237 = v4;
                    let a4238 = 2u64;
                    if nat_eq(a4237, a4238) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4239, a4240)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f141(if if v12 {
        let a4241 = v4;
        let a4242 = 0u64;
        nat_eq(a4241, a4242)
    } else {
        false
    } {
        let a4243 = v7;
        let a4244 = v11;
        nat_sub(a4243, a4244)
    } else {
        v7
    }, if if v12 {
        let a4245 = v4;
        let a4246 = 1u64;
        nat_eq(a4245, a4246)
    } else {
        false
    } {
        let a4247 = v8;
        let a4248 = v11;
        nat_sub(a4247, a4248)
    } else {
        v8
    }, if if v12 {
        let a4249 = v4;
        let a4250 = 2u64;
        nat_eq(a4249, a4250)
    } else {
        false
    } {
        let a4251 = v9;
        let a4252 = v11;
        nat_sub(a4251, a4252)
    } else {
        v9
    }, if if v12 {
        let a4253 = v4;
        let a4254 = 3u64;
        nat_eq(a4253, a4254)
    } else {
        false
    } {
        let a4255 = v10;
        let a4256 = v11;
        nat_sub(a4255, a4256)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4257, _, _, _) => {
            h4257
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4258, _, _) => {
            h4258
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4259, _) => {
            h4259
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4260) => {
            h4260
        }
    })?, f149(v0, v3)?))
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f73(v0, v2.clone(), 0u64);
    let v9: u64 = f73(v0, v2.clone(), 1u64);
    let v10: u64 = f73(v0, v2.clone(), 2u64);
    let v11: u64 = f73(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f72(v1.clone(), f212(v8, v3, match v1.clone() {
            Adt3::C0(h4261, _, _, _, _, _, _, _) => {
                h4261
            }
        })?, f212(v9, v4, match v1.clone() {
            Adt3::C0(_, h4262, _, _, _, _, _, _) => {
                h4262
            }
        })?, f212(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4263, _, _, _, _, _) => {
                h4263
            }
        })?, f212(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4264, _, _, _, _) => {
                h4264
            }
        })?)?, f149(v0, v2)?))
    } else {
        if if if if f213(v8, v3, match v1.clone() {
            Adt3::C0(h4265, _, _, _, _, _, _, _) => {
                h4265
            }
        })? {
            f213(v9, v4, match v1.clone() {
                Adt3::C0(_, h4266, _, _, _, _, _, _) => {
                    h4266
                }
            })?
        } else {
            false
        } {
            f213(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4267, _, _, _, _, _) => {
                    h4267
                }
            })?
        } else {
            false
        } {
            f213(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4268, _, _, _, _) => {
                    h4268
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f72(v1, f214(v8, v3)?, f214(v9, v4)?, f214(v10, v5)?, f214(v11, v6)?)?, f149(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f149(v0, v2)?))
        }
    }
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f73(v0, v2.clone(), 0u64);
    let v5: u64 = f73(v0, v2.clone(), 1u64);
    let v6: u64 = f73(v0, v2.clone(), 2u64);
    let v7: u64 = f73(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f72(v1.clone(), f215(v4, v3, match v1 {
        Adt3::C0(h4269, _, _, _, _, _, _, _) => {
            h4269
        }
    })?, v5, v6, v7)?, f149(v0, v2)?))
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4270 = v0;
    let a4271 = v3;
    if nat_lt(a4270, a4271) {
        Ok(Adt9::C1(false, {
            let a4272 = v0;
            let a4273 = 4u64;
            nat_mul(a4272, a4273)?
        }, {
            let a4274 = v0;
            let a4275 = 4u64;
            nat_mul(a4274, a4275)?
        }))
    } else {
        let v6: u64 = {
            let a4276 = v0;
            let a4277 = v3;
            nat_sub(a4276, a4277)
        };
        let v7: u64 = f149(v6, v1)?;
        let v8: u64 = f216(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4278 = v0;
    let a4279 = v3;
    if nat_lt(a4278, a4279) {
        Ok(Adt9::C1(true, 0u64, f149(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4280 = v0;
            let a4281 = v3;
            nat_sub(a4280, a4281)
        };
        let v5: u64 = f151(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f149(v4, v1)?, 3u64, v5))
    }
}

pub fn f157(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4283 = v1;
        let a4284 = match v3.clone() {
            Adt3::C0(h4282, _, _, _, _, _, _, _) => {
                h4282
            }
        };
        let a4285 = 0u64;
        nat_rem(a4283, a4284, a4285)
    };
    let v5: u64 = {
        let a4291 = {
            let a4287 = v1;
            let a4288 = match v3.clone() {
                Adt3::C0(h4286, _, _, _, _, _, _, _) => {
                    h4286
                }
            };
            let a4289 = 0u64;
            nat_quot(a4287, a4288, a4289)
        };
        let a4292 = match v3.clone() {
            Adt3::C0(_, h4290, _, _, _, _, _, _) => {
                h4290
            }
        };
        let a4293 = 0u64;
        nat_rem(a4291, a4292, a4293)
    };
    let v6: bool = f217(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f151(v1, v2, v3.clone())?
    } else {
        0u64
    }, f149(v1, v3)?))
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4294 = v3;
        let a4295 = 8u64;
        nat_add(a4294, a4295)?
    }, 0u64, f149(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f151(v0, v5, v2.clone())?
    }, 3u64, f149(v0, v2)?))
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4296 = v4;
    let a4297 = 0u64;
    if if nat_eq(a4296, a4297) {
        f53(v3)
    } else {
        false
    } {
        let m4298 = f124(v3);
        match m4298 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h4299, _) => {
                        h4299
                    }
                };
                Ok(Adt9::C5(v3, f149(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h4300) => {
                        h4300
                    }
                }, f149({
                    let a4301 = v0;
                    let a4302 = v6;
                    nat_mul(a4301, a4302)?
                }, v2.clone())?, {
                    let a4308 = {
                        let a4306 = {
                            let a4303 = v6;
                            let a4304 = 1u64;
                            nat_sub(a4303, a4304)
                        };
                        let a4307 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4305, _, _, _) => {
                                h4305
                            }
                        };
                        nat_mul(a4306, a4307)?
                    };
                    let a4309 = 4u64;
                    nat_add(a4308, a4309)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4310, _, _, _) => {
                        h4310
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f149(v0, v1)?, f149(v0, v2)?))
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4312 = v0;
        let a4313 = match v2.clone() {
            Adt3::C0(h4311, _, _, _, _, _, _, _) => {
                h4311
            }
        };
        let a4314 = 0u64;
        nat_rem(a4312, a4313, a4314)
    };
    let v7: u64 = {
        let a4316 = v0;
        let a4317 = match v2.clone() {
            Adt3::C0(h4315, _, _, _, _, _, _, _) => {
                h4315
            }
        };
        let a4318 = 0u64;
        nat_quot(a4316, a4317, a4318)
    };
    let v8: u64 = {
        let a4320 = v7;
        let a4321 = match v2.clone() {
            Adt3::C0(_, h4319, _, _, _, _, _, _) => {
                h4319
            }
        };
        let a4322 = 0u64;
        nat_rem(a4320, a4321, a4322)
    };
    let v9: u64 = {
        let a4324 = v7;
        let a4325 = match v2.clone() {
            Adt3::C0(_, h4323, _, _, _, _, _, _) => {
                h4323
            }
        };
        let a4326 = 0u64;
        nat_quot(a4324, a4325, a4326)
    };
    let v10: u64 = {
        let a4328 = v9;
        let a4329 = match v2.clone() {
            Adt3::C0(_, _, h4327, _, _, _, _, _) => {
                h4327
            }
        };
        let a4330 = 0u64;
        nat_rem(a4328, a4329, a4330)
    };
    let v11: u64 = {
        let a4332 = v9;
        let a4333 = match v2.clone() {
            Adt3::C0(_, _, h4331, _, _, _, _, _) => {
                h4331
            }
        };
        let a4334 = 0u64;
        nat_quot(a4332, a4333, a4334)
    };
    let v12: u64 = f149(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4343 = {
                let a4338 = {
                    let a4335 = v10;
                    let a4336 = v4;
                    let a4337 = 0u64;
                    nat_quot(a4335, a4336, a4337)
                };
                let a4339 = v5;
                nat_mul(a4338, a4339)?
            };
            let a4344 = {
                let a4340 = v8;
                let a4341 = v4;
                let a4342 = 0u64;
                nat_quot(a4340, a4341, a4342)
            };
            nat_add(a4343, a4344)?
        };
        let v14: u64 = {
            let a4364 = v6;
            let a4365 = {
                let a4362 = match v1.clone() {
                    Adt3::C0(h4345, _, _, _, _, _, _, _) => {
                        h4345
                    }
                };
                let a4363 = {
                    let a4360 = {
                        let a4346 = v8;
                        let a4347 = v4;
                        let a4348 = 0u64;
                        nat_rem(a4346, a4347, a4348)
                    };
                    let a4361 = {
                        let a4358 = match v1.clone() {
                            Adt3::C0(_, h4349, _, _, _, _, _, _) => {
                                h4349
                            }
                        };
                        let a4359 = {
                            let a4356 = {
                                let a4350 = v10;
                                let a4351 = v4;
                                let a4352 = 0u64;
                                nat_rem(a4350, a4351, a4352)
                            };
                            let a4357 = {
                                let a4354 = match v1.clone() {
                                    Adt3::C0(_, _, h4353, _, _, _, _, _) => {
                                        h4353
                                    }
                                };
                                let a4355 = v13;
                                nat_mul(a4354, a4355)?
                            };
                            nat_add(a4356, a4357)?
                        };
                        nat_mul(a4358, a4359)?
                    };
                    nat_add(a4360, a4361)?
                };
                nat_mul(a4362, a4363)?
            };
            nat_add(a4364, a4365)?
        };
        Ok(Adt9::C1(false, f149(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4371 = {
                let a4369 = {
                    let a4366 = v11;
                    let a4367 = v5;
                    let a4368 = 0u64;
                    nat_rem(a4366, a4367, a4368)
                };
                let a4370 = v4;
                nat_mul(a4369, a4370)?
            };
            let a4372 = v8;
            nat_add(a4371, a4372)?
        };
        let v16: u64 = {
            let a4378 = {
                let a4376 = {
                    let a4373 = v11;
                    let a4374 = v5;
                    let a4375 = 0u64;
                    nat_quot(a4373, a4374, a4375)
                };
                let a4377 = v4;
                nat_mul(a4376, a4377)?
            };
            let a4379 = v10;
            nat_add(a4378, a4379)?
        };
        let a4381 = v15;
        let a4382 = match v1.clone() {
            Adt3::C0(_, h4380, _, _, _, _, _, _) => {
                h4380
            }
        };
        if if nat_lt(a4381, a4382) {
            let a4384 = v16;
            let a4385 = match v1.clone() {
                Adt3::C0(_, _, h4383, _, _, _, _, _) => {
                    h4383
                }
            };
            nat_lt(a4384, a4385)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f149({
                let a4394 = v6;
                let a4395 = {
                    let a4392 = match v1.clone() {
                        Adt3::C0(h4386, _, _, _, _, _, _, _) => {
                            h4386
                        }
                    };
                    let a4393 = {
                        let a4390 = v15;
                        let a4391 = {
                            let a4388 = match v1.clone() {
                                Adt3::C0(_, h4387, _, _, _, _, _, _) => {
                                    h4387
                                }
                            };
                            let a4389 = v16;
                            nat_mul(a4388, a4389)?
                        };
                        nat_add(a4390, a4391)?
                    };
                    nat_mul(a4392, a4393)?
                };
                nat_add(a4394, a4395)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f140(23u64, v4);
    let v6: u64 = {
        let a4396 = v0;
        let a4397 = v5;
        let a4398 = 0u64;
        nat_rem(a4396, a4397, a4398)
    };
    let v7: u64 = {
        let a4399 = v0;
        let a4400 = v5;
        let a4401 = 0u64;
        nat_quot(a4399, a4400, a4401)
    };
    let v8: Adt13 = Adt13::C1(f149(v7, v2)?);
    let a4402 = v6;
    let a4403 = 0u64;
    if nat_eq(a4402, a4403) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f149(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f167(v3), 4u64))
    } else {
        let a4404 = v6;
        let a4405 = 1u64;
        if nat_eq(a4404, a4405) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4406 = v4;
                let a4407 = 0u64;
                if nat_eq(a4406, a4407) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f167(v4)))
        }
    }
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f149(v0, v2.clone())?;
    let v4: u64 = f149(v0, v1.clone())?;
    let a4412 = {
        let a4409 = v0;
        let a4410 = match v1 {
            Adt3::C0(h4408, _, _, _, _, _, _, _) => {
                h4408
            }
        };
        let a4411 = 0u64;
        nat_rem(a4409, a4410, a4411)
    };
    let a4413 = 0u64;
    if nat_eq(a4412, a4413) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f149({
            let a4414 = v0;
            let a4415 = 1u64;
            nat_sub(a4414, a4415)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4416 = v3;
    let a4417 = 13u64;
    if nat_eq(a4416, a4417) {
        f218(v0, v1, v4)
    } else {
        let v5: u64 = f208(match v1.clone() {
            Adt3::C0(h4418, _, _, _, _, _, _, _) => {
                h4418
            }
        }, {
            let a4419 = v3;
            let a4420 = 16u64;
            nat_eq(a4419, a4420)
        })?;
        let v6: u64 = {
            let a4421 = v0;
            let a4422 = v5;
            let a4423 = 0u64;
            nat_quot(a4421, a4422, a4423)
        };
        let v7: u64 = {
            let a4424 = v0;
            let a4425 = v5;
            let a4426 = 0u64;
            nat_rem(a4424, a4425, a4426)
        };
        let v8: u64 = {
            let a4430 = 1u64;
            let a4431 = {
                let a4428 = match v1.clone() {
                    Adt3::C0(h4427, _, _, _, _, _, _, _) => {
                        h4427
                    }
                };
                let a4429 = 2u64;
                nat_mul(a4428, a4429)?
            };
            nat_add(a4430, a4431)?
        };
        let a4432 = v7;
        let a4433 = v8;
        if nat_lt(a4432, a4433) {
            f219(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f149(v6, v2)?;
            let a4434 = v3;
            let a4435 = 14u64;
            if nat_eq(a4434, a4435) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4436 = v7;
                let a4437 = v8;
                if nat_eq(a4436, a4437) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4440 = v7;
                    let a4441 = {
                        let a4438 = v8;
                        let a4439 = 1u64;
                        nat_add(a4438, a4439)?
                    };
                    if nat_eq(a4440, a4441) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4442, _, _, _, _, _, _, _) => {
                                h4442
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

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4443 = v0;
    let a4444 = 0u64;
    if nat_eq(a4443, a4444) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4451 = {
            let a4445 = v0;
            let a4446 = 1u64;
            nat_add(a4445, a4446)?
        };
        let a4452 = {
            let a4449 = 2u64;
            let a4450 = {
                let a4447 = v3;
                let a4448 = 2u64;
                nat_mul(a4447, a4448)?
            };
            nat_add(a4449, a4450)?
        };
        if nat_eq(a4451, a4452) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4453 = v0;
                let a4454 = 1u64;
                nat_sub(a4453, a4454)
            };
            let v5: u64 = {
                let a4455 = v4;
                let a4456 = 2u64;
                let a4457 = 0u64;
                nat_quot(a4455, a4456, a4457)
            };
            let a4461 = {
                let a4458 = v4;
                let a4459 = 2u64;
                let a4460 = 0u64;
                nat_rem(a4458, a4459, a4460)
            };
            let a4462 = 0u64;
            if nat_eq(a4461, a4462) {
                Ok(Adt9::C4(17u64, 0u64, f149(v5, v1)?, 4u64, f149(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4463 = v0;
    let a4464 = f27(v1, v2, v3)?;
    if nat_lt(a4463, a4464) {
        let a4465 = v0;
        let a4466 = v1;
        if nat_lt(a4465, a4466) {
            Ok(Adt9::C1(false, {
                let a4467 = 4u64;
                let a4468 = v0;
                nat_mul(a4467, a4468)?
            }, {
                let a4469 = 4u64;
                let a4470 = v0;
                nat_mul(a4469, a4470)?
            }))
        } else {
            let v4: u64 = {
                let a4471 = v0;
                let a4472 = v1;
                nat_sub(a4471, a4472)
            };
            let v5: u64 = f220(v4, v2)?;
            let a4473 = v5;
            let a4474 = 2u64;
            if nat_lt(a4473, a4474) {
                Ok(Adt9::C4(6u64, {
                    let a4475 = v5;
                    let a4476 = 0u64;
                    if nat_eq(a4475, a4476) {
                        4u64
                    } else {
                        5u64
                    }
                }, f221(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f222(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f223(v4, v2)? {
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

pub fn f166(v0: u64, v1: u64) -> Adt11 {
    let a4477 = v0;
    let a4478 = 43u64;
    if nat_eq(a4477, a4478) {
        Adt11::C8(3u64)
    } else {
        let a4479 = v1;
        let a4480 = 43u64;
        if nat_eq(a4479, a4480) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f167(v0: u64) -> u64 {
    let m4481 = f124(v0);
    match m4481 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4483 = match v1.clone() {
                Adt18::C0(_, h4482, _) => {
                    h4482
                }
            };
            let a4484 = 1u64;
            if if nat_eq(a4483, a4484) {
                let a4486 = match v1.clone() {
                    Adt18::C0(_, _, h4485) => {
                        h4485
                    }
                };
                let a4487 = 1u64;
                if if if nat_eq(a4486, a4487) {
                    true
                } else {
                    let a4489 = match v1.clone() {
                        Adt18::C0(_, _, h4488) => {
                            h4488
                        }
                    };
                    let a4490 = 2u64;
                    nat_eq(a4489, a4490)
                } {
                    true
                } else {
                    let a4492 = match v1.clone() {
                        Adt18::C0(_, _, h4491) => {
                            h4491
                        }
                    };
                    let a4493 = 4u64;
                    nat_eq(a4492, a4493)
                } {
                    true
                } else {
                    let a4495 = match v1.clone() {
                        Adt18::C0(_, _, h4494) => {
                            h4494
                        }
                    };
                    let a4496 = 8u64;
                    nat_eq(a4495, a4496)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4497) => {
                        h4497
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f168(v0: u64, v1: u64) -> R<u64> {
    if f114(v0) {
        let a4498 = 2u64;
        let a4499 = v1;
        nat_mul(a4498, a4499)
    } else {
        let a4500 = v1;
        let a4501 = f204(v0);
        let a4502 = 0u64;
        Ok(nat_quot(a4500, a4501, a4502))
    }
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4503 = f124(v0);
    match m4503 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f141(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4504, _, _, _) => {
                    h4504
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4505, _, _) => {
                    h4505
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4506, _) => {
                    h4506
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4507) => {
                    h4507
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4508) => {
                    h4508
                }
            }, f141({
                let a4510 = v2;
                let a4511 = match v6.clone() {
                    Adt18::C0(_, h4509, _) => {
                        h4509
                    }
                };
                nat_mul(a4510, a4511)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4512, _, _, _) => {
                    h4512
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4513, _, _) => {
                    h4513
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4514, _) => {
                    h4514
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4515) => {
                    h4515
                }
            })?, match v6 {
                Adt18::C0(_, h4516, _) => {
                    h4516
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4517, _, _, _) => {
                    h4517
                }
            }))
        }
    }
}

pub fn f170(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4525 = {
        let a4518 = 2u64;
        let a4519 = f137(v1.clone())?;
        nat_mul(a4518, a4519)?
    };
    let a4526 = {
        let a4523 = f137(v1)?;
        let a4524 = {
            let a4521 = 1u64;
            let a4522 = match v0 {
                Adt3::C0(h4520, _, _, _, _, _, _, _) => {
                    h4520
                }
            };
            nat_add(a4521, a4522)?
        };
        nat_mul(a4523, a4524)?
    };
    nat_add(a4525, a4526)
}

pub fn f171(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f149(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f172(v0: u64, v1: u64) -> bool {
    let a4527 = v1;
    let a4528 = 2147483647u64;
    if nat_le(a4527, a4528) {
        let a4529 = v1;
        let a4530 = v0;
        nat_lt(a4529, a4530)
    } else {
        false
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4531 = f124(v0);
    match m4531 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4533 = v2;
                let a4534 = match v7.clone() {
                    Adt18::C0(_, h4532, _) => {
                        h4532
                    }
                };
                nat_mul(a4533, a4534)?
            };
            let m4547 = f224(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4535, _, _, _, _, _, _, _) => {
                    h4535
                }
            }, match v5.clone() {
                Adt3::C0(_, h4536, _, _, _, _, _, _) => {
                    h4536
                }
            }, match v5 {
                Adt3::C0(_, _, h4537, _, _, _, _, _) => {
                    h4537
                }
            }, {
                let a4540 = match v3.clone() {
                    Adt3::C0(h4538, _, _, _, _, _, _, _) => {
                        h4538
                    }
                };
                let a4541 = match v7.clone() {
                    Adt18::C0(_, h4539, _) => {
                        h4539
                    }
                };
                nat_mul(a4540, a4541)?
            }, match v3.clone() {
                Adt3::C0(_, h4542, _, _, _, _, _, _) => {
                    h4542
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4543, _, _, _) => {
                    h4543
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4544, _, _) => {
                    h4544
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4545, _) => {
                    h4545
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4546) => {
                    h4546
                }
            })?;
            match m4547 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4548 = v0;
                    let a4549 = 0u64;
                    if if nat_eq(a4548, a4549) {
                        true
                    } else {
                        let a4550 = v0;
                        let a4551 = 26u64;
                        nat_eq(a4550, a4551)
                    } {
                        Ok(Adt9::C1(false, v9, f149({
                            let a4555 = {
                                let a4553 = v1;
                                let a4554 = match v4.clone() {
                                    Adt3::C0(h4552, _, _, _, _, _, _, _) => {
                                        h4552
                                    }
                                };
                                nat_mul(a4553, a4554)?
                            };
                            let a4556 = v8;
                            nat_add(a4555, a4556)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4557) => {
                                h4557
                            }
                        }, f149({
                            let a4561 = {
                                let a4559 = v1;
                                let a4560 = match v4.clone() {
                                    Adt3::C0(h4558, _, _, _, _, _, _, _) => {
                                        h4558
                                    }
                                };
                                nat_mul(a4559, a4560)?
                            };
                            let a4562 = v8;
                            nat_add(a4561, a4562)?
                        }, v4.clone())?, {
                            let a4569 = {
                                let a4567 = {
                                    let a4564 = match v7 {
                                        Adt18::C0(_, h4563, _) => {
                                            h4563
                                        }
                                    };
                                    let a4565 = 1u64;
                                    nat_sub(a4564, a4565)
                                };
                                let a4568 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4566, _, _, _) => {
                                        h4566
                                    }
                                };
                                nat_mul(a4567, a4568)?
                            };
                            let a4570 = 4u64;
                            nat_add(a4569, a4570)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4571, _, _, _) => {
                                h4571
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f174(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4572 = v1;
    let a4573 = 0u64;
    if if nat_eq(a4572, a4573) {
        true
    } else {
        let a4574 = v2;
        let a4575 = 0u64;
        nat_eq(a4574, a4575)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f175(v0: u64, v1: u64) -> bool {
    let a4576 = v0;
    let a4577 = 4294967295u64;
    if nat_eq(a4576, a4577) {
        true
    } else {
        let a4578 = v0;
        let a4579 = 2147483648u64;
        if nat_lt(a4578, a4579) {
            let a4580 = v0;
            let a4581 = v1;
            nat_lt(a4580, a4581)
        } else {
            false
        }
    }
}

pub fn f176(v0: Adt7) -> R<u64> {
    let a4586 = if match v0.clone() {
        Adt7::C0(_, _, _, h4582, _) => {
            h4582
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4587 = {
        let a4584 = match v0 {
            Adt7::C0(_, _, _, _, h4583) => {
                h4583
            }
        };
        let a4585 = 1u64;
        if nat_eq(a4584, a4585) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4586, a4587)
}

pub fn f177(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4592 = {
            let a4589 = v2;
            let a4590 = match v0.clone() {
                Adt6::C0(_, h4588, _, _) => {
                    h4588
                }
            };
            nat_mul(a4589, a4590)?
        };
        let a4593 = f142(match v0.clone() {
            Adt6::C0(_, _, h4591, _) => {
                h4591
            }
        });
        nat_add(a4592, a4593)?
    };
    let v4: u64 = f143(match v0.clone() {
        Adt6::C0(_, _, h4594, _) => {
            h4594
        }
    });
    let v5: u64 = {
        let a4598 = {
            let a4596 = v3;
            let a4597 = match v0 {
                Adt6::C0(h4595, _, _, _) => {
                    h4595
                }
            };
            nat_add(a4596, a4597)?
        };
        let a4599 = v4;
        nat_sub(a4598, a4599)
    };
    let a4604 = {
        let a4600 = v5;
        let a4601 = v1;
        if nat_lt(a4600, a4601) {
            v5
        } else {
            v1
        }
    };
    let a4605 = {
        let a4602 = v3;
        let a4603 = v4;
        nat_sub(a4602, a4603)
    };
    Ok(nat_sub(a4604, a4605))
}

pub fn f178(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4614 = f75(match v0.clone() {
        Adt7::C0(h4606, _, _, _, _) => {
            h4606
        }
    }, match v1.clone() {
        Adt3::C0(h4607, _, _, _, _, _, _, _) => {
            h4607
        }
    }, f73(v3, v2.clone(), 0u64), {
        let a4611 = v4;
        let a4612 = {
            let m4610 = match v0.clone() {
                Adt7::C0(h4609, _, _, _, _) => {
                    h4609
                }
            };
            match m4610 {
                Adt6::C0(h4608, _, _, _) => {
                    h4608
                }
            }
        };
        let a4613 = 0u64;
        nat_rem(a4611, a4612, a4613)
    })?;
    match m4614 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4623 = f75(match v0.clone() {
                Adt7::C0(_, h4615, _, _, _) => {
                    h4615
                }
            }, match v1.clone() {
                Adt3::C0(_, h4616, _, _, _, _, _, _) => {
                    h4616
                }
            }, f73(v3, v2.clone(), 1u64), {
                let a4620 = v4;
                let a4621 = {
                    let m4619 = match v0 {
                        Adt7::C0(h4618, _, _, _, _) => {
                            h4618
                        }
                    };
                    match m4619 {
                        Adt6::C0(h4617, _, _, _) => {
                            h4617
                        }
                    }
                };
                let a4622 = 0u64;
                nat_quot(a4620, a4621, a4622)
            })?;
            match m4623 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f72(v1, v5, v6, f73(v3, v2.clone(), 2u64), f73(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f179(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4624 = v0;
    match m4624 {
        Adt16::C0(v19) => {
            f225(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4625 = v20;
            let a4626 = 66u64;
            if nat_eq(a4625, a4626) {
                f226(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f227(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f180(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4633 = v3;
        let a4634 = {
            let a4630 = f149({
                let a4627 = v0;
                let a4628 = 2u64;
                let a4629 = 0u64;
                nat_quot(a4627, a4628, a4629)
            }, v1)?;
            let a4631 = v2;
            let a4632 = 0u64;
            nat_quot(a4630, a4631, a4632)
        };
        nat_add(a4633, a4634)?
    };
    let a4638 = {
        let a4635 = v0;
        let a4636 = 2u64;
        let a4637 = 0u64;
        nat_rem(a4635, a4636, a4637)
    };
    let a4639 = 0u64;
    if nat_eq(a4638, a4639) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4640 = v4;
        let a4641 = 0u64;
        if nat_eq(a4640, a4641) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f181(v0: u64, v1: u64) -> R<u64> {
    let a4644 = 8u64;
    let a4645 = {
        let a4642 = f228(v0);
        let a4643 = v1;
        nat_mul(a4642, a4643)?
    };
    nat_add(a4644, a4645)
}

pub fn f182(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4646, _, _, _, _, _, _, _) => {
            h4646
        }
    };
    let v7: u64 = f228(v3);
    let v8: u64 = {
        let a4649 = 1u64;
        let a4650 = {
            let a4647 = v6;
            let a4648 = v7;
            nat_mul(a4647, a4648)?
        };
        nat_add(a4649, a4650)?
    };
    let a4651 = v0;
    let a4652 = 0u64;
    if nat_eq(a4651, a4652) {
        Ok(f229(0u64))
    } else {
        let a4653 = v0;
        let a4654 = v8;
        if nat_lt(a4653, a4654) {
            let v9: u64 = {
                let a4655 = v0;
                let a4656 = 1u64;
                nat_sub(a4655, a4656)
            };
            let v10: u64 = {
                let a4657 = v9;
                let a4658 = v7;
                let a4659 = 0u64;
                nat_rem(a4657, a4658, a4659)
            };
            let v11: u64 = f149({
                let a4665 = {
                    let a4660 = v1;
                    let a4661 = v6;
                    nat_mul(a4660, a4661)?
                };
                let a4666 = {
                    let a4662 = v9;
                    let a4663 = v7;
                    let a4664 = 0u64;
                    nat_quot(a4662, a4663, a4664)
                };
                nat_add(a4665, a4666)?
            }, v2)?;
            let a4667 = v3;
            let a4668 = 30u64;
            if nat_eq(a4667, a4668) {
                let a4669 = v10;
                let a4670 = 0u64;
                if nat_eq(a4669, a4670) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4671 = v10;
                    let a4672 = 1u64;
                    if nat_eq(a4671, a4672) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4673 = v10;
                        let a4674 = 2u64;
                        if nat_eq(a4673, a4674) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4675 = v10;
                let a4676 = 0u64;
                if nat_eq(a4675, a4676) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4677 = v10;
                    let a4678 = 1u64;
                    if nat_eq(a4677, a4678) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4679 = v0;
                let a4680 = v8;
                nat_sub(a4679, a4680)
            };
            let a4681 = v12;
            let a4682 = 0u64;
            if nat_eq(a4681, a4682) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4683 = v12;
                let a4684 = 1u64;
                if nat_eq(a4683, a4684) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4685 = v12;
                    let a4686 = 2u64;
                    if nat_eq(a4685, a4686) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4687 = v12;
                        let a4688 = 3u64;
                        if nat_eq(a4687, a4688) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4689 = v12;
                            let a4690 = 4u64;
                            if nat_eq(a4689, a4690) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4691 = v12;
                                let a4692 = 5u64;
                                if nat_eq(a4691, a4692) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4693 = v12;
                                    let a4694 = 6u64;
                                    if nat_eq(a4693, a4694) {
                                        if f230(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4695 = 3u64;
                                                let a4696 = v1;
                                                nat_add(a4695, a4696)?
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

pub fn f183(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4697 = v0;
        let a4698 = f108(v3);
        let a4699 = 0u64;
        nat_quot(a4697, a4698, a4699)
    };
    let v5: u64 = {
        let a4704 = 3u64;
        let a4705 = {
            let a4701 = v4;
            let a4702 = match v1.clone() {
                Adt3::C0(h4700, _, _, _, _, _, _, _) => {
                    h4700
                }
            };
            let a4703 = 0u64;
            nat_quot(a4701, a4702, a4703)
        };
        nat_add(a4704, a4705)?
    };
    let v6: u64 = f149(v4, v1)?;
    let v7: u64 = f149(v4, v2)?;
    let a4706 = v3;
    let a4707 = 30u64;
    if nat_eq(a4706, a4707) {
        let a4711 = {
            let a4708 = v0;
            let a4709 = 3u64;
            let a4710 = 0u64;
            nat_rem(a4708, a4709, a4710)
        };
        let a4712 = 0u64;
        if nat_eq(a4711, a4712) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4716 = {
                let a4713 = v0;
                let a4714 = 3u64;
                let a4715 = 0u64;
                nat_rem(a4713, a4714, a4715)
            };
            let a4717 = 1u64;
            if nat_eq(a4716, a4717) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f184(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4718 = v0;
        let a4719 = 8388608u64;
        let a4720 = 0u64;
        nat_quot(a4718, a4719, a4720)
    };
    let a4721 = v0;
    let a4722 = 2147483648u64;
    if if if nat_lt(a4721, a4722) {
        let a4723 = 127u64;
        let a4724 = v1;
        nat_le(a4723, a4724)
    } else {
        false
    } {
        let a4725 = v1;
        let a4726 = 190u64;
        nat_lt(a4725, a4726)
    } else {
        false
    } {
        let v2: u64 = {
            let a4730 = 8388608u64;
            let a4731 = {
                let a4727 = v0;
                let a4728 = 8388608u64;
                let a4729 = 0u64;
                nat_rem(a4727, a4728, a4729)
            };
            nat_add(a4730, a4731)?
        };
        let a4732 = v1;
        let a4733 = 150u64;
        if nat_lt(a4732, a4733) {
            let v3: u64 = f231({
                let a4734 = 150u64;
                let a4735 = v1;
                nat_sub(a4734, a4735)
            });
            let a4739 = {
                let a4736 = v2;
                let a4737 = v3;
                let a4738 = 0u64;
                nat_rem(a4736, a4737, a4738)
            };
            let a4740 = 0u64;
            if nat_eq(a4739, a4740) {
                let a4741 = v2;
                let a4742 = v3;
                let a4743 = 0u64;
                Ok(nat_quot(a4741, a4742, a4743))
            } else {
                Ok(0u64)
            }
        } else {
            let a4746 = f231({
                let a4744 = v1;
                let a4745 = 150u64;
                nat_sub(a4744, a4745)
            });
            let a4747 = v2;
            nat_mul(a4746, a4747)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f185(v0: Adt14, v1: Adt14) -> bool {
    let a4750 = match v0.clone() {
        Adt14::C0(_, h4748, _, _, _, _, _, _, _, _, _) => {
            h4748
        }
    };
    let a4751 = match v1.clone() {
        Adt14::C0(_, h4749, _, _, _, _, _, _, _, _, _) => {
            h4749
        }
    };
    if if if nat_eq(a4750, a4751) {
        let a4754 = match v0.clone() {
            Adt14::C0(_, _, h4752, _, _, _, _, _, _, _, _) => {
                h4752
            }
        };
        let a4755 = match v1.clone() {
            Adt14::C0(_, _, h4753, _, _, _, _, _, _, _, _) => {
                h4753
            }
        };
        nat_eq(a4754, a4755)
    } else {
        false
    } {
        let a4758 = match v0.clone() {
            Adt14::C0(_, _, _, h4756, _, _, _, _, _, _, _) => {
                h4756
            }
        };
        let a4759 = match v1.clone() {
            Adt14::C0(_, _, _, h4757, _, _, _, _, _, _, _) => {
                h4757
            }
        };
        nat_eq(a4758, a4759)
    } else {
        false
    } {
        let a4762 = match v0 {
            Adt14::C0(_, _, _, _, h4760, _, _, _, _, _, _) => {
                h4760
            }
        };
        let a4763 = match v1 {
            Adt14::C0(_, _, _, _, h4761, _, _, _, _, _, _) => {
                h4761
            }
        };
        nat_eq(a4762, a4763)
    } else {
        false
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4764 = v0;
    let a4765 = 0u64;
    if if if nat_eq(a4764, a4765) {
        true
    } else {
        let a4766 = v1;
        let a4767 = 0u64;
        nat_eq(a4766, a4767)
    } {
        true
    } else {
        let a4768 = v2;
        let a4769 = 0u64;
        nat_eq(a4768, a4769)
    } {
        true
    } else {
        let a4770 = v3;
        let a4771 = 0u64;
        nat_eq(a4770, a4771)
    }
}

pub fn f187(v0: Adt14, v1: Adt14) -> bool {
    if f193(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4772, _, _, _, _, _) => {
            h4772
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4773, _, _, _, _, _) => {
            h4773
        }
    }) {
        let a4775 = match v0 {
            Adt14::C0(_, h4774, _, _, _, _, _, _, _, _, _) => {
                h4774
            }
        };
        let a4776 = 2147483647u64;
        nat_le(a4775, a4776)
    } else {
        true
    }
}

pub fn f188(v0: Adt14) -> bool {
    let a4781 = {
        let a4778 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4777) => {
                h4777
            }
        };
        let a4779 = 4u64;
        let a4780 = 0u64;
        nat_rem(a4778, a4779, a4780)
    };
    let a4782 = 0u64;
    if if nat_eq(a4781, a4782) {
        f232(match v0.clone() {
            Adt14::C0(_, h4783, _, _, _, _, _, _, _, _, _) => {
                h4783
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4784, _, _, _, _, _) => {
                h4784
            }
        })
    } else {
        false
    } {
        f104(match v0.clone() {
            Adt14::C0(_, _, h4785, _, _, _, _, _, _, _, _) => {
                h4785
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4786, _, _, _, _, _, _, _) => {
                h4786
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4787, _, _, _, _, _, _) => {
                h4787
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4788, _, _, _, _) => {
                h4788
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4789, _, _, _) => {
                h4789
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4790, _, _) => {
                h4790
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f189(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4792 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4791, _) => {
            h4791
        }
    };
    let a4793 = 0u64;
    if nat_eq(a4792, a4793) {
        Ok(true)
    } else {
        let a4795 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4794, _) => {
                h4794
            }
        };
        let a4796 = 0u64;
        if nat_eq(a4795, a4796) {
            Ok(true)
        } else {
            let a4798 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4797) => {
                    h4797
                }
            };
            let a4799 = 0u64;
            if nat_eq(a4798, a4799) {
                Ok(true)
            } else {
                let a4801 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4800) => {
                        h4800
                    }
                };
                let a4802 = 0u64;
                if nat_eq(a4801, a4802) {
                    Ok(true)
                } else {
                    let a4808 = {
                        let a4805 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4803) => {
                                h4803
                            }
                        };
                        let a4806 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4804, _) => {
                                h4804
                            }
                        };
                        nat_add(a4805, a4806)?
                    };
                    let a4809 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4807) => {
                            h4807
                        }
                    };
                    if nat_le(a4808, a4809) {
                        Ok(true)
                    } else {
                        let a4815 = {
                            let a4812 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4810) => {
                                    h4810
                                }
                            };
                            let a4813 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4811, _) => {
                                    h4811
                                }
                            };
                            nat_add(a4812, a4813)?
                        };
                        let a4816 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4814) => {
                                h4814
                            }
                        };
                        Ok(nat_le(a4815, a4816))
                    }
                }
            }
        }
    }
}

pub fn f190(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f185(v0.clone(), v1.clone()) {
        let a4819 = match v0.clone() {
            Adt14::C0(h4817, _, _, _, _, _, _, _, _, _, _) => {
                h4817
            }
        };
        let a4820 = match v1.clone() {
            Adt14::C0(h4818, _, _, _, _, _, _, _, _, _, _) => {
                h4818
            }
        };
        nat_eq(a4819, a4820)
    } else {
        false
    } {
        let a4823 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4821) => {
                h4821
            }
        };
        let a4824 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4822) => {
                h4822
            }
        };
        nat_eq(a4823, a4824)
    } else {
        false
    } {
        let a4827 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4825, _, _, _, _, _) => {
                h4825
            }
        };
        let a4828 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4826, _, _, _, _, _) => {
                h4826
            }
        };
        nat_eq(a4827, a4828)
    } else {
        false
    } {
        let a4831 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4829, _, _, _, _) => {
                h4829
            }
        };
        let a4832 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4830, _, _, _, _) => {
                h4830
            }
        };
        nat_eq(a4831, a4832)
    } else {
        false
    } {
        let a4835 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4833, _, _, _) => {
                h4833
            }
        };
        let a4836 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4834, _, _, _) => {
                h4834
            }
        };
        nat_eq(a4835, a4836)
    } else {
        false
    } {
        let a4839 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4837, _, _) => {
                h4837
            }
        };
        let a4840 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4838, _, _) => {
                h4838
            }
        };
        nat_eq(a4839, a4840)
    } else {
        false
    }
}

pub fn f191(v0: Adt14) -> R<bool> {
    if f233(match v0.clone() {
        Adt14::C0(_, h4841, _, _, _, _, _, _, _, _, _) => {
            h4841
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4842, _, _, _, _, _) => {
            h4842
        }
    }) {
        f105(f234(match v0.clone() {
            Adt14::C0(_, h4843, _, _, _, _, _, _, _, _, _) => {
                h4843
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4844, _, _, _, _, _) => {
                h4844
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4845, _, _, _, _, _, _, _, _) => {
                h4845
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4846, _, _, _, _, _, _, _) => {
                h4846
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4847, _, _, _, _, _, _) => {
                h4847
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4848, _, _, _, _) => {
                h4848
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4849, _, _, _) => {
                h4849
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4850, _, _) => {
                h4850
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f192(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4851 = v1;
    let a4852 = 0u64;
    if if nat_eq(a4851, a4852) {
        true
    } else {
        let a4853 = v2;
        let a4854 = 0u64;
        nat_eq(a4853, a4854)
    } {
        Ok(0u64)
    } else {
        let a4857 = f106(v1, v2, v3)?;
        let a4858 = {
            let a4855 = v2;
            let a4856 = f235(v0, v1)?;
            nat_mul(a4855, a4856)?
        };
        nat_add(a4857, a4858)
    }
}

pub fn f193(v0: u64, v1: u64) -> bool {
    let a4859 = v0;
    let a4860 = 4u64;
    if nat_eq(a4859, a4860) {
        let a4861 = v1;
        let a4862 = 4u64;
        nat_eq(a4861, a4862)
    } else {
        false
    }
}

pub fn f194(v0: u64) -> R<u64> {
    let a4863 = 6u64;
    let a4864 = v0;
    nat_add(a4863, a4864)
}

pub fn f195(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4865, _, _, _, _, _, _, _, _, _, _) => {
            h4865
        }
    }, match v0.clone() {
        Adt14::C0(_, h4866, _, _, _, _, _, _, _, _, _) => {
            h4866
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4867, _, _, _, _, _, _, _, _) => {
            h4867
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4868, _, _, _, _, _, _, _) => {
            h4868
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4869, _, _, _, _, _, _) => {
            h4869
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4870, _, _, _, _) => {
            h4870
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4871, _, _, _) => {
            h4871
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4872, _, _) => {
            h4872
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4873, _) => {
            h4873
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4874) => {
            h4874
        }
    })
}

pub fn f196(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4875 = v0;
    let a4876 = 2147483647u64;
    if if if nat_le(a4875, a4876) {
        let a4877 = 0u64;
        let a4878 = v3;
        if nat_lt(a4877, a4878) {
            let a4879 = v3;
            let a4880 = 2147483647u64;
            nat_le(a4879, a4880)
        } else {
            false
        }
    } else {
        false
    } {
        let a4885 = {
            let a4883 = {
                let a4881 = v2;
                let a4882 = v3;
                nat_add(a4881, a4882)?
            };
            let a4884 = 1u64;
            nat_sub(a4883, a4884)
        };
        let a4886 = 2147483647u64;
        nat_le(a4885, a4886)
    } else {
        false
    } {
        let a4891 = {
            let a4889 = {
                let a4887 = v0;
                let a4888 = v1;
                nat_mul(a4887, a4888)?
            };
            let a4890 = f236(v2, v3)?;
            nat_mul(a4889, a4890)?
        };
        let a4892 = 9223372036854775807u64;
        Ok(nat_le(a4891, a4892))
    } else {
        Ok(false)
    }
}

pub fn f197(v0: Adt14) -> R<u64> {
    let a4923 = {
        let a4915 = {
            let a4907 = {
                let a4899 = 4u64;
                let a4900 = {
                    let a4897 = {
                        let a4894 = match v0.clone() {
                            Adt14::C0(_, h4893, _, _, _, _, _, _, _, _, _) => {
                                h4893
                            }
                        };
                        let a4895 = 1u64;
                        nat_sub(a4894, a4895)
                    };
                    let a4898 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4896, _, _, _, _, _) => {
                            h4896
                        }
                    };
                    nat_mul(a4897, a4898)?
                };
                nat_add(a4899, a4900)?
            };
            let a4908 = {
                let a4905 = {
                    let a4902 = match v0.clone() {
                        Adt14::C0(_, _, h4901, _, _, _, _, _, _, _, _) => {
                            h4901
                        }
                    };
                    let a4903 = 1u64;
                    nat_sub(a4902, a4903)
                };
                let a4906 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4904, _, _, _, _) => {
                        h4904
                    }
                };
                nat_mul(a4905, a4906)?
            };
            nat_add(a4907, a4908)?
        };
        let a4916 = {
            let a4913 = {
                let a4910 = match v0.clone() {
                    Adt14::C0(_, _, _, h4909, _, _, _, _, _, _, _) => {
                        h4909
                    }
                };
                let a4911 = 1u64;
                nat_sub(a4910, a4911)
            };
            let a4914 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4912, _, _, _) => {
                    h4912
                }
            };
            nat_mul(a4913, a4914)?
        };
        nat_add(a4915, a4916)?
    };
    let a4924 = {
        let a4921 = {
            let a4918 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4917, _, _, _, _, _, _) => {
                    h4917
                }
            };
            let a4919 = 1u64;
            nat_sub(a4918, a4919)
        };
        let a4922 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4920, _, _) => {
                h4920
            }
        };
        nat_mul(a4921, a4922)?
    };
    nat_add(a4923, a4924)
}

pub fn f198(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4925 = v0;
    let a4926 = 0u64;
    if if if if nat_eq(a4925, a4926) {
        true
    } else {
        let a4927 = v1;
        let a4928 = 0u64;
        nat_eq(a4927, a4928)
    } {
        true
    } else {
        let a4929 = v2;
        let a4930 = 0u64;
        nat_eq(a4929, a4930)
    } {
        true
    } else {
        let a4931 = v3;
        let a4932 = 0u64;
        nat_eq(a4931, a4932)
    } {
        Ok(0u64)
    } else {
        let a4933 = f237(v0, v1, v2, v3, v5)?;
        let a4934 = f238(v0, v1, v2, v3, v4)?;
        nat_add(a4933, a4934)
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4935 = v0;
    let a4936 = 0u64;
    if if nat_eq(a4935, a4936) {
        true
    } else {
        let a4937 = v1;
        let a4938 = 0u64;
        nat_eq(a4937, a4938)
    } {
        Ok(0u64)
    } else {
        let a4941 = f106(v0, v1, v2)?;
        let a4942 = {
            let a4939 = v1;
            let a4940 = f239(v0)?;
            nat_mul(a4939, a4940)?
        };
        nat_add(a4941, a4942)
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> bool {
    let a4943 = v0;
    let a4944 = 1u64;
    if nat_le(a4943, a4944) {
        true
    } else {
        let a4948 = {
            let a4945 = v1;
            let a4946 = v2;
            let a4947 = 1u64;
            nat_rem(a4945, a4946, a4947)
        };
        let a4949 = 0u64;
        nat_eq(a4948, a4949)
    }
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4950 = v1;
    let a4951 = 1u64;
    if if if nat_le(a4950, a4951) {
        true
    } else {
        let a4952 = v0;
        let a4953 = v2;
        nat_le(a4952, a4953)
    } {
        let a4954 = v3;
        let a4955 = 1u64;
        if nat_le(a4954, a4955) {
            true
        } else {
            let a4962 = {
                let a4960 = v0;
                let a4961 = {
                    let a4958 = {
                        let a4956 = v1;
                        let a4957 = 1u64;
                        nat_sub(a4956, a4957)
                    };
                    let a4959 = v2;
                    nat_mul(a4958, a4959)?
                };
                nat_add(a4960, a4961)?
            };
            let a4963 = v4;
            nat_le(a4962, a4963)
        }
    } else {
        false
    } {
        let a4964 = v5;
        let a4965 = 1u64;
        if nat_le(a4964, a4965) {
            Ok(true)
        } else {
            let a4978 = {
                let a4976 = {
                    let a4970 = v0;
                    let a4971 = {
                        let a4968 = {
                            let a4966 = v1;
                            let a4967 = 1u64;
                            nat_sub(a4966, a4967)
                        };
                        let a4969 = v2;
                        nat_mul(a4968, a4969)?
                    };
                    nat_add(a4970, a4971)?
                };
                let a4977 = {
                    let a4974 = {
                        let a4972 = v3;
                        let a4973 = 1u64;
                        nat_sub(a4972, a4973)
                    };
                    let a4975 = v4;
                    nat_mul(a4974, a4975)?
                };
                nat_add(a4976, a4977)?
            };
            let a4979 = v6;
            Ok(nat_le(a4978, a4979))
        }
    } else {
        Ok(false)
    }
}

pub fn f202(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f240(v0, v1, v6, v7, v8) {
        let a4980 = v5;
        let a4981 = 26u64;
        nat_eq(a4980, a4981)
    } else {
        false
    } {
        let a4982 = v3;
        let a4983 = v7;
        nat_eq(a4982, a4983)
    } else {
        false
    } {
        let a4984 = v4;
        let a4985 = v8;
        nat_eq(a4984, a4985)
    } else {
        false
    } {
        let a4986 = v9;
        let a4987 = 1u64;
        nat_eq(a4986, a4987)
    } else {
        false
    } {
        let a4988 = v10;
        let a4989 = f241(v0);
        nat_eq(a4988, a4989)
    } else {
        false
    } {
        let a4990 = v11;
        let a4991 = v1;
        nat_eq(a4990, a4991)
    } else {
        false
    } {
        let a4992 = v12;
        let a4993 = v6;
        nat_eq(a4992, a4993)
    } else {
        false
    } {
        let a4994 = v13;
        let a4995 = v7;
        nat_eq(a4994, a4995)
    } else {
        false
    } {
        let a4996 = v14;
        let a4997 = v8;
        nat_eq(a4996, a4997)
    } else {
        false
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4998 = v0;
    let a4999 = 0u64;
    if if if nat_eq(a4998, a4999) {
        true
    } else {
        let a5000 = v1;
        let a5001 = 0u64;
        nat_eq(a5000, a5001)
    } {
        true
    } else {
        let a5002 = v2;
        let a5003 = 0u64;
        nat_eq(a5002, a5003)
    } {
        true
    } else {
        let a5004 = v3;
        let a5005 = 0u64;
        nat_eq(a5004, a5005)
    }
}

pub fn f204(v0: u64) -> u64 {
    let m5006 = f124(v0);
    match m5006 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h5007, _) => {
                    h5007
                }
            }
        }
    }
}

pub fn f205(v0: Adt3, v1: bool) -> R<u64> {
    f242(v0.clone(), {
        let a5009 = match v0.clone() {
            Adt3::C0(h5008, _, _, _, _, _, _, _) => {
                h5008
            }
        };
        let a5010 = 1u64;
        nat_sub(a5009, a5010)
    }, {
        let a5012 = match v0.clone() {
            Adt3::C0(_, h5011, _, _, _, _, _, _) => {
                h5011
            }
        };
        let a5013 = 1u64;
        nat_sub(a5012, a5013)
    }, {
        let a5015 = match v0.clone() {
            Adt3::C0(_, _, h5014, _, _, _, _, _) => {
                h5014
            }
        };
        let a5016 = 1u64;
        nat_sub(a5015, a5016)
    }, {
        let a5018 = match v0 {
            Adt3::C0(_, _, _, h5017, _, _, _, _) => {
                h5017
            }
        };
        let a5019 = 1u64;
        nat_sub(a5018, a5019)
    }, v1)
}

pub fn f206(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5020 = v0;
    let a5021 = 0u64;
    if nat_eq(a5020, a5021) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5022 = v0;
            let a5023 = 1u64;
            nat_sub(a5022, a5023)
        };
        let v5: u64 = {
            let a5024 = v4;
            let a5025 = v1;
            let a5026 = 0u64;
            nat_quot(a5024, a5025, a5026)
        };
        let v6: u64 = {
            let a5034 = {
                let a5027 = v5;
                let a5028 = v2;
                nat_mul(a5027, a5028)?
            };
            let a5035 = {
                let a5032 = {
                    let a5029 = v4;
                    let a5030 = v1;
                    let a5031 = 0u64;
                    nat_rem(a5029, a5030, a5031)
                };
                let a5033 = v3;
                nat_mul(a5032, a5033)?
            };
            nat_add(a5034, a5035)?
        };
        let a5036 = v5;
        let a5037 = 0u64;
        if nat_eq(a5036, a5037) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5046 = {
                    let a5040 = {
                        let a5038 = v5;
                        let a5039 = 1u64;
                        nat_sub(a5038, a5039)
                    };
                    let a5041 = v2;
                    nat_mul(a5040, a5041)?
                };
                let a5047 = {
                    let a5044 = {
                        let a5042 = v1;
                        let a5043 = 1u64;
                        nat_sub(a5042, a5043)
                    };
                    let a5045 = v3;
                    nat_mul(a5044, a5045)?
                };
                nat_add(a5046, a5047)?
            };
            let a5048 = v6;
            let a5049 = v7;
            if nat_lt(a5048, a5049) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f207(v0: Adt3, v1: u64) -> bool {
    let a5050 = v1;
    let a5051 = 0u64;
    if !nat_eq(a5050, a5051) {
        true
    } else {
        let a5053 = match v0 {
            Adt3::C0(_, _, _, _, h5052, _, _, _) => {
                h5052
            }
        };
        let a5054 = 4u64;
        nat_eq(a5053, a5054)
    }
}

pub fn f208(v0: u64, v1: bool) -> R<u64> {
    let a5057 = {
        let a5055 = v0;
        let a5056 = 2u64;
        nat_mul(a5055, a5056)?
    };
    let a5058 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5057, a5058)
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5059 = f243(v0, v1);
    match m5059 {
        None => {
            if v5 {
                let m5060 = f244(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5060 {
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
                let a5061 = 4u64;
                let a5062 = v28;
                nat_le(a5061, a5062)
            };
            let a5063 = v2;
            let a5064 = 0u64;
            if if if if if nat_eq(a5063, a5064) {
                let a5065 = v4;
                let a5066 = 0u64;
                nat_eq(a5065, a5066)
            } else {
                false
            } {
                let a5067 = v6;
                let a5068 = v14;
                if nat_eq(a5067, a5068) {
                    let a5069 = v7;
                    let a5070 = v15;
                    nat_eq(a5069, a5070)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5071 = v8;
                let a5072 = v16;
                if nat_eq(a5071, a5072) {
                    let a5073 = v9;
                    let a5074 = v17;
                    nat_eq(a5073, a5074)
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
                        let a5075 = v3;
                        let a5076 = 0u64;
                        nat_eq(a5075, a5076)
                    } else {
                        false
                    } {
                        f127(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5081 = {
                        let a5077 = v6;
                        let a5078 = v7;
                        nat_mul(a5077, a5078)?
                    };
                    let a5082 = {
                        let a5079 = v8;
                        let a5080 = v9;
                        nat_mul(a5079, a5080)?
                    };
                    nat_mul(a5081, a5082)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5087 = {
        let a5085 = {
            let a5083 = v0;
            let a5084 = v1;
            nat_sub(a5083, a5084)
        };
        let a5086 = 1u64;
        nat_sub(a5085, a5086)
    };
    let a5088 = v2;
    nat_add(a5087, a5088)
}

pub fn f211(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f141({
        let a5089 = v0;
        let a5090 = v4;
        let a5091 = 0u64;
        nat_rem(a5089, a5090, a5091)
    }, {
        let a5092 = v1;
        let a5093 = v5;
        let a5094 = 0u64;
        nat_rem(a5092, a5093, a5094)
    }, {
        let a5095 = v2;
        let a5096 = v6;
        let a5097 = 0u64;
        nat_rem(a5095, a5096, a5097)
    }, {
        let a5098 = v3;
        let a5099 = v7;
        let a5100 = 0u64;
        nat_rem(a5098, a5099, a5100)
    }, v8, v9, v10, v11)
}

pub fn f212(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5101 = v1;
    let a5102 = 2147483648u64;
    if nat_lt(a5101, a5102) {
        let a5113 = {
            let a5111 = {
                let a5106 = {
                    let a5103 = v0;
                    let a5104 = v2;
                    let a5105 = 0u64;
                    nat_rem(a5103, a5104, a5105)
                };
                let a5107 = v2;
                nat_add(a5106, a5107)?
            };
            let a5112 = {
                let a5108 = v1;
                let a5109 = v2;
                let a5110 = 0u64;
                nat_rem(a5108, a5109, a5110)
            };
            nat_sub(a5111, a5112)
        };
        let a5114 = v2;
        let a5115 = 0u64;
        Ok(nat_rem(a5113, a5114, a5115))
    } else {
        let a5118 = {
            let a5116 = v0;
            let a5117 = f142(v1);
            nat_add(a5116, a5117)?
        };
        let a5119 = v2;
        let a5120 = 0u64;
        Ok(nat_rem(a5118, a5119, a5120))
    }
}

pub fn f213(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5123 = f143(v1);
    let a5124 = {
        let a5121 = v0;
        let a5122 = f142(v1);
        nat_add(a5121, a5122)?
    };
    if nat_le(a5123, a5124) {
        let a5125 = f214(v0, v1)?;
        let a5126 = v2;
        Ok(nat_lt(a5125, a5126))
    } else {
        Ok(false)
    }
}

pub fn f214(v0: u64, v1: u64) -> R<u64> {
    let a5129 = {
        let a5127 = v0;
        let a5128 = f142(v1);
        nat_add(a5127, a5128)?
    };
    let a5130 = f143(v1);
    Ok(nat_sub(a5129, a5130))
}

pub fn f215(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5131 = v0;
    let a5132 = v1;
    if nat_lt(a5131, a5132) {
        let a5133 = v1;
        let a5134 = v0;
        Ok(nat_sub(a5133, a5134))
    } else {
        let a5137 = v0;
        let a5138 = {
            let a5135 = v1;
            let a5136 = v2;
            nat_add(a5135, a5136)?
        };
        if nat_lt(a5137, a5138) {
            let a5139 = v0;
            let a5140 = v1;
            Ok(nat_sub(a5139, a5140))
        } else {
            let a5147 = {
                let a5143 = 2u64;
                let a5144 = {
                    let a5141 = v2;
                    let a5142 = 1u64;
                    nat_sub(a5141, a5142)
                };
                nat_mul(a5143, a5144)?
            };
            let a5148 = {
                let a5145 = v0;
                let a5146 = v1;
                nat_sub(a5145, a5146)
            };
            Ok(nat_sub(a5147, a5148))
        }
    }
}

pub fn f216(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5151 = {
        let a5149 = v2;
        let a5150 = f242(v1.clone(), f73(v0, v1.clone(), 0u64), f73(v0, v1.clone(), 1u64), f73(v0, v1.clone(), 2u64), f73(v0, v1.clone(), 3u64), false)?;
        nat_add(a5149, a5150)?
    };
    let a5152 = f242(v1.clone(), f73(v0, v1.clone(), 0u64), f73(v0, v1.clone(), 1u64), f73(v0, v1.clone(), 2u64), f73(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5151, a5152))
}

pub fn f217(v0: u64, v1: u64, v2: u64) -> bool {
    let a5153 = v0;
    let a5154 = 0u64;
    if nat_eq(a5153, a5154) {
        let a5155 = v2;
        let a5156 = v1;
        nat_le(a5155, a5156)
    } else {
        let a5157 = v0;
        let a5158 = 1u64;
        if nat_eq(a5157, a5158) {
            let a5159 = v2;
            let a5160 = v1;
            nat_lt(a5159, a5160)
        } else {
            let a5161 = v0;
            let a5162 = 2u64;
            if nat_eq(a5161, a5162) {
                let a5163 = v1;
                let a5164 = v2;
                nat_le(a5163, a5164)
            } else {
                let a5165 = v0;
                let a5166 = 3u64;
                if nat_eq(a5165, a5166) {
                    let a5167 = v1;
                    let a5168 = v2;
                    nat_lt(a5167, a5168)
                } else {
                    let a5169 = v1;
                    let a5170 = v2;
                    nat_eq(a5169, a5170)
                }
            }
        }
    }
}

pub fn f218(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5171 = v0;
    let a5172 = 0u64;
    if nat_eq(a5171, a5172) {
        Ok(f229(0u64))
    } else {
        let a5175 = {
            let a5173 = v0;
            let a5174 = 1u64;
            nat_add(a5173, a5174)?
        };
        let a5176 = f138(v1.clone(), 13u64, v2)?;
        if nat_eq(a5175, a5176) {
            Ok(Adt9::C4({
                let a5177 = v2;
                let a5178 = 0u64;
                if nat_eq(a5177, a5178) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5179 = v0;
                let a5180 = 1u64;
                nat_sub(a5179, a5180)
            };
            if f207(v1.clone(), v2) {
                let v4: u64 = f208(match v1.clone() {
                    Adt3::C0(h5181, _, _, _, _, _, _, _) => {
                        h5181
                    }
                }, false)?;
                let v5: u64 = {
                    let a5182 = v3;
                    let a5183 = v4;
                    let a5184 = 0u64;
                    nat_quot(a5182, a5183, a5184)
                };
                let v6: u64 = {
                    let a5185 = v3;
                    let a5186 = v4;
                    let a5187 = 0u64;
                    nat_rem(a5185, a5186, a5187)
                };
                let a5190 = {
                    let a5188 = v6;
                    let a5189 = 1u64;
                    nat_add(a5188, a5189)?
                };
                let a5191 = v4;
                if nat_eq(a5190, a5191) {
                    Ok(Adt9::C4({
                        let a5192 = v2;
                        let a5193 = 0u64;
                        if nat_eq(a5192, a5193) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f219(v6, v5, v1, v2)
                }
            } else {
                f245({
                    let a5194 = v3;
                    let a5195 = 2u64;
                    let a5196 = 0u64;
                    nat_quot(a5194, a5195, a5196)
                }, {
                    let a5197 = v3;
                    let a5198 = 2u64;
                    let a5199 = 0u64;
                    nat_rem(a5197, a5198, a5199)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f219(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5200 = v0;
    let a5201 = 0u64;
    if nat_eq(a5200, a5201) {
        Ok(f229(1u64))
    } else {
        f245({
            let a5210 = {
                let a5203 = v1;
                let a5204 = match v2.clone() {
                    Adt3::C0(h5202, _, _, _, _, _, _, _) => {
                        h5202
                    }
                };
                nat_mul(a5203, a5204)?
            };
            let a5211 = {
                let a5207 = {
                    let a5205 = v0;
                    let a5206 = 1u64;
                    nat_sub(a5205, a5206)
                };
                let a5208 = 2u64;
                let a5209 = 0u64;
                nat_quot(a5207, a5208, a5209)
            };
            nat_add(a5210, a5211)?
        }, {
            let a5214 = {
                let a5212 = v0;
                let a5213 = 1u64;
                nat_sub(a5212, a5213)
            };
            let a5215 = 2u64;
            let a5216 = 0u64;
            nat_rem(a5214, a5215, a5216)
        }, v2, v3, 1u64)
    }
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let a5217 = v0;
    let a5218 = f100(v1)?;
    let a5219 = 0u64;
    Ok(nat_rem(a5217, a5218, a5219))
}

pub fn f221(v0: u64, v1: u64) -> R<u64> {
    let a5220 = 4u64;
    let a5221 = f246(v0, v1)?;
    nat_mul(a5220, a5221)
}

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let a5222 = 4u64;
    let a5223 = f247(v0, v1)?;
    nat_mul(a5222, a5223)
}

pub fn f223(v0: u64, v1: u64) -> R<bool> {
    let a5229 = {
        let a5226 = {
            let a5224 = f220(v0, v1)?;
            let a5225 = 2u64;
            nat_sub(a5224, a5225)
        };
        let a5227 = 2u64;
        let a5228 = 0u64;
        nat_rem(a5226, a5227, a5228)
    };
    let a5230 = 0u64;
    Ok(nat_eq(a5229, a5230))
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5231 = f124(v0);
    match m5231 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f122(v0) {
                let a5232 = v1;
                let a5233 = v7;
                nat_lt(a5232, a5233)
            } else {
                false
            } {
                f172(v8, v2)
            } else {
                false
            } {
                let a5234 = v3;
                let a5235 = f248(v4, v5, v6)?;
                nat_lt(a5234, a5235)
            } else {
                false
            } {
                let a5240 = {
                    let a5237 = v7;
                    let a5238 = match v13.clone() {
                        Adt18::C0(_, h5236, _) => {
                            h5236
                        }
                    };
                    let a5239 = 0u64;
                    nat_rem(a5237, a5238, a5239)
                };
                let a5241 = 0u64;
                nat_eq(a5240, a5241)
            } else {
                false
            } {
                Ok(Some({
                    let a5270 = {
                        let a5260 = {
                            let a5250 = {
                                let a5246 = {
                                    let a5243 = v1;
                                    let a5244 = match v13 {
                                        Adt18::C0(_, h5242, _) => {
                                            h5242
                                        }
                                    };
                                    let a5245 = 0u64;
                                    nat_quot(a5243, a5244, a5245)
                                };
                                let a5247 = v9;
                                nat_mul(a5246, a5247)?
                            };
                            let a5251 = {
                                let a5248 = v2;
                                let a5249 = v10;
                                nat_mul(a5248, a5249)?
                            };
                            nat_add(a5250, a5251)?
                        };
                        let a5261 = {
                            let a5258 = {
                                let a5255 = {
                                    let a5252 = v3;
                                    let a5253 = v4;
                                    let a5254 = 0u64;
                                    nat_quot(a5252, a5253, a5254)
                                };
                                let a5256 = v5;
                                let a5257 = 0u64;
                                nat_rem(a5255, a5256, a5257)
                            };
                            let a5259 = v11;
                            nat_mul(a5258, a5259)?
                        };
                        nat_add(a5260, a5261)?
                    };
                    let a5271 = {
                        let a5268 = {
                            let a5265 = {
                                let a5262 = v3;
                                let a5263 = v4;
                                let a5264 = 0u64;
                                nat_quot(a5262, a5263, a5264)
                            };
                            let a5266 = v5;
                            let a5267 = 0u64;
                            nat_quot(a5265, a5266, a5267)
                        };
                        let a5269 = v12;
                        nat_mul(a5268, a5269)?
                    };
                    nat_add(a5270, a5271)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f249(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f250(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a5272 = v0;
    let a5273 = 64u64;
    if nat_eq(a5272, a5273) {
        f251(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5274 = v0;
        let a5275 = 65u64;
        if nat_eq(a5274, a5275) {
            f252(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f228(v0: u64) -> u64 {
    let a5276 = v0;
    let a5277 = 30u64;
    if nat_eq(a5276, a5277) {
        4u64
    } else {
        3u64
    }
}

pub fn f229(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f230(v0: u64) -> bool {
    let a5278 = 0u64;
    let a5279 = v0;
    if nat_lt(a5278, a5279) {
        let a5280 = v0;
        let a5281 = 2139095040u64;
        nat_le(a5280, a5281)
    } else {
        false
    }
}

pub fn f231(v0: u64) -> u64 {
    let a5282 = v0;
    let a5283 = 40u64;
    if nat_lt(a5282, a5283) {
        let a5284 = v0;
        let a5285 = 20u64;
        if nat_lt(a5284, a5285) {
            let a5286 = v0;
            let a5287 = 10u64;
            if nat_lt(a5286, a5287) {
                let a5288 = v0;
                let a5289 = 5u64;
                if nat_lt(a5288, a5289) {
                    let a5290 = v0;
                    let a5291 = 2u64;
                    if nat_lt(a5290, a5291) {
                        let a5292 = v0;
                        let a5293 = 1u64;
                        if nat_lt(a5292, a5293) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5294 = v0;
                        let a5295 = 3u64;
                        if nat_lt(a5294, a5295) {
                            4u64
                        } else {
                            let a5296 = v0;
                            let a5297 = 4u64;
                            if nat_lt(a5296, a5297) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5298 = v0;
                    let a5299 = 7u64;
                    if nat_lt(a5298, a5299) {
                        let a5300 = v0;
                        let a5301 = 6u64;
                        if nat_lt(a5300, a5301) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5302 = v0;
                        let a5303 = 8u64;
                        if nat_lt(a5302, a5303) {
                            128u64
                        } else {
                            let a5304 = v0;
                            let a5305 = 9u64;
                            if nat_lt(a5304, a5305) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5306 = v0;
                let a5307 = 15u64;
                if nat_lt(a5306, a5307) {
                    let a5308 = v0;
                    let a5309 = 12u64;
                    if nat_lt(a5308, a5309) {
                        let a5310 = v0;
                        let a5311 = 11u64;
                        if nat_lt(a5310, a5311) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5312 = v0;
                        let a5313 = 13u64;
                        if nat_lt(a5312, a5313) {
                            4096u64
                        } else {
                            let a5314 = v0;
                            let a5315 = 14u64;
                            if nat_lt(a5314, a5315) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5316 = v0;
                    let a5317 = 17u64;
                    if nat_lt(a5316, a5317) {
                        let a5318 = v0;
                        let a5319 = 16u64;
                        if nat_lt(a5318, a5319) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5320 = v0;
                        let a5321 = 18u64;
                        if nat_lt(a5320, a5321) {
                            131072u64
                        } else {
                            let a5322 = v0;
                            let a5323 = 19u64;
                            if nat_lt(a5322, a5323) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5324 = v0;
            let a5325 = 30u64;
            if nat_lt(a5324, a5325) {
                let a5326 = v0;
                let a5327 = 25u64;
                if nat_lt(a5326, a5327) {
                    let a5328 = v0;
                    let a5329 = 22u64;
                    if nat_lt(a5328, a5329) {
                        let a5330 = v0;
                        let a5331 = 21u64;
                        if nat_lt(a5330, a5331) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5332 = v0;
                        let a5333 = 23u64;
                        if nat_lt(a5332, a5333) {
                            4194304u64
                        } else {
                            let a5334 = v0;
                            let a5335 = 24u64;
                            if nat_lt(a5334, a5335) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5336 = v0;
                    let a5337 = 27u64;
                    if nat_lt(a5336, a5337) {
                        let a5338 = v0;
                        let a5339 = 26u64;
                        if nat_lt(a5338, a5339) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5340 = v0;
                        let a5341 = 28u64;
                        if nat_lt(a5340, a5341) {
                            134217728u64
                        } else {
                            let a5342 = v0;
                            let a5343 = 29u64;
                            if nat_lt(a5342, a5343) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5344 = v0;
                let a5345 = 35u64;
                if nat_lt(a5344, a5345) {
                    let a5346 = v0;
                    let a5347 = 32u64;
                    if nat_lt(a5346, a5347) {
                        let a5348 = v0;
                        let a5349 = 31u64;
                        if nat_lt(a5348, a5349) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5350 = v0;
                        let a5351 = 33u64;
                        if nat_lt(a5350, a5351) {
                            4294967296u64
                        } else {
                            let a5352 = v0;
                            let a5353 = 34u64;
                            if nat_lt(a5352, a5353) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5354 = v0;
                    let a5355 = 37u64;
                    if nat_lt(a5354, a5355) {
                        let a5356 = v0;
                        let a5357 = 36u64;
                        if nat_lt(a5356, a5357) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5358 = v0;
                        let a5359 = 38u64;
                        if nat_lt(a5358, a5359) {
                            137438953472u64
                        } else {
                            let a5360 = v0;
                            let a5361 = 39u64;
                            if nat_lt(a5360, a5361) {
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

pub fn f232(v0: u64, v1: u64) -> bool {
    let a5362 = v0;
    let a5363 = 1u64;
    if nat_le(a5362, a5363) {
        true
    } else {
        let a5367 = {
            let a5364 = v1;
            let a5365 = 4u64;
            let a5366 = 0u64;
            nat_rem(a5364, a5365, a5366)
        };
        let a5368 = 0u64;
        nat_eq(a5367, a5368)
    }
}

pub fn f233(v0: u64, v1: u64) -> bool {
    let a5369 = v0;
    let a5370 = 1u64;
    if nat_le(a5369, a5370) {
        true
    } else {
        let a5371 = 4u64;
        let a5372 = v1;
        nat_le(a5371, a5372)
    }
}

pub fn f234(v0: u64, v1: u64) -> R<u64> {
    let a5373 = v0;
    let a5374 = 0u64;
    if nat_eq(a5373, a5374) {
        Ok(0u64)
    } else {
        let a5379 = 4u64;
        let a5380 = {
            let a5377 = {
                let a5375 = v0;
                let a5376 = 1u64;
                nat_sub(a5375, a5376)
            };
            let a5378 = v1;
            nat_mul(a5377, a5378)?
        };
        nat_add(a5379, a5380)
    }
}

pub fn f235(v0: bool, v1: u64) -> R<u64> {
    let a5385 = {
        let a5383 = {
            let a5381 = f253(v0, v1)?;
            let a5382 = f254(v0, v1)?;
            nat_add(a5381, a5382)?
        };
        let a5384 = f255(v0);
        nat_add(a5383, a5384)?
    };
    let a5386 = v1;
    nat_add(a5385, a5386)
}

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f256(v0, v1)?;
    let v3: u64 = {
        let a5391 = {
            let a5389 = {
                let a5387 = v1;
                let a5388 = 1u64;
                nat_sub(a5387, a5388)
            };
            let a5390 = v2;
            nat_mul(a5389, a5390)?
        };
        let a5392 = v0;
        nat_sub(a5391, a5392)
    };
    let a5393 = v2;
    let a5394 = v3;
    if nat_lt(a5393, a5394) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f237(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5395 = v4;
    let a5396 = 0u64;
    if nat_eq(a5395, a5396) {
        Ok(0u64)
    } else {
        let a5403 = 2u64;
        let a5404 = {
            let a5401 = {
                let a5397 = v0;
                let a5398 = v1;
                nat_mul(a5397, a5398)?
            };
            let a5402 = {
                let a5399 = v2;
                let a5400 = v3;
                nat_mul(a5399, a5400)?
            };
            nat_mul(a5401, a5402)?
        };
        nat_mul(a5403, a5404)
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5409 = v3;
    let a5410 = {
        let a5407 = {
            let a5405 = f257(v2, v4)?;
            let a5406 = f258(v0, v1, f256(v2, v4)?)?;
            nat_mul(a5405, a5406)?
        };
        let a5408 = f258(v0, v1, f259(v2, v4)?)?;
        nat_add(a5407, a5408)?
    };
    nat_mul(a5409, a5410)
}

pub fn f239(v0: u64) -> R<u64> {
    let a5413 = 13u64;
    let a5414 = {
        let a5411 = 9u64;
        let a5412 = v0;
        nat_mul(a5411, a5412)?
    };
    nat_add(a5413, a5414)
}

pub fn f240(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f203(v1, v2, v3, v4) {
        f260(v0)
    } else {
        f122(v0)
    }
}

pub fn f241(v0: u64) -> u64 {
    let a5415 = v0;
    let a5416 = 26u64;
    if nat_eq(a5415, a5416) {
        26u64
    } else {
        0u64
    }
}

pub fn f242(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5437 = {
        let a5431 = {
            let a5425 = {
                let a5419 = v1;
                let a5420 = if v5 {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5417, _, _, _) => {
                            h5417
                        }
                    })
                } else {
                    f143(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5418, _, _, _) => {
                            h5418
                        }
                    })
                };
                nat_mul(a5419, a5420)?
            };
            let a5426 = {
                let a5423 = v2;
                let a5424 = if v5 {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5421, _, _) => {
                            h5421
                        }
                    })
                } else {
                    f143(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5422, _, _) => {
                            h5422
                        }
                    })
                };
                nat_mul(a5423, a5424)?
            };
            nat_add(a5425, a5426)?
        };
        let a5432 = {
            let a5429 = v3;
            let a5430 = if v5 {
                f142(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5427, _) => {
                        h5427
                    }
                })
            } else {
                f143(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5428, _) => {
                        h5428
                    }
                })
            };
            nat_mul(a5429, a5430)?
        };
        nat_add(a5431, a5432)?
    };
    let a5438 = {
        let a5435 = v4;
        let a5436 = if v5 {
            f142(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5433) => {
                    h5433
                }
            })
        } else {
            f143(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5434) => {
                    h5434
                }
            })
        };
        nat_mul(a5435, a5436)?
    };
    nat_add(a5437, a5438)
}

pub fn f243(v0: u64, v1: u64) -> Option<u64> {
    let a5439 = v0;
    let a5440 = 2u64;
    if nat_eq(a5439, a5440) {
        Some(0u64)
    } else {
        let a5441 = v0;
        let a5442 = 5u64;
        if nat_eq(a5441, a5442) {
            Some(1u64)
        } else {
            let a5443 = v0;
            let a5444 = 6u64;
            if nat_eq(a5443, a5444) {
                Some(2u64)
            } else {
                let a5445 = v0;
                let a5446 = 7u64;
                if nat_eq(a5445, a5446) {
                    Some(3u64)
                } else {
                    let a5447 = v0;
                    let a5448 = 9u64;
                    if nat_eq(a5447, a5448) {
                        Some(4u64)
                    } else {
                        let a5449 = v0;
                        let a5450 = 8u64;
                        if nat_eq(a5449, a5450) {
                            Some(5u64)
                        } else {
                            let a5451 = v0;
                            let a5452 = 89u64;
                            if nat_eq(a5451, a5452) {
                                let a5453 = v1;
                                let a5454 = 2u64;
                                if nat_eq(a5453, a5454) {
                                    Some(6u64)
                                } else {
                                    let a5455 = v1;
                                    let a5456 = 0u64;
                                    if nat_eq(a5455, a5456) {
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

pub fn f244(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5457 = v0;
    let a5458 = 30u64;
    if nat_eq(a5457, a5458) {
        let a5459 = v2;
        let a5460 = 0u64;
        if if if nat_eq(a5459, a5460) {
            true
        } else {
            let a5461 = v2;
            let a5462 = 1u64;
            nat_eq(a5461, a5462)
        } {
            let a5463 = v1;
            let a5464 = 0u64;
            if nat_eq(a5463, a5464) {
                true
            } else {
                f261(v1, v4)
            }
        } else {
            false
        } {
            f262(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f263(v1) {
            f263(v2)
        } else {
            false
        } {
            let a5465 = v1;
            let a5466 = 0u64;
            if if nat_eq(a5465, a5466) {
                true
            } else {
                let a5467 = v2;
                let a5468 = 0u64;
                nat_eq(a5467, a5468)
            } {
                true
            } else {
                let a5469 = v1;
                let a5470 = v2;
                nat_eq(a5469, a5470)
            }
        } else {
            false
        } {
            true
        } else {
            let a5471 = v1;
            let a5472 = 43u64;
            if nat_eq(a5471, a5472) {
                f263(v2)
            } else {
                false
            }
        } {
            f262(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f245(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5473 = v1;
    let a5474 = 0u64;
    if nat_eq(a5473, a5474) {
        Ok(Adt9::C4({
            let a5475 = v3;
            let a5476 = 0u64;
            if nat_eq(a5475, a5476) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f149(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5477 = v3;
            let a5478 = 0u64;
            if nat_eq(a5477, a5478) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f246(v0: u64, v1: u64) -> R<u64> {
    let a5479 = v0;
    let a5480 = f100(v1)?;
    let a5481 = 0u64;
    Ok(nat_quot(a5479, a5480, a5481))
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f246(v0, v1)?;
    let v3: u64 = {
        let a5484 = {
            let a5482 = f220(v0, v1)?;
            let a5483 = 2u64;
            nat_sub(a5482, a5483)
        };
        let a5485 = 2u64;
        let a5486 = 0u64;
        nat_quot(a5484, a5485, a5486)
    };
    let v4: u64 = {
        let a5487 = v2;
        let a5488 = v1;
        nat_mul(a5487, a5488)?
    };
    if f223(v0, v1)? {
        let a5489 = v4;
        let a5490 = v3;
        nat_add(a5489, a5490)
    } else {
        let a5502 = {
            let a5498 = v4;
            let a5499 = {
                let a5496 = {
                    let a5491 = v1;
                    let a5492 = 1u64;
                    nat_sub(a5491, a5492)
                };
                let a5497 = {
                    let a5493 = v2;
                    let a5494 = v1;
                    let a5495 = 0u64;
                    nat_rem(a5493, a5494, a5495)
                };
                nat_mul(a5496, a5497)?
            };
            nat_sub(a5498, a5499)
        };
        let a5503 = {
            let a5500 = v3;
            let a5501 = v1;
            nat_mul(a5500, a5501)?
        };
        nat_add(a5502, a5503)
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5504 = v0;
    let a5505 = 0u64;
    if if if nat_eq(a5504, a5505) {
        true
    } else {
        let a5506 = v1;
        let a5507 = 0u64;
        nat_eq(a5506, a5507)
    } {
        true
    } else {
        let a5508 = v2;
        let a5509 = 0u64;
        nat_eq(a5508, a5509)
    } {
        Ok(0u64)
    } else {
        let a5512 = {
            let a5510 = v0;
            let a5511 = v1;
            nat_mul(a5510, a5511)?
        };
        let a5513 = v2;
        nat_mul(a5512, a5513)
    }
}

pub fn f249(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f109(match v1.clone() {
        Adt20::C0(h5514, _, _, _, _, _, _, _) => {
            h5514
        }
    })?;
    let v6: u64 = {
        let a5520 = match v1.clone() {
            Adt20::C0(_, h5515, _, _, _, _, _, _) => {
                h5515
            }
        };
        let a5521 = {
            let a5518 = match v1.clone() {
                Adt20::C0(_, _, h5516, _, _, _, _, _) => {
                    h5516
                }
            };
            let a5519 = match v1.clone() {
                Adt20::C0(_, _, _, h5517, _, _, _, _) => {
                    h5517
                }
            };
            nat_mul(a5518, a5519)?
        };
        nat_mul(a5520, a5521)?
    };
    let a5523 = v0;
    let a5524 = f39(match v1.clone() {
        Adt20::C0(h5522, _, _, _, _, _, _, _) => {
            h5522
        }
    }, v6)?;
    if nat_lt(a5523, a5524) {
        f264({
            let a5525 = v0;
            let a5526 = v5;
            let a5527 = 0u64;
            nat_rem(a5525, a5526, a5527)
        }, {
            let a5528 = v0;
            let a5529 = v5;
            let a5530 = 0u64;
            nat_quot(a5528, a5529, a5530)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f250(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5531 = v6;
        let a5532 = 6u64;
        nat_sub(a5531, a5532)
    };
    let v9: Adt20 = f265(v3);
    let a5534 = v0;
    let a5535 = f199(match v1.clone() {
        Adt20::C0(h5533, _, _, _, _, _, _, _) => {
            h5533
        }
    }, v5, v8)?;
    if nat_lt(a5534, a5535) {
        let v10: u64 = f106(match v1.clone() {
            Adt20::C0(h5536, _, _, _, _, _, _, _) => {
                h5536
            }
        }, v5, v8)?;
        let a5537 = v0;
        let a5538 = v10;
        if nat_lt(a5537, a5538) {
            f266(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5539 = v0;
                let a5540 = v10;
                nat_sub(a5539, a5540)
            };
            let v12: u64 = f239(match v1.clone() {
                Adt20::C0(h5541, _, _, _, _, _, _, _) => {
                    h5541
                }
            })?;
            f267({
                let a5542 = v11;
                let a5543 = v12;
                let a5544 = 0u64;
                nat_rem(a5542, a5543, a5544)
            }, {
                let a5545 = v11;
                let a5546 = v12;
                let a5547 = 0u64;
                nat_quot(a5545, a5546, a5547)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f251(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5548 = v5;
        let a5549 = 6u64;
        nat_sub(a5548, a5549)
    };
    let v8: bool = f193(match v1.clone() {
        Adt20::C0(_, _, _, _, h5550, _, _, _) => {
            h5550
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5551, _, _, _) => {
            h5551
        }
    });
    let a5553 = v0;
    let a5554 = f192(v8, match v1.clone() {
        Adt20::C0(h5552, _, _, _, _, _, _, _) => {
            h5552
        }
    }, v4, v7)?;
    if nat_lt(a5553, a5554) {
        let v9: u64 = f106(match v1.clone() {
            Adt20::C0(h5555, _, _, _, _, _, _, _) => {
                h5555
            }
        }, v4, v7)?;
        let a5556 = v0;
        let a5557 = v9;
        if nat_lt(a5556, a5557) {
            f266(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5558 = v0;
                let a5559 = v9;
                nat_sub(a5558, a5559)
            };
            let v11: u64 = f235(v8, match v1.clone() {
                Adt20::C0(h5560, _, _, _, _, _, _, _) => {
                    h5560
                }
            })?;
            f268({
                let a5561 = v10;
                let a5562 = v11;
                let a5563 = 0u64;
                nat_rem(a5561, a5562, a5563)
            }, {
                let a5564 = v10;
                let a5565 = v11;
                let a5566 = 0u64;
                nat_quot(a5564, a5565, a5566)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f252(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5567 = v5;
        let a5568 = 6u64;
        nat_sub(a5567, a5568)
    };
    let v8: Adt20 = f265(v2);
    let v9: u64 = f237(match v1.clone() {
        Adt20::C0(h5569, _, _, _, _, _, _, _) => {
            h5569
        }
    }, match v1.clone() {
        Adt20::C0(_, h5570, _, _, _, _, _, _) => {
            h5570
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5571, _, _, _, _, _) => {
            h5571
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5572, _, _, _, _) => {
            h5572
        }
    }, v7)?;
    let a5577 = v0;
    let a5578 = f198(match v1.clone() {
        Adt20::C0(h5573, _, _, _, _, _, _, _) => {
            h5573
        }
    }, match v1.clone() {
        Adt20::C0(_, h5574, _, _, _, _, _, _) => {
            h5574
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5575, _, _, _, _, _) => {
            h5575
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5576, _, _, _, _) => {
            h5576
        }
    }, v4, v7)?;
    if nat_lt(a5577, a5578) {
        let a5579 = v0;
        let a5580 = v9;
        if nat_lt(a5579, a5580) {
            f266(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f269({
                let a5581 = v0;
                let a5582 = v9;
                nat_sub(a5581, a5582)
            }, match v1.clone() {
                Adt20::C0(h5583, _, _, _, _, _, _, _) => {
                    h5583
                }
            }, match v1.clone() {
                Adt20::C0(_, h5584, _, _, _, _, _, _) => {
                    h5584
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5585, _, _, _, _, _) => {
                    h5585
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5586, _, _, _, _) => {
                    h5586
                }
            }, v4)?;
            f270(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f253(v0: bool, v1: u64) -> R<u64> {
    let a5589 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5590 = {
        let a5587 = f271(v0);
        let a5588 = v1;
        nat_mul(a5587, a5588)?
    };
    nat_add(a5589, a5590)
}

pub fn f254(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5598 = {
            let a5591 = 13u64;
            let a5592 = f272(v1);
            nat_mul(a5591, a5592)?
        };
        let a5599 = {
            let a5596 = 4u64;
            let a5597 = {
                let a5593 = v1;
                let a5594 = 4u64;
                let a5595 = 0u64;
                nat_rem(a5593, a5594, a5595)
            };
            nat_mul(a5596, a5597)?
        };
        nat_add(a5598, a5599)
    } else {
        let a5600 = 3u64;
        let a5601 = v1;
        nat_mul(a5600, a5601)
    }
}

pub fn f255(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f256(v0: u64, v1: u64) -> R<u64> {
    let a5606 = {
        let a5604 = {
            let a5602 = v0;
            let a5603 = v1;
            nat_add(a5602, a5603)?
        };
        let a5605 = 1u64;
        nat_sub(a5604, a5605)
    };
    let a5607 = v1;
    let a5608 = 0u64;
    Ok(nat_quot(a5606, a5607, a5608))
}

pub fn f257(v0: u64, v1: u64) -> R<u64> {
    let a5609 = v0;
    let a5610 = f256(v0, v1)?;
    let a5611 = 0u64;
    Ok(nat_quot(a5609, a5610, a5611))
}

pub fn f258(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5612 = v2;
    let a5613 = 0u64;
    if nat_eq(a5612, a5613) {
        Ok(0u64)
    } else {
        let a5618 = 11u64;
        let a5619 = {
            let a5616 = {
                let a5614 = v1;
                let a5615 = v2;
                nat_mul(a5614, a5615)?
            };
            let a5617 = f273(v0)?;
            nat_mul(a5616, a5617)?
        };
        nat_add(a5618, a5619)
    }
}

pub fn f259(v0: u64, v1: u64) -> R<u64> {
    let a5620 = v0;
    let a5621 = f256(v0, v1)?;
    let a5622 = 0u64;
    Ok(nat_rem(a5620, a5621, a5622))
}

pub fn f260(v0: u64) -> bool {
    let m5623 = f124(v0);
    match m5623 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f261(v0: u64, v1: u64) -> bool {
    let a5624 = v0;
    let a5625 = 1u64;
    if nat_eq(a5624, a5625) {
        true
    } else {
        let v2: u64 = f274(v0);
        let a5626 = 0u64;
        let a5627 = v2;
        if nat_lt(a5626, a5627) {
            let a5631 = {
                let a5628 = v1;
                let a5629 = v2;
                let a5630 = 1u64;
                nat_rem(a5628, a5629, a5630)
            };
            let a5632 = 0u64;
            nat_eq(a5631, a5632)
        } else {
            false
        }
    }
}

pub fn f262(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5633 = v0;
    let a5634 = 28u64;
    if if if if nat_eq(a5633, a5634) {
        let a5635 = v1;
        let a5636 = 0u64;
        nat_eq(a5635, a5636)
    } else {
        false
    } {
        let a5637 = v2;
        let a5638 = 0u64;
        nat_eq(a5637, a5638)
    } else {
        false
    } {
        let a5639 = v3;
        let a5640 = 0u64;
        nat_eq(a5639, a5640)
    } else {
        false
    } {
        let a5641 = v4;
        let a5642 = v8;
        if if if if if if if if nat_eq(a5641, a5642) {
            let a5643 = v5;
            let a5644 = v12;
            nat_eq(a5643, a5644)
        } else {
            false
        } {
            let a5645 = v9;
            let a5646 = v13;
            nat_eq(a5645, a5646)
        } else {
            false
        } {
            let a5647 = v10;
            let a5648 = v14;
            nat_eq(a5647, a5648)
        } else {
            false
        } {
            let a5649 = v11;
            let a5650 = v15;
            nat_eq(a5649, a5650)
        } else {
            false
        } {
            let a5651 = 0u64;
            let a5652 = v6;
            nat_lt(a5651, a5652)
        } else {
            false
        } {
            let a5653 = 0u64;
            let a5654 = v7;
            nat_lt(a5653, a5654)
        } else {
            false
        } {
            let a5658 = {
                let a5655 = v10;
                let a5656 = v6;
                let a5657 = 1u64;
                nat_rem(a5655, a5656, a5657)
            };
            let a5659 = 0u64;
            if nat_eq(a5658, a5659) {
                let a5663 = {
                    let a5660 = v11;
                    let a5661 = v7;
                    let a5662 = 1u64;
                    nat_rem(a5660, a5661, a5662)
                };
                let a5664 = 0u64;
                nat_eq(a5663, a5664)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5665 = v10;
                let a5666 = v6;
                let a5667 = 0u64;
                nat_quot(a5665, a5666, a5667)
            }, {
                let a5668 = v11;
                let a5669 = v7;
                let a5670 = 0u64;
                nat_quot(a5668, a5669, a5670)
            }, {
                let a5671 = v5;
                let a5672 = v4;
                nat_mul(a5671, a5672)?
            }, {
                let a5673 = v4;
                let a5674 = v9;
                nat_mul(a5673, a5674)?
            }, {
                let a5675 = v5;
                let a5676 = v9;
                nat_mul(a5675, a5676)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f263(v0: u64) -> bool {
    let a5677 = v0;
    let a5678 = 0u64;
    if if nat_eq(a5677, a5678) {
        true
    } else {
        let a5679 = v0;
        let a5680 = 1u64;
        nat_eq(a5679, a5680)
    } {
        true
    } else {
        let a5681 = v0;
        let a5682 = 30u64;
        nat_eq(a5681, a5682)
    }
}

pub fn f264(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5683, _, _, _, _, _, _, _) => {
            h5683
        }
    };
    let v7: u64 = {
        let a5686 = 1u64;
        let a5687 = {
            let a5684 = 3u64;
            let a5685 = v6;
            nat_mul(a5684, a5685)?
        };
        nat_add(a5686, a5687)?
    };
    let a5688 = v0;
    let a5689 = 0u64;
    if nat_eq(a5688, a5689) {
        Ok(f275(0u64))
    } else {
        let a5690 = v0;
        let a5691 = v7;
        if nat_lt(a5690, a5691) {
            let v8: u64 = {
                let a5692 = v0;
                let a5693 = 1u64;
                nat_sub(a5692, a5693)
            };
            let v9: u64 = {
                let a5699 = {
                    let a5694 = v1;
                    let a5695 = v6;
                    nat_mul(a5694, a5695)?
                };
                let a5700 = {
                    let a5696 = v8;
                    let a5697 = 3u64;
                    let a5698 = 0u64;
                    nat_quot(a5696, a5697, a5698)
                };
                nat_add(a5699, a5700)?
            };
            let v10: u64 = f276(v9, v2)?;
            let v11: u64 = {
                let a5701 = v8;
                let a5702 = 3u64;
                let a5703 = 0u64;
                nat_rem(a5701, a5702, a5703)
            };
            let a5704 = v11;
            let a5705 = 0u64;
            if nat_eq(a5704, a5705) {
                Ok(f277(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5706 = v11;
                let a5707 = 1u64;
                if nat_eq(a5706, a5707) {
                    Ok(f277(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f277(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5708 = v0;
            let a5709 = v7;
            if nat_eq(a5708, a5709) {
                Ok(f277(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5712 = v0;
                let a5713 = {
                    let a5710 = v7;
                    let a5711 = 1u64;
                    nat_add(a5710, a5711)?
                };
                if nat_eq(a5712, a5713) {
                    Ok(f277(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5716 = v0;
                    let a5717 = {
                        let a5714 = v7;
                        let a5715 = 2u64;
                        nat_add(a5714, a5715)?
                    };
                    if nat_eq(a5716, a5717) {
                        Ok(f277(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5720 = v0;
                        let a5721 = {
                            let a5718 = v7;
                            let a5719 = 3u64;
                            nat_add(a5718, a5719)?
                        };
                        if nat_eq(a5720, a5721) {
                            let a5722 = v5;
                            let a5723 = 1u64;
                            if if nat_eq(a5722, a5723) {
                                true
                            } else {
                                let a5724 = v5;
                                let a5725 = 2u64;
                                nat_eq(a5724, a5725)
                            } {
                                Ok(f277(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f277(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5732 = {
                                    let a5726 = v1;
                                    let a5727 = v6;
                                    nat_mul(a5726, a5727)?
                                };
                                let a5733 = {
                                    let a5730 = v0;
                                    let a5731 = {
                                        let a5728 = v7;
                                        let a5729 = 4u64;
                                        nat_add(a5728, a5729)?
                                    };
                                    nat_sub(a5730, a5731)
                                };
                                nat_add(a5732, a5733)?
                            };
                            Ok(f277(10u64, 0u64, f276(v12, v2)?, 1u64, 2u64, 3u64, f276(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f265(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5734, _, _, _, _, _, _, _) => {
            h5734
        }
    }, match v0.clone() {
        Adt20::C0(_, h5735, _, _, _, _, _, _) => {
            h5735
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5736, _, _, _, _, _) => {
            h5736
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5737, _, _, _, _) => {
            h5737
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5738, _, _) => {
            h5738
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5739, _) => {
            h5739
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5740) => {
            h5740
        }
    })
}

pub fn f266(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5747 = v3;
        let a5748 = {
            let a5744 = f276({
                let a5741 = v0;
                let a5742 = 2u64;
                let a5743 = 0u64;
                nat_quot(a5741, a5742, a5743)
            }, v1)?;
            let a5745 = v2;
            let a5746 = 0u64;
            nat_quot(a5744, a5745, a5746)
        };
        nat_add(a5747, a5748)?
    };
    let a5752 = {
        let a5749 = v0;
        let a5750 = 2u64;
        let a5751 = 0u64;
        nat_rem(a5749, a5750, a5751)
    };
    let a5753 = 0u64;
    if nat_eq(a5752, a5753) {
        Ok(f277(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5754 = v4;
        let a5755 = 0u64;
        if nat_eq(a5754, a5755) {
            Ok(f277(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f267(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f278(match v2.clone() {
        Adt20::C0(h5756, _, _, _, _, _, _, _) => {
            h5756
        }
    })?;
    let v7: u64 = f279(match v2.clone() {
        Adt20::C0(h5757, _, _, _, _, _, _, _) => {
            h5757
        }
    })?;
    let a5758 = v0;
    let a5759 = 2u64;
    if nat_lt(a5758, a5759) {
        Ok(f275(v0))
    } else {
        let a5760 = v0;
        let a5761 = v6;
        if nat_lt(a5760, a5761) {
            f280({
                let a5762 = v0;
                let a5763 = 2u64;
                nat_sub(a5762, a5763)
            }, v1, v2, v3)
        } else {
            let a5764 = v0;
            let a5765 = v7;
            if nat_lt(a5764, a5765) {
                Ok(f281({
                    let a5766 = v0;
                    let a5767 = v6;
                    nat_sub(a5766, a5767)
                }, match v2 {
                    Adt20::C0(h5768, _, _, _, _, _, _, _) => {
                        h5768
                    }
                }, v5))
            } else {
                let a5770 = v0;
                let a5771 = f239(match v2.clone() {
                    Adt20::C0(h5769, _, _, _, _, _, _, _) => {
                        h5769
                    }
                })?;
                if nat_lt(a5770, a5771) {
                    f282({
                        let a5772 = v0;
                        let a5773 = v7;
                        nat_sub(a5772, a5773)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f268(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f253(v5, match v2.clone() {
        Adt20::C0(h5774, _, _, _, _, _, _, _) => {
            h5774
        }
    })?;
    let v7: u64 = {
        let a5776 = v6;
        let a5777 = f254(v5, match v2.clone() {
            Adt20::C0(h5775, _, _, _, _, _, _, _) => {
                h5775
            }
        })?;
        nat_add(a5776, a5777)?
    };
    let v8: u64 = {
        let a5778 = v7;
        let a5779 = f255(v5);
        nat_add(a5778, a5779)?
    };
    let a5780 = v0;
    let a5781 = v6;
    if nat_lt(a5780, a5781) {
        f283(v0, v1, v2, v5)
    } else {
        let a5782 = v0;
        let a5783 = v7;
        if nat_lt(a5782, a5783) {
            f284({
                let a5784 = v0;
                let a5785 = v6;
                nat_sub(a5784, a5785)
            }, v1, v2, v3, v5)
        } else {
            let a5786 = v0;
            let a5787 = v8;
            if nat_lt(a5786, a5787) {
                Ok(f285({
                    let a5788 = v0;
                    let a5789 = v7;
                    nat_sub(a5788, a5789)
                }, match v2 {
                    Adt20::C0(h5790, _, _, _, _, _, _, _) => {
                        h5790
                    }
                }, v4, v5))
            } else {
                let a5794 = v0;
                let a5795 = {
                    let a5792 = v8;
                    let a5793 = match v2.clone() {
                        Adt20::C0(h5791, _, _, _, _, _, _, _) => {
                            h5791
                        }
                    };
                    nat_add(a5792, a5793)?
                };
                if nat_lt(a5794, a5795) {
                    let v9: u64 = f276({
                        let a5801 = {
                            let a5797 = v1;
                            let a5798 = match v2 {
                                Adt20::C0(h5796, _, _, _, _, _, _, _) => {
                                    h5796
                                }
                            };
                            nat_mul(a5797, a5798)?
                        };
                        let a5802 = {
                            let a5799 = v0;
                            let a5800 = v8;
                            nat_sub(a5799, a5800)
                        };
                        nat_add(a5801, a5802)?
                    }, v3)?;
                    Ok(f277(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f256(v3, v5)?;
    let v7: u64 = f258(v1, v2, v6)?;
    let v8: u64 = {
        let a5805 = {
            let a5803 = f257(v3, v5)?;
            let a5804 = v4;
            nat_mul(a5803, a5804)?
        };
        let a5806 = v7;
        nat_mul(a5805, a5806)?
    };
    let a5807 = v0;
    let a5808 = v8;
    if nat_lt(a5807, a5808) {
        let v9: u64 = {
            let a5809 = v0;
            let a5810 = v7;
            let a5811 = 0u64;
            nat_quot(a5809, a5810, a5811)
        };
        Ok(Adt22::C0({
            let a5815 = {
                let a5812 = v9;
                let a5813 = v4;
                let a5814 = 0u64;
                nat_quot(a5812, a5813, a5814)
            };
            let a5816 = v6;
            nat_mul(a5815, a5816)?
        }, v6, {
            let a5817 = v9;
            let a5818 = v4;
            let a5819 = 0u64;
            nat_rem(a5817, a5818, a5819)
        }, {
            let a5820 = v0;
            let a5821 = v7;
            let a5822 = 0u64;
            nat_rem(a5820, a5821, a5822)
        }))
    } else {
        let v10: u64 = {
            let a5823 = v0;
            let a5824 = v8;
            nat_sub(a5823, a5824)
        };
        let v11: u64 = f258(v1, v2, f259(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5825 = f257(v3, v5)?;
            let a5826 = v6;
            nat_mul(a5825, a5826)?
        }, f259(v3, v5)?, {
            let a5827 = v10;
            let a5828 = v11;
            let a5829 = 0u64;
            nat_quot(a5827, a5828, a5829)
        }, {
            let a5830 = v10;
            let a5831 = v11;
            let a5832 = 0u64;
            nat_rem(a5830, a5831, a5832)
        }))
    }
}

pub fn f270(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5833) => {
            h5833
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5834, _, _, _, _, _, _, _) => {
            h5834
        }
    };
    let v6: u64 = {
        let a5837 = match v0.clone() {
            Adt22::C0(_, h5835, _, _) => {
                h5835
            }
        };
        let a5838 = match v1.clone() {
            Adt20::C0(_, h5836, _, _, _, _, _, _) => {
                h5836
            }
        };
        nat_mul(a5837, a5838)?
    };
    let v7: u64 = {
        let a5839 = v6;
        let a5840 = v5;
        nat_mul(a5839, a5840)?
    };
    let v8: u64 = {
        let a5843 = {
            let a5841 = 2u64;
            let a5842 = v5;
            nat_mul(a5841, a5842)?
        };
        let a5844 = 2u64;
        nat_add(a5843, a5844)?
    };
    let v9: u64 = {
        let a5847 = {
            let a5845 = 4u64;
            let a5846 = v5;
            nat_mul(a5845, a5846)?
        };
        let a5848 = 2u64;
        nat_add(a5847, a5848)?
    };
    let v10: u64 = {
        let a5851 = 1u64;
        let a5852 = {
            let a5849 = v6;
            let a5850 = v8;
            nat_mul(a5849, a5850)?
        };
        nat_add(a5851, a5852)?
    };
    let v11: u64 = {
        let a5853 = v10;
        let a5854 = 4u64;
        nat_add(a5853, a5854)?
    };
    let v12: u64 = {
        let a5857 = v11;
        let a5858 = {
            let a5855 = v6;
            let a5856 = v9;
            nat_mul(a5855, a5856)?
        };
        nat_add(a5857, a5858)?
    };
    let v13: u64 = {
        let a5859 = v12;
        let a5860 = 6u64;
        nat_add(a5859, a5860)?
    };
    let a5861 = v4;
    let a5862 = 0u64;
    if nat_eq(a5861, a5862) {
        Ok(f275(0u64))
    } else {
        let a5863 = v4;
        let a5864 = v10;
        if nat_lt(a5863, a5864) {
            let v14: u64 = {
                let a5865 = v4;
                let a5866 = 1u64;
                nat_sub(a5865, a5866)
            };
            f286(false, {
                let a5867 = v14;
                let a5868 = v8;
                let a5869 = 0u64;
                nat_rem(a5867, a5868, a5869)
            }, {
                let a5870 = v14;
                let a5871 = v8;
                let a5872 = 0u64;
                nat_quot(a5870, a5871, a5872)
            }, v0, v1, v2)
        } else {
            let a5873 = v4;
            let a5874 = v11;
            if nat_lt(a5873, a5874) {
                Ok(f287({
                    let a5875 = v4;
                    let a5876 = v10;
                    nat_sub(a5875, a5876)
                }, v7))
            } else {
                let a5877 = v4;
                let a5878 = v12;
                if nat_lt(a5877, a5878) {
                    let v15: u64 = {
                        let a5879 = v4;
                        let a5880 = v11;
                        nat_sub(a5879, a5880)
                    };
                    f286(true, {
                        let a5881 = v15;
                        let a5882 = v9;
                        let a5883 = 0u64;
                        nat_rem(a5881, a5882, a5883)
                    }, {
                        let a5884 = v15;
                        let a5885 = v9;
                        let a5886 = 0u64;
                        nat_quot(a5884, a5885, a5886)
                    }, v0, v1, v2)
                } else {
                    let a5887 = v4;
                    let a5888 = v13;
                    if nat_lt(a5887, a5888) {
                        Ok(f288({
                            let a5889 = v4;
                            let a5890 = v12;
                            nat_sub(a5889, a5890)
                        }, v7, v3))
                    } else {
                        let a5893 = v4;
                        let a5894 = {
                            let a5891 = v13;
                            let a5892 = v7;
                            nat_add(a5891, a5892)?
                        };
                        if nat_lt(a5893, a5894) {
                            let v16: u64 = {
                                let a5895 = v4;
                                let a5896 = v13;
                                nat_sub(a5895, a5896)
                            };
                            let v17: u64 = {
                                let a5897 = v16;
                                let a5898 = v5;
                                let a5899 = 0u64;
                                nat_quot(a5897, a5898, a5899)
                            };
                            let v18: u64 = {
                                let a5905 = {
                                    let a5900 = f289(v17, v0, v1)?;
                                    let a5901 = v5;
                                    nat_mul(a5900, a5901)?
                                };
                                let a5906 = {
                                    let a5902 = v16;
                                    let a5903 = v5;
                                    let a5904 = 0u64;
                                    nat_rem(a5902, a5903, a5904)
                                };
                                nat_add(a5905, a5906)?
                            };
                            let v19: u64 = f276(v18, v2)?;
                            Ok(f277(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f271(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f272(v0: u64) -> u64 {
    let a5907 = v0;
    let a5908 = 4u64;
    let a5909 = 0u64;
    nat_quot(a5907, a5908, a5909)
}

pub fn f273(v0: u64) -> R<u64> {
    let a5912 = {
        let a5910 = 7u64;
        let a5911 = v0;
        nat_mul(a5910, a5911)?
    };
    let a5913 = 4u64;
    nat_add(a5912, a5913)
}

pub fn f274(v0: u64) -> u64 {
    if f122(v0) {
        let m5914 = f124(v0);
        match m5914 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5916 = 1u64;
                let a5917 = match v1.clone() {
                    Adt18::C0(_, h5915, _) => {
                        h5915
                    }
                };
                if if nat_lt(a5916, a5917) {
                    true
                } else {
                    let a5918 = v0;
                    let a5919 = 43u64;
                    nat_eq(a5918, a5919)
                } {
                    match v1 {
                        Adt18::C0(_, h5920, _) => {
                            h5920
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

pub fn f275(v0: u64) -> Adt19 {
    f277(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f276(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5922 = v0;
        let a5923 = match v1.clone() {
            Adt20::C0(h5921, _, _, _, _, _, _, _) => {
                h5921
            }
        };
        let a5924 = 0u64;
        nat_quot(a5922, a5923, a5924)
    };
    let v3: u64 = {
        let a5926 = v2;
        let a5927 = match v1.clone() {
            Adt20::C0(_, h5925, _, _, _, _, _, _) => {
                h5925
            }
        };
        let a5928 = 0u64;
        nat_quot(a5926, a5927, a5928)
    };
    f141({
        let a5930 = v0;
        let a5931 = match v1.clone() {
            Adt20::C0(h5929, _, _, _, _, _, _, _) => {
                h5929
            }
        };
        let a5932 = 0u64;
        nat_rem(a5930, a5931, a5932)
    }, {
        let a5934 = v2;
        let a5935 = match v1.clone() {
            Adt20::C0(_, h5933, _, _, _, _, _, _) => {
                h5933
            }
        };
        let a5936 = 0u64;
        nat_rem(a5934, a5935, a5936)
    }, {
        let a5938 = v3;
        let a5939 = match v1.clone() {
            Adt20::C0(_, _, h5937, _, _, _, _, _) => {
                h5937
            }
        };
        let a5940 = 0u64;
        nat_rem(a5938, a5939, a5940)
    }, {
        let a5942 = v3;
        let a5943 = match v1.clone() {
            Adt20::C0(_, _, h5941, _, _, _, _, _) => {
                h5941
            }
        };
        let a5944 = 0u64;
        nat_quot(a5942, a5943, a5944)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5945, _, _, _) => {
            h5945
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5946, _, _) => {
            h5946
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5947, _) => {
            h5947
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5948) => {
            h5948
        }
    })
}

pub fn f277(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5949 = v0;
    let a5950 = 4u64;
    if if if nat_le(a5949, a5950) {
        true
    } else {
        let a5951 = 9u64;
        let a5952 = v0;
        if nat_le(a5951, a5952) {
            let a5953 = v0;
            let a5954 = 12u64;
            nat_le(a5953, a5954)
        } else {
            false
        }
    } {
        true
    } else {
        let a5955 = v0;
        let a5956 = 16u64;
        if if nat_eq(a5955, a5956) {
            true
        } else {
            let a5957 = v0;
            let a5958 = 19u64;
            nat_eq(a5957, a5958)
        } {
            true
        } else {
            let a5959 = v0;
            let a5960 = 21u64;
            if nat_eq(a5959, a5960) {
                true
            } else {
                let a5961 = v0;
                let a5962 = 22u64;
                nat_eq(a5961, a5962)
            }
        }
    } {
        Adt19::C1({
            let a5963 = v0;
            let a5964 = 4u64;
            if if nat_lt(a5963, a5964) {
                true
            } else {
                let a5965 = 21u64;
                let a5966 = v0;
                nat_le(a5965, a5966)
            } {
                0u64
            } else {
                let a5967 = v0;
                let a5968 = 16u64;
                if if nat_eq(a5967, a5968) {
                    true
                } else {
                    let a5969 = v0;
                    let a5970 = 19u64;
                    nat_eq(a5969, a5970)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a5971 = v0;
            let a5972 = 4u64;
            if nat_lt(a5971, a5972) {
                v0
            } else {
                let a5973 = v0;
                let a5974 = 4u64;
                if if nat_eq(a5973, a5974) {
                    true
                } else {
                    let a5975 = v0;
                    let a5976 = 16u64;
                    nat_eq(a5975, a5976)
                } {
                    0u64
                } else {
                    let a5977 = v0;
                    let a5978 = 19u64;
                    if nat_eq(a5977, a5978) {
                        3u64
                    } else {
                        let a5979 = 21u64;
                        let a5980 = v0;
                        if nat_le(a5979, a5980) {
                            let a5981 = v0;
                            let a5982 = 17u64;
                            nat_sub(a5981, a5982)
                        } else {
                            let a5983 = v0;
                            let a5984 = 8u64;
                            nat_sub(a5983, a5984)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f278(v0: u64) -> R<u64> {
    let a5987 = 2u64;
    let a5988 = {
        let a5985 = 6u64;
        let a5986 = v0;
        nat_mul(a5985, a5986)?
    };
    nat_add(a5987, a5988)
}

pub fn f279(v0: u64) -> R<u64> {
    let a5989 = f278(v0)?;
    let a5990 = 11u64;
    nat_add(a5989, a5990)
}

pub fn f280(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a5991 = v0;
        let a5992 = 6u64;
        let a5993 = 0u64;
        nat_rem(a5991, a5992, a5993)
    };
    let v5: u64 = {
        let a6000 = {
            let a5995 = v1;
            let a5996 = match v2.clone() {
                Adt20::C0(h5994, _, _, _, _, _, _, _) => {
                    h5994
                }
            };
            nat_mul(a5995, a5996)?
        };
        let a6001 = {
            let a5997 = v0;
            let a5998 = 6u64;
            let a5999 = 0u64;
            nat_quot(a5997, a5998, a5999)
        };
        nat_add(a6000, a6001)?
    };
    let v6: u64 = f276(v5, v3)?;
    let v7: u64 = f276(v5, v2)?;
    let a6002 = v4;
    let a6003 = 0u64;
    if nat_eq(a6002, a6003) {
        Ok(f277(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6004 = v4;
        let a6005 = 1u64;
        if nat_eq(a6004, a6005) {
            Ok(f277(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6006 = v4;
            let a6007 = 2u64;
            if nat_eq(a6006, a6007) {
                Ok(f277(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6008 = v4;
                let a6009 = 3u64;
                if nat_eq(a6008, a6009) {
                    Ok(f277(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6010 = v4;
                    let a6011 = 4u64;
                    if nat_eq(a6010, a6011) {
                        Ok(f277(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f277(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f281(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6012 = v0;
    let a6013 = 0u64;
    if nat_eq(a6012, a6013) {
        f277(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6014 = v0;
        let a6015 = 1u64;
        if nat_eq(a6014, a6015) {
            f277(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6016 = v0;
            let a6017 = 2u64;
            if nat_eq(a6016, a6017) {
                f277(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6018 = v0;
                let a6019 = 3u64;
                if nat_eq(a6018, a6019) {
                    f277(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6020 = v0;
                    let a6021 = 4u64;
                    if nat_eq(a6020, a6021) {
                        f277(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6022 = v0;
                        let a6023 = 5u64;
                        if nat_eq(a6022, a6023) {
                            f277(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6024 = v0;
                            let a6025 = 6u64;
                            if nat_eq(a6024, a6025) {
                                f277(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6026 = v0;
                                let a6027 = 7u64;
                                if nat_eq(a6026, a6027) {
                                    f277(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6028 = v0;
                                    let a6029 = 8u64;
                                    if nat_eq(a6028, a6029) {
                                        f277(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6030 = v0;
                                        let a6031 = 9u64;
                                        if nat_eq(a6030, a6031) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6032 = v0;
                                            let a6033 = 10u64;
                                            if nat_eq(a6032, a6033) {
                                                f277(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f282(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6034 = v0;
        let a6035 = 3u64;
        let a6036 = 0u64;
        nat_rem(a6034, a6035, a6036)
    };
    let v6: u64 = {
        let a6043 = {
            let a6038 = v1;
            let a6039 = match v2.clone() {
                Adt20::C0(h6037, _, _, _, _, _, _, _) => {
                    h6037
                }
            };
            nat_mul(a6038, a6039)?
        };
        let a6044 = {
            let a6040 = v0;
            let a6041 = 3u64;
            let a6042 = 0u64;
            nat_quot(a6040, a6041, a6042)
        };
        nat_add(a6043, a6044)?
    };
    let v7: u64 = f276(v6, v3)?;
    let v8: u64 = f276(v6, v2)?;
    let v9: u64 = f276(v6, v4)?;
    let a6045 = v5;
    let a6046 = 0u64;
    if nat_eq(a6045, a6046) {
        Ok(f277(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6047 = v5;
        let a6048 = 1u64;
        if nat_eq(a6047, a6048) {
            Ok(f277(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f277(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f283(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6049, _, _, _, _, _, _, _) => {
            h6049
        }
    };
    let v5: u64 = {
        let a6052 = 1u64;
        let a6053 = {
            let a6050 = v4;
            let a6051 = f271(v3);
            nat_mul(a6050, a6051)?
        };
        nat_add(a6052, a6053)?
    };
    let a6054 = v0;
    let a6055 = 0u64;
    if nat_eq(a6054, a6055) {
        Ok(f275(0u64))
    } else {
        let a6056 = v0;
        let a6057 = v5;
        if nat_lt(a6056, a6057) {
            let v6: u64 = {
                let a6058 = v0;
                let a6059 = 1u64;
                nat_sub(a6058, a6059)
            };
            let v7: u64 = f276({
                let a6065 = {
                    let a6060 = v1;
                    let a6061 = v4;
                    nat_mul(a6060, a6061)?
                };
                let a6066 = {
                    let a6062 = v6;
                    let a6063 = f271(v3);
                    let a6064 = 0u64;
                    nat_quot(a6062, a6063, a6064)
                };
                nat_add(a6065, a6066)?
            }, v2)?;
            if v3 {
                let a6070 = {
                    let a6067 = v6;
                    let a6068 = 2u64;
                    let a6069 = 0u64;
                    nat_rem(a6067, a6068, a6069)
                };
                let a6071 = 0u64;
                if nat_eq(a6070, a6071) {
                    Ok(f277(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f277(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f277(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6072 = v0;
                let a6073 = v5;
                nat_sub(a6072, a6073)
            };
            let a6074 = v8;
            let a6075 = 0u64;
            if nat_eq(a6074, a6075) {
                Ok(f277(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6076 = v8;
                    let a6077 = 1u64;
                    nat_eq(a6076, a6077)
                } else {
                    false
                } {
                    Ok(f277(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6078 = v8;
                    let a6079 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6078, a6079) {
                        Ok(f277(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6080 = v8;
                        let a6081 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6080, a6081) {
                            Ok(f275(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f284(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a6083 = 13u64;
        let a6084 = f272(match v2.clone() {
            Adt20::C0(h6082, _, _, _, _, _, _, _) => {
                h6082
            }
        });
        nat_mul(a6083, a6084)?
    } else {
        0u64
    };
    let a6085 = v0;
    let a6086 = v5;
    if nat_lt(a6085, a6086) {
        f290({
            let a6087 = v0;
            let a6088 = 13u64;
            let a6089 = 0u64;
            nat_rem(a6087, a6088, a6089)
        }, v1, {
            let a6090 = v0;
            let a6091 = 13u64;
            let a6092 = 0u64;
            nat_quot(a6090, a6091, a6092)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6093 = v0;
            let a6094 = v5;
            nat_sub(a6093, a6094)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f291({
            let a6095 = v6;
            let a6096 = v7;
            let a6097 = 0u64;
            nat_rem(a6095, a6096, a6097)
        }, {
            let a6107 = {
                let a6102 = {
                    let a6099 = v1;
                    let a6100 = match v2.clone() {
                        Adt20::C0(h6098, _, _, _, _, _, _, _) => {
                            h6098
                        }
                    };
                    nat_mul(a6099, a6100)?
                };
                let a6103 = if v4 {
                    f292(match v2.clone() {
                        Adt20::C0(h6101, _, _, _, _, _, _, _) => {
                            h6101
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6102, a6103)?
            };
            let a6108 = {
                let a6104 = v6;
                let a6105 = v7;
                let a6106 = 0u64;
                nat_quot(a6104, a6105, a6106)
            };
            nat_add(a6107, a6108)?
        }, v2, v3, v4)
    }
}

pub fn f285(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a6109 = v0;
    let a6110 = 0u64;
    if nat_eq(a6109, a6110) {
        f277(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6111 = v0;
        let a6112 = 1u64;
        if nat_eq(a6111, a6112) {
            f277(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6113 = v0;
                let a6114 = 2u64;
                nat_eq(a6113, a6114)
            } else {
                false
            } {
                f277(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6115 = v0;
                let a6116 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6115, a6116) {
                    f277(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6117 = v0;
                    let a6118 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6117, a6118) {
                        f277(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6119 = v0;
                        let a6120 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6119, a6120) {
                            f277(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f286(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6124 = 1u64;
        let a6125 = {
            let a6122 = match v4.clone() {
                Adt20::C0(h6121, _, _, _, _, _, _, _) => {
                    h6121
                }
            };
            let a6123 = v6;
            nat_mul(a6122, a6123)?
        };
        nat_add(a6124, a6125)?
    };
    let a6126 = v1;
    let a6127 = 0u64;
    if nat_eq(a6126, a6127) {
        Ok(f275(1u64))
    } else {
        let a6128 = v1;
        let a6129 = v7;
        if nat_lt(a6128, a6129) {
            let v8: u64 = {
                let a6130 = v1;
                let a6131 = 1u64;
                nat_sub(a6130, a6131)
            };
            let v9: u64 = {
                let a6138 = {
                    let a6133 = f289(v2, v3, v4.clone())?;
                    let a6134 = match v4.clone() {
                        Adt20::C0(h6132, _, _, _, _, _, _, _) => {
                            h6132
                        }
                    };
                    nat_mul(a6133, a6134)?
                };
                let a6139 = {
                    let a6135 = v8;
                    let a6136 = v6;
                    let a6137 = 0u64;
                    nat_quot(a6135, a6136, a6137)
                };
                nat_add(a6138, a6139)?
            };
            let v10: u64 = f276(v9, v4)?;
            let v11: u64 = f276(v9, v5)?;
            let v12: u64 = {
                let a6140 = v8;
                let a6141 = v6;
                let a6142 = 0u64;
                nat_rem(a6140, a6141, a6142)
            };
            if v0 {
                let a6143 = v12;
                let a6144 = 0u64;
                if nat_eq(a6143, a6144) {
                    Ok(f277(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6145 = v12;
                    let a6146 = 1u64;
                    if nat_eq(a6145, a6146) {
                        Ok(f277(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6147 = v12;
                        let a6148 = 2u64;
                        if nat_eq(a6147, a6148) {
                            Ok(f277(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f277(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6149 = v12;
                let a6150 = 0u64;
                if nat_eq(a6149, a6150) {
                    Ok(f277(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f277(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6151 = v1;
            let a6152 = v7;
            if nat_eq(a6151, a6152) {
                Ok(f277(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f287(v0: u64, v1: u64) -> Adt19 {
    let a6153 = v0;
    let a6154 = 0u64;
    if nat_eq(a6153, a6154) {
        f277(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6155 = v0;
        let a6156 = 1u64;
        if nat_eq(a6155, a6156) {
            f277(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6157 = v0;
            let a6158 = 2u64;
            if nat_eq(a6157, a6158) {
                f277(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6159 = v0;
                let a6160 = 3u64;
                if nat_eq(a6159, a6160) {
                    f275(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f288(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6161 = v0;
    let a6162 = 0u64;
    if nat_eq(a6161, a6162) {
        f277(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6163 = v0;
        let a6164 = 1u64;
        if nat_eq(a6163, a6164) {
            f277(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6165 = v0;
            let a6166 = 2u64;
            if nat_eq(a6165, a6166) {
                f277(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6167 = v0;
                let a6168 = 3u64;
                if nat_eq(a6167, a6168) {
                    f277(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6169 = v0;
                    let a6170 = 4u64;
                    if nat_eq(a6169, a6170) {
                        f277(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6171 = v0;
                        let a6172 = 5u64;
                        if nat_eq(a6171, a6172) {
                            f277(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f289(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a6183 = {
        let a6181 = {
            let a6178 = {
                let a6175 = match v1.clone() {
                    Adt22::C0(_, _, h6173, _) => {
                        h6173
                    }
                };
                let a6176 = match v2.clone() {
                    Adt20::C0(_, _, h6174, _, _, _, _, _) => {
                        h6174
                    }
                };
                nat_mul(a6175, a6176)?
            };
            let a6179 = match v1 {
                Adt22::C0(h6177, _, _, _) => {
                    h6177
                }
            };
            nat_add(a6178, a6179)?
        };
        let a6182 = match v2 {
            Adt20::C0(_, h6180, _, _, _, _, _, _) => {
                h6180
            }
        };
        nat_mul(a6181, a6182)?
    };
    let a6184 = v0;
    nat_add(a6183, a6184)
}

pub fn f290(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6192 = {
            let a6190 = {
                let a6186 = v1;
                let a6187 = match v3.clone() {
                    Adt20::C0(h6185, _, _, _, _, _, _, _) => {
                        h6185
                    }
                };
                nat_mul(a6186, a6187)?
            };
            let a6191 = {
                let a6188 = v2;
                let a6189 = 4u64;
                nat_mul(a6188, a6189)?
            };
            nat_add(a6190, a6191)?
        };
        let a6193 = f293(v0);
        nat_add(a6192, a6193)?
    };
    let v6: u64 = f276(v5, v3)?;
    let v7: u64 = f276(v5, v4)?;
    let a6194 = v0;
    let a6195 = 0u64;
    if if if if nat_eq(a6194, a6195) {
        true
    } else {
        let a6196 = v0;
        let a6197 = 2u64;
        nat_eq(a6196, a6197)
    } {
        true
    } else {
        let a6198 = v0;
        let a6199 = 5u64;
        nat_eq(a6198, a6199)
    } {
        true
    } else {
        let a6200 = v0;
        let a6201 = 7u64;
        nat_eq(a6200, a6201)
    } {
        Ok(f277(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6202 = v0;
        let a6203 = 1u64;
        if if if if nat_eq(a6202, a6203) {
            true
        } else {
            let a6204 = v0;
            let a6205 = 3u64;
            nat_eq(a6204, a6205)
        } {
            true
        } else {
            let a6206 = v0;
            let a6207 = 6u64;
            nat_eq(a6206, a6207)
        } {
            true
        } else {
            let a6208 = v0;
            let a6209 = 8u64;
            nat_eq(a6208, a6209)
        } {
            Ok(f277(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6210 = v0;
                let a6211 = 1u64;
                if nat_eq(a6210, a6211) {
                    4u64
                } else {
                    let a6212 = v0;
                    let a6213 = 6u64;
                    if nat_eq(a6212, a6213) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6214 = v0;
            let a6215 = 4u64;
            if nat_eq(a6214, a6215) {
                Ok(f277(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6216 = v0;
                let a6217 = 9u64;
                if nat_eq(a6216, a6217) {
                    Ok(f277(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6218 = v0;
                    let a6219 = 10u64;
                    if nat_eq(a6218, a6219) {
                        Ok(f277(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6220 = v0;
                        let a6221 = 11u64;
                        if nat_eq(a6220, a6221) {
                            Ok(f277(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6222 = v0;
                            let a6223 = 12u64;
                            if nat_eq(a6222, a6223) {
                                Ok(f277(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f291(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f276(v1, v2)?;
    let v6: u64 = f276(v1, v3)?;
    let a6224 = v0;
    let a6225 = 0u64;
    if nat_eq(a6224, a6225) {
        Ok(f277(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6226 = v0;
        let a6227 = 1u64;
        if nat_eq(a6226, a6227) {
            Ok(f277(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6228 = v0;
                let a6229 = 2u64;
                if nat_eq(a6228, a6229) {
                    Ok(f277(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6230 = v0;
                    let a6231 = 3u64;
                    if nat_eq(a6230, a6231) {
                        Ok(f277(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a6232 = v0;
                let a6233 = 2u64;
                if nat_eq(a6232, a6233) {
                    Ok(f277(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f292(v0: u64) -> R<u64> {
    let a6234 = 4u64;
    let a6235 = f272(v0);
    nat_mul(a6234, a6235)
}

pub fn f293(v0: u64) -> u64 {
    let a6236 = v0;
    let a6237 = 5u64;
    if nat_lt(a6236, a6237) {
        let a6238 = v0;
        let a6239 = 2u64;
        if nat_lt(a6238, a6239) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6240 = v0;
        let a6241 = 7u64;
        if nat_lt(a6240, a6241) {
            2u64
        } else {
            3u64
        }
    }
}
