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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, bool, u64, bool, Adt2),
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
    C35(u64),
    C36(Adt3, bool, u64),
    C37(Adt3, Adt3, Option<Adt3>),
    C38(Adt3, Adt3, u64, u64),
    C39(bool),
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
    C6(u64, u64, u64, bool, u64),
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
    C7,
    C8,
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
    C9,
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
    C3(u64, u64),
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
    C1(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt18 {
    C0,
    C1(Adt19),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt22 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt9> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) => {
            let a2 = v1;
            let a3 = 96u64;
            if if nat_eq(a2, a3) {
                true
            } else {
                let a4 = v1;
                let a5 = 97u64;
                nat_eq(a4, a5)
            } {
                Ok(Adt9::C0(f1({
                    let a6 = v1;
                    let a7 = 96u64;
                    nat_eq(a6, a7)
                }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
            } else {
                let v13: bool = {
                    let m8 = v7;
                    match m8 {
                        None => {
                            v8
                        }
                        Some(_) => {
                            true
                        }
                    }
                };
                let a9 = v1;
                let a10 = 85u64;
                if nat_eq(a9, a10) {
                    Ok(Adt9::C0(f2(v2, v3, v4, v5, {
                        let m11 = v6;
                        match m11 {
                            None => {
                                v13
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }, v11)?))
                } else {
                    let a12 = v1;
                    let a13 = 87u64;
                    if nat_eq(a12, a13) {
                        Ok(Adt9::C0(f3(v3, v4, v5, v6, v13, v11)?))
                    } else {
                        let a14 = v1;
                        let a15 = 64u64;
                        if nat_eq(a14, a15) {
                            Ok(Adt9::C0(f4(v2, v11)?))
                        } else {
                            let m16 = v3;
                            match m16 {
                                None => {
                                    Ok(Adt9::C0(Adt10::C1))
                                }
                                Some(v15) => {
                                    let a17 = v1;
                                    let a18 = 48u64;
                                    if if nat_eq(a17, a18) {
                                        true
                                    } else {
                                        let a19 = v1;
                                        let a20 = 56u64;
                                        nat_eq(a19, a20)
                                    } {
                                        Ok(Adt9::C0(f5({
                                            let a21 = v1;
                                            let a22 = 56u64;
                                            nat_eq(a21, a22)
                                        }, v2, v15, v4, v11)?))
                                    } else {
                                        let a23 = v1;
                                        let a24 = 53u64;
                                        if if nat_eq(a23, a24) {
                                            true
                                        } else {
                                            let a25 = v1;
                                            let a26 = 54u64;
                                            nat_eq(a25, a26)
                                        } {
                                            Ok(Adt9::C0(f6({
                                                let a27 = v1;
                                                let a28 = 54u64;
                                                nat_eq(a27, a28)
                                            }, v2, v15, v4, v11)?))
                                        } else {
                                            let a29 = v1;
                                            let a30 = 78u64;
                                            if nat_eq(a29, a30) {
                                                Ok(Adt9::C0(f7(v2, v15, v4, v5, v11)?))
                                            } else {
                                                let a31 = v1;
                                                let a32 = 23u64;
                                                if if if nat_eq(a31, a32) {
                                                    true
                                                } else {
                                                    let a33 = v1;
                                                    let a34 = 26u64;
                                                    nat_eq(a33, a34)
                                                } {
                                                    true
                                                } else {
                                                    let a35 = v1;
                                                    let a36 = 25u64;
                                                    nat_eq(a35, a36)
                                                } {
                                                    Ok(Adt9::C0(f8(v1, v2, v15, v4, v11)?))
                                                } else {
                                                    let a37 = v1;
                                                    let a38 = 24u64;
                                                    if nat_eq(a37, a38) {
                                                        Ok(Adt9::C0(f9(v2, v15, v11)?))
                                                    } else {
                                                        let a39 = v1;
                                                        let a40 = 27u64;
                                                        if nat_eq(a39, a40) {
                                                            Ok(Adt9::C0(f10(v2, v15, v11)?))
                                                        } else {
                                                            let a41 = v1;
                                                            let a42 = 3u64;
                                                            if nat_eq(a41, a42) {
                                                                Ok(Adt9::C0(f11(v15, v4, v5, v11)?))
                                                            } else {
                                                                f12(v1, v2, v15, v4, v11)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v21, v22, v23, v24, v25) => {
            f15(v21, v22, v23, v24, v25)
        }
        Adt0::C2(v36) => {
            f17(v36)
        }
        Adt0::C3(v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49) => {
            Ok(f18(v37, v38, v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49))
        }
        Adt0::C4(v16, v17, v18, v19) => {
            let a44 = 0u64;
            let a45 = match v16.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h43, _, _, _, _, _, _, _) => {
                    h43
                }
            };
            if nat_lt(a44, a45) {
                f13(v16, v17, v18, v19)
            } else {
                f14(v16, v17, v18, v19)
            }
        }
        Adt0::C5(v20) => {
            Ok(Adt9::C9({
                let a46 = 3240099840u64;
                let a47 = v20;
                if if nat_le(a46, a47) {
                    let a48 = v20;
                    let a49 = 4286578688u64;
                    nat_le(a48, a49)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a50 = 1092616192u64;
                    let a51 = v20;
                    if if nat_le(a50, a51) {
                        let a52 = v20;
                        let a53 = 2139095040u64;
                        nat_le(a52, a53)
                    } else {
                        false
                    } {
                        Some(v20)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v26, v27, v28, v29, v30, v31, v32, v33, v34, v35) => {
            f16(v26, v27, v28, v29, v30, true, v31, v32, v33, v34, v35)
        }
    }
}

pub fn f1(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m54 = v3.clone();
        match m54 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m55 = v4.clone();
        match m55 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m56 = v5.clone();
        match m56 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m57 = v6.clone();
        match m57 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m58 = v7.clone();
        match m58 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m59 = v5.clone();
        match m59 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m60 = v3;
    if if if if if if if if if if if match m60 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m61 = v4;
        match m61 {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    } else {
        false
    } {
        let m62 = v5;
        match m62 {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    } else {
        false
    } {
        if v0 {
            let m63 = v6;
            if match m63 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m64 = v7;
                match m64 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            } else {
                false
            }
        } else {
            let m65 = v6;
            if !match m65 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m66 = v7;
                !match m66 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        !v8
    } else {
        false
    } {
        if if if if if if if if if v2 {
            let a73 = {
                let a70 = {
                    let a67 = v1;
                    let a68 = 4u64;
                    let a69 = 0u64;
                    nat_quot(a67, a68, a69)
                };
                let a71 = 2u64;
                let a72 = 0u64;
                nat_rem(a70, a71, a72)
            };
            let a74 = 1u64;
            nat_eq(a73, a74)
        } else {
            false
        } {
            let a76 = match v11.clone() {
                Adt2::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            };
            let a77 = 0u64;
            nat_eq(a76, a77)
        } else {
            false
        } {
            let a79 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h78, _, _, _, _, _) => {
                    h78
                }
            };
            let a80 = 4u64;
            nat_eq(a79, a80)
        } else {
            false
        } {
            f19(v11.clone(), f20(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f21(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a82 = match v11.clone() {
                Adt2::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                    h81
                }
            };
            let a83 = 1u64;
            if nat_le(a82, a83) {
                true
            } else {
                let a86 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h84, _, _, _, _, _) => {
                        h84
                    }
                };
                let a87 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h85, _, _, _, _, _) => {
                        h85
                    }
                };
                nat_eq(a86, a87)
            }
        } else {
            false
        } {
            let a89 = match v11.clone() {
                Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            let a90 = 1u64;
            if nat_le(a89, a90) {
                true
            } else {
                let a93 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h91, _, _, _, _) => {
                        h91
                    }
                };
                let a94 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h92, _, _, _, _) => {
                        h92
                    }
                };
                nat_eq(a93, a94)
            }
        } else {
            false
        } {
            let a96 = match v11.clone() {
                Adt2::C0(_, _, _, h95, _, _, _, _, _, _, _) => {
                    h95
                }
            };
            let a97 = 1u64;
            if nat_le(a96, a97) {
                true
            } else {
                let a100 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h98, _, _, _) => {
                        h98
                    }
                };
                let a101 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h99, _, _, _) => {
                        h99
                    }
                };
                nat_eq(a100, a101)
            }
        } else {
            false
        } {
            let a103 = match v11.clone() {
                Adt2::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                    h102
                }
            };
            let a104 = 1u64;
            if nat_le(a103, a104) {
                true
            } else {
                let a107 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h105, _, _) => {
                        h105
                    }
                };
                let a108 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h106, _, _) => {
                        h106
                    }
                };
                nat_eq(a107, a108)
            }
        } else {
            false
        }
    } else {
        false
    } {
        f21(v11.clone(), v13)?
    } else {
        false
    } {
        if v0 {
            if f21(v11.clone(), v15)? {
                f21(v11.clone(), v17)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a110 = match v20.clone() {
            Adt2::C0(h109, _, _, _, _, _, _, _, _, _, _) => {
                h109
            }
        };
        let a111 = 0u64;
        nat_eq(a110, a111)
    } else {
        false
    } {
        let a112 = f20(v20.clone(), 1u64)?;
        let a113 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a115 = if v0 {
            28u64
        } else {
            8u64
        };
        let a116 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h114, _) => {
                h114
            }
        };
        nat_le(a115, a116)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a119 = f22(v0);
            let a120 = {
                let a117 = f20(v11.clone(), 1u64)?;
                let a118 = f23(v0);
                nat_mul(a117, a118)?
            };
            nat_add(a119, a120)?
        }, 4u64, f24(v11, 1u64), f24(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m121 = v1.clone();
        match m121 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m122 = v2.clone();
        match m122 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m123 = v3.clone();
        match m123 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h124, _, _, _, _, _, _, _, _, _, _) => {
            h124
        }
    };
    let m125 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m125 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m126 = v2;
        match m126 {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    } else {
        false
    } {
        let m127 = v3;
        match m127 {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    } else {
        false
    } {
        !v4
    } else {
        false
    } {
        let a129 = match v7.clone() {
            Adt2::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                h128
            }
        };
        let a130 = 0u64;
        if if nat_eq(a129, a130) {
            f19(v7.clone(), f20(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f25(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a132 = match v9.clone() {
            Adt2::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        };
        let a133 = 0u64;
        if if nat_eq(a132, a133) {
            f19(v9.clone(), f20(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f25(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a135 = match v11.clone() {
            Adt2::C0(h134, _, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        };
        let a136 = 0u64;
        if if nat_eq(a135, a136) {
            f19(v11.clone(), f20(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f25(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a138 = match v7.clone() {
            Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        };
        let a139 = 24u64;
        nat_eq(a138, a139)
    } else {
        false
    } {
        let a141 = match v7.clone() {
            Adt2::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
                h140
            }
        };
        let a142 = 1u64;
        nat_eq(a141, a142)
    } else {
        false
    } {
        let a144 = match v7.clone() {
            Adt2::C0(_, _, _, _, h143, _, _, _, _, _, _) => {
                h143
            }
        };
        let a145 = 1u64;
        nat_eq(a144, a145)
    } else {
        false
    } {
        let a147 = match v11.clone() {
            Adt2::C0(_, h146, _, _, _, _, _, _, _, _, _) => {
                h146
            }
        };
        let a148 = 24u64;
        nat_eq(a147, a148)
    } else {
        false
    } {
        let a150 = match v11.clone() {
            Adt2::C0(_, _, h149, _, _, _, _, _, _, _, _) => {
                h149
            }
        };
        let a151 = 1u64;
        nat_eq(a150, a151)
    } else {
        false
    } {
        let a153 = match v11.clone() {
            Adt2::C0(_, _, _, h152, _, _, _, _, _, _, _) => {
                h152
            }
        };
        let a154 = 1u64;
        nat_eq(a153, a154)
    } else {
        false
    } {
        let a156 = match v11.clone() {
            Adt2::C0(_, _, _, _, h155, _, _, _, _, _, _) => {
                h155
            }
        };
        let a157 = 1u64;
        nat_eq(a156, a157)
    } else {
        false
    } {
        let a159 = match v9.clone() {
            Adt2::C0(_, _, h158, _, _, _, _, _, _, _, _) => {
                h158
            }
        };
        let a160 = 1u64;
        nat_eq(a159, a160)
    } else {
        false
    } {
        let a162 = match v9.clone() {
            Adt2::C0(_, _, _, h161, _, _, _, _, _, _, _) => {
                h161
            }
        };
        let a163 = 1u64;
        nat_eq(a162, a163)
    } else {
        false
    } {
        let a165 = match v9.clone() {
            Adt2::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                h164
            }
        };
        let a166 = 1u64;
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a168 = 3u64;
        let a169 = match v9.clone() {
            Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                h167
            }
        };
        nat_le(a168, a169)
    } else {
        false
    } {
        let a171 = match v5.clone() {
            Adt2::C0(_, h170, _, _, _, _, _, _, _, _, _) => {
                h170
            }
        };
        let a172 = 4u64;
        nat_eq(a171, a172)
    } else {
        false
    } {
        let a174 = match v5.clone() {
            Adt2::C0(_, _, h173, _, _, _, _, _, _, _, _) => {
                h173
            }
        };
        let a175 = 4u64;
        nat_eq(a174, a175)
    } else {
        false
    } {
        let a178 = match v5.clone() {
            Adt2::C0(_, _, _, h176, _, _, _, _, _, _, _) => {
                h176
            }
        };
        let a179 = match v7.clone() {
            Adt2::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                h177
            }
        };
        nat_eq(a178, a179)
    } else {
        false
    } {
        let a181 = match v5.clone() {
            Adt2::C0(_, _, _, _, h180, _, _, _, _, _, _) => {
                h180
            }
        };
        let a182 = 1u64;
        nat_eq(a181, a182)
    } else {
        false
    } {
        f26(v5.clone())?
    } else {
        false
    } {
        let a183 = 0u64;
        let a184 = v12;
        nat_lt(a183, a184)
    } else {
        false
    } {
        let a185 = v12;
        let a186 = 2147483648u64;
        nat_lt(a185, a186)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a188 = match v7.clone() {
                Adt2::C0(_, _, h187, _, _, _, _, _, _, _, _) => {
                    h187
                }
            };
            let a189 = f27(v12)?;
            nat_mul(a188, a189)?
        }, 4u64, f24(v7, 1u64), f24(v5, 1u64), Adt4::C38(f24(v9, 1u64), f24(v11, 1u64), match v0 {
            Adt1::C0(h190, _, _, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m191 = v0.clone();
        match m191 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m192 = v1.clone();
        match m192 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m193 = v2.clone();
        match m193 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    if if if v4 {
        false
    } else {
        let m194 = v0;
        match m194 {
            None => {
                false
            }
            Some(_) => {
                let m195 = v1;
                match m195 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m196 = v2;
                        match m196 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }
                }
            }
        }
    } {
        let a198 = match v7.clone() {
            Adt2::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
                h197
            }
        };
        let a199 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a198, a199) {
            let a201 = match v7.clone() {
                Adt2::C0(_, _, _, _, h200, _, _, _, _, _, _) => {
                    h200
                }
            };
            let a202 = 1u64;
            nat_eq(a201, a202)
        } else {
            false
        } {
            f19(v7.clone(), f20(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f25(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a204 = match v9.clone() {
                Adt2::C0(h203, _, _, _, _, _, _, _, _, _, _) => {
                    h203
                }
            };
            let a205 = 0u64;
            if if if nat_eq(a204, a205) {
                let a207 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h206, _, _, _, _, _, _) => {
                        h206
                    }
                };
                let a208 = 1u64;
                nat_eq(a207, a208)
            } else {
                false
            } {
                f19(v9.clone(), f20(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f25(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a210 = match v11.clone() {
                Adt2::C0(h209, _, _, _, _, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a211 = 0u64;
            if if if nat_eq(a210, a211) {
                let a213 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h212, _, _, _, _, _, _) => {
                        h212
                    }
                };
                let a214 = 1u64;
                nat_eq(a213, a214)
            } else {
                false
            } {
                f19(v11.clone(), f20(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f25(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a216 = match v7.clone() {
                Adt2::C0(_, _, _, h215, _, _, _, _, _, _, _) => {
                    h215
                }
            };
            let a217 = 1u64;
            nat_eq(a216, a217)
        } else {
            false
        } {
            let a219 = match v11.clone() {
                Adt2::C0(_, _, _, h218, _, _, _, _, _, _, _) => {
                    h218
                }
            };
            let a220 = 1u64;
            nat_eq(a219, a220)
        } else {
            false
        } {
            let a223 = match v9.clone() {
                Adt2::C0(_, h221, _, _, _, _, _, _, _, _, _) => {
                    h221
                }
            };
            let a224 = match v7.clone() {
                Adt2::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                    h222
                }
            };
            nat_eq(a223, a224)
        } else {
            false
        } {
            let a227 = match v9.clone() {
                Adt2::C0(_, _, _, h225, _, _, _, _, _, _, _) => {
                    h225
                }
            };
            let a228 = match v7.clone() {
                Adt2::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                    h226
                }
            };
            nat_eq(a227, a228)
        } else {
            false
        } {
            let a231 = match v11.clone() {
                Adt2::C0(_, h229, _, _, _, _, _, _, _, _, _) => {
                    h229
                }
            };
            let a232 = match v9.clone() {
                Adt2::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                    h230
                }
            };
            nat_eq(a231, a232)
        } else {
            false
        } {
            let a235 = match v11.clone() {
                Adt2::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
                    h233
                }
            };
            let a236 = match v7.clone() {
                Adt2::C0(_, _, h234, _, _, _, _, _, _, _, _) => {
                    h234
                }
            };
            nat_eq(a235, a236)
        } else {
            false
        } {
            let a238 = 0u64;
            let a239 = match v9.clone() {
                Adt2::C0(_, _, h237, _, _, _, _, _, _, _, _) => {
                    h237
                }
            };
            if nat_lt(a238, a239) {
                f28(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f26(v5.clone())?
        } else {
            false
        } {
            let m240 = v3.clone();
            match m240 {
                None => {
                    true
                }
                Some(v15) => {
                    let a242 = match v15.clone() {
                        Adt2::C0(h241, _, _, _, _, _, _, _, _, _, _) => {
                            h241
                        }
                    };
                    let a243 = 0u64;
                    if if if if if if nat_eq(a242, a243) {
                        let a245 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h244, _, _, _, _, _, _) => {
                                h244
                            }
                        };
                        let a246 = 1u64;
                        nat_eq(a245, a246)
                    } else {
                        false
                    } {
                        f19(v15.clone(), f20(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f25(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a249 = match v15.clone() {
                            Adt2::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        let a250 = match v9.clone() {
                            Adt2::C0(_, _, h248, _, _, _, _, _, _, _, _) => {
                                h248
                            }
                        };
                        nat_eq(a249, a250)
                    } else {
                        false
                    } {
                        let a253 = match v15.clone() {
                            Adt2::C0(_, _, h251, _, _, _, _, _, _, _, _) => {
                                h251
                            }
                        };
                        let a254 = match v9.clone() {
                            Adt2::C0(_, _, h252, _, _, _, _, _, _, _, _) => {
                                h252
                            }
                        };
                        nat_eq(a253, a254)
                    } else {
                        false
                    } {
                        let a257 = match v15 {
                            Adt2::C0(_, _, _, h255, _, _, _, _, _, _, _) => {
                                h255
                            }
                        };
                        let a258 = match v7.clone() {
                            Adt2::C0(_, _, h256, _, _, _, _, _, _, _, _) => {
                                h256
                            }
                        };
                        nat_eq(a257, a258)
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    } else {
        false
    } {
        let v17: Option<Adt3> = {
            let m259 = v3;
            match m259 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f24(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a261 = f20(v5.clone(), 1u64)?;
            let a262 = f29(match v9.clone() {
                Adt2::C0(_, _, h260, _, _, _, _, _, _, _, _) => {
                    h260
                }
            }, v17.clone())?;
            nat_mul(a261, a262)?
        }, 4u64, f24(v7, 1u64), f24(v5, 1u64), Adt4::C37(f24(v9, 1u64), f24(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f20(v1.clone(), 1u64)?;
    let a264 = match v1.clone() {
        Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
            h263
        }
    };
    let a265 = 0u64;
    if if if if if if if if nat_eq(a264, a265) {
        let a267 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h266, _, _, _, _, _) => {
                h266
            }
        };
        let a268 = 4u64;
        nat_eq(a267, a268)
    } else {
        false
    } {
        f30(match v0.clone() {
            Adt1::C0(h269, _, _, _, _, _, _, _, _, _, _, _) => {
                h269
            }
        })
    } else {
        false
    } {
        f30(match v0.clone() {
            Adt1::C0(_, h270, _, _, _, _, _, _, _, _, _, _) => {
                h270
            }
        })
    } else {
        false
    } {
        let a272 = 0u64;
        let a273 = match v0.clone() {
            Adt1::C0(_, _, h271, _, _, _, _, _, _, _, _, _) => {
                h271
            }
        };
        nat_lt(a272, a273)
    } else {
        false
    } {
        let a275 = match v0.clone() {
            Adt1::C0(_, _, h274, _, _, _, _, _, _, _, _, _) => {
                h274
            }
        };
        let a276 = 2139095040u64;
        nat_lt(a275, a276)
    } else {
        false
    } {
        let a277 = v2;
        let a278 = 9223372036854775808u64;
        nat_lt(a277, a278)
    } else {
        false
    } {
        let a280 = f31(v2)?;
        let a281 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h279, _) => {
                h279
            }
        };
        nat_le(a280, a281)
    } else {
        false
    } {
        Ok(Adt10::C4(f32(v2)?, 4u64, f24(v1.clone(), 1u64), f24(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        }, match v0.clone() {
            Adt1::C0(_, h283, _, _, _, _, _, _, _, _, _, _) => {
                h283
            }
        }, match v0 {
            Adt1::C0(_, _, h284, _, _, _, _, _, _, _, _, _) => {
                h284
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m285 = v3;
    match m285 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f33(v0, v2.clone());
            let v7: Adt3 = f33(v0, v5.clone());
            let v8: Adt3 = f33(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h286, _, _, _, _, _, _, _) => {
                    h286
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h287, _, _, _, _, _, _) => {
                    h287
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h288, _, _, _, _, _) => {
                    h288
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h289, _, _, _, _) => {
                    h289
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h290, _, _, _, _, _, _, _) => {
                    h290
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h291, _, _, _, _, _, _) => {
                    h291
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h292, _, _, _, _, _) => {
                    h292
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h293, _, _, _, _, _, _, _, _, _, _, _) => {
                    h293
                }
            };
            let v17: u64 = {
                let a298 = v13;
                let a299 = {
                    let a296 = {
                        let a294 = v9;
                        let a295 = 1u64;
                        nat_sub(a294, a295)
                    };
                    let a297 = v16;
                    nat_mul(a296, a297)?
                };
                nat_add(a298, a299)?
            };
            let v18: u64 = {
                let a304 = v14;
                let a305 = {
                    let a302 = {
                        let a300 = v10;
                        let a301 = 1u64;
                        nat_sub(a300, a301)
                    };
                    let a303 = v16;
                    nat_mul(a302, a303)?
                };
                nat_add(a304, a305)?
            };
            let v19: u64 = {
                let a308 = {
                    let a306 = v9;
                    let a307 = v10;
                    nat_mul(a306, a307)?
                };
                let a309 = v12;
                nat_mul(a308, a309)?
            };
            let v20: u64 = {
                let a312 = {
                    let a310 = v13;
                    let a311 = v14;
                    nat_mul(a310, a311)?
                };
                let a313 = v15;
                nat_mul(a312, a313)?
            };
            let v21: u64 = {
                let a314 = v19;
                let a315 = 32u64;
                if nat_lt(a314, a315) {
                    v19
                } else {
                    32u64
                }
            };
            let a317 = match v2.clone() {
                Adt2::C0(h316, _, _, _, _, _, _, _, _, _, _) => {
                    h316
                }
            };
            let a318 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a317, a318) {
                true
            } else {
                let a320 = match v2.clone() {
                    Adt2::C0(h319, _, _, _, _, _, _, _, _, _, _) => {
                        h319
                    }
                };
                let a321 = 1u64;
                nat_eq(a320, a321)
            } {
                let a323 = match v5.clone() {
                    Adt2::C0(h322, _, _, _, _, _, _, _, _, _, _) => {
                        h322
                    }
                };
                let a324 = 0u64;
                nat_eq(a323, a324)
            } else {
                false
            } {
                let a326 = match v4.clone() {
                    Adt2::C0(h325, _, _, _, _, _, _, _, _, _, _) => {
                        h325
                    }
                };
                let a327 = 0u64;
                nat_eq(a326, a327)
            } else {
                false
            } {
                let a328 = v16;
                let a329 = 2147483648u64;
                nat_lt(a328, a329)
            } else {
                false
            } {
                let a330 = 0u64;
                let a331 = v9;
                nat_lt(a330, a331)
            } else {
                false
            } {
                let a332 = 0u64;
                let a333 = v10;
                nat_lt(a332, a333)
            } else {
                false
            } {
                let a334 = 0u64;
                let a335 = v11;
                nat_lt(a334, a335)
            } else {
                false
            } {
                let a336 = 0u64;
                let a337 = v12;
                nat_lt(a336, a337)
            } else {
                false
            } {
                let a338 = 0u64;
                let a339 = v13;
                nat_lt(a338, a339)
            } else {
                false
            } {
                let a340 = 0u64;
                let a341 = v14;
                nat_lt(a340, a341)
            } else {
                false
            } {
                let a342 = 0u64;
                let a343 = v15;
                nat_lt(a342, a343)
            } else {
                false
            } {
                let a344 = 0u64;
                let a345 = v16;
                nat_lt(a344, a345)
            } else {
                false
            } {
                let a347 = match v6.clone() {
                    Adt3::C0(_, _, _, h346, _, _, _, _) => {
                        h346
                    }
                };
                let a348 = v11;
                nat_eq(a347, a348)
            } else {
                false
            } {
                let a350 = match v8.clone() {
                    Adt3::C0(h349, _, _, _, _, _, _, _) => {
                        h349
                    }
                };
                let a351 = v17;
                nat_eq(a350, a351)
            } else {
                false
            } {
                let a353 = match v8.clone() {
                    Adt3::C0(_, h352, _, _, _, _, _, _) => {
                        h352
                    }
                };
                let a354 = v18;
                nat_eq(a353, a354)
            } else {
                false
            } {
                let a356 = match v8.clone() {
                    Adt3::C0(_, _, h355, _, _, _, _, _) => {
                        h355
                    }
                };
                let a357 = v15;
                nat_eq(a356, a357)
            } else {
                false
            } {
                let a359 = match v8.clone() {
                    Adt3::C0(_, _, _, h358, _, _, _, _) => {
                        h358
                    }
                };
                let a360 = v12;
                nat_eq(a359, a360)
            } else {
                false
            } {
                f34(v4.clone())?
            } else {
                false
            } {
                let a365 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h361, _, _, _, _, _) => {
                        h361
                    }
                };
                let a366 = {
                    let a363 = match v2.clone() {
                        Adt2::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                            h362
                        }
                    };
                    let a364 = 1u64;
                    if nat_eq(a363, a364) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a365, a366)
            } else {
                false
            } {
                let a368 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h367, _, _, _, _, _) => {
                        h367
                    }
                };
                let a369 = 4u64;
                nat_eq(a368, a369)
            } else {
                false
            } {
                f25(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f25(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a374 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h370, _, _, _, _) => {
                            h370
                        }
                    };
                    let a375 = {
                        let a372 = v13;
                        let a373 = match v2 {
                            Adt2::C0(_, _, _, _, _, h371, _, _, _, _, _) => {
                                h371
                            }
                        };
                        nat_mul(a372, a373)?
                    };
                    nat_eq(a374, a375)
                } else {
                    let a377 = match v1.clone() {
                        Adt1::C0(_, h376, _, _, _, _, _, _, _, _, _, _) => {
                            h376
                        }
                    };
                    let a378 = 0u64;
                    if if if if if if nat_eq(a377, a378) {
                        let a380 = match v1 {
                            Adt1::C0(_, _, h379, _, _, _, _, _, _, _, _, _) => {
                                h379
                            }
                        };
                        let a381 = 1u64;
                        nat_eq(a380, a381)
                    } else {
                        false
                    } {
                        let a383 = match v2 {
                            Adt2::C0(_, _, _, _, h382, _, _, _, _, _, _) => {
                                h382
                            }
                        };
                        let a384 = 1u64;
                        nat_eq(a383, a384)
                    } else {
                        false
                    } {
                        let a386 = match v5.clone() {
                            Adt2::C0(_, _, _, h385, _, _, _, _, _, _, _) => {
                                h385
                            }
                        };
                        let a387 = 1u64;
                        nat_eq(a386, a387)
                    } else {
                        false
                    } {
                        let a389 = match v5 {
                            Adt2::C0(_, _, _, _, h388, _, _, _, _, _, _) => {
                                h388
                            }
                        };
                        let a390 = 1u64;
                        nat_eq(a389, a390)
                    } else {
                        false
                    } {
                        let a392 = match v4.clone() {
                            Adt2::C0(_, _, _, h391, _, _, _, _, _, _, _) => {
                                h391
                            }
                        };
                        let a393 = 1u64;
                        nat_eq(a392, a393)
                    } else {
                        false
                    } {
                        let a395 = match v4 {
                            Adt2::C0(_, _, _, _, h394, _, _, _, _, _, _) => {
                                h394
                            }
                        };
                        let a396 = 1u64;
                        nat_eq(a395, a396)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a397 = v11;
                    let a398 = v20;
                    nat_mul(a397, a398)?
                }, v21, {
                    let a399 = v21;
                    let a400 = v11;
                    nat_mul(a399, a400)?
                }, {
                    let a401 = v21;
                    let a402 = v20;
                    nat_mul(a401, a402)?
                }, {
                    let a407 = {
                        let a403 = v17;
                        let a404 = v18;
                        nat_mul(a403, a404)?
                    };
                    let a408 = {
                        let a405 = v15;
                        let a406 = v12;
                        nat_mul(a405, a406)?
                    };
                    nat_mul(a407, a408)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f6(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m409 = v3;
    match m409 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h410, _, _, _, _, _, _, _, _, _) => {
                    h410
                }
            }, match v1.clone() {
                Adt1::C0(h411, _, _, _, _, _, _, _, _, _, _, _) => {
                    h411
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h412, _, _, _, _, _, _, _, _) => {
                        h412
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h413, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h414, _, _, _, _, _) => {
                        h414
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h415, _, _, _, _, _, _, _) => {
                        h415
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h416, _, _, _, _, _, _, _, _) => {
                    h416
                }
            }, match v1.clone() {
                Adt1::C0(_, h417, _, _, _, _, _, _, _, _, _, _) => {
                    h417
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h418, _, _, _, _, _, _, _) => {
                        h418
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h419, _, _, _, _, _, _, _, _) => {
                        h419
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h420, _, _, _, _) => {
                        h420
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h421, _, _, _, _, _, _) => {
                        h421
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                        h422
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h423, _, _, _, _, _, _, _, _, _) => {
                        h423
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h424, _, _, _, _, _, _) => {
                        h424
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h425, _, _, _) => {
                        h425
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f35(v6.clone(), match v5.clone() {
                Adt2::C0(_, h426, _, _, _, _, _, _, _, _, _) => {
                    h426
                }
            })?;
            let v10: u64 = f35(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h427, _, _, _, _, _, _, _, _) => {
                    h427
                }
            })?;
            let v11: u64 = if v0 {
                f35(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h428, _, _, _, _, _, _, _) => {
                        h428
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h429, _, _) => {
                        h429
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h430, _, _, _, _, _, _, _) => {
                        h430
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h431, _) => {
                        h431
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h432, _, _, _, _, _, _) => {
                        h432
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h433) => {
                        h433
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h434, _, _, _, _, _, _) => {
                        h434
                    }
                }
            };
            let v15: u64 = {
                let a439 = {
                    let a435 = v9;
                    let a436 = v10;
                    nat_mul(a435, a436)?
                };
                let a440 = {
                    let a437 = v11;
                    let a438 = v13;
                    nat_mul(a437, a438)?
                };
                nat_mul(a439, a440)?
            };
            let v16: u64 = {
                let a448 = {
                    let a443 = match v6.clone() {
                        Adt6::C0(h441, _, _, _) => {
                            h441
                        }
                    };
                    let a444 = match v7.clone() {
                        Adt6::C0(h442, _, _, _) => {
                            h442
                        }
                    };
                    nat_mul(a443, a444)?
                };
                let a449 = {
                    let a446 = match v8.clone() {
                        Adt6::C0(h445, _, _, _) => {
                            h445
                        }
                    };
                    let a447 = v12;
                    nat_mul(a446, a447)?
                };
                nat_mul(a448, a449)?
            };
            let v17: u64 = {
                let a450 = v15;
                let a451 = 32u64;
                if nat_lt(a450, a451) {
                    v15
                } else {
                    32u64
                }
            };
            let a453 = match v2.clone() {
                Adt2::C0(h452, _, _, _, _, _, _, _, _, _, _) => {
                    h452
                }
            };
            let a454 = 0u64;
            if if if if if if if if if if if nat_eq(a453, a454) {
                true
            } else {
                let a456 = match v2.clone() {
                    Adt2::C0(h455, _, _, _, _, _, _, _, _, _, _) => {
                        h455
                    }
                };
                let a457 = 1u64;
                nat_eq(a456, a457)
            } {
                let a459 = match v5.clone() {
                    Adt2::C0(h458, _, _, _, _, _, _, _, _, _, _) => {
                        h458
                    }
                };
                let a460 = 0u64;
                nat_eq(a459, a460)
            } else {
                false
            } {
                let a462 = match v4.clone() {
                    Adt2::C0(h461, _, _, _, _, _, _, _, _, _, _) => {
                        h461
                    }
                };
                let a463 = 0u64;
                nat_eq(a462, a463)
            } else {
                false
            } {
                let a465 = match v4.clone() {
                    Adt2::C0(_, h464, _, _, _, _, _, _, _, _, _) => {
                        h464
                    }
                };
                let a466 = v9;
                nat_eq(a465, a466)
            } else {
                false
            } {
                let a468 = match v4.clone() {
                    Adt2::C0(_, _, h467, _, _, _, _, _, _, _, _) => {
                        h467
                    }
                };
                let a469 = v10;
                nat_eq(a468, a469)
            } else {
                false
            } {
                if v0 {
                    let a470 = 0u64;
                    let a471 = v12;
                    if if if if if if if nat_lt(a470, a471) {
                        let a472 = v12;
                        let a473 = 2147483648u64;
                        nat_lt(a472, a473)
                    } else {
                        false
                    } {
                        let a474 = v14;
                        let a475 = 2147483648u64;
                        nat_lt(a474, a475)
                    } else {
                        false
                    } {
                        let a476 = v13;
                        let a477 = 2147483648u64;
                        nat_lt(a476, a477)
                    } else {
                        false
                    } {
                        let a481 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h478, _, _, _, _, _, _) => {
                                h478
                            }
                        };
                        let a482 = {
                            let a479 = v12;
                            let a480 = v14;
                            nat_mul(a479, a480)?
                        };
                        nat_eq(a481, a482)
                    } else {
                        false
                    } {
                        let a486 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h483, _, _, _, _, _, _) => {
                                h483
                            }
                        };
                        let a487 = {
                            let a484 = v12;
                            let a485 = v13;
                            nat_mul(a484, a485)?
                        };
                        nat_eq(a486, a487)
                    } else {
                        false
                    } {
                        let a489 = match v4.clone() {
                            Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                                h488
                            }
                        };
                        let a490 = v11;
                        nat_eq(a489, a490)
                    } else {
                        false
                    } {
                        let a494 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h491, _, _, _, _, _, _) => {
                                h491
                            }
                        };
                        let a495 = {
                            let a492 = v14;
                            let a493 = v13;
                            nat_mul(a492, a493)?
                        };
                        nat_eq(a494, a495)
                    } else {
                        false
                    }
                } else {
                    let a497 = match v2.clone() {
                        Adt2::C0(_, _, _, h496, _, _, _, _, _, _, _) => {
                            h496
                        }
                    };
                    let a498 = v12;
                    if if if nat_eq(a497, a498) {
                        let a500 = match v4.clone() {
                            Adt2::C0(_, _, _, h499, _, _, _, _, _, _, _) => {
                                h499
                            }
                        };
                        let a501 = v14;
                        nat_eq(a500, a501)
                    } else {
                        false
                    } {
                        let a503 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h502, _, _, _, _, _, _) => {
                                h502
                            }
                        };
                        let a504 = v13;
                        nat_eq(a503, a504)
                    } else {
                        false
                    } {
                        let a505 = 0u64;
                        let a506 = v16;
                        nat_lt(a505, a506)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a507 = 0u64;
                let a508 = v14;
                nat_lt(a507, a508)
            } else {
                false
            } {
                let a509 = 0u64;
                let a510 = v15;
                nat_lt(a509, a510)
            } else {
                false
            } {
                f25(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f25(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a511 = v16;
                    let a512 = v14;
                    nat_mul(a511, a512)?
                }, v17, {
                    let a513 = v17;
                    let a514 = v16;
                    nat_mul(a513, a514)?
                }, {
                    let a515 = v17;
                    let a516 = v14;
                    nat_mul(a515, a516)?
                }, 0u64, f24(v2, 1u64), f24(v5, 1u64), f24(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f7(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m517 = v2;
    match m517 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m518 = v3;
            match m518 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a520 = match v1.clone() {
                        Adt2::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                            h519
                        }
                    };
                    let a521 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a520, a521) {
                        let a523 = match v5.clone() {
                            Adt2::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                                h522
                            }
                        };
                        let a524 = 0u64;
                        nat_eq(a523, a524)
                    } else {
                        false
                    } {
                        let a526 = match v6.clone() {
                            Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                                h525
                            }
                        };
                        let a527 = 0u64;
                        nat_eq(a526, a527)
                    } else {
                        false
                    } {
                        let a529 = match v4.clone() {
                            Adt2::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                                h528
                            }
                        };
                        let a530 = 0u64;
                        nat_eq(a529, a530)
                    } else {
                        false
                    } {
                        f28(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f28(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f34(v1.clone())?
                    } else {
                        false
                    } {
                        f34(v5.clone())?
                    } else {
                        false
                    } {
                        f34(v6)?
                    } else {
                        false
                    } {
                        f34(v4.clone())?
                    } else {
                        false
                    } {
                        let a536 = match v1.clone() {
                            Adt2::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
                                h531
                            }
                        };
                        let a537 = {
                            let a534 = match v5.clone() {
                                Adt2::C0(_, h532, _, _, _, _, _, _, _, _, _) => {
                                    h532
                                }
                            };
                            let a535 = match v5.clone() {
                                Adt2::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
                                    h533
                                }
                            };
                            nat_mul(a534, a535)?
                        };
                        nat_eq(a536, a537)
                    } else {
                        false
                    } {
                        let a543 = match v1.clone() {
                            Adt2::C0(_, _, h538, _, _, _, _, _, _, _, _) => {
                                h538
                            }
                        };
                        let a544 = {
                            let a541 = match v5.clone() {
                                Adt2::C0(_, _, h539, _, _, _, _, _, _, _, _) => {
                                    h539
                                }
                            };
                            let a542 = match v5.clone() {
                                Adt2::C0(_, _, _, h540, _, _, _, _, _, _, _) => {
                                    h540
                                }
                            };
                            nat_mul(a541, a542)?
                        };
                        nat_eq(a543, a544)
                    } else {
                        false
                    } {
                        let a547 = match v1.clone() {
                            Adt2::C0(_, _, _, h545, _, _, _, _, _, _, _) => {
                                h545
                            }
                        };
                        let a548 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h546, _, _, _, _, _, _) => {
                                h546
                            }
                        };
                        nat_eq(a547, a548)
                    } else {
                        false
                    } {
                        let a550 = match v0.clone() {
                            Adt1::C0(h549, _, _, _, _, _, _, _, _, _, _, _) => {
                                h549
                            }
                        };
                        let a551 = 4294967296u64;
                        nat_lt(a550, a551)
                    } else {
                        false
                    } {
                        let a553 = match v0.clone() {
                            Adt1::C0(h552, _, _, _, _, _, _, _, _, _, _, _) => {
                                h552
                            }
                        };
                        let a554 = 0u64;
                        if nat_eq(a553, a554) {
                            if f25(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a557 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h555) => {
                                        h555
                                    }
                                };
                                let a558 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h556) => {
                                        h556
                                    }
                                };
                                nat_eq(a557, a558)
                            }
                        } else {
                            let a561 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h559) => {
                                    h559
                                }
                            };
                            let a562 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h560) => {
                                    h560
                                }
                            };
                            nat_eq(a561, a562)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a564 = match v4.clone() {
                                Adt2::C0(_, h563, _, _, _, _, _, _, _, _, _) => {
                                    h563
                                }
                            };
                            let a565 = 0u64;
                            if if if nat_eq(a564, a565) {
                                true
                            } else {
                                let a567 = match v4.clone() {
                                    Adt2::C0(_, _, h566, _, _, _, _, _, _, _, _) => {
                                        h566
                                    }
                                };
                                let a568 = 0u64;
                                nat_eq(a567, a568)
                            } {
                                true
                            } else {
                                let a570 = match v4.clone() {
                                    Adt2::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                                        h569
                                    }
                                };
                                let a571 = 0u64;
                                nat_eq(a570, a571)
                            } {
                                true
                            } else {
                                let a573 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h572, _, _, _, _, _, _) => {
                                        h572
                                    }
                                };
                                let a574 = 0u64;
                                nat_eq(a573, a574)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                                h575
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a577 = match v0 {
                                Adt1::C0(h576, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h576
                                }
                            };
                            let a578 = 0u64;
                            if nat_eq(a577, a578) {
                                f20(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f36(v8, match v5.clone() {
                                Adt2::C0(_, _, h579, _, _, _, _, _, _, _, _) => {
                                    h579
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h580, _, _, _, _, _, _, _) => {
                                    h580
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h581, _, _, _, _, _, _) => {
                                    h581
                                }
                            })?
                        };
                        Ok(Adt10::C4(f37(v9, v8, v10)?, 4u64, f24(v1, 1u64), f24(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f38(v2.clone());
    let v6: Adt14 = f38(v4.clone());
    let v8: Adt15 = {
        let m582 = v3;
        match m582 {
            None => {
                f39(v0, match v1.clone() {
                    Adt1::C0(h583, _, _, _, _, _, _, _, _, _, _, _) => {
                        h583
                    }
                }, match v1 {
                    Adt1::C0(_, h584, _, _, _, _, _, _, _, _, _, _) => {
                        h584
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f39(v0, match v1.clone() {
                    Adt1::C0(h585, _, _, _, _, _, _, _, _, _, _, _) => {
                        h585
                    }
                }, match v1 {
                    Adt1::C0(_, h586, _, _, _, _, _, _, _, _, _, _) => {
                        h586
                    }
                }, true, v5, f38(v7), v6)?
            }
        }
    };
    let m587 = v8;
    match m587 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f24(v2, 1u64), f24(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f9(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
            h588
        }
    };
    let v4: u64 = f40(v3);
    let a590 = match v1.clone() {
        Adt2::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    };
    let a591 = 0u64;
    if if if if if if if if nat_eq(a590, a591) {
        true
    } else {
        let a593 = match v1.clone() {
            Adt2::C0(h592, _, _, _, _, _, _, _, _, _, _) => {
                h592
            }
        };
        let a594 = 30u64;
        nat_eq(a593, a594)
    } {
        let a597 = match v1.clone() {
            Adt2::C0(h595, _, _, _, _, _, _, _, _, _, _) => {
                h595
            }
        };
        let a598 = match v2.clone() {
            Adt2::C0(h596, _, _, _, _, _, _, _, _, _, _) => {
                h596
            }
        };
        nat_eq(a597, a598)
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        f41(match v0.clone() {
            Adt1::C0(h599, _, _, _, _, _, _, _, _, _, _, _) => {
                h599
            }
        })
    } else {
        false
    } {
        f42(v1.clone(), v4)
    } else {
        false
    } {
        f42(v2.clone(), v4)
    } else {
        false
    } {
        if f25(v1.clone(), v2.clone())? {
            true
        } else {
            f43(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f44(match v1.clone() {
            Adt2::C0(_, h600, _, _, _, _, _, _, _, _, _) => {
                h600
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h601, _, _, _, _, _, _, _, _) => {
                h601
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h602, _, _, _, _, _, _, _) => {
                h602
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h603, _, _, _, _, _, _) => {
                h603
            }
        }) {
            0u64
        } else {
            let a609 = match v1.clone() {
                Adt2::C0(_, _, h604, _, _, _, _, _, _, _, _) => {
                    h604
                }
            };
            let a610 = {
                let a607 = match v1.clone() {
                    Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                        h605
                    }
                };
                let a608 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h606, _, _, _, _, _, _) => {
                        h606
                    }
                };
                nat_mul(a607, a608)?
            };
            nat_mul(a609, a610)?
        };
        let v6: u64 = {
            let a611 = v5;
            let a612 = 0u64;
            if if nat_eq(a611, a612) {
                true
            } else {
                f45(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a614 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h613, _) => {
                        h613
                    }
                };
                let a615 = v4;
                let a616 = 0u64;
                nat_quot(a614, a615, a616)
            }
        };
        Ok(Adt10::C4(f46(v3, match v1.clone() {
            Adt2::C0(_, h617, _, _, _, _, _, _, _, _, _) => {
                h617
            }
        }, v5, v6)?, v4, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h618, _, _, _, _, _, _, _, _, _, _, _) => {
                h618
            }
        }, v5, f47(match v1 {
            Adt2::C0(_, h619, _, _, _, _, _, _, _, _, _) => {
                h619
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f10(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a621 = match v1.clone() {
        Adt2::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
            h620
        }
    };
    let a622 = 0u64;
    if if if if if nat_eq(a621, a622) {
        let a624 = match v2.clone() {
            Adt2::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                h623
            }
        };
        let a625 = 0u64;
        nat_eq(a624, a625)
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        f48(match v0.clone() {
            Adt1::C0(h626, _, _, _, _, _, _, _, _, _, _, _) => {
                h626
            }
        })
    } else {
        false
    } {
        if f25(v1.clone(), v2.clone())? {
            true
        } else {
            f43(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f24(v1, 1u64);
        let v4: u64 = {
            let a632 = match v3.clone() {
                Adt3::C0(_, h627, _, _, _, _, _, _) => {
                    h627
                }
            };
            let a633 = {
                let a630 = match v3.clone() {
                    Adt3::C0(_, _, h628, _, _, _, _, _) => {
                        h628
                    }
                };
                let a631 = match v3.clone() {
                    Adt3::C0(_, _, _, h629, _, _, _, _) => {
                        h629
                    }
                };
                nat_mul(a630, a631)?
            };
            nat_mul(a632, a633)?
        };
        Ok(Adt10::C4(f49(match v3.clone() {
            Adt3::C0(h634, _, _, _, _, _, _, _) => {
                h634
            }
        }, v4)?, 4u64, v3, f24(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h635, _, _, _, _, _, _, _, _, _, _, _) => {
                h635
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f11(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m636 = v1;
    match m636 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m637 = v2;
            match m637 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a639 = match v0.clone() {
                        Adt2::C0(h638, _, _, _, _, _, _, _, _, _, _) => {
                            h638
                        }
                    };
                    let a640 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a639, a640) {
                        let a642 = match v4.clone() {
                            Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                                h641
                            }
                        };
                        let a643 = 0u64;
                        nat_eq(a642, a643)
                    } else {
                        false
                    } {
                        let a645 = match v5.clone() {
                            Adt2::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
                                h644
                            }
                        };
                        let a646 = 26u64;
                        nat_eq(a645, a646)
                    } else {
                        false
                    } {
                        let a648 = match v3.clone() {
                            Adt2::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
                                h647
                            }
                        };
                        let a649 = 0u64;
                        nat_eq(a648, a649)
                    } else {
                        false
                    } {
                        f28(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a652 = match v0.clone() {
                            Adt2::C0(_, h650, _, _, _, _, _, _, _, _, _) => {
                                h650
                            }
                        };
                        let a653 = match v4.clone() {
                            Adt2::C0(_, h651, _, _, _, _, _, _, _, _, _) => {
                                h651
                            }
                        };
                        nat_eq(a652, a653)
                    } else {
                        false
                    } {
                        let a656 = match v0.clone() {
                            Adt2::C0(_, _, h654, _, _, _, _, _, _, _, _) => {
                                h654
                            }
                        };
                        let a657 = match v5.clone() {
                            Adt2::C0(_, h655, _, _, _, _, _, _, _, _, _) => {
                                h655
                            }
                        };
                        nat_eq(a656, a657)
                    } else {
                        false
                    } {
                        let a660 = match v0.clone() {
                            Adt2::C0(_, _, _, h658, _, _, _, _, _, _, _) => {
                                h658
                            }
                        };
                        let a661 = match v5.clone() {
                            Adt2::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                                h659
                            }
                        };
                        nat_eq(a660, a661)
                    } else {
                        false
                    } {
                        let a663 = 0u64;
                        let a664 = match v5.clone() {
                            Adt2::C0(_, _, _, h662, _, _, _, _, _, _, _) => {
                                h662
                            }
                        };
                        nat_lt(a663, a664)
                    } else {
                        false
                    } {
                        let a666 = 0u64;
                        let a667 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                                h665
                            }
                        };
                        nat_lt(a666, a667)
                    } else {
                        false
                    } {
                        let a669 = 4u64;
                        let a670 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h668, _, _, _, _, _) => {
                                h668
                            }
                        };
                        nat_le(a669, a670)
                    } else {
                        false
                    } {
                        let a672 = 4u64;
                        let a673 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h671, _, _, _, _, _) => {
                                h671
                            }
                        };
                        nat_le(a672, a673)
                    } else {
                        false
                    } {
                        let a675 = 4u64;
                        let a676 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
                                h674
                            }
                        };
                        nat_le(a675, a676)
                    } else {
                        false
                    } {
                        let a678 = 4u64;
                        let a679 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h677, _, _, _, _, _) => {
                                h677
                            }
                        };
                        nat_le(a678, a679)
                    } else {
                        false
                    } {
                        f25(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f25(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f25(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f50(match v0.clone() {
                            Adt2::C0(_, h680, _, _, _, _, _, _, _, _, _) => {
                                h680
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h681, _, _, _, _, _, _, _, _) => {
                                h681
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h682, _, _, _, _, _, _, _) => {
                                h682
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                                h683
                            }
                        })?, 4u64, f24(v0, 1u64), f24(v3, 1u64), Adt4::C22(f24(v4, 1u64), f24(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f12(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a684 = v0;
    let a685 = 28u64;
    if if if nat_eq(a684, a685) {
        true
    } else {
        let a686 = v0;
        let a687 = 29u64;
        nat_eq(a686, a687)
    } {
        let a689 = 0u64;
        let a690 = match v1.clone() {
            Adt1::C0(h688, _, _, _, _, _, _, _, _, _, _, _) => {
                h688
            }
        };
        if nat_lt(a689, a690) {
            true
        } else {
            let a692 = 0u64;
            let a693 = match v1.clone() {
                Adt1::C0(_, _, _, h691, _, _, _, _, _, _, _, _) => {
                    h691
                }
            };
            nat_lt(a692, a693)
        }
    } else {
        false
    } {
        let m697 = f51(match v2.clone() {
            Adt2::C0(h694, _, _, _, _, _, _, _, _, _, _) => {
                h694
            }
        }, match v1.clone() {
            Adt1::C0(h695, _, _, _, _, _, _, _, _, _, _, _) => {
                h695
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h696, _, _, _, _, _, _, _, _) => {
                h696
            }
        });
        match m697 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f52(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f52(v0, v1, v2, v3, v4)
    }
}

pub fn f13(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h698, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h698
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h699, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h699
        }
    };
    let v6: u64 = {
        let a700 = v3;
        let a701 = v5;
        let a702 = 0u64;
        nat_rem(a700, a701, a702)
    };
    let a703 = v1;
    let a704 = 0u64;
    if nat_eq(a703, a704) {
        let a706 = v3;
        let a707 = match v0.clone() {
            Adt8::C0(_, _, _, h705, _, _, _, _, _, _, _, _, _, _, _) => {
                h705
            }
        };
        if nat_lt(a706, a707) {
            Ok(Adt9::C8(Some(f53(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h708, _, _, _, _, _, _) => {
                    h708
                }
            }, {
                let a712 = v6;
                let a713 = {
                    let m711 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h710, _, _, _, _, _, _) => {
                            h710
                        }
                    };
                    match m711 {
                        Adt3::C0(h709, _, _, _, _, _, _, _) => {
                            h709
                        }
                    }
                };
                let a714 = 0u64;
                nat_rem(a712, a713, a714)
            }, {
                let a724 = {
                    let a718 = v6;
                    let a719 = {
                        let m717 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h716, _, _, _, _, _, _) => {
                                h716
                            }
                        };
                        match m717 {
                            Adt3::C0(h715, _, _, _, _, _, _, _) => {
                                h715
                            }
                        }
                    };
                    let a720 = 0u64;
                    nat_quot(a718, a719, a720)
                };
                let a725 = {
                    let m723 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h722, _, _, _, _, _, _) => {
                            h722
                        }
                    };
                    match m723 {
                        Adt3::C0(_, h721, _, _, _, _, _, _) => {
                            h721
                        }
                    }
                };
                let a726 = 0u64;
                nat_rem(a724, a725, a726)
            }, {
                let a736 = {
                    let a730 = v6;
                    let a731 = {
                        let m729 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h728, _, _, _, _, _, _) => {
                                h728
                            }
                        };
                        match m729 {
                            Adt3::C0(h727, _, _, _, _, _, _, _) => {
                                h727
                            }
                        }
                    };
                    let a732 = 0u64;
                    nat_quot(a730, a731, a732)
                };
                let a737 = {
                    let m735 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h734, _, _, _, _, _, _) => {
                            h734
                        }
                    };
                    match m735 {
                        Adt3::C0(_, h733, _, _, _, _, _, _) => {
                            h733
                        }
                    }
                };
                let a738 = 0u64;
                nat_quot(a736, a737, a738)
            }, {
                let a739 = v3;
                let a740 = v5;
                let a741 = 0u64;
                nat_quot(a739, a740, a741)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a747 = v2;
            let a748 = {
                let a744 = v3;
                let a745 = {
                    let a742 = v1;
                    let a743 = 1u64;
                    if nat_eq(a742, a743) {
                        v4
                    } else {
                        v5
                    }
                };
                let a746 = 0u64;
                nat_quot(a744, a745, a746)
            };
            nat_add(a747, a748)?
        };
        let a750 = v7;
        let a751 = match v0.clone() {
            Adt8::C0(h749, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h749
            }
        };
        if nat_lt(a750, a751) {
            let v8: u64 = {
                let a755 = v7;
                let a756 = {
                    let m754 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h753, _, _, _, _, _) => {
                            h753
                        }
                    };
                    match m754 {
                        Adt3::C0(h752, _, _, _, _, _, _, _) => {
                            h752
                        }
                    }
                };
                let a757 = 0u64;
                nat_rem(a755, a756, a757)
            };
            let v9: u64 = {
                let a761 = v7;
                let a762 = {
                    let m760 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h759, _, _, _, _, _) => {
                            h759
                        }
                    };
                    match m760 {
                        Adt3::C0(h758, _, _, _, _, _, _, _) => {
                            h758
                        }
                    }
                };
                let a763 = 0u64;
                nat_quot(a761, a762, a763)
            };
            let v10: u64 = {
                let a767 = v9;
                let a768 = {
                    let m766 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h765, _, _, _, _, _) => {
                            h765
                        }
                    };
                    match m766 {
                        Adt3::C0(_, h764, _, _, _, _, _, _) => {
                            h764
                        }
                    }
                };
                let a769 = 0u64;
                nat_rem(a767, a768, a769)
            };
            let v11: u64 = {
                let a773 = v9;
                let a774 = {
                    let m772 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h771, _, _, _, _, _) => {
                            h771
                        }
                    };
                    match m772 {
                        Adt3::C0(_, h770, _, _, _, _, _, _) => {
                            h770
                        }
                    }
                };
                let a775 = 0u64;
                nat_quot(a773, a774, a775)
            };
            let a776 = v1;
            let a777 = 1u64;
            if nat_eq(a776, a777) {
                Ok(Adt9::C8(Some(f53(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h778, _, _, _, _, _) => {
                        h778
                    }
                }, v8, v10, {
                    let a779 = v3;
                    let a780 = v4;
                    let a781 = 0u64;
                    nat_rem(a779, a780, a781)
                }, v11)?)))
            } else {
                let a782 = v1;
                let a783 = 2u64;
                if nat_eq(a782, a783) {
                    Ok(Adt9::C8(Some({
                        let a829 = f53(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h784, _, _, _, _) => {
                                h784
                            }
                        }, {
                            let a796 = {
                                let a788 = v8;
                                let a789 = {
                                    let m787 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h786, _, _, _) => {
                                            h786
                                        }
                                    };
                                    match m787 {
                                        Adt6::C0(_, h785, _, _) => {
                                            h785
                                        }
                                    }
                                };
                                nat_mul(a788, a789)?
                            };
                            let a797 = {
                                let a793 = v6;
                                let a794 = {
                                    let m792 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h791, _, _, _, _, _, _) => {
                                            h791
                                        }
                                    };
                                    match m792 {
                                        Adt3::C0(h790, _, _, _, _, _, _, _) => {
                                            h790
                                        }
                                    }
                                };
                                let a795 = 0u64;
                                nat_rem(a793, a794, a795)
                            };
                            nat_add(a796, a797)?
                        }, {
                            let a815 = {
                                let a801 = v10;
                                let a802 = {
                                    let m800 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h799, _, _, _) => {
                                            h799
                                        }
                                    };
                                    match m800 {
                                        Adt6::C0(_, h798, _, _) => {
                                            h798
                                        }
                                    }
                                };
                                nat_mul(a801, a802)?
                            };
                            let a816 = {
                                let a812 = {
                                    let a806 = v6;
                                    let a807 = {
                                        let m805 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h804, _, _, _, _, _, _) => {
                                                h804
                                            }
                                        };
                                        match m805 {
                                            Adt3::C0(h803, _, _, _, _, _, _, _) => {
                                                h803
                                            }
                                        }
                                    };
                                    let a808 = 0u64;
                                    nat_quot(a806, a807, a808)
                                };
                                let a813 = {
                                    let m811 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h810, _, _, _, _, _, _) => {
                                            h810
                                        }
                                    };
                                    match m811 {
                                        Adt3::C0(_, h809, _, _, _, _, _, _) => {
                                            h809
                                        }
                                    }
                                };
                                let a814 = 0u64;
                                nat_rem(a812, a813, a814)
                            };
                            nat_add(a815, a816)?
                        }, {
                            let a826 = {
                                let a820 = v6;
                                let a821 = {
                                    let m819 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h818, _, _, _, _, _, _) => {
                                            h818
                                        }
                                    };
                                    match m819 {
                                        Adt3::C0(h817, _, _, _, _, _, _, _) => {
                                            h817
                                        }
                                    }
                                };
                                let a822 = 0u64;
                                nat_quot(a820, a821, a822)
                            };
                            let a827 = {
                                let m825 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h824, _, _, _, _, _, _) => {
                                        h824
                                    }
                                };
                                match m825 {
                                    Adt3::C0(_, h823, _, _, _, _, _, _) => {
                                        h823
                                    }
                                }
                            };
                            let a828 = 0u64;
                            nat_quot(a826, a827, a828)
                        }, v11)?;
                        let a830 = 4u64;
                        let a831 = 0u64;
                        nat_quot(a829, a830, a831)
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

pub fn f14(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a832 = v1;
    let a833 = 0u64;
    if nat_eq(a832, a833) {
        let a835 = v3;
        let a836 = match v0.clone() {
            Adt8::C0(_, _, _, h834, _, _, _, _, _, _, _, _, _, _, _) => {
                h834
            }
        };
        if nat_lt(a835, a836) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h837, _, _, _, _, _, _) => {
                    h837
                }
            };
            let v5: u64 = {
                let a839 = v3;
                let a840 = match v0.clone() {
                    Adt8::C0(_, _, h838, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h838
                    }
                };
                let a841 = 0u64;
                nat_quot(a839, a840, a841)
            };
            Ok(Adt9::C8(Some(f53(v4.clone(), f54(v5, v4.clone(), 0u64), f54(v5, v4.clone(), 1u64), f54(v5, v4, 2u64), f55(v0.clone(), v5, {
                let a843 = v3;
                let a844 = match v0 {
                    Adt8::C0(_, _, h842, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h842
                    }
                };
                let a845 = 0u64;
                nat_rem(a843, a844, a845)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h846, _, _, _, _) => {
                h846
            }
        };
        let v7: u64 = {
            let a847 = v1;
            let a848 = 1u64;
            if nat_eq(a847, a848) {
                match v0.clone() {
                    Adt8::C0(_, h849, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h849
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h850, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h850
                    }
                }
            }
        };
        let v8: u64 = {
            let a854 = v2;
            let a855 = {
                let a851 = v3;
                let a852 = v7;
                let a853 = 0u64;
                nat_quot(a851, a852, a853)
            };
            nat_add(a854, a855)?
        };
        let v9: u64 = {
            let a856 = v3;
            let a857 = v7;
            let a858 = 0u64;
            nat_rem(a856, a857, a858)
        };
        let v10: u64 = {
            let a860 = v8;
            let a861 = match v6.clone() {
                Adt3::C0(h859, _, _, _, _, _, _, _) => {
                    h859
                }
            };
            let a862 = 0u64;
            nat_rem(a860, a861, a862)
        };
        let v11: u64 = {
            let a864 = v8;
            let a865 = match v6.clone() {
                Adt3::C0(h863, _, _, _, _, _, _, _) => {
                    h863
                }
            };
            let a866 = 0u64;
            nat_quot(a864, a865, a866)
        };
        let v12: u64 = {
            let a868 = v11;
            let a869 = match v6.clone() {
                Adt3::C0(_, h867, _, _, _, _, _, _) => {
                    h867
                }
            };
            let a870 = 0u64;
            nat_rem(a868, a869, a870)
        };
        let v13: u64 = {
            let a872 = v11;
            let a873 = match v6.clone() {
                Adt3::C0(_, h871, _, _, _, _, _, _) => {
                    h871
                }
            };
            let a874 = 0u64;
            nat_quot(a872, a873, a874)
        };
        let a876 = v8;
        let a877 = match v0.clone() {
            Adt8::C0(h875, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h875
            }
        };
        if nat_lt(a876, a877) {
            let a878 = v1;
            let a879 = 1u64;
            if nat_eq(a878, a879) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h880, _, _, _, _, _, _) => {
                        h880
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h881, _, _, _, _, _) => {
                        h881
                    }
                };
                let m884 = f56(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h882, _, _, _) => {
                        h882
                    }
                }, match v15.clone() {
                    Adt3::C0(h883, _, _, _, _, _, _, _) => {
                        h883
                    }
                }, v10, f54(v9, v14.clone(), 0u64))?;
                match m884 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m887 = f56(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h885, _, _) => {
                                h885
                            }
                        }, match v15 {
                            Adt3::C0(_, h886, _, _, _, _, _, _) => {
                                h886
                            }
                        }, v12, f54(v9, v14, 1u64))?;
                        match m887 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f57(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a888 = v1;
                let a889 = 2u64;
                if nat_eq(a888, a889) {
                    Ok(Adt9::C8(Some({
                        let a891 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h890) => {
                                h890
                            }
                        };
                        let a892 = 0u64;
                        if nat_eq(a891, a892) {
                            f53(v6, v10, v12, v9, v13)?
                        } else {
                            f53(v6.clone(), v10, v12, {
                                let a894 = v13;
                                let a895 = match v6.clone() {
                                    Adt3::C0(_, _, h893, _, _, _, _, _) => {
                                        h893
                                    }
                                };
                                let a896 = 0u64;
                                nat_rem(a894, a895, a896)
                            }, {
                                let a904 = {
                                    let a902 = {
                                        let a898 = v13;
                                        let a899 = match v6 {
                                            Adt3::C0(_, _, h897, _, _, _, _, _) => {
                                                h897
                                            }
                                        };
                                        let a900 = 0u64;
                                        nat_quot(a898, a899, a900)
                                    };
                                    let a903 = match v0 {
                                        Adt8::C0(_, _, h901, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h901
                                        }
                                    };
                                    nat_mul(a902, a903)?
                                };
                                let a905 = v9;
                                nat_add(a904, a905)?
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

pub fn f15(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f16(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f16(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f58(v3.clone(), {
        let m906 = v3.clone();
        match m906 {
            Adt4::C0 => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f69(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C19(v69, v70) => {
                f70(v69, v0, v1, v2, v70, v4)?
            }
            Adt4::C20(v71, v72) => {
                f71(v71, v0, v1, v2, v72, v4)?
            }
            Adt4::C21 => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C22(v73, v74) => {
                f72(v0, v1, v2, v73, v74, v4)?
            }
            Adt4::C23(v88, v89, v90, v91) => {
                f79(v0, v91, v88, v89, v90, v4)?
            }
            Adt4::C24(v87) => {
                if v5 {
                    f77(Adt16::C0(v87), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f59(v0, v1, v2, v3)?
            }
            Adt4::C26(v83, v84, v85, v86) => {
                let a908 = v83;
                let a909 = 64u64;
                if if if nat_eq(a908, a909) {
                    true
                } else {
                    let a910 = v83;
                    let a911 = 65u64;
                    nat_eq(a910, a911)
                } {
                    true
                } else {
                    let a912 = v83;
                    let a913 = 66u64;
                    nat_eq(a912, a913)
                } {
                    if v5 {
                        f77(Adt16::C1(v83, v84, v85, v86), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a914 = v83;
                    let a915 = 0u64;
                    if if nat_eq(a914, a915) {
                        true
                    } else {
                        let a916 = v83;
                        let a917 = 30u64;
                        nat_eq(a916, a917)
                    } {
                        f78(v0, v1, v2, v83, v84, v85, v86, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v82) => {
                f76(v0, v1, v2, v82)?
            }
            Adt4::C28(v78) => {
                f73(v0, v1, v2, v78, v4)?
            }
            Adt4::C29(v79) => {
                f74(v0, v1, v2, v79)?
            }
            Adt4::C30(v80, _) => {
                f75(v0, v1, v2, v80)?
            }
            Adt4::C31(v33, v34) => {
                f60(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f61(v0, match v1 {
                    Adt3::C0(h907, _, _, _, _, _, _, _) => {
                        h907
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f62(v0, v1, v36, v2)?
            }
            Adt4::C34(v49, v50, v51, v52) => {
                f68(v0, v1, v49, v2, v50, v51, v52)?
            }
            Adt4::C35(v48) => {
                f67(v0, v48, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f63(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f64(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f65(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f66(v0, v1, v47)?
            }
        }
    }))
}

pub fn f17(v0: Adt4) -> R<Adt9> {
    let m918 = v0;
    match m918 {
        Adt4::C0 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C1 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C2(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C4(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C7(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C8(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt9::C6(0u64, v18, v19, true, 3u64))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt9::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C11(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C15 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C16(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C17 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C18(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C21 => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C22(_, _) => {
            Ok(Adt9::C6(7u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt9::C6(0u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C24(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C25(v66, _, v68) => {
            Ok(Adt9::C6({
                let a924 = {
                    let a920 = 0u64;
                    let a921 = v66;
                    if nat_lt(a920, a921) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a925 = {
                    let a922 = 0u64;
                    let a923 = v68;
                    if nat_lt(a922, a923) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a924, a925)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v74, _, _, v77) => {
            Ok(Adt9::C6({
                let a926 = v74;
                let a927 = 66u64;
                if nat_eq(a926, a927) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v77))
        }
        Adt4::C27(_) => {
            Ok(Adt9::C6(2u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C28(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C29(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C30(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C31(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C32(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C33(_) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C34(_, _, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C35(v39) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v39))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m919 = v32;
                match m919 {
                    None => {
                        7u64
                    }
                    Some(_) => {
                        15u64
                    }
                }
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C38(_, _, _, _) => {
            Ok(Adt9::C6(7u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C39(v38) => {
            Ok(Adt9::C6(if v38 {
                31u64
            } else {
                7u64
            }, 0u64, 0u64, false, 12u64))
        }
    }
}

pub fn f18(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a928 = v1;
    let a929 = 5u64;
    if if if if if nat_lt(a928, a929) {
        true
    } else {
        f80(v0, v1, v2, v3, v4)
    } {
        let a930 = v5;
        let a931 = 5u64;
        nat_lt(a930, a931)
    } else {
        false
    } {
        let a932 = v7;
        let a933 = 5u64;
        nat_lt(a932, a933)
    } else {
        false
    } {
        let a934 = v9;
        let a935 = 2u64;
        if nat_lt(a934, a935) {
            true
        } else {
            let a936 = v9;
            let a937 = 3u64;
            nat_eq(a936, a937)
        }
    } else {
        false
    } {
        Adt9::C7(f81(v0, v1, v2, v3, v4), f82(v5, v6), f82(v7, v8), f83({
            let a938 = v9;
            let a939 = 0u64;
            if nat_eq(a938, a939) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f19(v0: Adt2, v1: u64) -> R<bool> {
    let a940 = v1;
    let a941 = 0u64;
    if nat_eq(a940, a941) {
        Ok(true)
    } else {
        let a947 = {
            let a944 = f84({
                let a942 = v1;
                let a943 = 1u64;
                nat_sub(a942, a943)
            }, f24(v0.clone(), 1u64))?;
            let a945 = 4u64;
            nat_add(a944, a945)?
        };
        let a948 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h946, _) => {
                h946
            }
        };
        Ok(nat_le(a947, a948))
    }
}

pub fn f20(v0: Adt2, v1: u64) -> R<u64> {
    let a960 = {
        let a954 = {
            let a950 = match v0.clone() {
                Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                    h949
                }
            };
            let a951 = v1;
            let a952 = 0u64;
            nat_quot(a950, a951, a952)
        };
        let a955 = match v0.clone() {
            Adt2::C0(_, _, h953, _, _, _, _, _, _, _, _) => {
                h953
            }
        };
        nat_mul(a954, a955)?
    };
    let a961 = {
        let a958 = match v0.clone() {
            Adt2::C0(_, _, _, h956, _, _, _, _, _, _, _) => {
                h956
            }
        };
        let a959 = match v0 {
            Adt2::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                h957
            }
        };
        nat_mul(a958, a959)?
    };
    nat_mul(a960, a961)
}

pub fn f21(v0: Adt2, v1: Adt2) -> R<bool> {
    let a963 = match v1.clone() {
        Adt2::C0(h962, _, _, _, _, _, _, _, _, _, _) => {
            h962
        }
    };
    let a964 = 0u64;
    if if nat_eq(a963, a964) {
        f28(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a965 = f20(v0.clone(), 1u64)?;
        let a966 = 0u64;
        if nat_eq(a965, a966) {
            Ok(true)
        } else {
            let a972 = {
                let a969 = f84({
                    let a967 = f20(v0.clone(), 1u64)?;
                    let a968 = 1u64;
                    nat_sub(a967, a968)
                }, f24(v0, 1u64))?;
                let a970 = 4u64;
                nat_add(a969, a970)?
            };
            let a973 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h971, _) => {
                    h971
                }
            };
            Ok(nat_le(a972, a973))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        4u64
    }
}

pub fn f23(v0: bool) -> u64 {
    if v0 {
        15u64
    } else {
        3u64
    }
}

pub fn f24(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a975 = match v0.clone() {
            Adt2::C0(_, h974, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a976 = v1;
        let a977 = 0u64;
        nat_quot(a975, a976, a977)
    }, match v0.clone() {
        Adt2::C0(_, _, h978, _, _, _, _, _, _, _, _) => {
            h978
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h979, _, _, _, _, _, _, _) => {
            h979
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h980, _, _, _, _, _, _) => {
            h980
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h981, _, _, _, _, _) => {
            h981
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h982, _, _, _, _) => {
            h982
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h983, _, _, _) => {
            h983
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h984, _, _) => {
            h984
        }
    })
}

pub fn f25(v0: Adt2, v1: Adt2) -> R<bool> {
    let a986 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h985, _) => {
            h985
        }
    };
    let a987 = 0u64;
    if nat_eq(a986, a987) {
        Ok(true)
    } else {
        let a989 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h988, _) => {
                h988
            }
        };
        let a990 = 0u64;
        if nat_eq(a989, a990) {
            Ok(true)
        } else {
            let a992 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h991) => {
                    h991
                }
            };
            let a993 = 0u64;
            if nat_eq(a992, a993) {
                Ok(true)
            } else {
                let a995 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h994) => {
                        h994
                    }
                };
                let a996 = 0u64;
                if nat_eq(a995, a996) {
                    Ok(true)
                } else {
                    let a1002 = {
                        let a999 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h997) => {
                                h997
                            }
                        };
                        let a1000 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h998, _) => {
                                h998
                            }
                        };
                        nat_add(a999, a1000)?
                    };
                    let a1003 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1001) => {
                            h1001
                        }
                    };
                    if nat_le(a1002, a1003) {
                        Ok(true)
                    } else {
                        let a1009 = {
                            let a1006 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1004) => {
                                    h1004
                                }
                            };
                            let a1007 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1005, _) => {
                                    h1005
                                }
                            };
                            nat_add(a1006, a1007)?
                        };
                        let a1010 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1008) => {
                                h1008
                            }
                        };
                        Ok(nat_le(a1009, a1010))
                    }
                }
            }
        }
    }
}

pub fn f26(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1016 = match v0.clone() {
            Adt2::C0(_, _, h1011, _, _, _, _, _, _, _, _) => {
                h1011
            }
        };
        let a1017 = {
            let a1014 = match v0.clone() {
                Adt2::C0(_, _, _, h1012, _, _, _, _, _, _, _) => {
                    h1012
                }
            };
            let a1015 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
                    h1013
                }
            };
            nat_mul(a1014, a1015)?
        };
        nat_mul(a1016, a1017)?
    };
    let v2: u64 = f20(v0.clone(), 1u64)?;
    let a1019 = match v0.clone() {
        Adt2::C0(h1018, _, _, _, _, _, _, _, _, _, _) => {
            h1018
        }
    };
    let a1020 = 0u64;
    if if if if if if if nat_eq(a1019, a1020) {
        let a1022 = match v0.clone() {
            Adt2::C0(_, h1021, _, _, _, _, _, _, _, _, _) => {
                h1021
            }
        };
        let a1023 = 2147483647u64;
        nat_le(a1022, a1023)
    } else {
        false
    } {
        let a1024 = v1;
        let a1025 = 2147483647u64;
        nat_le(a1024, a1025)
    } else {
        false
    } {
        let a1027 = match v0.clone() {
            Adt2::C0(_, h1026, _, _, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1028 = 1u64;
        if nat_eq(a1027, a1028) {
            true
        } else {
            let a1030 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1029, _, _, _, _, _) => {
                    h1029
                }
            };
            let a1031 = 4u64;
            nat_eq(a1030, a1031)
        }
    } else {
        false
    } {
        let a1032 = v1;
        let a1033 = 1u64;
        if nat_le(a1032, a1033) {
            true
        } else {
            let a1038 = {
                let a1035 = match v0.clone() {
                    Adt2::C0(_, h1034, _, _, _, _, _, _, _, _, _) => {
                        h1034
                    }
                };
                let a1036 = 4u64;
                nat_mul(a1035, a1036)?
            };
            let a1039 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1037, _, _, _, _) => {
                    h1037
                }
            };
            nat_le(a1038, a1039)
        }
    } else {
        false
    } {
        let a1041 = match v0.clone() {
            Adt2::C0(_, _, _, h1040, _, _, _, _, _, _, _) => {
                h1040
            }
        };
        let a1042 = 1u64;
        if nat_eq(a1041, a1042) {
            true
        } else {
            let a1048 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1043, _, _, _) => {
                    h1043
                }
            };
            let a1049 = {
                let a1046 = match v0.clone() {
                    Adt2::C0(_, _, h1044, _, _, _, _, _, _, _, _) => {
                        h1044
                    }
                };
                let a1047 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1045, _, _, _, _) => {
                        h1045
                    }
                };
                nat_mul(a1046, a1047)?
            };
            nat_eq(a1048, a1049)
        }
    } else {
        false
    } {
        let a1051 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1050, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1052 = 1u64;
        if nat_eq(a1051, a1052) {
            true
        } else {
            let a1061 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1053, _, _) => {
                    h1053
                }
            };
            let a1062 = {
                let a1059 = match v0.clone() {
                    Adt2::C0(_, _, _, h1054, _, _, _, _, _, _, _) => {
                        h1054
                    }
                };
                let a1060 = {
                    let a1057 = match v0.clone() {
                        Adt2::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                            h1055
                        }
                    };
                    let a1058 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1056, _, _, _, _) => {
                            h1056
                        }
                    };
                    nat_mul(a1057, a1058)?
                };
                nat_mul(a1059, a1060)?
            };
            nat_eq(a1061, a1062)
        }
    } else {
        false
    } {
        let a1063 = v2;
        let a1064 = 0u64;
        if nat_eq(a1063, a1064) {
            Ok(true)
        } else {
            let a1070 = {
                let a1067 = f84({
                    let a1065 = v2;
                    let a1066 = 1u64;
                    nat_sub(a1065, a1066)
                }, f24(v0.clone(), 1u64))?;
                let a1068 = 4u64;
                nat_add(a1067, a1068)?
            };
            let a1071 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1069, _) => {
                    h1069
                }
            };
            Ok(nat_le(a1070, a1071))
        }
    } else {
        Ok(false)
    }
}

pub fn f27(v0: u64) -> R<u64> {
    let a1078 = 148u64;
    let a1079 = {
        let a1076 = 36u64;
        let a1077 = {
            let a1074 = {
                let a1072 = 2u64;
                let a1073 = v0;
                nat_mul(a1072, a1073)?
            };
            let a1075 = 1u64;
            nat_sub(a1074, a1075)
        };
        nat_mul(a1076, a1077)?
    };
    nat_add(a1078, a1079)
}

pub fn f28(v0: Adt2, v1: Adt2) -> bool {
    let a1082 = match v0.clone() {
        Adt2::C0(_, h1080, _, _, _, _, _, _, _, _, _) => {
            h1080
        }
    };
    let a1083 = match v1.clone() {
        Adt2::C0(_, h1081, _, _, _, _, _, _, _, _, _) => {
            h1081
        }
    };
    if if if nat_eq(a1082, a1083) {
        let a1086 = match v0.clone() {
            Adt2::C0(_, _, h1084, _, _, _, _, _, _, _, _) => {
                h1084
            }
        };
        let a1087 = match v1.clone() {
            Adt2::C0(_, _, h1085, _, _, _, _, _, _, _, _) => {
                h1085
            }
        };
        nat_eq(a1086, a1087)
    } else {
        false
    } {
        let a1090 = match v0.clone() {
            Adt2::C0(_, _, _, h1088, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1091 = match v1.clone() {
            Adt2::C0(_, _, _, h1089, _, _, _, _, _, _, _) => {
                h1089
            }
        };
        nat_eq(a1090, a1091)
    } else {
        false
    } {
        let a1094 = match v0 {
            Adt2::C0(_, _, _, _, h1092, _, _, _, _, _, _) => {
                h1092
            }
        };
        let a1095 = match v1 {
            Adt2::C0(_, _, _, _, h1093, _, _, _, _, _, _) => {
                h1093
            }
        };
        nat_eq(a1094, a1095)
    } else {
        false
    }
}

pub fn f29(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1096 = v1;
    match m1096 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1099 = 1u64;
            let a1100 = {
                let a1097 = 2u64;
                let a1098 = v0;
                nat_mul(a1097, a1098)?
            };
            nat_add(a1099, a1100)
        }
    }
}

pub fn f30(v0: u64) -> bool {
    let a1101 = v0;
    let a1102 = 4294967296u64;
    if nat_lt(a1101, a1102) {
        let a1106 = {
            let a1103 = v0;
            let a1104 = 2147483648u64;
            let a1105 = 0u64;
            nat_rem(a1103, a1104, a1105)
        };
        let a1107 = 2139095040u64;
        nat_lt(a1106, a1107)
    } else {
        false
    }
}

pub fn f31(v0: u64) -> R<u64> {
    let a1108 = 4u64;
    let a1109 = v0;
    nat_mul(a1108, a1109)
}

pub fn f32(v0: u64) -> R<u64> {
    let a1110 = v0;
    let a1111 = 0u64;
    if nat_eq(a1110, a1111) {
        Ok(0u64)
    } else {
        let a1114 = 4u64;
        let a1115 = {
            let a1112 = 3u64;
            let a1113 = v0;
            nat_mul(a1112, a1113)?
        };
        nat_add(a1114, a1115)
    }
}

pub fn f33(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f24(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1116, _, _, _, _, _, _, _, _, _) => {
                h1116
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1117, _, _, _, _, _, _, _, _) => {
                h1117
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1118, _, _, _, _, _, _, _) => {
                h1118
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1119, _, _, _, _, _) => {
                h1119
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1120, _, _, _, _) => {
                h1120
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1121, _, _, _) => {
                h1121
            }
        })
    }
}

pub fn f34(v0: Adt2) -> R<bool> {
    let a1123 = match v0.clone() {
        Adt2::C0(_, h1122, _, _, _, _, _, _, _, _, _) => {
            h1122
        }
    };
    let a1124 = 1u64;
    if if if if nat_eq(a1123, a1124) {
        true
    } else {
        let a1126 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1125, _, _, _, _, _) => {
                h1125
            }
        };
        let a1127 = 4u64;
        nat_eq(a1126, a1127)
    } {
        let a1129 = match v0.clone() {
            Adt2::C0(_, _, h1128, _, _, _, _, _, _, _, _) => {
                h1128
            }
        };
        let a1130 = 1u64;
        if nat_eq(a1129, a1130) {
            true
        } else {
            let a1135 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1131, _, _, _, _) => {
                    h1131
                }
            };
            let a1136 = {
                let a1133 = match v0.clone() {
                    Adt2::C0(_, h1132, _, _, _, _, _, _, _, _, _) => {
                        h1132
                    }
                };
                let a1134 = 4u64;
                nat_mul(a1133, a1134)?
            };
            nat_eq(a1135, a1136)
        }
    } else {
        false
    } {
        let a1138 = match v0.clone() {
            Adt2::C0(_, _, _, h1137, _, _, _, _, _, _, _) => {
                h1137
            }
        };
        let a1139 = 1u64;
        if nat_eq(a1138, a1139) {
            true
        } else {
            let a1147 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1140, _, _, _) => {
                    h1140
                }
            };
            let a1148 = {
                let a1145 = {
                    let a1143 = match v0.clone() {
                        Adt2::C0(_, h1141, _, _, _, _, _, _, _, _, _) => {
                            h1141
                        }
                    };
                    let a1144 = match v0.clone() {
                        Adt2::C0(_, _, h1142, _, _, _, _, _, _, _, _) => {
                            h1142
                        }
                    };
                    nat_mul(a1143, a1144)?
                };
                let a1146 = 4u64;
                nat_mul(a1145, a1146)?
            };
            nat_eq(a1147, a1148)
        }
    } else {
        false
    } {
        let a1150 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1149, _, _, _, _, _, _) => {
                h1149
            }
        };
        let a1151 = 1u64;
        if nat_eq(a1150, a1151) {
            Ok(true)
        } else {
            let a1162 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1152, _, _) => {
                    h1152
                }
            };
            let a1163 = {
                let a1160 = {
                    let a1158 = {
                        let a1155 = match v0.clone() {
                            Adt2::C0(_, h1153, _, _, _, _, _, _, _, _, _) => {
                                h1153
                            }
                        };
                        let a1156 = match v0.clone() {
                            Adt2::C0(_, _, h1154, _, _, _, _, _, _, _, _) => {
                                h1154
                            }
                        };
                        nat_mul(a1155, a1156)?
                    };
                    let a1159 = match v0 {
                        Adt2::C0(_, _, _, h1157, _, _, _, _, _, _, _) => {
                            h1157
                        }
                    };
                    nat_mul(a1158, a1159)?
                };
                let a1161 = 4u64;
                nat_mul(a1160, a1161)?
            };
            Ok(nat_eq(a1162, a1163))
        }
    } else {
        Ok(false)
    }
}

pub fn f35(v0: Adt6, v1: u64) -> R<u64> {
    let a1165 = match v0.clone() {
        Adt6::C0(h1164, _, _, _) => {
            h1164
        }
    };
    let a1166 = 0u64;
    if nat_eq(a1165, a1166) {
        f85(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1167, _, _) => {
                h1167
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1168, _) => {
                h1168
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1169) => {
                h1169
            }
        }), {
            let a1171 = v1;
            let a1172 = match v0 {
                Adt6::C0(_, _, _, h1170) => {
                    h1170
                }
            };
            nat_add(a1171, a1172)?
        })
    } else {
        f85(v0, v1)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1177 = {
        let a1173 = v0;
        let a1174 = v1;
        nat_mul(a1173, a1174)?
    };
    let a1178 = {
        let a1175 = v2;
        let a1176 = f86(v3);
        nat_mul(a1175, a1176)?
    };
    nat_mul(a1177, a1178)
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1181 = v0;
    let a1182 = {
        let a1179 = f87(v1)?;
        let a1180 = v2;
        nat_mul(a1179, a1180)?
    };
    nat_add(a1181, a1182)
}

pub fn f38(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
            h1183
        }
    }, match v0.clone() {
        Adt2::C0(_, h1184, _, _, _, _, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1185, _, _, _, _, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1186, _, _, _, _, _, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1187, _, _, _, _, _, _) => {
            h1187
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1188, _, _, _, _, _) => {
            h1188
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1189, _, _, _, _) => {
            h1189
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1190, _, _, _) => {
            h1190
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1191, _, _) => {
            h1191
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1192, _) => {
            h1192
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1193) => {
            h1193
        }
    })
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1194 = v0;
    let a1195 = 23u64;
    if nat_eq(a1194, a1195) {
        f88(v1, v4, v6)
    } else {
        let a1196 = v0;
        let a1197 = 26u64;
        if nat_eq(a1196, a1197) {
            f89(v1, v2, v4, v6)
        } else {
            let a1198 = v0;
            let a1199 = 25u64;
            if nat_eq(a1198, a1199) {
                f90(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f40(v0: u64) -> u64 {
    let a1200 = v0;
    let a1201 = 30u64;
    if nat_eq(a1200, a1201) {
        2u64
    } else {
        4u64
    }
}

pub fn f41(v0: u64) -> bool {
    let a1202 = v0;
    let a1203 = 2139095040u64;
    if nat_le(a1202, a1203) {
        true
    } else {
        let a1204 = v0;
        let a1205 = 2147483648u64;
        nat_eq(a1204, a1205)
    }
}

pub fn f42(v0: Adt2, v1: u64) -> bool {
    let a1207 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1206, _, _, _, _, _) => {
            h1206
        }
    };
    let a1208 = v1;
    if if nat_eq(a1207, a1208) {
        let a1213 = {
            let a1210 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1209) => {
                    h1209
                }
            };
            let a1211 = v1;
            let a1212 = 0u64;
            nat_rem(a1210, a1211, a1212)
        };
        let a1214 = 0u64;
        nat_eq(a1213, a1214)
    } else {
        false
    } {
        f91(match v0.clone() {
            Adt2::C0(_, _, h1215, _, _, _, _, _, _, _, _) => {
                h1215
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1216, _, _, _, _, _, _, _) => {
                h1216
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1217, _, _, _, _, _, _) => {
                h1217
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1218, _, _, _, _) => {
                h1218
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1219, _, _, _) => {
                h1219
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1220, _, _) => {
                h1220
            }
        }, v1)
    } else {
        false
    }
}

pub fn f43(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f28(v0.clone(), v1.clone()) {
        let a1223 = match v0.clone() {
            Adt2::C0(h1221, _, _, _, _, _, _, _, _, _, _) => {
                h1221
            }
        };
        let a1224 = match v1.clone() {
            Adt2::C0(h1222, _, _, _, _, _, _, _, _, _, _) => {
                h1222
            }
        };
        nat_eq(a1223, a1224)
    } else {
        false
    } {
        let a1227 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1225) => {
                h1225
            }
        };
        let a1228 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1226) => {
                h1226
            }
        };
        nat_eq(a1227, a1228)
    } else {
        false
    } {
        let a1231 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1229, _, _, _, _, _) => {
                h1229
            }
        };
        let a1232 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1230, _, _, _, _, _) => {
                h1230
            }
        };
        nat_eq(a1231, a1232)
    } else {
        false
    } {
        let a1235 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1233, _, _, _, _) => {
                h1233
            }
        };
        let a1236 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1234, _, _, _, _) => {
                h1234
            }
        };
        nat_eq(a1235, a1236)
    } else {
        false
    } {
        let a1239 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1237, _, _, _) => {
                h1237
            }
        };
        let a1240 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1238, _, _, _) => {
                h1238
            }
        };
        nat_eq(a1239, a1240)
    } else {
        false
    } {
        let a1243 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1241, _, _) => {
                h1241
            }
        };
        let a1244 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1242, _, _) => {
                h1242
            }
        };
        nat_eq(a1243, a1244)
    } else {
        false
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1245 = v0;
    let a1246 = 0u64;
    if if if nat_eq(a1245, a1246) {
        true
    } else {
        let a1247 = v1;
        let a1248 = 0u64;
        nat_eq(a1247, a1248)
    } {
        true
    } else {
        let a1249 = v2;
        let a1250 = 0u64;
        nat_eq(a1249, a1250)
    } {
        true
    } else {
        let a1251 = v3;
        let a1252 = 0u64;
        nat_eq(a1251, a1252)
    }
}

pub fn f45(v0: Adt2, v1: u64) -> R<bool> {
    f92({
        let a1254 = match v0.clone() {
            Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                h1253
            }
        };
        let a1255 = v1;
        nat_mul(a1254, a1255)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1256, _, _, _, _, _, _, _, _) => {
            h1256
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1257, _, _, _, _, _, _, _) => {
            h1257
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1258, _, _, _, _, _, _) => {
            h1258
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1259, _, _, _, _) => {
            h1259
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1260, _, _, _) => {
            h1260
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1261, _, _) => {
            h1261
        }
    })
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1262 = v1;
    let a1263 = 0u64;
    if if nat_eq(a1262, a1263) {
        true
    } else {
        let a1264 = v2;
        let a1265 = 0u64;
        nat_eq(a1264, a1265)
    } {
        Ok(0u64)
    } else {
        let a1272 = {
            let a1266 = f93(v1, v2, v3)?;
            let a1267 = f94(v0, v1, v2)?;
            nat_add(a1266, a1267)?
        };
        let a1273 = {
            let a1270 = f95(v0);
            let a1271 = {
                let a1268 = v1;
                let a1269 = v2;
                nat_mul(a1268, a1269)?
            };
            nat_mul(a1270, a1271)?
        };
        nat_add(a1272, a1273)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1274 = v0;
    let a1275 = 0u64;
    if if nat_eq(a1274, a1275) {
        true
    } else {
        let a1276 = v1;
        let a1277 = 0u64;
        nat_eq(a1276, a1277)
    } {
        Ok(3u64)
    } else {
        let a1280 = {
            let a1278 = 3u64;
            let a1279 = v1;
            nat_add(a1278, a1279)?
        };
        let a1281 = v2;
        nat_add(a1280, a1281)
    }
}

pub fn f48(v0: u64) -> bool {
    let a1282 = v0;
    let a1283 = 2139095040u64;
    if nat_le(a1282, a1283) {
        true
    } else {
        let a1284 = v0;
        let a1285 = 2147483648u64;
        nat_eq(a1284, a1285)
    }
}

pub fn f49(v0: u64, v1: u64) -> R<u64> {
    let a1286 = v0;
    let a1287 = 0u64;
    if nat_eq(a1286, a1287) {
        Ok(0u64)
    } else {
        let a1288 = v1;
        let a1289 = f96(v0)?;
        nat_mul(a1288, a1289)
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1290 = v0;
    let a1291 = 0u64;
    if if if if nat_eq(a1290, a1291) {
        true
    } else {
        let a1292 = v1;
        let a1293 = 0u64;
        nat_eq(a1292, a1293)
    } {
        true
    } else {
        let a1294 = v2;
        let a1295 = 0u64;
        nat_eq(a1294, a1295)
    } {
        true
    } else {
        let a1296 = v3;
        let a1297 = 0u64;
        nat_eq(a1296, a1297)
    } {
        Ok(0u64)
    } else {
        let a1310 = {
            let a1300 = 2u64;
            let a1301 = {
                let a1298 = v1;
                let a1299 = v2;
                nat_mul(a1298, a1299)?
            };
            nat_mul(a1300, a1301)?
        };
        let a1311 = {
            let a1308 = {
                let a1304 = {
                    let a1302 = v1;
                    let a1303 = v2;
                    nat_mul(a1302, a1303)?
                };
                let a1305 = v3;
                nat_mul(a1304, a1305)?
            };
            let a1309 = {
                let a1306 = 1u64;
                let a1307 = v0;
                nat_add(a1306, a1307)?
            };
            nat_mul(a1308, a1309)?
        };
        nat_add(a1310, a1311)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1312 = v1;
        let a1313 = 0u64;
        if if if nat_eq(a1312, a1313) {
            true
        } else {
            let a1314 = v1;
            let a1315 = 10u64;
            nat_eq(a1314, a1315)
        } {
            true
        } else {
            let a1316 = v1;
            let a1317 = 15u64;
            nat_eq(a1316, a1317)
        } {
            true
        } else {
            let a1318 = v1;
            let a1319 = 20u64;
            nat_eq(a1318, a1319)
        }
    };
    let v4: bool = {
        let a1320 = v2;
        let a1321 = 0u64;
        if if if if if nat_eq(a1320, a1321) {
            true
        } else {
            let a1322 = v2;
            let a1323 = 10u64;
            nat_eq(a1322, a1323)
        } {
            true
        } else {
            let a1324 = v2;
            let a1325 = 15u64;
            nat_eq(a1324, a1325)
        } {
            true
        } else {
            let a1326 = v2;
            let a1327 = 20u64;
            nat_eq(a1326, a1327)
        } {
            true
        } else {
            let a1328 = v2;
            let a1329 = 30u64;
            nat_eq(a1328, a1329)
        } {
            true
        } else {
            let a1330 = v2;
            let a1331 = 40u64;
            nat_eq(a1330, a1331)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1332 = v0;
            let a1333 = 30u64;
            if if nat_eq(a1332, a1333) {
                let a1334 = v2;
                let a1335 = 10u64;
                nat_eq(a1334, a1335)
            } else {
                false
            } {
                true
            } else {
                if f97(v0) {
                    let a1336 = v2;
                    let a1337 = 10u64;
                    if if nat_eq(a1336, a1337) {
                        true
                    } else {
                        let a1338 = v2;
                        let a1339 = 15u64;
                        nat_eq(a1338, a1339)
                    } {
                        true
                    } else {
                        let a1340 = v2;
                        let a1341 = 20u64;
                        nat_eq(a1340, a1341)
                    }
                } else {
                    false
                }
            }
        })
    } else {
        None::<bool>
    }
}

pub fn f52(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1342, _, _, _, _, _, _, _, _, _, _, _) => {
            h1342
        }
    };
    Ok(Adt9::C0({
        let a1343 = v0;
        let a1344 = 86u64;
        if nat_eq(a1343, a1344) {
            let m1345 = v3;
            match m1345 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f98(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1346 = v0;
            let a1347 = 55u64;
            if nat_eq(a1346, a1347) {
                let m1348 = v3;
                match m1348 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f99(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1349 = v0;
                let a1350 = 22u64;
                if nat_eq(a1349, a1350) {
                    let m1351 = v3;
                    match m1351 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f100(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1352 = v0;
                    let a1353 = 44u64;
                    if nat_eq(a1352, a1353) {
                        f101(v1, v2, v3, v4)?
                    } else {
                        let a1354 = v0;
                        let a1355 = 52u64;
                        if nat_eq(a1354, a1355) {
                            f102(v1, v2, v4)?
                        } else {
                            let a1356 = v0;
                            let a1357 = 73u64;
                            if nat_eq(a1356, a1357) {
                                f103(v2, v3, v4)?
                            } else {
                                let a1358 = v0;
                                let a1359 = 57u64;
                                if if nat_eq(a1358, a1359) {
                                    true
                                } else {
                                    let a1360 = v0;
                                    let a1361 = 58u64;
                                    nat_eq(a1360, a1361)
                                } {
                                    let m1362 = v3;
                                    match m1362 {
                                        None => {
                                            f104({
                                                let a1363 = v0;
                                                let a1364 = 58u64;
                                                nat_eq(a1363, a1364)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a1365 = v0;
                                    let a1366 = 49u64;
                                    if nat_eq(a1365, a1366) {
                                        f105(v1, v2, v3, v4)?
                                    } else {
                                        let a1367 = v0;
                                        let a1368 = 51u64;
                                        if nat_eq(a1367, a1368) {
                                            f106(v1, v2, v3, v4)?
                                        } else {
                                            let a1369 = v0;
                                            let a1370 = 39u64;
                                            if nat_eq(a1369, a1370) {
                                                f107(v2, v3, v4)?
                                            } else {
                                                let a1371 = v0;
                                                let a1372 = 77u64;
                                                if nat_eq(a1371, a1372) {
                                                    f108(v2, v4)?
                                                } else {
                                                    let a1373 = v0;
                                                    let a1374 = 40u64;
                                                    if nat_eq(a1373, a1374) {
                                                        f109(v2, v3, v4)?
                                                    } else {
                                                        if f110(v0) {
                                                            let a1375 = v0;
                                                            let a1376 = 33u64;
                                                            if if if nat_eq(a1375, a1376) {
                                                                let a1378 = match v2.clone() {
                                                                    Adt2::C0(h1377, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1377
                                                                    }
                                                                };
                                                                let a1379 = 0u64;
                                                                nat_eq(a1378, a1379)
                                                            } else {
                                                                false
                                                            } {
                                                                let a1381 = match v4.clone() {
                                                                    Adt2::C0(h1380, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1380
                                                                    }
                                                                };
                                                                let a1382 = 26u64;
                                                                nat_eq(a1381, a1382)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1383 = v3;
                                                                match m1383 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f111(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a1384 = v0;
                                                                let a1385 = 33u64;
                                                                if if if nat_eq(a1384, a1385) {
                                                                    let a1387 = match v4.clone() {
                                                                        Adt2::C0(h1386, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1386
                                                                        }
                                                                    };
                                                                    let a1388 = 0u64;
                                                                    nat_eq(a1387, a1388)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f112(match v2.clone() {
                                                                        Adt2::C0(h1389, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1389
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f113(v2, v3, v4)?
                                                                } else {
                                                                    f114(v0, v2, {
                                                                        let m1390 = v3;
                                                                        match m1390 {
                                                                            None => {
                                                                                false
                                                                            }
                                                                            Some(_) => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }, v4)?
                                                                }
                                                            }
                                                        } else {
                                                            let a1391 = v0;
                                                            let a1392 = 19u64;
                                                            if nat_eq(a1391, a1392) {
                                                                f115(v2, v4)?
                                                            } else {
                                                                let a1393 = v0;
                                                                let a1394 = 21u64;
                                                                if nat_eq(a1393, a1394) {
                                                                    f116(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a1395 = v0;
                                                                    let a1396 = 61u64;
                                                                    if if if nat_eq(a1395, a1396) {
                                                                        true
                                                                    } else {
                                                                        let a1397 = v0;
                                                                        let a1398 = 62u64;
                                                                        nat_eq(a1397, a1398)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a1399 = v0;
                                                                        let a1400 = 63u64;
                                                                        nat_eq(a1399, a1400)
                                                                    } {
                                                                        f117(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a1401 = v0;
                                                                        let a1402 = 4u64;
                                                                        if if nat_eq(a1401, a1402) {
                                                                            true
                                                                        } else {
                                                                            let a1403 = v0;
                                                                            let a1404 = 32u64;
                                                                            nat_eq(a1403, a1404)
                                                                        } {
                                                                            f118(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a1405 = v0;
                                                                            let a1406 = 20u64;
                                                                            if nat_eq(a1405, a1406) {
                                                                                f119(v2, v4)?
                                                                            } else {
                                                                                let a1407 = v0;
                                                                                let a1408 = 42u64;
                                                                                if nat_eq(a1407, a1408) {
                                                                                    f120(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a1409 = v0;
                                                                                    let a1410 = 69u64;
                                                                                    if nat_eq(a1409, a1410) {
                                                                                        let a1411 = v5;
                                                                                        let a1412 = 4u64;
                                                                                        if nat_lt(a1411, a1412) {
                                                                                            f120(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a1413 = v0;
                                                                                        let a1414 = 70u64;
                                                                                        if nat_eq(a1413, a1414) {
                                                                                            f121(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a1415 = v0;
                                                                                            let a1416 = 75u64;
                                                                                            if if nat_eq(a1415, a1416) {
                                                                                                true
                                                                                            } else {
                                                                                                let a1417 = v0;
                                                                                                let a1418 = 76u64;
                                                                                                nat_eq(a1417, a1418)
                                                                                            } {
                                                                                                f122({
                                                                                                    let a1419 = v0;
                                                                                                    let a1420 = 76u64;
                                                                                                    nat_eq(a1419, a1420)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a1421 = v0;
                                                                                                let a1422 = 31u64;
                                                                                                if if if nat_eq(a1421, a1422) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1423 = v0;
                                                                                                    let a1424 = 47u64;
                                                                                                    nat_eq(a1423, a1424)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1425 = v0;
                                                                                                    let a1426 = 68u64;
                                                                                                    nat_eq(a1425, a1426)
                                                                                                } {
                                                                                                    f123(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1427 = v0;
                                                                                                    let a1428 = 15u64;
                                                                                                    if nat_eq(a1427, a1428) {
                                                                                                        f124(v2, v4)?
                                                                                                    } else {
                                                                                                        let a1429 = v0;
                                                                                                        let a1430 = 17u64;
                                                                                                        if nat_eq(a1429, a1430) {
                                                                                                            f125(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1431 = v0;
                                                                                                            let a1432 = 18u64;
                                                                                                            if nat_eq(a1431, a1432) {
                                                                                                                f126(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a1433 = v0;
                                                                                                                let a1434 = 13u64;
                                                                                                                if if if nat_eq(a1433, a1434) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1435 = v0;
                                                                                                                    let a1436 = 14u64;
                                                                                                                    nat_eq(a1435, a1436)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1437 = v0;
                                                                                                                    let a1438 = 16u64;
                                                                                                                    nat_eq(a1437, a1438)
                                                                                                                } {
                                                                                                                    f127(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m1439 = f128(v0, v5)?;
                                                                                                                    match m1439 {
                                                                                                                        None => {
                                                                                                                            f129(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f130(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f53(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f131(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1440, _, _, _) => {
            h1440
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1441, _, _) => {
            h1441
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1442, _) => {
            h1442
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1443) => {
            h1443
        }
    })
}

pub fn f54(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1445 = v0;
        let a1446 = match v1.clone() {
            Adt3::C0(h1444, _, _, _, _, _, _, _) => {
                h1444
            }
        };
        let a1447 = 0u64;
        nat_quot(a1445, a1446, a1447)
    };
    let v4: u64 = {
        let a1449 = v3;
        let a1450 = match v1.clone() {
            Adt3::C0(_, h1448, _, _, _, _, _, _) => {
                h1448
            }
        };
        let a1451 = 0u64;
        nat_quot(a1449, a1450, a1451)
    };
    let a1452 = v2;
    let a1453 = 0u64;
    if nat_eq(a1452, a1453) {
        let a1455 = v0;
        let a1456 = match v1 {
            Adt3::C0(h1454, _, _, _, _, _, _, _) => {
                h1454
            }
        };
        let a1457 = 0u64;
        nat_rem(a1455, a1456, a1457)
    } else {
        let a1458 = v2;
        let a1459 = 1u64;
        if nat_eq(a1458, a1459) {
            let a1461 = v3;
            let a1462 = match v1 {
                Adt3::C0(_, h1460, _, _, _, _, _, _) => {
                    h1460
                }
            };
            let a1463 = 0u64;
            nat_rem(a1461, a1462, a1463)
        } else {
            let a1464 = v2;
            let a1465 = 2u64;
            if nat_eq(a1464, a1465) {
                let a1467 = v4;
                let a1468 = match v1 {
                    Adt3::C0(_, _, h1466, _, _, _, _, _) => {
                        h1466
                    }
                };
                let a1469 = 0u64;
                nat_rem(a1467, a1468, a1469)
            } else {
                let a1471 = v4;
                let a1472 = match v1 {
                    Adt3::C0(_, _, h1470, _, _, _, _, _) => {
                        h1470
                    }
                };
                let a1473 = 0u64;
                nat_quot(a1471, a1472, a1473)
            }
        }
    }
}

pub fn f55(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1475 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1474) => {
            h1474
        }
    };
    let a1476 = 0u64;
    if nat_eq(a1475, a1476) {
        Ok(v2)
    } else {
        let a1481 = {
            let a1478 = v2;
            let a1479 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1477) => {
                    h1477
                }
            };
            nat_mul(a1478, a1479)?
        };
        let a1482 = f54(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1480, _, _, _, _, _, _) => {
                h1480
            }
        }, 3u64);
        nat_add(a1481, a1482)
    }
}

pub fn f56(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1492 = {
            let a1489 = {
                let a1484 = v2;
                let a1485 = match v0.clone() {
                    Adt6::C0(_, h1483, _, _) => {
                        h1483
                    }
                };
                nat_mul(a1484, a1485)?
            };
            let a1490 = {
                let a1487 = v3;
                let a1488 = match v0.clone() {
                    Adt6::C0(_, _, _, h1486) => {
                        h1486
                    }
                };
                nat_mul(a1487, a1488)?
            };
            nat_add(a1489, a1490)?
        };
        let a1493 = f132(match v0.clone() {
            Adt6::C0(_, _, h1491, _) => {
                h1491
            }
        });
        nat_add(a1492, a1493)?
    };
    let v5: u64 = f133(match v0 {
        Adt6::C0(_, _, h1494, _) => {
            h1494
        }
    });
    let a1495 = v5;
    let a1496 = v4;
    if if nat_le(a1495, a1496) {
        let a1499 = {
            let a1497 = v4;
            let a1498 = v5;
            nat_sub(a1497, a1498)
        };
        let a1500 = v1;
        nat_lt(a1499, a1500)
    } else {
        false
    } {
        Ok(Some({
            let a1501 = v4;
            let a1502 = v5;
            nat_sub(a1501, a1502)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f57(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1503, _, _, _, _, _) => {
            h1503
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1504, _, _, _, _, _, _) => {
            h1504
        }
    };
    let v7: u64 = f54(v1, v6.clone(), 2u64);
    let a1506 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1505) => {
            h1505
        }
    };
    let a1507 = 0u64;
    if nat_eq(a1506, a1507) {
        Ok(Adt9::C8(Some(f53(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1510 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1509, _, _, _, _) => {
                    h1509
                }
            };
            match m1510 {
                Adt3::C0(_, _, h1508, _, _, _, _, _) => {
                    h1508
                }
            }
        };
        let m1516 = f56(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1511, _) => {
                h1511
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1512, _, _, _, _, _) => {
                h1512
            }
        }, {
            let a1513 = v2;
            let a1514 = v8;
            let a1515 = 0u64;
            nat_rem(a1513, a1514, a1515)
        }, v7)?;
        match m1516 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1523 = {
                        let a1521 = {
                            let a1517 = v2;
                            let a1518 = v8;
                            let a1519 = 0u64;
                            nat_quot(a1517, a1518, a1519)
                        };
                        let a1522 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1520) => {
                                h1520
                            }
                        };
                        nat_mul(a1521, a1522)?
                    };
                    let a1524 = f54(v1, v6, 3u64);
                    nat_add(a1523, a1524)?
                };
                Ok(Adt9::C8(Some(f53(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f58(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1525 = v1;
    match m1525 {
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
        Adt9::C6(v38, v39, v40, v41, v42) => {
            Adt9::C6(v38, v39, v40, v41, v42)
        }
        Adt9::C7(v43, v44, v45, v46, v47, v48) => {
            Adt9::C7(v43, v44, v45, v46, v47, v48)
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

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1526 = v3;
    match m1526 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f84(v0, v1)?, f84(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f140(v0, v1, v2.clone())?, f84(v0, v2)?))
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
            Ok(Adt9::C1(true, 0u64, f84(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f147(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v48, v49) => {
            f148(v0, v1, v2, v48, v49)
        }
        Adt4::C12(v50, v51, v52) => {
            f149(v0, v1, v2, v50, v51, v52)
        }
        Adt4::C13(_, v54, _, _) => {
            Ok(Adt9::C3(v54, v54, f84(v0, v1)?, f84(v0, v2)?))
        }
        Adt4::C14(v57, v58, v59) => {
            let a1529 = v57;
            let a1530 = 23u64;
            if nat_eq(a1529, a1530) {
                f150(v0, v1, v2, v58, v59)
            } else {
                Ok(Adt9::C3(v58, v59, f84(v0, v1)?, f84(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f151(v0, v1, v2)
        }
        Adt4::C16(v60, v61) => {
            f152(v0, v1, v2, v60, v61)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v62, v63) => {
            f153(v0, v1, v63, v62)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f84(f139(v0, match v2.clone() {
                Adt3::C0(h1527, _, _, _, _, _, _, _) => {
                    h1527
                }
            }, match v2.clone() {
                Adt3::C0(_, h1528, _, _, _, _, _, _) => {
                    h1528
                }
            })?, v1)?, f84(v0, v2)?))
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
        Adt4::C25(v68, v69, v70) => {
            f154(v0, v68, v69, v70)
        }
        Adt4::C26(v76, _, _, _) => {
            let a1531 = v76;
            let a1532 = 64u64;
            if if if nat_eq(a1531, a1532) {
                true
            } else {
                let a1533 = v76;
                let a1534 = 65u64;
                nat_eq(a1533, a1534)
            } {
                true
            } else {
                let a1535 = v76;
                let a1536 = 66u64;
                nat_eq(a1535, a1536)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v75) => {
            f76(v0, v1, v2, v75)
        }
        Adt4::C28(v71) => {
            f73(v0, v1, v2, v71, 0u64)
        }
        Adt4::C29(v72) => {
            f74(v0, v1, v2, v72)
        }
        Adt4::C30(v73, _) => {
            f75(v0, v1, v2, v73)
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
        Adt4::C35(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C37(_, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C38(_, _, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C39(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f155(v3, v4);
    let v6: u64 = {
        let a1537 = v0;
        let a1538 = v5;
        let a1539 = 0u64;
        nat_quot(a1537, a1538, a1539)
    };
    let a1540 = v5;
    let a1541 = 1u64;
    if nat_eq(a1540, a1541) {
        Ok(f134(f156(v3, v4), v3, v4, 0u64, f84(v6, v1)?, 2u64, 0u64, 3u64, f84(v6, v2)?))
    } else {
        let a1545 = {
            let a1542 = v0;
            let a1543 = 2u64;
            let a1544 = 0u64;
            nat_rem(a1542, a1543, a1544)
        };
        let a1546 = 0u64;
        if nat_eq(a1545, a1546) {
            Ok(f134(f156(v3, 0u64), v3, 0u64, 0u64, f84(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f134(f156(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f84(v6, v2)?))
        }
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f157(v1)?;
    let v4: u64 = {
        let a1547 = v0;
        let a1548 = v3;
        let a1549 = 0u64;
        nat_rem(a1547, a1548, a1549)
    };
    let v5: u64 = {
        let a1552 = {
            let a1550 = v1;
            let a1551 = 3u64;
            nat_mul(a1550, a1551)?
        };
        let a1553 = 1u64;
        nat_add(a1552, a1553)?
    };
    let a1554 = v4;
    let a1555 = 0u64;
    if nat_eq(a1554, a1555) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1556 = v4;
        let a1557 = v5;
        if nat_eq(a1556, a1557) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1562 = v4;
                let a1563 = {
                    let a1558 = v4;
                    let a1559 = v5;
                    if nat_lt(a1558, a1559) {
                        1u64
                    } else {
                        let a1560 = v5;
                        let a1561 = 1u64;
                        nat_add(a1560, a1561)?
                    }
                };
                nat_sub(a1562, a1563)
            };
            let v7: u64 = {
                let a1564 = v6;
                let a1565 = 3u64;
                let a1566 = 0u64;
                nat_rem(a1564, a1565, a1566)
            };
            let v8: u64 = {
                let a1577 = 4u64;
                let a1578 = {
                    let a1575 = {
                        let a1570 = {
                            let a1567 = v0;
                            let a1568 = v3;
                            let a1569 = 0u64;
                            nat_quot(a1567, a1568, a1569)
                        };
                        let a1571 = v1;
                        nat_mul(a1570, a1571)?
                    };
                    let a1576 = {
                        let a1572 = v6;
                        let a1573 = 3u64;
                        let a1574 = 0u64;
                        nat_quot(a1572, a1573, a1574)
                    };
                    nat_add(a1575, a1576)?
                };
                nat_mul(a1577, a1578)?
            };
            let a1579 = v4;
            let a1580 = v5;
            if nat_lt(a1579, a1580) {
                let a1581 = v7;
                let a1582 = 0u64;
                if nat_eq(a1581, a1582) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1583 = v7;
                    let a1584 = 1u64;
                    if nat_eq(a1583, a1584) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1585 = v7;
                let a1586 = 0u64;
                if nat_eq(a1585, a1586) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1587 = v7;
                    let a1588 = 1u64;
                    if nat_eq(a1587, a1588) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1589 = v0;
        let a1590 = 6u64;
        let a1591 = 0u64;
        nat_quot(a1589, a1590, a1591)
    };
    let v5: u64 = {
        let a1592 = v0;
        let a1593 = 6u64;
        let a1594 = 0u64;
        nat_rem(a1592, a1593, a1594)
    };
    let a1595 = v5;
    let a1596 = 0u64;
    if nat_eq(a1595, a1596) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f84(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1597 = v5;
        let a1598 = 1u64;
        if nat_eq(a1597, a1598) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1599 = v5;
            let a1600 = 2u64;
            if nat_eq(a1599, a1600) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f84(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1601 = v5;
                let a1602 = 3u64;
                if nat_eq(a1601, a1602) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1603 = v5;
                    let a1604 = 4u64;
                    if nat_eq(a1603, a1604) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f84(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f84(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f158(match v1.clone() {
        Adt3::C0(_, h1605, _, _, _, _, _, _) => {
            h1605
        }
    }, v4)?;
    let v8: u64 = {
        let a1606 = v0;
        let a1607 = v7;
        let a1608 = 0u64;
        nat_rem(a1606, a1607, a1608)
    };
    let v9: u64 = {
        let a1609 = v0;
        let a1610 = v7;
        let a1611 = 0u64;
        nat_quot(a1609, a1610, a1611)
    };
    let a1614 = {
        let a1612 = v8;
        let a1613 = 1u64;
        nat_add(a1612, a1613)?
    };
    let a1615 = v7;
    if nat_eq(a1614, a1615) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f84(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1616 = v8;
            let a1617 = v6;
            let a1618 = 0u64;
            nat_quot(a1616, a1617, a1618)
        };
        let v11: u64 = {
            let a1619 = v8;
            let a1620 = v6;
            let a1621 = 0u64;
            nat_rem(a1619, a1620, a1621)
        };
        let v12: u64 = {
            let a1623 = v9;
            let a1624 = match v1.clone() {
                Adt3::C0(h1622, _, _, _, _, _, _, _) => {
                    h1622
                }
            };
            let a1625 = 0u64;
            nat_rem(a1623, a1624, a1625)
        };
        let v13: u64 = {
            let a1627 = v9;
            let a1628 = match v1.clone() {
                Adt3::C0(h1626, _, _, _, _, _, _, _) => {
                    h1626
                }
            };
            let a1629 = 0u64;
            nat_quot(a1627, a1628, a1629)
        };
        let v14: u64 = if v4 {
            let a1641 = {
                let a1636 = {
                    let a1631 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1630, _, _, _) => {
                            h1630
                        }
                    };
                    let a1632 = v12;
                    nat_mul(a1631, a1632)?
                };
                let a1637 = {
                    let a1634 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1633, _, _) => {
                            h1633
                        }
                    };
                    let a1635 = v10;
                    nat_mul(a1634, a1635)?
                };
                nat_add(a1636, a1637)?
            };
            let a1642 = {
                let a1639 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1638, _) => {
                        h1638
                    }
                };
                let a1640 = v13;
                nat_mul(a1639, a1640)?
            };
            nat_add(a1641, a1642)?
        } else {
            let a1649 = {
                let a1644 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1643, _, _, _) => {
                        h1643
                    }
                };
                let a1645 = v10;
                nat_mul(a1644, a1645)?
            };
            let a1650 = {
                let a1647 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1646, _, _) => {
                        h1646
                    }
                };
                let a1648 = v13;
                nat_mul(a1647, a1648)?
            };
            nat_add(a1649, a1650)?
        };
        if if v4 {
            let a1651 = v11;
            let a1652 = 0u64;
            nat_eq(a1651, a1652)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1655 = {
                let a1653 = v11;
                let a1654 = 1u64;
                nat_add(a1653, a1654)?
            };
            let a1656 = v6;
            if nat_eq(a1655, a1656) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1657 = v10;
                    let a1658 = 0u64;
                    if nat_eq(a1657, a1658) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1670 = {
                        let a1665 = {
                            let a1660 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1659, _, _, _) => {
                                    h1659
                                }
                            };
                            let a1661 = v12;
                            nat_mul(a1660, a1661)?
                        };
                        let a1666 = {
                            let a1663 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1662, _, _) => {
                                    h1662
                                }
                            };
                            let a1664 = v10;
                            nat_mul(a1663, a1664)?
                        };
                        nat_add(a1665, a1666)?
                    };
                    let a1671 = {
                        let a1668 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1667, _) => {
                                h1667
                            }
                        };
                        let a1669 = v13;
                        nat_mul(a1668, a1669)?
                    };
                    nat_add(a1670, a1671)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f29(match v2.clone() {
        Adt3::C0(_, h1672, _, _, _, _, _, _) => {
            h1672
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1673 = v0;
        let a1674 = v6;
        let a1675 = 0u64;
        nat_rem(a1673, a1674, a1675)
    };
    let v8: u64 = {
        let a1676 = v0;
        let a1677 = v6;
        let a1678 = 0u64;
        nat_quot(a1676, a1677, a1678)
    };
    let v9: u64 = {
        let a1680 = v8;
        let a1681 = match v5.clone() {
            Adt3::C0(h1679, _, _, _, _, _, _, _) => {
                h1679
            }
        };
        let a1682 = 0u64;
        nat_rem(a1680, a1681, a1682)
    };
    let v10: u64 = {
        let a1684 = v8;
        let a1685 = match v5.clone() {
            Adt3::C0(h1683, _, _, _, _, _, _, _) => {
                h1683
            }
        };
        let a1686 = 0u64;
        nat_quot(a1684, a1685, a1686)
    };
    let v11: u64 = {
        let a1688 = v10;
        let a1689 = match v2.clone() {
            Adt3::C0(_, h1687, _, _, _, _, _, _) => {
                h1687
            }
        };
        let a1690 = 0u64;
        nat_rem(a1688, a1689, a1690)
    };
    let v12: u64 = {
        let a1692 = v10;
        let a1693 = match v2.clone() {
            Adt3::C0(_, h1691, _, _, _, _, _, _) => {
                h1691
            }
        };
        let a1694 = 0u64;
        nat_quot(a1692, a1693, a1694)
    };
    let a1695 = v7;
    let a1696 = 0u64;
    if nat_eq(a1695, a1696) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1703 = {
                let a1698 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1697, _, _, _) => {
                        h1697
                    }
                };
                let a1699 = v9;
                nat_mul(a1698, a1699)?
            };
            let a1704 = {
                let a1701 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1700, _, _) => {
                        h1700
                    }
                };
                let a1702 = v12;
                nat_mul(a1701, a1702)?
            };
            nat_add(a1703, a1704)?
        }), Adt12::C0(2u64, {
            let a1711 = {
                let a1706 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1705, _, _, _) => {
                        h1705
                    }
                };
                let a1707 = v11;
                nat_mul(a1706, a1707)?
            };
            let a1712 = {
                let a1709 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1708, _, _) => {
                        h1708
                    }
                };
                let a1710 = v12;
                nat_mul(a1709, a1710)?
            };
            nat_add(a1711, a1712)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1713 = v4;
        match m1713 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1725 = {
                        let a1720 = {
                            let a1715 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1714, _, _, _) => {
                                    h1714
                                }
                            };
                            let a1716 = v9;
                            nat_mul(a1715, a1716)?
                        };
                        let a1721 = {
                            let a1718 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1717, _, _) => {
                                    h1717
                                }
                            };
                            let a1719 = v11;
                            nat_mul(a1718, a1719)?
                        };
                        nat_add(a1720, a1721)?
                    };
                    let a1726 = {
                        let a1723 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1722, _) => {
                                h1722
                            }
                        };
                        let a1724 = v12;
                        nat_mul(a1723, a1724)?
                    };
                    nat_add(a1725, a1726)?
                }), Adt13::C1(f84(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1727 = v7;
                    let a1728 = 1u64;
                    nat_sub(a1727, a1728)
                };
                let v15: u64 = {
                    let a1729 = v14;
                    let a1730 = 2u64;
                    let a1731 = 0u64;
                    nat_quot(a1729, a1730, a1731)
                };
                let a1735 = {
                    let a1732 = v14;
                    let a1733 = 2u64;
                    let a1734 = 0u64;
                    nat_rem(a1732, a1733, a1734)
                };
                let a1736 = 0u64;
                if nat_eq(a1735, a1736) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1748 = {
                            let a1743 = {
                                let a1738 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1737, _, _, _) => {
                                        h1737
                                    }
                                };
                                let a1739 = v9;
                                nat_mul(a1738, a1739)?
                            };
                            let a1744 = {
                                let a1741 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1740, _, _) => {
                                        h1740
                                    }
                                };
                                let a1742 = v15;
                                nat_mul(a1741, a1742)?
                            };
                            nat_add(a1743, a1744)?
                        };
                        let a1749 = {
                            let a1746 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1745, _) => {
                                    h1745
                                }
                            };
                            let a1747 = v12;
                            nat_mul(a1746, a1747)?
                        };
                        nat_add(a1748, a1749)?
                    }), Adt12::C0(3u64, {
                        let a1761 = {
                            let a1756 = {
                                let a1751 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1750, _, _, _) => {
                                        h1750
                                    }
                                };
                                let a1752 = v11;
                                nat_mul(a1751, a1752)?
                            };
                            let a1757 = {
                                let a1754 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1753, _, _) => {
                                        h1753
                                    }
                                };
                                let a1755 = v15;
                                nat_mul(a1754, a1755)?
                            };
                            nat_add(a1756, a1757)?
                        };
                        let a1762 = {
                            let a1759 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1758, _) => {
                                    h1758
                                }
                            };
                            let a1760 = v12;
                            nat_mul(a1759, a1760)?
                        };
                        nat_add(a1761, a1762)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1765 = {
                            let a1763 = v7;
                            let a1764 = 1u64;
                            nat_add(a1763, a1764)?
                        };
                        let a1766 = v6;
                        if nat_eq(a1765, a1766) {
                            Adt13::C1(f84(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f27(v6)?;
    let v9: u64 = {
        let a1767 = v0;
        let a1768 = v8;
        let a1769 = 0u64;
        nat_rem(a1767, a1768, a1769)
    };
    let v10: u64 = {
        let a1770 = v0;
        let a1771 = v8;
        let a1772 = 0u64;
        nat_quot(a1770, a1771, a1772)
    };
    let a1773 = v9;
    let a1774 = 148u64;
    if nat_lt(a1773, a1774) {
        f159({
            let a1775 = v9;
            let a1776 = 37u64;
            let a1777 = 0u64;
            nat_rem(a1775, a1776, a1777)
        }, {
            let a1778 = v9;
            let a1779 = 37u64;
            let a1780 = 0u64;
            nat_quot(a1778, a1779, a1780)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f160({
            let a1781 = v9;
            let a1782 = 148u64;
            nat_sub(a1781, a1782)
        }, v10, v4, v5)
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f22(v2);
    let a1783 = v0;
    let a1784 = v3;
    if nat_lt(a1783, a1784) {
        let a1785 = v0;
        let a1786 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1785, a1786) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1787 = 4u64;
                let a1788 = v0;
                nat_mul(a1787, a1788)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1789 = v0;
            let a1790 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1789, a1790) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1793 = v0;
                let a1794 = {
                    let a1791 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1792 = 1u64;
                    nat_add(a1791, a1792)?
                };
                if nat_eq(a1793, a1794) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1795 = v0;
                    let a1796 = 9u64;
                    if nat_eq(a1795, a1796) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1797 = v0;
            let a1798 = v3;
            nat_sub(a1797, a1798)
        };
        let v5: u64 = f23(v2);
        let v6: u64 = {
            let a1799 = v4;
            let a1800 = v5;
            let a1801 = 0u64;
            nat_rem(a1799, a1800, a1801)
        };
        let v7: u64 = f84({
            let a1802 = v4;
            let a1803 = v5;
            let a1804 = 0u64;
            nat_quot(a1802, a1803, a1804)
        }, v1)?;
        if v2 {
            let a1805 = v6;
            let a1806 = 7u64;
            if nat_lt(a1805, a1806) {
                let a1807 = v6;
                let a1808 = 3u64;
                if nat_lt(a1807, a1808) {
                    let a1809 = v6;
                    let a1810 = 1u64;
                    if nat_lt(a1809, a1810) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1811 = v6;
                        let a1812 = 2u64;
                        if nat_lt(a1811, a1812) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1813 = v6;
                    let a1814 = 5u64;
                    if nat_lt(a1813, a1814) {
                        let a1815 = v6;
                        let a1816 = 4u64;
                        if nat_lt(a1815, a1816) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1817 = v6;
                        let a1818 = 6u64;
                        if nat_lt(a1817, a1818) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1819 = v6;
                let a1820 = 11u64;
                if nat_lt(a1819, a1820) {
                    let a1821 = v6;
                    let a1822 = 9u64;
                    if nat_lt(a1821, a1822) {
                        let a1823 = v6;
                        let a1824 = 8u64;
                        if nat_lt(a1823, a1824) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1825 = v6;
                        let a1826 = 10u64;
                        if nat_lt(a1825, a1826) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1827 = v6;
                    let a1828 = 13u64;
                    if nat_lt(a1827, a1828) {
                        let a1829 = v6;
                        let a1830 = 12u64;
                        if nat_lt(a1829, a1830) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1831 = v6;
                        let a1832 = 14u64;
                        if nat_lt(a1831, a1832) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1833 = v6;
            let a1834 = 1u64;
            if nat_lt(a1833, a1834) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1835 = v6;
                let a1836 = 2u64;
                if nat_lt(a1835, a1836) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f67(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a1837 = v0;
    let a1838 = v1;
    if nat_lt(a1837, a1838) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f84(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a1839 = v0;
            let a1840 = v1;
            nat_sub(a1839, a1840)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f84(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a1847 = {
            let a1845 = 3u64;
            let a1846 = {
                let a1843 = match v4.clone() {
                    Adt6::C0(h1841, _, _, _) => {
                        h1841
                    }
                };
                let a1844 = match v5.clone() {
                    Adt6::C0(h1842, _, _, _) => {
                        h1842
                    }
                };
                nat_mul(a1843, a1844)?
            };
            nat_mul(a1845, a1846)?
        };
        let a1848 = 2u64;
        nat_add(a1847, a1848)?
    };
    let v8: u64 = {
        let a1849 = v0;
        let a1850 = v7;
        let a1851 = 0u64;
        nat_quot(a1849, a1850, a1851)
    };
    let v9: u64 = {
        let a1852 = v0;
        let a1853 = v7;
        let a1854 = 0u64;
        nat_rem(a1852, a1853, a1854)
    };
    let a1855 = v9;
    let a1856 = 0u64;
    if nat_eq(a1855, a1856) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1859 = {
            let a1857 = v9;
            let a1858 = 1u64;
            nat_add(a1857, a1858)?
        };
        let a1860 = v7;
        if nat_eq(a1859, a1860) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f84(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a1861 = v9;
                let a1862 = 1u64;
                nat_sub(a1861, a1862)
            };
            let v11: u64 = {
                let a1863 = v10;
                let a1864 = 3u64;
                let a1865 = 0u64;
                nat_quot(a1863, a1864, a1865)
            };
            let v12: u64 = {
                let a1867 = v11;
                let a1868 = match v4.clone() {
                    Adt6::C0(h1866, _, _, _) => {
                        h1866
                    }
                };
                let a1869 = 0u64;
                nat_rem(a1867, a1868, a1869)
            };
            let v13: u64 = {
                let a1871 = v11;
                let a1872 = match v4.clone() {
                    Adt6::C0(h1870, _, _, _) => {
                        h1870
                    }
                };
                let a1873 = 0u64;
                nat_quot(a1871, a1872, a1873)
            };
            let v14: u64 = {
                let a1874 = v10;
                let a1875 = 3u64;
                let a1876 = 0u64;
                nat_rem(a1874, a1875, a1876)
            };
            let v15: u64 = f54(v8, v3.clone(), 2u64);
            let m1878 = f56(v4, match v2.clone() {
                Adt3::C0(h1877, _, _, _, _, _, _, _) => {
                    h1877
                }
            }, f54(v8, v3.clone(), 0u64), v12)?;
            match m1878 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m1880 = f56(v5, match v2.clone() {
                        Adt3::C0(_, h1879, _, _, _, _, _, _) => {
                            h1879
                        }
                    }, f54(v8, v3.clone(), 1u64), v13)?;
                    match m1880 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a1881 = v14;
                            let a1882 = 0u64;
                            if nat_eq(a1881, a1882) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f53(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f161(v6), 4u64))
                            } else {
                                let a1883 = v14;
                                let a1884 = 1u64;
                                if nat_eq(a1883, a1884) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f53(v2, v16, v17, v15, f54(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f162(match v1.clone() {
        Adt3::C0(h1885, _, _, _, _, _, _, _) => {
            h1885
        }
    })?;
    let v5: u64 = {
        let a1886 = v0;
        let a1887 = v4;
        let a1888 = 0u64;
        nat_quot(a1886, a1887, a1888)
    };
    let v6: u64 = {
        let a1889 = v0;
        let a1890 = v4;
        let a1891 = 0u64;
        nat_rem(a1889, a1890, a1891)
    };
    let a1892 = v6;
    let a1893 = 0u64;
    if nat_eq(a1892, a1893) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1894 = v6;
        let a1895 = 1u64;
        if nat_eq(a1894, a1895) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1898 = {
                let a1896 = v6;
                let a1897 = 1u64;
                nat_add(a1896, a1897)?
            };
            let a1899 = v4;
            if nat_eq(a1898, a1899) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f84(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1900 = v6;
                    let a1901 = 2u64;
                    nat_sub(a1900, a1901)
                };
                let v8: u64 = {
                    let a1902 = v7;
                    let a1903 = 3u64;
                    let a1904 = 0u64;
                    nat_quot(a1902, a1903, a1904)
                };
                let v9: u64 = {
                    let a1905 = v7;
                    let a1906 = 3u64;
                    let a1907 = 0u64;
                    nat_rem(a1905, a1906, a1907)
                };
                let v10: u64 = f84({
                    let a1911 = {
                        let a1909 = v5;
                        let a1910 = match v1.clone() {
                            Adt3::C0(h1908, _, _, _, _, _, _, _) => {
                                h1908
                            }
                        };
                        nat_mul(a1909, a1910)?
                    };
                    let a1912 = v8;
                    nat_add(a1911, a1912)?
                }, v1)?;
                let a1913 = v9;
                let a1914 = 0u64;
                if nat_eq(a1913, a1914) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1915 = v9;
                    let a1916 = 1u64;
                    if nat_eq(a1915, a1916) {
                        let a1917 = v3;
                        let a1918 = 2u64;
                        if nat_eq(a1917, a1918) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1919 = v3;
                        let a1920 = 2u64;
                        if if nat_eq(a1919, a1920) {
                            true
                        } else {
                            let a1921 = v3;
                            let a1922 = 3u64;
                            nat_eq(a1921, a1922)
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

pub fn f70(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1924 = 2u64;
        let a1925 = match v2.clone() {
            Adt3::C0(_, h1923, _, _, _, _, _, _) => {
                h1923
            }
        };
        nat_mul(a1924, a1925)?
    };
    let a1926 = v1;
    let a1927 = v6;
    if nat_lt(a1926, a1927) {
        let a1931 = {
            let a1928 = v1;
            let a1929 = 2u64;
            let a1930 = 0u64;
            nat_rem(a1928, a1929, a1930)
        };
        let a1932 = 0u64;
        if nat_eq(a1931, a1932) {
            Ok(Adt9::C4(19u64, 4u64, f84({
                let a1933 = v1;
                let a1934 = 2u64;
                let a1935 = 0u64;
                nat_quot(a1933, a1934, a1935)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1936 = v5;
            let a1937 = 2147483648u64;
            if if nat_lt(a1936, a1937) {
                let a1939 = v5;
                let a1940 = match v3 {
                    Adt3::C0(_, h1938, _, _, _, _, _, _) => {
                        h1938
                    }
                };
                nat_lt(a1939, a1940)
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
            let a1943 = match v3.clone() {
                Adt3::C0(h1941, _, _, _, _, _, _, _) => {
                    h1941
                }
            };
            let a1944 = match v3.clone() {
                Adt3::C0(_, h1942, _, _, _, _, _, _) => {
                    h1942
                }
            };
            nat_mul(a1943, a1944)?
        };
        let a1947 = {
            let a1945 = v1;
            let a1946 = v6;
            nat_sub(a1945, a1946)
        };
        let a1948 = v7;
        if nat_lt(a1947, a1948) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f84({
                let a1949 = v1;
                let a1950 = v6;
                nat_sub(a1949, a1950)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a1953 = {
                    let a1951 = v1;
                    let a1952 = v6;
                    nat_sub(a1951, a1952)
                };
                let a1954 = v7;
                nat_sub(a1953, a1954)
            };
            let v9: u64 = {
                let a1956 = 1u64;
                let a1957 = f163(v0, match v2.clone() {
                    Adt3::C0(h1955, _, _, _, _, _, _, _) => {
                        h1955
                    }
                })?;
                nat_add(a1956, a1957)?
            };
            let v10: u64 = {
                let a1958 = v8;
                let a1959 = v9;
                let a1960 = 0u64;
                nat_quot(a1958, a1959, a1960)
            };
            let v11: u64 = {
                let a1961 = v8;
                let a1962 = v9;
                let a1963 = 0u64;
                nat_rem(a1961, a1962, a1963)
            };
            let a1964 = v11;
            let a1965 = 0u64;
            if nat_eq(a1964, a1965) {
                Ok(Adt9::C4(19u64, 4u64, f84(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f164(v0) {
                    let a1969 = {
                        let a1966 = v11;
                        let a1967 = 2u64;
                        let a1968 = 0u64;
                        nat_rem(a1966, a1967, a1968)
                    };
                    let a1970 = 1u64;
                    if nat_eq(a1969, a1970) {
                        Ok(Adt9::C4(6u64, 0u64, f84({
                            let a1979 = {
                                let a1972 = v10;
                                let a1973 = match v2.clone() {
                                    Adt3::C0(h1971, _, _, _, _, _, _, _) => {
                                        h1971
                                    }
                                };
                                nat_mul(a1972, a1973)?
                            };
                            let a1980 = {
                                let a1976 = {
                                    let a1974 = v11;
                                    let a1975 = 1u64;
                                    nat_sub(a1974, a1975)
                                };
                                let a1977 = 2u64;
                                let a1978 = 0u64;
                                nat_quot(a1976, a1977, a1978)
                            };
                            nat_add(a1979, a1980)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f84({
                            let a1989 = {
                                let a1982 = v5;
                                let a1983 = match v2.clone() {
                                    Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                                        h1981
                                    }
                                };
                                nat_mul(a1982, a1983)?
                            };
                            let a1990 = {
                                let a1986 = {
                                    let a1984 = v11;
                                    let a1985 = 1u64;
                                    nat_sub(a1984, a1985)
                                };
                                let a1987 = 2u64;
                                let a1988 = 0u64;
                                nat_quot(a1986, a1987, a1988)
                            };
                            nat_add(a1989, a1990)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f84({
                            let a1999 = {
                                let a1992 = v5;
                                let a1993 = match v2 {
                                    Adt3::C0(h1991, _, _, _, _, _, _, _) => {
                                        h1991
                                    }
                                };
                                nat_mul(a1992, a1993)?
                            };
                            let a2000 = {
                                let a1996 = {
                                    let a1994 = v11;
                                    let a1995 = 1u64;
                                    nat_sub(a1994, a1995)
                                };
                                let a1997 = 2u64;
                                let a1998 = 0u64;
                                nat_quot(a1996, a1997, a1998)
                            };
                            nat_add(a1999, a2000)?
                        }, v3)?))
                    }
                } else {
                    f165(v0, v10, {
                        let a2001 = v11;
                        let a2002 = 1u64;
                        nat_sub(a2001, a2002)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2003 = v1;
    let a2004 = f166(v2.clone(), v4.clone())?;
    if nat_lt(a2003, a2004) {
        let a2007 = v1;
        let a2008 = {
            let a2005 = 2u64;
            let a2006 = f167(v4.clone())?;
            nat_mul(a2005, a2006)?
        };
        if nat_lt(a2007, a2008) {
            let a2012 = {
                let a2009 = v1;
                let a2010 = 2u64;
                let a2011 = 0u64;
                nat_rem(a2009, a2010, a2011)
            };
            let a2013 = 0u64;
            if nat_eq(a2012, a2013) {
                f168({
                    let a2014 = v1;
                    let a2015 = 2u64;
                    let a2016 = 0u64;
                    nat_quot(a2014, a2015, a2016)
                }, v4)
            } else {
                if f169(match v2 {
                    Adt3::C0(_, h2017, _, _, _, _, _, _) => {
                        h2017
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2020 = v1;
                let a2021 = {
                    let a2018 = 2u64;
                    let a2019 = f167(v4.clone())?;
                    nat_mul(a2018, a2019)?
                };
                nat_sub(a2020, a2021)
            };
            let v7: u64 = {
                let a2025 = v6;
                let a2026 = {
                    let a2023 = 1u64;
                    let a2024 = match v2.clone() {
                        Adt3::C0(h2022, _, _, _, _, _, _, _) => {
                            h2022
                        }
                    };
                    nat_add(a2023, a2024)?
                };
                let a2027 = 0u64;
                nat_quot(a2025, a2026, a2027)
            };
            let v8: u64 = {
                let a2031 = v6;
                let a2032 = {
                    let a2029 = 1u64;
                    let a2030 = match v2.clone() {
                        Adt3::C0(h2028, _, _, _, _, _, _, _) => {
                            h2028
                        }
                    };
                    nat_add(a2029, a2030)?
                };
                let a2033 = 0u64;
                nat_rem(a2031, a2032, a2033)
            };
            let a2034 = v8;
            let a2035 = 0u64;
            if nat_eq(a2034, a2035) {
                f168(v7, v4)
            } else {
                f170(v0, v7, {
                    let a2036 = v8;
                    let a2037 = 1u64;
                    nat_sub(a2036, a2037)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2040 = match v4.clone() {
            Adt3::C0(h2038, _, _, _, _, _, _, _) => {
                h2038
            }
        };
        let a2041 = match v4.clone() {
            Adt3::C0(_, h2039, _, _, _, _, _, _) => {
                h2039
            }
        };
        nat_mul(a2040, a2041)?
    };
    let v7: u64 = {
        let a2042 = 2u64;
        let a2043 = v6;
        nat_mul(a2042, a2043)?
    };
    let v8: u64 = f171(match v3.clone() {
        Adt3::C0(_, h2044, _, _, _, _, _, _) => {
            h2044
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2045, _, _, _, _, _) => {
            h2045
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2046, _, _, _, _) => {
            h2046
        }
    });
    let a2047 = v0;
    let a2048 = v7;
    if nat_lt(a2047, a2048) {
        let a2052 = {
            let a2049 = v0;
            let a2050 = 2u64;
            let a2051 = 0u64;
            nat_rem(a2049, a2050, a2051)
        };
        let a2053 = 0u64;
        if nat_eq(a2052, a2053) {
            Ok(Adt9::C4(19u64, 5u64, f84({
                let a2054 = v0;
                let a2055 = 2u64;
                let a2056 = 0u64;
                nat_quot(a2054, a2055, a2056)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f172(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2057 = v0;
            let a2058 = v7;
            nat_sub(a2057, a2058)
        };
        let v10: u64 = {
            let a2062 = v9;
            let a2063 = {
                let a2060 = 1u64;
                let a2061 = match v1.clone() {
                    Adt3::C0(h2059, _, _, _, _, _, _, _) => {
                        h2059
                    }
                };
                nat_add(a2060, a2061)?
            };
            let a2064 = 0u64;
            nat_quot(a2062, a2063, a2064)
        };
        let v11: u64 = {
            let a2068 = v9;
            let a2069 = {
                let a2066 = 1u64;
                let a2067 = match v1.clone() {
                    Adt3::C0(h2065, _, _, _, _, _, _, _) => {
                        h2065
                    }
                };
                nat_add(a2066, a2067)?
            };
            let a2070 = 0u64;
            nat_rem(a2068, a2069, a2070)
        };
        let a2071 = v11;
        let a2072 = 0u64;
        if nat_eq(a2071, a2072) {
            Ok(Adt9::C4(19u64, 5u64, f84({
                let a2073 = v10;
                let a2074 = v6;
                let a2075 = 0u64;
                nat_rem(a2073, a2074, a2075)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f172(v5, v8) {
                let v12: u64 = {
                    let a2081 = {
                        let a2077 = v10;
                        let a2078 = match v1.clone() {
                            Adt3::C0(h2076, _, _, _, _, _, _, _) => {
                                h2076
                            }
                        };
                        nat_mul(a2077, a2078)?
                    };
                    let a2082 = {
                        let a2079 = v11;
                        let a2080 = 1u64;
                        nat_sub(a2079, a2080)
                    };
                    nat_add(a2081, a2082)?
                };
                let a2083 = v5;
                let a2084 = 4294967295u64;
                if nat_eq(a2083, a2084) {
                    Ok(Adt9::C1(false, f84(v12, v1)?, f84(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f84(v12, v1)?, 4u64, f84({
                        let a2090 = {
                            let a2086 = v5;
                            let a2087 = match v3.clone() {
                                Adt3::C0(h2085, _, _, _, _, _, _, _) => {
                                    h2085
                                }
                            };
                            nat_mul(a2086, a2087)?
                        };
                        let a2091 = {
                            let a2088 = v11;
                            let a2089 = 1u64;
                            nat_sub(a2088, a2089)
                        };
                        nat_add(a2090, a2091)?
                    }, v3)?, 3u64, f84(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f173(v3.clone())?;
    let v6: u64 = {
        let a2092 = v0;
        let a2093 = v5;
        let a2094 = 0u64;
        nat_quot(a2092, a2093, a2094)
    };
    let v7: u64 = {
        let a2095 = v0;
        let a2096 = v5;
        let a2097 = 0u64;
        nat_rem(a2095, a2096, a2097)
    };
    let v8: u64 = f174(v3.clone())?;
    let a2098 = v7;
    let a2099 = 0u64;
    if nat_eq(a2098, a2099) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2100, _) => {
                h2100
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2104 = v5;
        let a2105 = {
            let a2102 = v7;
            let a2103 = if match v3.clone() {
                Adt7::C0(_, _, _, h2101, _) => {
                    h2101
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2102, a2103)?
        };
        if nat_le(a2104, a2105) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2106, _) => {
                    h2106
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f84(v6, v2)?))
            } else {
                let a2109 = {
                    let a2107 = v7;
                    let a2108 = 2u64;
                    nat_add(a2107, a2108)?
                };
                let a2110 = v5;
                if nat_eq(a2109, a2110) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2111, _, _) => {
                            h2111
                        }
                    } {
                        let a2118 = {
                            let m2114 = match v3.clone() {
                                Adt7::C0(h2113, _, _, _, _) => {
                                    h2113
                                }
                            };
                            match m2114 {
                                Adt6::C0(h2112, _, _, _) => {
                                    h2112
                                }
                            }
                        };
                        let a2119 = {
                            let m2117 = match v3 {
                                Adt7::C0(_, h2116, _, _, _) => {
                                    h2116
                                }
                            };
                            match m2117 {
                                Adt6::C0(h2115, _, _, _) => {
                                    h2115
                                }
                            }
                        };
                        nat_mul(a2118, a2119)?
                    } else {
                        f175(match v3 {
                            Adt7::C0(h2120, _, _, _, _) => {
                                h2120
                            }
                        }, match v1 {
                            Adt3::C0(h2121, _, _, _, _, _, _, _) => {
                                h2121
                            }
                        }, f54(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2122 = v9;
                        let a2123 = 0u64;
                        if nat_eq(a2122, a2123) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f84(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2126 = {
                    let a2124 = v7;
                    let a2125 = 1u64;
                    nat_sub(a2124, a2125)
                };
                let a2127 = v8;
                let a2128 = 0u64;
                nat_rem(a2126, a2127, a2128)
            };
            let m2134 = f176(v3.clone(), v1, v2, v6, {
                let a2131 = {
                    let a2129 = v7;
                    let a2130 = 1u64;
                    nat_sub(a2129, a2130)
                };
                let a2132 = v8;
                let a2133 = 0u64;
                nat_quot(a2131, a2132, a2133)
            })?;
            match m2134 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2136 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2135) => {
                            h2135
                        }
                    };
                    let a2137 = 1u64;
                    if if nat_eq(a2136, a2137) {
                        let a2138 = v10;
                        let a2139 = 0u64;
                        nat_eq(a2138, a2139)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2140, _) => {
                                h2140
                            }
                        } {
                            let a2143 = {
                                let a2141 = v10;
                                let a2142 = 1u64;
                                nat_add(a2141, a2142)?
                            };
                            let a2144 = v8;
                            if nat_eq(a2143, a2144) {
                                let a2145 = v4;
                                let a2146 = 2u64;
                                if nat_eq(a2145, a2146) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2148 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2147) => {
                                                h2147
                                            }
                                        };
                                        let a2149 = 1u64;
                                        if nat_eq(a2148, a2149) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2151 = match v3 {
                                            Adt7::C0(_, _, _, _, h2150) => {
                                                h2150
                                            }
                                        };
                                        let a2152 = 1u64;
                                        if nat_eq(a2151, a2152) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2154 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2153) => {
                                            h2153
                                        }
                                    };
                                    let a2155 = 1u64;
                                    if nat_eq(a2154, a2155) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2157 = match v3 {
                                        Adt7::C0(_, _, _, _, h2156) => {
                                            h2156
                                        }
                                    };
                                    let a2158 = 1u64;
                                    if nat_eq(a2157, a2158) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2160 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2159) => {
                                        h2159
                                    }
                                };
                                let a2161 = 1u64;
                                if nat_eq(a2160, a2161) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2163 = match v3 {
                                    Adt7::C0(_, _, _, _, h2162) => {
                                        h2162
                                    }
                                };
                                let a2164 = 1u64;
                                if nat_eq(a2163, a2164) {
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

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f177(match v3.clone() {
        Adt3::C0(h2165, _, _, _, _, _, _, _) => {
            h2165
        }
    })?;
    let v5: u64 = {
        let a2166 = v0;
        let a2167 = v4;
        let a2168 = 0u64;
        nat_rem(a2166, a2167, a2168)
    };
    let v6: u64 = {
        let a2169 = v0;
        let a2170 = v4;
        let a2171 = 0u64;
        nat_quot(a2169, a2170, a2171)
    };
    let a2172 = v5;
    let a2173 = 0u64;
    if nat_eq(a2172, a2173) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2176 = {
            let a2174 = v5;
            let a2175 = 1u64;
            nat_add(a2174, a2175)?
        };
        let a2177 = v4;
        if nat_eq(a2176, a2177) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f84(v6, v2)?))
        } else {
            let a2181 = {
                let a2178 = v5;
                let a2179 = 2u64;
                let a2180 = 0u64;
                nat_rem(a2178, a2179, a2180)
            };
            let a2182 = 0u64;
            if nat_eq(a2181, a2182) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2185 = {
                        let a2183 = v5;
                        let a2184 = 1u64;
                        nat_sub(a2183, a2184)
                    };
                    let a2186 = 2u64;
                    let a2187 = 0u64;
                    nat_quot(a2185, a2186, a2187)
                };
                let v8: u64 = f54(v6, v2.clone(), 0u64);
                let v9: u64 = f54(v6, v2.clone(), 1u64);
                let v10: u64 = f54(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f53(v1, {
                    let a2188 = v9;
                    let a2189 = v7;
                    nat_add(a2188, a2189)?
                }, v8, v10, 0u64)?, 4u64, f53(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f178(v3.clone())?;
    let v5: u64 = {
        let a2190 = v0;
        let a2191 = v4;
        let a2192 = 0u64;
        nat_rem(a2190, a2191, a2192)
    };
    let v6: u64 = {
        let a2193 = v0;
        let a2194 = v4;
        let a2195 = 0u64;
        nat_quot(a2193, a2194, a2195)
    };
    let a2196 = v5;
    let a2197 = 0u64;
    if nat_eq(a2196, a2197) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2200 = {
            let a2198 = v5;
            let a2199 = 1u64;
            nat_add(a2198, a2199)?
        };
        let a2201 = v4;
        if nat_eq(a2200, a2201) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f84(v6, v2)?))
        } else {
            let a2205 = {
                let a2202 = v5;
                let a2203 = 2u64;
                let a2204 = 0u64;
                nat_rem(a2202, a2203, a2204)
            };
            let a2206 = 0u64;
            if nat_eq(a2205, a2206) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2208 = f54(v6, v2.clone(), 0u64);
                    let a2209 = match v3.clone() {
                        Adt6::C0(_, _, h2207, _) => {
                            h2207
                        }
                    };
                    nat_add(a2208, a2209)?
                };
                let v8: u64 = {
                    let a2217 = {
                        let a2214 = {
                            let a2211 = v7;
                            let a2212 = match v3.clone() {
                                Adt6::C0(_, h2210, _, _) => {
                                    h2210
                                }
                            };
                            nat_add(a2211, a2212)?
                        };
                        let a2215 = match v3.clone() {
                            Adt6::C0(h2213, _, _, _) => {
                                h2213
                            }
                        };
                        nat_sub(a2214, a2215)
                    };
                    let a2218 = match v3.clone() {
                        Adt6::C0(_, h2216, _, _) => {
                            h2216
                        }
                    };
                    let a2219 = 0u64;
                    nat_quot(a2217, a2218, a2219)
                };
                let v9: u64 = {
                    let a2225 = v8;
                    let a2226 = {
                        let a2222 = {
                            let a2220 = v5;
                            let a2221 = 1u64;
                            nat_sub(a2220, a2221)
                        };
                        let a2223 = 2u64;
                        let a2224 = 0u64;
                        nat_quot(a2222, a2223, a2224)
                    };
                    nat_add(a2225, a2226)?
                };
                let v10: u64 = {
                    let a2228 = v9;
                    let a2229 = match v3.clone() {
                        Adt6::C0(_, h2227, _, _) => {
                            h2227
                        }
                    };
                    nat_mul(a2228, a2229)?
                };
                let v11: u64 = {
                    let a2230 = v7;
                    let a2231 = v10;
                    nat_sub(a2230, a2231)
                };
                let a2233 = v9;
                let a2234 = match v1.clone() {
                    Adt3::C0(_, h2232, _, _, _, _, _, _) => {
                        h2232
                    }
                };
                if if if nat_lt(a2233, a2234) {
                    let a2235 = v10;
                    let a2236 = v7;
                    nat_le(a2235, a2236)
                } else {
                    false
                } {
                    let a2238 = v11;
                    let a2239 = match v3.clone() {
                        Adt6::C0(h2237, _, _, _) => {
                            h2237
                        }
                    };
                    nat_lt(a2238, a2239)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f53(v1, {
                        let a2243 = {
                            let a2241 = f54(v6, v2, 1u64);
                            let a2242 = match v3 {
                                Adt6::C0(h2240, _, _, _) => {
                                    h2240
                                }
                            };
                            nat_mul(a2241, a2242)?
                        };
                        let a2244 = v11;
                        nat_add(a2243, a2244)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f54(v0, v2.clone(), 0u64);
    let v5: u64 = f54(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2246 = v5;
        let a2247 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2245, _) => {
                h2245
            }
        };
        let a2248 = 0u64;
        nat_rem(a2246, a2247, a2248)
    };
    let v7: u64 = {
        let a2250 = v5;
        let a2251 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2249, _) => {
                h2249
            }
        };
        let a2252 = 0u64;
        nat_quot(a2250, a2251, a2252)
    };
    let v8: Option<u64> = f56(match v3.clone() {
        Adt5::C0(h2253, _, _, _, _, _, _, _) => {
            h2253
        }
    }, match v1.clone() {
        Adt3::C0(h2254, _, _, _, _, _, _, _) => {
            h2254
        }
    }, f54(v0, v2.clone(), 1u64), {
        let a2258 = v4;
        let a2259 = {
            let m2257 = match v3.clone() {
                Adt5::C0(h2256, _, _, _, _, _, _, _) => {
                    h2256
                }
            };
            match m2257 {
                Adt6::C0(h2255, _, _, _) => {
                    h2255
                }
            }
        };
        let a2260 = 0u64;
        nat_rem(a2258, a2259, a2260)
    })?;
    let v9: u64 = {
        let a2264 = v4;
        let a2265 = {
            let m2263 = match v3.clone() {
                Adt5::C0(h2262, _, _, _, _, _, _, _) => {
                    h2262
                }
            };
            match m2263 {
                Adt6::C0(h2261, _, _, _) => {
                    h2261
                }
            }
        };
        let a2266 = 0u64;
        nat_quot(a2264, a2265, a2266)
    };
    let v10: Option<u64> = f56(match v3.clone() {
        Adt5::C0(_, h2267, _, _, _, _, _, _) => {
            h2267
        }
    }, match v1.clone() {
        Adt3::C0(_, h2268, _, _, _, _, _, _) => {
            h2268
        }
    }, f54(v0, v2.clone(), 2u64), {
        let a2272 = v9;
        let a2273 = {
            let m2271 = match v3.clone() {
                Adt5::C0(_, h2270, _, _, _, _, _, _) => {
                    h2270
                }
            };
            match m2271 {
                Adt6::C0(h2269, _, _, _) => {
                    h2269
                }
            }
        };
        let a2274 = 0u64;
        nat_rem(a2272, a2273, a2274)
    })?;
    let v11: u64 = {
        let a2278 = v9;
        let a2279 = {
            let m2277 = match v3.clone() {
                Adt5::C0(_, h2276, _, _, _, _, _, _) => {
                    h2276
                }
            };
            match m2277 {
                Adt6::C0(h2275, _, _, _) => {
                    h2275
                }
            }
        };
        let a2280 = 0u64;
        nat_quot(a2278, a2279, a2280)
    };
    let v12: Option<u64> = f56(match v3.clone() {
        Adt5::C0(_, _, h2281, _, _, _, _, _) => {
            h2281
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2282, _, _, _, _, _) => {
            h2282
        }
    }, v6, {
        let a2286 = v11;
        let a2287 = {
            let m2285 = match v3.clone() {
                Adt5::C0(_, _, h2284, _, _, _, _, _) => {
                    h2284
                }
            };
            match m2285 {
                Adt6::C0(h2283, _, _, _) => {
                    h2283
                }
            }
        };
        let a2288 = 0u64;
        nat_rem(a2286, a2287, a2288)
    })?;
    let v13: u64 = {
        let a2292 = v11;
        let a2293 = {
            let m2291 = match v3.clone() {
                Adt5::C0(_, _, h2290, _, _, _, _, _) => {
                    h2290
                }
            };
            match m2291 {
                Adt6::C0(h2289, _, _, _) => {
                    h2289
                }
            }
        };
        let a2294 = 0u64;
        nat_quot(a2292, a2293, a2294)
    };
    let v17: Adt12 = {
        let m2295 = v8;
        match m2295 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2296 = v10;
                match m2296 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2297 = v12;
                        match m2297 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2306 = {
                                        let a2301 = f53(v1, v14, v15, v16, 0u64)?;
                                        let a2302 = {
                                            let a2299 = v7;
                                            let a2300 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2298, _, _, _) => {
                                                    h2298
                                                }
                                            };
                                            nat_mul(a2299, a2300)?
                                        };
                                        nat_add(a2301, a2302)?
                                    };
                                    let a2307 = {
                                        let a2304 = v13;
                                        let a2305 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2303, _, _, _, _) => {
                                                h2303
                                            }
                                        };
                                        nat_mul(a2304, a2305)?
                                    };
                                    nat_add(a2306, a2307)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2308, _, _) => {
            h2308
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2309) => {
            h2309
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f84(v0, v2)?), {
        let a2311 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2310, _, _) => {
                h2310
            }
        };
        let a2312 = 0u64;
        if nat_eq(a2311, a2312) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2314 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2313) => {
                h2313
            }
        };
        let a2315 = 0u64;
        if nat_eq(a2314, a2315) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f77(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2328 = f179(v0, v1, match v2.clone() {
        Adt3::C0(h2316, _, _, _, _, _, _, _) => {
            h2316
        }
    }, match v2.clone() {
        Adt3::C0(_, h2317, _, _, _, _, _, _) => {
            h2317
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2318, _, _, _, _, _) => {
            h2318
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2319, _, _, _, _) => {
            h2319
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2320, _, _, _) => {
            h2320
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2321, _, _) => {
            h2321
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2322, _) => {
            h2322
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2323) => {
            h2323
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2324, _, _, _) => {
            h2324
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2325, _, _) => {
            h2325
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2326, _) => {
            h2326
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2327) => {
            h2327
        }
    }, v4)?;
    match m2328 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f18(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2331 = v6;
        let a2332 = {
            let a2329 = 3u64;
            let a2330 = v5;
            nat_add(a2329, a2330)?
        };
        nat_sub(a2331, a2332)
    };
    let a2334 = v0;
    let a2335 = f46(v3, match v1.clone() {
        Adt3::C0(h2333, _, _, _, _, _, _, _) => {
            h2333
        }
    }, v5, v8)?;
    if nat_lt(a2334, a2335) {
        let v9: u64 = f93(match v1.clone() {
            Adt3::C0(h2336, _, _, _, _, _, _, _) => {
                h2336
            }
        }, v5, v8)?;
        let v10: u64 = f94(v3, match v1.clone() {
            Adt3::C0(h2337, _, _, _, _, _, _, _) => {
                h2337
            }
        }, v5)?;
        let a2338 = v0;
        let a2339 = v9;
        if nat_lt(a2338, a2339) {
            f180(v0, v2, f40(v3), {
                let a2340 = 3u64;
                let a2341 = v5;
                nat_add(a2340, a2341)?
            }, v7)
        } else {
            let a2344 = v0;
            let a2345 = {
                let a2342 = v9;
                let a2343 = v10;
                nat_add(a2342, a2343)?
            };
            if nat_lt(a2344, a2345) {
                let v11: u64 = {
                    let a2346 = v0;
                    let a2347 = v9;
                    nat_sub(a2346, a2347)
                };
                let v12: u64 = f181(v3, match v1.clone() {
                    Adt3::C0(h2348, _, _, _, _, _, _, _) => {
                        h2348
                    }
                })?;
                f182({
                    let a2349 = v11;
                    let a2350 = v12;
                    let a2351 = 0u64;
                    nat_rem(a2349, a2350, a2351)
                }, {
                    let a2352 = v11;
                    let a2353 = v12;
                    let a2354 = 0u64;
                    nat_quot(a2352, a2353, a2354)
                }, v1, v3, v4, v7)
            } else {
                f183({
                    let a2357 = v0;
                    let a2358 = {
                        let a2355 = v9;
                        let a2356 = v10;
                        nat_add(a2355, a2356)?
                    };
                    nat_sub(a2357, a2358)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2359 = v0;
    let a2360 = f32(v1)?;
    if nat_lt(a2359, a2360) {
        let a2361 = v0;
        let a2362 = 4u64;
        if nat_lt(a2361, a2362) {
            let a2363 = v0;
            let a2364 = 0u64;
            if nat_eq(a2363, a2364) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2365 = v0;
                let a2366 = 1u64;
                if nat_eq(a2365, a2366) {
                    let a2367 = v5;
                    let a2368 = 0u64;
                    if nat_eq(a2367, a2368) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2369 = v0;
                    let a2370 = 2u64;
                    if nat_eq(a2369, a2370) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2371 = v0;
            let a2372 = 4u64;
            if if nat_eq(a2371, a2372) {
                let a2373 = f184(v5)?;
                let a2374 = v1;
                !nat_eq(a2373, a2374)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2377 = {
                        let a2375 = v0;
                        let a2376 = 4u64;
                        nat_sub(a2375, a2376)
                    };
                    let a2378 = 3u64;
                    let a2379 = 0u64;
                    nat_quot(a2377, a2378, a2379)
                };
                let v7: u64 = {
                    let a2382 = {
                        let a2380 = v0;
                        let a2381 = 4u64;
                        nat_sub(a2380, a2381)
                    };
                    let a2383 = 3u64;
                    let a2384 = 0u64;
                    nat_rem(a2382, a2383, a2384)
                };
                let a2385 = v7;
                let a2386 = 0u64;
                if nat_eq(a2385, a2386) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2387 = v7;
                    let a2388 = 1u64;
                    if nat_eq(a2387, a2388) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f31(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f80(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2389 = v1;
        let a2390 = 5u64;
        nat_eq(a2389, a2390)
    } else {
        false
    } {
        let a2391 = v2;
        let a2392 = 16u64;
        nat_eq(a2391, a2392)
    } else {
        false
    } {
        let a2393 = v3;
        let a2394 = 0u64;
        nat_eq(a2393, a2394)
    } else {
        false
    } {
        let a2395 = v4;
        let a2396 = 0u64;
        nat_eq(a2395, a2396)
    } else {
        false
    }
}

pub fn f81(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2397 = v1;
    let a2398 = 0u64;
    if nat_eq(a2397, a2398) {
        if v0 {
            let a2399 = v3;
            let a2400 = 0u64;
            if if if nat_eq(a2399, a2400) {
                let a2401 = v4;
                let a2402 = 43u64;
                nat_eq(a2401, a2402)
            } else {
                false
            } {
                true
            } else {
                let a2403 = v3;
                let a2404 = 43u64;
                if nat_eq(a2403, a2404) {
                    let a2405 = v4;
                    let a2406 = 0u64;
                    nat_eq(a2405, a2406)
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
        let a2407 = v1;
        let a2408 = 1u64;
        if nat_eq(a2407, a2408) {
            Adt11::C1(v2)
        } else {
            let a2409 = v1;
            let a2410 = 2u64;
            if nat_eq(a2409, a2410) {
                Adt11::C5(v2)
            } else {
                let a2411 = v1;
                let a2412 = 3u64;
                if nat_eq(a2411, a2412) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2413 = v1;
                    let a2414 = 4u64;
                    if nat_eq(a2413, a2414) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f80(v0, v1, v2, v3, v4) {
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

pub fn f82(v0: u64, v1: u64) -> Adt12 {
    let a2415 = v0;
    let a2416 = 0u64;
    if nat_eq(a2415, a2416) {
        Adt12::C0(0u64, v1)
    } else {
        let a2417 = v0;
        let a2418 = 1u64;
        if nat_eq(a2417, a2418) {
            Adt12::C1(v1)
        } else {
            let a2419 = v0;
            let a2420 = 2u64;
            if nat_eq(a2419, a2420) {
                Adt12::C2(v1)
            } else {
                let a2421 = v0;
                let a2422 = 3u64;
                if nat_eq(a2421, a2422) {
                    Adt12::C3(v1)
                } else {
                    let a2423 = v0;
                    let a2424 = 13u64;
                    if nat_lt(a2423, a2424) {
                        Adt12::C0({
                            let a2425 = v0;
                            let a2426 = 3u64;
                            nat_sub(a2425, a2426)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: u64) -> Adt13 {
    let a2427 = v0;
    let a2428 = 1u64;
    if nat_eq(a2427, a2428) {
        Adt13::C0(v1)
    } else {
        let a2429 = v0;
        let a2430 = 3u64;
        if if nat_eq(a2429, a2430) {
            true
        } else {
            let a2431 = v0;
            let a2432 = 6u64;
            nat_eq(a2431, a2432)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f84(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2434 = v0;
        let a2435 = match v1.clone() {
            Adt3::C0(h2433, _, _, _, _, _, _, _) => {
                h2433
            }
        };
        let a2436 = 0u64;
        nat_quot(a2434, a2435, a2436)
    };
    let v3: u64 = {
        let a2438 = v2;
        let a2439 = match v1.clone() {
            Adt3::C0(_, h2437, _, _, _, _, _, _) => {
                h2437
            }
        };
        let a2440 = 0u64;
        nat_quot(a2438, a2439, a2440)
    };
    f131({
        let a2442 = v0;
        let a2443 = match v1.clone() {
            Adt3::C0(h2441, _, _, _, _, _, _, _) => {
                h2441
            }
        };
        let a2444 = 0u64;
        nat_rem(a2442, a2443, a2444)
    }, {
        let a2446 = v2;
        let a2447 = match v1.clone() {
            Adt3::C0(_, h2445, _, _, _, _, _, _) => {
                h2445
            }
        };
        let a2448 = 0u64;
        nat_rem(a2446, a2447, a2448)
    }, {
        let a2450 = v3;
        let a2451 = match v1.clone() {
            Adt3::C0(_, _, h2449, _, _, _, _, _) => {
                h2449
            }
        };
        let a2452 = 0u64;
        nat_rem(a2450, a2451, a2452)
    }, {
        let a2454 = v3;
        let a2455 = match v1.clone() {
            Adt3::C0(_, _, h2453, _, _, _, _, _) => {
                h2453
            }
        };
        let a2456 = 0u64;
        nat_quot(a2454, a2455, a2456)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2457, _, _, _) => {
            h2457
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2458, _, _) => {
            h2458
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2459, _) => {
            h2459
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2460) => {
            h2460
        }
    })
}

pub fn f85(v0: Adt6, v1: u64) -> R<u64> {
    let a2462 = 0u64;
    let a2463 = match v0.clone() {
        Adt6::C0(h2461, _, _, _) => {
            h2461
        }
    };
    if if if if if nat_lt(a2462, a2463) {
        let a2465 = 0u64;
        let a2466 = match v0.clone() {
            Adt6::C0(_, h2464, _, _) => {
                h2464
            }
        };
        nat_lt(a2465, a2466)
    } else {
        false
    } {
        let a2468 = 0u64;
        let a2469 = match v0.clone() {
            Adt6::C0(_, _, _, h2467) => {
                h2467
            }
        };
        nat_lt(a2468, a2469)
    } else {
        false
    } {
        let a2471 = match v0.clone() {
            Adt6::C0(_, h2470, _, _) => {
                h2470
            }
        };
        let a2472 = 2147483648u64;
        nat_lt(a2471, a2472)
    } else {
        false
    } {
        let a2474 = match v0.clone() {
            Adt6::C0(_, _, _, h2473) => {
                h2473
            }
        };
        let a2475 = 2147483648u64;
        nat_lt(a2474, a2475)
    } else {
        false
    } {
        let v2: u64 = {
            let a2479 = v1;
            let a2480 = {
                let a2477 = 2u64;
                let a2478 = f133(match v0.clone() {
                    Adt6::C0(_, _, h2476, _) => {
                        h2476
                    }
                });
                nat_mul(a2477, a2478)?
            };
            nat_add(a2479, a2480)?
        };
        let v3: u64 = {
            let a2492 = {
                let a2487 = 1u64;
                let a2488 = {
                    let a2485 = match v0.clone() {
                        Adt6::C0(_, _, _, h2481) => {
                            h2481
                        }
                    };
                    let a2486 = {
                        let a2483 = match v0.clone() {
                            Adt6::C0(h2482, _, _, _) => {
                                h2482
                            }
                        };
                        let a2484 = 1u64;
                        nat_sub(a2483, a2484)
                    };
                    nat_mul(a2485, a2486)?
                };
                nat_add(a2487, a2488)?
            };
            let a2493 = {
                let a2490 = 2u64;
                let a2491 = f132(match v0.clone() {
                    Adt6::C0(_, _, h2489, _) => {
                        h2489
                    }
                });
                nat_mul(a2490, a2491)?
            };
            nat_add(a2492, a2493)?
        };
        let a2494 = v3;
        let a2495 = v2;
        if nat_le(a2494, a2495) {
            let a2502 = 1u64;
            let a2503 = {
                let a2499 = {
                    let a2496 = v2;
                    let a2497 = v3;
                    nat_sub(a2496, a2497)
                };
                let a2500 = match v0 {
                    Adt6::C0(_, h2498, _, _) => {
                        h2498
                    }
                };
                let a2501 = 0u64;
                nat_quot(a2499, a2500, a2501)
            };
            nat_add(a2502, a2503)
        } else {
            let a2507 = {
                let a2504 = v3;
                let a2505 = v2;
                nat_sub(a2504, a2505)
            };
            let a2508 = match v0 {
                Adt6::C0(_, h2506, _, _) => {
                    h2506
                }
            };
            if nat_lt(a2507, a2508) {
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
        let a2509 = v0;
        let a2510 = 4294967296u64;
        let a2511 = 0u64;
        nat_rem(a2509, a2510, a2511)
    };
    let a2512 = v1;
    let a2513 = 2147483648u64;
    if nat_lt(a2512, a2513) {
        v1
    } else {
        0u64
    }
}

pub fn f87(v0: u64) -> R<u64> {
    let a2516 = 2u64;
    let a2517 = {
        let a2514 = 2u64;
        let a2515 = v0;
        nat_mul(a2514, a2515)?
    };
    nat_add(a2516, a2517)
}

pub fn f88(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2519 = match v1.clone() {
        Adt14::C0(h2518, _, _, _, _, _, _, _, _, _, _) => {
            h2518
        }
    };
    let a2520 = 0u64;
    if if if if nat_eq(a2519, a2520) {
        let a2522 = match v2.clone() {
            Adt14::C0(h2521, _, _, _, _, _, _, _, _, _, _) => {
                h2521
            }
        };
        let a2523 = 0u64;
        nat_eq(a2522, a2523)
    } else {
        false
    } {
        f185(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f186(match v1.clone() {
            Adt14::C0(_, h2524, _, _, _, _, _, _, _, _, _) => {
                h2524
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2525, _, _, _, _, _, _, _, _) => {
                h2525
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2526, _, _, _, _, _, _, _) => {
                h2526
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2527, _, _, _, _, _, _) => {
                h2527
            }
        }) {
            true
        } else {
            if if if if f41(v0) {
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
            Adt14::C0(_, h2528, _, _, _, _, _, _, _, _, _) => {
                h2528
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2529, _, _, _, _, _, _, _, _) => {
                h2529
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2530, _, _, _, _, _, _, _) => {
                h2530
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2531, _, _, _, _, _, _) => {
                h2531
            }
        }) {
            0u64
        } else {
            let a2537 = match v1.clone() {
                Adt14::C0(_, _, h2532, _, _, _, _, _, _, _, _) => {
                    h2532
                }
            };
            let a2538 = {
                let a2535 = match v1.clone() {
                    Adt14::C0(_, _, _, h2533, _, _, _, _, _, _, _) => {
                        h2533
                    }
                };
                let a2536 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2534, _, _, _, _, _, _) => {
                        h2534
                    }
                };
                nat_mul(a2535, a2536)?
            };
            nat_mul(a2537, a2538)?
        };
        let v4: u64 = {
            let a2539 = v3;
            let a2540 = 0u64;
            if if nat_eq(a2539, a2540) {
                true
            } else {
                f191(v2.clone())?
            } {
                0u64
            } else {
                let a2542 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2541, _) => {
                        h2541
                    }
                };
                let a2543 = 4u64;
                let a2544 = 0u64;
                nat_quot(a2542, a2543, a2544)
            }
        };
        Ok(Adt15::C1(f192(f193(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2545, _, _, _, _, _) => {
                h2545
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2546, _, _, _, _, _) => {
                h2546
            }
        }), match v1 {
            Adt14::C0(_, h2547, _, _, _, _, _, _, _, _, _) => {
                h2547
            }
        }, v3, v4)?, 4u64, 64u64, v3, f194(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f89(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f195(v3.clone());
    let a2549 = match v2.clone() {
        Adt14::C0(h2548, _, _, _, _, _, _, _, _, _, _) => {
            h2548
        }
    };
    let a2550 = 0u64;
    if if if if nat_eq(a2549, a2550) {
        let a2552 = match v3.clone() {
            Adt14::C0(h2551, _, _, _, _, _, _, _, _, _, _) => {
                h2551
            }
        };
        let a2553 = 0u64;
        nat_eq(a2552, a2553)
    } else {
        false
    } {
        f185(v2.clone(), v3)
    } else {
        false
    } {
        if f186(match v2.clone() {
            Adt14::C0(_, h2554, _, _, _, _, _, _, _, _, _) => {
                h2554
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2555, _, _, _, _, _, _, _, _) => {
                h2555
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2556, _, _, _, _, _, _, _) => {
                h2556
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2557, _, _, _, _, _, _) => {
                h2557
            }
        }) {
            true
        } else {
            let a2559 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2558, _, _, _, _, _) => {
                    h2558
                }
            };
            let a2560 = 4u64;
            if if if if if nat_eq(a2559, a2560) {
                f196(match v2.clone() {
                    Adt14::C0(_, h2561, _, _, _, _, _, _, _, _, _) => {
                        h2561
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2562, _, _, _, _, _, _, _, _) => {
                        h2562
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2563, _, _, _, _, _, _, _) => {
                        h2563
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
                let a2565 = f197(v4.clone())?;
                let a2566 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2564, _) => {
                        h2564
                    }
                };
                nat_le(a2565, a2566)
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
            f191(v4.clone())?
        } {
            0u64
        } else {
            let a2572 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2571, _) => {
                    h2571
                }
            };
            let a2573 = 4u64;
            let a2574 = 0u64;
            nat_quot(a2572, a2573, a2574)
        };
        Ok(Adt15::C1(f198(match v2.clone() {
            Adt14::C0(_, h2575, _, _, _, _, _, _, _, _, _) => {
                h2575
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2576, _, _, _, _, _, _, _, _) => {
                h2576
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2577, _, _, _, _, _, _, _) => {
                h2577
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2578, _, _, _, _, _, _) => {
                h2578
            }
        }, v0, v5)?, 4u64, 65u64, v0, f194(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f90(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f195(v4.clone());
    if if if if if if if v1 {
        let a2580 = match v2.clone() {
            Adt14::C0(h2579, _, _, _, _, _, _, _, _, _, _) => {
                h2579
            }
        };
        let a2581 = 0u64;
        nat_eq(a2580, a2581)
    } else {
        false
    } {
        let a2583 = match v3.clone() {
            Adt14::C0(h2582, _, _, _, _, _, _, _, _, _, _) => {
                h2582
            }
        };
        let a2584 = 0u64;
        nat_eq(a2583, a2584)
    } else {
        false
    } {
        let a2586 = match v4.clone() {
            Adt14::C0(h2585, _, _, _, _, _, _, _, _, _, _) => {
                h2585
            }
        };
        let a2587 = 0u64;
        nat_eq(a2586, a2587)
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
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2591, _, _, _, _, _, _) => {
                h2591
            }
        }) {
            true
        } else {
            let a2593 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2592, _, _, _, _, _) => {
                    h2592
                }
            };
            let a2594 = 4u64;
            if if if if if if if nat_eq(a2593, a2594) {
                let a2596 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2595, _, _, _, _, _) => {
                        h2595
                    }
                };
                let a2597 = 4u64;
                nat_eq(a2596, a2597)
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
                let a2599 = f197(v5.clone())?;
                let a2600 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2598, _) => {
                        h2598
                    }
                };
                nat_le(a2599, a2600)
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
            0u64
        } else {
            let a2610 = match v2.clone() {
                Adt14::C0(_, _, h2605, _, _, _, _, _, _, _, _) => {
                    h2605
                }
            };
            let a2611 = {
                let a2608 = match v2.clone() {
                    Adt14::C0(_, _, _, h2606, _, _, _, _, _, _, _) => {
                        h2606
                    }
                };
                let a2609 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2607, _, _, _, _, _, _) => {
                        h2607
                    }
                };
                nat_mul(a2608, a2609)?
            };
            nat_mul(a2610, a2611)?
        };
        let v7: u64 = {
            let a2612 = v6;
            let a2613 = 0u64;
            if if nat_eq(a2612, a2613) {
                true
            } else {
                f191(v5.clone())?
            } {
                0u64
            } else {
                let a2615 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2614, _) => {
                        h2614
                    }
                };
                let a2616 = 4u64;
                let a2617 = 0u64;
                nat_quot(a2615, a2616, a2617)
            }
        };
        Ok(Adt15::C1(f199(match v2 {
            Adt14::C0(_, h2618, _, _, _, _, _, _, _, _, _) => {
                h2618
            }
        }, v6, v7)?, 4u64, 66u64, v6, f194(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
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

pub fn f92(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
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

pub fn f93(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2619 = v2;
    let a2620 = 0u64;
    if nat_eq(a2619, a2620) {
        Ok(0u64)
    } else {
        let a2623 = 2u64;
        let a2624 = {
            let a2621 = v0;
            let a2622 = v1;
            nat_mul(a2621, a2622)?
        };
        nat_mul(a2623, a2624)
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2625 = v2;
    let a2626 = f181(v0, v1)?;
    nat_mul(a2625, a2626)
}

pub fn f95(v0: u64) -> u64 {
    let a2627 = v0;
    let a2628 = 30u64;
    if nat_eq(a2627, a2628) {
        3u64
    } else {
        1u64
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a2631 = 5u64;
    let a2632 = {
        let a2629 = 4u64;
        let a2630 = v0;
        nat_mul(a2629, a2630)?
    };
    nat_add(a2631, a2632)
}

pub fn f97(v0: u64) -> bool {
    let m2633 = f202(v0);
    match m2633 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a2635 = 1u64;
            let a2636 = match v1 {
                Adt19::C0(_, h2634, _) => {
                    h2634
                }
            };
            nat_lt(a2635, a2636)
        }
    }
}

pub fn f98(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a2638 = 0u64;
        let a2639 = match v0.clone() {
            Adt1::C0(_, h2637, _, _, _, _, _, _, _, _, _, _) => {
                h2637
            }
        };
        nat_lt(a2638, a2639)
    };
    let a2641 = match v1.clone() {
        Adt2::C0(h2640, _, _, _, _, _, _, _, _, _, _) => {
            h2640
        }
    };
    let a2642 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a2641, a2642) {
        let a2644 = match v2.clone() {
            Adt2::C0(h2643, _, _, _, _, _, _, _, _, _, _) => {
                h2643
            }
        };
        let a2645 = 0u64;
        nat_eq(a2644, a2645)
    } else {
        false
    } {
        let a2647 = match v3.clone() {
            Adt2::C0(h2646, _, _, _, _, _, _, _, _, _, _) => {
                h2646
            }
        };
        let a2648 = 0u64;
        nat_eq(a2647, a2648)
    } else {
        false
    } {
        let a2650 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2649, _, _, _, _, _, _) => {
                h2649
            }
        };
        let a2651 = 1u64;
        nat_eq(a2650, a2651)
    } else {
        false
    } {
        let a2653 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2652, _, _, _, _, _, _) => {
                h2652
            }
        };
        let a2654 = 1u64;
        nat_eq(a2653, a2654)
    } else {
        false
    } {
        let a2656 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2655, _, _, _, _, _, _) => {
                h2655
            }
        };
        let a2657 = 1u64;
        nat_eq(a2656, a2657)
    } else {
        false
    } {
        let a2659 = 0u64;
        let a2660 = match v1.clone() {
            Adt2::C0(_, _, h2658, _, _, _, _, _, _, _, _) => {
                h2658
            }
        };
        nat_lt(a2659, a2660)
    } else {
        false
    } {
        let a2662 = match v3.clone() {
            Adt2::C0(_, _, _, h2661, _, _, _, _, _, _, _) => {
                h2661
            }
        };
        let a2663 = 1u64;
        nat_eq(a2662, a2663)
    } else {
        false
    } {
        let a2666 = match v3.clone() {
            Adt2::C0(_, h2664, _, _, _, _, _, _, _, _, _) => {
                h2664
            }
        };
        let a2667 = match v1.clone() {
            Adt2::C0(_, h2665, _, _, _, _, _, _, _, _, _) => {
                h2665
            }
        };
        nat_eq(a2666, a2667)
    } else {
        false
    } {
        let a2670 = match v3.clone() {
            Adt2::C0(_, _, h2668, _, _, _, _, _, _, _, _) => {
                h2668
            }
        };
        let a2671 = match v1.clone() {
            Adt2::C0(_, _, _, h2669, _, _, _, _, _, _, _) => {
                h2669
            }
        };
        nat_eq(a2670, a2671)
    } else {
        false
    } {
        if v4 {
            f28(v1.clone(), v2.clone())
        } else {
            let a2674 = match v2.clone() {
                Adt2::C0(_, h2672, _, _, _, _, _, _, _, _, _) => {
                    h2672
                }
            };
            let a2675 = match v1.clone() {
                Adt2::C0(_, _, h2673, _, _, _, _, _, _, _, _) => {
                    h2673
                }
            };
            if if nat_eq(a2674, a2675) {
                let a2678 = match v2.clone() {
                    Adt2::C0(_, _, h2676, _, _, _, _, _, _, _, _) => {
                        h2676
                    }
                };
                let a2679 = match v1.clone() {
                    Adt2::C0(_, _, _, h2677, _, _, _, _, _, _, _) => {
                        h2677
                    }
                };
                nat_eq(a2678, a2679)
            } else {
                false
            } {
                let a2681 = match v2.clone() {
                    Adt2::C0(_, _, _, h2680, _, _, _, _, _, _, _) => {
                        h2680
                    }
                };
                let a2682 = 1u64;
                nat_eq(a2681, a2682)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f19(v1.clone(), f20(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f19(v2.clone(), f20(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v3.clone())?
    } else {
        false
    } {
        f25(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f25(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2684 = f20(v3.clone(), 1u64)?;
            let a2685 = f158(match v1.clone() {
                Adt2::C0(_, _, h2683, _, _, _, _, _, _, _, _) => {
                    h2683
                }
            }, v4)?;
            nat_mul(a2684, a2685)?
        }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C36(f24(v2, 1u64), v4, match v0 {
            Adt1::C0(h2686, _, _, _, _, _, _, _, _, _, _, _) => {
                h2686
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f99(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h2687, _, _, _, _, _, _, _, _, _) => {
            h2687
        }
    }, match v0.clone() {
        Adt1::C0(h2688, _, _, _, _, _, _, _, _, _, _, _) => {
            h2688
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h2689, _, _, _, _, _, _, _, _, _) => {
            h2689
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h2690, _, _, _, _, _, _, _) => {
            h2690
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h2691, _, _, _, _, _, _, _, _) => {
            h2691
        }
    }, match v0.clone() {
        Adt1::C0(_, h2692, _, _, _, _, _, _, _, _, _, _) => {
            h2692
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h2693, _, _, _, _, _, _, _, _) => {
            h2693
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h2694, _, _, _, _, _, _) => {
            h2694
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h2695, _, _, _, _, _, _, _, _, _, _) => {
            h2695
        }
    };
    let v7: bool = {
        let a2698 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h2696, _, _, _) => {
                h2696
            }
        };
        let a2699 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2697, _, _, _, _, _) => {
                h2697
            }
        };
        nat_lt(a2698, a2699)
    };
    let a2700 = v6;
    let a2701 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a2700, a2701) {
        true
    } else {
        let a2702 = v6;
        let a2703 = 1u64;
        nat_eq(a2702, a2703)
    } {
        let a2705 = match v2.clone() {
            Adt2::C0(h2704, _, _, _, _, _, _, _, _, _, _) => {
                h2704
            }
        };
        let a2706 = 0u64;
        nat_eq(a2705, a2706)
    } else {
        false
    } {
        let a2708 = match v3.clone() {
            Adt2::C0(h2707, _, _, _, _, _, _, _, _, _, _) => {
                h2707
            }
        };
        let a2709 = 0u64;
        nat_eq(a2708, a2709)
    } else {
        false
    } {
        let a2711 = match v1.clone() {
            Adt2::C0(_, _, _, h2710, _, _, _, _, _, _, _) => {
                h2710
            }
        };
        let a2712 = 1u64;
        nat_eq(a2711, a2712)
    } else {
        false
    } {
        let a2715 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2713, _, _, _, _, _, _) => {
                h2713
            }
        };
        let a2716 = match v2.clone() {
            Adt2::C0(_, _, _, h2714, _, _, _, _, _, _, _) => {
                h2714
            }
        };
        nat_eq(a2715, a2716)
    } else {
        false
    } {
        let a2719 = match v3.clone() {
            Adt2::C0(_, _, _, h2717, _, _, _, _, _, _, _) => {
                h2717
            }
        };
        let a2720 = match v2.clone() {
            Adt2::C0(_, _, _, h2718, _, _, _, _, _, _, _) => {
                h2718
            }
        };
        nat_eq(a2719, a2720)
    } else {
        false
    } {
        let a2723 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2721, _, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2722, _, _, _, _, _, _) => {
                h2722
            }
        };
        nat_eq(a2723, a2724)
    } else {
        false
    } {
        let a2726 = 0u64;
        let a2727 = match v4.clone() {
            Adt6::C0(h2725, _, _, _) => {
                h2725
            }
        };
        if if if if if nat_lt(a2726, a2727) {
            let a2729 = 0u64;
            let a2730 = match v4.clone() {
                Adt6::C0(_, h2728, _, _) => {
                    h2728
                }
            };
            nat_lt(a2729, a2730)
        } else {
            false
        } {
            let a2732 = match v4.clone() {
                Adt6::C0(_, h2731, _, _) => {
                    h2731
                }
            };
            let a2733 = 2147483648u64;
            nat_lt(a2732, a2733)
        } else {
            false
        } {
            let a2735 = 0u64;
            let a2736 = match v4.clone() {
                Adt6::C0(_, _, _, h2734) => {
                    h2734
                }
            };
            nat_lt(a2735, a2736)
        } else {
            false
        } {
            let a2738 = match v4.clone() {
                Adt6::C0(_, _, _, h2737) => {
                    h2737
                }
            };
            let a2739 = 2147483648u64;
            nat_lt(a2738, a2739)
        } else {
            false
        } {
            let a2742 = match v3.clone() {
                Adt2::C0(_, h2740, _, _, _, _, _, _, _, _, _) => {
                    h2740
                }
            };
            let a2743 = f85(v4.clone(), match v2.clone() {
                Adt2::C0(_, h2741, _, _, _, _, _, _, _, _, _) => {
                    h2741
                }
            })?;
            nat_eq(a2742, a2743)
        } else {
            false
        }
    } else {
        false
    } {
        let a2745 = 0u64;
        let a2746 = match v5.clone() {
            Adt6::C0(h2744, _, _, _) => {
                h2744
            }
        };
        if if if if if nat_lt(a2745, a2746) {
            let a2748 = 0u64;
            let a2749 = match v5.clone() {
                Adt6::C0(_, h2747, _, _) => {
                    h2747
                }
            };
            nat_lt(a2748, a2749)
        } else {
            false
        } {
            let a2751 = match v5.clone() {
                Adt6::C0(_, h2750, _, _) => {
                    h2750
                }
            };
            let a2752 = 2147483648u64;
            nat_lt(a2751, a2752)
        } else {
            false
        } {
            let a2754 = 0u64;
            let a2755 = match v5.clone() {
                Adt6::C0(_, _, _, h2753) => {
                    h2753
                }
            };
            nat_lt(a2754, a2755)
        } else {
            false
        } {
            let a2757 = match v5.clone() {
                Adt6::C0(_, _, _, h2756) => {
                    h2756
                }
            };
            let a2758 = 2147483648u64;
            nat_lt(a2757, a2758)
        } else {
            false
        } {
            let a2761 = match v3.clone() {
                Adt2::C0(_, _, h2759, _, _, _, _, _, _, _, _) => {
                    h2759
                }
            };
            let a2762 = f85(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2760, _, _, _, _, _, _, _, _) => {
                    h2760
                }
            })?;
            nat_eq(a2761, a2762)
        } else {
            false
        }
    } else {
        false
    } {
        f203(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f203(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f203(v1.clone(), {
            let a2763 = v6;
            let a2764 = 0u64;
            if nat_eq(a2763, a2764) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f25(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f25(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2773 = {
                let a2771 = {
                    let a2769 = 3u64;
                    let a2770 = {
                        let a2767 = match v4.clone() {
                            Adt6::C0(h2765, _, _, _) => {
                                h2765
                            }
                        };
                        let a2768 = match v5.clone() {
                            Adt6::C0(h2766, _, _, _) => {
                                h2766
                            }
                        };
                        nat_mul(a2767, a2768)?
                    };
                    nat_mul(a2769, a2770)?
                };
                let a2772 = 2u64;
                nat_add(a2771, a2772)?
            };
            let a2774 = f20(v3.clone(), 1u64)?;
            nat_mul(a2773, a2774)?
        }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C34(f24(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f100(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f28(v0.clone(), v1.clone()) {
        f28(v0.clone(), v2.clone())
    } else {
        false
    } {
        f26(v0.clone())?
    } else {
        false
    } {
        f26(v1.clone())?
    } else {
        false
    } {
        f26(v2.clone())?
    } else {
        false
    } {
        if f25(v0.clone(), v2.clone())? {
            true
        } else {
            f43(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f25(v1.clone(), v2.clone())? {
            true
        } else {
            f43(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2775 = 6u64;
            let a2776 = f20(v0.clone(), 1u64)?;
            nat_mul(a2775, a2776)?
        }, 2u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C33(f24(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f101(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2777 = v2;
    match m2777 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h2778, _, _, _, _, _, _, _, _, _) => {
                    h2778
                }
            };
            let v6: u64 = {
                let a2784 = match v1.clone() {
                    Adt2::C0(_, _, h2779, _, _, _, _, _, _, _, _) => {
                        h2779
                    }
                };
                let a2785 = {
                    let a2782 = match v1.clone() {
                        Adt2::C0(_, _, _, h2780, _, _, _, _, _, _, _) => {
                            h2780
                        }
                    };
                    let a2783 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2781, _, _, _, _, _, _) => {
                            h2781
                        }
                    };
                    nat_mul(a2782, a2783)?
                };
                nat_mul(a2784, a2785)?
            };
            if if if if if if if if if if f28(v1.clone(), v4.clone()) {
                f28(v1.clone(), v3.clone())
            } else {
                false
            } {
                f204(v1.clone(), v6)?
            } else {
                false
            } {
                f204(v4.clone(), v6)?
            } else {
                false
            } {
                f204(v3.clone(), v6)?
            } else {
                false
            } {
                if f25(v1.clone(), v3.clone())? {
                    true
                } else {
                    f43(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f25(v4.clone(), v3.clone())? {
                    true
                } else {
                    f43(v4, v3.clone())
                }
            } else {
                false
            } {
                let a2786 = v5;
                let a2787 = 2147483647u64;
                nat_le(a2786, a2787)
            } else {
                false
            } {
                let a2788 = v6;
                let a2789 = 2147483647u64;
                nat_le(a2788, a2789)
            } else {
                false
            } {
                let a2791 = match v0.clone() {
                    Adt1::C0(h2790, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2790
                    }
                };
                let a2792 = 4294967295u64;
                if nat_le(a2791, a2792) {
                    let a2794 = match v0.clone() {
                        Adt1::C0(_, h2793, _, _, _, _, _, _, _, _, _, _) => {
                            h2793
                        }
                    };
                    let a2795 = 4294967295u64;
                    nat_le(a2794, a2795)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2796 = v5;
                    let a2797 = 0u64;
                    if nat_eq(a2796, a2797) {
                        0u64
                    } else {
                        let a2798 = v6;
                        let a2799 = f157(v5)?;
                        nat_mul(a2798, a2799)?
                    }
                }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h2800, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2800
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f102(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h2801, _, _, _, _, _, _, _, _, _, _, _) => {
            h2801
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h2802, _, _, _, _, _, _, _, _, _, _) => {
            h2802
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h2803, _, _, _, _, _, _, _, _, _) => {
            h2803
        }
    };
    let v6: u64 = {
        let a2805 = match v1.clone() {
            Adt2::C0(_, h2804, _, _, _, _, _, _, _, _, _) => {
                h2804
            }
        };
        let a2806 = v4;
        let a2807 = 0u64;
        nat_quot(a2805, a2806, a2807)
    };
    let v7: u64 = {
        let a2809 = match v1.clone() {
            Adt2::C0(h2808, _, _, _, _, _, _, _, _, _, _) => {
                h2808
            }
        };
        let a2810 = 0u64;
        if nat_eq(a2809, a2810) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f164(match v1.clone() {
        Adt2::C0(h2811, _, _, _, _, _, _, _, _, _, _) => {
            h2811
        }
    }) {
        let a2814 = match v1.clone() {
            Adt2::C0(h2812, _, _, _, _, _, _, _, _, _, _) => {
                h2812
            }
        };
        let a2815 = match v2.clone() {
            Adt2::C0(h2813, _, _, _, _, _, _, _, _, _, _) => {
                h2813
            }
        };
        nat_eq(a2814, a2815)
    } else {
        false
    } {
        let a2816 = 0u64;
        let a2817 = v3;
        nat_lt(a2816, a2817)
    } else {
        false
    } {
        let a2818 = v3;
        let a2819 = 2147483648u64;
        nat_lt(a2818, a2819)
    } else {
        false
    } {
        let a2820 = 0u64;
        let a2821 = v4;
        nat_lt(a2820, a2821)
    } else {
        false
    } {
        let a2822 = v4;
        let a2823 = 2147483648u64;
        nat_lt(a2822, a2823)
    } else {
        false
    } {
        let a2824 = v5;
        let a2825 = 2147483648u64;
        nat_lt(a2824, a2825)
    } else {
        false
    } {
        let a2826 = 0u64;
        let a2827 = v6;
        nat_lt(a2826, a2827)
    } else {
        false
    } {
        let a2829 = 0u64;
        let a2830 = match v2.clone() {
            Adt2::C0(_, h2828, _, _, _, _, _, _, _, _, _) => {
                h2828
            }
        };
        nat_lt(a2829, a2830)
    } else {
        false
    } {
        let a2834 = match v1.clone() {
            Adt2::C0(_, h2831, _, _, _, _, _, _, _, _, _) => {
                h2831
            }
        };
        let a2835 = {
            let a2832 = v6;
            let a2833 = v4;
            nat_mul(a2832, a2833)?
        };
        nat_eq(a2834, a2835)
    } else {
        false
    } {
        let a2837 = match v2.clone() {
            Adt2::C0(_, _, h2836, _, _, _, _, _, _, _, _) => {
                h2836
            }
        };
        let a2838 = v4;
        nat_eq(a2837, a2838)
    } else {
        false
    } {
        let a2851 = {
            let a2844 = {
                let a2842 = match v2.clone() {
                    Adt2::C0(_, h2839, _, _, _, _, _, _, _, _, _) => {
                        h2839
                    }
                };
                let a2843 = {
                    let a2840 = v5;
                    let a2841 = 2u64;
                    nat_mul(a2840, a2841)?
                };
                nat_add(a2842, a2843)?
            };
            let a2845 = v3;
            nat_add(a2844, a2845)?
        };
        let a2852 = {
            let a2849 = {
                let a2847 = match v1.clone() {
                    Adt2::C0(_, _, h2846, _, _, _, _, _, _, _, _) => {
                        h2846
                    }
                };
                let a2848 = v3;
                nat_mul(a2847, a2848)?
            };
            let a2850 = v6;
            nat_add(a2849, a2850)?
        };
        nat_eq(a2851, a2852)
    } else {
        false
    } {
        f205(v1.clone(), v7)?
    } else {
        false
    } {
        f205(v2.clone(), v7)?
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a2853 = f20(v2.clone(), 1u64)?;
            let a2854 = f178(v8.clone())?;
            nat_mul(a2853, a2854)?
        }, v7, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h2855, _, _, _, _, _, _, _, _, _, _) => {
                h2855
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f103(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2856 = v1;
    match m2856 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a2858 = match v0.clone() {
                Adt2::C0(h2857, _, _, _, _, _, _, _, _, _, _) => {
                    h2857
                }
            };
            let a2859 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a2858, a2859) {
                let a2861 = match v3.clone() {
                    Adt2::C0(h2860, _, _, _, _, _, _, _, _, _, _) => {
                        h2860
                    }
                };
                let a2862 = 0u64;
                nat_eq(a2861, a2862)
            } else {
                false
            } {
                let a2864 = match v2.clone() {
                    Adt2::C0(h2863, _, _, _, _, _, _, _, _, _, _) => {
                        h2863
                    }
                };
                let a2865 = 0u64;
                nat_eq(a2864, a2865)
            } else {
                false
            } {
                let a2867 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h2866, _, _, _, _, _, _) => {
                        h2866
                    }
                };
                let a2868 = 1u64;
                nat_eq(a2867, a2868)
            } else {
                false
            } {
                let a2870 = match v3.clone() {
                    Adt2::C0(_, _, _, h2869, _, _, _, _, _, _, _) => {
                        h2869
                    }
                };
                let a2871 = 1u64;
                nat_eq(a2870, a2871)
            } else {
                false
            } {
                let a2873 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2872, _, _, _, _, _, _) => {
                        h2872
                    }
                };
                let a2874 = 1u64;
                nat_eq(a2873, a2874)
            } else {
                false
            } {
                let a2877 = match v0.clone() {
                    Adt2::C0(_, _, h2875, _, _, _, _, _, _, _, _) => {
                        h2875
                    }
                };
                let a2878 = match v3.clone() {
                    Adt2::C0(_, _, h2876, _, _, _, _, _, _, _, _) => {
                        h2876
                    }
                };
                nat_eq(a2877, a2878)
            } else {
                false
            } {
                let a2883 = match v3.clone() {
                    Adt2::C0(_, h2879, _, _, _, _, _, _, _, _, _) => {
                        h2879
                    }
                };
                let a2884 = {
                    let a2881 = match v0.clone() {
                        Adt2::C0(_, h2880, _, _, _, _, _, _, _, _, _) => {
                            h2880
                        }
                    };
                    let a2882 = 1u64;
                    nat_add(a2881, a2882)?
                };
                nat_le(a2883, a2884)
            } else {
                false
            } {
                let a2887 = match v2.clone() {
                    Adt2::C0(_, h2885, _, _, _, _, _, _, _, _, _) => {
                        h2885
                    }
                };
                let a2888 = match v0.clone() {
                    Adt2::C0(_, _, h2886, _, _, _, _, _, _, _, _) => {
                        h2886
                    }
                };
                nat_eq(a2887, a2888)
            } else {
                false
            } {
                let a2896 = match v2.clone() {
                    Adt2::C0(_, _, h2889, _, _, _, _, _, _, _, _) => {
                        h2889
                    }
                };
                let a2897 = {
                    let a2894 = {
                        let a2891 = match v0.clone() {
                            Adt2::C0(_, h2890, _, _, _, _, _, _, _, _, _) => {
                                h2890
                            }
                        };
                        let a2892 = 1u64;
                        nat_add(a2891, a2892)?
                    };
                    let a2895 = match v3.clone() {
                        Adt2::C0(_, h2893, _, _, _, _, _, _, _, _, _) => {
                            h2893
                        }
                    };
                    nat_sub(a2894, a2895)
                };
                nat_eq(a2896, a2897)
            } else {
                false
            } {
                let a2900 = match v2.clone() {
                    Adt2::C0(_, _, _, h2898, _, _, _, _, _, _, _) => {
                        h2898
                    }
                };
                let a2901 = match v0.clone() {
                    Adt2::C0(_, _, _, h2899, _, _, _, _, _, _, _) => {
                        h2899
                    }
                };
                nat_eq(a2900, a2901)
            } else {
                false
            } {
                let a2903 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h2902, _, _, _, _, _, _) => {
                        h2902
                    }
                };
                let a2904 = 1u64;
                nat_eq(a2903, a2904)
            } else {
                false
            } {
                f34(v2.clone())?
            } else {
                false
            } {
                let a2906 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h2905, _, _, _, _, _) => {
                        h2905
                    }
                };
                let a2907 = 4u64;
                nat_eq(a2906, a2907)
            } else {
                false
            } {
                let a2909 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h2908, _, _, _, _, _) => {
                        h2908
                    }
                };
                let a2910 = 4u64;
                nat_eq(a2909, a2910)
            } else {
                false
            } {
                let a2915 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2911, _, _, _, _) => {
                        h2911
                    }
                };
                let a2916 = {
                    let a2913 = match v0.clone() {
                        Adt2::C0(_, h2912, _, _, _, _, _, _, _, _, _) => {
                            h2912
                        }
                    };
                    let a2914 = 4u64;
                    nat_mul(a2913, a2914)?
                };
                nat_eq(a2915, a2916)
            } else {
                false
            } {
                let a2921 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2917, _, _, _, _) => {
                        h2917
                    }
                };
                let a2922 = {
                    let a2919 = match v3.clone() {
                        Adt2::C0(_, h2918, _, _, _, _, _, _, _, _, _) => {
                            h2918
                        }
                    };
                    let a2920 = 4u64;
                    nat_mul(a2919, a2920)?
                };
                nat_eq(a2921, a2922)
            } else {
                false
            } {
                f25(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f25(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2924 = f20(v2.clone(), 1u64)?;
                    let a2925 = f177(match v3.clone() {
                        Adt2::C0(_, h2923, _, _, _, _, _, _, _, _, _) => {
                            h2923
                        }
                    })?;
                    nat_mul(a2924, a2925)?
                }, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C29(f24(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f104(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h2926, _, _, _, _, _, _, _, _, _, _) => {
            h2926
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h2927, _, _, _, _, _, _, _, _) => {
                h2927
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2928, _, _, _, _, _, _, _, _, _) => {
                h2928
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h2929, _, _, _, _, _, _) => {
                h2929
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h2930, _, _, _, _, _, _, _, _) => {
                h2930
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h2931, _, _, _, _, _, _, _, _, _) => {
                h2931
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h2932, _, _, _, _, _, _, _) => {
                h2932
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h2933, _, _, _, _, _) => {
                h2933
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a2935 = match v1.clone() {
        Adt1::C0(h2934, _, _, _, _, _, _, _, _, _, _, _) => {
            h2934
        }
    };
    let a2936 = 1u64;
    if if if if if if if if if if if nat_le(a2935, a2936) {
        let a2938 = match v2.clone() {
            Adt2::C0(h2937, _, _, _, _, _, _, _, _, _, _) => {
                h2937
            }
        };
        let a2939 = 1u64;
        nat_le(a2938, a2939)
    } else {
        false
    } {
        let a2941 = match v3.clone() {
            Adt2::C0(h2940, _, _, _, _, _, _, _, _, _, _) => {
                h2940
            }
        };
        let a2942 = 0u64;
        nat_eq(a2941, a2942)
    } else {
        false
    } {
        let a2947 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2943, _, _, _, _, _) => {
                h2943
            }
        };
        let a2948 = {
            let a2945 = match v2.clone() {
                Adt2::C0(h2944, _, _, _, _, _, _, _, _, _, _) => {
                    h2944
                }
            };
            let a2946 = 0u64;
            if nat_eq(a2945, a2946) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a2947, a2948)
    } else {
        false
    } {
        f34(v3.clone())?
    } else {
        false
    } {
        f25(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f206(v4.clone(), match v2.clone() {
            Adt2::C0(_, h2949, _, _, _, _, _, _, _, _, _) => {
                h2949
            }
        }, match v3.clone() {
            Adt2::C0(_, h2950, _, _, _, _, _, _, _, _, _) => {
                h2950
            }
        })?
    } else {
        false
    } {
        if v0 {
            f206(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2951, _, _, _, _, _, _, _, _) => {
                    h2951
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h2952, _, _, _, _, _, _, _, _) => {
                    h2952
                }
            })?
        } else {
            let a2955 = match v2.clone() {
                Adt2::C0(_, _, h2953, _, _, _, _, _, _, _, _) => {
                    h2953
                }
            };
            let a2956 = match v3.clone() {
                Adt2::C0(_, _, h2954, _, _, _, _, _, _, _, _) => {
                    h2954
                }
            };
            nat_eq(a2955, a2956)
        }
    } else {
        false
    } {
        let a2959 = match v2.clone() {
            Adt2::C0(_, _, _, h2957, _, _, _, _, _, _, _) => {
                h2957
            }
        };
        let a2960 = match v3.clone() {
            Adt2::C0(_, _, _, h2958, _, _, _, _, _, _, _) => {
                h2958
            }
        };
        nat_eq(a2959, a2960)
    } else {
        false
    } {
        let a2963 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2961, _, _, _, _, _, _) => {
                h2961
            }
        };
        let a2964 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2962, _, _, _, _, _, _) => {
                h2962
            }
        };
        nat_eq(a2963, a2964)
    } else {
        false
    } {
        let a2969 = {
            let a2967 = match v4.clone() {
                Adt6::C0(h2965, _, _, _) => {
                    h2965
                }
            };
            let a2968 = match v5.clone() {
                Adt6::C0(h2966, _, _, _) => {
                    h2966
                }
            };
            nat_mul(a2967, a2968)?
        };
        let a2970 = 2147483648u64;
        nat_lt(a2969, a2970)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a2972 = match v1 {
                Adt1::C0(h2971, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2971
                }
            };
            let a2973 = 0u64;
            nat_eq(a2972, a2973)
        }, match v2.clone() {
            Adt2::C0(h2974, _, _, _, _, _, _, _, _, _, _) => {
                h2974
            }
        });
        Ok(Adt10::C4({
            let a2975 = f20(v3.clone(), 1u64)?;
            let a2976 = f173(v6.clone())?;
            nat_mul(a2975, a2976)?
        }, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f105(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2977 = v2;
    match m2977 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a2979 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h2978, _, _, _, _, _) => {
                        h2978
                    }
                };
                let a2980 = 1u64;
                nat_eq(a2979, a2980)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2981, _, _, _, _, _, _, _, _, _) => {
                    h2981
                }
            }, match v0.clone() {
                Adt1::C0(h2982, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2982
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2983, _, _, _, _, _, _, _, _, _) => {
                    h2983
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2984, _, _, _, _, _, _, _) => {
                    h2984
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h2985, _, _, _, _, _, _, _, _) => {
                        h2985
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h2986, _, _, _, _, _, _, _, _, _, _) => {
                        h2986
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h2987, _, _, _, _, _, _, _, _) => {
                        h2987
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h2988, _, _, _, _, _, _) => {
                        h2988
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2989, _, _, _, _, _, _, _) => {
                        h2989
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h2990, _, _, _, _, _, _, _, _) => {
                        h2990
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h2991, _, _, _, _, _, _) => {
                        h2991
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2992, _, _, _, _, _, _, _) => {
                        h2992
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h2993, _, _, _, _, _, _, _, _) => {
                        h2993
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h2994, _, _, _, _, _, _, _, _, _, _) => {
                    h2994
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h2995, _, _, _, _, _, _, _, _, _, _) => {
                    h2995
                }
            };
            let v14: u64 = {
                let a2996 = v12;
                let a2997 = 0u64;
                if nat_eq(a2996, a2997) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a2999 = match v4.clone() {
                    Adt2::C0(_, h2998, _, _, _, _, _, _, _, _, _) => {
                        h2998
                    }
                };
                let a3000 = v14;
                nat_mul(a2999, a3000)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3001, _, _, _) => {
                        h3001
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3002, _, _, _, _) => {
                        h3002
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3003, _, _) => {
                        h3003
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3004, _, _, _) => {
                        h3004
                    }
                }
            };
            let a3006 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h3005, _, _, _, _, _) => {
                    h3005
                }
            };
            let a3007 = 1u64;
            if if if if if if if if if if nat_le(a3006, a3007) {
                let a3008 = v12;
                let a3009 = 0u64;
                if nat_eq(a3008, a3009) {
                    true
                } else {
                    let a3010 = v12;
                    let a3011 = 1u64;
                    if nat_eq(a3010, a3011) {
                        let a3012 = v13;
                        let a3013 = 1u64;
                        nat_eq(a3012, a3013)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3014 = v13;
                let a3015 = 0u64;
                if nat_eq(a3014, a3015) {
                    true
                } else {
                    let a3016 = v13;
                    let a3017 = 1u64;
                    nat_eq(a3016, a3017)
                }
            } else {
                false
            } {
                let a3019 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3018, _, _, _, _, _) => {
                        h3018
                    }
                };
                let a3020 = v14;
                nat_eq(a3019, a3020)
            } else {
                false
            } {
                let a3028 = match v3.clone() {
                    Adt2::C0(_, h3021, _, _, _, _, _, _, _, _, _) => {
                        h3021
                    }
                };
                let a3029 = {
                    let a3026 = v9;
                    let a3027 = {
                        let a3024 = match v6.clone() {
                            Adt6::C0(h3022, _, _, _) => {
                                h3022
                            }
                        };
                        let a3025 = match v7.clone() {
                            Adt6::C0(h3023, _, _, _) => {
                                h3023
                            }
                        };
                        nat_mul(a3024, a3025)?
                    };
                    nat_mul(a3026, a3027)?
                };
                nat_eq(a3028, a3029)
            } else {
                false
            } {
                let a3032 = match v3.clone() {
                    Adt2::C0(_, _, h3030, _, _, _, _, _, _, _, _) => {
                        h3030
                    }
                };
                let a3033 = f85(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h3031, _, _, _, _, _, _, _, _, _) => {
                        h3031
                    }
                })?;
                nat_eq(a3032, a3033)
            } else {
                false
            } {
                let a3035 = 0u64;
                let a3036 = match v3.clone() {
                    Adt2::C0(_, _, h3034, _, _, _, _, _, _, _, _) => {
                        h3034
                    }
                };
                nat_lt(a3035, a3036)
            } else {
                false
            } {
                if v5 {
                    let a3038 = match v1 {
                        Adt2::C0(_, _, _, h3037, _, _, _, _, _, _, _) => {
                            h3037
                        }
                    };
                    let a3039 = v9;
                    if if if nat_eq(a3038, a3039) {
                        let a3041 = match v3.clone() {
                            Adt2::C0(_, _, _, h3040, _, _, _, _, _, _, _) => {
                                h3040
                            }
                        };
                        let a3042 = f85(v7.clone(), v11)?;
                        nat_eq(a3041, a3042)
                    } else {
                        false
                    } {
                        let a3044 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3043, _, _, _, _, _, _) => {
                                h3043
                            }
                        };
                        let a3045 = v10;
                        nat_eq(a3044, a3045)
                    } else {
                        false
                    } {
                        let a3047 = 0u64;
                        let a3048 = match v3.clone() {
                            Adt2::C0(_, _, _, h3046, _, _, _, _, _, _, _) => {
                                h3046
                            }
                        };
                        nat_lt(a3047, a3048)
                    } else {
                        false
                    }
                } else {
                    let a3050 = match v1 {
                        Adt2::C0(_, _, h3049, _, _, _, _, _, _, _, _) => {
                            h3049
                        }
                    };
                    let a3051 = v9;
                    if if if nat_eq(a3050, a3051) {
                        let a3053 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3052, _, _, _, _, _, _) => {
                                h3052
                            }
                        };
                        let a3054 = 1u64;
                        nat_eq(a3053, a3054)
                    } else {
                        false
                    } {
                        let a3056 = match v3.clone() {
                            Adt2::C0(_, _, _, h3055, _, _, _, _, _, _, _) => {
                                h3055
                            }
                        };
                        let a3057 = v10;
                        nat_eq(a3056, a3057)
                    } else {
                        false
                    } {
                        let a3059 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3058, _, _, _, _, _, _) => {
                                h3058
                            }
                        };
                        let a3060 = 1u64;
                        nat_eq(a3059, a3060)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f44(match v4.clone() {
                    Adt2::C0(_, h3061, _, _, _, _, _, _, _, _, _) => {
                        h3061
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a3081 = {
                        let a3078 = {
                            let a3076 = {
                                let a3070 = {
                                    let a3064 = {
                                        let a3062 = v10;
                                        let a3063 = 1u64;
                                        nat_sub(a3062, a3063)
                                    };
                                    let a3065 = v17;
                                    nat_mul(a3064, a3065)?
                                };
                                let a3071 = {
                                    let a3068 = {
                                        let a3066 = v9;
                                        let a3067 = 1u64;
                                        nat_sub(a3066, a3067)
                                    };
                                    let a3069 = v16;
                                    nat_mul(a3068, a3069)?
                                };
                                nat_add(a3070, a3071)?
                            };
                            let a3077 = {
                                let a3074 = {
                                    let a3072 = v11;
                                    let a3073 = 1u64;
                                    nat_sub(a3072, a3073)
                                };
                                let a3075 = v15;
                                nat_mul(a3074, a3075)?
                            };
                            nat_add(a3076, a3077)?
                        };
                        let a3079 = v15;
                        nat_add(a3078, a3079)?
                    };
                    let a3082 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3080, _) => {
                            h3080
                        }
                    };
                    nat_le(a3081, a3082)
                }
            } else {
                false
            } {
                f25(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f20(v3.clone(), 1u64)?, {
                    let a3083 = v13;
                    let a3084 = 0u64;
                    if nat_eq(a3083, a3084) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h3085, _, _, _, _, _, _, _, _, _) => {
                        h3085
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f24(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h3086, _, _, _, _, _, _, _, _, _) => {
                            h3086
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3087, _, _, _, _, _, _, _, _) => {
                            h3087
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h3088, _, _, _, _, _, _, _) => {
                            h3088
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h3089, _, _, _, _, _) => {
                            h3089
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3090, _, _, _, _) => {
                            h3090
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h3091, _, _, _) => {
                            h3091
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f106(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3092 = v2;
    match m3092 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3093, _, _, _, _, _, _, _, _, _) => {
                    h3093
                }
            }, match v0.clone() {
                Adt1::C0(h3094, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3094
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3095, _, _, _, _, _, _, _, _) => {
                    h3095
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3096, _, _, _, _, _) => {
                    h3096
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3097, _, _, _, _, _, _, _, _) => {
                    h3097
                }
            }, match v0.clone() {
                Adt1::C0(_, h3098, _, _, _, _, _, _, _, _, _, _) => {
                    h3098
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3099, _, _, _, _, _, _, _) => {
                    h3099
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3100, _, _, _, _) => {
                    h3100
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3101, _, _, _, _, _, _, _) => {
                    h3101
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3102, _, _, _, _, _, _, _, _, _) => {
                    h3102
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3103, _, _, _, _, _, _) => {
                    h3103
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3104, _, _, _) => {
                    h3104
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3105, _, _) => {
                    h3105
                }
            };
            let v9: u64 = f85(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3106, _, _, _, _, _, _, _) => {
                    h3106
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3107, _, _, _, _, _, _, _, _, _, _) => {
                    h3107
                }
            };
            let v11: u64 = {
                let a3115 = v8;
                let a3116 = {
                    let a3113 = match v5.clone() {
                        Adt6::C0(h3108, _, _, _) => {
                            h3108
                        }
                    };
                    let a3114 = {
                        let a3111 = match v6.clone() {
                            Adt6::C0(h3109, _, _, _) => {
                                h3109
                            }
                        };
                        let a3112 = match v7.clone() {
                            Adt6::C0(h3110, _, _, _) => {
                                h3110
                            }
                        };
                        nat_mul(a3111, a3112)?
                    };
                    nat_mul(a3113, a3114)?
                };
                nat_mul(a3115, a3116)?
            };
            let a3118 = match v4.clone() {
                Adt2::C0(h3117, _, _, _, _, _, _, _, _, _, _) => {
                    h3117
                }
            };
            let a3119 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3118, a3119) {
                let a3120 = v10;
                let a3121 = 0u64;
                if nat_eq(a3120, a3121) {
                    true
                } else {
                    let a3122 = v10;
                    let a3123 = 1u64;
                    nat_eq(a3122, a3123)
                }
            } else {
                false
            } {
                let a3124 = 0u64;
                let a3125 = v8;
                nat_lt(a3124, a3125)
            } else {
                false
            } {
                let a3126 = v8;
                let a3127 = 2147483648u64;
                nat_lt(a3126, a3127)
            } else {
                false
            } {
                let a3129 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3128, _, _, _, _, _) => {
                        h3128
                    }
                };
                let a3130 = 4u64;
                nat_eq(a3129, a3130)
            } else {
                false
            } {
                let a3131 = 0u64;
                let a3132 = v9;
                nat_lt(a3131, a3132)
            } else {
                false
            } {
                let a3134 = 0u64;
                let a3135 = match v3.clone() {
                    Adt2::C0(_, _, h3133, _, _, _, _, _, _, _, _) => {
                        h3133
                    }
                };
                nat_lt(a3134, a3135)
            } else {
                false
            } {
                let a3137 = 0u64;
                let a3138 = match v3.clone() {
                    Adt2::C0(_, _, _, h3136, _, _, _, _, _, _, _) => {
                        h3136
                    }
                };
                nat_lt(a3137, a3138)
            } else {
                false
            } {
                let a3140 = match v3.clone() {
                    Adt2::C0(_, h3139, _, _, _, _, _, _, _, _, _) => {
                        h3139
                    }
                };
                let a3141 = v11;
                nat_eq(a3140, a3141)
            } else {
                false
            } {
                let a3144 = match v3.clone() {
                    Adt2::C0(_, _, h3142, _, _, _, _, _, _, _, _) => {
                        h3142
                    }
                };
                let a3145 = f85(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3143, _, _, _, _, _, _, _, _, _) => {
                        h3143
                    }
                })?;
                nat_eq(a3144, a3145)
            } else {
                false
            } {
                let a3148 = match v3.clone() {
                    Adt2::C0(_, _, _, h3146, _, _, _, _, _, _, _) => {
                        h3146
                    }
                };
                let a3149 = f85(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3147, _, _, _, _, _, _, _, _) => {
                        h3147
                    }
                })?;
                nat_eq(a3148, a3149)
            } else {
                false
            } {
                let a3157 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3150, _, _, _, _, _, _) => {
                        h3150
                    }
                };
                let a3158 = {
                    let a3155 = v9;
                    let a3156 = {
                        let a3152 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3151, _, _, _, _, _, _) => {
                                h3151
                            }
                        };
                        let a3153 = v8;
                        let a3154 = 0u64;
                        nat_quot(a3152, a3153, a3154)
                    };
                    nat_mul(a3155, a3156)?
                };
                nat_eq(a3157, a3158)
            } else {
                false
            } {
                f25(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f20(v3.clone(), 1u64)?, {
                    let a3159 = v10;
                    let a3160 = 0u64;
                    if nat_eq(a3159, a3160) {
                        4u64
                    } else {
                        2u64
                    }
                }, f24(v4.clone(), 1u64), f24(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3161, _, _) => {
                        h3161
                    }
                }, {
                    let a3163 = v8;
                    let a3164 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3162, _, _) => {
                            h3162
                        }
                    };
                    nat_mul(a3163, a3164)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f107(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3165 = v1;
    match m3165 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f207(v0, v3, v2)
        }
    }
}

pub fn f108(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3167 = match v0.clone() {
        Adt2::C0(h3166, _, _, _, _, _, _, _, _, _, _) => {
            h3166
        }
    };
    let a3168 = 1u64;
    if if nat_eq(a3167, a3168) {
        true
    } else {
        let a3170 = match v0.clone() {
            Adt2::C0(h3169, _, _, _, _, _, _, _, _, _, _) => {
                h3169
            }
        };
        let a3171 = 30u64;
        nat_eq(a3170, a3171)
    } {
        let a3173 = match v1.clone() {
            Adt2::C0(h3172, _, _, _, _, _, _, _, _, _, _) => {
                h3172
            }
        };
        let a3174 = 1u64;
        if if if if if if if if if if if nat_eq(a3173, a3174) {
            let a3177 = match v0.clone() {
                Adt2::C0(_, h3175, _, _, _, _, _, _, _, _, _) => {
                    h3175
                }
            };
            let a3178 = match v1.clone() {
                Adt2::C0(_, h3176, _, _, _, _, _, _, _, _, _) => {
                    h3176
                }
            };
            nat_eq(a3177, a3178)
        } else {
            false
        } {
            let a3180 = 0u64;
            let a3181 = match v1.clone() {
                Adt2::C0(_, _, h3179, _, _, _, _, _, _, _, _) => {
                    h3179
                }
            };
            nat_lt(a3180, a3181)
        } else {
            false
        } {
            let a3184 = match v1.clone() {
                Adt2::C0(_, _, h3182, _, _, _, _, _, _, _, _) => {
                    h3182
                }
            };
            let a3185 = match v1.clone() {
                Adt2::C0(_, _, _, h3183, _, _, _, _, _, _, _) => {
                    h3183
                }
            };
            nat_eq(a3184, a3185)
        } else {
            false
        } {
            let a3187 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3186, _, _, _, _, _, _) => {
                    h3186
                }
            };
            let a3188 = 1u64;
            nat_eq(a3187, a3188)
        } else {
            false
        } {
            let a3196 = match v0.clone() {
                Adt2::C0(_, _, h3189, _, _, _, _, _, _, _, _) => {
                    h3189
                }
            };
            let a3197 = {
                let a3194 = {
                    let a3192 = match v1.clone() {
                        Adt2::C0(_, _, h3190, _, _, _, _, _, _, _, _) => {
                            h3190
                        }
                    };
                    let a3193 = match v1.clone() {
                        Adt2::C0(_, _, h3191, _, _, _, _, _, _, _, _) => {
                            h3191
                        }
                    };
                    nat_add(a3192, a3193)?
                };
                let a3195 = 1u64;
                nat_sub(a3194, a3195)
            };
            nat_eq(a3196, a3197)
        } else {
            false
        } {
            let a3199 = 0u64;
            let a3200 = match v0.clone() {
                Adt2::C0(_, _, _, h3198, _, _, _, _, _, _, _) => {
                    h3198
                }
            };
            nat_lt(a3199, a3200)
        } else {
            false
        } {
            let a3202 = 0u64;
            let a3203 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3201, _, _, _, _, _, _) => {
                    h3201
                }
            };
            nat_lt(a3202, a3203)
        } else {
            false
        } {
            let a3205 = 2u64;
            let a3206 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3204, _, _, _, _, _) => {
                    h3204
                }
            };
            nat_le(a3205, a3206)
        } else {
            false
        } {
            let a3208 = 2u64;
            let a3209 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3207, _, _, _, _, _) => {
                    h3207
                }
            };
            nat_le(a3208, a3209)
        } else {
            false
        } {
            f25(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f20(v1.clone(), 1u64)?, 2u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f109(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3210 = v1;
    match m3210 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f208(v0, v3, v2)
        }
    }
}

pub fn f110(v0: u64) -> bool {
    let a3211 = v0;
    let a3212 = 1u64;
    if if nat_eq(a3211, a3212) {
        true
    } else {
        let a3213 = v0;
        let a3214 = 33u64;
        nat_eq(a3213, a3214)
    } {
        true
    } else {
        let a3215 = v0;
        let a3216 = 34u64;
        nat_eq(a3215, a3216)
    }
}

pub fn f111(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f20(v0.clone(), 1u64)?;
    if if if if if f43(v1, v2.clone()) {
        let a3217 = v3;
        let a3218 = f20(v2.clone(), 1u64)?;
        nat_eq(a3217, a3218)
    } else {
        false
    } {
        f19(v0.clone(), v3)?
    } else {
        false
    } {
        f19(v2.clone(), v3)?
    } else {
        false
    } {
        let a3219 = v3;
        let a3220 = 0u64;
        if nat_eq(a3219, a3220) {
            true
        } else {
            let a3222 = 4u64;
            let a3223 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3221, _, _, _, _, _) => {
                    h3221
                }
            };
            nat_le(a3222, a3223)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3224 = 2u64;
            let a3225 = v3;
            nat_mul(a3224, a3225)?
        }, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: u64) -> bool {
    if f209(v0) {
        !f210(v0)
    } else {
        false
    }
}

pub fn f113(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3226 = v1;
    match m3226 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3228 = f202(match v0.clone() {
                Adt2::C0(h3227, _, _, _, _, _, _, _, _, _, _) => {
                    h3227
                }
            });
            match m3228 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3229, _) => {
                            h3229
                        }
                    };
                    if if if if if if f43(v3, v2.clone()) {
                        f25(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3230 = f20(v0.clone(), 1u64)?;
                        let a3231 = f20(v2.clone(), 1u64)?;
                        nat_eq(a3230, a3231)
                    } else {
                        false
                    } {
                        let a3236 = {
                            let a3233 = match v0.clone() {
                                Adt2::C0(_, h3232, _, _, _, _, _, _, _, _, _) => {
                                    h3232
                                }
                            };
                            let a3234 = v5;
                            let a3235 = 1u64;
                            nat_rem(a3233, a3234, a3235)
                        };
                        let a3237 = 0u64;
                        if nat_eq(a3236, a3237) {
                            let a3240 = match v4 {
                                Adt19::C0(_, _, h3238) => {
                                    h3238
                                }
                            };
                            let a3241 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3239, _, _, _, _, _) => {
                                    h3239
                                }
                            };
                            nat_le(a3240, a3241)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3242 = v5;
                        let a3243 = 1u64;
                        if nat_eq(a3242, a3243) {
                            let a3245 = 4u64;
                            let a3246 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3244, _, _, _, _, _) => {
                                    h3244
                                }
                            };
                            nat_le(a3245, a3246)
                        } else {
                            let a3248 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3247, _, _, _, _, _) => {
                                    h3247
                                }
                            };
                            let a3249 = 4u64;
                            nat_eq(a3248, a3249)
                        }
                    } else {
                        false
                    } {
                        let a3254 = {
                            let a3251 = match v2.clone() {
                                Adt2::C0(_, h3250, _, _, _, _, _, _, _, _, _) => {
                                    h3250
                                }
                            };
                            let a3252 = v5;
                            let a3253 = 1u64;
                            nat_rem(a3251, a3252, a3253)
                        };
                        let a3255 = 0u64;
                        if nat_eq(a3254, a3255) {
                            true
                        } else {
                            f34(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f20(v0.clone(), v5)?, 4u64, f24(v0.clone(), v5), f24(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3256, _, _, _, _, _, _, _, _, _, _) => {
                                h3256
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

pub fn f114(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3259 = match v1.clone() {
        Adt2::C0(h3257, _, _, _, _, _, _, _, _, _, _) => {
            h3257
        }
    };
    let a3260 = match v3.clone() {
        Adt2::C0(h3258, _, _, _, _, _, _, _, _, _, _) => {
            h3258
        }
    };
    if nat_eq(a3259, a3260) {
        f211(v0, v1, v2, v3)
    } else {
        if if if if f210(match v1.clone() {
            Adt2::C0(h3261, _, _, _, _, _, _, _, _, _, _) => {
                h3261
            }
        }) {
            f210(match v3.clone() {
                Adt2::C0(h3262, _, _, _, _, _, _, _, _, _, _) => {
                    h3262
                }
            })
        } else {
            false
        } {
            let a3263 = v0;
            let a3264 = 33u64;
            if nat_eq(a3263, a3264) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3265 = f20(v1.clone(), 1u64)?;
            let a3266 = f20(v3.clone(), 1u64)?;
            nat_eq(a3265, a3266)
        } else {
            false
        } {
            let m3268 = f202(match v3.clone() {
                Adt2::C0(h3267, _, _, _, _, _, _, _, _, _, _) => {
                    h3267
                }
            });
            match m3268 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3269, _, _, _, _, _, _, _, _, _, _) => {
                            h3269
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3270, _, _, _, _, _, _, _, _, _, _) => {
                            h3270
                        }
                    };
                    let v7: bool = {
                        let a3271 = v5;
                        let a3272 = 43u64;
                        if nat_eq(a3271, a3272) {
                            true
                        } else {
                            let a3273 = v6;
                            let a3274 = 43u64;
                            nat_eq(a3273, a3274)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3275 = f20(v3.clone(), 1u64)?;
                        let a3276 = if v7 {
                            f155(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3275, a3276)?
                    }, match v4 {
                        Adt19::C0(_, _, h3277) => {
                            h3277
                        }
                    }, f24(v1, 1u64), f24(v3, 1u64), if v7 {
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

pub fn f115(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3279 = match v0.clone() {
        Adt2::C0(h3278, _, _, _, _, _, _, _, _, _, _) => {
            h3278
        }
    };
    let a3280 = 0u64;
    if if if if nat_eq(a3279, a3280) {
        true
    } else {
        let a3282 = match v0.clone() {
            Adt2::C0(h3281, _, _, _, _, _, _, _, _, _, _) => {
                h3281
            }
        };
        let a3283 = 1u64;
        nat_eq(a3282, a3283)
    } {
        true
    } else {
        let a3285 = match v0.clone() {
            Adt2::C0(h3284, _, _, _, _, _, _, _, _, _, _) => {
                h3284
            }
        };
        let a3286 = 25u64;
        if nat_eq(a3285, a3286) {
            true
        } else {
            let a3288 = match v0.clone() {
                Adt2::C0(h3287, _, _, _, _, _, _, _, _, _, _) => {
                    h3287
                }
            };
            let a3289 = 26u64;
            nat_eq(a3288, a3289)
        }
    } {
        true
    } else {
        let a3291 = match v0.clone() {
            Adt2::C0(h3290, _, _, _, _, _, _, _, _, _, _) => {
                h3290
            }
        };
        let a3292 = 30u64;
        nat_eq(a3291, a3292)
    } {
        let a3295 = match v0.clone() {
            Adt2::C0(h3293, _, _, _, _, _, _, _, _, _, _) => {
                h3293
            }
        };
        let a3296 = match v1.clone() {
            Adt2::C0(h3294, _, _, _, _, _, _, _, _, _, _) => {
                h3294
            }
        };
        if if nat_eq(a3295, a3296) {
            f212(match v1.clone() {
                Adt2::C0(_, h3297, _, _, _, _, _, _, _, _, _) => {
                    h3297
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3298, _, _, _, _, _, _, _, _) => {
                    h3298
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3299, _, _, _, _, _, _, _) => {
                    h3299
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3300, _, _, _, _, _, _) => {
                    h3300
                }
            }, match v0.clone() {
                Adt2::C0(_, h3301, _, _, _, _, _, _, _, _, _) => {
                    h3301
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3302, _, _, _, _, _, _, _, _) => {
                    h3302
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3303, _, _, _, _, _, _, _) => {
                    h3303
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3304, _, _, _, _, _, _) => {
                    h3304
                }
            })
        } else {
            false
        } {
            let m3306 = f202(match v0.clone() {
                Adt2::C0(h3305, _, _, _, _, _, _, _, _, _, _) => {
                    h3305
                }
            });
            match m3306 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f20(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3307) => {
                            h3307
                        }
                    }, f24(v0, 1u64), f24(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f116(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3308 = v2;
    match m3308 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3309 = v0;
            let a3310 = 4u64;
            if if if nat_lt(a3309, a3310) {
                let a3313 = match v1.clone() {
                    Adt2::C0(h3311, _, _, _, _, _, _, _, _, _, _) => {
                        h3311
                    }
                };
                let a3314 = match v4.clone() {
                    Adt2::C0(h3312, _, _, _, _, _, _, _, _, _, _) => {
                        h3312
                    }
                };
                if nat_eq(a3313, a3314) {
                    let a3317 = match v1.clone() {
                        Adt2::C0(h3315, _, _, _, _, _, _, _, _, _, _) => {
                            h3315
                        }
                    };
                    let a3318 = match v3.clone() {
                        Adt2::C0(h3316, _, _, _, _, _, _, _, _, _, _) => {
                            h3316
                        }
                    };
                    nat_eq(a3317, a3318)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f213(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3319, _, _, _, _, _, _, _, _, _) => {
                        h3319
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3320, _, _, _, _, _, _, _, _, _) => {
                        h3320
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3321, _, _, _, _, _, _, _, _, _) => {
                        h3321
                    }
                })? {
                    f213(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3322, _, _, _, _, _, _, _, _) => {
                            h3322
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3323, _, _, _, _, _, _, _, _) => {
                            h3323
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3324, _, _, _, _, _, _, _, _) => {
                            h3324
                        }
                    })?
                } else {
                    false
                } {
                    f213(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3325, _, _, _, _, _, _, _) => {
                            h3325
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3326, _, _, _, _, _, _, _) => {
                            h3326
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3327, _, _, _, _, _, _, _) => {
                            h3327
                        }
                    })?
                } else {
                    false
                } {
                    f213(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3328, _, _, _, _, _, _) => {
                            h3328
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3329, _, _, _, _, _, _) => {
                            h3329
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3330, _, _, _, _, _, _) => {
                            h3330
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3332 = f202(match v1.clone() {
                    Adt2::C0(h3331, _, _, _, _, _, _, _, _, _, _) => {
                        h3331
                    }
                });
                match m3332 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3333, _) => {
                                h3333
                            }
                        };
                        let a3338 = {
                            let a3335 = match v1.clone() {
                                Adt2::C0(_, h3334, _, _, _, _, _, _, _, _, _) => {
                                    h3334
                                }
                            };
                            let a3336 = v6;
                            let a3337 = 1u64;
                            nat_rem(a3335, a3336, a3337)
                        };
                        let a3339 = 0u64;
                        if if if nat_eq(a3338, a3339) {
                            let a3344 = {
                                let a3341 = match v4.clone() {
                                    Adt2::C0(_, h3340, _, _, _, _, _, _, _, _, _) => {
                                        h3340
                                    }
                                };
                                let a3342 = v6;
                                let a3343 = 1u64;
                                nat_rem(a3341, a3342, a3343)
                            };
                            let a3345 = 0u64;
                            nat_eq(a3344, a3345)
                        } else {
                            false
                        } {
                            let a3350 = {
                                let a3347 = match v3.clone() {
                                    Adt2::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                                        h3346
                                    }
                                };
                                let a3348 = v6;
                                let a3349 = 1u64;
                                nat_rem(a3347, a3348, a3349)
                            };
                            let a3351 = 0u64;
                            nat_eq(a3350, a3351)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f20(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3352) => {
                                    h3352
                                }
                            }, f24(v1, v6), f24(v3, v6), Adt4::C2(v0, f24(v4, v6))))
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

pub fn f117(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3354 = match v2.clone() {
        Adt2::C0(h3353, _, _, _, _, _, _, _, _, _, _) => {
            h3353
        }
    };
    let a3355 = 0u64;
    if if nat_eq(a3354, a3355) {
        let a3357 = match v3.clone() {
            Adt2::C0(h3356, _, _, _, _, _, _, _, _, _, _) => {
                h3356
            }
        };
        let a3358 = 0u64;
        nat_eq(a3357, a3358)
    } else {
        false
    } {
        let a3359 = v0;
        let a3360 = 61u64;
        if nat_eq(a3359, a3360) {
            let v4: bool = {
                let a3362 = 0u64;
                let a3363 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3361, _, _, _) => {
                        h3361
                    }
                };
                nat_lt(a3362, a3363)
            };
            if if if if if if f214(match v2.clone() {
                Adt2::C0(_, h3364, _, _, _, _, _, _, _, _, _) => {
                    h3364
                }
            }, match v1.clone() {
                Adt1::C0(h3365, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3365
                }
            }, match v1.clone() {
                Adt1::C0(_, h3366, _, _, _, _, _, _, _, _, _, _) => {
                    h3366
                }
            }, match v3.clone() {
                Adt2::C0(_, h3367, _, _, _, _, _, _, _, _, _) => {
                    h3367
                }
            })? {
                f214(match v2.clone() {
                    Adt2::C0(_, _, h3368, _, _, _, _, _, _, _, _) => {
                        h3368
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3369, _, _, _, _, _, _, _, _, _) => {
                        h3369
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3370, _, _, _, _, _, _, _, _) => {
                        h3370
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3371, _, _, _, _, _, _, _, _) => {
                        h3371
                    }
                })?
            } else {
                false
            } {
                f214(match v2.clone() {
                    Adt2::C0(_, _, _, h3372, _, _, _, _, _, _, _) => {
                        h3372
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3373, _, _, _, _, _, _, _) => {
                        h3373
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3374, _, _, _, _, _, _) => {
                        h3374
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3375, _, _, _, _, _, _, _) => {
                        h3375
                    }
                })?
            } else {
                false
            } {
                f214(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3376, _, _, _, _, _, _) => {
                        h3376
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3377, _, _, _, _, _) => {
                        h3377
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3378, _, _, _, _) => {
                        h3378
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3379, _, _, _, _, _, _) => {
                        h3379
                    }
                })?
            } else {
                false
            } {
                let a3381 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3380, _, _, _) => {
                        h3380
                    }
                };
                let a3382 = 4294967295u64;
                nat_le(a3381, a3382)
            } else {
                false
            } {
                if v4 {
                    !f44(match v2.clone() {
                        Adt2::C0(_, h3383, _, _, _, _, _, _, _, _, _) => {
                            h3383
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3384, _, _, _, _, _, _, _, _) => {
                            h3384
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3385, _, _, _, _, _, _, _) => {
                            h3385
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3386, _, _, _, _, _, _) => {
                            h3386
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f20(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3387, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3387
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3388, _, _, _, _, _, _, _, _, _) => {
                        h3388
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3389, _, _, _, _, _, _, _) => {
                        h3389
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3390, _, _, _, _, _) => {
                        h3390
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3391 = v0;
            let a3392 = 62u64;
            if nat_eq(a3391, a3392) {
                let a3394 = match v1.clone() {
                    Adt1::C0(h3393, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3393
                    }
                };
                let a3395 = 2147483648u64;
                if if if if if nat_lt(a3394, a3395) {
                    let a3397 = match v1.clone() {
                        Adt1::C0(_, h3396, _, _, _, _, _, _, _, _, _, _) => {
                            h3396
                        }
                    };
                    let a3398 = 2147483648u64;
                    nat_lt(a3397, a3398)
                } else {
                    false
                } {
                    let a3401 = match v1.clone() {
                        Adt1::C0(h3399, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3399
                        }
                    };
                    let a3402 = match v2.clone() {
                        Adt2::C0(_, h3400, _, _, _, _, _, _, _, _, _) => {
                            h3400
                        }
                    };
                    nat_lt(a3401, a3402)
                } else {
                    false
                } {
                    let a3405 = match v1.clone() {
                        Adt1::C0(_, h3403, _, _, _, _, _, _, _, _, _, _) => {
                            h3403
                        }
                    };
                    let a3406 = match v2.clone() {
                        Adt2::C0(_, h3404, _, _, _, _, _, _, _, _, _) => {
                            h3404
                        }
                    };
                    nat_lt(a3405, a3406)
                } else {
                    false
                } {
                    let a3409 = match v2.clone() {
                        Adt2::C0(_, _, h3407, _, _, _, _, _, _, _, _) => {
                            h3407
                        }
                    };
                    let a3410 = match v3.clone() {
                        Adt2::C0(_, _, h3408, _, _, _, _, _, _, _, _) => {
                            h3408
                        }
                    };
                    if if if nat_eq(a3409, a3410) {
                        let a3413 = match v2.clone() {
                            Adt2::C0(_, _, _, h3411, _, _, _, _, _, _, _) => {
                                h3411
                            }
                        };
                        let a3414 = match v3.clone() {
                            Adt2::C0(_, _, _, h3412, _, _, _, _, _, _, _) => {
                                h3412
                            }
                        };
                        nat_eq(a3413, a3414)
                    } else {
                        false
                    } {
                        let a3417 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3415, _, _, _, _, _, _) => {
                                h3415
                            }
                        };
                        let a3418 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3416, _, _, _, _, _, _) => {
                                h3416
                            }
                        };
                        nat_eq(a3417, a3418)
                    } else {
                        false
                    } {
                        let a3427 = match v3.clone() {
                            Adt2::C0(_, h3419, _, _, _, _, _, _, _, _, _) => {
                                h3419
                            }
                        };
                        let a3428 = {
                            let a3425 = {
                                let a3422 = match v2.clone() {
                                    Adt2::C0(_, h3420, _, _, _, _, _, _, _, _, _) => {
                                        h3420
                                    }
                                };
                                let a3423 = match v1.clone() {
                                    Adt1::C0(h3421, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3421
                                    }
                                };
                                nat_add(a3422, a3423)?
                            };
                            let a3426 = match v1.clone() {
                                Adt1::C0(_, h3424, _, _, _, _, _, _, _, _, _, _) => {
                                    h3424
                                }
                            };
                            nat_add(a3425, a3426)?
                        };
                        nat_eq(a3427, a3428)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f20(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3429, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3429
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3430 = v0;
                let a3431 = 63u64;
                if nat_eq(a3430, a3431) {
                    if if if if if f215(match v2.clone() {
                        Adt2::C0(_, h3432, _, _, _, _, _, _, _, _, _) => {
                            h3432
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3433, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3433
                        }
                    })? {
                        f215(match v2.clone() {
                            Adt2::C0(_, _, h3434, _, _, _, _, _, _, _, _) => {
                                h3434
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3435, _, _, _, _, _, _, _, _, _, _) => {
                                h3435
                            }
                        })?
                    } else {
                        false
                    } {
                        f215(match v2.clone() {
                            Adt2::C0(_, _, _, h3436, _, _, _, _, _, _, _) => {
                                h3436
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3437, _, _, _, _, _, _, _, _, _) => {
                                h3437
                            }
                        })?
                    } else {
                        false
                    } {
                        f215(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3438, _, _, _, _, _, _) => {
                                h3438
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3439, _, _, _, _, _, _, _, _) => {
                                h3439
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3442 = match v2.clone() {
                            Adt2::C0(_, h3440, _, _, _, _, _, _, _, _, _) => {
                                h3440
                            }
                        };
                        let a3443 = match v3.clone() {
                            Adt2::C0(_, h3441, _, _, _, _, _, _, _, _, _) => {
                                h3441
                            }
                        };
                        if if if nat_eq(a3442, a3443) {
                            let a3446 = match v2.clone() {
                                Adt2::C0(_, _, h3444, _, _, _, _, _, _, _, _) => {
                                    h3444
                                }
                            };
                            let a3447 = match v3.clone() {
                                Adt2::C0(_, _, h3445, _, _, _, _, _, _, _, _) => {
                                    h3445
                                }
                            };
                            nat_eq(a3446, a3447)
                        } else {
                            false
                        } {
                            let a3450 = match v2.clone() {
                                Adt2::C0(_, _, _, h3448, _, _, _, _, _, _, _) => {
                                    h3448
                                }
                            };
                            let a3451 = match v3.clone() {
                                Adt2::C0(_, _, _, h3449, _, _, _, _, _, _, _) => {
                                    h3449
                                }
                            };
                            nat_eq(a3450, a3451)
                        } else {
                            false
                        } {
                            let a3454 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3452, _, _, _, _, _, _) => {
                                    h3452
                                }
                            };
                            let a3455 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3453, _, _, _, _, _, _) => {
                                    h3453
                                }
                            };
                            nat_eq(a3454, a3455)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f20(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3456, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3456
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3457, _, _, _, _, _, _, _, _, _, _) => {
                                h3457
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3458, _, _, _, _, _, _, _, _, _) => {
                                h3458
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3459, _, _, _, _, _, _, _, _) => {
                                h3459
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

pub fn f118(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3460 = v3;
    match m3460 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3461 = v0;
                let a3462 = 4u64;
                nat_eq(a3461, a3462)
            };
            let v7: bool = {
                let a3464 = 0u64;
                let a3465 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3463, _, _, _, _, _, _, _) => {
                        h3463
                    }
                };
                nat_lt(a3464, a3465)
            };
            let v8: u64 = f20(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3466, _, _, _, _, _, _, _, _, _) => {
                    h3466
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3467, _, _, _, _, _, _, _, _) => {
                    h3467
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3468, _, _, _, _, _, _, _) => {
                    h3468
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3469, _, _, _, _, _, _) => {
                    h3469
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3470, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3470
                }
            }, match v1.clone() {
                Adt1::C0(_, h3471, _, _, _, _, _, _, _, _, _, _) => {
                    h3471
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3472, _, _, _, _, _, _, _, _, _) => {
                    h3472
                }
            });
            if if if if if if if if if if if if v6 {
                let a3474 = match v2.clone() {
                    Adt2::C0(h3473, _, _, _, _, _, _, _, _, _, _) => {
                        h3473
                    }
                };
                let a3475 = 0u64;
                if nat_eq(a3474, a3475) {
                    let a3477 = match v5.clone() {
                        Adt2::C0(h3476, _, _, _, _, _, _, _, _, _, _) => {
                            h3476
                        }
                    };
                    let a3478 = 0u64;
                    nat_eq(a3477, a3478)
                } else {
                    false
                }
            } else {
                let a3480 = match v2.clone() {
                    Adt2::C0(h3479, _, _, _, _, _, _, _, _, _, _) => {
                        h3479
                    }
                };
                let a3481 = 0u64;
                if if nat_eq(a3480, a3481) {
                    true
                } else {
                    let a3483 = match v2.clone() {
                        Adt2::C0(h3482, _, _, _, _, _, _, _, _, _, _) => {
                            h3482
                        }
                    };
                    let a3484 = 26u64;
                    nat_eq(a3483, a3484)
                } {
                    let a3486 = match v5.clone() {
                        Adt2::C0(h3485, _, _, _, _, _, _, _, _, _, _) => {
                            h3485
                        }
                    };
                    let a3487 = 0u64;
                    if nat_eq(a3486, a3487) {
                        true
                    } else {
                        let a3489 = match v5.clone() {
                            Adt2::C0(h3488, _, _, _, _, _, _, _, _, _, _) => {
                                h3488
                            }
                        };
                        let a3490 = 26u64;
                        nat_eq(a3489, a3490)
                    }
                } else {
                    false
                }
            } {
                let a3493 = match v2.clone() {
                    Adt2::C0(h3491, _, _, _, _, _, _, _, _, _, _) => {
                        h3491
                    }
                };
                let a3494 = match v4.clone() {
                    Adt2::C0(h3492, _, _, _, _, _, _, _, _, _, _) => {
                        h3492
                    }
                };
                nat_eq(a3493, a3494)
            } else {
                false
            } {
                f28(v2.clone(), v4.clone())
            } else {
                false
            } {
                f34(v2.clone())?
            } else {
                false
            } {
                f34(v4.clone())?
            } else {
                false
            } {
                let a3496 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3495, _, _, _, _, _) => {
                        h3495
                    }
                };
                let a3497 = 4u64;
                nat_eq(a3496, a3497)
            } else {
                false
            } {
                let a3498 = v8;
                let a3499 = f20(v2.clone(), 1u64)?;
                nat_le(a3498, a3499)
            } else {
                false
            } {
                let a3501 = match v1.clone() {
                    Adt1::C0(_, _, _, h3500, _, _, _, _, _, _, _, _) => {
                        h3500
                    }
                };
                let a3502 = 1073741824u64;
                nat_lt(a3501, a3502)
            } else {
                false
            } {
                let a3504 = match v1.clone() {
                    Adt1::C0(h3503, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3503
                    }
                };
                let a3505 = 4294967295u64;
                if if if if nat_le(a3504, a3505) {
                    let a3507 = match v1.clone() {
                        Adt1::C0(_, h3506, _, _, _, _, _, _, _, _, _, _) => {
                            h3506
                        }
                    };
                    let a3508 = 4294967295u64;
                    nat_le(a3507, a3508)
                } else {
                    false
                } {
                    let a3510 = match v1.clone() {
                        Adt1::C0(_, _, h3509, _, _, _, _, _, _, _, _, _) => {
                            h3509
                        }
                    };
                    let a3511 = 4294967295u64;
                    nat_le(a3510, a3511)
                } else {
                    false
                } {
                    let a3513 = match v1.clone() {
                        Adt1::C0(_, _, _, h3512, _, _, _, _, _, _, _, _) => {
                            h3512
                        }
                    };
                    let a3514 = 4294967295u64;
                    nat_le(a3513, a3514)
                } else {
                    false
                } {
                    let a3516 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3515, _, _, _, _, _, _, _) => {
                            h3515
                        }
                    };
                    let a3517 = 4294967295u64;
                    nat_le(a3516, a3517)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a3520 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3518) => {
                            h3518
                        }
                    };
                    let a3521 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3519) => {
                            h3519
                        }
                    };
                    nat_eq(a3520, a3521)
                } else {
                    true
                }
            } else {
                false
            } {
                f216(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3522, _, _, _, _, _, _, _, _) => {
                        h3522
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3523, _) => {
                        h3523
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f20(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a3524 = v10;
                    let a3525 = v8;
                    nat_add(a3524, a3525)?
                }, 4u64, f24(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3526, _, _, _, _, _, _, _, _) => {
                        h3526
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f119(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3528 = match v0.clone() {
        Adt2::C0(h3527, _, _, _, _, _, _, _, _, _, _) => {
            h3527
        }
    };
    let a3529 = 0u64;
    if if if nat_eq(a3528, a3529) {
        let a3531 = match v1.clone() {
            Adt2::C0(h3530, _, _, _, _, _, _, _, _, _, _) => {
                h3530
            }
        };
        let a3532 = 0u64;
        nat_eq(a3531, a3532)
    } else {
        false
    } {
        f212(match v0.clone() {
            Adt2::C0(_, h3533, _, _, _, _, _, _, _, _, _) => {
                h3533
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3534, _, _, _, _, _, _, _, _) => {
                h3534
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3535, _, _, _, _, _, _, _) => {
                h3535
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3536, _, _, _, _, _, _) => {
                h3536
            }
        }, match v1.clone() {
            Adt2::C0(_, h3537, _, _, _, _, _, _, _, _, _) => {
                h3537
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3538, _, _, _, _, _, _, _, _) => {
                h3538
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3539, _, _, _, _, _, _, _) => {
                h3539
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3540, _, _, _, _, _, _) => {
                h3540
            }
        })
    } else {
        false
    } {
        let v2: u64 = f20(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3541 = v2;
            let a3542 = f20(v0.clone(), 1u64)?;
            nat_add(a3541, a3542)?
        }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f120(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3544 = match v1.clone() {
        Adt2::C0(h3543, _, _, _, _, _, _, _, _, _, _) => {
            h3543
        }
    };
    let a3545 = 0u64;
    if if if nat_eq(a3544, a3545) {
        true
    } else {
        let a3547 = match v1.clone() {
            Adt2::C0(h3546, _, _, _, _, _, _, _, _, _, _) => {
                h3546
            }
        };
        let a3548 = 1u64;
        nat_eq(a3547, a3548)
    } {
        true
    } else {
        let a3550 = match v1.clone() {
            Adt2::C0(h3549, _, _, _, _, _, _, _, _, _, _) => {
                h3549
            }
        };
        let a3551 = 30u64;
        nat_eq(a3550, a3551)
    } {
        let a3554 = match v1.clone() {
            Adt2::C0(h3552, _, _, _, _, _, _, _, _, _, _) => {
                h3552
            }
        };
        let a3555 = match v2.clone() {
            Adt2::C0(h3553, _, _, _, _, _, _, _, _, _, _) => {
                h3553
            }
        };
        if if if if if nat_eq(a3554, a3555) {
            let a3558 = match v1.clone() {
                Adt2::C0(_, h3556, _, _, _, _, _, _, _, _, _) => {
                    h3556
                }
            };
            let a3559 = match v2.clone() {
                Adt2::C0(_, h3557, _, _, _, _, _, _, _, _, _) => {
                    h3557
                }
            };
            nat_eq(a3558, a3559)
        } else {
            false
        } {
            let a3562 = match v1.clone() {
                Adt2::C0(_, _, _, h3560, _, _, _, _, _, _, _) => {
                    h3560
                }
            };
            let a3563 = match v2.clone() {
                Adt2::C0(_, _, _, h3561, _, _, _, _, _, _, _) => {
                    h3561
                }
            };
            nat_eq(a3562, a3563)
        } else {
            false
        } {
            let a3566 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3564, _, _, _, _, _, _) => {
                    h3564
                }
            };
            let a3567 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3565, _, _, _, _, _, _) => {
                    h3565
                }
            };
            nat_eq(a3566, a3567)
        } else {
            false
        } {
            let a3568 = v0;
            let a3569 = 4u64;
            if nat_eq(a3568, a3569) {
                let a3571 = match v1.clone() {
                    Adt2::C0(_, _, h3570, _, _, _, _, _, _, _, _) => {
                        h3570
                    }
                };
                let a3572 = 1u64;
                if nat_eq(a3571, a3572) {
                    let a3575 = match v1.clone() {
                        Adt2::C0(_, h3573, _, _, _, _, _, _, _, _, _) => {
                            h3573
                        }
                    };
                    let a3576 = match v2.clone() {
                        Adt2::C0(_, _, h3574, _, _, _, _, _, _, _, _) => {
                            h3574
                        }
                    };
                    nat_eq(a3575, a3576)
                } else {
                    false
                }
            } else {
                let a3577 = v0;
                let a3578 = 4u64;
                if if nat_lt(a3577, a3578) {
                    let a3581 = match v1.clone() {
                        Adt2::C0(_, h3579, _, _, _, _, _, _, _, _, _) => {
                            h3579
                        }
                    };
                    let a3582 = match v1.clone() {
                        Adt2::C0(_, _, h3580, _, _, _, _, _, _, _, _) => {
                            h3580
                        }
                    };
                    nat_eq(a3581, a3582)
                } else {
                    false
                } {
                    let a3585 = match v1.clone() {
                        Adt2::C0(_, _, h3583, _, _, _, _, _, _, _, _) => {
                            h3583
                        }
                    };
                    let a3586 = match v2.clone() {
                        Adt2::C0(_, _, h3584, _, _, _, _, _, _, _, _) => {
                            h3584
                        }
                    };
                    nat_eq(a3585, a3586)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m3588 = f202(match v1.clone() {
                Adt2::C0(h3587, _, _, _, _, _, _, _, _, _, _) => {
                    h3587
                }
            });
            match m3588 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f20(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h3589) => {
                            h3589
                        }
                    }, f24(v1, 1u64), f24(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f121(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f217(match v1.clone() {
        Adt2::C0(h3590, _, _, _, _, _, _, _, _, _, _) => {
            h3590
        }
    }, v0) {
        let a3593 = match v1.clone() {
            Adt2::C0(h3591, _, _, _, _, _, _, _, _, _, _) => {
                h3591
            }
        };
        let a3594 = match v2.clone() {
            Adt2::C0(h3592, _, _, _, _, _, _, _, _, _, _) => {
                h3592
            }
        };
        nat_eq(a3593, a3594)
    } else {
        false
    } {
        let a3597 = match v1.clone() {
            Adt2::C0(_, h3595, _, _, _, _, _, _, _, _, _) => {
                h3595
            }
        };
        let a3598 = match v2.clone() {
            Adt2::C0(_, h3596, _, _, _, _, _, _, _, _, _) => {
                h3596
            }
        };
        if if if nat_eq(a3597, a3598) {
            let a3601 = match v1.clone() {
                Adt2::C0(_, _, h3599, _, _, _, _, _, _, _, _) => {
                    h3599
                }
            };
            let a3602 = match v2.clone() {
                Adt2::C0(_, _, h3600, _, _, _, _, _, _, _, _) => {
                    h3600
                }
            };
            nat_eq(a3601, a3602)
        } else {
            false
        } {
            let a3605 = match v1.clone() {
                Adt2::C0(_, _, _, h3603, _, _, _, _, _, _, _) => {
                    h3603
                }
            };
            let a3606 = match v2.clone() {
                Adt2::C0(_, _, _, h3604, _, _, _, _, _, _, _) => {
                    h3604
                }
            };
            nat_eq(a3605, a3606)
        } else {
            false
        } {
            let a3609 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3607, _, _, _, _, _, _) => {
                    h3607
                }
            };
            let a3610 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3608, _, _, _, _, _, _) => {
                    h3608
                }
            };
            nat_eq(a3609, a3610)
        } else {
            false
        }
    } else {
        false
    } {
        let m3612 = f202(match v1.clone() {
            Adt2::C0(h3611, _, _, _, _, _, _, _, _, _, _) => {
                h3611
            }
        });
        match m3612 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f20(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h3613) => {
                        h3613
                    }
                }, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h3614, _, _, _, _, _, _, _, _, _, _) => {
                        h3614
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f122(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h3615, _, _, _, _, _, _, _, _, _, _, _) => {
                h3615
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3616, _, _, _, _, _, _, _, _, _) => {
                h3616
            }
        }
    };
    let a3617 = 0u64;
    let a3618 = v4;
    if if nat_lt(a3617, a3618) {
        let a3619 = v4;
        let a3620 = 2147483648u64;
        nat_lt(a3619, a3620)
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
        let v7: u64 = f218(match v5.clone() {
            Adt2::C0(_, _, h3621, _, _, _, _, _, _, _, _) => {
                h3621
            }
        }, v4)?;
        let v8: u64 = f218(match v5.clone() {
            Adt2::C0(_, _, _, h3622, _, _, _, _, _, _, _) => {
                h3622
            }
        }, v4)?;
        let a3624 = match v5.clone() {
            Adt2::C0(h3623, _, _, _, _, _, _, _, _, _, _) => {
                h3623
            }
        };
        let a3625 = 0u64;
        if if if if if nat_eq(a3624, a3625) {
            let a3627 = match v6.clone() {
                Adt2::C0(h3626, _, _, _, _, _, _, _, _, _, _) => {
                    h3626
                }
            };
            let a3628 = 0u64;
            nat_eq(a3627, a3628)
        } else {
            false
        } {
            let a3630 = match v5.clone() {
                Adt2::C0(_, _, _, _, h3629, _, _, _, _, _, _) => {
                    h3629
                }
            };
            let a3631 = 1u64;
            nat_eq(a3630, a3631)
        } else {
            false
        } {
            let a3634 = match v5.clone() {
                Adt2::C0(_, h3632, _, _, _, _, _, _, _, _, _) => {
                    h3632
                }
            };
            let a3635 = match v6.clone() {
                Adt2::C0(_, h3633, _, _, _, _, _, _, _, _, _) => {
                    h3633
                }
            };
            nat_eq(a3634, a3635)
        } else {
            false
        } {
            if v0 {
                let a3637 = match v5.clone() {
                    Adt2::C0(_, _, h3636, _, _, _, _, _, _, _, _) => {
                        h3636
                    }
                };
                let a3638 = 2147483648u64;
                if if if nat_lt(a3637, a3638) {
                    let a3640 = match v5.clone() {
                        Adt2::C0(_, _, _, h3639, _, _, _, _, _, _, _) => {
                            h3639
                        }
                    };
                    let a3641 = 2147483648u64;
                    nat_lt(a3640, a3641)
                } else {
                    false
                } {
                    let a3644 = {
                        let a3642 = v7;
                        let a3643 = v8;
                        nat_mul(a3642, a3643)?
                    };
                    let a3645 = 2147483648u64;
                    nat_le(a3644, a3645)
                } else {
                    false
                } {
                    f219(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a3647 = match v1.clone() {
                    Adt1::C0(h3646, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3646
                    }
                };
                let a3648 = v7;
                if if if if nat_eq(a3647, a3648) {
                    let a3650 = match v1 {
                        Adt1::C0(_, h3649, _, _, _, _, _, _, _, _, _, _) => {
                            h3649
                        }
                    };
                    let a3651 = v8;
                    nat_eq(a3650, a3651)
                } else {
                    false
                } {
                    f220(v7, v8, v4)?
                } else {
                    false
                } {
                    let a3653 = match v6.clone() {
                        Adt2::C0(_, _, h3652, _, _, _, _, _, _, _, _) => {
                            h3652
                        }
                    };
                    let a3654 = v4;
                    if nat_eq(a3653, a3654) {
                        let a3656 = match v6.clone() {
                            Adt2::C0(_, _, _, h3655, _, _, _, _, _, _, _) => {
                                h3655
                            }
                        };
                        let a3657 = v4;
                        nat_eq(a3656, a3657)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a3661 = match v6 {
                        Adt2::C0(_, _, _, _, h3658, _, _, _, _, _, _) => {
                            h3658
                        }
                    };
                    let a3662 = {
                        let a3659 = v7;
                        let a3660 = v8;
                        nat_mul(a3659, a3660)?
                    };
                    nat_eq(a3661, a3662)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f20(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f123(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3664 = match v2.clone() {
        Adt2::C0(h3663, _, _, _, _, _, _, _, _, _, _) => {
            h3663
        }
    };
    let a3665 = 0u64;
    if if if if if nat_eq(a3664, a3665) {
        true
    } else {
        let a3667 = match v2.clone() {
            Adt2::C0(h3666, _, _, _, _, _, _, _, _, _, _) => {
                h3666
            }
        };
        let a3668 = 1u64;
        nat_eq(a3667, a3668)
    } {
        true
    } else {
        let a3670 = match v2.clone() {
            Adt2::C0(h3669, _, _, _, _, _, _, _, _, _, _) => {
                h3669
            }
        };
        let a3671 = 30u64;
        nat_eq(a3670, a3671)
    } {
        let a3674 = match v2.clone() {
            Adt2::C0(h3672, _, _, _, _, _, _, _, _, _, _) => {
                h3672
            }
        };
        let a3675 = match v3.clone() {
            Adt2::C0(h3673, _, _, _, _, _, _, _, _, _, _) => {
                h3673
            }
        };
        nat_eq(a3674, a3675)
    } else {
        false
    } {
        if if f28(v2.clone(), v3.clone()) {
            let a3677 = match v1.clone() {
                Adt1::C0(h3676, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3676
                }
            };
            let a3678 = 4294967296u64;
            nat_lt(a3677, a3678)
        } else {
            false
        } {
            let a3680 = match v1.clone() {
                Adt1::C0(_, h3679, _, _, _, _, _, _, _, _, _, _) => {
                    h3679
                }
            };
            let a3681 = 4294967296u64;
            nat_lt(a3680, a3681)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f20(v3.clone(), 1u64)?, {
            let a3683 = match v2.clone() {
                Adt2::C0(h3682, _, _, _, _, _, _, _, _, _, _) => {
                    h3682
                }
            };
            let a3684 = 0u64;
            if nat_eq(a3683, a3684) {
                4u64
            } else {
                2u64
            }
        }, f24(v2.clone(), 1u64), f24(v3, 1u64), Adt4::C13({
            let a3685 = v0;
            let a3686 = 31u64;
            if nat_eq(a3685, a3686) {
                0u64
            } else {
                let a3687 = v0;
                let a3688 = 47u64;
                if nat_eq(a3687, a3688) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h3689, _, _, _, _, _, _, _, _, _, _) => {
                h3689
            }
        }, match v1.clone() {
            Adt1::C0(h3690, _, _, _, _, _, _, _, _, _, _, _) => {
                h3690
            }
        }, match v1 {
            Adt1::C0(_, h3691, _, _, _, _, _, _, _, _, _, _) => {
                h3691
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f124(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3693 = match v0.clone() {
        Adt2::C0(h3692, _, _, _, _, _, _, _, _, _, _) => {
            h3692
        }
    };
    let a3694 = 0u64;
    if if if nat_eq(a3693, a3694) {
        let a3696 = match v1.clone() {
            Adt2::C0(h3695, _, _, _, _, _, _, _, _, _, _) => {
                h3695
            }
        };
        let a3697 = 0u64;
        nat_eq(a3696, a3697)
    } else {
        false
    } {
        f28(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f20(v1.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f125(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3699 = match v0.clone() {
        Adt2::C0(h3698, _, _, _, _, _, _, _, _, _, _) => {
            h3698
        }
    };
    let a3700 = 0u64;
    if if nat_eq(a3699, a3700) {
        let a3702 = match v1.clone() {
            Adt2::C0(h3701, _, _, _, _, _, _, _, _, _, _) => {
                h3701
            }
        };
        let a3703 = 26u64;
        nat_eq(a3702, a3703)
    } else {
        false
    } {
        let a3705 = match v0.clone() {
            Adt2::C0(_, _, _, h3704, _, _, _, _, _, _, _) => {
                h3704
            }
        };
        let a3706 = 1u64;
        if if if if if if if nat_eq(a3705, a3706) {
            let a3708 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3707, _, _, _, _, _, _) => {
                    h3707
                }
            };
            let a3709 = 1u64;
            nat_eq(a3708, a3709)
        } else {
            false
        } {
            let a3711 = match v0.clone() {
                Adt2::C0(_, h3710, _, _, _, _, _, _, _, _, _) => {
                    h3710
                }
            };
            let a3712 = 2147483647u64;
            nat_le(a3711, a3712)
        } else {
            false
        } {
            let a3715 = match v1.clone() {
                Adt2::C0(_, h3713, _, _, _, _, _, _, _, _, _) => {
                    h3713
                }
            };
            let a3716 = match v0.clone() {
                Adt2::C0(_, _, h3714, _, _, _, _, _, _, _, _) => {
                    h3714
                }
            };
            nat_eq(a3715, a3716)
        } else {
            false
        } {
            let a3718 = match v1.clone() {
                Adt2::C0(_, _, h3717, _, _, _, _, _, _, _, _) => {
                    h3717
                }
            };
            let a3719 = 1u64;
            nat_eq(a3718, a3719)
        } else {
            false
        } {
            let a3721 = match v1.clone() {
                Adt2::C0(_, _, _, h3720, _, _, _, _, _, _, _) => {
                    h3720
                }
            };
            let a3722 = 1u64;
            nat_eq(a3721, a3722)
        } else {
            false
        } {
            let a3724 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3723, _, _, _, _, _, _) => {
                    h3723
                }
            };
            let a3725 = 1u64;
            nat_eq(a3724, a3725)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a3728 = match v0.clone() {
                    Adt2::C0(_, _, h3726, _, _, _, _, _, _, _, _) => {
                        h3726
                    }
                };
                let a3729 = f162(match v0.clone() {
                    Adt2::C0(_, h3727, _, _, _, _, _, _, _, _, _) => {
                        h3727
                    }
                })?;
                nat_mul(a3728, a3729)?
            }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f126(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3730 = v1;
    match m3730 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f167(f24(v0.clone(), 1u64))?;
            let a3732 = match v0.clone() {
                Adt2::C0(h3731, _, _, _, _, _, _, _, _, _, _) => {
                    h3731
                }
            };
            let a3733 = 26u64;
            if if if nat_eq(a3732, a3733) {
                let a3735 = match v3.clone() {
                    Adt2::C0(h3734, _, _, _, _, _, _, _, _, _, _) => {
                        h3734
                    }
                };
                let a3736 = 26u64;
                nat_eq(a3735, a3736)
            } else {
                false
            } {
                let a3738 = match v2.clone() {
                    Adt2::C0(h3737, _, _, _, _, _, _, _, _, _, _) => {
                        h3737
                    }
                };
                let a3739 = 27u64;
                nat_eq(a3738, a3739)
            } else {
                false
            } {
                if if if if if if f28(v0.clone(), v3.clone()) {
                    let a3741 = match v2.clone() {
                        Adt2::C0(_, h3740, _, _, _, _, _, _, _, _, _) => {
                            h3740
                        }
                    };
                    let a3742 = 1u64;
                    nat_eq(a3741, a3742)
                } else {
                    false
                } {
                    let a3744 = match v2.clone() {
                        Adt2::C0(_, _, h3743, _, _, _, _, _, _, _, _) => {
                            h3743
                        }
                    };
                    let a3745 = 1u64;
                    nat_eq(a3744, a3745)
                } else {
                    false
                } {
                    let a3747 = match v2.clone() {
                        Adt2::C0(_, _, _, h3746, _, _, _, _, _, _, _) => {
                            h3746
                        }
                    };
                    let a3748 = 1u64;
                    nat_eq(a3747, a3748)
                } else {
                    false
                } {
                    let a3750 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3749, _, _, _, _, _, _) => {
                            h3749
                        }
                    };
                    let a3751 = 1u64;
                    nat_eq(a3750, a3751)
                } else {
                    false
                } {
                    let a3752 = v4;
                    let a3753 = 9223372036854775808u64;
                    nat_lt(a3752, a3753)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3756 = 2u64;
                        let a3757 = {
                            let a3754 = v4;
                            let a3755 = 2u64;
                            nat_mul(a3754, a3755)?
                        };
                        nat_add(a3756, a3757)?
                    }, 8u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C18(v4, f24(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f127(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h3758, _, _, _, _, _, _, _, _, _, _) => {
            h3758
        }
    };
    let a3759 = v3;
    let a3760 = 0u64;
    if if nat_eq(a3759, a3760) {
        true
    } else {
        let a3761 = v0;
        let a3762 = 13u64;
        if nat_eq(a3761, a3762) {
            let a3763 = v3;
            let a3764 = 1u64;
            if nat_eq(a3763, a3764) {
                true
            } else {
                let a3765 = v3;
                let a3766 = 30u64;
                nat_eq(a3765, a3766)
            }
        } else {
            false
        }
    } {
        let a3768 = v3;
        let a3769 = match v2.clone() {
            Adt2::C0(h3767, _, _, _, _, _, _, _, _, _, _) => {
                h3767
            }
        };
        if if nat_eq(a3768, a3769) {
            let a3770 = v0;
            let a3771 = 13u64;
            if nat_eq(a3770, a3771) {
                let a3773 = match v2.clone() {
                    Adt2::C0(_, h3772, _, _, _, _, _, _, _, _, _) => {
                        h3772
                    }
                };
                let a3774 = 1u64;
                if if if nat_eq(a3773, a3774) {
                    let a3776 = match v2.clone() {
                        Adt2::C0(_, _, h3775, _, _, _, _, _, _, _, _) => {
                            h3775
                        }
                    };
                    let a3777 = 1u64;
                    nat_eq(a3776, a3777)
                } else {
                    false
                } {
                    let a3779 = match v2.clone() {
                        Adt2::C0(_, _, _, h3778, _, _, _, _, _, _, _) => {
                            h3778
                        }
                    };
                    let a3780 = 1u64;
                    nat_eq(a3779, a3780)
                } else {
                    false
                } {
                    let a3782 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3781, _, _, _, _, _, _) => {
                            h3781
                        }
                    };
                    let a3783 = 1u64;
                    nat_eq(a3782, a3783)
                } else {
                    false
                }
            } else {
                let a3785 = match v2.clone() {
                    Adt2::C0(_, h3784, _, _, _, _, _, _, _, _, _) => {
                        h3784
                    }
                };
                let a3786 = 1u64;
                if if if nat_eq(a3785, a3786) {
                    let a3789 = match v1.clone() {
                        Adt2::C0(_, _, h3787, _, _, _, _, _, _, _, _) => {
                            h3787
                        }
                    };
                    let a3790 = match v2.clone() {
                        Adt2::C0(_, _, h3788, _, _, _, _, _, _, _, _) => {
                            h3788
                        }
                    };
                    nat_eq(a3789, a3790)
                } else {
                    false
                } {
                    let a3793 = match v1.clone() {
                        Adt2::C0(_, _, _, h3791, _, _, _, _, _, _, _) => {
                            h3791
                        }
                    };
                    let a3794 = match v2.clone() {
                        Adt2::C0(_, _, _, h3792, _, _, _, _, _, _, _) => {
                            h3792
                        }
                    };
                    nat_eq(a3793, a3794)
                } else {
                    false
                } {
                    let a3797 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3795, _, _, _, _, _, _) => {
                            h3795
                        }
                    };
                    let a3798 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3796, _, _, _, _, _, _) => {
                            h3796
                        }
                    };
                    nat_eq(a3797, a3798)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f24(v1, 1u64);
            Ok(Adt10::C4(f221(v4.clone(), v0, v3)?, {
                let a3799 = v3;
                let a3800 = 0u64;
                if nat_eq(a3799, a3800) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f24(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f128(v0: u64, v1: u64) -> R<Option<u64>> {
    let a3801 = v0;
    let a3802 = 89u64;
    if if nat_eq(a3801, a3802) {
        let a3803 = v1;
        let a3804 = 14u64;
        nat_eq(a3803, a3804)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a3805 = v0;
        let a3806 = 89u64;
        if if nat_eq(a3805, a3806) {
            let a3807 = v1;
            let a3808 = 15u64;
            nat_eq(a3807, a3808)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a3809 = v0;
            let a3810 = 89u64;
            if if nat_eq(a3809, a3810) {
                let a3811 = 8u64;
                let a3812 = v1;
                if nat_le(a3811, a3812) {
                    let a3813 = v1;
                    let a3814 = 9u64;
                    nat_le(a3813, a3814)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a3815 = v1;
                    let a3816 = 12u64;
                    nat_add(a3815, a3816)?
                }))
            } else {
                let a3817 = v0;
                let a3818 = 89u64;
                if if nat_eq(a3817, a3818) {
                    let a3819 = v1;
                    let a3820 = 4u64;
                    nat_eq(a3819, a3820)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a3821 = 10u64;
                    let a3822 = v0;
                    if if nat_le(a3821, a3822) {
                        let a3823 = v0;
                        let a3824 = 12u64;
                        nat_le(a3823, a3824)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a3825 = v0;
                            let a3826 = 3u64;
                            nat_add(a3825, a3826)?
                        }))
                    } else {
                        let a3827 = v0;
                        let a3828 = 89u64;
                        if if nat_eq(a3827, a3828) {
                            let a3829 = v1;
                            let a3830 = 7u64;
                            nat_eq(a3829, a3830)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a3831 = v0;
                            let a3832 = 89u64;
                            if if nat_eq(a3831, a3832) {
                                let a3833 = v1;
                                let a3834 = 10u64;
                                nat_eq(a3833, a3834)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a3835 = v0;
                                let a3836 = 89u64;
                                if if nat_eq(a3835, a3836) {
                                    let a3837 = v1;
                                    let a3838 = 13u64;
                                    nat_eq(a3837, a3838)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a3839 = v0;
                                    let a3840 = 8u64;
                                    if nat_eq(a3839, a3840) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a3841 = v0;
                                        let a3842 = 9u64;
                                        if nat_eq(a3841, a3842) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a3843 = v0;
                                            let a3844 = 89u64;
                                            if nat_eq(a3843, a3844) {
                                                let a3845 = v1;
                                                let a3846 = 0u64;
                                                if nat_eq(a3845, a3846) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a3847 = v1;
                                                    let a3848 = 1u64;
                                                    if nat_eq(a3847, a3848) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a3849 = v1;
                                                        let a3850 = 2u64;
                                                        if nat_eq(a3849, a3850) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a3851 = v1;
                                                            let a3852 = 3u64;
                                                            if nat_eq(a3851, a3852) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a3853 = v1;
                                                                let a3854 = 6u64;
                                                                if nat_eq(a3853, a3854) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a3855 = v1;
                                                                    let a3856 = 12u64;
                                                                    if nat_eq(a3855, a3856) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a3857 = v1;
                                                                        let a3858 = 11u64;
                                                                        if nat_eq(a3857, a3858) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a3859 = v1;
                                                                            let a3860 = 18u64;
                                                                            if nat_eq(a3859, a3860) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a3861 = v1;
                                                                                let a3862 = 19u64;
                                                                                if nat_eq(a3861, a3862) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a3863 = v1;
                                                                                    let a3864 = 20u64;
                                                                                    if nat_eq(a3863, a3864) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a3865 = v1;
                                                                                        let a3866 = 21u64;
                                                                                        if nat_eq(a3865, a3866) {
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

pub fn f129(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3867 = f222(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m3867 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a3868 = v0;
            let a3869 = 2u64;
            if if if if nat_eq(a3868, a3869) {
                true
            } else {
                let a3870 = v0;
                let a3871 = 5u64;
                nat_eq(a3870, a3871)
            } {
                true
            } else {
                let a3872 = v0;
                let a3873 = 6u64;
                nat_eq(a3872, a3873)
            } {
                true
            } else {
                let a3874 = v0;
                let a3875 = 7u64;
                nat_eq(a3874, a3875)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f24(v2.clone(), 1u64), f24(v4, 1u64), Adt4::C10(v7, v8, {
                    let m3876 = v3;
                    match m3876 {
                        None => {
                            f24(v2, 1u64)
                        }
                        Some(v14) => {
                            f24(v14, 1u64)
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
        Adt10::C7 => {
            Ok(Adt10::C7)
        }
        Adt10::C8 => {
            Ok(Adt10::C8)
        }
    }
}

pub fn f130(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3878 = match v1.clone() {
        Adt2::C0(h3877, _, _, _, _, _, _, _, _, _, _) => {
            h3877
        }
    };
    let a3879 = 0u64;
    if if if if if nat_eq(a3878, a3879) {
        true
    } else {
        let a3881 = match v1.clone() {
            Adt2::C0(h3880, _, _, _, _, _, _, _, _, _, _) => {
                h3880
            }
        };
        let a3882 = 1u64;
        nat_eq(a3881, a3882)
    } {
        true
    } else {
        let a3884 = match v1.clone() {
            Adt2::C0(h3883, _, _, _, _, _, _, _, _, _, _) => {
                h3883
            }
        };
        let a3885 = 30u64;
        nat_eq(a3884, a3885)
    } {
        let a3888 = match v1.clone() {
            Adt2::C0(h3886, _, _, _, _, _, _, _, _, _, _) => {
                h3886
            }
        };
        let a3889 = match v2.clone() {
            Adt2::C0(h3887, _, _, _, _, _, _, _, _, _, _) => {
                h3887
            }
        };
        if nat_eq(a3888, a3889) {
            true
        } else {
            let a3891 = match v2.clone() {
                Adt2::C0(h3890, _, _, _, _, _, _, _, _, _, _) => {
                    h3890
                }
            };
            let a3892 = 0u64;
            nat_eq(a3891, a3892)
        }
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3894 = f20(v2.clone(), 1u64)?;
            let a3895 = f223(v0, match v2.clone() {
                Adt2::C0(h3893, _, _, _, _, _, _, _, _, _, _) => {
                    h3893
                }
            });
            nat_mul(a3894, a3895)?
        }, {
            let a3897 = match v2.clone() {
                Adt2::C0(h3896, _, _, _, _, _, _, _, _, _, _) => {
                    h3896
                }
            };
            let a3898 = 0u64;
            if nat_eq(a3897, a3898) {
                4u64
            } else {
                2u64
            }
        }, f24(v1.clone(), 1u64), f24(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h3899, _, _, _, _, _, _, _, _, _, _) => {
                h3899
            }
        }, match v2 {
            Adt2::C0(h3900, _, _, _, _, _, _, _, _, _, _) => {
                h3900
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3913 = {
        let a3909 = {
            let a3905 = {
                let a3901 = v0;
                let a3902 = v4;
                nat_mul(a3901, a3902)?
            };
            let a3906 = {
                let a3903 = v1;
                let a3904 = v5;
                nat_mul(a3903, a3904)?
            };
            nat_add(a3905, a3906)?
        };
        let a3910 = {
            let a3907 = v2;
            let a3908 = v6;
            nat_mul(a3907, a3908)?
        };
        nat_add(a3909, a3910)?
    };
    let a3914 = {
        let a3911 = v3;
        let a3912 = v7;
        nat_mul(a3911, a3912)?
    };
    nat_add(a3913, a3914)
}

pub fn f132(v0: u64) -> u64 {
    let a3915 = v0;
    let a3916 = 2147483648u64;
    if nat_lt(a3915, a3916) {
        0u64
    } else {
        let a3917 = 4294967296u64;
        let a3918 = v0;
        nat_sub(a3917, a3918)
    }
}

pub fn f133(v0: u64) -> u64 {
    let a3919 = v0;
    let a3920 = 2147483648u64;
    if nat_lt(a3919, a3920) {
        v0
    } else {
        0u64
    }
}

pub fn f134(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3921 = v7;
        let a3922 = 6u64;
        if nat_eq(a3921, a3922) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f82(v3, v4), f82(v5, v6), f83(v7, v8), f161(v1), f161(v9))
}

pub fn f135(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3923 = v0;
    match m3923 {
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
        Adt4::C13(v50, v51, v52, v53) => {
            Adt11::C4(v50, v51, v52, v53)
        }
        Adt4::C14(v54, _, _) => {
            let a3924 = 13u64;
            let a3925 = v54;
            if nat_le(a3924, a3925) {
                Adt11::C7(v54, v1, v2)
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
        Adt4::C27(v74) => {
            Adt11::C2(match v74.clone() {
                Adt5::C0(_, _, _, _, _, h3926, _, _) => {
                    h3926
                }
            }, match v74 {
                Adt5::C0(_, _, _, _, _, _, _, h3927) => {
                    h3927
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
        Adt4::C35(_) => {
            Adt11::C6
        }
        Adt4::C36(_, _, _) => {
            Adt11::C6
        }
        Adt4::C37(_, _, _) => {
            Adt11::C6
        }
        Adt4::C38(_, _, _, _) => {
            Adt11::C6
        }
        Adt4::C39(_) => {
            Adt11::C6
        }
    }
}

pub fn f136(v0: Adt4) -> u64 {
    let m3928 = v0;
    match m3928 {
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
        Adt4::C14(v52, _, _) => {
            v52
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
        Adt4::C35(_) => {
            0u64
        }
        Adt4::C36(_, _, _) => {
            0u64
        }
        Adt4::C37(_, _, _) => {
            0u64
        }
        Adt4::C38(_, _, _, _) => {
            0u64
        }
        Adt4::C39(_) => {
            0u64
        }
    }
}

pub fn f137(v0: Adt4, v1: u64) -> u64 {
    let m3929 = v0;
    match m3929 {
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
        Adt4::C16(_, v57) => {
            v57
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v60, _) => {
            let a3930 = v1;
            let a3931 = 6u64;
            if nat_eq(a3930, a3931) {
                v60
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
        Adt4::C26(v74, _, _, _) => {
            let a3940 = v1;
            let a3941 = 6u64;
            if if if nat_eq(a3940, a3941) {
                true
            } else {
                let a3942 = v1;
                let a3943 = 7u64;
                nat_eq(a3942, a3943)
            } {
                let a3944 = v74;
                let a3945 = 30u64;
                nat_eq(a3944, a3945)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v73) => {
            match v73 {
                Adt5::C0(_, _, _, _, _, _, _, h3939) => {
                    h3939
                }
            }
        }
        Adt4::C28(v69) => {
            let a3932 = v1;
            let a3933 = 6u64;
            if nat_eq(a3932, a3933) {
                match v69 {
                    Adt7::C0(_, _, _, _, h3934) => {
                        h3934
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v72) => {
            let a3935 = v1;
            let a3936 = 6u64;
            if if nat_eq(a3935, a3936) {
                true
            } else {
                let a3937 = v1;
                let a3938 = 7u64;
                nat_eq(a3937, a3938)
            } {
                v72
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
        Adt4::C35(_) => {
            0u64
        }
        Adt4::C36(_, _, _) => {
            0u64
        }
        Adt4::C37(_, _, _) => {
            0u64
        }
        Adt4::C38(_, _, _, _) => {
            0u64
        }
        Adt4::C39(_) => {
            0u64
        }
    }
}

pub fn f138(v0: Adt4, v1: u64) -> Adt11 {
    let a3946 = v1;
    let a3947 = 4u64;
    if nat_lt(a3946, a3947) {
        Adt11::C0(v1)
    } else {
        let a3948 = v1;
        let a3949 = 4u64;
        if nat_eq(a3948, a3949) {
            Adt11::C1(0u64)
        } else {
            let a3950 = v1;
            let a3951 = 5u64;
            if nat_eq(a3950, a3951) {
                Adt11::C1(3u64)
            } else {
                let a3952 = v1;
                let a3953 = 6u64;
                if nat_eq(a3952, a3953) {
                    Adt11::C2(f137(v0, v1), 0u64, false)
                } else {
                    let a3954 = v1;
                    let a3955 = 7u64;
                    if nat_eq(a3954, a3955) {
                        Adt11::C2(0u64, f137(v0, v1), false)
                    } else {
                        let a3956 = v1;
                        let a3957 = 16u64;
                        if nat_lt(a3956, a3957) {
                            Adt11::C1({
                                let a3958 = v1;
                                let a3959 = 8u64;
                                nat_sub(a3958, a3959)
                            })
                        } else {
                            let a3960 = v1;
                            let a3961 = 20u64;
                            if nat_lt(a3960, a3961) {
                                Adt11::C5({
                                    let a3962 = v1;
                                    let a3963 = 16u64;
                                    nat_sub(a3962, a3963)
                                })
                            } else {
                                let a3964 = v1;
                                let a3965 = 20u64;
                                if nat_eq(a3964, a3965) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3966 = v1;
                                    let a3967 = 21u64;
                                    if nat_eq(a3966, a3967) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3968 = v1;
                                        let a3969 = 22u64;
                                        if nat_eq(a3968, a3969) {
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

pub fn f139(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3987 = {
        let a3982 = f224(v2, {
            let a3973 = {
                let a3970 = v0;
                let a3971 = v1;
                let a3972 = 0u64;
                nat_quot(a3970, a3971, a3972)
            };
            let a3974 = v2;
            let a3975 = 0u64;
            nat_rem(a3973, a3974, a3975)
        }, {
            let a3979 = {
                let a3976 = v0;
                let a3977 = v1;
                let a3978 = 0u64;
                nat_quot(a3976, a3977, a3978)
            };
            let a3980 = v2;
            let a3981 = 0u64;
            nat_quot(a3979, a3980, a3981)
        })?;
        let a3983 = v1;
        nat_mul(a3982, a3983)?
    };
    let a3988 = {
        let a3984 = v0;
        let a3985 = v1;
        let a3986 = 0u64;
        nat_rem(a3984, a3985, a3986)
    };
    nat_add(a3987, a3988)
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3990 = v0;
        let a3991 = match v2.clone() {
            Adt3::C0(h3989, _, _, _, _, _, _, _) => {
                h3989
            }
        };
        let a3992 = 0u64;
        nat_quot(a3990, a3991, a3992)
    };
    let v4: u64 = {
        let a3994 = v3;
        let a3995 = match v2.clone() {
            Adt3::C0(_, h3993, _, _, _, _, _, _) => {
                h3993
            }
        };
        let a3996 = 0u64;
        nat_quot(a3994, a3995, a3996)
    };
    f225({
        let a3998 = v0;
        let a3999 = match v2.clone() {
            Adt3::C0(h3997, _, _, _, _, _, _, _) => {
                h3997
            }
        };
        let a4000 = 0u64;
        nat_rem(a3998, a3999, a4000)
    }, {
        let a4002 = v3;
        let a4003 = match v2.clone() {
            Adt3::C0(_, h4001, _, _, _, _, _, _) => {
                h4001
            }
        };
        let a4004 = 0u64;
        nat_rem(a4002, a4003, a4004)
    }, {
        let a4006 = v4;
        let a4007 = match v2.clone() {
            Adt3::C0(_, _, h4005, _, _, _, _, _) => {
                h4005
            }
        };
        let a4008 = 0u64;
        nat_rem(a4006, a4007, a4008)
    }, {
        let a4010 = v4;
        let a4011 = match v2 {
            Adt3::C0(_, _, h4009, _, _, _, _, _) => {
                h4009
            }
        };
        let a4012 = 0u64;
        nat_quot(a4010, a4011, a4012)
    }, match v1.clone() {
        Adt3::C0(h4013, _, _, _, _, _, _, _) => {
            h4013
        }
    }, match v1.clone() {
        Adt3::C0(_, h4014, _, _, _, _, _, _) => {
            h4014
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4015, _, _, _, _, _) => {
            h4015
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4016, _, _, _, _) => {
            h4016
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4017, _, _, _) => {
            h4017
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4018, _, _) => {
            h4018
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4019, _) => {
            h4019
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4020) => {
            h4020
        }
    })
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4022 = v0;
        let a4023 = match v3.clone() {
            Adt3::C0(h4021, _, _, _, _, _, _, _) => {
                h4021
            }
        };
        let a4024 = 0u64;
        nat_quot(a4022, a4023, a4024)
    };
    let v6: u64 = {
        let a4026 = v5;
        let a4027 = match v3.clone() {
            Adt3::C0(_, h4025, _, _, _, _, _, _) => {
                h4025
            }
        };
        let a4028 = 0u64;
        nat_quot(a4026, a4027, a4028)
    };
    let v7: u64 = {
        let a4030 = v0;
        let a4031 = match v3.clone() {
            Adt3::C0(h4029, _, _, _, _, _, _, _) => {
                h4029
            }
        };
        let a4032 = 0u64;
        nat_rem(a4030, a4031, a4032)
    };
    let v8: u64 = {
        let a4034 = v5;
        let a4035 = match v3.clone() {
            Adt3::C0(_, h4033, _, _, _, _, _, _) => {
                h4033
            }
        };
        let a4036 = 0u64;
        nat_rem(a4034, a4035, a4036)
    };
    let v9: u64 = {
        let a4038 = v6;
        let a4039 = match v3.clone() {
            Adt3::C0(_, _, h4037, _, _, _, _, _) => {
                h4037
            }
        };
        let a4040 = 0u64;
        nat_rem(a4038, a4039, a4040)
    };
    let v10: u64 = {
        let a4042 = v6;
        let a4043 = match v3.clone() {
            Adt3::C0(_, _, h4041, _, _, _, _, _) => {
                h4041
            }
        };
        let a4044 = 0u64;
        nat_quot(a4042, a4043, a4044)
    };
    let v11: u64 = {
        let a4045 = v4;
        let a4046 = 0u64;
        if nat_eq(a4045, a4046) {
            match v1.clone() {
                Adt3::C0(h4047, _, _, _, _, _, _, _) => {
                    h4047
                }
            }
        } else {
            let a4048 = v4;
            let a4049 = 1u64;
            if nat_eq(a4048, a4049) {
                match v1.clone() {
                    Adt3::C0(_, h4050, _, _, _, _, _, _) => {
                        h4050
                    }
                }
            } else {
                let a4051 = v4;
                let a4052 = 2u64;
                if nat_eq(a4051, a4052) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4053, _, _, _, _, _) => {
                            h4053
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4054, _, _, _, _) => {
                            h4054
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4061 = v11;
        let a4062 = {
            let a4055 = v4;
            let a4056 = 0u64;
            if nat_eq(a4055, a4056) {
                v7
            } else {
                let a4057 = v4;
                let a4058 = 1u64;
                if nat_eq(a4057, a4058) {
                    v8
                } else {
                    let a4059 = v4;
                    let a4060 = 2u64;
                    if nat_eq(a4059, a4060) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4061, a4062)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f131(if if v12 {
        let a4063 = v4;
        let a4064 = 0u64;
        nat_eq(a4063, a4064)
    } else {
        false
    } {
        let a4065 = v7;
        let a4066 = v11;
        nat_sub(a4065, a4066)
    } else {
        v7
    }, if if v12 {
        let a4067 = v4;
        let a4068 = 1u64;
        nat_eq(a4067, a4068)
    } else {
        false
    } {
        let a4069 = v8;
        let a4070 = v11;
        nat_sub(a4069, a4070)
    } else {
        v8
    }, if if v12 {
        let a4071 = v4;
        let a4072 = 2u64;
        nat_eq(a4071, a4072)
    } else {
        false
    } {
        let a4073 = v9;
        let a4074 = v11;
        nat_sub(a4073, a4074)
    } else {
        v9
    }, if if v12 {
        let a4075 = v4;
        let a4076 = 3u64;
        nat_eq(a4075, a4076)
    } else {
        false
    } {
        let a4077 = v10;
        let a4078 = v11;
        nat_sub(a4077, a4078)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4079, _, _, _) => {
            h4079
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4080, _, _) => {
            h4080
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4081, _) => {
            h4081
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4082) => {
            h4082
        }
    })?, f84(v0, v3)?))
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f54(v0, v2.clone(), 0u64);
    let v9: u64 = f54(v0, v2.clone(), 1u64);
    let v10: u64 = f54(v0, v2.clone(), 2u64);
    let v11: u64 = f54(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f53(v1.clone(), f226(v8, v3, match v1.clone() {
            Adt3::C0(h4083, _, _, _, _, _, _, _) => {
                h4083
            }
        })?, f226(v9, v4, match v1.clone() {
            Adt3::C0(_, h4084, _, _, _, _, _, _) => {
                h4084
            }
        })?, f226(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4085, _, _, _, _, _) => {
                h4085
            }
        })?, f226(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4086, _, _, _, _) => {
                h4086
            }
        })?)?, f84(v0, v2)?))
    } else {
        if if if if f227(v8, v3, match v1.clone() {
            Adt3::C0(h4087, _, _, _, _, _, _, _) => {
                h4087
            }
        })? {
            f227(v9, v4, match v1.clone() {
                Adt3::C0(_, h4088, _, _, _, _, _, _) => {
                    h4088
                }
            })?
        } else {
            false
        } {
            f227(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4089, _, _, _, _, _) => {
                    h4089
                }
            })?
        } else {
            false
        } {
            f227(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4090, _, _, _, _) => {
                    h4090
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f53(v1, f228(v8, v3)?, f228(v9, v4)?, f228(v10, v5)?, f228(v11, v6)?)?, f84(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f84(v0, v2)?))
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f54(v0, v2.clone(), 0u64);
    let v5: u64 = f54(v0, v2.clone(), 1u64);
    let v6: u64 = f54(v0, v2.clone(), 2u64);
    let v7: u64 = f54(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f53(v1.clone(), f229(v4, v3, match v1 {
        Adt3::C0(h4091, _, _, _, _, _, _, _) => {
            h4091
        }
    })?, v5, v6, v7)?, f84(v0, v2)?))
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4092 = v0;
    let a4093 = v3;
    if nat_lt(a4092, a4093) {
        Ok(Adt9::C1(false, {
            let a4094 = v0;
            let a4095 = 4u64;
            nat_mul(a4094, a4095)?
        }, {
            let a4096 = v0;
            let a4097 = 4u64;
            nat_mul(a4096, a4097)?
        }))
    } else {
        let v6: u64 = {
            let a4098 = v0;
            let a4099 = v3;
            nat_sub(a4098, a4099)
        };
        let v7: u64 = f84(v6, v1)?;
        let v8: u64 = f230(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4100 = v0;
    let a4101 = v3;
    if nat_lt(a4100, a4101) {
        Ok(Adt9::C1(true, 0u64, f84(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4102 = v0;
            let a4103 = v3;
            nat_sub(a4102, a4103)
        };
        let v5: u64 = f140(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f84(v4, v1)?, 3u64, v5))
    }
}

pub fn f146(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4105 = v1;
        let a4106 = match v3.clone() {
            Adt3::C0(h4104, _, _, _, _, _, _, _) => {
                h4104
            }
        };
        let a4107 = 0u64;
        nat_rem(a4105, a4106, a4107)
    };
    let v5: u64 = {
        let a4113 = {
            let a4109 = v1;
            let a4110 = match v3.clone() {
                Adt3::C0(h4108, _, _, _, _, _, _, _) => {
                    h4108
                }
            };
            let a4111 = 0u64;
            nat_quot(a4109, a4110, a4111)
        };
        let a4114 = match v3.clone() {
            Adt3::C0(_, h4112, _, _, _, _, _, _) => {
                h4112
            }
        };
        let a4115 = 0u64;
        nat_rem(a4113, a4114, a4115)
    };
    let v6: bool = f231(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f140(v1, v2, v3.clone())?
    } else {
        0u64
    }, f84(v1, v3)?))
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4116 = v3;
        let a4117 = 8u64;
        nat_add(a4116, a4117)?
    }, 0u64, f84(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f140(v0, v5, v2.clone())?
    }, 3u64, f84(v0, v2)?))
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4118 = v4;
    let a4119 = 0u64;
    if if nat_eq(a4118, a4119) {
        f112(v3)
    } else {
        false
    } {
        let m4120 = f202(v3);
        match m4120 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4121, _) => {
                        h4121
                    }
                };
                Ok(Adt9::C5(v3, f84(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4122) => {
                        h4122
                    }
                }, f84({
                    let a4123 = v0;
                    let a4124 = v6;
                    nat_mul(a4123, a4124)?
                }, v2.clone())?, {
                    let a4130 = {
                        let a4128 = {
                            let a4125 = v6;
                            let a4126 = 1u64;
                            nat_sub(a4125, a4126)
                        };
                        let a4129 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4127, _, _, _) => {
                                h4127
                            }
                        };
                        nat_mul(a4128, a4129)?
                    };
                    let a4131 = 4u64;
                    nat_add(a4130, a4131)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4132, _, _, _) => {
                        h4132
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f84(v0, v1)?, f84(v0, v2)?))
    }
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4134 = v0;
        let a4135 = match v2.clone() {
            Adt3::C0(h4133, _, _, _, _, _, _, _) => {
                h4133
            }
        };
        let a4136 = 0u64;
        nat_rem(a4134, a4135, a4136)
    };
    let v7: u64 = {
        let a4138 = v0;
        let a4139 = match v2.clone() {
            Adt3::C0(h4137, _, _, _, _, _, _, _) => {
                h4137
            }
        };
        let a4140 = 0u64;
        nat_quot(a4138, a4139, a4140)
    };
    let v8: u64 = {
        let a4142 = v7;
        let a4143 = match v2.clone() {
            Adt3::C0(_, h4141, _, _, _, _, _, _) => {
                h4141
            }
        };
        let a4144 = 0u64;
        nat_rem(a4142, a4143, a4144)
    };
    let v9: u64 = {
        let a4146 = v7;
        let a4147 = match v2.clone() {
            Adt3::C0(_, h4145, _, _, _, _, _, _) => {
                h4145
            }
        };
        let a4148 = 0u64;
        nat_quot(a4146, a4147, a4148)
    };
    let v10: u64 = {
        let a4150 = v9;
        let a4151 = match v2.clone() {
            Adt3::C0(_, _, h4149, _, _, _, _, _) => {
                h4149
            }
        };
        let a4152 = 0u64;
        nat_rem(a4150, a4151, a4152)
    };
    let v11: u64 = {
        let a4154 = v9;
        let a4155 = match v2.clone() {
            Adt3::C0(_, _, h4153, _, _, _, _, _) => {
                h4153
            }
        };
        let a4156 = 0u64;
        nat_quot(a4154, a4155, a4156)
    };
    let v12: u64 = f84(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4165 = {
                let a4160 = {
                    let a4157 = v10;
                    let a4158 = v4;
                    let a4159 = 0u64;
                    nat_quot(a4157, a4158, a4159)
                };
                let a4161 = v5;
                nat_mul(a4160, a4161)?
            };
            let a4166 = {
                let a4162 = v8;
                let a4163 = v4;
                let a4164 = 0u64;
                nat_quot(a4162, a4163, a4164)
            };
            nat_add(a4165, a4166)?
        };
        let v14: u64 = {
            let a4186 = v6;
            let a4187 = {
                let a4184 = match v1.clone() {
                    Adt3::C0(h4167, _, _, _, _, _, _, _) => {
                        h4167
                    }
                };
                let a4185 = {
                    let a4182 = {
                        let a4168 = v8;
                        let a4169 = v4;
                        let a4170 = 0u64;
                        nat_rem(a4168, a4169, a4170)
                    };
                    let a4183 = {
                        let a4180 = match v1.clone() {
                            Adt3::C0(_, h4171, _, _, _, _, _, _) => {
                                h4171
                            }
                        };
                        let a4181 = {
                            let a4178 = {
                                let a4172 = v10;
                                let a4173 = v4;
                                let a4174 = 0u64;
                                nat_rem(a4172, a4173, a4174)
                            };
                            let a4179 = {
                                let a4176 = match v1.clone() {
                                    Adt3::C0(_, _, h4175, _, _, _, _, _) => {
                                        h4175
                                    }
                                };
                                let a4177 = v13;
                                nat_mul(a4176, a4177)?
                            };
                            nat_add(a4178, a4179)?
                        };
                        nat_mul(a4180, a4181)?
                    };
                    nat_add(a4182, a4183)?
                };
                nat_mul(a4184, a4185)?
            };
            nat_add(a4186, a4187)?
        };
        Ok(Adt9::C1(false, f84(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4193 = {
                let a4191 = {
                    let a4188 = v11;
                    let a4189 = v5;
                    let a4190 = 0u64;
                    nat_rem(a4188, a4189, a4190)
                };
                let a4192 = v4;
                nat_mul(a4191, a4192)?
            };
            let a4194 = v8;
            nat_add(a4193, a4194)?
        };
        let v16: u64 = {
            let a4200 = {
                let a4198 = {
                    let a4195 = v11;
                    let a4196 = v5;
                    let a4197 = 0u64;
                    nat_quot(a4195, a4196, a4197)
                };
                let a4199 = v4;
                nat_mul(a4198, a4199)?
            };
            let a4201 = v10;
            nat_add(a4200, a4201)?
        };
        let a4203 = v15;
        let a4204 = match v1.clone() {
            Adt3::C0(_, h4202, _, _, _, _, _, _) => {
                h4202
            }
        };
        if if nat_lt(a4203, a4204) {
            let a4206 = v16;
            let a4207 = match v1.clone() {
                Adt3::C0(_, _, h4205, _, _, _, _, _) => {
                    h4205
                }
            };
            nat_lt(a4206, a4207)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f84({
                let a4216 = v6;
                let a4217 = {
                    let a4214 = match v1.clone() {
                        Adt3::C0(h4208, _, _, _, _, _, _, _) => {
                            h4208
                        }
                    };
                    let a4215 = {
                        let a4212 = v15;
                        let a4213 = {
                            let a4210 = match v1.clone() {
                                Adt3::C0(_, h4209, _, _, _, _, _, _) => {
                                    h4209
                                }
                            };
                            let a4211 = v16;
                            nat_mul(a4210, a4211)?
                        };
                        nat_add(a4212, a4213)?
                    };
                    nat_mul(a4214, a4215)?
                };
                nat_add(a4216, a4217)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f223(23u64, v4);
    let v6: u64 = {
        let a4218 = v0;
        let a4219 = v5;
        let a4220 = 0u64;
        nat_rem(a4218, a4219, a4220)
    };
    let v7: u64 = {
        let a4221 = v0;
        let a4222 = v5;
        let a4223 = 0u64;
        nat_quot(a4221, a4222, a4223)
    };
    let v8: Adt13 = Adt13::C1(f84(v7, v2)?);
    let a4224 = v6;
    let a4225 = 0u64;
    if nat_eq(a4224, a4225) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f84(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f161(v3), 4u64))
    } else {
        let a4226 = v6;
        let a4227 = 1u64;
        if nat_eq(a4226, a4227) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4228 = v4;
                let a4229 = 0u64;
                if nat_eq(a4228, a4229) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f161(v4)))
        }
    }
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f84(v0, v2.clone())?;
    let v4: u64 = f84(v0, v1.clone())?;
    let a4234 = {
        let a4231 = v0;
        let a4232 = match v1 {
            Adt3::C0(h4230, _, _, _, _, _, _, _) => {
                h4230
            }
        };
        let a4233 = 0u64;
        nat_rem(a4231, a4232, a4233)
    };
    let a4235 = 0u64;
    if nat_eq(a4234, a4235) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f84({
            let a4236 = v0;
            let a4237 = 1u64;
            nat_sub(a4236, a4237)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4238 = v3;
    let a4239 = 13u64;
    if nat_eq(a4238, a4239) {
        f232(v0, v1, v4)
    } else {
        let v5: u64 = f233(match v1.clone() {
            Adt3::C0(h4240, _, _, _, _, _, _, _) => {
                h4240
            }
        }, {
            let a4241 = v3;
            let a4242 = 16u64;
            nat_eq(a4241, a4242)
        })?;
        let v6: u64 = {
            let a4243 = v0;
            let a4244 = v5;
            let a4245 = 0u64;
            nat_quot(a4243, a4244, a4245)
        };
        let v7: u64 = {
            let a4246 = v0;
            let a4247 = v5;
            let a4248 = 0u64;
            nat_rem(a4246, a4247, a4248)
        };
        let v8: u64 = {
            let a4252 = 1u64;
            let a4253 = {
                let a4250 = match v1.clone() {
                    Adt3::C0(h4249, _, _, _, _, _, _, _) => {
                        h4249
                    }
                };
                let a4251 = 2u64;
                nat_mul(a4250, a4251)?
            };
            nat_add(a4252, a4253)?
        };
        let a4254 = v7;
        let a4255 = v8;
        if nat_lt(a4254, a4255) {
            f234(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f84(v6, v2)?;
            let a4256 = v3;
            let a4257 = 14u64;
            if nat_eq(a4256, a4257) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4258 = v7;
                let a4259 = v8;
                if nat_eq(a4258, a4259) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4262 = v7;
                    let a4263 = {
                        let a4260 = v8;
                        let a4261 = 1u64;
                        nat_add(a4260, a4261)?
                    };
                    if nat_eq(a4262, a4263) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4264, _, _, _, _, _, _, _) => {
                                h4264
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

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4265 = v0;
    let a4266 = 0u64;
    if nat_eq(a4265, a4266) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4273 = {
            let a4267 = v0;
            let a4268 = 1u64;
            nat_add(a4267, a4268)?
        };
        let a4274 = {
            let a4271 = 2u64;
            let a4272 = {
                let a4269 = v3;
                let a4270 = 2u64;
                nat_mul(a4269, a4270)?
            };
            nat_add(a4271, a4272)?
        };
        if nat_eq(a4273, a4274) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4275 = v0;
                let a4276 = 1u64;
                nat_sub(a4275, a4276)
            };
            let v5: u64 = {
                let a4277 = v4;
                let a4278 = 2u64;
                let a4279 = 0u64;
                nat_quot(a4277, a4278, a4279)
            };
            let a4283 = {
                let a4280 = v4;
                let a4281 = 2u64;
                let a4282 = 0u64;
                nat_rem(a4280, a4281, a4282)
            };
            let a4284 = 0u64;
            if nat_eq(a4283, a4284) {
                Ok(Adt9::C4(17u64, 0u64, f84(v5, v1)?, 4u64, f84(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4285 = v0;
    let a4286 = f37(v1, v2, v3)?;
    if nat_lt(a4285, a4286) {
        let a4287 = v0;
        let a4288 = v1;
        if nat_lt(a4287, a4288) {
            Ok(Adt9::C1(false, {
                let a4289 = 4u64;
                let a4290 = v0;
                nat_mul(a4289, a4290)?
            }, {
                let a4291 = 4u64;
                let a4292 = v0;
                nat_mul(a4291, a4292)?
            }))
        } else {
            let v4: u64 = {
                let a4293 = v0;
                let a4294 = v1;
                nat_sub(a4293, a4294)
            };
            let v5: u64 = f235(v4, v2)?;
            let a4295 = v5;
            let a4296 = 2u64;
            if nat_lt(a4295, a4296) {
                Ok(Adt9::C4(6u64, {
                    let a4297 = v5;
                    let a4298 = 0u64;
                    if nat_eq(a4297, a4298) {
                        4u64
                    } else {
                        5u64
                    }
                }, f236(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f237(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f238(v4, v2)? {
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

pub fn f155(v0: u64, v1: u64) -> u64 {
    let a4299 = v0;
    let a4300 = 0u64;
    if if nat_eq(a4299, a4300) {
        true
    } else {
        let a4301 = v1;
        let a4302 = 0u64;
        nat_eq(a4301, a4302)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f156(v0: u64, v1: u64) -> Adt11 {
    let a4303 = v0;
    let a4304 = 43u64;
    if nat_eq(a4303, a4304) {
        Adt11::C8(3u64)
    } else {
        let a4305 = v1;
        let a4306 = 43u64;
        if nat_eq(a4305, a4306) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f157(v0: u64) -> R<u64> {
    let a4309 = 2u64;
    let a4310 = {
        let a4307 = 6u64;
        let a4308 = v0;
        nat_mul(a4307, a4308)?
    };
    nat_add(a4309, a4310)
}

pub fn f158(v0: u64, v1: bool) -> R<u64> {
    let a4313 = {
        let a4311 = v0;
        let a4312 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4311, a4312)?
    };
    let a4314 = 1u64;
    nat_add(a4313, a4314)
}

pub fn f159(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4315 = v0;
    let a4316 = 16u64;
    if nat_lt(a4315, a4316) {
        let v9: u64 = {
            let a4317 = v0;
            let a4318 = 4u64;
            let a4319 = 0u64;
            nat_quot(a4317, a4318, a4319)
        };
        let v10: u64 = {
            let a4324 = {
                let a4322 = 8u64;
                let a4323 = {
                    let a4320 = 4u64;
                    let a4321 = v1;
                    nat_mul(a4320, a4321)?
                };
                nat_add(a4322, a4323)?
            };
            let a4325 = v9;
            nat_add(a4324, a4325)?
        };
        let v11: u64 = f53(v6, v9, v1, v2, 0u64)?;
        let a4329 = {
            let a4326 = v0;
            let a4327 = 4u64;
            let a4328 = 0u64;
            nat_rem(a4326, a4327, a4328)
        };
        let a4330 = 0u64;
        if nat_eq(a4329, a4330) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f53(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f53(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4334 = {
                let a4331 = v0;
                let a4332 = 4u64;
                let a4333 = 0u64;
                nat_rem(a4331, a4332, a4333)
            };
            let a4335 = 1u64;
            if nat_eq(a4334, a4335) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f53(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4339 = {
                    let a4336 = v0;
                    let a4337 = 4u64;
                    let a4338 = 0u64;
                    nat_rem(a4336, a4337, a4338)
                };
                let a4340 = 2u64;
                if nat_eq(a4339, a4340) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4341 = v9;
                        let a4342 = 0u64;
                        if nat_eq(a4341, a4342) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4343 = v8;
                        let a4344 = 2u64;
                        if nat_eq(a4343, a4344) {
                            let a4345 = v9;
                            let a4346 = 0u64;
                            if nat_eq(a4345, a4346) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        } else {
                            Adt12::C3(v11)
                        }
                    }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
            }
        }
    } else {
        let a4347 = v0;
        let a4348 = 28u64;
        if nat_lt(a4347, a4348) {
            let v12: u64 = {
                let a4349 = v0;
                let a4350 = 16u64;
                nat_sub(a4349, a4350)
            };
            let v13: u64 = {
                let a4351 = v12;
                let a4352 = 3u64;
                let a4353 = 0u64;
                nat_quot(a4351, a4352, a4353)
            };
            let v14: u64 = f53(v6, v13, v1, v2, 0u64)?;
            let a4357 = {
                let a4354 = v12;
                let a4355 = 3u64;
                let a4356 = 0u64;
                nat_rem(a4354, a4355, a4356)
            };
            let a4358 = 0u64;
            if nat_eq(a4357, a4358) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4362 = {
                    let a4359 = v12;
                    let a4360 = 3u64;
                    let a4361 = 0u64;
                    nat_rem(a4359, a4360, a4361)
                };
                let a4363 = 1u64;
                if nat_eq(a4362, a4363) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4364 = v13;
                        let a4365 = 0u64;
                        if nat_eq(a4364, a4365) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4366 = v0;
            let a4367 = 28u64;
            if nat_eq(a4366, a4367) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4368 = v0;
                    let a4369 = 29u64;
                    nat_sub(a4368, a4369)
                };
                let v16: u64 = {
                    let a4370 = v15;
                    let a4371 = 2u64;
                    let a4372 = 0u64;
                    nat_quot(a4370, a4371, a4372)
                };
                let v17: u64 = f53(v6, v16, v1, v2, 0u64)?;
                let a4376 = {
                    let a4373 = v15;
                    let a4374 = 2u64;
                    let a4375 = 0u64;
                    nat_rem(a4373, a4374, a4375)
                };
                let a4377 = 0u64;
                if nat_eq(a4376, a4377) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f160(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4378 = v0;
        let a4379 = 9u64;
        let a4380 = 0u64;
        nat_rem(a4378, a4379, a4380)
    };
    let v5: u64 = {
        let a4384 = {
            let a4381 = v0;
            let a4382 = 9u64;
            let a4383 = 0u64;
            nat_quot(a4381, a4382, a4383)
        };
        let a4385 = 4u64;
        let a4386 = 0u64;
        nat_rem(a4384, a4385, a4386)
    };
    let v6: bool = {
        let a4393 = {
            let a4390 = {
                let a4387 = v0;
                let a4388 = 36u64;
                let a4389 = 0u64;
                nat_quot(a4387, a4388, a4389)
            };
            let a4391 = 2u64;
            let a4392 = 0u64;
            nat_rem(a4390, a4391, a4392)
        };
        let a4394 = 0u64;
        nat_eq(a4393, a4394)
    };
    let v7: u64 = {
        let a4395 = v4;
        let a4396 = 4u64;
        if nat_lt(a4395, a4396) {
            v4
        } else {
            let a4397 = v4;
            let a4398 = 5u64;
            nat_sub(a4397, a4398)
        }
    };
    let v8: u64 = f53(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a4399 = v4;
    let a4400 = 4u64;
    if nat_lt(a4399, a4400) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4401 = v4;
            let a4402 = 0u64;
            if nat_eq(a4401, a4402) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4403 = v4;
        let a4404 = 4u64;
        if nat_eq(a4403, a4404) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f161(v0: u64) -> u64 {
    let m4405 = f202(v0);
    match m4405 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4407 = match v1.clone() {
                Adt19::C0(_, h4406, _) => {
                    h4406
                }
            };
            let a4408 = 1u64;
            if if nat_eq(a4407, a4408) {
                let a4410 = match v1.clone() {
                    Adt19::C0(_, _, h4409) => {
                        h4409
                    }
                };
                let a4411 = 1u64;
                if if if nat_eq(a4410, a4411) {
                    true
                } else {
                    let a4413 = match v1.clone() {
                        Adt19::C0(_, _, h4412) => {
                            h4412
                        }
                    };
                    let a4414 = 2u64;
                    nat_eq(a4413, a4414)
                } {
                    true
                } else {
                    let a4416 = match v1.clone() {
                        Adt19::C0(_, _, h4415) => {
                            h4415
                        }
                    };
                    let a4417 = 4u64;
                    nat_eq(a4416, a4417)
                } {
                    true
                } else {
                    let a4419 = match v1.clone() {
                        Adt19::C0(_, _, h4418) => {
                            h4418
                        }
                    };
                    let a4420 = 8u64;
                    nat_eq(a4419, a4420)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4421) => {
                        h4421
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f162(v0: u64) -> R<u64> {
    let a4424 = 3u64;
    let a4425 = {
        let a4422 = v0;
        let a4423 = 3u64;
        nat_mul(a4422, a4423)?
    };
    nat_add(a4424, a4425)
}

pub fn f163(v0: u64, v1: u64) -> R<u64> {
    if f164(v0) {
        let a4426 = 2u64;
        let a4427 = v1;
        nat_mul(a4426, a4427)
    } else {
        let a4428 = v1;
        let a4429 = f239(v0);
        let a4430 = 0u64;
        Ok(nat_quot(a4428, a4429, a4430))
    }
}

pub fn f164(v0: u64) -> bool {
    let a4431 = v0;
    let a4432 = 0u64;
    if if nat_eq(a4431, a4432) {
        true
    } else {
        let a4433 = v0;
        let a4434 = 1u64;
        nat_eq(a4433, a4434)
    } {
        true
    } else {
        let a4435 = v0;
        let a4436 = 30u64;
        nat_eq(a4435, a4436)
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4437 = f202(v0);
    match m4437 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f131(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4438, _, _, _) => {
                    h4438
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4439, _, _) => {
                    h4439
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4440, _) => {
                    h4440
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4441) => {
                    h4441
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4442) => {
                    h4442
                }
            }, f131({
                let a4444 = v2;
                let a4445 = match v6.clone() {
                    Adt19::C0(_, h4443, _) => {
                        h4443
                    }
                };
                nat_mul(a4444, a4445)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4446, _, _, _) => {
                    h4446
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4447, _, _) => {
                    h4447
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4448, _) => {
                    h4448
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4449) => {
                    h4449
                }
            })?, match v6 {
                Adt19::C0(_, h4450, _) => {
                    h4450
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4451, _, _, _) => {
                    h4451
                }
            }))
        }
    }
}

pub fn f166(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4459 = {
        let a4452 = 2u64;
        let a4453 = f167(v1.clone())?;
        nat_mul(a4452, a4453)?
    };
    let a4460 = {
        let a4457 = f167(v1)?;
        let a4458 = {
            let a4455 = 1u64;
            let a4456 = match v0 {
                Adt3::C0(h4454, _, _, _, _, _, _, _) => {
                    h4454
                }
            };
            nat_add(a4455, a4456)?
        };
        nat_mul(a4457, a4458)?
    };
    nat_add(a4459, a4460)
}

pub fn f167(v0: Adt3) -> R<u64> {
    if f44(match v0.clone() {
        Adt3::C0(h4461, _, _, _, _, _, _, _) => {
            h4461
        }
    }, match v0.clone() {
        Adt3::C0(_, h4462, _, _, _, _, _, _) => {
            h4462
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4463, _, _, _, _, _) => {
            h4463
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4464, _, _, _, _) => {
            h4464
        }
    }) {
        Ok(0u64)
    } else {
        let a4473 = match v0.clone() {
            Adt3::C0(h4465, _, _, _, _, _, _, _) => {
                h4465
            }
        };
        let a4474 = {
            let a4471 = match v0.clone() {
                Adt3::C0(_, h4466, _, _, _, _, _, _) => {
                    h4466
                }
            };
            let a4472 = {
                let a4469 = match v0.clone() {
                    Adt3::C0(_, _, h4467, _, _, _, _, _) => {
                        h4467
                    }
                };
                let a4470 = match v0 {
                    Adt3::C0(_, _, _, h4468, _, _, _, _) => {
                        h4468
                    }
                };
                nat_mul(a4469, a4470)?
            };
            nat_mul(a4471, a4472)?
        };
        nat_mul(a4473, a4474)
    }
}

pub fn f168(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f84(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f169(v0: u64, v1: u64) -> bool {
    let a4475 = v1;
    let a4476 = 2147483647u64;
    if nat_le(a4475, a4476) {
        let a4477 = v1;
        let a4478 = v0;
        nat_lt(a4477, a4478)
    } else {
        false
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4479 = f202(v0);
    match m4479 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a4481 = v2;
                let a4482 = match v7.clone() {
                    Adt19::C0(_, h4480, _) => {
                        h4480
                    }
                };
                nat_mul(a4481, a4482)?
            };
            let m4495 = f240(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4483, _, _, _, _, _, _, _) => {
                    h4483
                }
            }, match v5.clone() {
                Adt3::C0(_, h4484, _, _, _, _, _, _) => {
                    h4484
                }
            }, match v5 {
                Adt3::C0(_, _, h4485, _, _, _, _, _) => {
                    h4485
                }
            }, {
                let a4488 = match v3.clone() {
                    Adt3::C0(h4486, _, _, _, _, _, _, _) => {
                        h4486
                    }
                };
                let a4489 = match v7.clone() {
                    Adt19::C0(_, h4487, _) => {
                        h4487
                    }
                };
                nat_mul(a4488, a4489)?
            }, match v3.clone() {
                Adt3::C0(_, h4490, _, _, _, _, _, _) => {
                    h4490
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4491, _, _, _) => {
                    h4491
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4492, _, _) => {
                    h4492
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4493, _) => {
                    h4493
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4494) => {
                    h4494
                }
            })?;
            match m4495 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4496 = v0;
                    let a4497 = 0u64;
                    if if nat_eq(a4496, a4497) {
                        true
                    } else {
                        let a4498 = v0;
                        let a4499 = 26u64;
                        nat_eq(a4498, a4499)
                    } {
                        Ok(Adt9::C1(false, v9, f84({
                            let a4503 = {
                                let a4501 = v1;
                                let a4502 = match v4.clone() {
                                    Adt3::C0(h4500, _, _, _, _, _, _, _) => {
                                        h4500
                                    }
                                };
                                nat_mul(a4501, a4502)?
                            };
                            let a4504 = v8;
                            nat_add(a4503, a4504)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h4505) => {
                                h4505
                            }
                        }, f84({
                            let a4509 = {
                                let a4507 = v1;
                                let a4508 = match v4.clone() {
                                    Adt3::C0(h4506, _, _, _, _, _, _, _) => {
                                        h4506
                                    }
                                };
                                nat_mul(a4507, a4508)?
                            };
                            let a4510 = v8;
                            nat_add(a4509, a4510)?
                        }, v4.clone())?, {
                            let a4517 = {
                                let a4515 = {
                                    let a4512 = match v7 {
                                        Adt19::C0(_, h4511, _) => {
                                            h4511
                                        }
                                    };
                                    let a4513 = 1u64;
                                    nat_sub(a4512, a4513)
                                };
                                let a4516 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4514, _, _, _) => {
                                        h4514
                                    }
                                };
                                nat_mul(a4515, a4516)?
                            };
                            let a4518 = 4u64;
                            nat_add(a4517, a4518)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4519, _, _, _) => {
                                h4519
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4520 = v1;
    let a4521 = 0u64;
    if if nat_eq(a4520, a4521) {
        true
    } else {
        let a4522 = v2;
        let a4523 = 0u64;
        nat_eq(a4522, a4523)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f172(v0: u64, v1: u64) -> bool {
    let a4524 = v0;
    let a4525 = 4294967295u64;
    if nat_eq(a4524, a4525) {
        true
    } else {
        let a4526 = v0;
        let a4527 = 2147483648u64;
        if nat_lt(a4526, a4527) {
            let a4528 = v0;
            let a4529 = v1;
            nat_lt(a4528, a4529)
        } else {
            false
        }
    }
}

pub fn f173(v0: Adt7) -> R<u64> {
    let a4541 = if match v0.clone() {
        Adt7::C0(_, _, _, h4530, _) => {
            h4530
        }
    } {
        2u64
    } else {
        3u64
    };
    let a4542 = {
        let a4539 = {
            let a4537 = {
                let m4533 = match v0.clone() {
                    Adt7::C0(h4532, _, _, _, _) => {
                        h4532
                    }
                };
                match m4533 {
                    Adt6::C0(h4531, _, _, _) => {
                        h4531
                    }
                }
            };
            let a4538 = {
                let m4536 = match v0.clone() {
                    Adt7::C0(_, h4535, _, _, _) => {
                        h4535
                    }
                };
                match m4536 {
                    Adt6::C0(h4534, _, _, _) => {
                        h4534
                    }
                }
            };
            nat_mul(a4537, a4538)?
        };
        let a4540 = f174(v0)?;
        nat_mul(a4539, a4540)?
    };
    nat_add(a4541, a4542)
}

pub fn f174(v0: Adt7) -> R<u64> {
    let a4547 = if match v0.clone() {
        Adt7::C0(_, _, _, h4543, _) => {
            h4543
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4548 = {
        let a4545 = match v0 {
            Adt7::C0(_, _, _, _, h4544) => {
                h4544
            }
        };
        let a4546 = 1u64;
        if nat_eq(a4545, a4546) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4547, a4548)
}

pub fn f175(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4553 = {
            let a4550 = v2;
            let a4551 = match v0.clone() {
                Adt6::C0(_, h4549, _, _) => {
                    h4549
                }
            };
            nat_mul(a4550, a4551)?
        };
        let a4554 = f132(match v0.clone() {
            Adt6::C0(_, _, h4552, _) => {
                h4552
            }
        });
        nat_add(a4553, a4554)?
    };
    let v4: u64 = f133(match v0.clone() {
        Adt6::C0(_, _, h4555, _) => {
            h4555
        }
    });
    let v5: u64 = {
        let a4559 = {
            let a4557 = v3;
            let a4558 = match v0 {
                Adt6::C0(h4556, _, _, _) => {
                    h4556
                }
            };
            nat_add(a4557, a4558)?
        };
        let a4560 = v4;
        nat_sub(a4559, a4560)
    };
    let a4565 = {
        let a4561 = v5;
        let a4562 = v1;
        if nat_lt(a4561, a4562) {
            v5
        } else {
            v1
        }
    };
    let a4566 = {
        let a4563 = v3;
        let a4564 = v4;
        nat_sub(a4563, a4564)
    };
    Ok(nat_sub(a4565, a4566))
}

pub fn f176(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4575 = f56(match v0.clone() {
        Adt7::C0(h4567, _, _, _, _) => {
            h4567
        }
    }, match v1.clone() {
        Adt3::C0(h4568, _, _, _, _, _, _, _) => {
            h4568
        }
    }, f54(v3, v2.clone(), 0u64), {
        let a4572 = v4;
        let a4573 = {
            let m4571 = match v0.clone() {
                Adt7::C0(h4570, _, _, _, _) => {
                    h4570
                }
            };
            match m4571 {
                Adt6::C0(h4569, _, _, _) => {
                    h4569
                }
            }
        };
        let a4574 = 0u64;
        nat_rem(a4572, a4573, a4574)
    })?;
    match m4575 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4584 = f56(match v0.clone() {
                Adt7::C0(_, h4576, _, _, _) => {
                    h4576
                }
            }, match v1.clone() {
                Adt3::C0(_, h4577, _, _, _, _, _, _) => {
                    h4577
                }
            }, f54(v3, v2.clone(), 1u64), {
                let a4581 = v4;
                let a4582 = {
                    let m4580 = match v0 {
                        Adt7::C0(h4579, _, _, _, _) => {
                            h4579
                        }
                    };
                    match m4580 {
                        Adt6::C0(h4578, _, _, _) => {
                            h4578
                        }
                    }
                };
                let a4583 = 0u64;
                nat_quot(a4581, a4582, a4583)
            })?;
            match m4584 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f53(v1, v5, v6, f54(v3, v2.clone(), 2u64), f54(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f177(v0: u64) -> R<u64> {
    let a4587 = 2u64;
    let a4588 = {
        let a4585 = v0;
        let a4586 = 2u64;
        nat_mul(a4585, a4586)?
    };
    nat_add(a4587, a4588)
}

pub fn f178(v0: Adt6) -> R<u64> {
    let a4601 = 2u64;
    let a4602 = {
        let a4599 = 2u64;
        let a4600 = {
            let a4596 = {
                let a4593 = match v0.clone() {
                    Adt6::C0(h4589, _, _, _) => {
                        h4589
                    }
                };
                let a4594 = {
                    let a4591 = match v0.clone() {
                        Adt6::C0(_, h4590, _, _) => {
                            h4590
                        }
                    };
                    let a4592 = 1u64;
                    nat_sub(a4591, a4592)
                };
                nat_add(a4593, a4594)?
            };
            let a4597 = match v0 {
                Adt6::C0(_, h4595, _, _) => {
                    h4595
                }
            };
            let a4598 = 0u64;
            nat_quot(a4596, a4597, a4598)
        };
        nat_mul(a4599, a4600)?
    };
    nat_add(a4601, a4602)
}

pub fn f179(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m4603 = v0;
    match m4603 {
        Adt16::C0(v19) => {
            f241(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4604 = v20;
            let a4605 = 66u64;
            if nat_eq(a4604, a4605) {
                f242(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f243(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f180(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4612 = v3;
        let a4613 = {
            let a4609 = f84({
                let a4606 = v0;
                let a4607 = 2u64;
                let a4608 = 0u64;
                nat_quot(a4606, a4607, a4608)
            }, v1)?;
            let a4610 = v2;
            let a4611 = 0u64;
            nat_quot(a4609, a4610, a4611)
        };
        nat_add(a4612, a4613)?
    };
    let a4617 = {
        let a4614 = v0;
        let a4615 = 2u64;
        let a4616 = 0u64;
        nat_rem(a4614, a4615, a4616)
    };
    let a4618 = 0u64;
    if nat_eq(a4617, a4618) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4619 = v4;
        let a4620 = 0u64;
        if nat_eq(a4619, a4620) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f181(v0: u64, v1: u64) -> R<u64> {
    let a4623 = 8u64;
    let a4624 = {
        let a4621 = f244(v0);
        let a4622 = v1;
        nat_mul(a4621, a4622)?
    };
    nat_add(a4623, a4624)
}

pub fn f182(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4625, _, _, _, _, _, _, _) => {
            h4625
        }
    };
    let v7: u64 = f244(v3);
    let v8: u64 = {
        let a4628 = 1u64;
        let a4629 = {
            let a4626 = v6;
            let a4627 = v7;
            nat_mul(a4626, a4627)?
        };
        nat_add(a4628, a4629)?
    };
    let a4630 = v0;
    let a4631 = 0u64;
    if nat_eq(a4630, a4631) {
        Ok(f245(0u64))
    } else {
        let a4632 = v0;
        let a4633 = v8;
        if nat_lt(a4632, a4633) {
            let v9: u64 = {
                let a4634 = v0;
                let a4635 = 1u64;
                nat_sub(a4634, a4635)
            };
            let v10: u64 = {
                let a4636 = v9;
                let a4637 = v7;
                let a4638 = 0u64;
                nat_rem(a4636, a4637, a4638)
            };
            let v11: u64 = f84({
                let a4644 = {
                    let a4639 = v1;
                    let a4640 = v6;
                    nat_mul(a4639, a4640)?
                };
                let a4645 = {
                    let a4641 = v9;
                    let a4642 = v7;
                    let a4643 = 0u64;
                    nat_quot(a4641, a4642, a4643)
                };
                nat_add(a4644, a4645)?
            }, v2)?;
            let a4646 = v3;
            let a4647 = 30u64;
            if nat_eq(a4646, a4647) {
                let a4648 = v10;
                let a4649 = 0u64;
                if nat_eq(a4648, a4649) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4650 = v10;
                    let a4651 = 1u64;
                    if nat_eq(a4650, a4651) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4652 = v10;
                        let a4653 = 2u64;
                        if nat_eq(a4652, a4653) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4654 = v10;
                let a4655 = 0u64;
                if nat_eq(a4654, a4655) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4656 = v10;
                    let a4657 = 1u64;
                    if nat_eq(a4656, a4657) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4658 = v0;
                let a4659 = v8;
                nat_sub(a4658, a4659)
            };
            let a4660 = v12;
            let a4661 = 0u64;
            if nat_eq(a4660, a4661) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4662 = v12;
                let a4663 = 1u64;
                if nat_eq(a4662, a4663) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4664 = v12;
                    let a4665 = 2u64;
                    if nat_eq(a4664, a4665) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4666 = v12;
                        let a4667 = 3u64;
                        if nat_eq(a4666, a4667) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4668 = v12;
                            let a4669 = 4u64;
                            if nat_eq(a4668, a4669) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4670 = v12;
                                let a4671 = 5u64;
                                if nat_eq(a4670, a4671) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4672 = v12;
                                    let a4673 = 6u64;
                                    if nat_eq(a4672, a4673) {
                                        if f246(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4674 = 3u64;
                                                let a4675 = v1;
                                                nat_add(a4674, a4675)?
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
        let a4676 = v0;
        let a4677 = f95(v3);
        let a4678 = 0u64;
        nat_quot(a4676, a4677, a4678)
    };
    let v5: u64 = {
        let a4683 = 3u64;
        let a4684 = {
            let a4680 = v4;
            let a4681 = match v1.clone() {
                Adt3::C0(h4679, _, _, _, _, _, _, _) => {
                    h4679
                }
            };
            let a4682 = 0u64;
            nat_quot(a4680, a4681, a4682)
        };
        nat_add(a4683, a4684)?
    };
    let v6: u64 = f84(v4, v1)?;
    let v7: u64 = f84(v4, v2)?;
    let a4685 = v3;
    let a4686 = 30u64;
    if nat_eq(a4685, a4686) {
        let a4690 = {
            let a4687 = v0;
            let a4688 = 3u64;
            let a4689 = 0u64;
            nat_rem(a4687, a4688, a4689)
        };
        let a4691 = 0u64;
        if nat_eq(a4690, a4691) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4695 = {
                let a4692 = v0;
                let a4693 = 3u64;
                let a4694 = 0u64;
                nat_rem(a4692, a4693, a4694)
            };
            let a4696 = 1u64;
            if nat_eq(a4695, a4696) {
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
        let a4697 = v0;
        let a4698 = 8388608u64;
        let a4699 = 0u64;
        nat_quot(a4697, a4698, a4699)
    };
    let a4700 = v0;
    let a4701 = 2147483648u64;
    if if if nat_lt(a4700, a4701) {
        let a4702 = 127u64;
        let a4703 = v1;
        nat_le(a4702, a4703)
    } else {
        false
    } {
        let a4704 = v1;
        let a4705 = 190u64;
        nat_lt(a4704, a4705)
    } else {
        false
    } {
        let v2: u64 = {
            let a4709 = 8388608u64;
            let a4710 = {
                let a4706 = v0;
                let a4707 = 8388608u64;
                let a4708 = 0u64;
                nat_rem(a4706, a4707, a4708)
            };
            nat_add(a4709, a4710)?
        };
        let a4711 = v1;
        let a4712 = 150u64;
        if nat_lt(a4711, a4712) {
            let v3: u64 = f247({
                let a4713 = 150u64;
                let a4714 = v1;
                nat_sub(a4713, a4714)
            });
            let a4718 = {
                let a4715 = v2;
                let a4716 = v3;
                let a4717 = 0u64;
                nat_rem(a4715, a4716, a4717)
            };
            let a4719 = 0u64;
            if nat_eq(a4718, a4719) {
                let a4720 = v2;
                let a4721 = v3;
                let a4722 = 0u64;
                Ok(nat_quot(a4720, a4721, a4722))
            } else {
                Ok(0u64)
            }
        } else {
            let a4725 = f247({
                let a4723 = v1;
                let a4724 = 150u64;
                nat_sub(a4723, a4724)
            });
            let a4726 = v2;
            nat_mul(a4725, a4726)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f185(v0: Adt14, v1: Adt14) -> bool {
    let a4729 = match v0.clone() {
        Adt14::C0(_, h4727, _, _, _, _, _, _, _, _, _) => {
            h4727
        }
    };
    let a4730 = match v1.clone() {
        Adt14::C0(_, h4728, _, _, _, _, _, _, _, _, _) => {
            h4728
        }
    };
    if if if nat_eq(a4729, a4730) {
        let a4733 = match v0.clone() {
            Adt14::C0(_, _, h4731, _, _, _, _, _, _, _, _) => {
                h4731
            }
        };
        let a4734 = match v1.clone() {
            Adt14::C0(_, _, h4732, _, _, _, _, _, _, _, _) => {
                h4732
            }
        };
        nat_eq(a4733, a4734)
    } else {
        false
    } {
        let a4737 = match v0.clone() {
            Adt14::C0(_, _, _, h4735, _, _, _, _, _, _, _) => {
                h4735
            }
        };
        let a4738 = match v1.clone() {
            Adt14::C0(_, _, _, h4736, _, _, _, _, _, _, _) => {
                h4736
            }
        };
        nat_eq(a4737, a4738)
    } else {
        false
    } {
        let a4741 = match v0 {
            Adt14::C0(_, _, _, _, h4739, _, _, _, _, _, _) => {
                h4739
            }
        };
        let a4742 = match v1 {
            Adt14::C0(_, _, _, _, h4740, _, _, _, _, _, _) => {
                h4740
            }
        };
        nat_eq(a4741, a4742)
    } else {
        false
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4743 = v0;
    let a4744 = 0u64;
    if if if nat_eq(a4743, a4744) {
        true
    } else {
        let a4745 = v1;
        let a4746 = 0u64;
        nat_eq(a4745, a4746)
    } {
        true
    } else {
        let a4747 = v2;
        let a4748 = 0u64;
        nat_eq(a4747, a4748)
    } {
        true
    } else {
        let a4749 = v3;
        let a4750 = 0u64;
        nat_eq(a4749, a4750)
    }
}

pub fn f187(v0: Adt14, v1: Adt14) -> bool {
    if f193(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4751, _, _, _, _, _) => {
            h4751
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4752, _, _, _, _, _) => {
            h4752
        }
    }) {
        let a4754 = match v0 {
            Adt14::C0(_, h4753, _, _, _, _, _, _, _, _, _) => {
                h4753
            }
        };
        let a4755 = 2147483647u64;
        nat_le(a4754, a4755)
    } else {
        true
    }
}

pub fn f188(v0: Adt14) -> bool {
    let a4760 = {
        let a4757 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4756) => {
                h4756
            }
        };
        let a4758 = 4u64;
        let a4759 = 0u64;
        nat_rem(a4757, a4758, a4759)
    };
    let a4761 = 0u64;
    if if nat_eq(a4760, a4761) {
        f248(match v0.clone() {
            Adt14::C0(_, h4762, _, _, _, _, _, _, _, _, _) => {
                h4762
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4763, _, _, _, _, _) => {
                h4763
            }
        })
    } else {
        false
    } {
        f91(match v0.clone() {
            Adt14::C0(_, _, h4764, _, _, _, _, _, _, _, _) => {
                h4764
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4765, _, _, _, _, _, _, _) => {
                h4765
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4766, _, _, _, _, _, _) => {
                h4766
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4767, _, _, _, _) => {
                h4767
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4768, _, _, _) => {
                h4768
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4769, _, _) => {
                h4769
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f189(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4771 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4770, _) => {
            h4770
        }
    };
    let a4772 = 0u64;
    if nat_eq(a4771, a4772) {
        Ok(true)
    } else {
        let a4774 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4773, _) => {
                h4773
            }
        };
        let a4775 = 0u64;
        if nat_eq(a4774, a4775) {
            Ok(true)
        } else {
            let a4777 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4776) => {
                    h4776
                }
            };
            let a4778 = 0u64;
            if nat_eq(a4777, a4778) {
                Ok(true)
            } else {
                let a4780 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4779) => {
                        h4779
                    }
                };
                let a4781 = 0u64;
                if nat_eq(a4780, a4781) {
                    Ok(true)
                } else {
                    let a4787 = {
                        let a4784 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4782) => {
                                h4782
                            }
                        };
                        let a4785 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4783, _) => {
                                h4783
                            }
                        };
                        nat_add(a4784, a4785)?
                    };
                    let a4788 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4786) => {
                            h4786
                        }
                    };
                    if nat_le(a4787, a4788) {
                        Ok(true)
                    } else {
                        let a4794 = {
                            let a4791 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4789) => {
                                    h4789
                                }
                            };
                            let a4792 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4790, _) => {
                                    h4790
                                }
                            };
                            nat_add(a4791, a4792)?
                        };
                        let a4795 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4793) => {
                                h4793
                            }
                        };
                        Ok(nat_le(a4794, a4795))
                    }
                }
            }
        }
    }
}

pub fn f190(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f185(v0.clone(), v1.clone()) {
        let a4798 = match v0.clone() {
            Adt14::C0(h4796, _, _, _, _, _, _, _, _, _, _) => {
                h4796
            }
        };
        let a4799 = match v1.clone() {
            Adt14::C0(h4797, _, _, _, _, _, _, _, _, _, _) => {
                h4797
            }
        };
        nat_eq(a4798, a4799)
    } else {
        false
    } {
        let a4802 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4800) => {
                h4800
            }
        };
        let a4803 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4801) => {
                h4801
            }
        };
        nat_eq(a4802, a4803)
    } else {
        false
    } {
        let a4806 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4804, _, _, _, _, _) => {
                h4804
            }
        };
        let a4807 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4805, _, _, _, _, _) => {
                h4805
            }
        };
        nat_eq(a4806, a4807)
    } else {
        false
    } {
        let a4810 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4808, _, _, _, _) => {
                h4808
            }
        };
        let a4811 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4809, _, _, _, _) => {
                h4809
            }
        };
        nat_eq(a4810, a4811)
    } else {
        false
    } {
        let a4814 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4812, _, _, _) => {
                h4812
            }
        };
        let a4815 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4813, _, _, _) => {
                h4813
            }
        };
        nat_eq(a4814, a4815)
    } else {
        false
    } {
        let a4818 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4816, _, _) => {
                h4816
            }
        };
        let a4819 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4817, _, _) => {
                h4817
            }
        };
        nat_eq(a4818, a4819)
    } else {
        false
    }
}

pub fn f191(v0: Adt14) -> R<bool> {
    if f249(match v0.clone() {
        Adt14::C0(_, h4820, _, _, _, _, _, _, _, _, _) => {
            h4820
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4821, _, _, _, _, _) => {
            h4821
        }
    }) {
        f92(f250(match v0.clone() {
            Adt14::C0(_, h4822, _, _, _, _, _, _, _, _, _) => {
                h4822
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4823, _, _, _, _, _) => {
                h4823
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4824, _, _, _, _, _, _, _, _) => {
                h4824
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4825, _, _, _, _, _, _, _) => {
                h4825
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4826, _, _, _, _, _, _) => {
                h4826
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4827, _, _, _, _) => {
                h4827
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4828, _, _, _) => {
                h4828
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4829, _, _) => {
                h4829
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f192(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4830 = v1;
    let a4831 = 0u64;
    if if nat_eq(a4830, a4831) {
        true
    } else {
        let a4832 = v2;
        let a4833 = 0u64;
        nat_eq(a4832, a4833)
    } {
        Ok(0u64)
    } else {
        let a4836 = f93(v1, v2, v3)?;
        let a4837 = {
            let a4834 = v2;
            let a4835 = f251(v0, v1)?;
            nat_mul(a4834, a4835)?
        };
        nat_add(a4836, a4837)
    }
}

pub fn f193(v0: u64, v1: u64) -> bool {
    let a4838 = v0;
    let a4839 = 4u64;
    if nat_eq(a4838, a4839) {
        let a4840 = v1;
        let a4841 = 4u64;
        nat_eq(a4840, a4841)
    } else {
        false
    }
}

pub fn f194(v0: u64) -> R<u64> {
    let a4842 = 6u64;
    let a4843 = v0;
    nat_add(a4842, a4843)
}

pub fn f195(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4844, _, _, _, _, _, _, _, _, _, _) => {
            h4844
        }
    }, match v0.clone() {
        Adt14::C0(_, h4845, _, _, _, _, _, _, _, _, _) => {
            h4845
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4846, _, _, _, _, _, _, _, _) => {
            h4846
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4847, _, _, _, _, _, _, _) => {
            h4847
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4848, _, _, _, _, _, _) => {
            h4848
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4849, _, _, _, _) => {
            h4849
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4850, _, _, _) => {
            h4850
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4851, _, _) => {
            h4851
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4852, _) => {
            h4852
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4853) => {
            h4853
        }
    })
}

pub fn f196(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4854 = v0;
    let a4855 = 2147483647u64;
    if if if nat_le(a4854, a4855) {
        let a4856 = 0u64;
        let a4857 = v3;
        if nat_lt(a4856, a4857) {
            let a4858 = v3;
            let a4859 = 2147483647u64;
            nat_le(a4858, a4859)
        } else {
            false
        }
    } else {
        false
    } {
        let a4864 = {
            let a4862 = {
                let a4860 = v2;
                let a4861 = v3;
                nat_add(a4860, a4861)?
            };
            let a4863 = 1u64;
            nat_sub(a4862, a4863)
        };
        let a4865 = 2147483647u64;
        nat_le(a4864, a4865)
    } else {
        false
    } {
        let a4870 = {
            let a4868 = {
                let a4866 = v0;
                let a4867 = v1;
                nat_mul(a4866, a4867)?
            };
            let a4869 = f252(v2, v3)?;
            nat_mul(a4868, a4869)?
        };
        let a4871 = 9223372036854775807u64;
        Ok(nat_le(a4870, a4871))
    } else {
        Ok(false)
    }
}

pub fn f197(v0: Adt14) -> R<u64> {
    let a4902 = {
        let a4894 = {
            let a4886 = {
                let a4878 = 4u64;
                let a4879 = {
                    let a4876 = {
                        let a4873 = match v0.clone() {
                            Adt14::C0(_, h4872, _, _, _, _, _, _, _, _, _) => {
                                h4872
                            }
                        };
                        let a4874 = 1u64;
                        nat_sub(a4873, a4874)
                    };
                    let a4877 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4875, _, _, _, _, _) => {
                            h4875
                        }
                    };
                    nat_mul(a4876, a4877)?
                };
                nat_add(a4878, a4879)?
            };
            let a4887 = {
                let a4884 = {
                    let a4881 = match v0.clone() {
                        Adt14::C0(_, _, h4880, _, _, _, _, _, _, _, _) => {
                            h4880
                        }
                    };
                    let a4882 = 1u64;
                    nat_sub(a4881, a4882)
                };
                let a4885 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4883, _, _, _, _) => {
                        h4883
                    }
                };
                nat_mul(a4884, a4885)?
            };
            nat_add(a4886, a4887)?
        };
        let a4895 = {
            let a4892 = {
                let a4889 = match v0.clone() {
                    Adt14::C0(_, _, _, h4888, _, _, _, _, _, _, _) => {
                        h4888
                    }
                };
                let a4890 = 1u64;
                nat_sub(a4889, a4890)
            };
            let a4893 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4891, _, _, _) => {
                    h4891
                }
            };
            nat_mul(a4892, a4893)?
        };
        nat_add(a4894, a4895)?
    };
    let a4903 = {
        let a4900 = {
            let a4897 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4896, _, _, _, _, _, _) => {
                    h4896
                }
            };
            let a4898 = 1u64;
            nat_sub(a4897, a4898)
        };
        let a4901 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4899, _, _) => {
                h4899
            }
        };
        nat_mul(a4900, a4901)?
    };
    nat_add(a4902, a4903)
}

pub fn f198(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4904 = v0;
    let a4905 = 0u64;
    if if if if nat_eq(a4904, a4905) {
        true
    } else {
        let a4906 = v1;
        let a4907 = 0u64;
        nat_eq(a4906, a4907)
    } {
        true
    } else {
        let a4908 = v2;
        let a4909 = 0u64;
        nat_eq(a4908, a4909)
    } {
        true
    } else {
        let a4910 = v3;
        let a4911 = 0u64;
        nat_eq(a4910, a4911)
    } {
        Ok(0u64)
    } else {
        let a4912 = f253(v0, v1, v2, v3, v5)?;
        let a4913 = f254(v0, v1, v2, v3, v4)?;
        nat_add(a4912, a4913)
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4914 = v0;
    let a4915 = 0u64;
    if if nat_eq(a4914, a4915) {
        true
    } else {
        let a4916 = v1;
        let a4917 = 0u64;
        nat_eq(a4916, a4917)
    } {
        Ok(0u64)
    } else {
        let a4920 = f93(v0, v1, v2)?;
        let a4921 = {
            let a4918 = v1;
            let a4919 = f255(v0)?;
            nat_mul(a4918, a4919)?
        };
        nat_add(a4920, a4921)
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> bool {
    let a4922 = v0;
    let a4923 = 1u64;
    if nat_le(a4922, a4923) {
        true
    } else {
        let a4927 = {
            let a4924 = v1;
            let a4925 = v2;
            let a4926 = 1u64;
            nat_rem(a4924, a4925, a4926)
        };
        let a4928 = 0u64;
        nat_eq(a4927, a4928)
    }
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4929 = v1;
    let a4930 = 1u64;
    if if if nat_le(a4929, a4930) {
        true
    } else {
        let a4931 = v0;
        let a4932 = v2;
        nat_le(a4931, a4932)
    } {
        let a4933 = v3;
        let a4934 = 1u64;
        if nat_le(a4933, a4934) {
            true
        } else {
            let a4941 = {
                let a4939 = v0;
                let a4940 = {
                    let a4937 = {
                        let a4935 = v1;
                        let a4936 = 1u64;
                        nat_sub(a4935, a4936)
                    };
                    let a4938 = v2;
                    nat_mul(a4937, a4938)?
                };
                nat_add(a4939, a4940)?
            };
            let a4942 = v4;
            nat_le(a4941, a4942)
        }
    } else {
        false
    } {
        let a4943 = v5;
        let a4944 = 1u64;
        if nat_le(a4943, a4944) {
            Ok(true)
        } else {
            let a4957 = {
                let a4955 = {
                    let a4949 = v0;
                    let a4950 = {
                        let a4947 = {
                            let a4945 = v1;
                            let a4946 = 1u64;
                            nat_sub(a4945, a4946)
                        };
                        let a4948 = v2;
                        nat_mul(a4947, a4948)?
                    };
                    nat_add(a4949, a4950)?
                };
                let a4956 = {
                    let a4953 = {
                        let a4951 = v3;
                        let a4952 = 1u64;
                        nat_sub(a4951, a4952)
                    };
                    let a4954 = v4;
                    nat_mul(a4953, a4954)?
                };
                nat_add(a4955, a4956)?
            };
            let a4958 = v6;
            Ok(nat_le(a4957, a4958))
        }
    } else {
        Ok(false)
    }
}

pub fn f202(v0: u64) -> Adt18 {
    let a4959 = v0;
    let a4960 = 0u64;
    if nat_eq(a4959, a4960) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a4961 = v0;
        let a4962 = 1u64;
        if nat_eq(a4961, a4962) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a4963 = v0;
            let a4964 = 2u64;
            if nat_eq(a4963, a4964) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a4965 = v0;
                let a4966 = 3u64;
                if nat_eq(a4965, a4966) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a4967 = v0;
                    let a4968 = 6u64;
                    if nat_eq(a4967, a4968) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a4969 = v0;
                        let a4970 = 7u64;
                        if nat_eq(a4969, a4970) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a4971 = v0;
                            let a4972 = 8u64;
                            if nat_eq(a4971, a4972) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a4973 = v0;
                                let a4974 = 9u64;
                                if nat_eq(a4973, a4974) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a4975 = v0;
                                    let a4976 = 10u64;
                                    if nat_eq(a4975, a4976) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a4977 = v0;
                                        let a4978 = 11u64;
                                        if nat_eq(a4977, a4978) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a4979 = v0;
                                            let a4980 = 12u64;
                                            if nat_eq(a4979, a4980) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a4981 = v0;
                                                let a4982 = 13u64;
                                                if nat_eq(a4981, a4982) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a4983 = v0;
                                                    let a4984 = 14u64;
                                                    if nat_eq(a4983, a4984) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a4985 = v0;
                                                        let a4986 = 15u64;
                                                        if nat_eq(a4985, a4986) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a4987 = v0;
                                                            let a4988 = 16u64;
                                                            if nat_eq(a4987, a4988) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a4989 = v0;
                                                                let a4990 = 17u64;
                                                                if nat_eq(a4989, a4990) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a4991 = v0;
                                                                    let a4992 = 18u64;
                                                                    if nat_eq(a4991, a4992) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a4993 = v0;
                                                                        let a4994 = 19u64;
                                                                        if nat_eq(a4993, a4994) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a4995 = v0;
                                                                            let a4996 = 20u64;
                                                                            if nat_eq(a4995, a4996) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a4997 = v0;
                                                                                let a4998 = 21u64;
                                                                                if nat_eq(a4997, a4998) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a4999 = v0;
                                                                                    let a5000 = 22u64;
                                                                                    if nat_eq(a4999, a5000) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5001 = v0;
                                                                                        let a5002 = 23u64;
                                                                                        if nat_eq(a5001, a5002) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5003 = v0;
                                                                                            let a5004 = 24u64;
                                                                                            if nat_eq(a5003, a5004) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5005 = v0;
                                                                                                let a5006 = 25u64;
                                                                                                if nat_eq(a5005, a5006) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5007 = v0;
                                                                                                    let a5008 = 26u64;
                                                                                                    if nat_eq(a5007, a5008) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5009 = v0;
                                                                                                        let a5010 = 27u64;
                                                                                                        if nat_eq(a5009, a5010) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5011 = v0;
                                                                                                            let a5012 = 28u64;
                                                                                                            if nat_eq(a5011, a5012) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5013 = v0;
                                                                                                                let a5014 = 29u64;
                                                                                                                if nat_eq(a5013, a5014) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5015 = v0;
                                                                                                                    let a5016 = 30u64;
                                                                                                                    if nat_eq(a5015, a5016) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5017 = v0;
                                                                                                                        let a5018 = 34u64;
                                                                                                                        if nat_eq(a5017, a5018) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5019 = v0;
                                                                                                                            let a5020 = 35u64;
                                                                                                                            if nat_eq(a5019, a5020) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5021 = v0;
                                                                                                                                let a5022 = 39u64;
                                                                                                                                if nat_eq(a5021, a5022) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5023 = v0;
                                                                                                                                    let a5024 = 40u64;
                                                                                                                                    if nat_eq(a5023, a5024) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5025 = v0;
                                                                                                                                        let a5026 = 41u64;
                                                                                                                                        if nat_eq(a5025, a5026) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5027 = v0;
                                                                                                                                            let a5028 = 42u64;
                                                                                                                                            if nat_eq(a5027, a5028) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5029 = v0;
                                                                                                                                                let a5030 = 43u64;
                                                                                                                                                if nat_eq(a5029, a5030) {
                                                                                                                                                    Adt18::C1(Adt19::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt18::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f203(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5032 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5031, _, _, _, _, _, _) => {
                    h5031
                }
            };
            let a5033 = 1u64;
            if if if if if nat_le(a5032, a5033) {
                true
            } else {
                let a5035 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5034, _, _) => {
                        h5034
                    }
                };
                let a5036 = v1;
                nat_eq(a5035, a5036)
            } {
                let a5038 = match v0.clone() {
                    Adt2::C0(_, h5037, _, _, _, _, _, _, _, _, _) => {
                        h5037
                    }
                };
                let a5039 = 1u64;
                if nat_le(a5038, a5039) {
                    true
                } else {
                    let a5044 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5040, _, _, _, _, _) => {
                            h5040
                        }
                    };
                    let a5045 = {
                        let a5042 = v1;
                        let a5043 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5041, _, _, _, _, _, _) => {
                                h5041
                            }
                        };
                        nat_mul(a5042, a5043)?
                    };
                    nat_eq(a5044, a5045)
                }
            } else {
                false
            } {
                let a5047 = match v0.clone() {
                    Adt2::C0(_, _, h5046, _, _, _, _, _, _, _, _) => {
                        h5046
                    }
                };
                let a5048 = 1u64;
                if nat_le(a5047, a5048) {
                    true
                } else {
                    let a5056 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5049, _, _, _, _) => {
                            h5049
                        }
                    };
                    let a5057 = {
                        let a5054 = {
                            let a5051 = v1;
                            let a5052 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5050, _, _, _, _, _, _) => {
                                    h5050
                                }
                            };
                            nat_mul(a5051, a5052)?
                        };
                        let a5055 = match v0.clone() {
                            Adt2::C0(_, h5053, _, _, _, _, _, _, _, _, _) => {
                                h5053
                            }
                        };
                        nat_mul(a5054, a5055)?
                    };
                    nat_eq(a5056, a5057)
                }
            } else {
                false
            } {
                let a5059 = match v0.clone() {
                    Adt2::C0(_, _, _, h5058, _, _, _, _, _, _, _) => {
                        h5058
                    }
                };
                let a5060 = 1u64;
                if nat_le(a5059, a5060) {
                    true
                } else {
                    let a5071 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5061, _, _, _) => {
                            h5061
                        }
                    };
                    let a5072 = {
                        let a5069 = {
                            let a5066 = {
                                let a5063 = v1;
                                let a5064 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5062, _, _, _, _, _, _) => {
                                        h5062
                                    }
                                };
                                nat_mul(a5063, a5064)?
                            };
                            let a5067 = match v0.clone() {
                                Adt2::C0(_, h5065, _, _, _, _, _, _, _, _, _) => {
                                    h5065
                                }
                            };
                            nat_mul(a5066, a5067)?
                        };
                        let a5070 = match v0.clone() {
                            Adt2::C0(_, _, h5068, _, _, _, _, _, _, _, _) => {
                                h5068
                            }
                        };
                        nat_mul(a5069, a5070)?
                    };
                    nat_eq(a5071, a5072)
                }
            } else {
                false
            } {
                let a5086 = {
                    let a5083 = {
                        let a5080 = {
                            let a5077 = {
                                let a5074 = v1;
                                let a5075 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5073, _, _, _, _, _, _) => {
                                        h5073
                                    }
                                };
                                nat_mul(a5074, a5075)?
                            };
                            let a5078 = match v0.clone() {
                                Adt2::C0(_, h5076, _, _, _, _, _, _, _, _, _) => {
                                    h5076
                                }
                            };
                            nat_mul(a5077, a5078)?
                        };
                        let a5081 = match v0.clone() {
                            Adt2::C0(_, _, h5079, _, _, _, _, _, _, _, _) => {
                                h5079
                            }
                        };
                        nat_mul(a5080, a5081)?
                    };
                    let a5084 = match v0.clone() {
                        Adt2::C0(_, _, _, h5082, _, _, _, _, _, _, _) => {
                            h5082
                        }
                    };
                    nat_mul(a5083, a5084)?
                };
                let a5087 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5085, _) => {
                        h5085
                    }
                };
                Ok(nat_le(a5086, a5087))
            } else {
                Ok(false)
            }
        } else {
            let a5089 = match v0.clone() {
                Adt2::C0(_, _, _, h5088, _, _, _, _, _, _, _) => {
                    h5088
                }
            };
            let a5090 = 1u64;
            if if if if if nat_le(a5089, a5090) {
                true
            } else {
                let a5092 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5091, _, _, _) => {
                        h5091
                    }
                };
                let a5093 = v1;
                nat_eq(a5092, a5093)
            } {
                let a5095 = match v0.clone() {
                    Adt2::C0(_, h5094, _, _, _, _, _, _, _, _, _) => {
                        h5094
                    }
                };
                let a5096 = 1u64;
                if nat_le(a5095, a5096) {
                    true
                } else {
                    let a5101 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5097, _, _, _, _, _) => {
                            h5097
                        }
                    };
                    let a5102 = {
                        let a5099 = v1;
                        let a5100 = match v0.clone() {
                            Adt2::C0(_, _, _, h5098, _, _, _, _, _, _, _) => {
                                h5098
                            }
                        };
                        nat_mul(a5099, a5100)?
                    };
                    nat_eq(a5101, a5102)
                }
            } else {
                false
            } {
                let a5104 = match v0.clone() {
                    Adt2::C0(_, _, h5103, _, _, _, _, _, _, _, _) => {
                        h5103
                    }
                };
                let a5105 = 1u64;
                if nat_le(a5104, a5105) {
                    true
                } else {
                    let a5113 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5106, _, _, _, _) => {
                            h5106
                        }
                    };
                    let a5114 = {
                        let a5111 = {
                            let a5108 = v1;
                            let a5109 = match v0.clone() {
                                Adt2::C0(_, _, _, h5107, _, _, _, _, _, _, _) => {
                                    h5107
                                }
                            };
                            nat_mul(a5108, a5109)?
                        };
                        let a5112 = match v0.clone() {
                            Adt2::C0(_, h5110, _, _, _, _, _, _, _, _, _) => {
                                h5110
                            }
                        };
                        nat_mul(a5111, a5112)?
                    };
                    nat_eq(a5113, a5114)
                }
            } else {
                false
            } {
                let a5116 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5115, _, _, _, _, _, _) => {
                        h5115
                    }
                };
                let a5117 = 1u64;
                if nat_le(a5116, a5117) {
                    true
                } else {
                    let a5128 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5118, _, _) => {
                            h5118
                        }
                    };
                    let a5129 = {
                        let a5126 = {
                            let a5123 = {
                                let a5120 = v1;
                                let a5121 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5119, _, _, _, _, _, _, _) => {
                                        h5119
                                    }
                                };
                                nat_mul(a5120, a5121)?
                            };
                            let a5124 = match v0.clone() {
                                Adt2::C0(_, h5122, _, _, _, _, _, _, _, _, _) => {
                                    h5122
                                }
                            };
                            nat_mul(a5123, a5124)?
                        };
                        let a5127 = match v0.clone() {
                            Adt2::C0(_, _, h5125, _, _, _, _, _, _, _, _) => {
                                h5125
                            }
                        };
                        nat_mul(a5126, a5127)?
                    };
                    nat_eq(a5128, a5129)
                }
            } else {
                false
            } {
                let a5143 = {
                    let a5140 = {
                        let a5137 = {
                            let a5134 = {
                                let a5131 = v1;
                                let a5132 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5130, _, _, _, _, _, _, _) => {
                                        h5130
                                    }
                                };
                                nat_mul(a5131, a5132)?
                            };
                            let a5135 = match v0.clone() {
                                Adt2::C0(_, h5133, _, _, _, _, _, _, _, _, _) => {
                                    h5133
                                }
                            };
                            nat_mul(a5134, a5135)?
                        };
                        let a5138 = match v0.clone() {
                            Adt2::C0(_, _, h5136, _, _, _, _, _, _, _, _) => {
                                h5136
                            }
                        };
                        nat_mul(a5137, a5138)?
                    };
                    let a5141 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5139, _, _, _, _, _, _) => {
                            h5139
                        }
                    };
                    nat_mul(a5140, a5141)?
                };
                let a5144 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5142, _) => {
                        h5142
                    }
                };
                Ok(nat_le(a5143, a5144))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5146 = match v0.clone() {
            Adt2::C0(_, h5145, _, _, _, _, _, _, _, _, _) => {
                h5145
            }
        };
        let a5147 = 1u64;
        if if if if if nat_le(a5146, a5147) {
            true
        } else {
            let a5149 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5148, _, _, _, _, _) => {
                    h5148
                }
            };
            let a5150 = v1;
            nat_eq(a5149, a5150)
        } {
            let a5152 = match v0.clone() {
                Adt2::C0(_, _, h5151, _, _, _, _, _, _, _, _) => {
                    h5151
                }
            };
            let a5153 = 1u64;
            if nat_le(a5152, a5153) {
                true
            } else {
                let a5158 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5154, _, _, _, _) => {
                        h5154
                    }
                };
                let a5159 = {
                    let a5156 = v1;
                    let a5157 = match v0.clone() {
                        Adt2::C0(_, h5155, _, _, _, _, _, _, _, _, _) => {
                            h5155
                        }
                    };
                    nat_mul(a5156, a5157)?
                };
                nat_eq(a5158, a5159)
            }
        } else {
            false
        } {
            let a5161 = match v0.clone() {
                Adt2::C0(_, _, _, h5160, _, _, _, _, _, _, _) => {
                    h5160
                }
            };
            let a5162 = 1u64;
            if nat_le(a5161, a5162) {
                true
            } else {
                let a5170 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5163, _, _, _) => {
                        h5163
                    }
                };
                let a5171 = {
                    let a5168 = {
                        let a5165 = v1;
                        let a5166 = match v0.clone() {
                            Adt2::C0(_, h5164, _, _, _, _, _, _, _, _, _) => {
                                h5164
                            }
                        };
                        nat_mul(a5165, a5166)?
                    };
                    let a5169 = match v0.clone() {
                        Adt2::C0(_, _, h5167, _, _, _, _, _, _, _, _) => {
                            h5167
                        }
                    };
                    nat_mul(a5168, a5169)?
                };
                nat_eq(a5170, a5171)
            }
        } else {
            false
        } {
            let a5173 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5172, _, _, _, _, _, _) => {
                    h5172
                }
            };
            let a5174 = 1u64;
            if nat_le(a5173, a5174) {
                true
            } else {
                let a5185 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5175, _, _) => {
                        h5175
                    }
                };
                let a5186 = {
                    let a5183 = {
                        let a5180 = {
                            let a5177 = v1;
                            let a5178 = match v0.clone() {
                                Adt2::C0(_, h5176, _, _, _, _, _, _, _, _, _) => {
                                    h5176
                                }
                            };
                            nat_mul(a5177, a5178)?
                        };
                        let a5181 = match v0.clone() {
                            Adt2::C0(_, _, h5179, _, _, _, _, _, _, _, _) => {
                                h5179
                            }
                        };
                        nat_mul(a5180, a5181)?
                    };
                    let a5184 = match v0.clone() {
                        Adt2::C0(_, _, _, h5182, _, _, _, _, _, _, _) => {
                            h5182
                        }
                    };
                    nat_mul(a5183, a5184)?
                };
                nat_eq(a5185, a5186)
            }
        } else {
            false
        } {
            let a5200 = {
                let a5197 = {
                    let a5194 = {
                        let a5191 = {
                            let a5188 = v1;
                            let a5189 = match v0.clone() {
                                Adt2::C0(_, h5187, _, _, _, _, _, _, _, _, _) => {
                                    h5187
                                }
                            };
                            nat_mul(a5188, a5189)?
                        };
                        let a5192 = match v0.clone() {
                            Adt2::C0(_, _, h5190, _, _, _, _, _, _, _, _) => {
                                h5190
                            }
                        };
                        nat_mul(a5191, a5192)?
                    };
                    let a5195 = match v0.clone() {
                        Adt2::C0(_, _, _, h5193, _, _, _, _, _, _, _) => {
                            h5193
                        }
                    };
                    nat_mul(a5194, a5195)?
                };
                let a5198 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5196, _, _, _, _, _, _) => {
                        h5196
                    }
                };
                nat_mul(a5197, a5198)?
            };
            let a5201 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5199, _) => {
                    h5199
                }
            };
            Ok(nat_le(a5200, a5201))
        } else {
            Ok(false)
        }
    }
}

pub fn f204(v0: Adt2, v1: u64) -> R<bool> {
    let a5203 = match v0.clone() {
        Adt2::C0(h5202, _, _, _, _, _, _, _, _, _, _) => {
            h5202
        }
    };
    let a5204 = 0u64;
    if if nat_eq(a5203, a5204) {
        f34(v0.clone())?
    } else {
        false
    } {
        let a5205 = v1;
        let a5206 = 1u64;
        if nat_le(a5205, a5206) {
            Ok(true)
        } else {
            let a5211 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5207, _, _, _, _) => {
                    h5207
                }
            };
            let a5212 = {
                let a5209 = 4u64;
                let a5210 = match v0 {
                    Adt2::C0(_, h5208, _, _, _, _, _, _, _, _, _) => {
                        h5208
                    }
                };
                nat_mul(a5209, a5210)?
            };
            Ok(nat_eq(a5211, a5212))
        }
    } else {
        Ok(false)
    }
}

pub fn f205(v0: Adt2, v1: u64) -> R<bool> {
    let a5214 = match v0.clone() {
        Adt2::C0(_, _, _, h5213, _, _, _, _, _, _, _) => {
            h5213
        }
    };
    let a5215 = 1u64;
    if if if nat_eq(a5214, a5215) {
        let a5217 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5216, _, _, _, _, _, _) => {
                h5216
            }
        };
        let a5218 = 1u64;
        nat_eq(a5217, a5218)
    } else {
        false
    } {
        let a5220 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5219, _, _, _, _, _) => {
                h5219
            }
        };
        let a5221 = v1;
        nat_eq(a5220, a5221)
    } else {
        false
    } {
        let a5223 = match v0.clone() {
            Adt2::C0(_, _, h5222, _, _, _, _, _, _, _, _) => {
                h5222
            }
        };
        let a5224 = 1u64;
        if nat_eq(a5223, a5224) {
            Ok(true)
        } else {
            let a5229 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5225, _, _, _, _) => {
                    h5225
                }
            };
            let a5230 = {
                let a5227 = match v0 {
                    Adt2::C0(_, h5226, _, _, _, _, _, _, _, _, _) => {
                        h5226
                    }
                };
                let a5228 = v1;
                nat_mul(a5227, a5228)?
            };
            Ok(nat_eq(a5229, a5230))
        }
    } else {
        Ok(false)
    }
}

pub fn f206(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5234 = {
            let a5231 = v2;
            let a5232 = 1u64;
            nat_sub(a5231, a5232)
        };
        let a5235 = match v0.clone() {
            Adt6::C0(_, h5233, _, _) => {
                h5233
            }
        };
        nat_mul(a5234, a5235)?
    };
    let a5237 = 0u64;
    let a5238 = match v0.clone() {
        Adt6::C0(h5236, _, _, _) => {
            h5236
        }
    };
    if if if if if if if if nat_lt(a5237, a5238) {
        let a5240 = match v0.clone() {
            Adt6::C0(h5239, _, _, _) => {
                h5239
            }
        };
        let a5241 = 2147483648u64;
        nat_lt(a5240, a5241)
    } else {
        false
    } {
        let a5243 = 0u64;
        let a5244 = match v0.clone() {
            Adt6::C0(_, h5242, _, _) => {
                h5242
            }
        };
        nat_lt(a5243, a5244)
    } else {
        false
    } {
        let a5246 = match v0.clone() {
            Adt6::C0(_, h5245, _, _) => {
                h5245
            }
        };
        let a5247 = 2147483648u64;
        nat_lt(a5246, a5247)
    } else {
        false
    } {
        let a5248 = v1;
        let a5249 = 2147483648u64;
        nat_lt(a5248, a5249)
    } else {
        false
    } {
        let a5250 = 0u64;
        let a5251 = v2;
        nat_lt(a5250, a5251)
    } else {
        false
    } {
        let a5252 = v3;
        let a5253 = 2147483648u64;
        nat_lt(a5252, a5253)
    } else {
        false
    } {
        let a5265 = {
            let a5260 = {
                let a5257 = v3;
                let a5258 = {
                    let a5255 = match v0.clone() {
                        Adt6::C0(h5254, _, _, _) => {
                            h5254
                        }
                    };
                    let a5256 = 1u64;
                    nat_sub(a5255, a5256)
                };
                nat_add(a5257, a5258)?
            };
            let a5261 = f132(match v0.clone() {
                Adt6::C0(_, _, h5259, _) => {
                    h5259
                }
            });
            nat_add(a5260, a5261)?
        };
        let a5266 = {
            let a5263 = 2147483647u64;
            let a5264 = f133(match v0.clone() {
                Adt6::C0(_, _, h5262, _) => {
                    h5262
                }
            });
            nat_add(a5263, a5264)?
        };
        nat_le(a5265, a5266)
    } else {
        false
    } {
        let a5268 = match v0 {
            Adt6::C0(_, _, h5267, _) => {
                h5267
            }
        };
        let a5269 = 2147483648u64;
        Ok(!nat_eq(a5268, a5269))
    } else {
        Ok(false)
    }
}

pub fn f207(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f256(match v0.clone() {
        Adt2::C0(h5270, _, _, _, _, _, _, _, _, _, _) => {
            h5270
        }
    }, match v0.clone() {
        Adt2::C0(_, h5271, _, _, _, _, _, _, _, _, _) => {
            h5271
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5272, _, _, _, _, _, _, _, _) => {
            h5272
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5273, _, _, _, _, _, _, _) => {
            h5273
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5274, _, _, _, _, _, _) => {
            h5274
        }
    }, match v1.clone() {
        Adt2::C0(h5275, _, _, _, _, _, _, _, _, _, _) => {
            h5275
        }
    }, match v1.clone() {
        Adt2::C0(_, h5276, _, _, _, _, _, _, _, _, _) => {
            h5276
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5277, _, _, _, _, _, _, _, _) => {
            h5277
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5278, _, _, _, _, _, _, _) => {
            h5278
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5279, _, _, _, _, _, _) => {
            h5279
        }
    }, match v2.clone() {
        Adt2::C0(h5280, _, _, _, _, _, _, _, _, _, _) => {
            h5280
        }
    }, match v2.clone() {
        Adt2::C0(_, h5281, _, _, _, _, _, _, _, _, _) => {
            h5281
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5282, _, _, _, _, _, _, _, _) => {
            h5282
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5283, _, _, _, _, _, _, _) => {
            h5283
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5284, _, _, _, _, _, _) => {
            h5284
        }
    }) {
        let m5286 = f202(match v0.clone() {
            Adt2::C0(h5285, _, _, _, _, _, _, _, _, _, _) => {
                h5285
            }
        });
        match m5286 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5292 = {
                    let a5289 = match v0.clone() {
                        Adt2::C0(_, h5287, _, _, _, _, _, _, _, _, _) => {
                            h5287
                        }
                    };
                    let a5290 = match v3.clone() {
                        Adt19::C0(_, h5288, _) => {
                            h5288
                        }
                    };
                    let a5291 = 0u64;
                    nat_rem(a5289, a5290, a5291)
                };
                let a5293 = 0u64;
                if if nat_eq(a5292, a5293) {
                    if f257(match v0.clone() {
                        Adt2::C0(_, h5294, _, _, _, _, _, _, _, _, _) => {
                            h5294
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5295, _, _, _, _, _, _, _, _, _) => {
                            h5295
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5296, _, _, _, _, _, _, _, _) => {
                            h5296
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5297, _, _, _, _, _, _, _) => {
                            h5297
                        }
                    }) {
                        true
                    } else {
                        let a5299 = 4u64;
                        let a5300 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5298, _, _, _, _, _) => {
                                h5298
                            }
                        };
                        if if nat_le(a5299, a5300) {
                            let a5302 = match v3.clone() {
                                Adt19::C0(_, h5301, _) => {
                                    h5301
                                }
                            };
                            let a5303 = 1u64;
                            if nat_eq(a5302, a5303) {
                                true
                            } else {
                                let a5306 = match v3.clone() {
                                    Adt19::C0(_, _, h5304) => {
                                        h5304
                                    }
                                };
                                let a5307 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5305, _, _, _, _, _) => {
                                        h5305
                                    }
                                };
                                nat_le(a5306, a5307)
                            }
                        } else {
                            false
                        } {
                            if f25(v0.clone(), v2.clone())? {
                                f25(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f24(v0.clone(), match v3 {
                        Adt19::C0(_, h5308, _) => {
                            h5308
                        }
                    });
                    Ok(Adt10::C4(if f257(match v0.clone() {
                        Adt2::C0(_, h5309, _, _, _, _, _, _, _, _, _) => {
                            h5309
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5310, _, _, _, _, _, _, _, _, _) => {
                            h5310
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5311, _, _, _, _, _, _, _, _) => {
                            h5311
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5312, _, _, _, _, _, _, _) => {
                            h5312
                        }
                    }) {
                        0u64
                    } else {
                        f166(v4.clone(), f24(v1.clone(), 1u64))?
                    }, 4u64, v4, f24(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5313, _, _, _, _, _, _, _, _, _, _) => {
                            h5313
                        }
                    }, f24(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f208(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f209(match v0.clone() {
        Adt2::C0(h5314, _, _, _, _, _, _, _, _, _, _) => {
            h5314
        }
    }) {
        let a5320 = {
            let a5317 = match v0.clone() {
                Adt2::C0(_, h5315, _, _, _, _, _, _, _, _, _) => {
                    h5315
                }
            };
            let a5318 = f239(match v0.clone() {
                Adt2::C0(h5316, _, _, _, _, _, _, _, _, _, _) => {
                    h5316
                }
            });
            let a5319 = 1u64;
            nat_rem(a5317, a5318, a5319)
        };
        let a5321 = 0u64;
        nat_eq(a5320, a5321)
    } else {
        false
    } {
        let a5323 = match v1.clone() {
            Adt2::C0(h5322, _, _, _, _, _, _, _, _, _, _) => {
                h5322
            }
        };
        let a5324 = 26u64;
        nat_eq(a5323, a5324)
    } else {
        false
    } {
        let a5326 = match v2.clone() {
            Adt2::C0(h5325, _, _, _, _, _, _, _, _, _, _) => {
                h5325
            }
        };
        let a5327 = 0u64;
        nat_eq(a5326, a5327)
    } else {
        false
    } {
        let a5330 = match v0.clone() {
            Adt2::C0(_, h5328, _, _, _, _, _, _, _, _, _) => {
                h5328
            }
        };
        let a5331 = match v2.clone() {
            Adt2::C0(_, h5329, _, _, _, _, _, _, _, _, _) => {
                h5329
            }
        };
        nat_eq(a5330, a5331)
    } else {
        false
    } {
        let a5334 = match v1.clone() {
            Adt2::C0(_, h5332, _, _, _, _, _, _, _, _, _) => {
                h5332
            }
        };
        let a5335 = match v0.clone() {
            Adt2::C0(_, _, h5333, _, _, _, _, _, _, _, _) => {
                h5333
            }
        };
        nat_le(a5334, a5335)
    } else {
        false
    } {
        let a5337 = match v0.clone() {
            Adt2::C0(_, _, _, h5336, _, _, _, _, _, _, _) => {
                h5336
            }
        };
        let a5338 = 1u64;
        nat_eq(a5337, a5338)
    } else {
        false
    } {
        let a5340 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5339, _, _, _, _, _, _) => {
                h5339
            }
        };
        let a5341 = 1u64;
        nat_eq(a5340, a5341)
    } else {
        false
    } {
        let a5343 = match v2.clone() {
            Adt2::C0(_, _, _, h5342, _, _, _, _, _, _, _) => {
                h5342
            }
        };
        let a5344 = 1u64;
        nat_eq(a5343, a5344)
    } else {
        false
    } {
        let a5346 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5345, _, _, _, _, _, _) => {
                h5345
            }
        };
        let a5347 = 1u64;
        nat_eq(a5346, a5347)
    } else {
        false
    } {
        let a5349 = match v1.clone() {
            Adt2::C0(_, _, h5348, _, _, _, _, _, _, _, _) => {
                h5348
            }
        };
        let a5350 = 1u64;
        nat_eq(a5349, a5350)
    } else {
        false
    } {
        let a5352 = match v1.clone() {
            Adt2::C0(_, _, _, h5351, _, _, _, _, _, _, _) => {
                h5351
            }
        };
        let a5353 = 1u64;
        nat_eq(a5352, a5353)
    } else {
        false
    } {
        let a5355 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5354, _, _, _, _, _, _) => {
                h5354
            }
        };
        let a5356 = 1u64;
        nat_eq(a5355, a5356)
    } else {
        false
    } {
        if f44(match v2.clone() {
            Adt2::C0(_, h5357, _, _, _, _, _, _, _, _, _) => {
                h5357
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5358, _, _, _, _, _, _, _, _) => {
                h5358
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5359, _, _, _, _, _, _, _) => {
                h5359
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5360, _, _, _, _, _, _) => {
                h5360
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5362 = match v1.clone() {
                Adt2::C0(_, h5361, _, _, _, _, _, _, _, _, _) => {
                    h5361
                }
            };
            let a5363 = 0u64;
            if nat_eq(a5362, a5363) {
                Ok(Adt10::C4(f20(v2.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5365 = match v0.clone() {
                    Adt2::C0(_, h5364, _, _, _, _, _, _, _, _, _) => {
                        h5364
                    }
                };
                let a5366 = 2147483647u64;
                if if if if if if nat_le(a5365, a5366) {
                    let a5368 = match v1.clone() {
                        Adt2::C0(_, h5367, _, _, _, _, _, _, _, _, _) => {
                            h5367
                        }
                    };
                    let a5369 = 2147483647u64;
                    nat_le(a5368, a5369)
                } else {
                    false
                } {
                    let a5371 = 4u64;
                    let a5372 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5370, _, _, _, _, _) => {
                            h5370
                        }
                    };
                    nat_le(a5371, a5372)
                } else {
                    false
                } {
                    let a5378 = {
                        let a5375 = match v2.clone() {
                            Adt2::C0(_, h5373, _, _, _, _, _, _, _, _, _) => {
                                h5373
                            }
                        };
                        let a5376 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5374, _, _, _, _, _) => {
                                h5374
                            }
                        };
                        nat_mul(a5375, a5376)?
                    };
                    let a5379 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5377, _, _, _, _) => {
                            h5377
                        }
                    };
                    nat_le(a5378, a5379)
                } else {
                    false
                } {
                    f25(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f25(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5392 = {
                            let a5383 = {
                                let a5381 = 2u64;
                                let a5382 = match v1.clone() {
                                    Adt2::C0(_, h5380, _, _, _, _, _, _, _, _, _) => {
                                        h5380
                                    }
                                };
                                nat_mul(a5381, a5382)?
                            };
                            let a5384 = f20(v2.clone(), 1u64)?;
                            nat_add(a5383, a5384)?
                        };
                        let a5393 = {
                            let a5390 = match v1.clone() {
                                Adt2::C0(_, h5385, _, _, _, _, _, _, _, _, _) => {
                                    h5385
                                }
                            };
                            let a5391 = {
                                let a5388 = 1u64;
                                let a5389 = f163(match v0.clone() {
                                    Adt2::C0(h5386, _, _, _, _, _, _, _, _, _, _) => {
                                        h5386
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5387, _, _, _, _, _, _, _, _, _) => {
                                        h5387
                                    }
                                })?;
                                nat_add(a5388, a5389)?
                            };
                            nat_mul(a5390, a5391)?
                        };
                        nat_add(a5392, a5393)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5394, _, _, _, _, _, _, _, _, _) => {
                            h5394
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5395, _, _, _, _, _, _, _, _, _) => {
                            h5395
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5396, _, _, _, _, _) => {
                            h5396
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5397, _, _, _, _) => {
                            h5397
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5398, _, _, _) => {
                            h5398
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5399, _, _) => {
                            h5399
                        }
                    }), f24(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5400, _, _, _, _, _, _, _, _, _, _) => {
                            h5400
                        }
                    }, f24(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f209(v0: u64) -> bool {
    let a5401 = v0;
    let a5402 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5401, a5402) {
        true
    } else {
        let a5403 = v0;
        let a5404 = 1u64;
        nat_eq(a5403, a5404)
    } {
        true
    } else {
        let a5405 = v0;
        let a5406 = 2u64;
        nat_eq(a5405, a5406)
    } {
        true
    } else {
        let a5407 = v0;
        let a5408 = 3u64;
        nat_eq(a5407, a5408)
    } {
        true
    } else {
        let a5409 = v0;
        let a5410 = 6u64;
        nat_eq(a5409, a5410)
    } {
        true
    } else {
        let a5411 = v0;
        let a5412 = 7u64;
        nat_eq(a5411, a5412)
    } {
        true
    } else {
        let a5413 = v0;
        let a5414 = 8u64;
        nat_eq(a5413, a5414)
    } {
        true
    } else {
        let a5415 = v0;
        let a5416 = 10u64;
        nat_eq(a5415, a5416)
    } {
        true
    } else {
        let a5417 = v0;
        let a5418 = 11u64;
        nat_eq(a5417, a5418)
    } {
        true
    } else {
        let a5419 = v0;
        let a5420 = 12u64;
        nat_eq(a5419, a5420)
    } {
        true
    } else {
        let a5421 = v0;
        let a5422 = 13u64;
        nat_eq(a5421, a5422)
    } {
        true
    } else {
        let a5423 = v0;
        let a5424 = 14u64;
        nat_eq(a5423, a5424)
    } {
        true
    } else {
        let a5425 = v0;
        let a5426 = 16u64;
        nat_eq(a5425, a5426)
    } {
        true
    } else {
        let a5427 = v0;
        let a5428 = 17u64;
        nat_eq(a5427, a5428)
    } {
        true
    } else {
        let a5429 = v0;
        let a5430 = 18u64;
        nat_eq(a5429, a5430)
    } {
        true
    } else {
        let a5431 = v0;
        let a5432 = 19u64;
        nat_eq(a5431, a5432)
    } {
        true
    } else {
        let a5433 = v0;
        let a5434 = 20u64;
        nat_eq(a5433, a5434)
    } {
        true
    } else {
        let a5435 = v0;
        let a5436 = 21u64;
        nat_eq(a5435, a5436)
    } {
        true
    } else {
        let a5437 = v0;
        let a5438 = 22u64;
        nat_eq(a5437, a5438)
    } {
        true
    } else {
        let a5439 = v0;
        let a5440 = 23u64;
        nat_eq(a5439, a5440)
    } {
        true
    } else {
        let a5441 = v0;
        let a5442 = 26u64;
        nat_eq(a5441, a5442)
    } {
        true
    } else {
        let a5443 = v0;
        let a5444 = 29u64;
        nat_eq(a5443, a5444)
    } {
        true
    } else {
        let a5445 = v0;
        let a5446 = 30u64;
        nat_eq(a5445, a5446)
    } {
        true
    } else {
        let a5447 = v0;
        let a5448 = 34u64;
        nat_eq(a5447, a5448)
    } {
        true
    } else {
        let a5449 = v0;
        let a5450 = 35u64;
        nat_eq(a5449, a5450)
    } {
        true
    } else {
        let a5451 = v0;
        let a5452 = 39u64;
        nat_eq(a5451, a5452)
    } {
        true
    } else {
        let a5453 = v0;
        let a5454 = 40u64;
        nat_eq(a5453, a5454)
    } {
        true
    } else {
        let a5455 = v0;
        let a5456 = 41u64;
        nat_eq(a5455, a5456)
    } {
        true
    } else {
        let a5457 = v0;
        let a5458 = 42u64;
        nat_eq(a5457, a5458)
    } {
        true
    } else {
        let a5459 = v0;
        let a5460 = 43u64;
        nat_eq(a5459, a5460)
    }
}

pub fn f210(v0: u64) -> bool {
    if f164(v0) {
        true
    } else {
        let a5461 = v0;
        let a5462 = 43u64;
        nat_eq(a5461, a5462)
    }
}

pub fn f211(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5465 = match v1.clone() {
        Adt2::C0(h5463, _, _, _, _, _, _, _, _, _, _) => {
            h5463
        }
    };
    let a5466 = match v3.clone() {
        Adt2::C0(h5464, _, _, _, _, _, _, _, _, _, _) => {
            h5464
        }
    };
    if if nat_eq(a5465, a5466) {
        let a5467 = v0;
        let a5468 = 33u64;
        if nat_eq(a5467, a5468) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5470 = f202(match v1.clone() {
            Adt2::C0(h5469, _, _, _, _, _, _, _, _, _, _) => {
                h5469
            }
        });
        match m5470 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5471, _) => {
                        h5471
                    }
                };
                let v6: u64 = f20(v1.clone(), v5)?;
                let a5476 = {
                    let a5473 = match v1.clone() {
                        Adt2::C0(_, h5472, _, _, _, _, _, _, _, _, _) => {
                            h5472
                        }
                    };
                    let a5474 = v5;
                    let a5475 = 1u64;
                    nat_rem(a5473, a5474, a5475)
                };
                let a5477 = 0u64;
                if if if nat_eq(a5476, a5477) {
                    let a5482 = {
                        let a5479 = match v3.clone() {
                            Adt2::C0(_, h5478, _, _, _, _, _, _, _, _, _) => {
                                h5478
                            }
                        };
                        let a5480 = v5;
                        let a5481 = 1u64;
                        nat_rem(a5479, a5480, a5481)
                    };
                    let a5483 = 0u64;
                    nat_eq(a5482, a5483)
                } else {
                    false
                } {
                    let a5484 = v6;
                    let a5485 = f20(v3.clone(), v5)?;
                    nat_eq(a5484, a5485)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h5486) => {
                            h5486
                        }
                    }, f24(v1, v5), f24(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f44(v4, v5, v6, v7) {
        f44(v0, v1, v2, v3)
    } else {
        let a5490 = {
            let a5487 = v0;
            let a5488 = v4;
            let a5489 = 1u64;
            nat_rem(a5487, a5488, a5489)
        };
        let a5491 = 0u64;
        if if if nat_eq(a5490, a5491) {
            let a5495 = {
                let a5492 = v1;
                let a5493 = v5;
                let a5494 = 1u64;
                nat_rem(a5492, a5493, a5494)
            };
            let a5496 = 0u64;
            nat_eq(a5495, a5496)
        } else {
            false
        } {
            let a5500 = {
                let a5497 = v2;
                let a5498 = v6;
                let a5499 = 1u64;
                nat_rem(a5497, a5498, a5499)
            };
            let a5501 = 0u64;
            nat_eq(a5500, a5501)
        } else {
            false
        } {
            let a5505 = {
                let a5502 = v3;
                let a5503 = v7;
                let a5504 = 1u64;
                nat_rem(a5502, a5503, a5504)
            };
            let a5506 = 0u64;
            nat_eq(a5505, a5506)
        } else {
            false
        }
    }
}

pub fn f213(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5507 = v0;
    let a5508 = v1;
    if nat_eq(a5507, a5508) {
        let a5511 = v4;
        let a5512 = {
            let a5509 = v2;
            let a5510 = v3;
            nat_add(a5509, a5510)?
        };
        Ok(nat_eq(a5511, a5512))
    } else {
        let a5513 = v2;
        let a5514 = v3;
        if nat_eq(a5513, a5514) {
            let a5515 = v4;
            let a5516 = v2;
            Ok(nat_eq(a5515, a5516))
        } else {
            Ok(false)
        }
    }
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5519 = {
            let a5517 = v0;
            let a5518 = f133(v1);
            nat_add(a5517, a5518)?
        };
        let a5520 = f133(v2);
        nat_add(a5519, a5520)?
    };
    let v5: u64 = {
        let a5521 = f132(v1);
        let a5522 = f132(v2);
        nat_add(a5521, a5522)?
    };
    let a5523 = v1;
    let a5524 = 4294967295u64;
    if if nat_le(a5523, a5524) {
        let a5525 = v2;
        let a5526 = 4294967295u64;
        nat_le(a5525, a5526)
    } else {
        false
    } {
        let a5527 = v5;
        let a5528 = v4;
        if nat_le(a5527, a5528) {
            let a5531 = v3;
            let a5532 = {
                let a5529 = v4;
                let a5530 = v5;
                nat_sub(a5529, a5530)
            };
            Ok(nat_eq(a5531, a5532))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f215(v0: u64, v1: u64) -> R<bool> {
    let a5533 = v1;
    let a5534 = 4294967295u64;
    if nat_le(a5533, a5534) {
        let a5537 = {
            let a5535 = f133(v1);
            let a5536 = f132(v1);
            nat_add(a5535, a5536)?
        };
        let a5538 = v0;
        Ok(nat_lt(a5537, a5538))
    } else {
        Ok(false)
    }
}

pub fn f216(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f258(v0.clone(), false)?;
    let v6: u64 = f258(v0, true)?;
    let a5539 = v6;
    let a5540 = v1;
    if if nat_le(a5539, a5540) {
        let a5547 = {
            let a5545 = {
                let a5541 = v1;
                let a5542 = v5;
                nat_add(a5541, a5542)?
            };
            let a5546 = {
                let a5543 = v3;
                let a5544 = 0u64;
                if nat_eq(a5543, a5544) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5545, a5546)?
        };
        let a5548 = v2;
        nat_le(a5547, a5548)
    } else {
        false
    } {
        if v4 {
            let a5553 = {
                let a5551 = {
                    let a5549 = v1;
                    let a5550 = v5;
                    nat_add(a5549, a5550)?
                };
                let a5552 = v6;
                nat_sub(a5551, a5552)
            };
            let a5554 = v2;
            Ok(nat_lt(a5553, a5554))
        } else {
            let a5559 = {
                let a5557 = {
                    let a5555 = v1;
                    let a5556 = v5;
                    nat_add(a5555, a5556)?
                };
                let a5558 = v6;
                nat_sub(a5557, a5558)
            };
            let a5560 = v2;
            Ok(nat_le(a5559, a5560))
        }
    } else {
        Ok(false)
    }
}

pub fn f217(v0: u64, v1: u64) -> bool {
    let a5561 = v0;
    let a5562 = 0u64;
    if if nat_eq(a5561, a5562) {
        true
    } else {
        let a5563 = v0;
        let a5564 = 1u64;
        nat_eq(a5563, a5564)
    } {
        let a5565 = v1;
        let a5566 = 4294967296u64;
        nat_lt(a5565, a5566)
    } else {
        false
    }
}

pub fn f218(v0: u64, v1: u64) -> R<u64> {
    let a5571 = {
        let a5569 = v0;
        let a5570 = {
            let a5567 = v1;
            let a5568 = 1u64;
            nat_sub(a5567, a5568)
        };
        nat_add(a5569, a5570)?
    };
    let a5572 = v1;
    let a5573 = 0u64;
    Ok(nat_quot(a5571, a5572, a5573))
}

pub fn f219(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f44(match v0.clone() {
        Adt2::C0(_, h5574, _, _, _, _, _, _, _, _, _) => {
            h5574
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5575, _, _, _, _, _, _, _, _) => {
            h5575
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5576, _, _, _, _, _, _, _) => {
            h5576
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5577, _, _, _, _, _, _) => {
            h5577
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a5580 = match v1.clone() {
                Adt2::C0(_, h5578, _, _, _, _, _, _, _, _, _) => {
                    h5578
                }
            };
            let a5581 = match v1.clone() {
                Adt2::C0(_, _, h5579, _, _, _, _, _, _, _, _) => {
                    h5579
                }
            };
            nat_mul(a5580, a5581)?
        };
        let v5: u64 = {
            let a5583 = v4;
            let a5584 = match v1.clone() {
                Adt2::C0(_, _, _, h5582, _, _, _, _, _, _, _) => {
                    h5582
                }
            };
            nat_mul(a5583, a5584)?
        };
        let a5597 = {
            let a5595 = {
                let a5586 = match v0.clone() {
                    Adt2::C0(_, h5585, _, _, _, _, _, _, _, _, _) => {
                        h5585
                    }
                };
                let a5587 = 1u64;
                nat_sub(a5586, a5587)
            };
            let a5596 = {
                let a5593 = f259(match v0.clone() {
                    Adt2::C0(_, _, h5588, _, _, _, _, _, _, _, _) => {
                        h5588
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h5589, _, _, _, _, _, _, _, _, _) => {
                        h5589
                    }
                })?;
                let a5594 = f259(match v0 {
                    Adt2::C0(_, _, _, h5590, _, _, _, _, _, _, _) => {
                        h5590
                    }
                }, v2, {
                    let a5591 = v3;
                    let a5592 = v5;
                    nat_mul(a5591, a5592)?
                }, v4)?;
                nat_add(a5593, a5594)?
            };
            nat_add(a5595, a5596)?
        };
        let a5598 = f20(v1, 1u64)?;
        Ok(nat_lt(a5597, a5598))
    }
}

pub fn f220(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5599 = v0;
    let a5600 = 2147483648u64;
    if if if if nat_lt(a5599, a5600) {
        let a5601 = v1;
        let a5602 = 2147483648u64;
        nat_lt(a5601, a5602)
    } else {
        false
    } {
        let a5605 = {
            let a5603 = v0;
            let a5604 = v1;
            nat_mul(a5603, a5604)?
        };
        let a5606 = 2147483648u64;
        nat_lt(a5605, a5606)
    } else {
        false
    } {
        let a5611 = {
            let a5609 = {
                let a5607 = v0;
                let a5608 = 1u64;
                nat_sub(a5607, a5608)
            };
            let a5610 = v2;
            nat_mul(a5609, a5610)?
        };
        let a5612 = 2147483648u64;
        nat_lt(a5611, a5612)
    } else {
        false
    } {
        let a5617 = {
            let a5615 = {
                let a5613 = v1;
                let a5614 = 1u64;
                nat_sub(a5613, a5614)
            };
            let a5616 = v2;
            nat_mul(a5615, a5616)?
        };
        let a5618 = 2147483648u64;
        Ok(nat_lt(a5617, a5618))
    } else {
        Ok(false)
    }
}

pub fn f221(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a5619 = v1;
    let a5620 = 13u64;
    if if nat_eq(a5619, a5620) {
        f44(match v0.clone() {
            Adt3::C0(h5621, _, _, _, _, _, _, _) => {
                h5621
            }
        }, match v0.clone() {
            Adt3::C0(_, h5622, _, _, _, _, _, _) => {
                h5622
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h5623, _, _, _, _, _) => {
                h5623
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h5624, _, _, _, _) => {
                h5624
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a5630 = match v0.clone() {
                Adt3::C0(_, h5625, _, _, _, _, _, _) => {
                    h5625
                }
            };
            let a5631 = {
                let a5628 = match v0.clone() {
                    Adt3::C0(_, _, h5626, _, _, _, _, _) => {
                        h5626
                    }
                };
                let a5629 = match v0.clone() {
                    Adt3::C0(_, _, _, h5627, _, _, _, _) => {
                        h5627
                    }
                };
                nat_mul(a5628, a5629)?
            };
            nat_mul(a5630, a5631)?
        };
        let a5632 = v1;
        let a5633 = 13u64;
        if nat_eq(a5632, a5633) {
            let a5642 = 2u64;
            let a5643 = if f260(v0.clone(), v2) {
                let a5635 = v3;
                let a5636 = f233(match v0 {
                    Adt3::C0(h5634, _, _, _, _, _, _, _) => {
                        h5634
                    }
                }, false)?;
                nat_mul(a5635, a5636)?
            } else {
                let a5640 = v3;
                let a5641 = {
                    let a5638 = match v0 {
                        Adt3::C0(h5637, _, _, _, _, _, _, _) => {
                            h5637
                        }
                    };
                    let a5639 = 2u64;
                    nat_mul(a5638, a5639)?
                };
                nat_mul(a5640, a5641)?
            };
            nat_add(a5642, a5643)
        } else {
            let a5647 = v3;
            let a5648 = f233(match v0 {
                Adt3::C0(h5644, _, _, _, _, _, _, _) => {
                    h5644
                }
            }, {
                let a5645 = v1;
                let a5646 = 16u64;
                nat_eq(a5645, a5646)
            })?;
            nat_mul(a5647, a5648)
        }
    }
}

pub fn f222(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a5649 = v0;
    let a5650 = 30u64;
    if if nat_eq(a5649, a5650) {
        let a5653 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h5651, _, _, _, _) => {
                h5651
            }
        };
        let a5654 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h5652, _, _, _, _, _) => {
                h5652
            }
        };
        nat_lt(a5653, a5654)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m5655 = v3;
        match m5655 {
            None => {
                f261(v0, v1, match v2.clone() {
                    Adt2::C0(h5656, _, _, _, _, _, _, _, _, _, _) => {
                        h5656
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h5657, _, _, _, _, _, _, _, _, _, _) => {
                        h5657
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h5658, _, _, _, _, _, _, _, _, _) => {
                        h5658
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5659, _, _, _, _, _, _, _, _) => {
                        h5659
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5660, _, _, _, _, _, _, _) => {
                        h5660
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5661, _, _, _, _, _, _) => {
                        h5661
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h5662, _, _, _, _, _, _, _, _, _) => {
                        h5662
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5663, _, _, _, _, _, _, _, _) => {
                        h5663
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5664, _, _, _, _, _, _, _) => {
                        h5664
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5665, _, _, _, _, _, _) => {
                        h5665
                    }
                })
            }
            Some(v5) => {
                f261(v0, v1, match v2.clone() {
                    Adt2::C0(h5666, _, _, _, _, _, _, _, _, _, _) => {
                        h5666
                    }
                }, match v5.clone() {
                    Adt2::C0(h5667, _, _, _, _, _, _, _, _, _, _) => {
                        h5667
                    }
                }, match v4.clone() {
                    Adt2::C0(h5668, _, _, _, _, _, _, _, _, _, _) => {
                        h5668
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h5669, _, _, _, _, _, _, _, _, _) => {
                        h5669
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5670, _, _, _, _, _, _, _, _) => {
                        h5670
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5671, _, _, _, _, _, _, _) => {
                        h5671
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5672, _, _, _, _, _, _) => {
                        h5672
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h5673, _, _, _, _, _, _, _, _, _) => {
                        h5673
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h5674, _, _, _, _, _, _, _, _) => {
                        h5674
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h5675, _, _, _, _, _, _, _) => {
                        h5675
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h5676, _, _, _, _, _, _) => {
                        h5676
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h5677, _, _, _, _, _, _, _, _, _) => {
                        h5677
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5678, _, _, _, _, _, _, _, _) => {
                        h5678
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5679, _, _, _, _, _, _, _) => {
                        h5679
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5680, _, _, _, _, _, _) => {
                        h5680
                    }
                })
            }
        }
    }
}

pub fn f223(v0: u64, v1: u64) -> u64 {
    let a5681 = v0;
    let a5682 = 23u64;
    if nat_eq(a5681, a5682) {
        let a5683 = v1;
        let a5684 = 0u64;
        if nat_eq(a5683, a5684) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f224(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5689 = {
        let a5687 = {
            let a5685 = v0;
            let a5686 = v1;
            nat_sub(a5685, a5686)
        };
        let a5688 = 1u64;
        nat_sub(a5687, a5688)
    };
    let a5690 = v2;
    nat_add(a5689, a5690)
}

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f131({
        let a5691 = v0;
        let a5692 = v4;
        let a5693 = 0u64;
        nat_rem(a5691, a5692, a5693)
    }, {
        let a5694 = v1;
        let a5695 = v5;
        let a5696 = 0u64;
        nat_rem(a5694, a5695, a5696)
    }, {
        let a5697 = v2;
        let a5698 = v6;
        let a5699 = 0u64;
        nat_rem(a5697, a5698, a5699)
    }, {
        let a5700 = v3;
        let a5701 = v7;
        let a5702 = 0u64;
        nat_rem(a5700, a5701, a5702)
    }, v8, v9, v10, v11)
}

pub fn f226(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5703 = v1;
    let a5704 = 2147483648u64;
    if nat_lt(a5703, a5704) {
        let a5715 = {
            let a5713 = {
                let a5708 = {
                    let a5705 = v0;
                    let a5706 = v2;
                    let a5707 = 0u64;
                    nat_rem(a5705, a5706, a5707)
                };
                let a5709 = v2;
                nat_add(a5708, a5709)?
            };
            let a5714 = {
                let a5710 = v1;
                let a5711 = v2;
                let a5712 = 0u64;
                nat_rem(a5710, a5711, a5712)
            };
            nat_sub(a5713, a5714)
        };
        let a5716 = v2;
        let a5717 = 0u64;
        Ok(nat_rem(a5715, a5716, a5717))
    } else {
        let a5720 = {
            let a5718 = v0;
            let a5719 = f132(v1);
            nat_add(a5718, a5719)?
        };
        let a5721 = v2;
        let a5722 = 0u64;
        Ok(nat_rem(a5720, a5721, a5722))
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5725 = f133(v1);
    let a5726 = {
        let a5723 = v0;
        let a5724 = f132(v1);
        nat_add(a5723, a5724)?
    };
    if nat_le(a5725, a5726) {
        let a5727 = f228(v0, v1)?;
        let a5728 = v2;
        Ok(nat_lt(a5727, a5728))
    } else {
        Ok(false)
    }
}

pub fn f228(v0: u64, v1: u64) -> R<u64> {
    let a5731 = {
        let a5729 = v0;
        let a5730 = f132(v1);
        nat_add(a5729, a5730)?
    };
    let a5732 = f133(v1);
    Ok(nat_sub(a5731, a5732))
}

pub fn f229(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5733 = v0;
    let a5734 = v1;
    if nat_lt(a5733, a5734) {
        let a5735 = v1;
        let a5736 = v0;
        Ok(nat_sub(a5735, a5736))
    } else {
        let a5739 = v0;
        let a5740 = {
            let a5737 = v1;
            let a5738 = v2;
            nat_add(a5737, a5738)?
        };
        if nat_lt(a5739, a5740) {
            let a5741 = v0;
            let a5742 = v1;
            Ok(nat_sub(a5741, a5742))
        } else {
            let a5749 = {
                let a5745 = 2u64;
                let a5746 = {
                    let a5743 = v2;
                    let a5744 = 1u64;
                    nat_sub(a5743, a5744)
                };
                nat_mul(a5745, a5746)?
            };
            let a5750 = {
                let a5747 = v0;
                let a5748 = v1;
                nat_sub(a5747, a5748)
            };
            Ok(nat_sub(a5749, a5750))
        }
    }
}

pub fn f230(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5753 = {
        let a5751 = v2;
        let a5752 = f262(v1.clone(), f54(v0, v1.clone(), 0u64), f54(v0, v1.clone(), 1u64), f54(v0, v1.clone(), 2u64), f54(v0, v1.clone(), 3u64), false)?;
        nat_add(a5751, a5752)?
    };
    let a5754 = f262(v1.clone(), f54(v0, v1.clone(), 0u64), f54(v0, v1.clone(), 1u64), f54(v0, v1.clone(), 2u64), f54(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5753, a5754))
}

pub fn f231(v0: u64, v1: u64, v2: u64) -> bool {
    let a5755 = v0;
    let a5756 = 0u64;
    if nat_eq(a5755, a5756) {
        let a5757 = v2;
        let a5758 = v1;
        nat_le(a5757, a5758)
    } else {
        let a5759 = v0;
        let a5760 = 1u64;
        if nat_eq(a5759, a5760) {
            let a5761 = v2;
            let a5762 = v1;
            nat_lt(a5761, a5762)
        } else {
            let a5763 = v0;
            let a5764 = 2u64;
            if nat_eq(a5763, a5764) {
                let a5765 = v1;
                let a5766 = v2;
                nat_le(a5765, a5766)
            } else {
                let a5767 = v0;
                let a5768 = 3u64;
                if nat_eq(a5767, a5768) {
                    let a5769 = v1;
                    let a5770 = v2;
                    nat_lt(a5769, a5770)
                } else {
                    let a5771 = v1;
                    let a5772 = v2;
                    nat_eq(a5771, a5772)
                }
            }
        }
    }
}

pub fn f232(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5773 = v0;
    let a5774 = 0u64;
    if nat_eq(a5773, a5774) {
        Ok(f245(0u64))
    } else {
        let a5777 = {
            let a5775 = v0;
            let a5776 = 1u64;
            nat_add(a5775, a5776)?
        };
        let a5778 = f221(v1.clone(), 13u64, v2)?;
        if nat_eq(a5777, a5778) {
            Ok(Adt9::C4({
                let a5779 = v2;
                let a5780 = 0u64;
                if nat_eq(a5779, a5780) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5781 = v0;
                let a5782 = 1u64;
                nat_sub(a5781, a5782)
            };
            if f260(v1.clone(), v2) {
                let v4: u64 = f233(match v1.clone() {
                    Adt3::C0(h5783, _, _, _, _, _, _, _) => {
                        h5783
                    }
                }, false)?;
                let v5: u64 = {
                    let a5784 = v3;
                    let a5785 = v4;
                    let a5786 = 0u64;
                    nat_quot(a5784, a5785, a5786)
                };
                let v6: u64 = {
                    let a5787 = v3;
                    let a5788 = v4;
                    let a5789 = 0u64;
                    nat_rem(a5787, a5788, a5789)
                };
                let a5792 = {
                    let a5790 = v6;
                    let a5791 = 1u64;
                    nat_add(a5790, a5791)?
                };
                let a5793 = v4;
                if nat_eq(a5792, a5793) {
                    Ok(Adt9::C4({
                        let a5794 = v2;
                        let a5795 = 0u64;
                        if nat_eq(a5794, a5795) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f234(v6, v5, v1, v2)
                }
            } else {
                f263({
                    let a5796 = v3;
                    let a5797 = 2u64;
                    let a5798 = 0u64;
                    nat_quot(a5796, a5797, a5798)
                }, {
                    let a5799 = v3;
                    let a5800 = 2u64;
                    let a5801 = 0u64;
                    nat_rem(a5799, a5800, a5801)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f233(v0: u64, v1: bool) -> R<u64> {
    let a5804 = {
        let a5802 = v0;
        let a5803 = 2u64;
        nat_mul(a5802, a5803)?
    };
    let a5805 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5804, a5805)
}

pub fn f234(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5806 = v0;
    let a5807 = 0u64;
    if nat_eq(a5806, a5807) {
        Ok(f245(1u64))
    } else {
        f263({
            let a5816 = {
                let a5809 = v1;
                let a5810 = match v2.clone() {
                    Adt3::C0(h5808, _, _, _, _, _, _, _) => {
                        h5808
                    }
                };
                nat_mul(a5809, a5810)?
            };
            let a5817 = {
                let a5813 = {
                    let a5811 = v0;
                    let a5812 = 1u64;
                    nat_sub(a5811, a5812)
                };
                let a5814 = 2u64;
                let a5815 = 0u64;
                nat_quot(a5813, a5814, a5815)
            };
            nat_add(a5816, a5817)?
        }, {
            let a5820 = {
                let a5818 = v0;
                let a5819 = 1u64;
                nat_sub(a5818, a5819)
            };
            let a5821 = 2u64;
            let a5822 = 0u64;
            nat_rem(a5820, a5821, a5822)
        }, v2, v3, 1u64)
    }
}

pub fn f235(v0: u64, v1: u64) -> R<u64> {
    let a5823 = v0;
    let a5824 = f87(v1)?;
    let a5825 = 0u64;
    Ok(nat_rem(a5823, a5824, a5825))
}

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let a5826 = 4u64;
    let a5827 = f264(v0, v1)?;
    nat_mul(a5826, a5827)
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let a5828 = 4u64;
    let a5829 = f265(v0, v1)?;
    nat_mul(a5828, a5829)
}

pub fn f238(v0: u64, v1: u64) -> R<bool> {
    let a5835 = {
        let a5832 = {
            let a5830 = f235(v0, v1)?;
            let a5831 = 2u64;
            nat_sub(a5830, a5831)
        };
        let a5833 = 2u64;
        let a5834 = 0u64;
        nat_rem(a5832, a5833, a5834)
    };
    let a5836 = 0u64;
    Ok(nat_eq(a5835, a5836))
}

pub fn f239(v0: u64) -> u64 {
    let m5837 = f202(v0);
    match m5837 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h5838, _) => {
                    h5838
                }
            }
        }
    }
}

pub fn f240(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5839 = f202(v0);
    match m5839 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f209(v0) {
                let a5840 = v1;
                let a5841 = v7;
                nat_lt(a5840, a5841)
            } else {
                false
            } {
                f169(v8, v2)
            } else {
                false
            } {
                let a5842 = v3;
                let a5843 = f266(v4, v5, v6)?;
                nat_lt(a5842, a5843)
            } else {
                false
            } {
                let a5848 = {
                    let a5845 = v7;
                    let a5846 = match v13.clone() {
                        Adt19::C0(_, h5844, _) => {
                            h5844
                        }
                    };
                    let a5847 = 0u64;
                    nat_rem(a5845, a5846, a5847)
                };
                let a5849 = 0u64;
                nat_eq(a5848, a5849)
            } else {
                false
            } {
                Ok(Some({
                    let a5878 = {
                        let a5868 = {
                            let a5858 = {
                                let a5854 = {
                                    let a5851 = v1;
                                    let a5852 = match v13 {
                                        Adt19::C0(_, h5850, _) => {
                                            h5850
                                        }
                                    };
                                    let a5853 = 0u64;
                                    nat_quot(a5851, a5852, a5853)
                                };
                                let a5855 = v9;
                                nat_mul(a5854, a5855)?
                            };
                            let a5859 = {
                                let a5856 = v2;
                                let a5857 = v10;
                                nat_mul(a5856, a5857)?
                            };
                            nat_add(a5858, a5859)?
                        };
                        let a5869 = {
                            let a5866 = {
                                let a5863 = {
                                    let a5860 = v3;
                                    let a5861 = v4;
                                    let a5862 = 0u64;
                                    nat_quot(a5860, a5861, a5862)
                                };
                                let a5864 = v5;
                                let a5865 = 0u64;
                                nat_rem(a5863, a5864, a5865)
                            };
                            let a5867 = v11;
                            nat_mul(a5866, a5867)?
                        };
                        nat_add(a5868, a5869)?
                    };
                    let a5879 = {
                        let a5876 = {
                            let a5873 = {
                                let a5870 = v3;
                                let a5871 = v4;
                                let a5872 = 0u64;
                                nat_quot(a5870, a5871, a5872)
                            };
                            let a5874 = v5;
                            let a5875 = 0u64;
                            nat_quot(a5873, a5874, a5875)
                        };
                        let a5877 = v12;
                        nat_mul(a5876, a5877)?
                    };
                    nat_add(a5878, a5879)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f267(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f242(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f268(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a5880 = v0;
    let a5881 = 64u64;
    if nat_eq(a5880, a5881) {
        f269(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5882 = v0;
        let a5883 = 65u64;
        if nat_eq(a5882, a5883) {
            f270(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f244(v0: u64) -> u64 {
    let a5884 = v0;
    let a5885 = 30u64;
    if nat_eq(a5884, a5885) {
        4u64
    } else {
        3u64
    }
}

pub fn f245(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f246(v0: u64) -> bool {
    let a5886 = 0u64;
    let a5887 = v0;
    if nat_lt(a5886, a5887) {
        let a5888 = v0;
        let a5889 = 2139095040u64;
        nat_le(a5888, a5889)
    } else {
        false
    }
}

pub fn f247(v0: u64) -> u64 {
    let a5890 = v0;
    let a5891 = 40u64;
    if nat_lt(a5890, a5891) {
        let a5892 = v0;
        let a5893 = 20u64;
        if nat_lt(a5892, a5893) {
            let a5894 = v0;
            let a5895 = 10u64;
            if nat_lt(a5894, a5895) {
                let a5896 = v0;
                let a5897 = 5u64;
                if nat_lt(a5896, a5897) {
                    let a5898 = v0;
                    let a5899 = 2u64;
                    if nat_lt(a5898, a5899) {
                        let a5900 = v0;
                        let a5901 = 1u64;
                        if nat_lt(a5900, a5901) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5902 = v0;
                        let a5903 = 3u64;
                        if nat_lt(a5902, a5903) {
                            4u64
                        } else {
                            let a5904 = v0;
                            let a5905 = 4u64;
                            if nat_lt(a5904, a5905) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5906 = v0;
                    let a5907 = 7u64;
                    if nat_lt(a5906, a5907) {
                        let a5908 = v0;
                        let a5909 = 6u64;
                        if nat_lt(a5908, a5909) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5910 = v0;
                        let a5911 = 8u64;
                        if nat_lt(a5910, a5911) {
                            128u64
                        } else {
                            let a5912 = v0;
                            let a5913 = 9u64;
                            if nat_lt(a5912, a5913) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5914 = v0;
                let a5915 = 15u64;
                if nat_lt(a5914, a5915) {
                    let a5916 = v0;
                    let a5917 = 12u64;
                    if nat_lt(a5916, a5917) {
                        let a5918 = v0;
                        let a5919 = 11u64;
                        if nat_lt(a5918, a5919) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5920 = v0;
                        let a5921 = 13u64;
                        if nat_lt(a5920, a5921) {
                            4096u64
                        } else {
                            let a5922 = v0;
                            let a5923 = 14u64;
                            if nat_lt(a5922, a5923) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5924 = v0;
                    let a5925 = 17u64;
                    if nat_lt(a5924, a5925) {
                        let a5926 = v0;
                        let a5927 = 16u64;
                        if nat_lt(a5926, a5927) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5928 = v0;
                        let a5929 = 18u64;
                        if nat_lt(a5928, a5929) {
                            131072u64
                        } else {
                            let a5930 = v0;
                            let a5931 = 19u64;
                            if nat_lt(a5930, a5931) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5932 = v0;
            let a5933 = 30u64;
            if nat_lt(a5932, a5933) {
                let a5934 = v0;
                let a5935 = 25u64;
                if nat_lt(a5934, a5935) {
                    let a5936 = v0;
                    let a5937 = 22u64;
                    if nat_lt(a5936, a5937) {
                        let a5938 = v0;
                        let a5939 = 21u64;
                        if nat_lt(a5938, a5939) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5940 = v0;
                        let a5941 = 23u64;
                        if nat_lt(a5940, a5941) {
                            4194304u64
                        } else {
                            let a5942 = v0;
                            let a5943 = 24u64;
                            if nat_lt(a5942, a5943) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5944 = v0;
                    let a5945 = 27u64;
                    if nat_lt(a5944, a5945) {
                        let a5946 = v0;
                        let a5947 = 26u64;
                        if nat_lt(a5946, a5947) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5948 = v0;
                        let a5949 = 28u64;
                        if nat_lt(a5948, a5949) {
                            134217728u64
                        } else {
                            let a5950 = v0;
                            let a5951 = 29u64;
                            if nat_lt(a5950, a5951) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5952 = v0;
                let a5953 = 35u64;
                if nat_lt(a5952, a5953) {
                    let a5954 = v0;
                    let a5955 = 32u64;
                    if nat_lt(a5954, a5955) {
                        let a5956 = v0;
                        let a5957 = 31u64;
                        if nat_lt(a5956, a5957) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5958 = v0;
                        let a5959 = 33u64;
                        if nat_lt(a5958, a5959) {
                            4294967296u64
                        } else {
                            let a5960 = v0;
                            let a5961 = 34u64;
                            if nat_lt(a5960, a5961) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5962 = v0;
                    let a5963 = 37u64;
                    if nat_lt(a5962, a5963) {
                        let a5964 = v0;
                        let a5965 = 36u64;
                        if nat_lt(a5964, a5965) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5966 = v0;
                        let a5967 = 38u64;
                        if nat_lt(a5966, a5967) {
                            137438953472u64
                        } else {
                            let a5968 = v0;
                            let a5969 = 39u64;
                            if nat_lt(a5968, a5969) {
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

pub fn f248(v0: u64, v1: u64) -> bool {
    let a5970 = v0;
    let a5971 = 1u64;
    if nat_le(a5970, a5971) {
        true
    } else {
        let a5975 = {
            let a5972 = v1;
            let a5973 = 4u64;
            let a5974 = 0u64;
            nat_rem(a5972, a5973, a5974)
        };
        let a5976 = 0u64;
        nat_eq(a5975, a5976)
    }
}

pub fn f249(v0: u64, v1: u64) -> bool {
    let a5977 = v0;
    let a5978 = 1u64;
    if nat_le(a5977, a5978) {
        true
    } else {
        let a5979 = 4u64;
        let a5980 = v1;
        nat_le(a5979, a5980)
    }
}

pub fn f250(v0: u64, v1: u64) -> R<u64> {
    let a5981 = v0;
    let a5982 = 0u64;
    if nat_eq(a5981, a5982) {
        Ok(0u64)
    } else {
        let a5987 = 4u64;
        let a5988 = {
            let a5985 = {
                let a5983 = v0;
                let a5984 = 1u64;
                nat_sub(a5983, a5984)
            };
            let a5986 = v1;
            nat_mul(a5985, a5986)?
        };
        nat_add(a5987, a5988)
    }
}

pub fn f251(v0: bool, v1: u64) -> R<u64> {
    let a5993 = {
        let a5991 = {
            let a5989 = f271(v0, v1)?;
            let a5990 = f272(v0, v1)?;
            nat_add(a5989, a5990)?
        };
        let a5992 = f273(v0);
        nat_add(a5991, a5992)?
    };
    let a5994 = v1;
    nat_add(a5993, a5994)
}

pub fn f252(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f274(v0, v1)?;
    let v3: u64 = {
        let a5999 = {
            let a5997 = {
                let a5995 = v1;
                let a5996 = 1u64;
                nat_sub(a5995, a5996)
            };
            let a5998 = v2;
            nat_mul(a5997, a5998)?
        };
        let a6000 = v0;
        nat_sub(a5999, a6000)
    };
    let a6001 = v2;
    let a6002 = v3;
    if nat_lt(a6001, a6002) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f253(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6003 = v4;
    let a6004 = 0u64;
    if nat_eq(a6003, a6004) {
        Ok(0u64)
    } else {
        let a6011 = 2u64;
        let a6012 = {
            let a6009 = {
                let a6005 = v0;
                let a6006 = v1;
                nat_mul(a6005, a6006)?
            };
            let a6010 = {
                let a6007 = v2;
                let a6008 = v3;
                nat_mul(a6007, a6008)?
            };
            nat_mul(a6009, a6010)?
        };
        nat_mul(a6011, a6012)
    }
}

pub fn f254(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6017 = v3;
    let a6018 = {
        let a6015 = {
            let a6013 = f275(v2, v4)?;
            let a6014 = f276(v0, v1, f274(v2, v4)?)?;
            nat_mul(a6013, a6014)?
        };
        let a6016 = f276(v0, v1, f277(v2, v4)?)?;
        nat_add(a6015, a6016)?
    };
    nat_mul(a6017, a6018)
}

pub fn f255(v0: u64) -> R<u64> {
    let a6021 = 13u64;
    let a6022 = {
        let a6019 = 9u64;
        let a6020 = v0;
        nat_mul(a6019, a6020)?
    };
    nat_add(a6021, a6022)
}

pub fn f256(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f278(v0, v1, v6, v7, v8) {
        let a6023 = v5;
        let a6024 = 26u64;
        nat_eq(a6023, a6024)
    } else {
        false
    } {
        let a6025 = v3;
        let a6026 = v7;
        nat_eq(a6025, a6026)
    } else {
        false
    } {
        let a6027 = v4;
        let a6028 = v8;
        nat_eq(a6027, a6028)
    } else {
        false
    } {
        let a6029 = v9;
        let a6030 = 1u64;
        nat_eq(a6029, a6030)
    } else {
        false
    } {
        let a6031 = v10;
        let a6032 = f279(v0);
        nat_eq(a6031, a6032)
    } else {
        false
    } {
        let a6033 = v11;
        let a6034 = v1;
        nat_eq(a6033, a6034)
    } else {
        false
    } {
        let a6035 = v12;
        let a6036 = v6;
        nat_eq(a6035, a6036)
    } else {
        false
    } {
        let a6037 = v13;
        let a6038 = v7;
        nat_eq(a6037, a6038)
    } else {
        false
    } {
        let a6039 = v14;
        let a6040 = v8;
        nat_eq(a6039, a6040)
    } else {
        false
    }
}

pub fn f257(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6041 = v0;
    let a6042 = 0u64;
    if if if nat_eq(a6041, a6042) {
        true
    } else {
        let a6043 = v1;
        let a6044 = 0u64;
        nat_eq(a6043, a6044)
    } {
        true
    } else {
        let a6045 = v2;
        let a6046 = 0u64;
        nat_eq(a6045, a6046)
    } {
        true
    } else {
        let a6047 = v3;
        let a6048 = 0u64;
        nat_eq(a6047, a6048)
    }
}

pub fn f258(v0: Adt3, v1: bool) -> R<u64> {
    f262(v0.clone(), {
        let a6050 = match v0.clone() {
            Adt3::C0(h6049, _, _, _, _, _, _, _) => {
                h6049
            }
        };
        let a6051 = 1u64;
        nat_sub(a6050, a6051)
    }, {
        let a6053 = match v0.clone() {
            Adt3::C0(_, h6052, _, _, _, _, _, _) => {
                h6052
            }
        };
        let a6054 = 1u64;
        nat_sub(a6053, a6054)
    }, {
        let a6056 = match v0.clone() {
            Adt3::C0(_, _, h6055, _, _, _, _, _) => {
                h6055
            }
        };
        let a6057 = 1u64;
        nat_sub(a6056, a6057)
    }, {
        let a6059 = match v0 {
            Adt3::C0(_, _, _, h6058, _, _, _, _) => {
                h6058
            }
        };
        let a6060 = 1u64;
        nat_sub(a6059, a6060)
    }, v1)
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6061 = v0;
    let a6062 = 0u64;
    if nat_eq(a6061, a6062) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6063 = v0;
            let a6064 = 1u64;
            nat_sub(a6063, a6064)
        };
        let v5: u64 = {
            let a6065 = v4;
            let a6066 = v1;
            let a6067 = 0u64;
            nat_quot(a6065, a6066, a6067)
        };
        let v6: u64 = {
            let a6075 = {
                let a6068 = v5;
                let a6069 = v2;
                nat_mul(a6068, a6069)?
            };
            let a6076 = {
                let a6073 = {
                    let a6070 = v4;
                    let a6071 = v1;
                    let a6072 = 0u64;
                    nat_rem(a6070, a6071, a6072)
                };
                let a6074 = v3;
                nat_mul(a6073, a6074)?
            };
            nat_add(a6075, a6076)?
        };
        let a6077 = v5;
        let a6078 = 0u64;
        if nat_eq(a6077, a6078) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6087 = {
                    let a6081 = {
                        let a6079 = v5;
                        let a6080 = 1u64;
                        nat_sub(a6079, a6080)
                    };
                    let a6082 = v2;
                    nat_mul(a6081, a6082)?
                };
                let a6088 = {
                    let a6085 = {
                        let a6083 = v1;
                        let a6084 = 1u64;
                        nat_sub(a6083, a6084)
                    };
                    let a6086 = v3;
                    nat_mul(a6085, a6086)?
                };
                nat_add(a6087, a6088)?
            };
            let a6089 = v6;
            let a6090 = v7;
            if nat_lt(a6089, a6090) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f260(v0: Adt3, v1: u64) -> bool {
    let a6091 = v1;
    let a6092 = 0u64;
    if !nat_eq(a6091, a6092) {
        true
    } else {
        let a6094 = match v0 {
            Adt3::C0(_, _, _, _, h6093, _, _, _) => {
                h6093
            }
        };
        let a6095 = 4u64;
        nat_eq(a6094, a6095)
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6096 = f280(v0, v1);
    match m6096 {
        None => {
            if v5 {
                let m6097 = f281(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6097 {
                    Adt22::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt22::C1 => {
                        Ok(Adt10::C1)
                    }
                    Adt22::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt10::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt10::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a6098 = 4u64;
                let a6099 = v28;
                nat_le(a6098, a6099)
            };
            let a6100 = v2;
            let a6101 = 0u64;
            if if if if if nat_eq(a6100, a6101) {
                let a6102 = v4;
                let a6103 = 0u64;
                nat_eq(a6102, a6103)
            } else {
                false
            } {
                let a6104 = v6;
                let a6105 = v14;
                if nat_eq(a6104, a6105) {
                    let a6106 = v7;
                    let a6107 = v15;
                    nat_eq(a6106, a6107)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6108 = v8;
                let a6109 = v16;
                if nat_eq(a6108, a6109) {
                    let a6110 = v9;
                    let a6111 = v17;
                    nat_eq(a6110, a6111)
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
                        let a6112 = v3;
                        let a6113 = 0u64;
                        nat_eq(a6112, a6113)
                    } else {
                        false
                    } {
                        f212(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6118 = {
                        let a6114 = v6;
                        let a6115 = v7;
                        nat_mul(a6114, a6115)?
                    };
                    let a6119 = {
                        let a6116 = v8;
                        let a6117 = v9;
                        nat_mul(a6116, a6117)?
                    };
                    nat_mul(a6118, a6119)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f262(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6140 = {
        let a6134 = {
            let a6128 = {
                let a6122 = v1;
                let a6123 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6120, _, _, _) => {
                            h6120
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6121, _, _, _) => {
                            h6121
                        }
                    })
                };
                nat_mul(a6122, a6123)?
            };
            let a6129 = {
                let a6126 = v2;
                let a6127 = if v5 {
                    f132(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6124, _, _) => {
                            h6124
                        }
                    })
                } else {
                    f133(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6125, _, _) => {
                            h6125
                        }
                    })
                };
                nat_mul(a6126, a6127)?
            };
            nat_add(a6128, a6129)?
        };
        let a6135 = {
            let a6132 = v3;
            let a6133 = if v5 {
                f132(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6130, _) => {
                        h6130
                    }
                })
            } else {
                f133(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6131, _) => {
                        h6131
                    }
                })
            };
            nat_mul(a6132, a6133)?
        };
        nat_add(a6134, a6135)?
    };
    let a6141 = {
        let a6138 = v4;
        let a6139 = if v5 {
            f132(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6136) => {
                    h6136
                }
            })
        } else {
            f133(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6137) => {
                    h6137
                }
            })
        };
        nat_mul(a6138, a6139)?
    };
    nat_add(a6140, a6141)
}

pub fn f263(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6142 = v1;
    let a6143 = 0u64;
    if nat_eq(a6142, a6143) {
        Ok(Adt9::C4({
            let a6144 = v3;
            let a6145 = 0u64;
            if nat_eq(a6144, a6145) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f84(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6146 = v3;
            let a6147 = 0u64;
            if nat_eq(a6146, a6147) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f264(v0: u64, v1: u64) -> R<u64> {
    let a6148 = v0;
    let a6149 = f87(v1)?;
    let a6150 = 0u64;
    Ok(nat_quot(a6148, a6149, a6150))
}

pub fn f265(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f264(v0, v1)?;
    let v3: u64 = {
        let a6153 = {
            let a6151 = f235(v0, v1)?;
            let a6152 = 2u64;
            nat_sub(a6151, a6152)
        };
        let a6154 = 2u64;
        let a6155 = 0u64;
        nat_quot(a6153, a6154, a6155)
    };
    let v4: u64 = {
        let a6156 = v2;
        let a6157 = v1;
        nat_mul(a6156, a6157)?
    };
    if f238(v0, v1)? {
        let a6158 = v4;
        let a6159 = v3;
        nat_add(a6158, a6159)
    } else {
        let a6171 = {
            let a6167 = v4;
            let a6168 = {
                let a6165 = {
                    let a6160 = v1;
                    let a6161 = 1u64;
                    nat_sub(a6160, a6161)
                };
                let a6166 = {
                    let a6162 = v2;
                    let a6163 = v1;
                    let a6164 = 0u64;
                    nat_rem(a6162, a6163, a6164)
                };
                nat_mul(a6165, a6166)?
            };
            nat_sub(a6167, a6168)
        };
        let a6172 = {
            let a6169 = v3;
            let a6170 = v1;
            nat_mul(a6169, a6170)?
        };
        nat_add(a6171, a6172)
    }
}

pub fn f266(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6173 = v0;
    let a6174 = 0u64;
    if if if nat_eq(a6173, a6174) {
        true
    } else {
        let a6175 = v1;
        let a6176 = 0u64;
        nat_eq(a6175, a6176)
    } {
        true
    } else {
        let a6177 = v2;
        let a6178 = 0u64;
        nat_eq(a6177, a6178)
    } {
        Ok(0u64)
    } else {
        let a6181 = {
            let a6179 = v0;
            let a6180 = v1;
            nat_mul(a6179, a6180)?
        };
        let a6182 = v2;
        nat_mul(a6181, a6182)
    }
}

pub fn f267(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f96(match v1.clone() {
        Adt20::C0(h6183, _, _, _, _, _, _, _) => {
            h6183
        }
    })?;
    let v6: u64 = {
        let a6189 = match v1.clone() {
            Adt20::C0(_, h6184, _, _, _, _, _, _) => {
                h6184
            }
        };
        let a6190 = {
            let a6187 = match v1.clone() {
                Adt20::C0(_, _, h6185, _, _, _, _, _) => {
                    h6185
                }
            };
            let a6188 = match v1.clone() {
                Adt20::C0(_, _, _, h6186, _, _, _, _) => {
                    h6186
                }
            };
            nat_mul(a6187, a6188)?
        };
        nat_mul(a6189, a6190)?
    };
    let a6192 = v0;
    let a6193 = f49(match v1.clone() {
        Adt20::C0(h6191, _, _, _, _, _, _, _) => {
            h6191
        }
    }, v6)?;
    if nat_lt(a6192, a6193) {
        f282({
            let a6194 = v0;
            let a6195 = v5;
            let a6196 = 0u64;
            nat_rem(a6194, a6195, a6196)
        }, {
            let a6197 = v0;
            let a6198 = v5;
            let a6199 = 0u64;
            nat_quot(a6197, a6198, a6199)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f268(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6200 = v6;
        let a6201 = 6u64;
        nat_sub(a6200, a6201)
    };
    let v9: Adt20 = f283(v3);
    let a6203 = v0;
    let a6204 = f199(match v1.clone() {
        Adt20::C0(h6202, _, _, _, _, _, _, _) => {
            h6202
        }
    }, v5, v8)?;
    if nat_lt(a6203, a6204) {
        let v10: u64 = f93(match v1.clone() {
            Adt20::C0(h6205, _, _, _, _, _, _, _) => {
                h6205
            }
        }, v5, v8)?;
        let a6206 = v0;
        let a6207 = v10;
        if nat_lt(a6206, a6207) {
            f284(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6208 = v0;
                let a6209 = v10;
                nat_sub(a6208, a6209)
            };
            let v12: u64 = f255(match v1.clone() {
                Adt20::C0(h6210, _, _, _, _, _, _, _) => {
                    h6210
                }
            })?;
            f285({
                let a6211 = v11;
                let a6212 = v12;
                let a6213 = 0u64;
                nat_rem(a6211, a6212, a6213)
            }, {
                let a6214 = v11;
                let a6215 = v12;
                let a6216 = 0u64;
                nat_quot(a6214, a6215, a6216)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f269(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6217 = v5;
        let a6218 = 6u64;
        nat_sub(a6217, a6218)
    };
    let v8: bool = f193(match v1.clone() {
        Adt20::C0(_, _, _, _, h6219, _, _, _) => {
            h6219
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6220, _, _, _) => {
            h6220
        }
    });
    let a6222 = v0;
    let a6223 = f192(v8, match v1.clone() {
        Adt20::C0(h6221, _, _, _, _, _, _, _) => {
            h6221
        }
    }, v4, v7)?;
    if nat_lt(a6222, a6223) {
        let v9: u64 = f93(match v1.clone() {
            Adt20::C0(h6224, _, _, _, _, _, _, _) => {
                h6224
            }
        }, v4, v7)?;
        let a6225 = v0;
        let a6226 = v9;
        if nat_lt(a6225, a6226) {
            f284(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6227 = v0;
                let a6228 = v9;
                nat_sub(a6227, a6228)
            };
            let v11: u64 = f251(v8, match v1.clone() {
                Adt20::C0(h6229, _, _, _, _, _, _, _) => {
                    h6229
                }
            })?;
            f286({
                let a6230 = v10;
                let a6231 = v11;
                let a6232 = 0u64;
                nat_rem(a6230, a6231, a6232)
            }, {
                let a6233 = v10;
                let a6234 = v11;
                let a6235 = 0u64;
                nat_quot(a6233, a6234, a6235)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f270(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6236 = v5;
        let a6237 = 6u64;
        nat_sub(a6236, a6237)
    };
    let v8: Adt20 = f283(v2);
    let v9: u64 = f253(match v1.clone() {
        Adt20::C0(h6238, _, _, _, _, _, _, _) => {
            h6238
        }
    }, match v1.clone() {
        Adt20::C0(_, h6239, _, _, _, _, _, _) => {
            h6239
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6240, _, _, _, _, _) => {
            h6240
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6241, _, _, _, _) => {
            h6241
        }
    }, v7)?;
    let a6246 = v0;
    let a6247 = f198(match v1.clone() {
        Adt20::C0(h6242, _, _, _, _, _, _, _) => {
            h6242
        }
    }, match v1.clone() {
        Adt20::C0(_, h6243, _, _, _, _, _, _) => {
            h6243
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6244, _, _, _, _, _) => {
            h6244
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6245, _, _, _, _) => {
            h6245
        }
    }, v4, v7)?;
    if nat_lt(a6246, a6247) {
        let a6248 = v0;
        let a6249 = v9;
        if nat_lt(a6248, a6249) {
            f284(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f287({
                let a6250 = v0;
                let a6251 = v9;
                nat_sub(a6250, a6251)
            }, match v1.clone() {
                Adt20::C0(h6252, _, _, _, _, _, _, _) => {
                    h6252
                }
            }, match v1.clone() {
                Adt20::C0(_, h6253, _, _, _, _, _, _) => {
                    h6253
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6254, _, _, _, _, _) => {
                    h6254
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6255, _, _, _, _) => {
                    h6255
                }
            }, v4)?;
            f288(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f271(v0: bool, v1: u64) -> R<u64> {
    let a6258 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6259 = {
        let a6256 = f289(v0);
        let a6257 = v1;
        nat_mul(a6256, a6257)?
    };
    nat_add(a6258, a6259)
}

pub fn f272(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6267 = {
            let a6260 = 13u64;
            let a6261 = f290(v1);
            nat_mul(a6260, a6261)?
        };
        let a6268 = {
            let a6265 = 4u64;
            let a6266 = {
                let a6262 = v1;
                let a6263 = 4u64;
                let a6264 = 0u64;
                nat_rem(a6262, a6263, a6264)
            };
            nat_mul(a6265, a6266)?
        };
        nat_add(a6267, a6268)
    } else {
        let a6269 = 3u64;
        let a6270 = v1;
        nat_mul(a6269, a6270)
    }
}

pub fn f273(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f274(v0: u64, v1: u64) -> R<u64> {
    let a6275 = {
        let a6273 = {
            let a6271 = v0;
            let a6272 = v1;
            nat_add(a6271, a6272)?
        };
        let a6274 = 1u64;
        nat_sub(a6273, a6274)
    };
    let a6276 = v1;
    let a6277 = 0u64;
    Ok(nat_quot(a6275, a6276, a6277))
}

pub fn f275(v0: u64, v1: u64) -> R<u64> {
    let a6278 = v0;
    let a6279 = f274(v0, v1)?;
    let a6280 = 0u64;
    Ok(nat_quot(a6278, a6279, a6280))
}

pub fn f276(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6281 = v2;
    let a6282 = 0u64;
    if nat_eq(a6281, a6282) {
        Ok(0u64)
    } else {
        let a6287 = 11u64;
        let a6288 = {
            let a6285 = {
                let a6283 = v1;
                let a6284 = v2;
                nat_mul(a6283, a6284)?
            };
            let a6286 = f291(v0)?;
            nat_mul(a6285, a6286)?
        };
        nat_add(a6287, a6288)
    }
}

pub fn f277(v0: u64, v1: u64) -> R<u64> {
    let a6289 = v0;
    let a6290 = f274(v0, v1)?;
    let a6291 = 0u64;
    Ok(nat_rem(a6289, a6290, a6291))
}

pub fn f278(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f257(v1, v2, v3, v4) {
        f292(v0)
    } else {
        f209(v0)
    }
}

pub fn f279(v0: u64) -> u64 {
    let a6292 = v0;
    let a6293 = 26u64;
    if nat_eq(a6292, a6293) {
        26u64
    } else {
        0u64
    }
}

pub fn f280(v0: u64, v1: u64) -> Option<u64> {
    let a6294 = v0;
    let a6295 = 2u64;
    if nat_eq(a6294, a6295) {
        Some(0u64)
    } else {
        let a6296 = v0;
        let a6297 = 5u64;
        if nat_eq(a6296, a6297) {
            Some(1u64)
        } else {
            let a6298 = v0;
            let a6299 = 6u64;
            if nat_eq(a6298, a6299) {
                Some(2u64)
            } else {
                let a6300 = v0;
                let a6301 = 7u64;
                if nat_eq(a6300, a6301) {
                    Some(3u64)
                } else {
                    let a6302 = v0;
                    let a6303 = 9u64;
                    if nat_eq(a6302, a6303) {
                        Some(4u64)
                    } else {
                        let a6304 = v0;
                        let a6305 = 8u64;
                        if nat_eq(a6304, a6305) {
                            Some(5u64)
                        } else {
                            let a6306 = v0;
                            let a6307 = 89u64;
                            if nat_eq(a6306, a6307) {
                                let a6308 = v1;
                                let a6309 = 2u64;
                                if nat_eq(a6308, a6309) {
                                    Some(6u64)
                                } else {
                                    let a6310 = v1;
                                    let a6311 = 0u64;
                                    if nat_eq(a6310, a6311) {
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

pub fn f281(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6312 = v0;
    let a6313 = 30u64;
    if nat_eq(a6312, a6313) {
        let a6314 = v2;
        let a6315 = 0u64;
        if if if nat_eq(a6314, a6315) {
            true
        } else {
            let a6316 = v2;
            let a6317 = 1u64;
            nat_eq(a6316, a6317)
        } {
            let a6318 = v1;
            let a6319 = 0u64;
            if nat_eq(a6318, a6319) {
                true
            } else {
                f293(v1, v4)
            }
        } else {
            false
        } {
            f294(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f295(v1) {
            f295(v2)
        } else {
            false
        } {
            let a6320 = v1;
            let a6321 = 0u64;
            if if nat_eq(a6320, a6321) {
                true
            } else {
                let a6322 = v2;
                let a6323 = 0u64;
                nat_eq(a6322, a6323)
            } {
                true
            } else {
                let a6324 = v1;
                let a6325 = v2;
                nat_eq(a6324, a6325)
            }
        } else {
            false
        } {
            true
        } else {
            let a6326 = v1;
            let a6327 = 43u64;
            if nat_eq(a6326, a6327) {
                f295(v2)
            } else {
                false
            }
        } {
            f294(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f282(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6328, _, _, _, _, _, _, _) => {
            h6328
        }
    };
    let v7: u64 = {
        let a6331 = 1u64;
        let a6332 = {
            let a6329 = 3u64;
            let a6330 = v6;
            nat_mul(a6329, a6330)?
        };
        nat_add(a6331, a6332)?
    };
    let a6333 = v0;
    let a6334 = 0u64;
    if nat_eq(a6333, a6334) {
        Ok(f296(0u64))
    } else {
        let a6335 = v0;
        let a6336 = v7;
        if nat_lt(a6335, a6336) {
            let v8: u64 = {
                let a6337 = v0;
                let a6338 = 1u64;
                nat_sub(a6337, a6338)
            };
            let v9: u64 = {
                let a6344 = {
                    let a6339 = v1;
                    let a6340 = v6;
                    nat_mul(a6339, a6340)?
                };
                let a6345 = {
                    let a6341 = v8;
                    let a6342 = 3u64;
                    let a6343 = 0u64;
                    nat_quot(a6341, a6342, a6343)
                };
                nat_add(a6344, a6345)?
            };
            let v10: u64 = f297(v9, v2)?;
            let v11: u64 = {
                let a6346 = v8;
                let a6347 = 3u64;
                let a6348 = 0u64;
                nat_rem(a6346, a6347, a6348)
            };
            let a6349 = v11;
            let a6350 = 0u64;
            if nat_eq(a6349, a6350) {
                Ok(f298(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6351 = v11;
                let a6352 = 1u64;
                if nat_eq(a6351, a6352) {
                    Ok(f298(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f298(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6353 = v0;
            let a6354 = v7;
            if nat_eq(a6353, a6354) {
                Ok(f298(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6357 = v0;
                let a6358 = {
                    let a6355 = v7;
                    let a6356 = 1u64;
                    nat_add(a6355, a6356)?
                };
                if nat_eq(a6357, a6358) {
                    Ok(f298(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6361 = v0;
                    let a6362 = {
                        let a6359 = v7;
                        let a6360 = 2u64;
                        nat_add(a6359, a6360)?
                    };
                    if nat_eq(a6361, a6362) {
                        Ok(f298(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6365 = v0;
                        let a6366 = {
                            let a6363 = v7;
                            let a6364 = 3u64;
                            nat_add(a6363, a6364)?
                        };
                        if nat_eq(a6365, a6366) {
                            let a6367 = v5;
                            let a6368 = 1u64;
                            if if nat_eq(a6367, a6368) {
                                true
                            } else {
                                let a6369 = v5;
                                let a6370 = 2u64;
                                nat_eq(a6369, a6370)
                            } {
                                Ok(f298(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f298(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6377 = {
                                    let a6371 = v1;
                                    let a6372 = v6;
                                    nat_mul(a6371, a6372)?
                                };
                                let a6378 = {
                                    let a6375 = v0;
                                    let a6376 = {
                                        let a6373 = v7;
                                        let a6374 = 4u64;
                                        nat_add(a6373, a6374)?
                                    };
                                    nat_sub(a6375, a6376)
                                };
                                nat_add(a6377, a6378)?
                            };
                            Ok(f298(10u64, 0u64, f297(v12, v2)?, 1u64, 2u64, 3u64, f297(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f283(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6379, _, _, _, _, _, _, _) => {
            h6379
        }
    }, match v0.clone() {
        Adt20::C0(_, h6380, _, _, _, _, _, _) => {
            h6380
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6381, _, _, _, _, _) => {
            h6381
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6382, _, _, _, _) => {
            h6382
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6383, _, _) => {
            h6383
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6384, _) => {
            h6384
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6385) => {
            h6385
        }
    })
}

pub fn f284(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6392 = v3;
        let a6393 = {
            let a6389 = f297({
                let a6386 = v0;
                let a6387 = 2u64;
                let a6388 = 0u64;
                nat_quot(a6386, a6387, a6388)
            }, v1)?;
            let a6390 = v2;
            let a6391 = 0u64;
            nat_quot(a6389, a6390, a6391)
        };
        nat_add(a6392, a6393)?
    };
    let a6397 = {
        let a6394 = v0;
        let a6395 = 2u64;
        let a6396 = 0u64;
        nat_rem(a6394, a6395, a6396)
    };
    let a6398 = 0u64;
    if nat_eq(a6397, a6398) {
        Ok(f298(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6399 = v4;
        let a6400 = 0u64;
        if nat_eq(a6399, a6400) {
            Ok(f298(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f285(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f299(match v2.clone() {
        Adt20::C0(h6401, _, _, _, _, _, _, _) => {
            h6401
        }
    })?;
    let v7: u64 = f300(match v2.clone() {
        Adt20::C0(h6402, _, _, _, _, _, _, _) => {
            h6402
        }
    })?;
    let a6403 = v0;
    let a6404 = 2u64;
    if nat_lt(a6403, a6404) {
        Ok(f296(v0))
    } else {
        let a6405 = v0;
        let a6406 = v6;
        if nat_lt(a6405, a6406) {
            f301({
                let a6407 = v0;
                let a6408 = 2u64;
                nat_sub(a6407, a6408)
            }, v1, v2, v3)
        } else {
            let a6409 = v0;
            let a6410 = v7;
            if nat_lt(a6409, a6410) {
                Ok(f302({
                    let a6411 = v0;
                    let a6412 = v6;
                    nat_sub(a6411, a6412)
                }, match v2 {
                    Adt20::C0(h6413, _, _, _, _, _, _, _) => {
                        h6413
                    }
                }, v5))
            } else {
                let a6415 = v0;
                let a6416 = f255(match v2.clone() {
                    Adt20::C0(h6414, _, _, _, _, _, _, _) => {
                        h6414
                    }
                })?;
                if nat_lt(a6415, a6416) {
                    f303({
                        let a6417 = v0;
                        let a6418 = v7;
                        nat_sub(a6417, a6418)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f286(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f271(v5, match v2.clone() {
        Adt20::C0(h6419, _, _, _, _, _, _, _) => {
            h6419
        }
    })?;
    let v7: u64 = {
        let a6421 = v6;
        let a6422 = f272(v5, match v2.clone() {
            Adt20::C0(h6420, _, _, _, _, _, _, _) => {
                h6420
            }
        })?;
        nat_add(a6421, a6422)?
    };
    let v8: u64 = {
        let a6423 = v7;
        let a6424 = f273(v5);
        nat_add(a6423, a6424)?
    };
    let a6425 = v0;
    let a6426 = v6;
    if nat_lt(a6425, a6426) {
        f304(v0, v1, v2, v5)
    } else {
        let a6427 = v0;
        let a6428 = v7;
        if nat_lt(a6427, a6428) {
            f305({
                let a6429 = v0;
                let a6430 = v6;
                nat_sub(a6429, a6430)
            }, v1, v2, v3, v5)
        } else {
            let a6431 = v0;
            let a6432 = v8;
            if nat_lt(a6431, a6432) {
                Ok(f306({
                    let a6433 = v0;
                    let a6434 = v7;
                    nat_sub(a6433, a6434)
                }, match v2 {
                    Adt20::C0(h6435, _, _, _, _, _, _, _) => {
                        h6435
                    }
                }, v4, v5))
            } else {
                let a6439 = v0;
                let a6440 = {
                    let a6437 = v8;
                    let a6438 = match v2.clone() {
                        Adt20::C0(h6436, _, _, _, _, _, _, _) => {
                            h6436
                        }
                    };
                    nat_add(a6437, a6438)?
                };
                if nat_lt(a6439, a6440) {
                    let v9: u64 = f297({
                        let a6446 = {
                            let a6442 = v1;
                            let a6443 = match v2 {
                                Adt20::C0(h6441, _, _, _, _, _, _, _) => {
                                    h6441
                                }
                            };
                            nat_mul(a6442, a6443)?
                        };
                        let a6447 = {
                            let a6444 = v0;
                            let a6445 = v8;
                            nat_sub(a6444, a6445)
                        };
                        nat_add(a6446, a6447)?
                    }, v3)?;
                    Ok(f298(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f287(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f274(v3, v5)?;
    let v7: u64 = f276(v1, v2, v6)?;
    let v8: u64 = {
        let a6450 = {
            let a6448 = f275(v3, v5)?;
            let a6449 = v4;
            nat_mul(a6448, a6449)?
        };
        let a6451 = v7;
        nat_mul(a6450, a6451)?
    };
    let a6452 = v0;
    let a6453 = v8;
    if nat_lt(a6452, a6453) {
        let v9: u64 = {
            let a6454 = v0;
            let a6455 = v7;
            let a6456 = 0u64;
            nat_quot(a6454, a6455, a6456)
        };
        Ok(Adt21::C0({
            let a6460 = {
                let a6457 = v9;
                let a6458 = v4;
                let a6459 = 0u64;
                nat_quot(a6457, a6458, a6459)
            };
            let a6461 = v6;
            nat_mul(a6460, a6461)?
        }, v6, {
            let a6462 = v9;
            let a6463 = v4;
            let a6464 = 0u64;
            nat_rem(a6462, a6463, a6464)
        }, {
            let a6465 = v0;
            let a6466 = v7;
            let a6467 = 0u64;
            nat_rem(a6465, a6466, a6467)
        }))
    } else {
        let v10: u64 = {
            let a6468 = v0;
            let a6469 = v8;
            nat_sub(a6468, a6469)
        };
        let v11: u64 = f276(v1, v2, f277(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6470 = f275(v3, v5)?;
            let a6471 = v6;
            nat_mul(a6470, a6471)?
        }, f277(v3, v5)?, {
            let a6472 = v10;
            let a6473 = v11;
            let a6474 = 0u64;
            nat_quot(a6472, a6473, a6474)
        }, {
            let a6475 = v10;
            let a6476 = v11;
            let a6477 = 0u64;
            nat_rem(a6475, a6476, a6477)
        }))
    }
}

pub fn f288(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h6478) => {
            h6478
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h6479, _, _, _, _, _, _, _) => {
            h6479
        }
    };
    let v6: u64 = {
        let a6482 = match v0.clone() {
            Adt21::C0(_, h6480, _, _) => {
                h6480
            }
        };
        let a6483 = match v1.clone() {
            Adt20::C0(_, h6481, _, _, _, _, _, _) => {
                h6481
            }
        };
        nat_mul(a6482, a6483)?
    };
    let v7: u64 = {
        let a6484 = v6;
        let a6485 = v5;
        nat_mul(a6484, a6485)?
    };
    let v8: u64 = {
        let a6488 = {
            let a6486 = 2u64;
            let a6487 = v5;
            nat_mul(a6486, a6487)?
        };
        let a6489 = 2u64;
        nat_add(a6488, a6489)?
    };
    let v9: u64 = {
        let a6492 = {
            let a6490 = 4u64;
            let a6491 = v5;
            nat_mul(a6490, a6491)?
        };
        let a6493 = 2u64;
        nat_add(a6492, a6493)?
    };
    let v10: u64 = {
        let a6496 = 1u64;
        let a6497 = {
            let a6494 = v6;
            let a6495 = v8;
            nat_mul(a6494, a6495)?
        };
        nat_add(a6496, a6497)?
    };
    let v11: u64 = {
        let a6498 = v10;
        let a6499 = 4u64;
        nat_add(a6498, a6499)?
    };
    let v12: u64 = {
        let a6502 = v11;
        let a6503 = {
            let a6500 = v6;
            let a6501 = v9;
            nat_mul(a6500, a6501)?
        };
        nat_add(a6502, a6503)?
    };
    let v13: u64 = {
        let a6504 = v12;
        let a6505 = 6u64;
        nat_add(a6504, a6505)?
    };
    let a6506 = v4;
    let a6507 = 0u64;
    if nat_eq(a6506, a6507) {
        Ok(f296(0u64))
    } else {
        let a6508 = v4;
        let a6509 = v10;
        if nat_lt(a6508, a6509) {
            let v14: u64 = {
                let a6510 = v4;
                let a6511 = 1u64;
                nat_sub(a6510, a6511)
            };
            f307(false, {
                let a6512 = v14;
                let a6513 = v8;
                let a6514 = 0u64;
                nat_rem(a6512, a6513, a6514)
            }, {
                let a6515 = v14;
                let a6516 = v8;
                let a6517 = 0u64;
                nat_quot(a6515, a6516, a6517)
            }, v0, v1, v2)
        } else {
            let a6518 = v4;
            let a6519 = v11;
            if nat_lt(a6518, a6519) {
                Ok(f308({
                    let a6520 = v4;
                    let a6521 = v10;
                    nat_sub(a6520, a6521)
                }, v7))
            } else {
                let a6522 = v4;
                let a6523 = v12;
                if nat_lt(a6522, a6523) {
                    let v15: u64 = {
                        let a6524 = v4;
                        let a6525 = v11;
                        nat_sub(a6524, a6525)
                    };
                    f307(true, {
                        let a6526 = v15;
                        let a6527 = v9;
                        let a6528 = 0u64;
                        nat_rem(a6526, a6527, a6528)
                    }, {
                        let a6529 = v15;
                        let a6530 = v9;
                        let a6531 = 0u64;
                        nat_quot(a6529, a6530, a6531)
                    }, v0, v1, v2)
                } else {
                    let a6532 = v4;
                    let a6533 = v13;
                    if nat_lt(a6532, a6533) {
                        Ok(f309({
                            let a6534 = v4;
                            let a6535 = v12;
                            nat_sub(a6534, a6535)
                        }, v7, v3))
                    } else {
                        let a6538 = v4;
                        let a6539 = {
                            let a6536 = v13;
                            let a6537 = v7;
                            nat_add(a6536, a6537)?
                        };
                        if nat_lt(a6538, a6539) {
                            let v16: u64 = {
                                let a6540 = v4;
                                let a6541 = v13;
                                nat_sub(a6540, a6541)
                            };
                            let v17: u64 = {
                                let a6542 = v16;
                                let a6543 = v5;
                                let a6544 = 0u64;
                                nat_quot(a6542, a6543, a6544)
                            };
                            let v18: u64 = {
                                let a6550 = {
                                    let a6545 = f310(v17, v0, v1)?;
                                    let a6546 = v5;
                                    nat_mul(a6545, a6546)?
                                };
                                let a6551 = {
                                    let a6547 = v16;
                                    let a6548 = v5;
                                    let a6549 = 0u64;
                                    nat_rem(a6547, a6548, a6549)
                                };
                                nat_add(a6550, a6551)?
                            };
                            let v19: u64 = f297(v18, v2)?;
                            Ok(f298(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f289(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f290(v0: u64) -> u64 {
    let a6552 = v0;
    let a6553 = 4u64;
    let a6554 = 0u64;
    nat_quot(a6552, a6553, a6554)
}

pub fn f291(v0: u64) -> R<u64> {
    let a6557 = {
        let a6555 = 7u64;
        let a6556 = v0;
        nat_mul(a6555, a6556)?
    };
    let a6558 = 4u64;
    nat_add(a6557, a6558)
}

pub fn f292(v0: u64) -> bool {
    let m6559 = f202(v0);
    match m6559 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f293(v0: u64, v1: u64) -> bool {
    let a6560 = v0;
    let a6561 = 1u64;
    if nat_eq(a6560, a6561) {
        true
    } else {
        let v2: u64 = f311(v0);
        let a6562 = 0u64;
        let a6563 = v2;
        if nat_lt(a6562, a6563) {
            let a6567 = {
                let a6564 = v1;
                let a6565 = v2;
                let a6566 = 1u64;
                nat_rem(a6564, a6565, a6566)
            };
            let a6568 = 0u64;
            nat_eq(a6567, a6568)
        } else {
            false
        }
    }
}

pub fn f294(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6569 = v0;
    let a6570 = 28u64;
    if if if if nat_eq(a6569, a6570) {
        let a6571 = v1;
        let a6572 = 0u64;
        nat_eq(a6571, a6572)
    } else {
        false
    } {
        let a6573 = v2;
        let a6574 = 0u64;
        nat_eq(a6573, a6574)
    } else {
        false
    } {
        let a6575 = v3;
        let a6576 = 0u64;
        nat_eq(a6575, a6576)
    } else {
        false
    } {
        let a6577 = v4;
        let a6578 = v8;
        if if if if if if if if nat_eq(a6577, a6578) {
            let a6579 = v5;
            let a6580 = v12;
            nat_eq(a6579, a6580)
        } else {
            false
        } {
            let a6581 = v9;
            let a6582 = v13;
            nat_eq(a6581, a6582)
        } else {
            false
        } {
            let a6583 = v10;
            let a6584 = v14;
            nat_eq(a6583, a6584)
        } else {
            false
        } {
            let a6585 = v11;
            let a6586 = v15;
            nat_eq(a6585, a6586)
        } else {
            false
        } {
            let a6587 = 0u64;
            let a6588 = v6;
            nat_lt(a6587, a6588)
        } else {
            false
        } {
            let a6589 = 0u64;
            let a6590 = v7;
            nat_lt(a6589, a6590)
        } else {
            false
        } {
            let a6594 = {
                let a6591 = v10;
                let a6592 = v6;
                let a6593 = 1u64;
                nat_rem(a6591, a6592, a6593)
            };
            let a6595 = 0u64;
            if nat_eq(a6594, a6595) {
                let a6599 = {
                    let a6596 = v11;
                    let a6597 = v7;
                    let a6598 = 1u64;
                    nat_rem(a6596, a6597, a6598)
                };
                let a6600 = 0u64;
                nat_eq(a6599, a6600)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a6601 = v10;
                let a6602 = v6;
                let a6603 = 0u64;
                nat_quot(a6601, a6602, a6603)
            }, {
                let a6604 = v11;
                let a6605 = v7;
                let a6606 = 0u64;
                nat_quot(a6604, a6605, a6606)
            }, {
                let a6607 = v5;
                let a6608 = v4;
                nat_mul(a6607, a6608)?
            }, {
                let a6609 = v4;
                let a6610 = v9;
                nat_mul(a6609, a6610)?
            }, {
                let a6611 = v5;
                let a6612 = v9;
                nat_mul(a6611, a6612)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f295(v0: u64) -> bool {
    let a6613 = v0;
    let a6614 = 0u64;
    if if nat_eq(a6613, a6614) {
        true
    } else {
        let a6615 = v0;
        let a6616 = 1u64;
        nat_eq(a6615, a6616)
    } {
        true
    } else {
        let a6617 = v0;
        let a6618 = 30u64;
        nat_eq(a6617, a6618)
    }
}

pub fn f296(v0: u64) -> Adt17 {
    f298(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f297(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6620 = v0;
        let a6621 = match v1.clone() {
            Adt20::C0(h6619, _, _, _, _, _, _, _) => {
                h6619
            }
        };
        let a6622 = 0u64;
        nat_quot(a6620, a6621, a6622)
    };
    let v3: u64 = {
        let a6624 = v2;
        let a6625 = match v1.clone() {
            Adt20::C0(_, h6623, _, _, _, _, _, _) => {
                h6623
            }
        };
        let a6626 = 0u64;
        nat_quot(a6624, a6625, a6626)
    };
    f131({
        let a6628 = v0;
        let a6629 = match v1.clone() {
            Adt20::C0(h6627, _, _, _, _, _, _, _) => {
                h6627
            }
        };
        let a6630 = 0u64;
        nat_rem(a6628, a6629, a6630)
    }, {
        let a6632 = v2;
        let a6633 = match v1.clone() {
            Adt20::C0(_, h6631, _, _, _, _, _, _) => {
                h6631
            }
        };
        let a6634 = 0u64;
        nat_rem(a6632, a6633, a6634)
    }, {
        let a6636 = v3;
        let a6637 = match v1.clone() {
            Adt20::C0(_, _, h6635, _, _, _, _, _) => {
                h6635
            }
        };
        let a6638 = 0u64;
        nat_rem(a6636, a6637, a6638)
    }, {
        let a6640 = v3;
        let a6641 = match v1.clone() {
            Adt20::C0(_, _, h6639, _, _, _, _, _) => {
                h6639
            }
        };
        let a6642 = 0u64;
        nat_quot(a6640, a6641, a6642)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6643, _, _, _) => {
            h6643
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6644, _, _) => {
            h6644
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6645, _) => {
            h6645
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6646) => {
            h6646
        }
    })
}

pub fn f298(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a6647 = v0;
    let a6648 = 4u64;
    if if if nat_le(a6647, a6648) {
        true
    } else {
        let a6649 = 9u64;
        let a6650 = v0;
        if nat_le(a6649, a6650) {
            let a6651 = v0;
            let a6652 = 12u64;
            nat_le(a6651, a6652)
        } else {
            false
        }
    } {
        true
    } else {
        let a6653 = v0;
        let a6654 = 16u64;
        if if nat_eq(a6653, a6654) {
            true
        } else {
            let a6655 = v0;
            let a6656 = 19u64;
            nat_eq(a6655, a6656)
        } {
            true
        } else {
            let a6657 = v0;
            let a6658 = 21u64;
            if nat_eq(a6657, a6658) {
                true
            } else {
                let a6659 = v0;
                let a6660 = 22u64;
                nat_eq(a6659, a6660)
            }
        }
    } {
        Adt17::C1({
            let a6661 = v0;
            let a6662 = 4u64;
            if if nat_lt(a6661, a6662) {
                true
            } else {
                let a6663 = 21u64;
                let a6664 = v0;
                nat_le(a6663, a6664)
            } {
                0u64
            } else {
                let a6665 = v0;
                let a6666 = 16u64;
                if if nat_eq(a6665, a6666) {
                    true
                } else {
                    let a6667 = v0;
                    let a6668 = 19u64;
                    nat_eq(a6667, a6668)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6669 = v0;
            let a6670 = 4u64;
            if nat_lt(a6669, a6670) {
                v0
            } else {
                let a6671 = v0;
                let a6672 = 4u64;
                if if nat_eq(a6671, a6672) {
                    true
                } else {
                    let a6673 = v0;
                    let a6674 = 16u64;
                    nat_eq(a6673, a6674)
                } {
                    0u64
                } else {
                    let a6675 = v0;
                    let a6676 = 19u64;
                    if nat_eq(a6675, a6676) {
                        3u64
                    } else {
                        let a6677 = 21u64;
                        let a6678 = v0;
                        if nat_le(a6677, a6678) {
                            let a6679 = v0;
                            let a6680 = 17u64;
                            nat_sub(a6679, a6680)
                        } else {
                            let a6681 = v0;
                            let a6682 = 8u64;
                            nat_sub(a6681, a6682)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f299(v0: u64) -> R<u64> {
    let a6685 = 2u64;
    let a6686 = {
        let a6683 = 6u64;
        let a6684 = v0;
        nat_mul(a6683, a6684)?
    };
    nat_add(a6685, a6686)
}

pub fn f300(v0: u64) -> R<u64> {
    let a6687 = f299(v0)?;
    let a6688 = 11u64;
    nat_add(a6687, a6688)
}

pub fn f301(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a6689 = v0;
        let a6690 = 6u64;
        let a6691 = 0u64;
        nat_rem(a6689, a6690, a6691)
    };
    let v5: u64 = {
        let a6698 = {
            let a6693 = v1;
            let a6694 = match v2.clone() {
                Adt20::C0(h6692, _, _, _, _, _, _, _) => {
                    h6692
                }
            };
            nat_mul(a6693, a6694)?
        };
        let a6699 = {
            let a6695 = v0;
            let a6696 = 6u64;
            let a6697 = 0u64;
            nat_quot(a6695, a6696, a6697)
        };
        nat_add(a6698, a6699)?
    };
    let v6: u64 = f297(v5, v3)?;
    let v7: u64 = f297(v5, v2)?;
    let a6700 = v4;
    let a6701 = 0u64;
    if nat_eq(a6700, a6701) {
        Ok(f298(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6702 = v4;
        let a6703 = 1u64;
        if nat_eq(a6702, a6703) {
            Ok(f298(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6704 = v4;
            let a6705 = 2u64;
            if nat_eq(a6704, a6705) {
                Ok(f298(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6706 = v4;
                let a6707 = 3u64;
                if nat_eq(a6706, a6707) {
                    Ok(f298(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6708 = v4;
                    let a6709 = 4u64;
                    if nat_eq(a6708, a6709) {
                        Ok(f298(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f298(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f302(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6710 = v0;
    let a6711 = 0u64;
    if nat_eq(a6710, a6711) {
        f298(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6712 = v0;
        let a6713 = 1u64;
        if nat_eq(a6712, a6713) {
            f298(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6714 = v0;
            let a6715 = 2u64;
            if nat_eq(a6714, a6715) {
                f298(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6716 = v0;
                let a6717 = 3u64;
                if nat_eq(a6716, a6717) {
                    f298(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6718 = v0;
                    let a6719 = 4u64;
                    if nat_eq(a6718, a6719) {
                        f298(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6720 = v0;
                        let a6721 = 5u64;
                        if nat_eq(a6720, a6721) {
                            f298(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6722 = v0;
                            let a6723 = 6u64;
                            if nat_eq(a6722, a6723) {
                                f298(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6724 = v0;
                                let a6725 = 7u64;
                                if nat_eq(a6724, a6725) {
                                    f298(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6726 = v0;
                                    let a6727 = 8u64;
                                    if nat_eq(a6726, a6727) {
                                        f298(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6728 = v0;
                                        let a6729 = 9u64;
                                        if nat_eq(a6728, a6729) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6730 = v0;
                                            let a6731 = 10u64;
                                            if nat_eq(a6730, a6731) {
                                                f298(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f303(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6732 = v0;
        let a6733 = 3u64;
        let a6734 = 0u64;
        nat_rem(a6732, a6733, a6734)
    };
    let v6: u64 = {
        let a6741 = {
            let a6736 = v1;
            let a6737 = match v2.clone() {
                Adt20::C0(h6735, _, _, _, _, _, _, _) => {
                    h6735
                }
            };
            nat_mul(a6736, a6737)?
        };
        let a6742 = {
            let a6738 = v0;
            let a6739 = 3u64;
            let a6740 = 0u64;
            nat_quot(a6738, a6739, a6740)
        };
        nat_add(a6741, a6742)?
    };
    let v7: u64 = f297(v6, v3)?;
    let v8: u64 = f297(v6, v2)?;
    let v9: u64 = f297(v6, v4)?;
    let a6743 = v5;
    let a6744 = 0u64;
    if nat_eq(a6743, a6744) {
        Ok(f298(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6745 = v5;
        let a6746 = 1u64;
        if nat_eq(a6745, a6746) {
            Ok(f298(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f298(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f304(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6747, _, _, _, _, _, _, _) => {
            h6747
        }
    };
    let v5: u64 = {
        let a6750 = 1u64;
        let a6751 = {
            let a6748 = v4;
            let a6749 = f289(v3);
            nat_mul(a6748, a6749)?
        };
        nat_add(a6750, a6751)?
    };
    let a6752 = v0;
    let a6753 = 0u64;
    if nat_eq(a6752, a6753) {
        Ok(f296(0u64))
    } else {
        let a6754 = v0;
        let a6755 = v5;
        if nat_lt(a6754, a6755) {
            let v6: u64 = {
                let a6756 = v0;
                let a6757 = 1u64;
                nat_sub(a6756, a6757)
            };
            let v7: u64 = f297({
                let a6763 = {
                    let a6758 = v1;
                    let a6759 = v4;
                    nat_mul(a6758, a6759)?
                };
                let a6764 = {
                    let a6760 = v6;
                    let a6761 = f289(v3);
                    let a6762 = 0u64;
                    nat_quot(a6760, a6761, a6762)
                };
                nat_add(a6763, a6764)?
            }, v2)?;
            if v3 {
                let a6768 = {
                    let a6765 = v6;
                    let a6766 = 2u64;
                    let a6767 = 0u64;
                    nat_rem(a6765, a6766, a6767)
                };
                let a6769 = 0u64;
                if nat_eq(a6768, a6769) {
                    Ok(f298(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f298(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f298(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6770 = v0;
                let a6771 = v5;
                nat_sub(a6770, a6771)
            };
            let a6772 = v8;
            let a6773 = 0u64;
            if nat_eq(a6772, a6773) {
                Ok(f298(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6774 = v8;
                    let a6775 = 1u64;
                    nat_eq(a6774, a6775)
                } else {
                    false
                } {
                    Ok(f298(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6776 = v8;
                    let a6777 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6776, a6777) {
                        Ok(f298(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6778 = v8;
                        let a6779 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6778, a6779) {
                            Ok(f296(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f305(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a6781 = 13u64;
        let a6782 = f290(match v2.clone() {
            Adt20::C0(h6780, _, _, _, _, _, _, _) => {
                h6780
            }
        });
        nat_mul(a6781, a6782)?
    } else {
        0u64
    };
    let a6783 = v0;
    let a6784 = v5;
    if nat_lt(a6783, a6784) {
        f312({
            let a6785 = v0;
            let a6786 = 13u64;
            let a6787 = 0u64;
            nat_rem(a6785, a6786, a6787)
        }, v1, {
            let a6788 = v0;
            let a6789 = 13u64;
            let a6790 = 0u64;
            nat_quot(a6788, a6789, a6790)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6791 = v0;
            let a6792 = v5;
            nat_sub(a6791, a6792)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f313({
            let a6793 = v6;
            let a6794 = v7;
            let a6795 = 0u64;
            nat_rem(a6793, a6794, a6795)
        }, {
            let a6805 = {
                let a6800 = {
                    let a6797 = v1;
                    let a6798 = match v2.clone() {
                        Adt20::C0(h6796, _, _, _, _, _, _, _) => {
                            h6796
                        }
                    };
                    nat_mul(a6797, a6798)?
                };
                let a6801 = if v4 {
                    f314(match v2.clone() {
                        Adt20::C0(h6799, _, _, _, _, _, _, _) => {
                            h6799
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6800, a6801)?
            };
            let a6806 = {
                let a6802 = v6;
                let a6803 = v7;
                let a6804 = 0u64;
                nat_quot(a6802, a6803, a6804)
            };
            nat_add(a6805, a6806)?
        }, v2, v3, v4)
    }
}

pub fn f306(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a6807 = v0;
    let a6808 = 0u64;
    if nat_eq(a6807, a6808) {
        f298(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6809 = v0;
        let a6810 = 1u64;
        if nat_eq(a6809, a6810) {
            f298(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6811 = v0;
                let a6812 = 2u64;
                nat_eq(a6811, a6812)
            } else {
                false
            } {
                f298(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6813 = v0;
                let a6814 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6813, a6814) {
                    f298(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6815 = v0;
                    let a6816 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6815, a6816) {
                        f298(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6817 = v0;
                        let a6818 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6817, a6818) {
                            f298(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f307(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6822 = 1u64;
        let a6823 = {
            let a6820 = match v4.clone() {
                Adt20::C0(h6819, _, _, _, _, _, _, _) => {
                    h6819
                }
            };
            let a6821 = v6;
            nat_mul(a6820, a6821)?
        };
        nat_add(a6822, a6823)?
    };
    let a6824 = v1;
    let a6825 = 0u64;
    if nat_eq(a6824, a6825) {
        Ok(f296(1u64))
    } else {
        let a6826 = v1;
        let a6827 = v7;
        if nat_lt(a6826, a6827) {
            let v8: u64 = {
                let a6828 = v1;
                let a6829 = 1u64;
                nat_sub(a6828, a6829)
            };
            let v9: u64 = {
                let a6836 = {
                    let a6831 = f310(v2, v3, v4.clone())?;
                    let a6832 = match v4.clone() {
                        Adt20::C0(h6830, _, _, _, _, _, _, _) => {
                            h6830
                        }
                    };
                    nat_mul(a6831, a6832)?
                };
                let a6837 = {
                    let a6833 = v8;
                    let a6834 = v6;
                    let a6835 = 0u64;
                    nat_quot(a6833, a6834, a6835)
                };
                nat_add(a6836, a6837)?
            };
            let v10: u64 = f297(v9, v4)?;
            let v11: u64 = f297(v9, v5)?;
            let v12: u64 = {
                let a6838 = v8;
                let a6839 = v6;
                let a6840 = 0u64;
                nat_rem(a6838, a6839, a6840)
            };
            if v0 {
                let a6841 = v12;
                let a6842 = 0u64;
                if nat_eq(a6841, a6842) {
                    Ok(f298(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6843 = v12;
                    let a6844 = 1u64;
                    if nat_eq(a6843, a6844) {
                        Ok(f298(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6845 = v12;
                        let a6846 = 2u64;
                        if nat_eq(a6845, a6846) {
                            Ok(f298(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f298(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6847 = v12;
                let a6848 = 0u64;
                if nat_eq(a6847, a6848) {
                    Ok(f298(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f298(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6849 = v1;
            let a6850 = v7;
            if nat_eq(a6849, a6850) {
                Ok(f298(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f308(v0: u64, v1: u64) -> Adt17 {
    let a6851 = v0;
    let a6852 = 0u64;
    if nat_eq(a6851, a6852) {
        f298(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6853 = v0;
        let a6854 = 1u64;
        if nat_eq(a6853, a6854) {
            f298(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6855 = v0;
            let a6856 = 2u64;
            if nat_eq(a6855, a6856) {
                f298(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6857 = v0;
                let a6858 = 3u64;
                if nat_eq(a6857, a6858) {
                    f296(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f309(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6859 = v0;
    let a6860 = 0u64;
    if nat_eq(a6859, a6860) {
        f298(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6861 = v0;
        let a6862 = 1u64;
        if nat_eq(a6861, a6862) {
            f298(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6863 = v0;
            let a6864 = 2u64;
            if nat_eq(a6863, a6864) {
                f298(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6865 = v0;
                let a6866 = 3u64;
                if nat_eq(a6865, a6866) {
                    f298(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6867 = v0;
                    let a6868 = 4u64;
                    if nat_eq(a6867, a6868) {
                        f298(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6869 = v0;
                        let a6870 = 5u64;
                        if nat_eq(a6869, a6870) {
                            f298(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f310(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a6881 = {
        let a6879 = {
            let a6876 = {
                let a6873 = match v1.clone() {
                    Adt21::C0(_, _, h6871, _) => {
                        h6871
                    }
                };
                let a6874 = match v2.clone() {
                    Adt20::C0(_, _, h6872, _, _, _, _, _) => {
                        h6872
                    }
                };
                nat_mul(a6873, a6874)?
            };
            let a6877 = match v1 {
                Adt21::C0(h6875, _, _, _) => {
                    h6875
                }
            };
            nat_add(a6876, a6877)?
        };
        let a6880 = match v2 {
            Adt20::C0(_, h6878, _, _, _, _, _, _) => {
                h6878
            }
        };
        nat_mul(a6879, a6880)?
    };
    let a6882 = v0;
    nat_add(a6881, a6882)
}

pub fn f311(v0: u64) -> u64 {
    if f209(v0) {
        let m6883 = f202(v0);
        match m6883 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a6885 = 1u64;
                let a6886 = match v1.clone() {
                    Adt19::C0(_, h6884, _) => {
                        h6884
                    }
                };
                if if nat_lt(a6885, a6886) {
                    true
                } else {
                    let a6887 = v0;
                    let a6888 = 43u64;
                    nat_eq(a6887, a6888)
                } {
                    match v1 {
                        Adt19::C0(_, h6889, _) => {
                            h6889
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

pub fn f312(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6897 = {
            let a6895 = {
                let a6891 = v1;
                let a6892 = match v3.clone() {
                    Adt20::C0(h6890, _, _, _, _, _, _, _) => {
                        h6890
                    }
                };
                nat_mul(a6891, a6892)?
            };
            let a6896 = {
                let a6893 = v2;
                let a6894 = 4u64;
                nat_mul(a6893, a6894)?
            };
            nat_add(a6895, a6896)?
        };
        let a6898 = f315(v0);
        nat_add(a6897, a6898)?
    };
    let v6: u64 = f297(v5, v3)?;
    let v7: u64 = f297(v5, v4)?;
    let a6899 = v0;
    let a6900 = 0u64;
    if if if if nat_eq(a6899, a6900) {
        true
    } else {
        let a6901 = v0;
        let a6902 = 2u64;
        nat_eq(a6901, a6902)
    } {
        true
    } else {
        let a6903 = v0;
        let a6904 = 5u64;
        nat_eq(a6903, a6904)
    } {
        true
    } else {
        let a6905 = v0;
        let a6906 = 7u64;
        nat_eq(a6905, a6906)
    } {
        Ok(f298(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6907 = v0;
        let a6908 = 1u64;
        if if if if nat_eq(a6907, a6908) {
            true
        } else {
            let a6909 = v0;
            let a6910 = 3u64;
            nat_eq(a6909, a6910)
        } {
            true
        } else {
            let a6911 = v0;
            let a6912 = 6u64;
            nat_eq(a6911, a6912)
        } {
            true
        } else {
            let a6913 = v0;
            let a6914 = 8u64;
            nat_eq(a6913, a6914)
        } {
            Ok(f298(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6915 = v0;
                let a6916 = 1u64;
                if nat_eq(a6915, a6916) {
                    4u64
                } else {
                    let a6917 = v0;
                    let a6918 = 6u64;
                    if nat_eq(a6917, a6918) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6919 = v0;
            let a6920 = 4u64;
            if nat_eq(a6919, a6920) {
                Ok(f298(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6921 = v0;
                let a6922 = 9u64;
                if nat_eq(a6921, a6922) {
                    Ok(f298(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6923 = v0;
                    let a6924 = 10u64;
                    if nat_eq(a6923, a6924) {
                        Ok(f298(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6925 = v0;
                        let a6926 = 11u64;
                        if nat_eq(a6925, a6926) {
                            Ok(f298(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6927 = v0;
                            let a6928 = 12u64;
                            if nat_eq(a6927, a6928) {
                                Ok(f298(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                            } else {
                                Ok(Adt17::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f313(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f297(v1, v2)?;
    let v6: u64 = f297(v1, v3)?;
    let a6929 = v0;
    let a6930 = 0u64;
    if nat_eq(a6929, a6930) {
        Ok(f298(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6931 = v0;
        let a6932 = 1u64;
        if nat_eq(a6931, a6932) {
            Ok(f298(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6933 = v0;
                let a6934 = 2u64;
                if nat_eq(a6933, a6934) {
                    Ok(f298(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6935 = v0;
                    let a6936 = 3u64;
                    if nat_eq(a6935, a6936) {
                        Ok(f298(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a6937 = v0;
                let a6938 = 2u64;
                if nat_eq(a6937, a6938) {
                    Ok(f298(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f314(v0: u64) -> R<u64> {
    let a6939 = 4u64;
    let a6940 = f290(v0);
    nat_mul(a6939, a6940)
}

pub fn f315(v0: u64) -> u64 {
    let a6941 = v0;
    let a6942 = 5u64;
    if nat_lt(a6941, a6942) {
        let a6943 = v0;
        let a6944 = 2u64;
        if nat_lt(a6943, a6944) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6945 = v0;
        let a6946 = 7u64;
        if nat_lt(a6945, a6946) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
