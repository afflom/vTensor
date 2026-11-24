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
    C40(bool, u64, u64),
    C41,
    C42(Adt6, Adt6),
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
            let a3 = 82u64;
            if if if nat_eq(a2, a3) {
                true
            } else {
                let a4 = v1;
                let a5 = 50u64;
                nat_eq(a4, a5)
            } {
                if if if f1(v5.clone()) {
                    true
                } else {
                    f1(v6.clone())
                } {
                    true
                } else {
                    f1(v7.clone())
                } {
                    true
                } else {
                    v8
                }
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let a6 = v1;
                let a7 = 94u64;
                if if nat_eq(a6, a7) {
                    true
                } else {
                    let a8 = v1;
                    let a9 = 95u64;
                    nat_eq(a8, a9)
                } {
                    Ok(Adt9::C0(f2({
                        let a10 = v1;
                        let a11 = 95u64;
                        nat_eq(a10, a11)
                    }, v3, v4, v5, {
                        let m12 = v6;
                        match m12 {
                            None => {
                                let m13 = v7;
                                match m13 {
                                    None => {
                                        v8
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }
                            Some(_) => {
                                true
                            }
                        }
                    }, v11)?))
                } else {
                    let a14 = v1;
                    let a15 = 96u64;
                    if if nat_eq(a14, a15) {
                        true
                    } else {
                        let a16 = v1;
                        let a17 = 97u64;
                        nat_eq(a16, a17)
                    } {
                        Ok(Adt9::C0(f3({
                            let a18 = v1;
                            let a19 = 96u64;
                            nat_eq(a18, a19)
                        }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                    } else {
                        let v15: bool = {
                            let m20 = v7;
                            match m20 {
                                None => {
                                    v8
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        };
                        let a21 = v1;
                        let a22 = 85u64;
                        if nat_eq(a21, a22) {
                            Ok(Adt9::C0(f4(v2, v3, v4, v5, {
                                let m23 = v6;
                                match m23 {
                                    None => {
                                        v15
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v11)?))
                        } else {
                            let a24 = v1;
                            let a25 = 87u64;
                            if nat_eq(a24, a25) {
                                Ok(Adt9::C0(f5(v3, v4, v5, v6, v15, v11)?))
                            } else {
                                let a26 = v1;
                                let a27 = 64u64;
                                if nat_eq(a26, a27) {
                                    Ok(Adt9::C0(f6(v2, v11)?))
                                } else {
                                    let m28 = v3;
                                    match m28 {
                                        None => {
                                            Ok(Adt9::C0(Adt10::C1))
                                        }
                                        Some(v17) => {
                                            let a29 = v1;
                                            let a30 = 48u64;
                                            if if nat_eq(a29, a30) {
                                                true
                                            } else {
                                                let a31 = v1;
                                                let a32 = 56u64;
                                                nat_eq(a31, a32)
                                            } {
                                                Ok(Adt9::C0(f7({
                                                    let a33 = v1;
                                                    let a34 = 56u64;
                                                    nat_eq(a33, a34)
                                                }, v2, v17, v4, v11)?))
                                            } else {
                                                let a35 = v1;
                                                let a36 = 53u64;
                                                if if nat_eq(a35, a36) {
                                                    true
                                                } else {
                                                    let a37 = v1;
                                                    let a38 = 54u64;
                                                    nat_eq(a37, a38)
                                                } {
                                                    Ok(Adt9::C0(f8({
                                                        let a39 = v1;
                                                        let a40 = 54u64;
                                                        nat_eq(a39, a40)
                                                    }, v2, v17, v4, v11)?))
                                                } else {
                                                    let a41 = v1;
                                                    let a42 = 78u64;
                                                    if nat_eq(a41, a42) {
                                                        Ok(Adt9::C0(f9(v2, v17, v4, v5, v11)?))
                                                    } else {
                                                        let a43 = v1;
                                                        let a44 = 23u64;
                                                        if if if nat_eq(a43, a44) {
                                                            true
                                                        } else {
                                                            let a45 = v1;
                                                            let a46 = 26u64;
                                                            nat_eq(a45, a46)
                                                        } {
                                                            true
                                                        } else {
                                                            let a47 = v1;
                                                            let a48 = 25u64;
                                                            nat_eq(a47, a48)
                                                        } {
                                                            Ok(Adt9::C0(f10(v1, v2, v17, v4, v11)?))
                                                        } else {
                                                            let a49 = v1;
                                                            let a50 = 24u64;
                                                            if nat_eq(a49, a50) {
                                                                Ok(Adt9::C0(f11(v2, v17, v11)?))
                                                            } else {
                                                                let a51 = v1;
                                                                let a52 = 27u64;
                                                                if nat_eq(a51, a52) {
                                                                    Ok(Adt9::C0(f12(v2, v17, v11)?))
                                                                } else {
                                                                    let a53 = v1;
                                                                    let a54 = 3u64;
                                                                    if nat_eq(a53, a54) {
                                                                        Ok(Adt9::C0(f13(v17, v4, v5, v11)?))
                                                                    } else {
                                                                        f14(v1, v2, v17, v4, v11)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v23, v24, v25, v26, v27) => {
            f17(v23, v24, v25, v26, v27)
        }
        Adt0::C2(v38) => {
            f19(v38)
        }
        Adt0::C3(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51) => {
            Ok(f20(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51))
        }
        Adt0::C4(v18, v19, v20, v21) => {
            let a56 = 0u64;
            let a57 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h55, _, _, _, _, _, _, _) => {
                    h55
                }
            };
            if nat_lt(a56, a57) {
                f15(v18, v19, v20, v21)
            } else {
                f16(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a58 = 3240099840u64;
                let a59 = v22;
                if if nat_le(a58, a59) {
                    let a60 = v22;
                    let a61 = 4286578688u64;
                    nat_le(a60, a61)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a62 = 1092616192u64;
                    let a63 = v22;
                    if if nat_le(a62, a63) {
                        let a64 = v22;
                        let a65 = 2139095040u64;
                        nat_le(a64, a65)
                    } else {
                        false
                    } {
                        Some(v22)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v28, v29, v30, v31, v32, v33, v34, v35, v36, v37) => {
            f18(v28, v29, v30, v31, v32, true, v33, v34, v35, v36, v37)
        }
    }
}

pub fn f1(v0: Option<Adt2>) -> bool {
    let m66 = v0;
    match m66 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f2(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m67 = v1.clone();
        match m67 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m68 = v2.clone();
        match m68 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m69 = v1.clone();
        match m69 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m70 = v3.clone();
        match m70 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m71 = v2.clone();
        match m71 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a77 = {
            let a74 = match v10.clone() {
                Adt2::C0(_, _, h72, _, _, _, _, _, _, _, _) => {
                    h72
                }
            };
            let a75 = match v10.clone() {
                Adt2::C0(_, _, _, h73, _, _, _, _, _, _, _) => {
                    h73
                }
            };
            nat_mul(a74, a75)?
        };
        let a78 = match v10.clone() {
            Adt2::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                h76
            }
        };
        nat_mul(a77, a78)?
    };
    let m79 = v1;
    if if if if if if if if if if match m79 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m80 = v2;
        match m80 {
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
            let m81 = v3;
            match m81 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m82 = v3;
            !match m82 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    } else {
        false
    } {
        !v4
    } else {
        false
    } {
        let a83 = 0u64;
        let a84 = v14;
        nat_lt(a83, a84)
    } else {
        false
    } {
        let a86 = 0u64;
        let a87 = match v10.clone() {
            Adt2::C0(_, h85, _, _, _, _, _, _, _, _, _) => {
                h85
            }
        };
        nat_lt(a86, a87)
    } else {
        false
    } {
        f21(v10.clone(), v13.clone())
    } else {
        false
    } {
        f22(v10.clone(), v14, v0)?
    } else {
        false
    } {
        f22(v13.clone(), v14, v0)?
    } else {
        false
    } {
        if v0 {
            let a89 = match v7.clone() {
                Adt2::C0(h88, _, _, _, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            let a90 = 0u64;
            if if if if nat_eq(a89, a90) {
                let a91 = f23(v7.clone(), 1u64)?;
                let a92 = 1u64;
                nat_eq(a91, a92)
            } else {
                false
            } {
                let a94 = 4u64;
                let a95 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h93, _) => {
                        h93
                    }
                };
                nat_le(a94, a95)
            } else {
                false
            } {
                f21(v10.clone(), v5.clone())
            } else {
                false
            } {
                f22(v5.clone(), v14, true)?
            } else {
                false
            }
        } else {
            let a97 = match v5.clone() {
                Adt2::C0(h96, _, _, _, _, _, _, _, _, _, _) => {
                    h96
                }
            };
            let a98 = 0u64;
            if if nat_eq(a97, a98) {
                let a99 = f23(v5.clone(), 1u64)?;
                let a100 = 1u64;
                nat_eq(a99, a100)
            } else {
                false
            } {
                let a102 = 4u64;
                let a103 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h101, _) => {
                        h101
                    }
                };
                nat_le(a102, a103)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a109 = {
                let a107 = 2u64;
                let a108 = {
                    let a105 = v14;
                    let a106 = f24(v0, match v10.clone() {
                        Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                            h104
                        }
                    })?;
                    nat_mul(a105, a106)?
                };
                nat_add(a107, a108)?
            };
            let a110 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a109, a110)?
        }, 4u64, f25(v10, 1u64), f25(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h111, _, _, _, _) => {
                h111
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m112 = v3.clone();
        match m112 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m113 = v4.clone();
        match m113 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m114 = v5.clone();
        match m114 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m115 = v6.clone();
        match m115 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m116 = v7.clone();
        match m116 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m117 = v5.clone();
        match m117 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m118 = v3;
    if if if if if if if if if if if match m118 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m119 = v4;
        match m119 {
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
        let m120 = v5;
        match m120 {
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
            let m121 = v6;
            if match m121 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m122 = v7;
                match m122 {
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
            let m123 = v6;
            if !match m123 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m124 = v7;
                !match m124 {
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
            let a131 = {
                let a128 = {
                    let a125 = v1;
                    let a126 = 4u64;
                    let a127 = 0u64;
                    nat_quot(a125, a126, a127)
                };
                let a129 = 2u64;
                let a130 = 0u64;
                nat_rem(a128, a129, a130)
            };
            let a132 = 1u64;
            nat_eq(a131, a132)
        } else {
            false
        } {
            let a134 = match v11.clone() {
                Adt2::C0(h133, _, _, _, _, _, _, _, _, _, _) => {
                    h133
                }
            };
            let a135 = 0u64;
            nat_eq(a134, a135)
        } else {
            false
        } {
            let a137 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h136, _, _, _, _, _) => {
                    h136
                }
            };
            let a138 = 4u64;
            nat_eq(a137, a138)
        } else {
            false
        } {
            f26(v11.clone(), f23(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f27(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a140 = match v11.clone() {
                Adt2::C0(_, h139, _, _, _, _, _, _, _, _, _) => {
                    h139
                }
            };
            let a141 = 1u64;
            if nat_le(a140, a141) {
                true
            } else {
                let a144 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h142, _, _, _, _, _) => {
                        h142
                    }
                };
                let a145 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h143, _, _, _, _, _) => {
                        h143
                    }
                };
                nat_eq(a144, a145)
            }
        } else {
            false
        } {
            let a147 = match v11.clone() {
                Adt2::C0(_, _, h146, _, _, _, _, _, _, _, _) => {
                    h146
                }
            };
            let a148 = 1u64;
            if nat_le(a147, a148) {
                true
            } else {
                let a151 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h149, _, _, _, _) => {
                        h149
                    }
                };
                let a152 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h150, _, _, _, _) => {
                        h150
                    }
                };
                nat_eq(a151, a152)
            }
        } else {
            false
        } {
            let a154 = match v11.clone() {
                Adt2::C0(_, _, _, h153, _, _, _, _, _, _, _) => {
                    h153
                }
            };
            let a155 = 1u64;
            if nat_le(a154, a155) {
                true
            } else {
                let a158 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h156, _, _, _) => {
                        h156
                    }
                };
                let a159 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h157, _, _, _) => {
                        h157
                    }
                };
                nat_eq(a158, a159)
            }
        } else {
            false
        } {
            let a161 = match v11.clone() {
                Adt2::C0(_, _, _, _, h160, _, _, _, _, _, _) => {
                    h160
                }
            };
            let a162 = 1u64;
            if nat_le(a161, a162) {
                true
            } else {
                let a165 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h163, _, _) => {
                        h163
                    }
                };
                let a166 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h164, _, _) => {
                        h164
                    }
                };
                nat_eq(a165, a166)
            }
        } else {
            false
        }
    } else {
        false
    } {
        f27(v11.clone(), v13)?
    } else {
        false
    } {
        if v0 {
            if f27(v11.clone(), v15)? {
                f27(v11.clone(), v17)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a168 = match v20.clone() {
            Adt2::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
                h167
            }
        };
        let a169 = 0u64;
        nat_eq(a168, a169)
    } else {
        false
    } {
        let a170 = f23(v20.clone(), 1u64)?;
        let a171 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a170, a171)
    } else {
        false
    } {
        let a173 = if v0 {
            28u64
        } else {
            8u64
        };
        let a174 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h172, _) => {
                h172
            }
        };
        nat_le(a173, a174)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a177 = f28(v0);
            let a178 = {
                let a175 = f23(v11.clone(), 1u64)?;
                let a176 = f29(v0);
                nat_mul(a175, a176)?
            };
            nat_add(a177, a178)?
        }, 4u64, f25(v11, 1u64), f25(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m179 = v1.clone();
        match m179 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m180 = v2.clone();
        match m180 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m181 = v3.clone();
        match m181 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h182, _, _, _, _, _, _, _, _, _, _) => {
            h182
        }
    };
    let m183 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m183 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m184 = v2;
        match m184 {
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
        let m185 = v3;
        match m185 {
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
        let a187 = match v7.clone() {
            Adt2::C0(h186, _, _, _, _, _, _, _, _, _, _) => {
                h186
            }
        };
        let a188 = 0u64;
        if if nat_eq(a187, a188) {
            f26(v7.clone(), f23(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f30(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a190 = match v9.clone() {
            Adt2::C0(h189, _, _, _, _, _, _, _, _, _, _) => {
                h189
            }
        };
        let a191 = 0u64;
        if if nat_eq(a190, a191) {
            f26(v9.clone(), f23(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f30(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a193 = match v11.clone() {
            Adt2::C0(h192, _, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        let a194 = 0u64;
        if if nat_eq(a193, a194) {
            f26(v11.clone(), f23(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f30(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a196 = match v7.clone() {
            Adt2::C0(_, h195, _, _, _, _, _, _, _, _, _) => {
                h195
            }
        };
        let a197 = 24u64;
        nat_eq(a196, a197)
    } else {
        false
    } {
        let a199 = match v7.clone() {
            Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                h198
            }
        };
        let a200 = 1u64;
        nat_eq(a199, a200)
    } else {
        false
    } {
        let a202 = match v7.clone() {
            Adt2::C0(_, _, _, _, h201, _, _, _, _, _, _) => {
                h201
            }
        };
        let a203 = 1u64;
        nat_eq(a202, a203)
    } else {
        false
    } {
        let a205 = match v11.clone() {
            Adt2::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                h204
            }
        };
        let a206 = 24u64;
        nat_eq(a205, a206)
    } else {
        false
    } {
        let a208 = match v11.clone() {
            Adt2::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
                h207
            }
        };
        let a209 = 1u64;
        nat_eq(a208, a209)
    } else {
        false
    } {
        let a211 = match v11.clone() {
            Adt2::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                h210
            }
        };
        let a212 = 1u64;
        nat_eq(a211, a212)
    } else {
        false
    } {
        let a214 = match v11.clone() {
            Adt2::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                h213
            }
        };
        let a215 = 1u64;
        nat_eq(a214, a215)
    } else {
        false
    } {
        let a217 = match v9.clone() {
            Adt2::C0(_, _, h216, _, _, _, _, _, _, _, _) => {
                h216
            }
        };
        let a218 = 1u64;
        nat_eq(a217, a218)
    } else {
        false
    } {
        let a220 = match v9.clone() {
            Adt2::C0(_, _, _, h219, _, _, _, _, _, _, _) => {
                h219
            }
        };
        let a221 = 1u64;
        nat_eq(a220, a221)
    } else {
        false
    } {
        let a223 = match v9.clone() {
            Adt2::C0(_, _, _, _, h222, _, _, _, _, _, _) => {
                h222
            }
        };
        let a224 = 1u64;
        nat_eq(a223, a224)
    } else {
        false
    } {
        let a226 = 3u64;
        let a227 = match v9.clone() {
            Adt2::C0(_, h225, _, _, _, _, _, _, _, _, _) => {
                h225
            }
        };
        nat_le(a226, a227)
    } else {
        false
    } {
        let a229 = match v5.clone() {
            Adt2::C0(_, h228, _, _, _, _, _, _, _, _, _) => {
                h228
            }
        };
        let a230 = 4u64;
        nat_eq(a229, a230)
    } else {
        false
    } {
        let a232 = match v5.clone() {
            Adt2::C0(_, _, h231, _, _, _, _, _, _, _, _) => {
                h231
            }
        };
        let a233 = 4u64;
        nat_eq(a232, a233)
    } else {
        false
    } {
        let a236 = match v5.clone() {
            Adt2::C0(_, _, _, h234, _, _, _, _, _, _, _) => {
                h234
            }
        };
        let a237 = match v7.clone() {
            Adt2::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
                h235
            }
        };
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a239 = match v5.clone() {
            Adt2::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                h238
            }
        };
        let a240 = 1u64;
        nat_eq(a239, a240)
    } else {
        false
    } {
        f31(v5.clone())?
    } else {
        false
    } {
        let a241 = 0u64;
        let a242 = v12;
        nat_lt(a241, a242)
    } else {
        false
    } {
        let a243 = v12;
        let a244 = 2147483648u64;
        nat_lt(a243, a244)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a246 = match v7.clone() {
                Adt2::C0(_, _, h245, _, _, _, _, _, _, _, _) => {
                    h245
                }
            };
            let a247 = f32(v12)?;
            nat_mul(a246, a247)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C38(f25(v9, 1u64), f25(v11, 1u64), match v0 {
            Adt1::C0(h248, _, _, _, _, _, _, _, _, _, _, _) => {
                h248
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m249 = v0.clone();
        match m249 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m250 = v1.clone();
        match m250 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m251 = v2.clone();
        match m251 {
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
        let m252 = v0;
        match m252 {
            None => {
                false
            }
            Some(_) => {
                let m253 = v1;
                match m253 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m254 = v2;
                        match m254 {
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
        let a256 = match v7.clone() {
            Adt2::C0(h255, _, _, _, _, _, _, _, _, _, _) => {
                h255
            }
        };
        let a257 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a256, a257) {
            let a259 = match v7.clone() {
                Adt2::C0(_, _, _, _, h258, _, _, _, _, _, _) => {
                    h258
                }
            };
            let a260 = 1u64;
            nat_eq(a259, a260)
        } else {
            false
        } {
            f26(v7.clone(), f23(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f30(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a262 = match v9.clone() {
                Adt2::C0(h261, _, _, _, _, _, _, _, _, _, _) => {
                    h261
                }
            };
            let a263 = 0u64;
            if if if nat_eq(a262, a263) {
                let a265 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                        h264
                    }
                };
                let a266 = 1u64;
                nat_eq(a265, a266)
            } else {
                false
            } {
                f26(v9.clone(), f23(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f30(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a268 = match v11.clone() {
                Adt2::C0(h267, _, _, _, _, _, _, _, _, _, _) => {
                    h267
                }
            };
            let a269 = 0u64;
            if if if nat_eq(a268, a269) {
                let a271 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h270, _, _, _, _, _, _) => {
                        h270
                    }
                };
                let a272 = 1u64;
                nat_eq(a271, a272)
            } else {
                false
            } {
                f26(v11.clone(), f23(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f30(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a274 = match v7.clone() {
                Adt2::C0(_, _, _, h273, _, _, _, _, _, _, _) => {
                    h273
                }
            };
            let a275 = 1u64;
            nat_eq(a274, a275)
        } else {
            false
        } {
            let a277 = match v11.clone() {
                Adt2::C0(_, _, _, h276, _, _, _, _, _, _, _) => {
                    h276
                }
            };
            let a278 = 1u64;
            nat_eq(a277, a278)
        } else {
            false
        } {
            let a281 = match v9.clone() {
                Adt2::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
                    h279
                }
            };
            let a282 = match v7.clone() {
                Adt2::C0(_, h280, _, _, _, _, _, _, _, _, _) => {
                    h280
                }
            };
            nat_eq(a281, a282)
        } else {
            false
        } {
            let a285 = match v9.clone() {
                Adt2::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                    h283
                }
            };
            let a286 = match v7.clone() {
                Adt2::C0(_, _, h284, _, _, _, _, _, _, _, _) => {
                    h284
                }
            };
            nat_eq(a285, a286)
        } else {
            false
        } {
            let a289 = match v11.clone() {
                Adt2::C0(_, h287, _, _, _, _, _, _, _, _, _) => {
                    h287
                }
            };
            let a290 = match v9.clone() {
                Adt2::C0(_, _, h288, _, _, _, _, _, _, _, _) => {
                    h288
                }
            };
            nat_eq(a289, a290)
        } else {
            false
        } {
            let a293 = match v11.clone() {
                Adt2::C0(_, _, h291, _, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a294 = match v7.clone() {
                Adt2::C0(_, _, h292, _, _, _, _, _, _, _, _) => {
                    h292
                }
            };
            nat_eq(a293, a294)
        } else {
            false
        } {
            let a296 = 0u64;
            let a297 = match v9.clone() {
                Adt2::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                    h295
                }
            };
            if nat_lt(a296, a297) {
                f21(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f31(v5.clone())?
        } else {
            false
        } {
            let m298 = v3.clone();
            match m298 {
                None => {
                    true
                }
                Some(v15) => {
                    let a300 = match v15.clone() {
                        Adt2::C0(h299, _, _, _, _, _, _, _, _, _, _) => {
                            h299
                        }
                    };
                    let a301 = 0u64;
                    if if if if if if nat_eq(a300, a301) {
                        let a303 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h302, _, _, _, _, _, _) => {
                                h302
                            }
                        };
                        let a304 = 1u64;
                        nat_eq(a303, a304)
                    } else {
                        false
                    } {
                        f26(v15.clone(), f23(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f30(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a307 = match v15.clone() {
                            Adt2::C0(_, h305, _, _, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a308 = match v9.clone() {
                            Adt2::C0(_, _, h306, _, _, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        nat_eq(a307, a308)
                    } else {
                        false
                    } {
                        let a311 = match v15.clone() {
                            Adt2::C0(_, _, h309, _, _, _, _, _, _, _, _) => {
                                h309
                            }
                        };
                        let a312 = match v9.clone() {
                            Adt2::C0(_, _, h310, _, _, _, _, _, _, _, _) => {
                                h310
                            }
                        };
                        nat_eq(a311, a312)
                    } else {
                        false
                    } {
                        let a315 = match v15 {
                            Adt2::C0(_, _, _, h313, _, _, _, _, _, _, _) => {
                                h313
                            }
                        };
                        let a316 = match v7.clone() {
                            Adt2::C0(_, _, h314, _, _, _, _, _, _, _, _) => {
                                h314
                            }
                        };
                        nat_eq(a315, a316)
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
            let m317 = v3;
            match m317 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f25(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a319 = f23(v5.clone(), 1u64)?;
            let a320 = f33(match v9.clone() {
                Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                    h318
                }
            }, v17.clone())?;
            nat_mul(a319, a320)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C37(f25(v9, 1u64), f25(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f23(v1.clone(), 1u64)?;
    let a322 = match v1.clone() {
        Adt2::C0(h321, _, _, _, _, _, _, _, _, _, _) => {
            h321
        }
    };
    let a323 = 0u64;
    if if if if if if if if nat_eq(a322, a323) {
        let a325 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h324, _, _, _, _, _) => {
                h324
            }
        };
        let a326 = 4u64;
        nat_eq(a325, a326)
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(h327, _, _, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        })
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(_, h328, _, _, _, _, _, _, _, _, _, _) => {
                h328
            }
        })
    } else {
        false
    } {
        let a330 = 0u64;
        let a331 = match v0.clone() {
            Adt1::C0(_, _, h329, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        };
        nat_lt(a330, a331)
    } else {
        false
    } {
        let a333 = match v0.clone() {
            Adt1::C0(_, _, h332, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        };
        let a334 = 2139095040u64;
        nat_lt(a333, a334)
    } else {
        false
    } {
        let a335 = v2;
        let a336 = 9223372036854775808u64;
        nat_lt(a335, a336)
    } else {
        false
    } {
        let a338 = f35(v2)?;
        let a339 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h337, _) => {
                h337
            }
        };
        nat_le(a338, a339)
    } else {
        false
    } {
        Ok(Adt10::C4(f36(v2)?, 4u64, f25(v1.clone(), 1u64), f25(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h340, _, _, _, _, _, _, _, _, _, _, _) => {
                h340
            }
        }, match v0.clone() {
            Adt1::C0(_, h341, _, _, _, _, _, _, _, _, _, _) => {
                h341
            }
        }, match v0 {
            Adt1::C0(_, _, h342, _, _, _, _, _, _, _, _, _) => {
                h342
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m343 = v3;
    match m343 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f37(v0, v2.clone());
            let v7: Adt3 = f37(v0, v5.clone());
            let v8: Adt3 = f37(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h344, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h345, _, _, _, _, _, _) => {
                    h345
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h346, _, _, _, _, _) => {
                    h346
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h347, _, _, _, _) => {
                    h347
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h348, _, _, _, _, _, _, _) => {
                    h348
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h349, _, _, _, _, _, _) => {
                    h349
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h350, _, _, _, _, _) => {
                    h350
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h351, _, _, _, _, _, _, _, _, _, _, _) => {
                    h351
                }
            };
            let v17: u64 = {
                let a356 = v13;
                let a357 = {
                    let a354 = {
                        let a352 = v9;
                        let a353 = 1u64;
                        nat_sub(a352, a353)
                    };
                    let a355 = v16;
                    nat_mul(a354, a355)?
                };
                nat_add(a356, a357)?
            };
            let v18: u64 = {
                let a362 = v14;
                let a363 = {
                    let a360 = {
                        let a358 = v10;
                        let a359 = 1u64;
                        nat_sub(a358, a359)
                    };
                    let a361 = v16;
                    nat_mul(a360, a361)?
                };
                nat_add(a362, a363)?
            };
            let v19: u64 = {
                let a366 = {
                    let a364 = v9;
                    let a365 = v10;
                    nat_mul(a364, a365)?
                };
                let a367 = v12;
                nat_mul(a366, a367)?
            };
            let v20: u64 = {
                let a370 = {
                    let a368 = v13;
                    let a369 = v14;
                    nat_mul(a368, a369)?
                };
                let a371 = v15;
                nat_mul(a370, a371)?
            };
            let v21: u64 = {
                let a372 = v19;
                let a373 = 32u64;
                if nat_lt(a372, a373) {
                    v19
                } else {
                    32u64
                }
            };
            let a375 = match v2.clone() {
                Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a376 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a375, a376) {
                true
            } else {
                let a378 = match v2.clone() {
                    Adt2::C0(h377, _, _, _, _, _, _, _, _, _, _) => {
                        h377
                    }
                };
                let a379 = 1u64;
                nat_eq(a378, a379)
            } {
                let a381 = match v5.clone() {
                    Adt2::C0(h380, _, _, _, _, _, _, _, _, _, _) => {
                        h380
                    }
                };
                let a382 = 0u64;
                nat_eq(a381, a382)
            } else {
                false
            } {
                let a384 = match v4.clone() {
                    Adt2::C0(h383, _, _, _, _, _, _, _, _, _, _) => {
                        h383
                    }
                };
                let a385 = 0u64;
                nat_eq(a384, a385)
            } else {
                false
            } {
                let a386 = v16;
                let a387 = 2147483648u64;
                nat_lt(a386, a387)
            } else {
                false
            } {
                let a388 = 0u64;
                let a389 = v9;
                nat_lt(a388, a389)
            } else {
                false
            } {
                let a390 = 0u64;
                let a391 = v10;
                nat_lt(a390, a391)
            } else {
                false
            } {
                let a392 = 0u64;
                let a393 = v11;
                nat_lt(a392, a393)
            } else {
                false
            } {
                let a394 = 0u64;
                let a395 = v12;
                nat_lt(a394, a395)
            } else {
                false
            } {
                let a396 = 0u64;
                let a397 = v13;
                nat_lt(a396, a397)
            } else {
                false
            } {
                let a398 = 0u64;
                let a399 = v14;
                nat_lt(a398, a399)
            } else {
                false
            } {
                let a400 = 0u64;
                let a401 = v15;
                nat_lt(a400, a401)
            } else {
                false
            } {
                let a402 = 0u64;
                let a403 = v16;
                nat_lt(a402, a403)
            } else {
                false
            } {
                let a405 = match v6.clone() {
                    Adt3::C0(_, _, _, h404, _, _, _, _) => {
                        h404
                    }
                };
                let a406 = v11;
                nat_eq(a405, a406)
            } else {
                false
            } {
                let a408 = match v8.clone() {
                    Adt3::C0(h407, _, _, _, _, _, _, _) => {
                        h407
                    }
                };
                let a409 = v17;
                nat_eq(a408, a409)
            } else {
                false
            } {
                let a411 = match v8.clone() {
                    Adt3::C0(_, h410, _, _, _, _, _, _) => {
                        h410
                    }
                };
                let a412 = v18;
                nat_eq(a411, a412)
            } else {
                false
            } {
                let a414 = match v8.clone() {
                    Adt3::C0(_, _, h413, _, _, _, _, _) => {
                        h413
                    }
                };
                let a415 = v15;
                nat_eq(a414, a415)
            } else {
                false
            } {
                let a417 = match v8.clone() {
                    Adt3::C0(_, _, _, h416, _, _, _, _) => {
                        h416
                    }
                };
                let a418 = v12;
                nat_eq(a417, a418)
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a423 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h419, _, _, _, _, _) => {
                        h419
                    }
                };
                let a424 = {
                    let a421 = match v2.clone() {
                        Adt2::C0(h420, _, _, _, _, _, _, _, _, _, _) => {
                            h420
                        }
                    };
                    let a422 = 1u64;
                    if nat_eq(a421, a422) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a423, a424)
            } else {
                false
            } {
                let a426 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h425, _, _, _, _, _) => {
                        h425
                    }
                };
                let a427 = 4u64;
                nat_eq(a426, a427)
            } else {
                false
            } {
                f30(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f30(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a432 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h428, _, _, _, _) => {
                            h428
                        }
                    };
                    let a433 = {
                        let a430 = v13;
                        let a431 = match v2 {
                            Adt2::C0(_, _, _, _, _, h429, _, _, _, _, _) => {
                                h429
                            }
                        };
                        nat_mul(a430, a431)?
                    };
                    nat_eq(a432, a433)
                } else {
                    let a435 = match v1.clone() {
                        Adt1::C0(_, h434, _, _, _, _, _, _, _, _, _, _) => {
                            h434
                        }
                    };
                    let a436 = 0u64;
                    if if if if if if nat_eq(a435, a436) {
                        let a438 = match v1 {
                            Adt1::C0(_, _, h437, _, _, _, _, _, _, _, _, _) => {
                                h437
                            }
                        };
                        let a439 = 1u64;
                        nat_eq(a438, a439)
                    } else {
                        false
                    } {
                        let a441 = match v2 {
                            Adt2::C0(_, _, _, _, h440, _, _, _, _, _, _) => {
                                h440
                            }
                        };
                        let a442 = 1u64;
                        nat_eq(a441, a442)
                    } else {
                        false
                    } {
                        let a444 = match v5.clone() {
                            Adt2::C0(_, _, _, h443, _, _, _, _, _, _, _) => {
                                h443
                            }
                        };
                        let a445 = 1u64;
                        nat_eq(a444, a445)
                    } else {
                        false
                    } {
                        let a447 = match v5 {
                            Adt2::C0(_, _, _, _, h446, _, _, _, _, _, _) => {
                                h446
                            }
                        };
                        let a448 = 1u64;
                        nat_eq(a447, a448)
                    } else {
                        false
                    } {
                        let a450 = match v4.clone() {
                            Adt2::C0(_, _, _, h449, _, _, _, _, _, _, _) => {
                                h449
                            }
                        };
                        let a451 = 1u64;
                        nat_eq(a450, a451)
                    } else {
                        false
                    } {
                        let a453 = match v4 {
                            Adt2::C0(_, _, _, _, h452, _, _, _, _, _, _) => {
                                h452
                            }
                        };
                        let a454 = 1u64;
                        nat_eq(a453, a454)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a455 = v11;
                    let a456 = v20;
                    nat_mul(a455, a456)?
                }, v21, {
                    let a457 = v21;
                    let a458 = v11;
                    nat_mul(a457, a458)?
                }, {
                    let a459 = v21;
                    let a460 = v20;
                    nat_mul(a459, a460)?
                }, {
                    let a465 = {
                        let a461 = v17;
                        let a462 = v18;
                        nat_mul(a461, a462)?
                    };
                    let a466 = {
                        let a463 = v15;
                        let a464 = v12;
                        nat_mul(a463, a464)?
                    };
                    nat_mul(a465, a466)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f8(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m467 = v3;
    match m467 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h468, _, _, _, _, _, _, _, _, _) => {
                    h468
                }
            }, match v1.clone() {
                Adt1::C0(h469, _, _, _, _, _, _, _, _, _, _, _) => {
                    h469
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h470, _, _, _, _, _, _, _, _) => {
                        h470
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h471, _, _, _, _, _, _, _, _, _) => {
                        h471
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h472, _, _, _, _, _) => {
                        h472
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h473, _, _, _, _, _, _, _) => {
                        h473
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h474, _, _, _, _, _, _, _, _) => {
                    h474
                }
            }, match v1.clone() {
                Adt1::C0(_, h475, _, _, _, _, _, _, _, _, _, _) => {
                    h475
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h476, _, _, _, _, _, _, _) => {
                        h476
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h477, _, _, _, _, _, _, _, _) => {
                        h477
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h478, _, _, _, _) => {
                        h478
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h479, _, _, _, _, _, _) => {
                        h479
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h480, _, _, _, _, _, _, _) => {
                        h480
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h481, _, _, _, _, _, _, _, _, _) => {
                        h481
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h482, _, _, _, _, _, _) => {
                        h482
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h483, _, _, _) => {
                        h483
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f39(v6.clone(), match v5.clone() {
                Adt2::C0(_, h484, _, _, _, _, _, _, _, _, _) => {
                    h484
                }
            })?;
            let v10: u64 = f39(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h485, _, _, _, _, _, _, _, _) => {
                    h485
                }
            })?;
            let v11: u64 = if v0 {
                f39(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h486, _, _, _, _, _, _, _) => {
                        h486
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h487, _, _) => {
                        h487
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                        h488
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h489, _) => {
                        h489
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h490, _, _, _, _, _, _) => {
                        h490
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h491) => {
                        h491
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h492, _, _, _, _, _, _) => {
                        h492
                    }
                }
            };
            let v15: u64 = {
                let a497 = {
                    let a493 = v9;
                    let a494 = v10;
                    nat_mul(a493, a494)?
                };
                let a498 = {
                    let a495 = v11;
                    let a496 = v13;
                    nat_mul(a495, a496)?
                };
                nat_mul(a497, a498)?
            };
            let v16: u64 = {
                let a506 = {
                    let a501 = match v6.clone() {
                        Adt6::C0(h499, _, _, _) => {
                            h499
                        }
                    };
                    let a502 = match v7.clone() {
                        Adt6::C0(h500, _, _, _) => {
                            h500
                        }
                    };
                    nat_mul(a501, a502)?
                };
                let a507 = {
                    let a504 = match v8.clone() {
                        Adt6::C0(h503, _, _, _) => {
                            h503
                        }
                    };
                    let a505 = v12;
                    nat_mul(a504, a505)?
                };
                nat_mul(a506, a507)?
            };
            let v17: u64 = {
                let a508 = v15;
                let a509 = 32u64;
                if nat_lt(a508, a509) {
                    v15
                } else {
                    32u64
                }
            };
            let a511 = match v2.clone() {
                Adt2::C0(h510, _, _, _, _, _, _, _, _, _, _) => {
                    h510
                }
            };
            let a512 = 0u64;
            if if if if if if if if if if if nat_eq(a511, a512) {
                true
            } else {
                let a514 = match v2.clone() {
                    Adt2::C0(h513, _, _, _, _, _, _, _, _, _, _) => {
                        h513
                    }
                };
                let a515 = 1u64;
                nat_eq(a514, a515)
            } {
                let a517 = match v5.clone() {
                    Adt2::C0(h516, _, _, _, _, _, _, _, _, _, _) => {
                        h516
                    }
                };
                let a518 = 0u64;
                nat_eq(a517, a518)
            } else {
                false
            } {
                let a520 = match v4.clone() {
                    Adt2::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
                        h519
                    }
                };
                let a521 = 0u64;
                nat_eq(a520, a521)
            } else {
                false
            } {
                let a523 = match v4.clone() {
                    Adt2::C0(_, h522, _, _, _, _, _, _, _, _, _) => {
                        h522
                    }
                };
                let a524 = v9;
                nat_eq(a523, a524)
            } else {
                false
            } {
                let a526 = match v4.clone() {
                    Adt2::C0(_, _, h525, _, _, _, _, _, _, _, _) => {
                        h525
                    }
                };
                let a527 = v10;
                nat_eq(a526, a527)
            } else {
                false
            } {
                if v0 {
                    let a528 = 0u64;
                    let a529 = v12;
                    if if if if if if if nat_lt(a528, a529) {
                        let a530 = v12;
                        let a531 = 2147483648u64;
                        nat_lt(a530, a531)
                    } else {
                        false
                    } {
                        let a532 = v14;
                        let a533 = 2147483648u64;
                        nat_lt(a532, a533)
                    } else {
                        false
                    } {
                        let a534 = v13;
                        let a535 = 2147483648u64;
                        nat_lt(a534, a535)
                    } else {
                        false
                    } {
                        let a539 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h536, _, _, _, _, _, _) => {
                                h536
                            }
                        };
                        let a540 = {
                            let a537 = v12;
                            let a538 = v14;
                            nat_mul(a537, a538)?
                        };
                        nat_eq(a539, a540)
                    } else {
                        false
                    } {
                        let a544 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h541, _, _, _, _, _, _) => {
                                h541
                            }
                        };
                        let a545 = {
                            let a542 = v12;
                            let a543 = v13;
                            nat_mul(a542, a543)?
                        };
                        nat_eq(a544, a545)
                    } else {
                        false
                    } {
                        let a547 = match v4.clone() {
                            Adt2::C0(_, _, _, h546, _, _, _, _, _, _, _) => {
                                h546
                            }
                        };
                        let a548 = v11;
                        nat_eq(a547, a548)
                    } else {
                        false
                    } {
                        let a552 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h549, _, _, _, _, _, _) => {
                                h549
                            }
                        };
                        let a553 = {
                            let a550 = v14;
                            let a551 = v13;
                            nat_mul(a550, a551)?
                        };
                        nat_eq(a552, a553)
                    } else {
                        false
                    }
                } else {
                    let a555 = match v2.clone() {
                        Adt2::C0(_, _, _, h554, _, _, _, _, _, _, _) => {
                            h554
                        }
                    };
                    let a556 = v12;
                    if if if nat_eq(a555, a556) {
                        let a558 = match v4.clone() {
                            Adt2::C0(_, _, _, h557, _, _, _, _, _, _, _) => {
                                h557
                            }
                        };
                        let a559 = v14;
                        nat_eq(a558, a559)
                    } else {
                        false
                    } {
                        let a561 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h560, _, _, _, _, _, _) => {
                                h560
                            }
                        };
                        let a562 = v13;
                        nat_eq(a561, a562)
                    } else {
                        false
                    } {
                        let a563 = 0u64;
                        let a564 = v16;
                        nat_lt(a563, a564)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a565 = 0u64;
                let a566 = v14;
                nat_lt(a565, a566)
            } else {
                false
            } {
                let a567 = 0u64;
                let a568 = v15;
                nat_lt(a567, a568)
            } else {
                false
            } {
                f30(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f30(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a569 = v16;
                    let a570 = v14;
                    nat_mul(a569, a570)?
                }, v17, {
                    let a571 = v17;
                    let a572 = v16;
                    nat_mul(a571, a572)?
                }, {
                    let a573 = v17;
                    let a574 = v14;
                    nat_mul(a573, a574)?
                }, 0u64, f25(v2, 1u64), f25(v5, 1u64), f25(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f9(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m575 = v2;
    match m575 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m576 = v3;
            match m576 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a578 = match v1.clone() {
                        Adt2::C0(h577, _, _, _, _, _, _, _, _, _, _) => {
                            h577
                        }
                    };
                    let a579 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a578, a579) {
                        let a581 = match v5.clone() {
                            Adt2::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                                h580
                            }
                        };
                        let a582 = 0u64;
                        nat_eq(a581, a582)
                    } else {
                        false
                    } {
                        let a584 = match v6.clone() {
                            Adt2::C0(h583, _, _, _, _, _, _, _, _, _, _) => {
                                h583
                            }
                        };
                        let a585 = 0u64;
                        nat_eq(a584, a585)
                    } else {
                        false
                    } {
                        let a587 = match v4.clone() {
                            Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                                h586
                            }
                        };
                        let a588 = 0u64;
                        nat_eq(a587, a588)
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
                        f38(v1.clone())?
                    } else {
                        false
                    } {
                        f38(v5.clone())?
                    } else {
                        false
                    } {
                        f38(v6)?
                    } else {
                        false
                    } {
                        f38(v4.clone())?
                    } else {
                        false
                    } {
                        let a594 = match v1.clone() {
                            Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                                h589
                            }
                        };
                        let a595 = {
                            let a592 = match v5.clone() {
                                Adt2::C0(_, h590, _, _, _, _, _, _, _, _, _) => {
                                    h590
                                }
                            };
                            let a593 = match v5.clone() {
                                Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
                                    h591
                                }
                            };
                            nat_mul(a592, a593)?
                        };
                        nat_eq(a594, a595)
                    } else {
                        false
                    } {
                        let a601 = match v1.clone() {
                            Adt2::C0(_, _, h596, _, _, _, _, _, _, _, _) => {
                                h596
                            }
                        };
                        let a602 = {
                            let a599 = match v5.clone() {
                                Adt2::C0(_, _, h597, _, _, _, _, _, _, _, _) => {
                                    h597
                                }
                            };
                            let a600 = match v5.clone() {
                                Adt2::C0(_, _, _, h598, _, _, _, _, _, _, _) => {
                                    h598
                                }
                            };
                            nat_mul(a599, a600)?
                        };
                        nat_eq(a601, a602)
                    } else {
                        false
                    } {
                        let a605 = match v1.clone() {
                            Adt2::C0(_, _, _, h603, _, _, _, _, _, _, _) => {
                                h603
                            }
                        };
                        let a606 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h604, _, _, _, _, _, _) => {
                                h604
                            }
                        };
                        nat_eq(a605, a606)
                    } else {
                        false
                    } {
                        let a608 = match v0.clone() {
                            Adt1::C0(h607, _, _, _, _, _, _, _, _, _, _, _) => {
                                h607
                            }
                        };
                        let a609 = 4294967296u64;
                        nat_lt(a608, a609)
                    } else {
                        false
                    } {
                        let a611 = match v0.clone() {
                            Adt1::C0(h610, _, _, _, _, _, _, _, _, _, _, _) => {
                                h610
                            }
                        };
                        let a612 = 0u64;
                        if nat_eq(a611, a612) {
                            if f30(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a615 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h613) => {
                                        h613
                                    }
                                };
                                let a616 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h614) => {
                                        h614
                                    }
                                };
                                nat_eq(a615, a616)
                            }
                        } else {
                            let a619 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h617) => {
                                    h617
                                }
                            };
                            let a620 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h618) => {
                                    h618
                                }
                            };
                            nat_eq(a619, a620)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a622 = match v4.clone() {
                                Adt2::C0(_, h621, _, _, _, _, _, _, _, _, _) => {
                                    h621
                                }
                            };
                            let a623 = 0u64;
                            if if if nat_eq(a622, a623) {
                                true
                            } else {
                                let a625 = match v4.clone() {
                                    Adt2::C0(_, _, h624, _, _, _, _, _, _, _, _) => {
                                        h624
                                    }
                                };
                                let a626 = 0u64;
                                nat_eq(a625, a626)
                            } {
                                true
                            } else {
                                let a628 = match v4.clone() {
                                    Adt2::C0(_, _, _, h627, _, _, _, _, _, _, _) => {
                                        h627
                                    }
                                };
                                let a629 = 0u64;
                                nat_eq(a628, a629)
                            } {
                                true
                            } else {
                                let a631 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h630, _, _, _, _, _, _) => {
                                        h630
                                    }
                                };
                                let a632 = 0u64;
                                nat_eq(a631, a632)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h633, _, _, _, _, _, _, _, _, _) => {
                                h633
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a635 = match v0 {
                                Adt1::C0(h634, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h634
                                }
                            };
                            let a636 = 0u64;
                            if nat_eq(a635, a636) {
                                f23(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f40(v8, match v5.clone() {
                                Adt2::C0(_, _, h637, _, _, _, _, _, _, _, _) => {
                                    h637
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h638, _, _, _, _, _, _, _) => {
                                    h638
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h639, _, _, _, _, _, _) => {
                                    h639
                                }
                            })?
                        };
                        Ok(Adt10::C4(f41(v9, v8, v10)?, 4u64, f25(v1, 1u64), f25(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f42(v2.clone());
    let v6: Adt14 = f42(v4.clone());
    let v8: Adt15 = {
        let m640 = v3;
        match m640 {
            None => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h641, _, _, _, _, _, _, _, _, _, _, _) => {
                        h641
                    }
                }, match v1 {
                    Adt1::C0(_, h642, _, _, _, _, _, _, _, _, _, _) => {
                        h642
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h643, _, _, _, _, _, _, _, _, _, _, _) => {
                        h643
                    }
                }, match v1 {
                    Adt1::C0(_, h644, _, _, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                }, true, v5, f42(v7), v6)?
            }
        }
    };
    let m645 = v8;
    match m645 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f25(v2, 1u64), f25(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f11(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
            h646
        }
    };
    let v4: u64 = f44(v3);
    let a648 = match v1.clone() {
        Adt2::C0(h647, _, _, _, _, _, _, _, _, _, _) => {
            h647
        }
    };
    let a649 = 0u64;
    if if if if if if if if nat_eq(a648, a649) {
        true
    } else {
        let a651 = match v1.clone() {
            Adt2::C0(h650, _, _, _, _, _, _, _, _, _, _) => {
                h650
            }
        };
        let a652 = 30u64;
        nat_eq(a651, a652)
    } {
        let a655 = match v1.clone() {
            Adt2::C0(h653, _, _, _, _, _, _, _, _, _, _) => {
                h653
            }
        };
        let a656 = match v2.clone() {
            Adt2::C0(h654, _, _, _, _, _, _, _, _, _, _) => {
                h654
            }
        };
        nat_eq(a655, a656)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f45(match v0.clone() {
            Adt1::C0(h657, _, _, _, _, _, _, _, _, _, _, _) => {
                h657
            }
        })
    } else {
        false
    } {
        f46(v1.clone(), v4)
    } else {
        false
    } {
        f46(v2.clone(), v4)
    } else {
        false
    } {
        if f30(v1.clone(), v2.clone())? {
            true
        } else {
            f47(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f48(match v1.clone() {
            Adt2::C0(_, h658, _, _, _, _, _, _, _, _, _) => {
                h658
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                h659
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h660, _, _, _, _, _, _, _) => {
                h660
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h661, _, _, _, _, _, _) => {
                h661
            }
        }) {
            0u64
        } else {
            let a667 = match v1.clone() {
                Adt2::C0(_, _, h662, _, _, _, _, _, _, _, _) => {
                    h662
                }
            };
            let a668 = {
                let a665 = match v1.clone() {
                    Adt2::C0(_, _, _, h663, _, _, _, _, _, _, _) => {
                        h663
                    }
                };
                let a666 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h664, _, _, _, _, _, _) => {
                        h664
                    }
                };
                nat_mul(a665, a666)?
            };
            nat_mul(a667, a668)?
        };
        let v6: u64 = {
            let a669 = v5;
            let a670 = 0u64;
            if if nat_eq(a669, a670) {
                true
            } else {
                f49(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a672 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h671, _) => {
                        h671
                    }
                };
                let a673 = v4;
                let a674 = 0u64;
                nat_quot(a672, a673, a674)
            }
        };
        Ok(Adt10::C4(f50(v3, match v1.clone() {
            Adt2::C0(_, h675, _, _, _, _, _, _, _, _, _) => {
                h675
            }
        }, v5, v6)?, v4, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h676, _, _, _, _, _, _, _, _, _, _, _) => {
                h676
            }
        }, v5, f51(match v1 {
            Adt2::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
                h677
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a679 = match v1.clone() {
        Adt2::C0(h678, _, _, _, _, _, _, _, _, _, _) => {
            h678
        }
    };
    let a680 = 0u64;
    if if if if if nat_eq(a679, a680) {
        let a682 = match v2.clone() {
            Adt2::C0(h681, _, _, _, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a683 = 0u64;
        nat_eq(a682, a683)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f52(match v0.clone() {
            Adt1::C0(h684, _, _, _, _, _, _, _, _, _, _, _) => {
                h684
            }
        })
    } else {
        false
    } {
        if f30(v1.clone(), v2.clone())? {
            true
        } else {
            f47(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f25(v1, 1u64);
        let v4: u64 = {
            let a690 = match v3.clone() {
                Adt3::C0(_, h685, _, _, _, _, _, _) => {
                    h685
                }
            };
            let a691 = {
                let a688 = match v3.clone() {
                    Adt3::C0(_, _, h686, _, _, _, _, _) => {
                        h686
                    }
                };
                let a689 = match v3.clone() {
                    Adt3::C0(_, _, _, h687, _, _, _, _) => {
                        h687
                    }
                };
                nat_mul(a688, a689)?
            };
            nat_mul(a690, a691)?
        };
        Ok(Adt10::C4(f53(match v3.clone() {
            Adt3::C0(h692, _, _, _, _, _, _, _) => {
                h692
            }
        }, v4)?, 4u64, v3, f25(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h693, _, _, _, _, _, _, _, _, _, _, _) => {
                h693
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f13(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m694 = v1;
    match m694 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m695 = v2;
            match m695 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a697 = match v0.clone() {
                        Adt2::C0(h696, _, _, _, _, _, _, _, _, _, _) => {
                            h696
                        }
                    };
                    let a698 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a697, a698) {
                        let a700 = match v4.clone() {
                            Adt2::C0(h699, _, _, _, _, _, _, _, _, _, _) => {
                                h699
                            }
                        };
                        let a701 = 0u64;
                        nat_eq(a700, a701)
                    } else {
                        false
                    } {
                        let a703 = match v5.clone() {
                            Adt2::C0(h702, _, _, _, _, _, _, _, _, _, _) => {
                                h702
                            }
                        };
                        let a704 = 26u64;
                        nat_eq(a703, a704)
                    } else {
                        false
                    } {
                        let a706 = match v3.clone() {
                            Adt2::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
                                h705
                            }
                        };
                        let a707 = 0u64;
                        nat_eq(a706, a707)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a710 = match v0.clone() {
                            Adt2::C0(_, h708, _, _, _, _, _, _, _, _, _) => {
                                h708
                            }
                        };
                        let a711 = match v4.clone() {
                            Adt2::C0(_, h709, _, _, _, _, _, _, _, _, _) => {
                                h709
                            }
                        };
                        nat_eq(a710, a711)
                    } else {
                        false
                    } {
                        let a714 = match v0.clone() {
                            Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        let a715 = match v5.clone() {
                            Adt2::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                                h713
                            }
                        };
                        nat_eq(a714, a715)
                    } else {
                        false
                    } {
                        let a718 = match v0.clone() {
                            Adt2::C0(_, _, _, h716, _, _, _, _, _, _, _) => {
                                h716
                            }
                        };
                        let a719 = match v5.clone() {
                            Adt2::C0(_, _, h717, _, _, _, _, _, _, _, _) => {
                                h717
                            }
                        };
                        nat_eq(a718, a719)
                    } else {
                        false
                    } {
                        let a721 = 0u64;
                        let a722 = match v5.clone() {
                            Adt2::C0(_, _, _, h720, _, _, _, _, _, _, _) => {
                                h720
                            }
                        };
                        nat_lt(a721, a722)
                    } else {
                        false
                    } {
                        let a724 = 0u64;
                        let a725 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                                h723
                            }
                        };
                        nat_lt(a724, a725)
                    } else {
                        false
                    } {
                        let a727 = 4u64;
                        let a728 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h726, _, _, _, _, _) => {
                                h726
                            }
                        };
                        nat_le(a727, a728)
                    } else {
                        false
                    } {
                        let a730 = 4u64;
                        let a731 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h729, _, _, _, _, _) => {
                                h729
                            }
                        };
                        nat_le(a730, a731)
                    } else {
                        false
                    } {
                        let a733 = 4u64;
                        let a734 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h732, _, _, _, _, _) => {
                                h732
                            }
                        };
                        nat_le(a733, a734)
                    } else {
                        false
                    } {
                        let a736 = 4u64;
                        let a737 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h735, _, _, _, _, _) => {
                                h735
                            }
                        };
                        nat_le(a736, a737)
                    } else {
                        false
                    } {
                        f30(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f30(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f30(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f54(match v0.clone() {
                            Adt2::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                                h738
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h739, _, _, _, _, _, _, _, _) => {
                                h739
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h740, _, _, _, _, _, _, _) => {
                                h740
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h741, _, _, _, _, _, _) => {
                                h741
                            }
                        })?, 4u64, f25(v0, 1u64), f25(v3, 1u64), Adt4::C22(f25(v4, 1u64), f25(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f14(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a742 = v0;
    let a743 = 28u64;
    if if if nat_eq(a742, a743) {
        true
    } else {
        let a744 = v0;
        let a745 = 29u64;
        nat_eq(a744, a745)
    } {
        let a747 = 0u64;
        let a748 = match v1.clone() {
            Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _, _) => {
                h746
            }
        };
        if nat_lt(a747, a748) {
            true
        } else {
            let a750 = 0u64;
            let a751 = match v1.clone() {
                Adt1::C0(_, _, _, h749, _, _, _, _, _, _, _, _) => {
                    h749
                }
            };
            nat_lt(a750, a751)
        }
    } else {
        false
    } {
        let m755 = f55(match v2.clone() {
            Adt2::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        }, match v1.clone() {
            Adt1::C0(h753, _, _, _, _, _, _, _, _, _, _, _) => {
                h753
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h754, _, _, _, _, _, _, _, _) => {
                h754
            }
        });
        match m755 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f56(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f56(v0, v1, v2, v3, v4)
    }
}

pub fn f15(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h756, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h756
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h757, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h757
        }
    };
    let v6: u64 = {
        let a758 = v3;
        let a759 = v5;
        let a760 = 0u64;
        nat_rem(a758, a759, a760)
    };
    let a761 = v1;
    let a762 = 0u64;
    if nat_eq(a761, a762) {
        let a764 = v3;
        let a765 = match v0.clone() {
            Adt8::C0(_, _, _, h763, _, _, _, _, _, _, _, _, _, _, _) => {
                h763
            }
        };
        if nat_lt(a764, a765) {
            Ok(Adt9::C8(Some(f57(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h766, _, _, _, _, _, _) => {
                    h766
                }
            }, {
                let a770 = v6;
                let a771 = {
                    let m769 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h768, _, _, _, _, _, _) => {
                            h768
                        }
                    };
                    match m769 {
                        Adt3::C0(h767, _, _, _, _, _, _, _) => {
                            h767
                        }
                    }
                };
                let a772 = 0u64;
                nat_rem(a770, a771, a772)
            }, {
                let a782 = {
                    let a776 = v6;
                    let a777 = {
                        let m775 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h774, _, _, _, _, _, _) => {
                                h774
                            }
                        };
                        match m775 {
                            Adt3::C0(h773, _, _, _, _, _, _, _) => {
                                h773
                            }
                        }
                    };
                    let a778 = 0u64;
                    nat_quot(a776, a777, a778)
                };
                let a783 = {
                    let m781 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h780, _, _, _, _, _, _) => {
                            h780
                        }
                    };
                    match m781 {
                        Adt3::C0(_, h779, _, _, _, _, _, _) => {
                            h779
                        }
                    }
                };
                let a784 = 0u64;
                nat_rem(a782, a783, a784)
            }, {
                let a794 = {
                    let a788 = v6;
                    let a789 = {
                        let m787 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h786, _, _, _, _, _, _) => {
                                h786
                            }
                        };
                        match m787 {
                            Adt3::C0(h785, _, _, _, _, _, _, _) => {
                                h785
                            }
                        }
                    };
                    let a790 = 0u64;
                    nat_quot(a788, a789, a790)
                };
                let a795 = {
                    let m793 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h792, _, _, _, _, _, _) => {
                            h792
                        }
                    };
                    match m793 {
                        Adt3::C0(_, h791, _, _, _, _, _, _) => {
                            h791
                        }
                    }
                };
                let a796 = 0u64;
                nat_quot(a794, a795, a796)
            }, {
                let a797 = v3;
                let a798 = v5;
                let a799 = 0u64;
                nat_quot(a797, a798, a799)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a805 = v2;
            let a806 = {
                let a802 = v3;
                let a803 = {
                    let a800 = v1;
                    let a801 = 1u64;
                    if nat_eq(a800, a801) {
                        v4
                    } else {
                        v5
                    }
                };
                let a804 = 0u64;
                nat_quot(a802, a803, a804)
            };
            nat_add(a805, a806)?
        };
        let a808 = v7;
        let a809 = match v0.clone() {
            Adt8::C0(h807, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        if nat_lt(a808, a809) {
            let v8: u64 = {
                let a813 = v7;
                let a814 = {
                    let m812 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h811, _, _, _, _, _) => {
                            h811
                        }
                    };
                    match m812 {
                        Adt3::C0(h810, _, _, _, _, _, _, _) => {
                            h810
                        }
                    }
                };
                let a815 = 0u64;
                nat_rem(a813, a814, a815)
            };
            let v9: u64 = {
                let a819 = v7;
                let a820 = {
                    let m818 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h817, _, _, _, _, _) => {
                            h817
                        }
                    };
                    match m818 {
                        Adt3::C0(h816, _, _, _, _, _, _, _) => {
                            h816
                        }
                    }
                };
                let a821 = 0u64;
                nat_quot(a819, a820, a821)
            };
            let v10: u64 = {
                let a825 = v9;
                let a826 = {
                    let m824 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h823, _, _, _, _, _) => {
                            h823
                        }
                    };
                    match m824 {
                        Adt3::C0(_, h822, _, _, _, _, _, _) => {
                            h822
                        }
                    }
                };
                let a827 = 0u64;
                nat_rem(a825, a826, a827)
            };
            let v11: u64 = {
                let a831 = v9;
                let a832 = {
                    let m830 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h829, _, _, _, _, _) => {
                            h829
                        }
                    };
                    match m830 {
                        Adt3::C0(_, h828, _, _, _, _, _, _) => {
                            h828
                        }
                    }
                };
                let a833 = 0u64;
                nat_quot(a831, a832, a833)
            };
            let a834 = v1;
            let a835 = 1u64;
            if nat_eq(a834, a835) {
                Ok(Adt9::C8(Some(f57(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h836, _, _, _, _, _) => {
                        h836
                    }
                }, v8, v10, {
                    let a837 = v3;
                    let a838 = v4;
                    let a839 = 0u64;
                    nat_rem(a837, a838, a839)
                }, v11)?)))
            } else {
                let a840 = v1;
                let a841 = 2u64;
                if nat_eq(a840, a841) {
                    Ok(Adt9::C8(Some({
                        let a887 = f57(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h842, _, _, _, _) => {
                                h842
                            }
                        }, {
                            let a854 = {
                                let a846 = v8;
                                let a847 = {
                                    let m845 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h844, _, _, _) => {
                                            h844
                                        }
                                    };
                                    match m845 {
                                        Adt6::C0(_, h843, _, _) => {
                                            h843
                                        }
                                    }
                                };
                                nat_mul(a846, a847)?
                            };
                            let a855 = {
                                let a851 = v6;
                                let a852 = {
                                    let m850 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h849, _, _, _, _, _, _) => {
                                            h849
                                        }
                                    };
                                    match m850 {
                                        Adt3::C0(h848, _, _, _, _, _, _, _) => {
                                            h848
                                        }
                                    }
                                };
                                let a853 = 0u64;
                                nat_rem(a851, a852, a853)
                            };
                            nat_add(a854, a855)?
                        }, {
                            let a873 = {
                                let a859 = v10;
                                let a860 = {
                                    let m858 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h857, _, _, _) => {
                                            h857
                                        }
                                    };
                                    match m858 {
                                        Adt6::C0(_, h856, _, _) => {
                                            h856
                                        }
                                    }
                                };
                                nat_mul(a859, a860)?
                            };
                            let a874 = {
                                let a870 = {
                                    let a864 = v6;
                                    let a865 = {
                                        let m863 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h862, _, _, _, _, _, _) => {
                                                h862
                                            }
                                        };
                                        match m863 {
                                            Adt3::C0(h861, _, _, _, _, _, _, _) => {
                                                h861
                                            }
                                        }
                                    };
                                    let a866 = 0u64;
                                    nat_quot(a864, a865, a866)
                                };
                                let a871 = {
                                    let m869 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h868, _, _, _, _, _, _) => {
                                            h868
                                        }
                                    };
                                    match m869 {
                                        Adt3::C0(_, h867, _, _, _, _, _, _) => {
                                            h867
                                        }
                                    }
                                };
                                let a872 = 0u64;
                                nat_rem(a870, a871, a872)
                            };
                            nat_add(a873, a874)?
                        }, {
                            let a884 = {
                                let a878 = v6;
                                let a879 = {
                                    let m877 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h876, _, _, _, _, _, _) => {
                                            h876
                                        }
                                    };
                                    match m877 {
                                        Adt3::C0(h875, _, _, _, _, _, _, _) => {
                                            h875
                                        }
                                    }
                                };
                                let a880 = 0u64;
                                nat_quot(a878, a879, a880)
                            };
                            let a885 = {
                                let m883 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h882, _, _, _, _, _, _) => {
                                        h882
                                    }
                                };
                                match m883 {
                                    Adt3::C0(_, h881, _, _, _, _, _, _) => {
                                        h881
                                    }
                                }
                            };
                            let a886 = 0u64;
                            nat_quot(a884, a885, a886)
                        }, v11)?;
                        let a888 = 4u64;
                        let a889 = 0u64;
                        nat_quot(a887, a888, a889)
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

pub fn f16(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a890 = v1;
    let a891 = 0u64;
    if nat_eq(a890, a891) {
        let a893 = v3;
        let a894 = match v0.clone() {
            Adt8::C0(_, _, _, h892, _, _, _, _, _, _, _, _, _, _, _) => {
                h892
            }
        };
        if nat_lt(a893, a894) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h895, _, _, _, _, _, _) => {
                    h895
                }
            };
            let v5: u64 = {
                let a897 = v3;
                let a898 = match v0.clone() {
                    Adt8::C0(_, _, h896, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h896
                    }
                };
                let a899 = 0u64;
                nat_quot(a897, a898, a899)
            };
            Ok(Adt9::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), f59(v0.clone(), v5, {
                let a901 = v3;
                let a902 = match v0 {
                    Adt8::C0(_, _, h900, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h900
                    }
                };
                let a903 = 0u64;
                nat_rem(a901, a902, a903)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h904, _, _, _, _) => {
                h904
            }
        };
        let v7: u64 = {
            let a905 = v1;
            let a906 = 1u64;
            if nat_eq(a905, a906) {
                match v0.clone() {
                    Adt8::C0(_, h907, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h907
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h908, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h908
                    }
                }
            }
        };
        let v8: u64 = {
            let a912 = v2;
            let a913 = {
                let a909 = v3;
                let a910 = v7;
                let a911 = 0u64;
                nat_quot(a909, a910, a911)
            };
            nat_add(a912, a913)?
        };
        let v9: u64 = {
            let a914 = v3;
            let a915 = v7;
            let a916 = 0u64;
            nat_rem(a914, a915, a916)
        };
        let v10: u64 = {
            let a918 = v8;
            let a919 = match v6.clone() {
                Adt3::C0(h917, _, _, _, _, _, _, _) => {
                    h917
                }
            };
            let a920 = 0u64;
            nat_rem(a918, a919, a920)
        };
        let v11: u64 = {
            let a922 = v8;
            let a923 = match v6.clone() {
                Adt3::C0(h921, _, _, _, _, _, _, _) => {
                    h921
                }
            };
            let a924 = 0u64;
            nat_quot(a922, a923, a924)
        };
        let v12: u64 = {
            let a926 = v11;
            let a927 = match v6.clone() {
                Adt3::C0(_, h925, _, _, _, _, _, _) => {
                    h925
                }
            };
            let a928 = 0u64;
            nat_rem(a926, a927, a928)
        };
        let v13: u64 = {
            let a930 = v11;
            let a931 = match v6.clone() {
                Adt3::C0(_, h929, _, _, _, _, _, _) => {
                    h929
                }
            };
            let a932 = 0u64;
            nat_quot(a930, a931, a932)
        };
        let a934 = v8;
        let a935 = match v0.clone() {
            Adt8::C0(h933, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h933
            }
        };
        if nat_lt(a934, a935) {
            let a936 = v1;
            let a937 = 1u64;
            if nat_eq(a936, a937) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h938, _, _, _, _, _, _) => {
                        h938
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h939, _, _, _, _, _) => {
                        h939
                    }
                };
                let m942 = f60(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h940, _, _, _) => {
                        h940
                    }
                }, match v15.clone() {
                    Adt3::C0(h941, _, _, _, _, _, _, _) => {
                        h941
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m942 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m945 = f60(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h943, _, _) => {
                                h943
                            }
                        }, match v15 {
                            Adt3::C0(_, h944, _, _, _, _, _, _) => {
                                h944
                            }
                        }, v12, f58(v9, v14, 1u64))?;
                        match m945 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f61(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a946 = v1;
                let a947 = 2u64;
                if nat_eq(a946, a947) {
                    Ok(Adt9::C8(Some({
                        let a949 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h948) => {
                                h948
                            }
                        };
                        let a950 = 0u64;
                        if nat_eq(a949, a950) {
                            f57(v6, v10, v12, v9, v13)?
                        } else {
                            f57(v6.clone(), v10, v12, {
                                let a952 = v13;
                                let a953 = match v6.clone() {
                                    Adt3::C0(_, _, h951, _, _, _, _, _) => {
                                        h951
                                    }
                                };
                                let a954 = 0u64;
                                nat_rem(a952, a953, a954)
                            }, {
                                let a962 = {
                                    let a960 = {
                                        let a956 = v13;
                                        let a957 = match v6 {
                                            Adt3::C0(_, _, h955, _, _, _, _, _) => {
                                                h955
                                            }
                                        };
                                        let a958 = 0u64;
                                        nat_quot(a956, a957, a958)
                                    };
                                    let a961 = match v0 {
                                        Adt8::C0(_, _, h959, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h959
                                        }
                                    };
                                    nat_mul(a960, a961)?
                                };
                                let a963 = v9;
                                nat_add(a962, a963)?
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

pub fn f17(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f18(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f18(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f62(v3.clone(), {
        let m964 = v3.clone();
        match m964 {
            Adt4::C0 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f76(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C19(v74, v75) => {
                f77(v74, v0, v1, v2, v75, v4)?
            }
            Adt4::C20(v76, v77) => {
                f78(v76, v0, v1, v2, v77, v4)?
            }
            Adt4::C21 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C22(v78, v79) => {
                f79(v0, v1, v2, v78, v79, v4)?
            }
            Adt4::C23(v93, v94, v95, v96) => {
                f86(v0, v96, v93, v94, v95, v4)?
            }
            Adt4::C24(v92) => {
                if v5 {
                    f84(Adt16::C0(v92), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C26(v88, v89, v90, v91) => {
                let a966 = v88;
                let a967 = 64u64;
                if if if nat_eq(a966, a967) {
                    true
                } else {
                    let a968 = v88;
                    let a969 = 65u64;
                    nat_eq(a968, a969)
                } {
                    true
                } else {
                    let a970 = v88;
                    let a971 = 66u64;
                    nat_eq(a970, a971)
                } {
                    if v5 {
                        f84(Adt16::C1(v88, v89, v90, v91), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a972 = v88;
                    let a973 = 0u64;
                    if if nat_eq(a972, a973) {
                        true
                    } else {
                        let a974 = v88;
                        let a975 = 30u64;
                        nat_eq(a974, a975)
                    } {
                        f85(v0, v1, v2, v88, v89, v90, v91, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v87) => {
                f83(v0, v1, v2, v87)?
            }
            Adt4::C28(v83) => {
                f80(v0, v1, v2, v83, v4)?
            }
            Adt4::C29(v84) => {
                f81(v0, v1, v2, v84)?
            }
            Adt4::C30(v85, _) => {
                f82(v0, v1, v2, v85)?
            }
            Adt4::C31(v33, v34) => {
                f64(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f65(v0, match v1 {
                    Adt3::C0(h965, _, _, _, _, _, _, _) => {
                        h965
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f66(v0, v1, v36, v2)?
            }
            Adt4::C34(v54, v55, v56, v57) => {
                f75(v0, v1, v54, v2, v55, v56, v57)?
            }
            Adt4::C35(v53) => {
                f74(v0, v53, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f67(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f68(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f69(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f70(v0, v1, v47)?
            }
            Adt4::C40(v48, v49, v50) => {
                f71(v0, v1, v2, v48, v49, v50, v4)?
            }
            Adt4::C41 => {
                f73(v0, v1, v2, v4)?
            }
            Adt4::C42(v51, v52) => {
                f72(v0, v1, v2, v51, v52)?
            }
        }
    }))
}

pub fn f19(v0: Adt4) -> R<Adt9> {
    let m976 = v0;
    match m976 {
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
        Adt4::C25(v71, _, v73) => {
            Ok(Adt9::C6({
                let a982 = {
                    let a978 = 0u64;
                    let a979 = v71;
                    if nat_lt(a978, a979) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a983 = {
                    let a980 = 0u64;
                    let a981 = v73;
                    if nat_lt(a980, a981) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a982, a983)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v79, _, _, v82) => {
            Ok(Adt9::C6({
                let a984 = v79;
                let a985 = 66u64;
                if nat_eq(a984, a985) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v82))
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
        Adt4::C35(v44) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v44))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m977 = v32;
                match m977 {
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
        Adt4::C40(v39, _, _) => {
            Ok(Adt9::C6(if v39 {
                7u64
            } else {
                3u64
            }, 0u64, 0u64, false, 7u64))
        }
        Adt4::C41 => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C42(_, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a986 = v1;
    let a987 = 5u64;
    if if if if if nat_lt(a986, a987) {
        true
    } else {
        f87(v0, v1, v2, v3, v4)
    } {
        let a988 = v5;
        let a989 = 5u64;
        nat_lt(a988, a989)
    } else {
        false
    } {
        let a990 = v7;
        let a991 = 5u64;
        nat_lt(a990, a991)
    } else {
        false
    } {
        let a992 = v9;
        let a993 = 2u64;
        if nat_lt(a992, a993) {
            true
        } else {
            let a994 = v9;
            let a995 = 3u64;
            nat_eq(a994, a995)
        }
    } else {
        false
    } {
        Adt9::C7(f88(v0, v1, v2, v3, v4), f89(v5, v6), f89(v7, v8), f90({
            let a996 = v9;
            let a997 = 0u64;
            if nat_eq(a996, a997) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f21(v0: Adt2, v1: Adt2) -> bool {
    let a1000 = match v0.clone() {
        Adt2::C0(_, h998, _, _, _, _, _, _, _, _, _) => {
            h998
        }
    };
    let a1001 = match v1.clone() {
        Adt2::C0(_, h999, _, _, _, _, _, _, _, _, _) => {
            h999
        }
    };
    if if if nat_eq(a1000, a1001) {
        let a1004 = match v0.clone() {
            Adt2::C0(_, _, h1002, _, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = match v1.clone() {
            Adt2::C0(_, _, h1003, _, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        nat_eq(a1004, a1005)
    } else {
        false
    } {
        let a1008 = match v0.clone() {
            Adt2::C0(_, _, _, h1006, _, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = match v1.clone() {
            Adt2::C0(_, _, _, h1007, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        nat_eq(a1008, a1009)
    } else {
        false
    } {
        let a1012 = match v0 {
            Adt2::C0(_, _, _, _, h1010, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = match v1 {
            Adt2::C0(_, _, _, _, h1011, _, _, _, _, _, _) => {
                h1011
            }
        };
        nat_eq(a1012, a1013)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1015 = match v0.clone() {
        Adt2::C0(h1014, _, _, _, _, _, _, _, _, _, _) => {
            h1014
        }
    };
    let a1016 = 0u64;
    if if if nat_eq(a1015, a1016) {
        let a1018 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1017, _, _, _, _, _) => {
                h1017
            }
        };
        let a1019 = 4u64;
        nat_eq(a1018, a1019)
    } else {
        false
    } {
        if v2 {
            f38(v0.clone())?
        } else {
            true
        }
    } else {
        false
    } {
        let a1031 = {
            let a1028 = {
                let a1023 = {
                    let a1020 = v1;
                    let a1021 = 1u64;
                    nat_sub(a1020, a1021)
                };
                let a1024 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1022, _, _, _, _) => {
                        h1022
                    }
                };
                nat_mul(a1023, a1024)?
            };
            let a1029 = {
                let a1026 = 4u64;
                let a1027 = match v0.clone() {
                    Adt2::C0(_, h1025, _, _, _, _, _, _, _, _, _) => {
                        h1025
                    }
                };
                nat_mul(a1026, a1027)?
            };
            nat_add(a1028, a1029)?
        };
        let a1032 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1030, _) => {
                h1030
            }
        };
        Ok(nat_le(a1031, a1032))
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a1044 = {
        let a1038 = {
            let a1034 = match v0.clone() {
                Adt2::C0(_, h1033, _, _, _, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1035 = v1;
            let a1036 = 0u64;
            nat_quot(a1034, a1035, a1036)
        };
        let a1039 = match v0.clone() {
            Adt2::C0(_, _, h1037, _, _, _, _, _, _, _, _) => {
                h1037
            }
        };
        nat_mul(a1038, a1039)?
    };
    let a1045 = {
        let a1042 = match v0.clone() {
            Adt2::C0(_, _, _, h1040, _, _, _, _, _, _, _) => {
                h1040
            }
        };
        let a1043 = match v0 {
            Adt2::C0(_, _, _, _, h1041, _, _, _, _, _, _) => {
                h1041
            }
        };
        nat_mul(a1042, a1043)?
    };
    nat_mul(a1044, a1045)
}

pub fn f24(v0: bool, v1: u64) -> R<u64> {
    let a1048 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1049 = {
        let a1046 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1047 = v1;
        nat_mul(a1046, a1047)?
    };
    nat_add(a1048, a1049)
}

pub fn f25(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1051 = match v0.clone() {
            Adt2::C0(_, h1050, _, _, _, _, _, _, _, _, _) => {
                h1050
            }
        };
        let a1052 = v1;
        let a1053 = 0u64;
        nat_quot(a1051, a1052, a1053)
    }, match v0.clone() {
        Adt2::C0(_, _, h1054, _, _, _, _, _, _, _, _) => {
            h1054
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1055, _, _, _, _, _, _, _) => {
            h1055
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1056, _, _, _, _, _, _) => {
            h1056
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1057, _, _, _, _, _) => {
            h1057
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1058, _, _, _, _) => {
            h1058
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1059, _, _, _) => {
            h1059
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1060, _, _) => {
            h1060
        }
    })
}

pub fn f26(v0: Adt2, v1: u64) -> R<bool> {
    let a1061 = v1;
    let a1062 = 0u64;
    if nat_eq(a1061, a1062) {
        Ok(true)
    } else {
        let a1068 = {
            let a1065 = f91({
                let a1063 = v1;
                let a1064 = 1u64;
                nat_sub(a1063, a1064)
            }, f25(v0.clone(), 1u64))?;
            let a1066 = 4u64;
            nat_add(a1065, a1066)?
        };
        let a1069 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1067, _) => {
                h1067
            }
        };
        Ok(nat_le(a1068, a1069))
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1071 = match v1.clone() {
        Adt2::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
            h1070
        }
    };
    let a1072 = 0u64;
    if if nat_eq(a1071, a1072) {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1073 = f23(v0.clone(), 1u64)?;
        let a1074 = 0u64;
        if nat_eq(a1073, a1074) {
            Ok(true)
        } else {
            let a1080 = {
                let a1077 = f91({
                    let a1075 = f23(v0.clone(), 1u64)?;
                    let a1076 = 1u64;
                    nat_sub(a1075, a1076)
                }, f25(v0, 1u64))?;
                let a1078 = 4u64;
                nat_add(a1077, a1078)?
            };
            let a1081 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1079, _) => {
                    h1079
                }
            };
            Ok(nat_le(a1080, a1081))
        }
    } else {
        Ok(false)
    }
}

pub fn f28(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        4u64
    }
}

pub fn f29(v0: bool) -> u64 {
    if v0 {
        15u64
    } else {
        3u64
    }
}

pub fn f30(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1083 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1082, _) => {
            h1082
        }
    };
    let a1084 = 0u64;
    if nat_eq(a1083, a1084) {
        Ok(true)
    } else {
        let a1086 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1085, _) => {
                h1085
            }
        };
        let a1087 = 0u64;
        if nat_eq(a1086, a1087) {
            Ok(true)
        } else {
            let a1089 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1088) => {
                    h1088
                }
            };
            let a1090 = 0u64;
            if nat_eq(a1089, a1090) {
                Ok(true)
            } else {
                let a1092 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1091) => {
                        h1091
                    }
                };
                let a1093 = 0u64;
                if nat_eq(a1092, a1093) {
                    Ok(true)
                } else {
                    let a1099 = {
                        let a1096 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1094) => {
                                h1094
                            }
                        };
                        let a1097 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1095, _) => {
                                h1095
                            }
                        };
                        nat_add(a1096, a1097)?
                    };
                    let a1100 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1098) => {
                            h1098
                        }
                    };
                    if nat_le(a1099, a1100) {
                        Ok(true)
                    } else {
                        let a1106 = {
                            let a1103 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1101) => {
                                    h1101
                                }
                            };
                            let a1104 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1102, _) => {
                                    h1102
                                }
                            };
                            nat_add(a1103, a1104)?
                        };
                        let a1107 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1105) => {
                                h1105
                            }
                        };
                        Ok(nat_le(a1106, a1107))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1113 = match v0.clone() {
            Adt2::C0(_, _, h1108, _, _, _, _, _, _, _, _) => {
                h1108
            }
        };
        let a1114 = {
            let a1111 = match v0.clone() {
                Adt2::C0(_, _, _, h1109, _, _, _, _, _, _, _) => {
                    h1109
                }
            };
            let a1112 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1110, _, _, _, _, _, _) => {
                    h1110
                }
            };
            nat_mul(a1111, a1112)?
        };
        nat_mul(a1113, a1114)?
    };
    let v2: u64 = f23(v0.clone(), 1u64)?;
    let a1116 = match v0.clone() {
        Adt2::C0(h1115, _, _, _, _, _, _, _, _, _, _) => {
            h1115
        }
    };
    let a1117 = 0u64;
    if if if if if if if nat_eq(a1116, a1117) {
        let a1119 = match v0.clone() {
            Adt2::C0(_, h1118, _, _, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1120 = 2147483647u64;
        nat_le(a1119, a1120)
    } else {
        false
    } {
        let a1121 = v1;
        let a1122 = 2147483647u64;
        nat_le(a1121, a1122)
    } else {
        false
    } {
        let a1124 = match v0.clone() {
            Adt2::C0(_, h1123, _, _, _, _, _, _, _, _, _) => {
                h1123
            }
        };
        let a1125 = 1u64;
        if nat_eq(a1124, a1125) {
            true
        } else {
            let a1127 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1126, _, _, _, _, _) => {
                    h1126
                }
            };
            let a1128 = 4u64;
            nat_eq(a1127, a1128)
        }
    } else {
        false
    } {
        let a1129 = v1;
        let a1130 = 1u64;
        if nat_le(a1129, a1130) {
            true
        } else {
            let a1135 = {
                let a1132 = match v0.clone() {
                    Adt2::C0(_, h1131, _, _, _, _, _, _, _, _, _) => {
                        h1131
                    }
                };
                let a1133 = 4u64;
                nat_mul(a1132, a1133)?
            };
            let a1136 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1134, _, _, _, _) => {
                    h1134
                }
            };
            nat_le(a1135, a1136)
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
            let a1145 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1140, _, _, _) => {
                    h1140
                }
            };
            let a1146 = {
                let a1143 = match v0.clone() {
                    Adt2::C0(_, _, h1141, _, _, _, _, _, _, _, _) => {
                        h1141
                    }
                };
                let a1144 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1142, _, _, _, _) => {
                        h1142
                    }
                };
                nat_mul(a1143, a1144)?
            };
            nat_eq(a1145, a1146)
        }
    } else {
        false
    } {
        let a1148 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1147, _, _, _, _, _, _) => {
                h1147
            }
        };
        let a1149 = 1u64;
        if nat_eq(a1148, a1149) {
            true
        } else {
            let a1158 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1150, _, _) => {
                    h1150
                }
            };
            let a1159 = {
                let a1156 = match v0.clone() {
                    Adt2::C0(_, _, _, h1151, _, _, _, _, _, _, _) => {
                        h1151
                    }
                };
                let a1157 = {
                    let a1154 = match v0.clone() {
                        Adt2::C0(_, _, h1152, _, _, _, _, _, _, _, _) => {
                            h1152
                        }
                    };
                    let a1155 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1153, _, _, _, _) => {
                            h1153
                        }
                    };
                    nat_mul(a1154, a1155)?
                };
                nat_mul(a1156, a1157)?
            };
            nat_eq(a1158, a1159)
        }
    } else {
        false
    } {
        let a1160 = v2;
        let a1161 = 0u64;
        if nat_eq(a1160, a1161) {
            Ok(true)
        } else {
            let a1167 = {
                let a1164 = f91({
                    let a1162 = v2;
                    let a1163 = 1u64;
                    nat_sub(a1162, a1163)
                }, f25(v0.clone(), 1u64))?;
                let a1165 = 4u64;
                nat_add(a1164, a1165)?
            };
            let a1168 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1166, _) => {
                    h1166
                }
            };
            Ok(nat_le(a1167, a1168))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1175 = 148u64;
    let a1176 = {
        let a1173 = 36u64;
        let a1174 = {
            let a1171 = {
                let a1169 = 2u64;
                let a1170 = v0;
                nat_mul(a1169, a1170)?
            };
            let a1172 = 1u64;
            nat_sub(a1171, a1172)
        };
        nat_mul(a1173, a1174)?
    };
    nat_add(a1175, a1176)
}

pub fn f33(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1177 = v1;
    match m1177 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1180 = 1u64;
            let a1181 = {
                let a1178 = 2u64;
                let a1179 = v0;
                nat_mul(a1178, a1179)?
            };
            nat_add(a1180, a1181)
        }
    }
}

pub fn f34(v0: u64) -> bool {
    let a1182 = v0;
    let a1183 = 4294967296u64;
    if nat_lt(a1182, a1183) {
        let a1187 = {
            let a1184 = v0;
            let a1185 = 2147483648u64;
            let a1186 = 0u64;
            nat_rem(a1184, a1185, a1186)
        };
        let a1188 = 2139095040u64;
        nat_lt(a1187, a1188)
    } else {
        false
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a1189 = 4u64;
    let a1190 = v0;
    nat_mul(a1189, a1190)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1191 = v0;
    let a1192 = 0u64;
    if nat_eq(a1191, a1192) {
        Ok(0u64)
    } else {
        let a1195 = 4u64;
        let a1196 = {
            let a1193 = 3u64;
            let a1194 = v0;
            nat_mul(a1193, a1194)?
        };
        nat_add(a1195, a1196)
    }
}

pub fn f37(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f25(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1197, _, _, _, _, _, _, _, _, _) => {
                h1197
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1198, _, _, _, _, _, _, _, _) => {
                h1198
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1199, _, _, _, _, _, _, _) => {
                h1199
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1200, _, _, _, _, _) => {
                h1200
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1201, _, _, _, _) => {
                h1201
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1202, _, _, _) => {
                h1202
            }
        })
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let a1204 = match v0.clone() {
        Adt2::C0(_, h1203, _, _, _, _, _, _, _, _, _) => {
            h1203
        }
    };
    let a1205 = 1u64;
    if if if if nat_eq(a1204, a1205) {
        true
    } else {
        let a1207 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1206, _, _, _, _, _) => {
                h1206
            }
        };
        let a1208 = 4u64;
        nat_eq(a1207, a1208)
    } {
        let a1210 = match v0.clone() {
            Adt2::C0(_, _, h1209, _, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        let a1211 = 1u64;
        if nat_eq(a1210, a1211) {
            true
        } else {
            let a1216 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1212, _, _, _, _) => {
                    h1212
                }
            };
            let a1217 = {
                let a1214 = match v0.clone() {
                    Adt2::C0(_, h1213, _, _, _, _, _, _, _, _, _) => {
                        h1213
                    }
                };
                let a1215 = 4u64;
                nat_mul(a1214, a1215)?
            };
            nat_eq(a1216, a1217)
        }
    } else {
        false
    } {
        let a1219 = match v0.clone() {
            Adt2::C0(_, _, _, h1218, _, _, _, _, _, _, _) => {
                h1218
            }
        };
        let a1220 = 1u64;
        if nat_eq(a1219, a1220) {
            true
        } else {
            let a1228 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1221, _, _, _) => {
                    h1221
                }
            };
            let a1229 = {
                let a1226 = {
                    let a1224 = match v0.clone() {
                        Adt2::C0(_, h1222, _, _, _, _, _, _, _, _, _) => {
                            h1222
                        }
                    };
                    let a1225 = match v0.clone() {
                        Adt2::C0(_, _, h1223, _, _, _, _, _, _, _, _) => {
                            h1223
                        }
                    };
                    nat_mul(a1224, a1225)?
                };
                let a1227 = 4u64;
                nat_mul(a1226, a1227)?
            };
            nat_eq(a1228, a1229)
        }
    } else {
        false
    } {
        let a1231 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1230, _, _, _, _, _, _) => {
                h1230
            }
        };
        let a1232 = 1u64;
        if nat_eq(a1231, a1232) {
            Ok(true)
        } else {
            let a1243 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1233, _, _) => {
                    h1233
                }
            };
            let a1244 = {
                let a1241 = {
                    let a1239 = {
                        let a1236 = match v0.clone() {
                            Adt2::C0(_, h1234, _, _, _, _, _, _, _, _, _) => {
                                h1234
                            }
                        };
                        let a1237 = match v0.clone() {
                            Adt2::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                                h1235
                            }
                        };
                        nat_mul(a1236, a1237)?
                    };
                    let a1240 = match v0 {
                        Adt2::C0(_, _, _, h1238, _, _, _, _, _, _, _) => {
                            h1238
                        }
                    };
                    nat_mul(a1239, a1240)?
                };
                let a1242 = 4u64;
                nat_mul(a1241, a1242)?
            };
            Ok(nat_eq(a1243, a1244))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt6, v1: u64) -> R<u64> {
    let a1246 = match v0.clone() {
        Adt6::C0(h1245, _, _, _) => {
            h1245
        }
    };
    let a1247 = 0u64;
    if nat_eq(a1246, a1247) {
        f92(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1248, _, _) => {
                h1248
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1249, _) => {
                h1249
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1250) => {
                h1250
            }
        }), {
            let a1252 = v1;
            let a1253 = match v0 {
                Adt6::C0(_, _, _, h1251) => {
                    h1251
                }
            };
            nat_add(a1252, a1253)?
        })
    } else {
        f92(v0, v1)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1258 = {
        let a1254 = v0;
        let a1255 = v1;
        nat_mul(a1254, a1255)?
    };
    let a1259 = {
        let a1256 = v2;
        let a1257 = f93(v3);
        nat_mul(a1256, a1257)?
    };
    nat_mul(a1258, a1259)
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1262 = v0;
    let a1263 = {
        let a1260 = f94(v1)?;
        let a1261 = v2;
        nat_mul(a1260, a1261)?
    };
    nat_add(a1262, a1263)
}

pub fn f42(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1264, _, _, _, _, _, _, _, _, _, _) => {
            h1264
        }
    }, match v0.clone() {
        Adt2::C0(_, h1265, _, _, _, _, _, _, _, _, _) => {
            h1265
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1266, _, _, _, _, _, _, _, _) => {
            h1266
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1267, _, _, _, _, _, _, _) => {
            h1267
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1268, _, _, _, _, _, _) => {
            h1268
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1269, _, _, _, _, _) => {
            h1269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1270, _, _, _, _) => {
            h1270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1271, _, _, _) => {
            h1271
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1272, _, _) => {
            h1272
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1273, _) => {
            h1273
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1274) => {
            h1274
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1275 = v0;
    let a1276 = 23u64;
    if nat_eq(a1275, a1276) {
        f95(v1, v4, v6)
    } else {
        let a1277 = v0;
        let a1278 = 26u64;
        if nat_eq(a1277, a1278) {
            f96(v1, v2, v4, v6)
        } else {
            let a1279 = v0;
            let a1280 = 25u64;
            if nat_eq(a1279, a1280) {
                f97(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    let a1281 = v0;
    let a1282 = 30u64;
    if nat_eq(a1281, a1282) {
        2u64
    } else {
        4u64
    }
}

pub fn f45(v0: u64) -> bool {
    let a1283 = v0;
    let a1284 = 2139095040u64;
    if nat_le(a1283, a1284) {
        true
    } else {
        let a1285 = v0;
        let a1286 = 2147483648u64;
        nat_eq(a1285, a1286)
    }
}

pub fn f46(v0: Adt2, v1: u64) -> bool {
    let a1288 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1287, _, _, _, _, _) => {
            h1287
        }
    };
    let a1289 = v1;
    if if nat_eq(a1288, a1289) {
        let a1294 = {
            let a1291 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1290) => {
                    h1290
                }
            };
            let a1292 = v1;
            let a1293 = 0u64;
            nat_rem(a1291, a1292, a1293)
        };
        let a1295 = 0u64;
        nat_eq(a1294, a1295)
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt2::C0(_, _, h1296, _, _, _, _, _, _, _, _) => {
                h1296
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1297, _, _, _, _, _, _, _) => {
                h1297
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1298, _, _, _, _, _, _) => {
                h1298
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1299, _, _, _, _) => {
                h1299
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1300, _, _, _) => {
                h1300
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1301, _, _) => {
                h1301
            }
        }, v1)
    } else {
        false
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a1304 = match v0.clone() {
            Adt2::C0(h1302, _, _, _, _, _, _, _, _, _, _) => {
                h1302
            }
        };
        let a1305 = match v1.clone() {
            Adt2::C0(h1303, _, _, _, _, _, _, _, _, _, _) => {
                h1303
            }
        };
        nat_eq(a1304, a1305)
    } else {
        false
    } {
        let a1308 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1306) => {
                h1306
            }
        };
        let a1309 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1307) => {
                h1307
            }
        };
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1312 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1310, _, _, _, _, _) => {
                h1310
            }
        };
        let a1313 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1311, _, _, _, _, _) => {
                h1311
            }
        };
        nat_eq(a1312, a1313)
    } else {
        false
    } {
        let a1316 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1314, _, _, _, _) => {
                h1314
            }
        };
        let a1317 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1315, _, _, _, _) => {
                h1315
            }
        };
        nat_eq(a1316, a1317)
    } else {
        false
    } {
        let a1320 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1318, _, _, _) => {
                h1318
            }
        };
        let a1321 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1319, _, _, _) => {
                h1319
            }
        };
        nat_eq(a1320, a1321)
    } else {
        false
    } {
        let a1324 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1322, _, _) => {
                h1322
            }
        };
        let a1325 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1323, _, _) => {
                h1323
            }
        };
        nat_eq(a1324, a1325)
    } else {
        false
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1326 = v0;
    let a1327 = 0u64;
    if if if nat_eq(a1326, a1327) {
        true
    } else {
        let a1328 = v1;
        let a1329 = 0u64;
        nat_eq(a1328, a1329)
    } {
        true
    } else {
        let a1330 = v2;
        let a1331 = 0u64;
        nat_eq(a1330, a1331)
    } {
        true
    } else {
        let a1332 = v3;
        let a1333 = 0u64;
        nat_eq(a1332, a1333)
    }
}

pub fn f49(v0: Adt2, v1: u64) -> R<bool> {
    f99({
        let a1335 = match v0.clone() {
            Adt2::C0(_, h1334, _, _, _, _, _, _, _, _, _) => {
                h1334
            }
        };
        let a1336 = v1;
        nat_mul(a1335, a1336)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1337, _, _, _, _, _, _, _, _) => {
            h1337
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1338, _, _, _, _, _, _, _) => {
            h1338
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1339, _, _, _, _, _, _) => {
            h1339
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1340, _, _, _, _) => {
            h1340
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1341, _, _, _) => {
            h1341
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1342, _, _) => {
            h1342
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1343 = v1;
    let a1344 = 0u64;
    if if nat_eq(a1343, a1344) {
        true
    } else {
        let a1345 = v2;
        let a1346 = 0u64;
        nat_eq(a1345, a1346)
    } {
        Ok(0u64)
    } else {
        let a1353 = {
            let a1347 = f100(v1, v2, v3)?;
            let a1348 = f101(v0, v1, v2)?;
            nat_add(a1347, a1348)?
        };
        let a1354 = {
            let a1351 = f102(v0);
            let a1352 = {
                let a1349 = v1;
                let a1350 = v2;
                nat_mul(a1349, a1350)?
            };
            nat_mul(a1351, a1352)?
        };
        nat_add(a1353, a1354)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1355 = v0;
    let a1356 = 0u64;
    if if nat_eq(a1355, a1356) {
        true
    } else {
        let a1357 = v1;
        let a1358 = 0u64;
        nat_eq(a1357, a1358)
    } {
        Ok(3u64)
    } else {
        let a1361 = {
            let a1359 = 3u64;
            let a1360 = v1;
            nat_add(a1359, a1360)?
        };
        let a1362 = v2;
        nat_add(a1361, a1362)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1363 = v0;
    let a1364 = 2139095040u64;
    if nat_le(a1363, a1364) {
        true
    } else {
        let a1365 = v0;
        let a1366 = 2147483648u64;
        nat_eq(a1365, a1366)
    }
}

pub fn f53(v0: u64, v1: u64) -> R<u64> {
    let a1367 = v0;
    let a1368 = 0u64;
    if nat_eq(a1367, a1368) {
        Ok(0u64)
    } else {
        let a1369 = v1;
        let a1370 = f103(v0)?;
        nat_mul(a1369, a1370)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1371 = v0;
    let a1372 = 0u64;
    if if if if nat_eq(a1371, a1372) {
        true
    } else {
        let a1373 = v1;
        let a1374 = 0u64;
        nat_eq(a1373, a1374)
    } {
        true
    } else {
        let a1375 = v2;
        let a1376 = 0u64;
        nat_eq(a1375, a1376)
    } {
        true
    } else {
        let a1377 = v3;
        let a1378 = 0u64;
        nat_eq(a1377, a1378)
    } {
        Ok(0u64)
    } else {
        let a1391 = {
            let a1381 = 2u64;
            let a1382 = {
                let a1379 = v1;
                let a1380 = v2;
                nat_mul(a1379, a1380)?
            };
            nat_mul(a1381, a1382)?
        };
        let a1392 = {
            let a1389 = {
                let a1385 = {
                    let a1383 = v1;
                    let a1384 = v2;
                    nat_mul(a1383, a1384)?
                };
                let a1386 = v3;
                nat_mul(a1385, a1386)?
            };
            let a1390 = {
                let a1387 = 1u64;
                let a1388 = v0;
                nat_add(a1387, a1388)?
            };
            nat_mul(a1389, a1390)?
        };
        nat_add(a1391, a1392)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1393 = v1;
        let a1394 = 0u64;
        if if if nat_eq(a1393, a1394) {
            true
        } else {
            let a1395 = v1;
            let a1396 = 10u64;
            nat_eq(a1395, a1396)
        } {
            true
        } else {
            let a1397 = v1;
            let a1398 = 15u64;
            nat_eq(a1397, a1398)
        } {
            true
        } else {
            let a1399 = v1;
            let a1400 = 20u64;
            nat_eq(a1399, a1400)
        }
    };
    let v4: bool = {
        let a1401 = v2;
        let a1402 = 0u64;
        if if if if if nat_eq(a1401, a1402) {
            true
        } else {
            let a1403 = v2;
            let a1404 = 10u64;
            nat_eq(a1403, a1404)
        } {
            true
        } else {
            let a1405 = v2;
            let a1406 = 15u64;
            nat_eq(a1405, a1406)
        } {
            true
        } else {
            let a1407 = v2;
            let a1408 = 20u64;
            nat_eq(a1407, a1408)
        } {
            true
        } else {
            let a1409 = v2;
            let a1410 = 30u64;
            nat_eq(a1409, a1410)
        } {
            true
        } else {
            let a1411 = v2;
            let a1412 = 40u64;
            nat_eq(a1411, a1412)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1413 = v0;
            let a1414 = 30u64;
            if if nat_eq(a1413, a1414) {
                let a1415 = v2;
                let a1416 = 10u64;
                nat_eq(a1415, a1416)
            } else {
                false
            } {
                true
            } else {
                if f104(v0) {
                    let a1417 = v2;
                    let a1418 = 10u64;
                    if if nat_eq(a1417, a1418) {
                        true
                    } else {
                        let a1419 = v2;
                        let a1420 = 15u64;
                        nat_eq(a1419, a1420)
                    } {
                        true
                    } else {
                        let a1421 = v2;
                        let a1422 = 20u64;
                        nat_eq(a1421, a1422)
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

pub fn f56(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1423, _, _, _, _, _, _, _, _, _, _, _) => {
            h1423
        }
    };
    Ok(Adt9::C0({
        let a1424 = v0;
        let a1425 = 50u64;
        if nat_eq(a1424, a1425) {
            let m1426 = v3;
            match m1426 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f105(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1427 = v0;
            let a1428 = 82u64;
            if nat_eq(a1427, a1428) {
                let m1429 = v3;
                match m1429 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f106(v2, v7, v4)?
                    }
                }
            } else {
                let a1430 = v0;
                let a1431 = 86u64;
                if nat_eq(a1430, a1431) {
                    let m1432 = v3;
                    match m1432 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f107(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1433 = v0;
                    let a1434 = 55u64;
                    if nat_eq(a1433, a1434) {
                        let m1435 = v3;
                        match m1435 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f108(v1, v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1436 = v0;
                        let a1437 = 22u64;
                        if nat_eq(a1436, a1437) {
                            let m1438 = v3;
                            match m1438 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f109(v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1439 = v0;
                            let a1440 = 44u64;
                            if nat_eq(a1439, a1440) {
                                f110(v1, v2, v3, v4)?
                            } else {
                                let a1441 = v0;
                                let a1442 = 52u64;
                                if nat_eq(a1441, a1442) {
                                    f111(v1, v2, v4)?
                                } else {
                                    let a1443 = v0;
                                    let a1444 = 73u64;
                                    if nat_eq(a1443, a1444) {
                                        f112(v2, v3, v4)?
                                    } else {
                                        let a1445 = v0;
                                        let a1446 = 57u64;
                                        if if nat_eq(a1445, a1446) {
                                            true
                                        } else {
                                            let a1447 = v0;
                                            let a1448 = 58u64;
                                            nat_eq(a1447, a1448)
                                        } {
                                            let m1449 = v3;
                                            match m1449 {
                                                None => {
                                                    f113({
                                                        let a1450 = v0;
                                                        let a1451 = 58u64;
                                                        nat_eq(a1450, a1451)
                                                    }, v1, v2, v4)?
                                                }
                                                Some(_) => {
                                                    Adt10::C1
                                                }
                                            }
                                        } else {
                                            let a1452 = v0;
                                            let a1453 = 49u64;
                                            if nat_eq(a1452, a1453) {
                                                f114(false, v1, v2, v3, v4)?
                                            } else {
                                                let a1454 = v0;
                                                let a1455 = 51u64;
                                                if nat_eq(a1454, a1455) {
                                                    f115(v1, v2, v3, v4)?
                                                } else {
                                                    let a1456 = v0;
                                                    let a1457 = 39u64;
                                                    if nat_eq(a1456, a1457) {
                                                        f116(v2, v3, v4)?
                                                    } else {
                                                        let a1458 = v0;
                                                        let a1459 = 77u64;
                                                        if nat_eq(a1458, a1459) {
                                                            f117(v2, v4)?
                                                        } else {
                                                            let a1460 = v0;
                                                            let a1461 = 40u64;
                                                            if nat_eq(a1460, a1461) {
                                                                f118(v2, v3, v4)?
                                                            } else {
                                                                if f119(v0) {
                                                                    let a1462 = v0;
                                                                    let a1463 = 33u64;
                                                                    if if if nat_eq(a1462, a1463) {
                                                                        let a1465 = match v2.clone() {
                                                                            Adt2::C0(h1464, _, _, _, _, _, _, _, _, _, _) => {
                                                                                h1464
                                                                            }
                                                                        };
                                                                        let a1466 = 0u64;
                                                                        nat_eq(a1465, a1466)
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        let a1468 = match v4.clone() {
                                                                            Adt2::C0(h1467, _, _, _, _, _, _, _, _, _, _) => {
                                                                                h1467
                                                                            }
                                                                        };
                                                                        let a1469 = 26u64;
                                                                        nat_eq(a1468, a1469)
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        let m1470 = v3;
                                                                        match m1470 {
                                                                            None => {
                                                                                Adt10::C1
                                                                            }
                                                                            Some(v12) => {
                                                                                f120(v2, v12, v4)?
                                                                            }
                                                                        }
                                                                    } else {
                                                                        let a1471 = v0;
                                                                        let a1472 = 33u64;
                                                                        if if if nat_eq(a1471, a1472) {
                                                                            let a1474 = match v4.clone() {
                                                                                Adt2::C0(h1473, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    h1473
                                                                                }
                                                                            };
                                                                            let a1475 = 0u64;
                                                                            nat_eq(a1474, a1475)
                                                                        } else {
                                                                            false
                                                                        } {
                                                                            f121(match v2.clone() {
                                                                                Adt2::C0(h1476, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    h1476
                                                                                }
                                                                            })
                                                                        } else {
                                                                            false
                                                                        } {
                                                                            f122(v2, v3, v4)?
                                                                        } else {
                                                                            f123(v0, v2, {
                                                                                let m1477 = v3;
                                                                                match m1477 {
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
                                                                    let a1478 = v0;
                                                                    let a1479 = 19u64;
                                                                    if nat_eq(a1478, a1479) {
                                                                        f124(v2, v4)?
                                                                    } else {
                                                                        let a1480 = v0;
                                                                        let a1481 = 21u64;
                                                                        if nat_eq(a1480, a1481) {
                                                                            f125(v5, v2, v3, v4)?
                                                                        } else {
                                                                            let a1482 = v0;
                                                                            let a1483 = 61u64;
                                                                            if if if nat_eq(a1482, a1483) {
                                                                                true
                                                                            } else {
                                                                                let a1484 = v0;
                                                                                let a1485 = 62u64;
                                                                                nat_eq(a1484, a1485)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a1486 = v0;
                                                                                let a1487 = 63u64;
                                                                                nat_eq(a1486, a1487)
                                                                            } {
                                                                                f126(v0, v1, v2, v4)?
                                                                            } else {
                                                                                let a1488 = v0;
                                                                                let a1489 = 4u64;
                                                                                if if nat_eq(a1488, a1489) {
                                                                                    true
                                                                                } else {
                                                                                    let a1490 = v0;
                                                                                    let a1491 = 32u64;
                                                                                    nat_eq(a1490, a1491)
                                                                                } {
                                                                                    f127(v0, v1, v2, v3, v4)?
                                                                                } else {
                                                                                    let a1492 = v0;
                                                                                    let a1493 = 20u64;
                                                                                    if nat_eq(a1492, a1493) {
                                                                                        f128(v2, v4)?
                                                                                    } else {
                                                                                        let a1494 = v0;
                                                                                        let a1495 = 42u64;
                                                                                        if nat_eq(a1494, a1495) {
                                                                                            f129(4u64, v2, v4)?
                                                                                        } else {
                                                                                            let a1496 = v0;
                                                                                            let a1497 = 69u64;
                                                                                            if nat_eq(a1496, a1497) {
                                                                                                let a1498 = v5;
                                                                                                let a1499 = 4u64;
                                                                                                if nat_lt(a1498, a1499) {
                                                                                                    f129(v5, v2, v4)?
                                                                                                } else {
                                                                                                    Adt10::C1
                                                                                                }
                                                                                            } else {
                                                                                                let a1500 = v0;
                                                                                                let a1501 = 70u64;
                                                                                                if nat_eq(a1500, a1501) {
                                                                                                    f130(v5, v2, v4)?
                                                                                                } else {
                                                                                                    let a1502 = v0;
                                                                                                    let a1503 = 75u64;
                                                                                                    if if nat_eq(a1502, a1503) {
                                                                                                        true
                                                                                                    } else {
                                                                                                        let a1504 = v0;
                                                                                                        let a1505 = 76u64;
                                                                                                        nat_eq(a1504, a1505)
                                                                                                    } {
                                                                                                        f131({
                                                                                                            let a1506 = v0;
                                                                                                            let a1507 = 76u64;
                                                                                                            nat_eq(a1506, a1507)
                                                                                                        }, v1, v2, v4)?
                                                                                                    } else {
                                                                                                        let a1508 = v0;
                                                                                                        let a1509 = 31u64;
                                                                                                        if if if nat_eq(a1508, a1509) {
                                                                                                            true
                                                                                                        } else {
                                                                                                            let a1510 = v0;
                                                                                                            let a1511 = 47u64;
                                                                                                            nat_eq(a1510, a1511)
                                                                                                        } {
                                                                                                            true
                                                                                                        } else {
                                                                                                            let a1512 = v0;
                                                                                                            let a1513 = 68u64;
                                                                                                            nat_eq(a1512, a1513)
                                                                                                        } {
                                                                                                            f132(v0, v1, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1514 = v0;
                                                                                                            let a1515 = 15u64;
                                                                                                            if nat_eq(a1514, a1515) {
                                                                                                                f133(v2, v4)?
                                                                                                            } else {
                                                                                                                let a1516 = v0;
                                                                                                                let a1517 = 17u64;
                                                                                                                if nat_eq(a1516, a1517) {
                                                                                                                    f134(v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1518 = v0;
                                                                                                                    let a1519 = 18u64;
                                                                                                                    if nat_eq(a1518, a1519) {
                                                                                                                        f135(v2, v3, v4)?
                                                                                                                    } else {
                                                                                                                        let a1520 = v0;
                                                                                                                        let a1521 = 13u64;
                                                                                                                        if if if nat_eq(a1520, a1521) {
                                                                                                                            true
                                                                                                                        } else {
                                                                                                                            let a1522 = v0;
                                                                                                                            let a1523 = 14u64;
                                                                                                                            nat_eq(a1522, a1523)
                                                                                                                        } {
                                                                                                                            true
                                                                                                                        } else {
                                                                                                                            let a1524 = v0;
                                                                                                                            let a1525 = 16u64;
                                                                                                                            nat_eq(a1524, a1525)
                                                                                                                        } {
                                                                                                                            f136(v0, v2, v4)?
                                                                                                                        } else {
                                                                                                                            let m1526 = f137(v0, v5)?;
                                                                                                                            match m1526 {
                                                                                                                                None => {
                                                                                                                                    f138(v0, v5, v2, v3, v4)?
                                                                                                                                }
                                                                                                                                Some(v14) => {
                                                                                                                                    f139(v14, v2, v4)?
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f57(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f140(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1527, _, _, _) => {
            h1527
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1528, _, _) => {
            h1528
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1529, _) => {
            h1529
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1530) => {
            h1530
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1532 = v0;
        let a1533 = match v1.clone() {
            Adt3::C0(h1531, _, _, _, _, _, _, _) => {
                h1531
            }
        };
        let a1534 = 0u64;
        nat_quot(a1532, a1533, a1534)
    };
    let v4: u64 = {
        let a1536 = v3;
        let a1537 = match v1.clone() {
            Adt3::C0(_, h1535, _, _, _, _, _, _) => {
                h1535
            }
        };
        let a1538 = 0u64;
        nat_quot(a1536, a1537, a1538)
    };
    let a1539 = v2;
    let a1540 = 0u64;
    if nat_eq(a1539, a1540) {
        let a1542 = v0;
        let a1543 = match v1 {
            Adt3::C0(h1541, _, _, _, _, _, _, _) => {
                h1541
            }
        };
        let a1544 = 0u64;
        nat_rem(a1542, a1543, a1544)
    } else {
        let a1545 = v2;
        let a1546 = 1u64;
        if nat_eq(a1545, a1546) {
            let a1548 = v3;
            let a1549 = match v1 {
                Adt3::C0(_, h1547, _, _, _, _, _, _) => {
                    h1547
                }
            };
            let a1550 = 0u64;
            nat_rem(a1548, a1549, a1550)
        } else {
            let a1551 = v2;
            let a1552 = 2u64;
            if nat_eq(a1551, a1552) {
                let a1554 = v4;
                let a1555 = match v1 {
                    Adt3::C0(_, _, h1553, _, _, _, _, _) => {
                        h1553
                    }
                };
                let a1556 = 0u64;
                nat_rem(a1554, a1555, a1556)
            } else {
                let a1558 = v4;
                let a1559 = match v1 {
                    Adt3::C0(_, _, h1557, _, _, _, _, _) => {
                        h1557
                    }
                };
                let a1560 = 0u64;
                nat_quot(a1558, a1559, a1560)
            }
        }
    }
}

pub fn f59(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1562 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1561) => {
            h1561
        }
    };
    let a1563 = 0u64;
    if nat_eq(a1562, a1563) {
        Ok(v2)
    } else {
        let a1568 = {
            let a1565 = v2;
            let a1566 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1564) => {
                    h1564
                }
            };
            nat_mul(a1565, a1566)?
        };
        let a1569 = f58(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1567, _, _, _, _, _, _) => {
                h1567
            }
        }, 3u64);
        nat_add(a1568, a1569)
    }
}

pub fn f60(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1579 = {
            let a1576 = {
                let a1571 = v2;
                let a1572 = match v0.clone() {
                    Adt6::C0(_, h1570, _, _) => {
                        h1570
                    }
                };
                nat_mul(a1571, a1572)?
            };
            let a1577 = {
                let a1574 = v3;
                let a1575 = match v0.clone() {
                    Adt6::C0(_, _, _, h1573) => {
                        h1573
                    }
                };
                nat_mul(a1574, a1575)?
            };
            nat_add(a1576, a1577)?
        };
        let a1580 = f141(match v0.clone() {
            Adt6::C0(_, _, h1578, _) => {
                h1578
            }
        });
        nat_add(a1579, a1580)?
    };
    let v5: u64 = f142(match v0 {
        Adt6::C0(_, _, h1581, _) => {
            h1581
        }
    });
    let a1582 = v5;
    let a1583 = v4;
    if if nat_le(a1582, a1583) {
        let a1586 = {
            let a1584 = v4;
            let a1585 = v5;
            nat_sub(a1584, a1585)
        };
        let a1587 = v1;
        nat_lt(a1586, a1587)
    } else {
        false
    } {
        Ok(Some({
            let a1588 = v4;
            let a1589 = v5;
            nat_sub(a1588, a1589)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1590, _, _, _, _, _) => {
            h1590
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1591, _, _, _, _, _, _) => {
            h1591
        }
    };
    let v7: u64 = f58(v1, v6.clone(), 2u64);
    let a1593 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1592) => {
            h1592
        }
    };
    let a1594 = 0u64;
    if nat_eq(a1593, a1594) {
        Ok(Adt9::C8(Some(f57(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1597 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1596, _, _, _, _) => {
                    h1596
                }
            };
            match m1597 {
                Adt3::C0(_, _, h1595, _, _, _, _, _) => {
                    h1595
                }
            }
        };
        let m1603 = f60(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1598, _) => {
                h1598
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1599, _, _, _, _, _) => {
                h1599
            }
        }, {
            let a1600 = v2;
            let a1601 = v8;
            let a1602 = 0u64;
            nat_rem(a1600, a1601, a1602)
        }, v7)?;
        match m1603 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1610 = {
                        let a1608 = {
                            let a1604 = v2;
                            let a1605 = v8;
                            let a1606 = 0u64;
                            nat_quot(a1604, a1605, a1606)
                        };
                        let a1609 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1607) => {
                                h1607
                            }
                        };
                        nat_mul(a1608, a1609)?
                    };
                    let a1611 = f58(v1, v6, 3u64);
                    nat_add(a1610, a1611)?
                };
                Ok(Adt9::C8(Some(f57(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1612 = v1;
    match m1612 {
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
            f143(f144(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f145(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f146(v0.clone(), v24);
            f143(f147(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1613 = v3;
    match m1613 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f91(v0, v1)?, f91(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f149(v0, v1, v2.clone())?, f91(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f150(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f151(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f152(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f151(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f153(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f154(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f155(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f91(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f156(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v53, v54) => {
            f157(v0, v1, v2, v53, v54)
        }
        Adt4::C12(v55, v56, v57) => {
            f158(v0, v1, v2, v55, v56, v57)
        }
        Adt4::C13(_, v59, _, _) => {
            Ok(Adt9::C3(v59, v59, f91(v0, v1)?, f91(v0, v2)?))
        }
        Adt4::C14(v62, v63, v64) => {
            let a1616 = v62;
            let a1617 = 23u64;
            if nat_eq(a1616, a1617) {
                f159(v0, v1, v2, v63, v64)
            } else {
                Ok(Adt9::C3(v63, v64, f91(v0, v1)?, f91(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f160(v0, v1, v2)
        }
        Adt4::C16(v65, v66) => {
            f161(v0, v1, v2, v65, v66)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v67, v68) => {
            f162(v0, v1, v68, v67)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f91(f148(v0, match v2.clone() {
                Adt3::C0(h1614, _, _, _, _, _, _, _) => {
                    h1614
                }
            }, match v2.clone() {
                Adt3::C0(_, h1615, _, _, _, _, _, _) => {
                    h1615
                }
            })?, v1)?, f91(v0, v2)?))
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
        Adt4::C25(v73, v74, v75) => {
            f163(v0, v73, v74, v75)
        }
        Adt4::C26(v81, _, _, _) => {
            let a1618 = v81;
            let a1619 = 64u64;
            if if if nat_eq(a1618, a1619) {
                true
            } else {
                let a1620 = v81;
                let a1621 = 65u64;
                nat_eq(a1620, a1621)
            } {
                true
            } else {
                let a1622 = v81;
                let a1623 = 66u64;
                nat_eq(a1622, a1623)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v80) => {
            f83(v0, v1, v2, v80)
        }
        Adt4::C28(v76) => {
            f80(v0, v1, v2, v76, 0u64)
        }
        Adt4::C29(v77) => {
            f81(v0, v1, v2, v77)
        }
        Adt4::C30(v78, _) => {
            f82(v0, v1, v2, v78)
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
        Adt4::C40(_, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C41 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C42(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f164(v3, v4);
    let v6: u64 = {
        let a1624 = v0;
        let a1625 = v5;
        let a1626 = 0u64;
        nat_quot(a1624, a1625, a1626)
    };
    let a1627 = v5;
    let a1628 = 1u64;
    if nat_eq(a1627, a1628) {
        Ok(f143(f165(v3, v4), v3, v4, 0u64, f91(v6, v1)?, 2u64, 0u64, 3u64, f91(v6, v2)?))
    } else {
        let a1632 = {
            let a1629 = v0;
            let a1630 = 2u64;
            let a1631 = 0u64;
            nat_rem(a1629, a1630, a1631)
        };
        let a1633 = 0u64;
        if nat_eq(a1632, a1633) {
            Ok(f143(f165(v3, 0u64), v3, 0u64, 0u64, f91(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f143(f165(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f91(v6, v2)?))
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f166(v1)?;
    let v4: u64 = {
        let a1634 = v0;
        let a1635 = v3;
        let a1636 = 0u64;
        nat_rem(a1634, a1635, a1636)
    };
    let v5: u64 = {
        let a1639 = {
            let a1637 = v1;
            let a1638 = 3u64;
            nat_mul(a1637, a1638)?
        };
        let a1640 = 1u64;
        nat_add(a1639, a1640)?
    };
    let a1641 = v4;
    let a1642 = 0u64;
    if nat_eq(a1641, a1642) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1643 = v4;
        let a1644 = v5;
        if nat_eq(a1643, a1644) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1649 = v4;
                let a1650 = {
                    let a1645 = v4;
                    let a1646 = v5;
                    if nat_lt(a1645, a1646) {
                        1u64
                    } else {
                        let a1647 = v5;
                        let a1648 = 1u64;
                        nat_add(a1647, a1648)?
                    }
                };
                nat_sub(a1649, a1650)
            };
            let v7: u64 = {
                let a1651 = v6;
                let a1652 = 3u64;
                let a1653 = 0u64;
                nat_rem(a1651, a1652, a1653)
            };
            let v8: u64 = {
                let a1664 = 4u64;
                let a1665 = {
                    let a1662 = {
                        let a1657 = {
                            let a1654 = v0;
                            let a1655 = v3;
                            let a1656 = 0u64;
                            nat_quot(a1654, a1655, a1656)
                        };
                        let a1658 = v1;
                        nat_mul(a1657, a1658)?
                    };
                    let a1663 = {
                        let a1659 = v6;
                        let a1660 = 3u64;
                        let a1661 = 0u64;
                        nat_quot(a1659, a1660, a1661)
                    };
                    nat_add(a1662, a1663)?
                };
                nat_mul(a1664, a1665)?
            };
            let a1666 = v4;
            let a1667 = v5;
            if nat_lt(a1666, a1667) {
                let a1668 = v7;
                let a1669 = 0u64;
                if nat_eq(a1668, a1669) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1670 = v7;
                    let a1671 = 1u64;
                    if nat_eq(a1670, a1671) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1672 = v7;
                let a1673 = 0u64;
                if nat_eq(a1672, a1673) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1674 = v7;
                    let a1675 = 1u64;
                    if nat_eq(a1674, a1675) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1676 = v0;
        let a1677 = 6u64;
        let a1678 = 0u64;
        nat_quot(a1676, a1677, a1678)
    };
    let v5: u64 = {
        let a1679 = v0;
        let a1680 = 6u64;
        let a1681 = 0u64;
        nat_rem(a1679, a1680, a1681)
    };
    let a1682 = v5;
    let a1683 = 0u64;
    if nat_eq(a1682, a1683) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f91(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1684 = v5;
        let a1685 = 1u64;
        if nat_eq(a1684, a1685) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1686 = v5;
            let a1687 = 2u64;
            if nat_eq(a1686, a1687) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f91(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1688 = v5;
                let a1689 = 3u64;
                if nat_eq(a1688, a1689) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1690 = v5;
                    let a1691 = 4u64;
                    if nat_eq(a1690, a1691) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f91(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f91(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f167(match v1.clone() {
        Adt3::C0(_, h1692, _, _, _, _, _, _) => {
            h1692
        }
    }, v4)?;
    let v8: u64 = {
        let a1693 = v0;
        let a1694 = v7;
        let a1695 = 0u64;
        nat_rem(a1693, a1694, a1695)
    };
    let v9: u64 = {
        let a1696 = v0;
        let a1697 = v7;
        let a1698 = 0u64;
        nat_quot(a1696, a1697, a1698)
    };
    let a1701 = {
        let a1699 = v8;
        let a1700 = 1u64;
        nat_add(a1699, a1700)?
    };
    let a1702 = v7;
    if nat_eq(a1701, a1702) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f91(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1703 = v8;
            let a1704 = v6;
            let a1705 = 0u64;
            nat_quot(a1703, a1704, a1705)
        };
        let v11: u64 = {
            let a1706 = v8;
            let a1707 = v6;
            let a1708 = 0u64;
            nat_rem(a1706, a1707, a1708)
        };
        let v12: u64 = {
            let a1710 = v9;
            let a1711 = match v1.clone() {
                Adt3::C0(h1709, _, _, _, _, _, _, _) => {
                    h1709
                }
            };
            let a1712 = 0u64;
            nat_rem(a1710, a1711, a1712)
        };
        let v13: u64 = {
            let a1714 = v9;
            let a1715 = match v1.clone() {
                Adt3::C0(h1713, _, _, _, _, _, _, _) => {
                    h1713
                }
            };
            let a1716 = 0u64;
            nat_quot(a1714, a1715, a1716)
        };
        let v14: u64 = if v4 {
            let a1728 = {
                let a1723 = {
                    let a1718 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1717, _, _, _) => {
                            h1717
                        }
                    };
                    let a1719 = v12;
                    nat_mul(a1718, a1719)?
                };
                let a1724 = {
                    let a1721 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1720, _, _) => {
                            h1720
                        }
                    };
                    let a1722 = v10;
                    nat_mul(a1721, a1722)?
                };
                nat_add(a1723, a1724)?
            };
            let a1729 = {
                let a1726 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1725, _) => {
                        h1725
                    }
                };
                let a1727 = v13;
                nat_mul(a1726, a1727)?
            };
            nat_add(a1728, a1729)?
        } else {
            let a1736 = {
                let a1731 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1730, _, _, _) => {
                        h1730
                    }
                };
                let a1732 = v10;
                nat_mul(a1731, a1732)?
            };
            let a1737 = {
                let a1734 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1733, _, _) => {
                        h1733
                    }
                };
                let a1735 = v13;
                nat_mul(a1734, a1735)?
            };
            nat_add(a1736, a1737)?
        };
        if if v4 {
            let a1738 = v11;
            let a1739 = 0u64;
            nat_eq(a1738, a1739)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1742 = {
                let a1740 = v11;
                let a1741 = 1u64;
                nat_add(a1740, a1741)?
            };
            let a1743 = v6;
            if nat_eq(a1742, a1743) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1744 = v10;
                    let a1745 = 0u64;
                    if nat_eq(a1744, a1745) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1757 = {
                        let a1752 = {
                            let a1747 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1746, _, _, _) => {
                                    h1746
                                }
                            };
                            let a1748 = v12;
                            nat_mul(a1747, a1748)?
                        };
                        let a1753 = {
                            let a1750 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1749, _, _) => {
                                    h1749
                                }
                            };
                            let a1751 = v10;
                            nat_mul(a1750, a1751)?
                        };
                        nat_add(a1752, a1753)?
                    };
                    let a1758 = {
                        let a1755 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1754, _) => {
                                h1754
                            }
                        };
                        let a1756 = v13;
                        nat_mul(a1755, a1756)?
                    };
                    nat_add(a1757, a1758)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f33(match v2.clone() {
        Adt3::C0(_, h1759, _, _, _, _, _, _) => {
            h1759
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1760 = v0;
        let a1761 = v6;
        let a1762 = 0u64;
        nat_rem(a1760, a1761, a1762)
    };
    let v8: u64 = {
        let a1763 = v0;
        let a1764 = v6;
        let a1765 = 0u64;
        nat_quot(a1763, a1764, a1765)
    };
    let v9: u64 = {
        let a1767 = v8;
        let a1768 = match v5.clone() {
            Adt3::C0(h1766, _, _, _, _, _, _, _) => {
                h1766
            }
        };
        let a1769 = 0u64;
        nat_rem(a1767, a1768, a1769)
    };
    let v10: u64 = {
        let a1771 = v8;
        let a1772 = match v5.clone() {
            Adt3::C0(h1770, _, _, _, _, _, _, _) => {
                h1770
            }
        };
        let a1773 = 0u64;
        nat_quot(a1771, a1772, a1773)
    };
    let v11: u64 = {
        let a1775 = v10;
        let a1776 = match v2.clone() {
            Adt3::C0(_, h1774, _, _, _, _, _, _) => {
                h1774
            }
        };
        let a1777 = 0u64;
        nat_rem(a1775, a1776, a1777)
    };
    let v12: u64 = {
        let a1779 = v10;
        let a1780 = match v2.clone() {
            Adt3::C0(_, h1778, _, _, _, _, _, _) => {
                h1778
            }
        };
        let a1781 = 0u64;
        nat_quot(a1779, a1780, a1781)
    };
    let a1782 = v7;
    let a1783 = 0u64;
    if nat_eq(a1782, a1783) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1790 = {
                let a1785 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1784, _, _, _) => {
                        h1784
                    }
                };
                let a1786 = v9;
                nat_mul(a1785, a1786)?
            };
            let a1791 = {
                let a1788 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1787, _, _) => {
                        h1787
                    }
                };
                let a1789 = v12;
                nat_mul(a1788, a1789)?
            };
            nat_add(a1790, a1791)?
        }), Adt12::C0(2u64, {
            let a1798 = {
                let a1793 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1792, _, _, _) => {
                        h1792
                    }
                };
                let a1794 = v11;
                nat_mul(a1793, a1794)?
            };
            let a1799 = {
                let a1796 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1795, _, _) => {
                        h1795
                    }
                };
                let a1797 = v12;
                nat_mul(a1796, a1797)?
            };
            nat_add(a1798, a1799)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1800 = v4;
        match m1800 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1812 = {
                        let a1807 = {
                            let a1802 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1801, _, _, _) => {
                                    h1801
                                }
                            };
                            let a1803 = v9;
                            nat_mul(a1802, a1803)?
                        };
                        let a1808 = {
                            let a1805 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1804, _, _) => {
                                    h1804
                                }
                            };
                            let a1806 = v11;
                            nat_mul(a1805, a1806)?
                        };
                        nat_add(a1807, a1808)?
                    };
                    let a1813 = {
                        let a1810 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1809, _) => {
                                h1809
                            }
                        };
                        let a1811 = v12;
                        nat_mul(a1810, a1811)?
                    };
                    nat_add(a1812, a1813)?
                }), Adt13::C1(f91(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1814 = v7;
                    let a1815 = 1u64;
                    nat_sub(a1814, a1815)
                };
                let v15: u64 = {
                    let a1816 = v14;
                    let a1817 = 2u64;
                    let a1818 = 0u64;
                    nat_quot(a1816, a1817, a1818)
                };
                let a1822 = {
                    let a1819 = v14;
                    let a1820 = 2u64;
                    let a1821 = 0u64;
                    nat_rem(a1819, a1820, a1821)
                };
                let a1823 = 0u64;
                if nat_eq(a1822, a1823) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1835 = {
                            let a1830 = {
                                let a1825 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1824, _, _, _) => {
                                        h1824
                                    }
                                };
                                let a1826 = v9;
                                nat_mul(a1825, a1826)?
                            };
                            let a1831 = {
                                let a1828 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1827, _, _) => {
                                        h1827
                                    }
                                };
                                let a1829 = v15;
                                nat_mul(a1828, a1829)?
                            };
                            nat_add(a1830, a1831)?
                        };
                        let a1836 = {
                            let a1833 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1832, _) => {
                                    h1832
                                }
                            };
                            let a1834 = v12;
                            nat_mul(a1833, a1834)?
                        };
                        nat_add(a1835, a1836)?
                    }), Adt12::C0(3u64, {
                        let a1848 = {
                            let a1843 = {
                                let a1838 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1837, _, _, _) => {
                                        h1837
                                    }
                                };
                                let a1839 = v11;
                                nat_mul(a1838, a1839)?
                            };
                            let a1844 = {
                                let a1841 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1840, _, _) => {
                                        h1840
                                    }
                                };
                                let a1842 = v15;
                                nat_mul(a1841, a1842)?
                            };
                            nat_add(a1843, a1844)?
                        };
                        let a1849 = {
                            let a1846 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1845, _) => {
                                    h1845
                                }
                            };
                            let a1847 = v12;
                            nat_mul(a1846, a1847)?
                        };
                        nat_add(a1848, a1849)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1852 = {
                            let a1850 = v7;
                            let a1851 = 1u64;
                            nat_add(a1850, a1851)?
                        };
                        let a1853 = v6;
                        if nat_eq(a1852, a1853) {
                            Adt13::C1(f91(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f32(v6)?;
    let v9: u64 = {
        let a1854 = v0;
        let a1855 = v8;
        let a1856 = 0u64;
        nat_rem(a1854, a1855, a1856)
    };
    let v10: u64 = {
        let a1857 = v0;
        let a1858 = v8;
        let a1859 = 0u64;
        nat_quot(a1857, a1858, a1859)
    };
    let a1860 = v9;
    let a1861 = 148u64;
    if nat_lt(a1860, a1861) {
        f168({
            let a1862 = v9;
            let a1863 = 37u64;
            let a1864 = 0u64;
            nat_rem(a1862, a1863, a1864)
        }, {
            let a1865 = v9;
            let a1866 = 37u64;
            let a1867 = 0u64;
            nat_quot(a1865, a1866, a1867)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f169({
            let a1868 = v9;
            let a1869 = 148u64;
            nat_sub(a1868, a1869)
        }, v10, v4, v5)
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f28(v2);
    let a1870 = v0;
    let a1871 = v3;
    if nat_lt(a1870, a1871) {
        let a1872 = v0;
        let a1873 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1872, a1873) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1874 = 4u64;
                let a1875 = v0;
                nat_mul(a1874, a1875)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1876 = v0;
            let a1877 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1876, a1877) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1880 = v0;
                let a1881 = {
                    let a1878 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1879 = 1u64;
                    nat_add(a1878, a1879)?
                };
                if nat_eq(a1880, a1881) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1882 = v0;
                    let a1883 = 9u64;
                    if nat_eq(a1882, a1883) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1884 = v0;
            let a1885 = v3;
            nat_sub(a1884, a1885)
        };
        let v5: u64 = f29(v2);
        let v6: u64 = {
            let a1886 = v4;
            let a1887 = v5;
            let a1888 = 0u64;
            nat_rem(a1886, a1887, a1888)
        };
        let v7: u64 = f91({
            let a1889 = v4;
            let a1890 = v5;
            let a1891 = 0u64;
            nat_quot(a1889, a1890, a1891)
        }, v1)?;
        if v2 {
            let a1892 = v6;
            let a1893 = 7u64;
            if nat_lt(a1892, a1893) {
                let a1894 = v6;
                let a1895 = 3u64;
                if nat_lt(a1894, a1895) {
                    let a1896 = v6;
                    let a1897 = 1u64;
                    if nat_lt(a1896, a1897) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1898 = v6;
                        let a1899 = 2u64;
                        if nat_lt(a1898, a1899) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1900 = v6;
                    let a1901 = 5u64;
                    if nat_lt(a1900, a1901) {
                        let a1902 = v6;
                        let a1903 = 4u64;
                        if nat_lt(a1902, a1903) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1904 = v6;
                        let a1905 = 6u64;
                        if nat_lt(a1904, a1905) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1906 = v6;
                let a1907 = 11u64;
                if nat_lt(a1906, a1907) {
                    let a1908 = v6;
                    let a1909 = 9u64;
                    if nat_lt(a1908, a1909) {
                        let a1910 = v6;
                        let a1911 = 8u64;
                        if nat_lt(a1910, a1911) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1912 = v6;
                        let a1913 = 10u64;
                        if nat_lt(a1912, a1913) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1914 = v6;
                    let a1915 = 13u64;
                    if nat_lt(a1914, a1915) {
                        let a1916 = v6;
                        let a1917 = 12u64;
                        if nat_lt(a1916, a1917) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1918 = v6;
                        let a1919 = 14u64;
                        if nat_lt(a1918, a1919) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1920 = v6;
            let a1921 = 1u64;
            if nat_lt(a1920, a1921) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1922 = v6;
                let a1923 = 2u64;
                if nat_lt(a1922, a1923) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: u64) -> R<Adt9> {
    let v7: u64 = match v1.clone() {
        Adt3::C0(h1924, _, _, _, _, _, _, _) => {
            h1924
        }
    };
    let v8: u64 = f24(v3, v7)?;
    let v9: u64 = {
        let a1925 = v0;
        let a1926 = 2u64;
        nat_sub(a1925, a1926)
    };
    let v10: u64 = {
        let a1927 = v9;
        let a1928 = v8;
        let a1929 = 0u64;
        nat_quot(a1927, a1928, a1929)
    };
    let v11: u64 = {
        let a1930 = v9;
        let a1931 = v8;
        let a1932 = 0u64;
        nat_rem(a1930, a1931, a1932)
    };
    let a1933 = v0;
    let a1934 = 0u64;
    if nat_eq(a1933, a1934) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a1935 = v0;
        let a1936 = 1u64;
        if nat_eq(a1935, a1936) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a1937 = v10;
            let a1938 = v5;
            if nat_eq(a1937, a1938) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a1941 = v11;
                let a1942 = {
                    let a1939 = 2u64;
                    let a1940 = v7;
                    nat_mul(a1939, a1940)?
                };
                if nat_lt(a1941, a1942) {
                    let v12: u64 = {
                        let a1943 = v11;
                        let a1944 = 2u64;
                        let a1945 = 0u64;
                        nat_quot(a1943, a1944, a1945)
                    };
                    let a1949 = {
                        let a1946 = v11;
                        let a1947 = 2u64;
                        let a1948 = 0u64;
                        nat_rem(a1946, a1947, a1948)
                    };
                    let a1950 = 0u64;
                    if nat_eq(a1949, a1950) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a1951 = v12;
                            let a1952 = 0u64;
                            if nat_eq(a1951, a1952) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a1958 = {
                                let a1954 = v10;
                                let a1955 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h1953, _, _) => {
                                        h1953
                                    }
                                };
                                nat_mul(a1954, a1955)?
                            };
                            let a1959 = {
                                let a1956 = 4u64;
                                let a1957 = v12;
                                nat_mul(a1956, a1957)?
                            };
                            nat_add(a1958, a1959)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a1960 = v6;
                            let a1961 = 2u64;
                            if nat_eq(a1960, a1961) {
                                let a1962 = v12;
                                let a1963 = 0u64;
                                if nat_eq(a1962, a1963) {
                                    Adt12::C2(4286578688u64)
                                } else {
                                    Adt12::C1(0u64)
                                }
                            } else {
                                Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a1969 = {
                                        let a1965 = v10;
                                        let a1966 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1964, _, _) => {
                                                h1964
                                            }
                                        };
                                        nat_mul(a1965, a1966)?
                                    };
                                    let a1970 = {
                                        let a1967 = 4u64;
                                        let a1968 = v12;
                                        nat_mul(a1967, a1968)?
                                    };
                                    nat_add(a1969, a1970)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a1973 = v11;
                    let a1974 = {
                        let a1971 = 6u64;
                        let a1972 = v7;
                        nat_mul(a1971, a1972)?
                    };
                    if nat_lt(a1973, a1974) {
                        let v13: u64 = {
                            let a1977 = v11;
                            let a1978 = {
                                let a1975 = 2u64;
                                let a1976 = v7;
                                nat_mul(a1975, a1976)?
                            };
                            nat_sub(a1977, a1978)
                        };
                        let v14: u64 = {
                            let a1979 = v13;
                            let a1980 = 4u64;
                            let a1981 = 0u64;
                            nat_quot(a1979, a1980, a1981)
                        };
                        let a1985 = {
                            let a1982 = v13;
                            let a1983 = 4u64;
                            let a1984 = 0u64;
                            nat_rem(a1982, a1983, a1984)
                        };
                        let a1986 = 2u64;
                        if nat_lt(a1985, a1986) {
                            let a1990 = {
                                let a1987 = v13;
                                let a1988 = 4u64;
                                let a1989 = 0u64;
                                nat_rem(a1987, a1988, a1989)
                            };
                            let a1991 = 1u64;
                            if nat_lt(a1990, a1991) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a1997 = {
                                        let a1993 = v10;
                                        let a1994 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1992, _, _) => {
                                                h1992
                                            }
                                        };
                                        nat_mul(a1993, a1994)?
                                    };
                                    let a1998 = {
                                        let a1995 = 4u64;
                                        let a1996 = v14;
                                        nat_mul(a1995, a1996)?
                                    };
                                    nat_add(a1997, a1998)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2004 = {
                                            let a2000 = v10;
                                            let a2001 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h1999, _, _) => {
                                                    h1999
                                                }
                                            };
                                            nat_mul(a2000, a2001)?
                                        };
                                        let a2005 = {
                                            let a2002 = 4u64;
                                            let a2003 = v14;
                                            nat_mul(a2002, a2003)?
                                        };
                                        nat_add(a2004, a2005)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2009 = {
                                let a2006 = v13;
                                let a2007 = 4u64;
                                let a2008 = 0u64;
                                nat_rem(a2006, a2007, a2008)
                            };
                            let a2010 = 3u64;
                            if nat_lt(a2009, a2010) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2016 = {
                                            let a2012 = v10;
                                            let a2013 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2011, _, _) => {
                                                    h2011
                                                }
                                            };
                                            nat_mul(a2012, a2013)?
                                        };
                                        let a2017 = {
                                            let a2014 = 4u64;
                                            let a2015 = v14;
                                            nat_mul(a2014, a2015)?
                                        };
                                        nat_add(a2016, a2017)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2018 = v14;
                                    let a2019 = 0u64;
                                    if nat_eq(a2018, a2019) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2024 = v11;
                        let a2025 = {
                            let a2022 = {
                                let a2020 = 6u64;
                                let a2021 = v7;
                                nat_mul(a2020, a2021)?
                            };
                            let a2023 = 2u64;
                            nat_add(a2022, a2023)?
                        };
                        if nat_lt(a2024, a2025) {
                            let a2028 = v11;
                            let a2029 = {
                                let a2026 = 6u64;
                                let a2027 = v7;
                                nat_mul(a2026, a2027)?
                            };
                            if nat_eq(a2028, a2029) {
                                if v3 {
                                    Ok(Adt9::C7(Adt11::C0(4u64), Adt12::C2(4607182418800017408u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                }
                            } else {
                                if v3 {
                                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C7(13u64, 0u64, 0u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                }
                            }
                        } else {
                            if v3 {
                                let v15: u64 = {
                                    let a2034 = v11;
                                    let a2035 = {
                                        let a2032 = {
                                            let a2030 = 6u64;
                                            let a2031 = v7;
                                            nat_mul(a2030, a2031)?
                                        };
                                        let a2033 = 2u64;
                                        nat_add(a2032, a2033)?
                                    };
                                    nat_sub(a2034, a2035)
                                };
                                let v16: u64 = {
                                    let a2036 = v15;
                                    let a2037 = 3u64;
                                    let a2038 = 0u64;
                                    nat_quot(a2036, a2037, a2038)
                                };
                                let a2042 = {
                                    let a2039 = v15;
                                    let a2040 = 3u64;
                                    let a2041 = 0u64;
                                    nat_rem(a2039, a2040, a2041)
                                };
                                let a2043 = 1u64;
                                if nat_lt(a2042, a2043) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2049 = {
                                            let a2045 = v10;
                                            let a2046 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2044, _, _) => {
                                                    h2044
                                                }
                                            };
                                            nat_mul(a2045, a2046)?
                                        };
                                        let a2050 = {
                                            let a2047 = 4u64;
                                            let a2048 = v16;
                                            nat_mul(a2047, a2048)?
                                        };
                                        nat_add(a2049, a2050)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2054 = {
                                        let a2051 = v15;
                                        let a2052 = 3u64;
                                        let a2053 = 0u64;
                                        nat_rem(a2051, a2052, a2053)
                                    };
                                    let a2055 = 2u64;
                                    if nat_lt(a2054, a2055) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2060 = {
                                                let a2056 = v10;
                                                let a2057 = v4;
                                                nat_mul(a2056, a2057)?
                                            };
                                            let a2061 = {
                                                let a2058 = 4u64;
                                                let a2059 = v16;
                                                nat_mul(a2058, a2059)?
                                            };
                                            nat_add(a2060, a2061)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2067 = {
                                                let a2063 = v10;
                                                let a2064 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2062, _, _) => {
                                                        h2062
                                                    }
                                                };
                                                nat_mul(a2063, a2064)?
                                            };
                                            let a2068 = {
                                                let a2065 = 4u64;
                                                let a2066 = v16;
                                                nat_mul(a2065, a2066)?
                                            };
                                            nat_add(a2067, a2068)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2073 = v11;
                                let a2074 = {
                                    let a2071 = {
                                        let a2069 = 11u64;
                                        let a2070 = v7;
                                        nat_mul(a2069, a2070)?
                                    };
                                    let a2072 = 2u64;
                                    nat_add(a2071, a2072)?
                                };
                                if nat_lt(a2073, a2074) {
                                    let v17: u64 = {
                                        let a2079 = v11;
                                        let a2080 = {
                                            let a2077 = {
                                                let a2075 = 6u64;
                                                let a2076 = v7;
                                                nat_mul(a2075, a2076)?
                                            };
                                            let a2078 = 2u64;
                                            nat_add(a2077, a2078)?
                                        };
                                        nat_sub(a2079, a2080)
                                    };
                                    let v18: u64 = {
                                        let a2081 = v17;
                                        let a2082 = 5u64;
                                        let a2083 = 0u64;
                                        nat_quot(a2081, a2082, a2083)
                                    };
                                    let a2087 = {
                                        let a2084 = v17;
                                        let a2085 = 5u64;
                                        let a2086 = 0u64;
                                        nat_rem(a2084, a2085, a2086)
                                    };
                                    let a2088 = 2u64;
                                    if nat_lt(a2087, a2088) {
                                        let a2092 = {
                                            let a2089 = v17;
                                            let a2090 = 5u64;
                                            let a2091 = 0u64;
                                            nat_rem(a2089, a2090, a2091)
                                        };
                                        let a2093 = 1u64;
                                        if nat_lt(a2092, a2093) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2099 = {
                                                    let a2095 = v10;
                                                    let a2096 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2094, _, _) => {
                                                            h2094
                                                        }
                                                    };
                                                    nat_mul(a2095, a2096)?
                                                };
                                                let a2100 = {
                                                    let a2097 = 4u64;
                                                    let a2098 = v18;
                                                    nat_mul(a2097, a2098)?
                                                };
                                                nat_add(a2099, a2100)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2104 = {
                                            let a2101 = v17;
                                            let a2102 = 5u64;
                                            let a2103 = 0u64;
                                            nat_rem(a2101, a2102, a2103)
                                        };
                                        let a2105 = 3u64;
                                        if nat_lt(a2104, a2105) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2110 = {
                                                    let a2106 = v10;
                                                    let a2107 = v4;
                                                    nat_mul(a2106, a2107)?
                                                };
                                                let a2111 = {
                                                    let a2108 = 4u64;
                                                    let a2109 = v18;
                                                    nat_mul(a2108, a2109)?
                                                };
                                                nat_add(a2110, a2111)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2115 = {
                                                let a2112 = v17;
                                                let a2113 = 5u64;
                                                let a2114 = 0u64;
                                                nat_rem(a2112, a2113, a2114)
                                            };
                                            let a2116 = 4u64;
                                            if nat_lt(a2115, a2116) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2117 = v18;
                                                    let a2118 = 0u64;
                                                    if nat_eq(a2117, a2118) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2123 = v11;
                                    let a2124 = {
                                        let a2121 = {
                                            let a2119 = 11u64;
                                            let a2120 = v7;
                                            nat_mul(a2119, a2120)?
                                        };
                                        let a2122 = 2u64;
                                        nat_add(a2121, a2122)?
                                    };
                                    if nat_eq(a2123, a2124) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2125 = v10;
                                            let a2126 = 0u64;
                                            if nat_eq(a2125, a2126) {
                                                Adt12::C2(0u64)
                                            } else {
                                                Adt12::C1(5u64)
                                            }
                                        }, Adt12::C1(4u64), Adt13::C0(5u64), 4u64, 4u64))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2129 = match v3.clone() {
            Adt6::C0(h2127, _, _, _) => {
                h2127
            }
        };
        let a2130 = match v4.clone() {
            Adt6::C0(h2128, _, _, _) => {
                h2128
            }
        };
        nat_mul(a2129, a2130)?
    };
    let v6: u64 = {
        let a2131 = v5;
        let a2132 = 2u64;
        nat_add(a2131, a2132)?
    };
    let v7: u64 = {
        let a2133 = v0;
        let a2134 = v6;
        let a2135 = 0u64;
        nat_quot(a2133, a2134, a2135)
    };
    let v8: u64 = {
        let a2136 = v0;
        let a2137 = v6;
        let a2138 = 0u64;
        nat_rem(a2136, a2137, a2138)
    };
    let a2139 = v8;
    let a2140 = 0u64;
    if nat_eq(a2139, a2140) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2143 = {
            let a2141 = v8;
            let a2142 = 1u64;
            nat_add(a2141, a2142)?
        };
        let a2144 = v6;
        if nat_eq(a2143, a2144) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f91(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2145 = v8;
                let a2146 = 1u64;
                nat_sub(a2145, a2146)
            };
            let m2152 = f170(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2147, _, _, _, _, _, _) => {
                    h2147
                }
            }, f58(v7, v2.clone(), 0u64), {
                let a2149 = v9;
                let a2150 = match v3.clone() {
                    Adt6::C0(h2148, _, _, _) => {
                        h2148
                    }
                };
                let a2151 = 0u64;
                nat_rem(a2149, a2150, a2151)
            })?;
            match m2152 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2158 = f170(v4, match v1.clone() {
                        Adt3::C0(_, _, h2153, _, _, _, _, _) => {
                            h2153
                        }
                    }, f58(v7, v2.clone(), 1u64), {
                        let a2155 = v9;
                        let a2156 = match v3 {
                            Adt6::C0(h2154, _, _, _) => {
                                h2154
                            }
                        };
                        let a2157 = 0u64;
                        nat_quot(a2155, a2156, a2157)
                    })?;
                    match m2158 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f57(v1, {
                                let a2161 = {
                                    let a2159 = f58(v7, v2.clone(), 2u64);
                                    let a2160 = v5;
                                    nat_mul(a2159, a2160)?
                                };
                                let a2162 = v9;
                                nat_add(a2161, a2162)?
                            }, v10, v11, f58(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2163, _, _, _, _, _, _, _) => {
            h2163
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2164, _, _, _, _, _, _, _) => {
            h2164
        }
    };
    let v6: u64 = f171(v4)?;
    let v7: u64 = {
        let a2165 = v0;
        let a2166 = v6;
        let a2167 = 0u64;
        nat_quot(a2165, a2166, a2167)
    };
    let v8: u64 = {
        let a2168 = v7;
        let a2169 = v5;
        let a2170 = 0u64;
        nat_quot(a2168, a2169, a2170)
    };
    let v9: u64 = {
        let a2171 = v7;
        let a2172 = v5;
        let a2173 = 0u64;
        nat_rem(a2171, a2172, a2173)
    };
    let v10: u64 = {
        let a2174 = v3;
        let a2175 = v4;
        let a2176 = 0u64;
        nat_rem(a2174, a2175, a2176)
    };
    let v11: u64 = {
        let a2181 = 4u64;
        let a2182 = {
            let a2179 = v8;
            let a2180 = {
                let a2177 = v4;
                let a2178 = v4;
                nat_mul(a2177, a2178)?
            };
            nat_mul(a2179, a2180)?
        };
        nat_mul(a2181, a2182)?
    };
    let v12: u64 = {
        let a2187 = 4u64;
        let a2188 = {
            let a2185 = v8;
            let a2186 = {
                let a2183 = v4;
                let a2184 = v5;
                nat_mul(a2183, a2184)?
            };
            nat_mul(a2185, a2186)?
        };
        nat_mul(a2187, a2188)?
    };
    let v13: u64 = {
        let a2192 = {
            let a2189 = v0;
            let a2190 = v6;
            let a2191 = 0u64;
            nat_rem(a2189, a2190, a2191)
        };
        let a2193 = f171(v10)?;
        nat_sub(a2192, a2193)
    };
    let a2196 = v13;
    let a2197 = {
        let a2194 = v10;
        let a2195 = 2u64;
        nat_mul(a2194, a2195)?
    };
    if nat_lt(a2196, a2197) {
        let v14: u64 = {
            let a2198 = v13;
            let a2199 = 2u64;
            let a2200 = 0u64;
            nat_quot(a2198, a2199, a2200)
        };
        let a2204 = {
            let a2201 = v13;
            let a2202 = 2u64;
            let a2203 = 0u64;
            nat_rem(a2201, a2202, a2203)
        };
        let a2205 = 0u64;
        if nat_eq(a2204, a2205) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2212 = v11;
                let a2213 = {
                    let a2210 = 4u64;
                    let a2211 = {
                        let a2208 = {
                            let a2206 = v10;
                            let a2207 = v4;
                            nat_mul(a2206, a2207)?
                        };
                        let a2209 = v14;
                        nat_add(a2208, a2209)?
                    };
                    nat_mul(a2210, a2211)?
                };
                nat_add(a2212, a2213)?
            }), Adt12::C3({
                let a2220 = v12;
                let a2221 = {
                    let a2218 = 4u64;
                    let a2219 = {
                        let a2216 = {
                            let a2214 = v14;
                            let a2215 = v5;
                            nat_mul(a2214, a2215)?
                        };
                        let a2217 = v9;
                        nat_add(a2216, a2217)?
                    };
                    nat_mul(a2218, a2219)?
                };
                nat_add(a2220, a2221)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2222 = v14;
                let a2223 = 0u64;
                if nat_eq(a2222, a2223) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2226 = v13;
        let a2227 = {
            let a2224 = v10;
            let a2225 = 2u64;
            nat_mul(a2224, a2225)?
        };
        if nat_eq(a2226, a2227) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2234 = v12;
                let a2235 = {
                    let a2232 = 4u64;
                    let a2233 = {
                        let a2230 = {
                            let a2228 = v10;
                            let a2229 = v5;
                            nat_mul(a2228, a2229)?
                        };
                        let a2231 = v9;
                        nat_add(a2230, a2231)?
                    };
                    nat_mul(a2232, a2233)?
                };
                nat_add(a2234, a2235)?
            }), {
                let a2236 = v10;
                let a2237 = 0u64;
                if nat_eq(a2236, a2237) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2242 = v13;
            let a2243 = {
                let a2240 = {
                    let a2238 = v10;
                    let a2239 = 2u64;
                    nat_mul(a2238, a2239)?
                };
                let a2241 = 1u64;
                nat_add(a2240, a2241)?
            };
            if nat_eq(a2242, a2243) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2250 = v11;
                    let a2251 = {
                        let a2248 = 4u64;
                        let a2249 = {
                            let a2246 = {
                                let a2244 = v10;
                                let a2245 = v4;
                                nat_mul(a2244, a2245)?
                            };
                            let a2247 = v10;
                            nat_add(a2246, a2247)?
                        };
                        nat_mul(a2248, a2249)?
                    };
                    nat_add(a2250, a2251)?
                }), Adt13::C1({
                    let a2258 = v12;
                    let a2259 = {
                        let a2256 = 4u64;
                        let a2257 = {
                            let a2254 = {
                                let a2252 = v10;
                                let a2253 = v5;
                                nat_mul(a2252, a2253)?
                            };
                            let a2255 = v9;
                            nat_add(a2254, a2255)?
                        };
                        nat_mul(a2256, a2257)?
                    };
                    nat_add(a2258, a2259)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2262 = {
                        let a2260 = v10;
                        let a2261 = 1u64;
                        nat_add(a2260, a2261)?
                    };
                    let a2263 = v4;
                    if nat_eq(a2262, a2263) {
                        0u64
                    } else {
                        let a2264 = v10;
                        let a2265 = 1u64;
                        nat_add(a2264, a2265)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f74(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2266 = v0;
    let a2267 = v1;
    if nat_lt(a2266, a2267) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f91(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2268 = v0;
            let a2269 = v1;
            nat_sub(a2268, a2269)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f91(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2276 = {
            let a2274 = 3u64;
            let a2275 = {
                let a2272 = match v4.clone() {
                    Adt6::C0(h2270, _, _, _) => {
                        h2270
                    }
                };
                let a2273 = match v5.clone() {
                    Adt6::C0(h2271, _, _, _) => {
                        h2271
                    }
                };
                nat_mul(a2272, a2273)?
            };
            nat_mul(a2274, a2275)?
        };
        let a2277 = 2u64;
        nat_add(a2276, a2277)?
    };
    let v8: u64 = {
        let a2278 = v0;
        let a2279 = v7;
        let a2280 = 0u64;
        nat_quot(a2278, a2279, a2280)
    };
    let v9: u64 = {
        let a2281 = v0;
        let a2282 = v7;
        let a2283 = 0u64;
        nat_rem(a2281, a2282, a2283)
    };
    let a2284 = v9;
    let a2285 = 0u64;
    if nat_eq(a2284, a2285) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2288 = {
            let a2286 = v9;
            let a2287 = 1u64;
            nat_add(a2286, a2287)?
        };
        let a2289 = v7;
        if nat_eq(a2288, a2289) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f91(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2290 = v9;
                let a2291 = 1u64;
                nat_sub(a2290, a2291)
            };
            let v11: u64 = {
                let a2292 = v10;
                let a2293 = 3u64;
                let a2294 = 0u64;
                nat_quot(a2292, a2293, a2294)
            };
            let v12: u64 = {
                let a2296 = v11;
                let a2297 = match v4.clone() {
                    Adt6::C0(h2295, _, _, _) => {
                        h2295
                    }
                };
                let a2298 = 0u64;
                nat_rem(a2296, a2297, a2298)
            };
            let v13: u64 = {
                let a2300 = v11;
                let a2301 = match v4.clone() {
                    Adt6::C0(h2299, _, _, _) => {
                        h2299
                    }
                };
                let a2302 = 0u64;
                nat_quot(a2300, a2301, a2302)
            };
            let v14: u64 = {
                let a2303 = v10;
                let a2304 = 3u64;
                let a2305 = 0u64;
                nat_rem(a2303, a2304, a2305)
            };
            let v15: u64 = f58(v8, v3.clone(), 2u64);
            let m2307 = f60(v4, match v2.clone() {
                Adt3::C0(h2306, _, _, _, _, _, _, _) => {
                    h2306
                }
            }, f58(v8, v3.clone(), 0u64), v12)?;
            match m2307 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2309 = f60(v5, match v2.clone() {
                        Adt3::C0(_, h2308, _, _, _, _, _, _) => {
                            h2308
                        }
                    }, f58(v8, v3.clone(), 1u64), v13)?;
                    match m2309 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2310 = v14;
                            let a2311 = 0u64;
                            if nat_eq(a2310, a2311) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f57(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f172(v6), 4u64))
                            } else {
                                let a2312 = v14;
                                let a2313 = 1u64;
                                if nat_eq(a2312, a2313) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f57(v2, v16, v17, v15, f58(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f173(match v1.clone() {
        Adt3::C0(h2314, _, _, _, _, _, _, _) => {
            h2314
        }
    })?;
    let v5: u64 = {
        let a2315 = v0;
        let a2316 = v4;
        let a2317 = 0u64;
        nat_quot(a2315, a2316, a2317)
    };
    let v6: u64 = {
        let a2318 = v0;
        let a2319 = v4;
        let a2320 = 0u64;
        nat_rem(a2318, a2319, a2320)
    };
    let a2321 = v6;
    let a2322 = 0u64;
    if nat_eq(a2321, a2322) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2323 = v6;
        let a2324 = 1u64;
        if nat_eq(a2323, a2324) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2327 = {
                let a2325 = v6;
                let a2326 = 1u64;
                nat_add(a2325, a2326)?
            };
            let a2328 = v4;
            if nat_eq(a2327, a2328) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f91(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2329 = v6;
                    let a2330 = 2u64;
                    nat_sub(a2329, a2330)
                };
                let v8: u64 = {
                    let a2331 = v7;
                    let a2332 = 3u64;
                    let a2333 = 0u64;
                    nat_quot(a2331, a2332, a2333)
                };
                let v9: u64 = {
                    let a2334 = v7;
                    let a2335 = 3u64;
                    let a2336 = 0u64;
                    nat_rem(a2334, a2335, a2336)
                };
                let v10: u64 = f91({
                    let a2340 = {
                        let a2338 = v5;
                        let a2339 = match v1.clone() {
                            Adt3::C0(h2337, _, _, _, _, _, _, _) => {
                                h2337
                            }
                        };
                        nat_mul(a2338, a2339)?
                    };
                    let a2341 = v8;
                    nat_add(a2340, a2341)?
                }, v1)?;
                let a2342 = v9;
                let a2343 = 0u64;
                if nat_eq(a2342, a2343) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2344 = v9;
                    let a2345 = 1u64;
                    if nat_eq(a2344, a2345) {
                        let a2346 = v3;
                        let a2347 = 2u64;
                        if nat_eq(a2346, a2347) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2348 = v3;
                        let a2349 = 2u64;
                        if if nat_eq(a2348, a2349) {
                            true
                        } else {
                            let a2350 = v3;
                            let a2351 = 3u64;
                            nat_eq(a2350, a2351)
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
        let a2353 = 2u64;
        let a2354 = match v2.clone() {
            Adt3::C0(_, h2352, _, _, _, _, _, _) => {
                h2352
            }
        };
        nat_mul(a2353, a2354)?
    };
    let a2355 = v1;
    let a2356 = v6;
    if nat_lt(a2355, a2356) {
        let a2360 = {
            let a2357 = v1;
            let a2358 = 2u64;
            let a2359 = 0u64;
            nat_rem(a2357, a2358, a2359)
        };
        let a2361 = 0u64;
        if nat_eq(a2360, a2361) {
            Ok(Adt9::C4(19u64, 4u64, f91({
                let a2362 = v1;
                let a2363 = 2u64;
                let a2364 = 0u64;
                nat_quot(a2362, a2363, a2364)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2365 = v5;
            let a2366 = 2147483648u64;
            if if nat_lt(a2365, a2366) {
                let a2368 = v5;
                let a2369 = match v3 {
                    Adt3::C0(_, h2367, _, _, _, _, _, _) => {
                        h2367
                    }
                };
                nat_lt(a2368, a2369)
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
            let a2372 = match v3.clone() {
                Adt3::C0(h2370, _, _, _, _, _, _, _) => {
                    h2370
                }
            };
            let a2373 = match v3.clone() {
                Adt3::C0(_, h2371, _, _, _, _, _, _) => {
                    h2371
                }
            };
            nat_mul(a2372, a2373)?
        };
        let a2376 = {
            let a2374 = v1;
            let a2375 = v6;
            nat_sub(a2374, a2375)
        };
        let a2377 = v7;
        if nat_lt(a2376, a2377) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f91({
                let a2378 = v1;
                let a2379 = v6;
                nat_sub(a2378, a2379)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2382 = {
                    let a2380 = v1;
                    let a2381 = v6;
                    nat_sub(a2380, a2381)
                };
                let a2383 = v7;
                nat_sub(a2382, a2383)
            };
            let v9: u64 = {
                let a2385 = 1u64;
                let a2386 = f174(v0, match v2.clone() {
                    Adt3::C0(h2384, _, _, _, _, _, _, _) => {
                        h2384
                    }
                })?;
                nat_add(a2385, a2386)?
            };
            let v10: u64 = {
                let a2387 = v8;
                let a2388 = v9;
                let a2389 = 0u64;
                nat_quot(a2387, a2388, a2389)
            };
            let v11: u64 = {
                let a2390 = v8;
                let a2391 = v9;
                let a2392 = 0u64;
                nat_rem(a2390, a2391, a2392)
            };
            let a2393 = v11;
            let a2394 = 0u64;
            if nat_eq(a2393, a2394) {
                Ok(Adt9::C4(19u64, 4u64, f91(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f175(v0) {
                    let a2398 = {
                        let a2395 = v11;
                        let a2396 = 2u64;
                        let a2397 = 0u64;
                        nat_rem(a2395, a2396, a2397)
                    };
                    let a2399 = 1u64;
                    if nat_eq(a2398, a2399) {
                        Ok(Adt9::C4(6u64, 0u64, f91({
                            let a2408 = {
                                let a2401 = v10;
                                let a2402 = match v2.clone() {
                                    Adt3::C0(h2400, _, _, _, _, _, _, _) => {
                                        h2400
                                    }
                                };
                                nat_mul(a2401, a2402)?
                            };
                            let a2409 = {
                                let a2405 = {
                                    let a2403 = v11;
                                    let a2404 = 1u64;
                                    nat_sub(a2403, a2404)
                                };
                                let a2406 = 2u64;
                                let a2407 = 0u64;
                                nat_quot(a2405, a2406, a2407)
                            };
                            nat_add(a2408, a2409)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f91({
                            let a2418 = {
                                let a2411 = v5;
                                let a2412 = match v2.clone() {
                                    Adt3::C0(h2410, _, _, _, _, _, _, _) => {
                                        h2410
                                    }
                                };
                                nat_mul(a2411, a2412)?
                            };
                            let a2419 = {
                                let a2415 = {
                                    let a2413 = v11;
                                    let a2414 = 1u64;
                                    nat_sub(a2413, a2414)
                                };
                                let a2416 = 2u64;
                                let a2417 = 0u64;
                                nat_quot(a2415, a2416, a2417)
                            };
                            nat_add(a2418, a2419)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f91({
                            let a2428 = {
                                let a2421 = v5;
                                let a2422 = match v2 {
                                    Adt3::C0(h2420, _, _, _, _, _, _, _) => {
                                        h2420
                                    }
                                };
                                nat_mul(a2421, a2422)?
                            };
                            let a2429 = {
                                let a2425 = {
                                    let a2423 = v11;
                                    let a2424 = 1u64;
                                    nat_sub(a2423, a2424)
                                };
                                let a2426 = 2u64;
                                let a2427 = 0u64;
                                nat_quot(a2425, a2426, a2427)
                            };
                            nat_add(a2428, a2429)?
                        }, v3)?))
                    }
                } else {
                    f176(v0, v10, {
                        let a2430 = v11;
                        let a2431 = 1u64;
                        nat_sub(a2430, a2431)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2432 = v1;
    let a2433 = f177(v2.clone(), v4.clone())?;
    if nat_lt(a2432, a2433) {
        let a2436 = v1;
        let a2437 = {
            let a2434 = 2u64;
            let a2435 = f178(v4.clone())?;
            nat_mul(a2434, a2435)?
        };
        if nat_lt(a2436, a2437) {
            let a2441 = {
                let a2438 = v1;
                let a2439 = 2u64;
                let a2440 = 0u64;
                nat_rem(a2438, a2439, a2440)
            };
            let a2442 = 0u64;
            if nat_eq(a2441, a2442) {
                f179({
                    let a2443 = v1;
                    let a2444 = 2u64;
                    let a2445 = 0u64;
                    nat_quot(a2443, a2444, a2445)
                }, v4)
            } else {
                if f180(match v2 {
                    Adt3::C0(_, h2446, _, _, _, _, _, _) => {
                        h2446
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2449 = v1;
                let a2450 = {
                    let a2447 = 2u64;
                    let a2448 = f178(v4.clone())?;
                    nat_mul(a2447, a2448)?
                };
                nat_sub(a2449, a2450)
            };
            let v7: u64 = {
                let a2454 = v6;
                let a2455 = {
                    let a2452 = 1u64;
                    let a2453 = match v2.clone() {
                        Adt3::C0(h2451, _, _, _, _, _, _, _) => {
                            h2451
                        }
                    };
                    nat_add(a2452, a2453)?
                };
                let a2456 = 0u64;
                nat_quot(a2454, a2455, a2456)
            };
            let v8: u64 = {
                let a2460 = v6;
                let a2461 = {
                    let a2458 = 1u64;
                    let a2459 = match v2.clone() {
                        Adt3::C0(h2457, _, _, _, _, _, _, _) => {
                            h2457
                        }
                    };
                    nat_add(a2458, a2459)?
                };
                let a2462 = 0u64;
                nat_rem(a2460, a2461, a2462)
            };
            let a2463 = v8;
            let a2464 = 0u64;
            if nat_eq(a2463, a2464) {
                f179(v7, v4)
            } else {
                f181(v0, v7, {
                    let a2465 = v8;
                    let a2466 = 1u64;
                    nat_sub(a2465, a2466)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2469 = match v4.clone() {
            Adt3::C0(h2467, _, _, _, _, _, _, _) => {
                h2467
            }
        };
        let a2470 = match v4.clone() {
            Adt3::C0(_, h2468, _, _, _, _, _, _) => {
                h2468
            }
        };
        nat_mul(a2469, a2470)?
    };
    let v7: u64 = {
        let a2471 = 2u64;
        let a2472 = v6;
        nat_mul(a2471, a2472)?
    };
    let v8: u64 = f182(match v3.clone() {
        Adt3::C0(_, h2473, _, _, _, _, _, _) => {
            h2473
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2474, _, _, _, _, _) => {
            h2474
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2475, _, _, _, _) => {
            h2475
        }
    });
    let a2476 = v0;
    let a2477 = v7;
    if nat_lt(a2476, a2477) {
        let a2481 = {
            let a2478 = v0;
            let a2479 = 2u64;
            let a2480 = 0u64;
            nat_rem(a2478, a2479, a2480)
        };
        let a2482 = 0u64;
        if nat_eq(a2481, a2482) {
            Ok(Adt9::C4(19u64, 5u64, f91({
                let a2483 = v0;
                let a2484 = 2u64;
                let a2485 = 0u64;
                nat_quot(a2483, a2484, a2485)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f183(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2486 = v0;
            let a2487 = v7;
            nat_sub(a2486, a2487)
        };
        let v10: u64 = {
            let a2491 = v9;
            let a2492 = {
                let a2489 = 1u64;
                let a2490 = match v1.clone() {
                    Adt3::C0(h2488, _, _, _, _, _, _, _) => {
                        h2488
                    }
                };
                nat_add(a2489, a2490)?
            };
            let a2493 = 0u64;
            nat_quot(a2491, a2492, a2493)
        };
        let v11: u64 = {
            let a2497 = v9;
            let a2498 = {
                let a2495 = 1u64;
                let a2496 = match v1.clone() {
                    Adt3::C0(h2494, _, _, _, _, _, _, _) => {
                        h2494
                    }
                };
                nat_add(a2495, a2496)?
            };
            let a2499 = 0u64;
            nat_rem(a2497, a2498, a2499)
        };
        let a2500 = v11;
        let a2501 = 0u64;
        if nat_eq(a2500, a2501) {
            Ok(Adt9::C4(19u64, 5u64, f91({
                let a2502 = v10;
                let a2503 = v6;
                let a2504 = 0u64;
                nat_rem(a2502, a2503, a2504)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f183(v5, v8) {
                let v12: u64 = {
                    let a2510 = {
                        let a2506 = v10;
                        let a2507 = match v1.clone() {
                            Adt3::C0(h2505, _, _, _, _, _, _, _) => {
                                h2505
                            }
                        };
                        nat_mul(a2506, a2507)?
                    };
                    let a2511 = {
                        let a2508 = v11;
                        let a2509 = 1u64;
                        nat_sub(a2508, a2509)
                    };
                    nat_add(a2510, a2511)?
                };
                let a2512 = v5;
                let a2513 = 4294967295u64;
                if nat_eq(a2512, a2513) {
                    Ok(Adt9::C1(false, f91(v12, v1)?, f91(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f91(v12, v1)?, 4u64, f91({
                        let a2519 = {
                            let a2515 = v5;
                            let a2516 = match v3.clone() {
                                Adt3::C0(h2514, _, _, _, _, _, _, _) => {
                                    h2514
                                }
                            };
                            nat_mul(a2515, a2516)?
                        };
                        let a2520 = {
                            let a2517 = v11;
                            let a2518 = 1u64;
                            nat_sub(a2517, a2518)
                        };
                        nat_add(a2519, a2520)?
                    }, v3)?, 3u64, f91(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f184(v3.clone())?;
    let v6: u64 = {
        let a2521 = v0;
        let a2522 = v5;
        let a2523 = 0u64;
        nat_quot(a2521, a2522, a2523)
    };
    let v7: u64 = {
        let a2524 = v0;
        let a2525 = v5;
        let a2526 = 0u64;
        nat_rem(a2524, a2525, a2526)
    };
    let v8: u64 = f185(v3.clone())?;
    let a2527 = v7;
    let a2528 = 0u64;
    if nat_eq(a2527, a2528) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2529, _) => {
                h2529
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2533 = v5;
        let a2534 = {
            let a2531 = v7;
            let a2532 = if match v3.clone() {
                Adt7::C0(_, _, _, h2530, _) => {
                    h2530
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2531, a2532)?
        };
        if nat_le(a2533, a2534) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2535, _) => {
                    h2535
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f91(v6, v2)?))
            } else {
                let a2538 = {
                    let a2536 = v7;
                    let a2537 = 2u64;
                    nat_add(a2536, a2537)?
                };
                let a2539 = v5;
                if nat_eq(a2538, a2539) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2540, _, _) => {
                            h2540
                        }
                    } {
                        let a2547 = {
                            let m2543 = match v3.clone() {
                                Adt7::C0(h2542, _, _, _, _) => {
                                    h2542
                                }
                            };
                            match m2543 {
                                Adt6::C0(h2541, _, _, _) => {
                                    h2541
                                }
                            }
                        };
                        let a2548 = {
                            let m2546 = match v3 {
                                Adt7::C0(_, h2545, _, _, _) => {
                                    h2545
                                }
                            };
                            match m2546 {
                                Adt6::C0(h2544, _, _, _) => {
                                    h2544
                                }
                            }
                        };
                        nat_mul(a2547, a2548)?
                    } else {
                        f186(match v3 {
                            Adt7::C0(h2549, _, _, _, _) => {
                                h2549
                            }
                        }, match v1 {
                            Adt3::C0(h2550, _, _, _, _, _, _, _) => {
                                h2550
                            }
                        }, f58(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2551 = v9;
                        let a2552 = 0u64;
                        if nat_eq(a2551, a2552) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f91(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2555 = {
                    let a2553 = v7;
                    let a2554 = 1u64;
                    nat_sub(a2553, a2554)
                };
                let a2556 = v8;
                let a2557 = 0u64;
                nat_rem(a2555, a2556, a2557)
            };
            let m2563 = f187(v3.clone(), v1, v2, v6, {
                let a2560 = {
                    let a2558 = v7;
                    let a2559 = 1u64;
                    nat_sub(a2558, a2559)
                };
                let a2561 = v8;
                let a2562 = 0u64;
                nat_quot(a2560, a2561, a2562)
            })?;
            match m2563 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2565 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2564) => {
                            h2564
                        }
                    };
                    let a2566 = 1u64;
                    if if nat_eq(a2565, a2566) {
                        let a2567 = v10;
                        let a2568 = 0u64;
                        nat_eq(a2567, a2568)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2569, _) => {
                                h2569
                            }
                        } {
                            let a2572 = {
                                let a2570 = v10;
                                let a2571 = 1u64;
                                nat_add(a2570, a2571)?
                            };
                            let a2573 = v8;
                            if nat_eq(a2572, a2573) {
                                let a2574 = v4;
                                let a2575 = 2u64;
                                if nat_eq(a2574, a2575) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2577 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2576) => {
                                                h2576
                                            }
                                        };
                                        let a2578 = 1u64;
                                        if nat_eq(a2577, a2578) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2580 = match v3 {
                                            Adt7::C0(_, _, _, _, h2579) => {
                                                h2579
                                            }
                                        };
                                        let a2581 = 1u64;
                                        if nat_eq(a2580, a2581) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2583 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2582) => {
                                            h2582
                                        }
                                    };
                                    let a2584 = 1u64;
                                    if nat_eq(a2583, a2584) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2586 = match v3 {
                                        Adt7::C0(_, _, _, _, h2585) => {
                                            h2585
                                        }
                                    };
                                    let a2587 = 1u64;
                                    if nat_eq(a2586, a2587) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2589 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2588) => {
                                        h2588
                                    }
                                };
                                let a2590 = 1u64;
                                if nat_eq(a2589, a2590) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2592 = match v3 {
                                    Adt7::C0(_, _, _, _, h2591) => {
                                        h2591
                                    }
                                };
                                let a2593 = 1u64;
                                if nat_eq(a2592, a2593) {
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
    let v4: u64 = f188(match v3.clone() {
        Adt3::C0(h2594, _, _, _, _, _, _, _) => {
            h2594
        }
    })?;
    let v5: u64 = {
        let a2595 = v0;
        let a2596 = v4;
        let a2597 = 0u64;
        nat_rem(a2595, a2596, a2597)
    };
    let v6: u64 = {
        let a2598 = v0;
        let a2599 = v4;
        let a2600 = 0u64;
        nat_quot(a2598, a2599, a2600)
    };
    let a2601 = v5;
    let a2602 = 0u64;
    if nat_eq(a2601, a2602) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2605 = {
            let a2603 = v5;
            let a2604 = 1u64;
            nat_add(a2603, a2604)?
        };
        let a2606 = v4;
        if nat_eq(a2605, a2606) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f91(v6, v2)?))
        } else {
            let a2610 = {
                let a2607 = v5;
                let a2608 = 2u64;
                let a2609 = 0u64;
                nat_rem(a2607, a2608, a2609)
            };
            let a2611 = 0u64;
            if nat_eq(a2610, a2611) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2614 = {
                        let a2612 = v5;
                        let a2613 = 1u64;
                        nat_sub(a2612, a2613)
                    };
                    let a2615 = 2u64;
                    let a2616 = 0u64;
                    nat_quot(a2614, a2615, a2616)
                };
                let v8: u64 = f58(v6, v2.clone(), 0u64);
                let v9: u64 = f58(v6, v2.clone(), 1u64);
                let v10: u64 = f58(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f57(v1, {
                    let a2617 = v9;
                    let a2618 = v7;
                    nat_add(a2617, a2618)?
                }, v8, v10, 0u64)?, 4u64, f57(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f189(v3.clone())?;
    let v5: u64 = {
        let a2619 = v0;
        let a2620 = v4;
        let a2621 = 0u64;
        nat_rem(a2619, a2620, a2621)
    };
    let v6: u64 = {
        let a2622 = v0;
        let a2623 = v4;
        let a2624 = 0u64;
        nat_quot(a2622, a2623, a2624)
    };
    let a2625 = v5;
    let a2626 = 0u64;
    if nat_eq(a2625, a2626) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2629 = {
            let a2627 = v5;
            let a2628 = 1u64;
            nat_add(a2627, a2628)?
        };
        let a2630 = v4;
        if nat_eq(a2629, a2630) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f91(v6, v2)?))
        } else {
            let a2634 = {
                let a2631 = v5;
                let a2632 = 2u64;
                let a2633 = 0u64;
                nat_rem(a2631, a2632, a2633)
            };
            let a2635 = 0u64;
            if nat_eq(a2634, a2635) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2637 = f58(v6, v2.clone(), 0u64);
                    let a2638 = match v3.clone() {
                        Adt6::C0(_, _, h2636, _) => {
                            h2636
                        }
                    };
                    nat_add(a2637, a2638)?
                };
                let v8: u64 = {
                    let a2646 = {
                        let a2643 = {
                            let a2640 = v7;
                            let a2641 = match v3.clone() {
                                Adt6::C0(_, h2639, _, _) => {
                                    h2639
                                }
                            };
                            nat_add(a2640, a2641)?
                        };
                        let a2644 = match v3.clone() {
                            Adt6::C0(h2642, _, _, _) => {
                                h2642
                            }
                        };
                        nat_sub(a2643, a2644)
                    };
                    let a2647 = match v3.clone() {
                        Adt6::C0(_, h2645, _, _) => {
                            h2645
                        }
                    };
                    let a2648 = 0u64;
                    nat_quot(a2646, a2647, a2648)
                };
                let v9: u64 = {
                    let a2654 = v8;
                    let a2655 = {
                        let a2651 = {
                            let a2649 = v5;
                            let a2650 = 1u64;
                            nat_sub(a2649, a2650)
                        };
                        let a2652 = 2u64;
                        let a2653 = 0u64;
                        nat_quot(a2651, a2652, a2653)
                    };
                    nat_add(a2654, a2655)?
                };
                let v10: u64 = {
                    let a2657 = v9;
                    let a2658 = match v3.clone() {
                        Adt6::C0(_, h2656, _, _) => {
                            h2656
                        }
                    };
                    nat_mul(a2657, a2658)?
                };
                let v11: u64 = {
                    let a2659 = v7;
                    let a2660 = v10;
                    nat_sub(a2659, a2660)
                };
                let a2662 = v9;
                let a2663 = match v1.clone() {
                    Adt3::C0(_, h2661, _, _, _, _, _, _) => {
                        h2661
                    }
                };
                if if if nat_lt(a2662, a2663) {
                    let a2664 = v10;
                    let a2665 = v7;
                    nat_le(a2664, a2665)
                } else {
                    false
                } {
                    let a2667 = v11;
                    let a2668 = match v3.clone() {
                        Adt6::C0(h2666, _, _, _) => {
                            h2666
                        }
                    };
                    nat_lt(a2667, a2668)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f57(v1, {
                        let a2672 = {
                            let a2670 = f58(v6, v2, 1u64);
                            let a2671 = match v3 {
                                Adt6::C0(h2669, _, _, _) => {
                                    h2669
                                }
                            };
                            nat_mul(a2670, a2671)?
                        };
                        let a2673 = v11;
                        nat_add(a2672, a2673)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2675 = v5;
        let a2676 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2674, _) => {
                h2674
            }
        };
        let a2677 = 0u64;
        nat_rem(a2675, a2676, a2677)
    };
    let v7: u64 = {
        let a2679 = v5;
        let a2680 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2678, _) => {
                h2678
            }
        };
        let a2681 = 0u64;
        nat_quot(a2679, a2680, a2681)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2682, _, _, _, _, _, _, _) => {
            h2682
        }
    }, match v1.clone() {
        Adt3::C0(h2683, _, _, _, _, _, _, _) => {
            h2683
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a2687 = v4;
        let a2688 = {
            let m2686 = match v3.clone() {
                Adt5::C0(h2685, _, _, _, _, _, _, _) => {
                    h2685
                }
            };
            match m2686 {
                Adt6::C0(h2684, _, _, _) => {
                    h2684
                }
            }
        };
        let a2689 = 0u64;
        nat_rem(a2687, a2688, a2689)
    })?;
    let v9: u64 = {
        let a2693 = v4;
        let a2694 = {
            let m2692 = match v3.clone() {
                Adt5::C0(h2691, _, _, _, _, _, _, _) => {
                    h2691
                }
            };
            match m2692 {
                Adt6::C0(h2690, _, _, _) => {
                    h2690
                }
            }
        };
        let a2695 = 0u64;
        nat_quot(a2693, a2694, a2695)
    };
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2696, _, _, _, _, _, _) => {
            h2696
        }
    }, match v1.clone() {
        Adt3::C0(_, h2697, _, _, _, _, _, _) => {
            h2697
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a2701 = v9;
        let a2702 = {
            let m2700 = match v3.clone() {
                Adt5::C0(_, h2699, _, _, _, _, _, _) => {
                    h2699
                }
            };
            match m2700 {
                Adt6::C0(h2698, _, _, _) => {
                    h2698
                }
            }
        };
        let a2703 = 0u64;
        nat_rem(a2701, a2702, a2703)
    })?;
    let v11: u64 = {
        let a2707 = v9;
        let a2708 = {
            let m2706 = match v3.clone() {
                Adt5::C0(_, h2705, _, _, _, _, _, _) => {
                    h2705
                }
            };
            match m2706 {
                Adt6::C0(h2704, _, _, _) => {
                    h2704
                }
            }
        };
        let a2709 = 0u64;
        nat_quot(a2707, a2708, a2709)
    };
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2710, _, _, _, _, _) => {
            h2710
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2711, _, _, _, _, _) => {
            h2711
        }
    }, v6, {
        let a2715 = v11;
        let a2716 = {
            let m2714 = match v3.clone() {
                Adt5::C0(_, _, h2713, _, _, _, _, _) => {
                    h2713
                }
            };
            match m2714 {
                Adt6::C0(h2712, _, _, _) => {
                    h2712
                }
            }
        };
        let a2717 = 0u64;
        nat_rem(a2715, a2716, a2717)
    })?;
    let v13: u64 = {
        let a2721 = v11;
        let a2722 = {
            let m2720 = match v3.clone() {
                Adt5::C0(_, _, h2719, _, _, _, _, _) => {
                    h2719
                }
            };
            match m2720 {
                Adt6::C0(h2718, _, _, _) => {
                    h2718
                }
            }
        };
        let a2723 = 0u64;
        nat_quot(a2721, a2722, a2723)
    };
    let v17: Adt12 = {
        let m2724 = v8;
        match m2724 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2725 = v10;
                match m2725 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2726 = v12;
                        match m2726 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2735 = {
                                        let a2730 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a2731 = {
                                            let a2728 = v7;
                                            let a2729 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2727, _, _, _) => {
                                                    h2727
                                                }
                                            };
                                            nat_mul(a2728, a2729)?
                                        };
                                        nat_add(a2730, a2731)?
                                    };
                                    let a2736 = {
                                        let a2733 = v13;
                                        let a2734 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2732, _, _, _, _) => {
                                                h2732
                                            }
                                        };
                                        nat_mul(a2733, a2734)?
                                    };
                                    nat_add(a2735, a2736)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2737, _, _) => {
            h2737
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2738) => {
            h2738
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f91(v0, v2)?), {
        let a2740 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2739, _, _) => {
                h2739
            }
        };
        let a2741 = 0u64;
        if nat_eq(a2740, a2741) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2743 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2742) => {
                h2742
            }
        };
        let a2744 = 0u64;
        if nat_eq(a2743, a2744) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f84(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2757 = f190(v0, v1, match v2.clone() {
        Adt3::C0(h2745, _, _, _, _, _, _, _) => {
            h2745
        }
    }, match v2.clone() {
        Adt3::C0(_, h2746, _, _, _, _, _, _) => {
            h2746
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2747, _, _, _, _, _) => {
            h2747
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2748, _, _, _, _) => {
            h2748
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2749, _, _, _) => {
            h2749
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2750, _, _) => {
            h2750
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2751, _) => {
            h2751
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2752) => {
            h2752
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2753, _, _, _) => {
            h2753
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2754, _, _) => {
            h2754
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2755, _) => {
            h2755
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2756) => {
            h2756
        }
    }, v4)?;
    match m2757 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f20(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2760 = v6;
        let a2761 = {
            let a2758 = 3u64;
            let a2759 = v5;
            nat_add(a2758, a2759)?
        };
        nat_sub(a2760, a2761)
    };
    let a2763 = v0;
    let a2764 = f50(v3, match v1.clone() {
        Adt3::C0(h2762, _, _, _, _, _, _, _) => {
            h2762
        }
    }, v5, v8)?;
    if nat_lt(a2763, a2764) {
        let v9: u64 = f100(match v1.clone() {
            Adt3::C0(h2765, _, _, _, _, _, _, _) => {
                h2765
            }
        }, v5, v8)?;
        let v10: u64 = f101(v3, match v1.clone() {
            Adt3::C0(h2766, _, _, _, _, _, _, _) => {
                h2766
            }
        }, v5)?;
        let a2767 = v0;
        let a2768 = v9;
        if nat_lt(a2767, a2768) {
            f191(v0, v2, f44(v3), {
                let a2769 = 3u64;
                let a2770 = v5;
                nat_add(a2769, a2770)?
            }, v7)
        } else {
            let a2773 = v0;
            let a2774 = {
                let a2771 = v9;
                let a2772 = v10;
                nat_add(a2771, a2772)?
            };
            if nat_lt(a2773, a2774) {
                let v11: u64 = {
                    let a2775 = v0;
                    let a2776 = v9;
                    nat_sub(a2775, a2776)
                };
                let v12: u64 = f192(v3, match v1.clone() {
                    Adt3::C0(h2777, _, _, _, _, _, _, _) => {
                        h2777
                    }
                })?;
                f193({
                    let a2778 = v11;
                    let a2779 = v12;
                    let a2780 = 0u64;
                    nat_rem(a2778, a2779, a2780)
                }, {
                    let a2781 = v11;
                    let a2782 = v12;
                    let a2783 = 0u64;
                    nat_quot(a2781, a2782, a2783)
                }, v1, v3, v4, v7)
            } else {
                f194({
                    let a2786 = v0;
                    let a2787 = {
                        let a2784 = v9;
                        let a2785 = v10;
                        nat_add(a2784, a2785)?
                    };
                    nat_sub(a2786, a2787)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2788 = v0;
    let a2789 = f36(v1)?;
    if nat_lt(a2788, a2789) {
        let a2790 = v0;
        let a2791 = 4u64;
        if nat_lt(a2790, a2791) {
            let a2792 = v0;
            let a2793 = 0u64;
            if nat_eq(a2792, a2793) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2794 = v0;
                let a2795 = 1u64;
                if nat_eq(a2794, a2795) {
                    let a2796 = v5;
                    let a2797 = 0u64;
                    if nat_eq(a2796, a2797) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2798 = v0;
                    let a2799 = 2u64;
                    if nat_eq(a2798, a2799) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2800 = v0;
            let a2801 = 4u64;
            if if nat_eq(a2800, a2801) {
                let a2802 = f195(v5)?;
                let a2803 = v1;
                !nat_eq(a2802, a2803)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2806 = {
                        let a2804 = v0;
                        let a2805 = 4u64;
                        nat_sub(a2804, a2805)
                    };
                    let a2807 = 3u64;
                    let a2808 = 0u64;
                    nat_quot(a2806, a2807, a2808)
                };
                let v7: u64 = {
                    let a2811 = {
                        let a2809 = v0;
                        let a2810 = 4u64;
                        nat_sub(a2809, a2810)
                    };
                    let a2812 = 3u64;
                    let a2813 = 0u64;
                    nat_rem(a2811, a2812, a2813)
                };
                let a2814 = v7;
                let a2815 = 0u64;
                if nat_eq(a2814, a2815) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2816 = v7;
                    let a2817 = 1u64;
                    if nat_eq(a2816, a2817) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f35(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f87(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2818 = v1;
        let a2819 = 5u64;
        nat_eq(a2818, a2819)
    } else {
        false
    } {
        let a2820 = v2;
        let a2821 = 16u64;
        nat_eq(a2820, a2821)
    } else {
        false
    } {
        let a2822 = v3;
        let a2823 = 0u64;
        nat_eq(a2822, a2823)
    } else {
        false
    } {
        let a2824 = v4;
        let a2825 = 0u64;
        nat_eq(a2824, a2825)
    } else {
        false
    }
}

pub fn f88(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2826 = v1;
    let a2827 = 0u64;
    if nat_eq(a2826, a2827) {
        if v0 {
            let a2828 = v3;
            let a2829 = 0u64;
            if if if nat_eq(a2828, a2829) {
                let a2830 = v4;
                let a2831 = 43u64;
                nat_eq(a2830, a2831)
            } else {
                false
            } {
                true
            } else {
                let a2832 = v3;
                let a2833 = 43u64;
                if nat_eq(a2832, a2833) {
                    let a2834 = v4;
                    let a2835 = 0u64;
                    nat_eq(a2834, a2835)
                } else {
                    false
                }
            } {
                f165(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2836 = v1;
        let a2837 = 1u64;
        if nat_eq(a2836, a2837) {
            Adt11::C1(v2)
        } else {
            let a2838 = v1;
            let a2839 = 2u64;
            if nat_eq(a2838, a2839) {
                Adt11::C5(v2)
            } else {
                let a2840 = v1;
                let a2841 = 3u64;
                if nat_eq(a2840, a2841) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2842 = v1;
                    let a2843 = 4u64;
                    if nat_eq(a2842, a2843) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f87(v0, v1, v2, v3, v4) {
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

pub fn f89(v0: u64, v1: u64) -> Adt12 {
    let a2844 = v0;
    let a2845 = 0u64;
    if nat_eq(a2844, a2845) {
        Adt12::C0(0u64, v1)
    } else {
        let a2846 = v0;
        let a2847 = 1u64;
        if nat_eq(a2846, a2847) {
            Adt12::C1(v1)
        } else {
            let a2848 = v0;
            let a2849 = 2u64;
            if nat_eq(a2848, a2849) {
                Adt12::C2(v1)
            } else {
                let a2850 = v0;
                let a2851 = 3u64;
                if nat_eq(a2850, a2851) {
                    Adt12::C3(v1)
                } else {
                    let a2852 = v0;
                    let a2853 = 13u64;
                    if nat_lt(a2852, a2853) {
                        Adt12::C0({
                            let a2854 = v0;
                            let a2855 = 3u64;
                            nat_sub(a2854, a2855)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f90(v0: u64, v1: u64) -> Adt13 {
    let a2856 = v0;
    let a2857 = 1u64;
    if nat_eq(a2856, a2857) {
        Adt13::C0(v1)
    } else {
        let a2858 = v0;
        let a2859 = 3u64;
        if if nat_eq(a2858, a2859) {
            true
        } else {
            let a2860 = v0;
            let a2861 = 6u64;
            nat_eq(a2860, a2861)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f91(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2863 = v0;
        let a2864 = match v1.clone() {
            Adt3::C0(h2862, _, _, _, _, _, _, _) => {
                h2862
            }
        };
        let a2865 = 0u64;
        nat_quot(a2863, a2864, a2865)
    };
    let v3: u64 = {
        let a2867 = v2;
        let a2868 = match v1.clone() {
            Adt3::C0(_, h2866, _, _, _, _, _, _) => {
                h2866
            }
        };
        let a2869 = 0u64;
        nat_quot(a2867, a2868, a2869)
    };
    f140({
        let a2871 = v0;
        let a2872 = match v1.clone() {
            Adt3::C0(h2870, _, _, _, _, _, _, _) => {
                h2870
            }
        };
        let a2873 = 0u64;
        nat_rem(a2871, a2872, a2873)
    }, {
        let a2875 = v2;
        let a2876 = match v1.clone() {
            Adt3::C0(_, h2874, _, _, _, _, _, _) => {
                h2874
            }
        };
        let a2877 = 0u64;
        nat_rem(a2875, a2876, a2877)
    }, {
        let a2879 = v3;
        let a2880 = match v1.clone() {
            Adt3::C0(_, _, h2878, _, _, _, _, _) => {
                h2878
            }
        };
        let a2881 = 0u64;
        nat_rem(a2879, a2880, a2881)
    }, {
        let a2883 = v3;
        let a2884 = match v1.clone() {
            Adt3::C0(_, _, h2882, _, _, _, _, _) => {
                h2882
            }
        };
        let a2885 = 0u64;
        nat_quot(a2883, a2884, a2885)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2886, _, _, _) => {
            h2886
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2887, _, _) => {
            h2887
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2888, _) => {
            h2888
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2889) => {
            h2889
        }
    })
}

pub fn f92(v0: Adt6, v1: u64) -> R<u64> {
    let a2891 = 0u64;
    let a2892 = match v0.clone() {
        Adt6::C0(h2890, _, _, _) => {
            h2890
        }
    };
    if if if if if nat_lt(a2891, a2892) {
        let a2894 = 0u64;
        let a2895 = match v0.clone() {
            Adt6::C0(_, h2893, _, _) => {
                h2893
            }
        };
        nat_lt(a2894, a2895)
    } else {
        false
    } {
        let a2897 = 0u64;
        let a2898 = match v0.clone() {
            Adt6::C0(_, _, _, h2896) => {
                h2896
            }
        };
        nat_lt(a2897, a2898)
    } else {
        false
    } {
        let a2900 = match v0.clone() {
            Adt6::C0(_, h2899, _, _) => {
                h2899
            }
        };
        let a2901 = 2147483648u64;
        nat_lt(a2900, a2901)
    } else {
        false
    } {
        let a2903 = match v0.clone() {
            Adt6::C0(_, _, _, h2902) => {
                h2902
            }
        };
        let a2904 = 2147483648u64;
        nat_lt(a2903, a2904)
    } else {
        false
    } {
        let v2: u64 = {
            let a2908 = v1;
            let a2909 = {
                let a2906 = 2u64;
                let a2907 = f142(match v0.clone() {
                    Adt6::C0(_, _, h2905, _) => {
                        h2905
                    }
                });
                nat_mul(a2906, a2907)?
            };
            nat_add(a2908, a2909)?
        };
        let v3: u64 = {
            let a2921 = {
                let a2916 = 1u64;
                let a2917 = {
                    let a2914 = match v0.clone() {
                        Adt6::C0(_, _, _, h2910) => {
                            h2910
                        }
                    };
                    let a2915 = {
                        let a2912 = match v0.clone() {
                            Adt6::C0(h2911, _, _, _) => {
                                h2911
                            }
                        };
                        let a2913 = 1u64;
                        nat_sub(a2912, a2913)
                    };
                    nat_mul(a2914, a2915)?
                };
                nat_add(a2916, a2917)?
            };
            let a2922 = {
                let a2919 = 2u64;
                let a2920 = f141(match v0.clone() {
                    Adt6::C0(_, _, h2918, _) => {
                        h2918
                    }
                });
                nat_mul(a2919, a2920)?
            };
            nat_add(a2921, a2922)?
        };
        let a2923 = v3;
        let a2924 = v2;
        if nat_le(a2923, a2924) {
            let a2931 = 1u64;
            let a2932 = {
                let a2928 = {
                    let a2925 = v2;
                    let a2926 = v3;
                    nat_sub(a2925, a2926)
                };
                let a2929 = match v0 {
                    Adt6::C0(_, h2927, _, _) => {
                        h2927
                    }
                };
                let a2930 = 0u64;
                nat_quot(a2928, a2929, a2930)
            };
            nat_add(a2931, a2932)
        } else {
            let a2936 = {
                let a2933 = v3;
                let a2934 = v2;
                nat_sub(a2933, a2934)
            };
            let a2937 = match v0 {
                Adt6::C0(_, h2935, _, _) => {
                    h2935
                }
            };
            if nat_lt(a2936, a2937) {
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
        let a2938 = v0;
        let a2939 = 4294967296u64;
        let a2940 = 0u64;
        nat_rem(a2938, a2939, a2940)
    };
    let a2941 = v1;
    let a2942 = 2147483648u64;
    if nat_lt(a2941, a2942) {
        v1
    } else {
        0u64
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a2945 = 2u64;
    let a2946 = {
        let a2943 = 2u64;
        let a2944 = v0;
        nat_mul(a2943, a2944)?
    };
    nat_add(a2945, a2946)
}

pub fn f95(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2948 = match v1.clone() {
        Adt14::C0(h2947, _, _, _, _, _, _, _, _, _, _) => {
            h2947
        }
    };
    let a2949 = 0u64;
    if if if if nat_eq(a2948, a2949) {
        let a2951 = match v2.clone() {
            Adt14::C0(h2950, _, _, _, _, _, _, _, _, _, _) => {
                h2950
            }
        };
        let a2952 = 0u64;
        nat_eq(a2951, a2952)
    } else {
        false
    } {
        f196(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f197(match v1.clone() {
            Adt14::C0(_, h2953, _, _, _, _, _, _, _, _, _) => {
                h2953
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2954, _, _, _, _, _, _, _, _) => {
                h2954
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2955, _, _, _, _, _, _, _) => {
                h2955
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2956, _, _, _, _, _, _) => {
                h2956
            }
        }) {
            true
        } else {
            if if if if f45(v0) {
                f198(v1.clone(), v2.clone())
            } else {
                false
            } {
                f199(v1.clone())
            } else {
                false
            } {
                f199(v2.clone())
            } else {
                false
            } {
                if f200(v1.clone(), v2.clone())? {
                    true
                } else {
                    f201(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f197(match v1.clone() {
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
            0u64
        } else {
            let a2966 = match v1.clone() {
                Adt14::C0(_, _, h2961, _, _, _, _, _, _, _, _) => {
                    h2961
                }
            };
            let a2967 = {
                let a2964 = match v1.clone() {
                    Adt14::C0(_, _, _, h2962, _, _, _, _, _, _, _) => {
                        h2962
                    }
                };
                let a2965 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2963, _, _, _, _, _, _) => {
                        h2963
                    }
                };
                nat_mul(a2964, a2965)?
            };
            nat_mul(a2966, a2967)?
        };
        let v4: u64 = {
            let a2968 = v3;
            let a2969 = 0u64;
            if if nat_eq(a2968, a2969) {
                true
            } else {
                f202(v2.clone())?
            } {
                0u64
            } else {
                let a2971 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2970, _) => {
                        h2970
                    }
                };
                let a2972 = 4u64;
                let a2973 = 0u64;
                nat_quot(a2971, a2972, a2973)
            }
        };
        Ok(Adt15::C1(f203(f204(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2974, _, _, _, _, _) => {
                h2974
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2975, _, _, _, _, _) => {
                h2975
            }
        }), match v1 {
            Adt14::C0(_, h2976, _, _, _, _, _, _, _, _, _) => {
                h2976
            }
        }, v3, v4)?, 4u64, 64u64, v3, f205(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f206(v3.clone());
    let a2978 = match v2.clone() {
        Adt14::C0(h2977, _, _, _, _, _, _, _, _, _, _) => {
            h2977
        }
    };
    let a2979 = 0u64;
    if if if if nat_eq(a2978, a2979) {
        let a2981 = match v3.clone() {
            Adt14::C0(h2980, _, _, _, _, _, _, _, _, _, _) => {
                h2980
            }
        };
        let a2982 = 0u64;
        nat_eq(a2981, a2982)
    } else {
        false
    } {
        f196(v2.clone(), v3)
    } else {
        false
    } {
        if f197(match v2.clone() {
            Adt14::C0(_, h2983, _, _, _, _, _, _, _, _, _) => {
                h2983
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2984, _, _, _, _, _, _, _, _) => {
                h2984
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2985, _, _, _, _, _, _, _) => {
                h2985
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2986, _, _, _, _, _, _) => {
                h2986
            }
        }) {
            true
        } else {
            let a2988 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2987, _, _, _, _, _) => {
                    h2987
                }
            };
            let a2989 = 4u64;
            if if if if if nat_eq(a2988, a2989) {
                f207(match v2.clone() {
                    Adt14::C0(_, h2990, _, _, _, _, _, _, _, _, _) => {
                        h2990
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2991, _, _, _, _, _, _, _, _) => {
                        h2991
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2992, _, _, _, _, _, _, _) => {
                        h2992
                    }
                }, v0)?
            } else {
                false
            } {
                f199(v2.clone())
            } else {
                false
            } {
                f199(v4.clone())
            } else {
                false
            } {
                let a2994 = f208(v4.clone())?;
                let a2995 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2993, _) => {
                        h2993
                    }
                };
                nat_le(a2994, a2995)
            } else {
                false
            } {
                if f200(v2.clone(), v4.clone())? {
                    true
                } else {
                    f201(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f197(match v2.clone() {
            Adt14::C0(_, h2996, _, _, _, _, _, _, _, _, _) => {
                h2996
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2997, _, _, _, _, _, _, _, _) => {
                h2997
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2998, _, _, _, _, _, _, _) => {
                h2998
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2999, _, _, _, _, _, _) => {
                h2999
            }
        }) {
            true
        } else {
            f202(v4.clone())?
        } {
            0u64
        } else {
            let a3001 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3000, _) => {
                    h3000
                }
            };
            let a3002 = 4u64;
            let a3003 = 0u64;
            nat_quot(a3001, a3002, a3003)
        };
        Ok(Adt15::C1(f209(match v2.clone() {
            Adt14::C0(_, h3004, _, _, _, _, _, _, _, _, _) => {
                h3004
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3005, _, _, _, _, _, _, _, _) => {
                h3005
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3006, _, _, _, _, _, _, _) => {
                h3006
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3007, _, _, _, _, _, _) => {
                h3007
            }
        }, v0, v5)?, 4u64, 65u64, v0, f205(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f206(v4.clone());
    if if if if if if if v1 {
        let a3009 = match v2.clone() {
            Adt14::C0(h3008, _, _, _, _, _, _, _, _, _, _) => {
                h3008
            }
        };
        let a3010 = 0u64;
        nat_eq(a3009, a3010)
    } else {
        false
    } {
        let a3012 = match v3.clone() {
            Adt14::C0(h3011, _, _, _, _, _, _, _, _, _, _) => {
                h3011
            }
        };
        let a3013 = 0u64;
        nat_eq(a3012, a3013)
    } else {
        false
    } {
        let a3015 = match v4.clone() {
            Adt14::C0(h3014, _, _, _, _, _, _, _, _, _, _) => {
                h3014
            }
        };
        let a3016 = 0u64;
        nat_eq(a3015, a3016)
    } else {
        false
    } {
        f196(v2.clone(), v3.clone())
    } else {
        false
    } {
        f196(v2.clone(), v4)
    } else {
        false
    } {
        if f197(match v2.clone() {
            Adt14::C0(_, h3017, _, _, _, _, _, _, _, _, _) => {
                h3017
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3018, _, _, _, _, _, _, _, _) => {
                h3018
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3019, _, _, _, _, _, _, _) => {
                h3019
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3020, _, _, _, _, _, _) => {
                h3020
            }
        }) {
            true
        } else {
            let a3022 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3021, _, _, _, _, _) => {
                    h3021
                }
            };
            let a3023 = 4u64;
            if if if if if if if nat_eq(a3022, a3023) {
                let a3025 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3024, _, _, _, _, _) => {
                        h3024
                    }
                };
                let a3026 = 4u64;
                nat_eq(a3025, a3026)
            } else {
                false
            } {
                f199(v2.clone())
            } else {
                false
            } {
                f199(v3.clone())
            } else {
                false
            } {
                f199(v5.clone())
            } else {
                false
            } {
                let a3028 = f208(v5.clone())?;
                let a3029 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3027, _) => {
                        h3027
                    }
                };
                nat_le(a3028, a3029)
            } else {
                false
            } {
                if f200(v2.clone(), v5.clone())? {
                    true
                } else {
                    f201(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f200(v3.clone(), v5.clone())? {
                    true
                } else {
                    f201(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f197(match v2.clone() {
            Adt14::C0(_, h3030, _, _, _, _, _, _, _, _, _) => {
                h3030
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3031, _, _, _, _, _, _, _, _) => {
                h3031
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3032, _, _, _, _, _, _, _) => {
                h3032
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3033, _, _, _, _, _, _) => {
                h3033
            }
        }) {
            0u64
        } else {
            let a3039 = match v2.clone() {
                Adt14::C0(_, _, h3034, _, _, _, _, _, _, _, _) => {
                    h3034
                }
            };
            let a3040 = {
                let a3037 = match v2.clone() {
                    Adt14::C0(_, _, _, h3035, _, _, _, _, _, _, _) => {
                        h3035
                    }
                };
                let a3038 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3036, _, _, _, _, _, _) => {
                        h3036
                    }
                };
                nat_mul(a3037, a3038)?
            };
            nat_mul(a3039, a3040)?
        };
        let v7: u64 = {
            let a3041 = v6;
            let a3042 = 0u64;
            if if nat_eq(a3041, a3042) {
                true
            } else {
                f202(v5.clone())?
            } {
                0u64
            } else {
                let a3044 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3043, _) => {
                        h3043
                    }
                };
                let a3045 = 4u64;
                let a3046 = 0u64;
                nat_quot(a3044, a3045, a3046)
            }
        };
        Ok(Adt15::C1(f210(match v2 {
            Adt14::C0(_, h3047, _, _, _, _, _, _, _, _, _) => {
                h3047
            }
        }, v6, v7)?, 4u64, 66u64, v6, f205(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f211(v0, v3, v6) {
        f211(v1, v4, v6)
    } else {
        false
    } {
        f211(v2, v5, v6)
    } else {
        false
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f212(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f212(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f212(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f212(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f212(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f212(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3048 = v2;
    let a3049 = 0u64;
    if nat_eq(a3048, a3049) {
        Ok(0u64)
    } else {
        let a3052 = 2u64;
        let a3053 = {
            let a3050 = v0;
            let a3051 = v1;
            nat_mul(a3050, a3051)?
        };
        nat_mul(a3052, a3053)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3054 = v2;
    let a3055 = f192(v0, v1)?;
    nat_mul(a3054, a3055)
}

pub fn f102(v0: u64) -> u64 {
    let a3056 = v0;
    let a3057 = 30u64;
    if nat_eq(a3056, a3057) {
        3u64
    } else {
        1u64
    }
}

pub fn f103(v0: u64) -> R<u64> {
    let a3060 = 5u64;
    let a3061 = {
        let a3058 = 4u64;
        let a3059 = v0;
        nat_mul(a3058, a3059)?
    };
    nat_add(a3060, a3061)
}

pub fn f104(v0: u64) -> bool {
    let m3062 = f213(v0);
    match m3062 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3064 = 1u64;
            let a3065 = match v1 {
                Adt19::C0(_, h3063, _) => {
                    h3063
                }
            };
            nat_lt(a3064, a3065)
        }
    }
}

pub fn f105(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3067 = match v1.clone() {
        Adt2::C0(h3066, _, _, _, _, _, _, _, _, _, _) => {
            h3066
        }
    };
    let a3068 = 0u64;
    if if if if if if if nat_eq(a3067, a3068) {
        let a3070 = match v3.clone() {
            Adt2::C0(h3069, _, _, _, _, _, _, _, _, _, _) => {
                h3069
            }
        };
        let a3071 = 0u64;
        nat_eq(a3070, a3071)
    } else {
        false
    } {
        let a3073 = match v2.clone() {
            Adt2::C0(h3072, _, _, _, _, _, _, _, _, _, _) => {
                h3072
            }
        };
        let a3074 = 1u64;
        nat_le(a3073, a3074)
    } else {
        false
    } {
        f38(v1.clone())?
    } else {
        false
    } {
        f26(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v3.clone(), f23(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3076 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3075, _, _, _, _, _) => {
                h3075
            }
        };
        let a3077 = 0u64;
        if nat_eq(a3076, a3077) {
            true
        } else {
            let a3079 = match v3.clone() {
                Adt2::C0(_, _, h3078, _, _, _, _, _, _, _, _) => {
                    h3078
                }
            };
            let a3080 = 1u64;
            if nat_le(a3079, a3080) {
                true
            } else {
                let a3085 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3081, _, _, _, _) => {
                        h3081
                    }
                };
                let a3086 = {
                    let a3083 = 4u64;
                    let a3084 = match v3.clone() {
                        Adt2::C0(_, h3082, _, _, _, _, _, _, _, _, _) => {
                            h3082
                        }
                    };
                    nat_mul(a3083, a3084)?
                };
                nat_eq(a3085, a3086)
            }
        }
    } else {
        false
    } {
        f114(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f106(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3088 = match v0.clone() {
        Adt2::C0(h3087, _, _, _, _, _, _, _, _, _, _) => {
            h3087
        }
    };
    let a3089 = 0u64;
    if if if if if if if if if if if if nat_eq(a3088, a3089) {
        f38(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3091 = match v1.clone() {
            Adt2::C0(h3090, _, _, _, _, _, _, _, _, _, _) => {
                h3090
            }
        };
        let a3092 = 0u64;
        if if nat_eq(a3091, a3092) {
            f38(v1.clone())?
        } else {
            false
        } {
            f26(v1.clone(), f23(v1.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3094 = match v2.clone() {
            Adt2::C0(h3093, _, _, _, _, _, _, _, _, _, _) => {
                h3093
            }
        };
        let a3095 = 0u64;
        if if nat_eq(a3094, a3095) {
            f38(v2.clone())?
        } else {
            false
        } {
            f26(v2.clone(), f23(v2.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3098 = match v0.clone() {
            Adt2::C0(_, h3096, _, _, _, _, _, _, _, _, _) => {
                h3096
            }
        };
        let a3099 = match v0.clone() {
            Adt2::C0(_, _, h3097, _, _, _, _, _, _, _, _) => {
                h3097
            }
        };
        nat_eq(a3098, a3099)
    } else {
        false
    } {
        let a3102 = match v0.clone() {
            Adt2::C0(_, _, h3100, _, _, _, _, _, _, _, _) => {
                h3100
            }
        };
        let a3103 = match v1.clone() {
            Adt2::C0(_, _, h3101, _, _, _, _, _, _, _, _) => {
                h3101
            }
        };
        nat_eq(a3102, a3103)
    } else {
        false
    } {
        let a3106 = match v0.clone() {
            Adt2::C0(_, _, _, h3104, _, _, _, _, _, _, _) => {
                h3104
            }
        };
        let a3107 = match v1.clone() {
            Adt2::C0(_, _, _, h3105, _, _, _, _, _, _, _) => {
                h3105
            }
        };
        nat_eq(a3106, a3107)
    } else {
        false
    } {
        let a3110 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3108, _, _, _, _, _, _) => {
                h3108
            }
        };
        let a3111 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3109, _, _, _, _, _, _) => {
                h3109
            }
        };
        nat_eq(a3110, a3111)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f30(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3120 = {
                let a3117 = {
                    let a3114 = match v1.clone() {
                        Adt2::C0(_, _, _, h3112, _, _, _, _, _, _, _) => {
                            h3112
                        }
                    };
                    let a3115 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3113, _, _, _, _, _, _) => {
                            h3113
                        }
                    };
                    nat_mul(a3114, a3115)?
                };
                let a3118 = match v1 {
                    Adt2::C0(_, h3116, _, _, _, _, _, _, _, _, _) => {
                        h3116
                    }
                };
                nat_mul(a3117, a3118)?
            };
            let a3121 = f171(match v0.clone() {
                Adt2::C0(_, h3119, _, _, _, _, _, _, _, _, _) => {
                    h3119
                }
            })?;
            nat_mul(a3120, a3121)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3123 = 0u64;
        let a3124 = match v0.clone() {
            Adt1::C0(_, h3122, _, _, _, _, _, _, _, _, _, _) => {
                h3122
            }
        };
        nat_lt(a3123, a3124)
    };
    let a3126 = match v1.clone() {
        Adt2::C0(h3125, _, _, _, _, _, _, _, _, _, _) => {
            h3125
        }
    };
    let a3127 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3126, a3127) {
        let a3129 = match v2.clone() {
            Adt2::C0(h3128, _, _, _, _, _, _, _, _, _, _) => {
                h3128
            }
        };
        let a3130 = 0u64;
        nat_eq(a3129, a3130)
    } else {
        false
    } {
        let a3132 = match v3.clone() {
            Adt2::C0(h3131, _, _, _, _, _, _, _, _, _, _) => {
                h3131
            }
        };
        let a3133 = 0u64;
        nat_eq(a3132, a3133)
    } else {
        false
    } {
        let a3135 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3134, _, _, _, _, _, _) => {
                h3134
            }
        };
        let a3136 = 1u64;
        nat_eq(a3135, a3136)
    } else {
        false
    } {
        let a3138 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3137, _, _, _, _, _, _) => {
                h3137
            }
        };
        let a3139 = 1u64;
        nat_eq(a3138, a3139)
    } else {
        false
    } {
        let a3141 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3140, _, _, _, _, _, _) => {
                h3140
            }
        };
        let a3142 = 1u64;
        nat_eq(a3141, a3142)
    } else {
        false
    } {
        let a3144 = 0u64;
        let a3145 = match v1.clone() {
            Adt2::C0(_, _, h3143, _, _, _, _, _, _, _, _) => {
                h3143
            }
        };
        nat_lt(a3144, a3145)
    } else {
        false
    } {
        let a3147 = match v3.clone() {
            Adt2::C0(_, _, _, h3146, _, _, _, _, _, _, _) => {
                h3146
            }
        };
        let a3148 = 1u64;
        nat_eq(a3147, a3148)
    } else {
        false
    } {
        let a3151 = match v3.clone() {
            Adt2::C0(_, h3149, _, _, _, _, _, _, _, _, _) => {
                h3149
            }
        };
        let a3152 = match v1.clone() {
            Adt2::C0(_, h3150, _, _, _, _, _, _, _, _, _) => {
                h3150
            }
        };
        nat_eq(a3151, a3152)
    } else {
        false
    } {
        let a3155 = match v3.clone() {
            Adt2::C0(_, _, h3153, _, _, _, _, _, _, _, _) => {
                h3153
            }
        };
        let a3156 = match v1.clone() {
            Adt2::C0(_, _, _, h3154, _, _, _, _, _, _, _) => {
                h3154
            }
        };
        nat_eq(a3155, a3156)
    } else {
        false
    } {
        if v4 {
            f21(v1.clone(), v2.clone())
        } else {
            let a3159 = match v2.clone() {
                Adt2::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                    h3157
                }
            };
            let a3160 = match v1.clone() {
                Adt2::C0(_, _, h3158, _, _, _, _, _, _, _, _) => {
                    h3158
                }
            };
            if if nat_eq(a3159, a3160) {
                let a3163 = match v2.clone() {
                    Adt2::C0(_, _, h3161, _, _, _, _, _, _, _, _) => {
                        h3161
                    }
                };
                let a3164 = match v1.clone() {
                    Adt2::C0(_, _, _, h3162, _, _, _, _, _, _, _) => {
                        h3162
                    }
                };
                nat_eq(a3163, a3164)
            } else {
                false
            } {
                let a3166 = match v2.clone() {
                    Adt2::C0(_, _, _, h3165, _, _, _, _, _, _, _) => {
                        h3165
                    }
                };
                let a3167 = 1u64;
                nat_eq(a3166, a3167)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f26(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v2.clone(), f23(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v3.clone())?
    } else {
        false
    } {
        f30(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3169 = f23(v3.clone(), 1u64)?;
            let a3170 = f167(match v1.clone() {
                Adt2::C0(_, _, h3168, _, _, _, _, _, _, _, _) => {
                    h3168
                }
            }, v4)?;
            nat_mul(a3169, a3170)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C36(f25(v2, 1u64), v4, match v0 {
            Adt1::C0(h3171, _, _, _, _, _, _, _, _, _, _, _) => {
                h3171
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3172, _, _, _, _, _, _, _, _, _) => {
            h3172
        }
    }, match v0.clone() {
        Adt1::C0(h3173, _, _, _, _, _, _, _, _, _, _, _) => {
            h3173
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3174, _, _, _, _, _, _, _, _, _) => {
            h3174
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3175, _, _, _, _, _, _, _) => {
            h3175
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3176, _, _, _, _, _, _, _, _) => {
            h3176
        }
    }, match v0.clone() {
        Adt1::C0(_, h3177, _, _, _, _, _, _, _, _, _, _) => {
            h3177
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3178, _, _, _, _, _, _, _, _) => {
            h3178
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3179, _, _, _, _, _, _) => {
            h3179
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3180, _, _, _, _, _, _, _, _, _, _) => {
            h3180
        }
    };
    let v7: bool = {
        let a3183 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3181, _, _, _) => {
                h3181
            }
        };
        let a3184 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3182, _, _, _, _, _) => {
                h3182
            }
        };
        nat_lt(a3183, a3184)
    };
    let a3185 = v6;
    let a3186 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3185, a3186) {
        true
    } else {
        let a3187 = v6;
        let a3188 = 1u64;
        nat_eq(a3187, a3188)
    } {
        let a3190 = match v2.clone() {
            Adt2::C0(h3189, _, _, _, _, _, _, _, _, _, _) => {
                h3189
            }
        };
        let a3191 = 0u64;
        nat_eq(a3190, a3191)
    } else {
        false
    } {
        let a3193 = match v3.clone() {
            Adt2::C0(h3192, _, _, _, _, _, _, _, _, _, _) => {
                h3192
            }
        };
        let a3194 = 0u64;
        nat_eq(a3193, a3194)
    } else {
        false
    } {
        let a3196 = match v1.clone() {
            Adt2::C0(_, _, _, h3195, _, _, _, _, _, _, _) => {
                h3195
            }
        };
        let a3197 = 1u64;
        nat_eq(a3196, a3197)
    } else {
        false
    } {
        let a3200 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3198, _, _, _, _, _, _) => {
                h3198
            }
        };
        let a3201 = match v2.clone() {
            Adt2::C0(_, _, _, h3199, _, _, _, _, _, _, _) => {
                h3199
            }
        };
        nat_eq(a3200, a3201)
    } else {
        false
    } {
        let a3204 = match v3.clone() {
            Adt2::C0(_, _, _, h3202, _, _, _, _, _, _, _) => {
                h3202
            }
        };
        let a3205 = match v2.clone() {
            Adt2::C0(_, _, _, h3203, _, _, _, _, _, _, _) => {
                h3203
            }
        };
        nat_eq(a3204, a3205)
    } else {
        false
    } {
        let a3208 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3206, _, _, _, _, _, _) => {
                h3206
            }
        };
        let a3209 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3207, _, _, _, _, _, _) => {
                h3207
            }
        };
        nat_eq(a3208, a3209)
    } else {
        false
    } {
        let a3211 = 0u64;
        let a3212 = match v4.clone() {
            Adt6::C0(h3210, _, _, _) => {
                h3210
            }
        };
        if if if if if nat_lt(a3211, a3212) {
            let a3214 = 0u64;
            let a3215 = match v4.clone() {
                Adt6::C0(_, h3213, _, _) => {
                    h3213
                }
            };
            nat_lt(a3214, a3215)
        } else {
            false
        } {
            let a3217 = match v4.clone() {
                Adt6::C0(_, h3216, _, _) => {
                    h3216
                }
            };
            let a3218 = 2147483648u64;
            nat_lt(a3217, a3218)
        } else {
            false
        } {
            let a3220 = 0u64;
            let a3221 = match v4.clone() {
                Adt6::C0(_, _, _, h3219) => {
                    h3219
                }
            };
            nat_lt(a3220, a3221)
        } else {
            false
        } {
            let a3223 = match v4.clone() {
                Adt6::C0(_, _, _, h3222) => {
                    h3222
                }
            };
            let a3224 = 2147483648u64;
            nat_lt(a3223, a3224)
        } else {
            false
        } {
            let a3227 = match v3.clone() {
                Adt2::C0(_, h3225, _, _, _, _, _, _, _, _, _) => {
                    h3225
                }
            };
            let a3228 = f92(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3226, _, _, _, _, _, _, _, _, _) => {
                    h3226
                }
            })?;
            nat_eq(a3227, a3228)
        } else {
            false
        }
    } else {
        false
    } {
        let a3230 = 0u64;
        let a3231 = match v5.clone() {
            Adt6::C0(h3229, _, _, _) => {
                h3229
            }
        };
        if if if if if nat_lt(a3230, a3231) {
            let a3233 = 0u64;
            let a3234 = match v5.clone() {
                Adt6::C0(_, h3232, _, _) => {
                    h3232
                }
            };
            nat_lt(a3233, a3234)
        } else {
            false
        } {
            let a3236 = match v5.clone() {
                Adt6::C0(_, h3235, _, _) => {
                    h3235
                }
            };
            let a3237 = 2147483648u64;
            nat_lt(a3236, a3237)
        } else {
            false
        } {
            let a3239 = 0u64;
            let a3240 = match v5.clone() {
                Adt6::C0(_, _, _, h3238) => {
                    h3238
                }
            };
            nat_lt(a3239, a3240)
        } else {
            false
        } {
            let a3242 = match v5.clone() {
                Adt6::C0(_, _, _, h3241) => {
                    h3241
                }
            };
            let a3243 = 2147483648u64;
            nat_lt(a3242, a3243)
        } else {
            false
        } {
            let a3246 = match v3.clone() {
                Adt2::C0(_, _, h3244, _, _, _, _, _, _, _, _) => {
                    h3244
                }
            };
            let a3247 = f92(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3245, _, _, _, _, _, _, _, _) => {
                    h3245
                }
            })?;
            nat_eq(a3246, a3247)
        } else {
            false
        }
    } else {
        false
    } {
        f214(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f214(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f214(v1.clone(), {
            let a3248 = v6;
            let a3249 = 0u64;
            if nat_eq(a3248, a3249) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f30(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3258 = {
                let a3256 = {
                    let a3254 = 3u64;
                    let a3255 = {
                        let a3252 = match v4.clone() {
                            Adt6::C0(h3250, _, _, _) => {
                                h3250
                            }
                        };
                        let a3253 = match v5.clone() {
                            Adt6::C0(h3251, _, _, _) => {
                                h3251
                            }
                        };
                        nat_mul(a3252, a3253)?
                    };
                    nat_mul(a3254, a3255)?
                };
                let a3257 = 2u64;
                nat_add(a3256, a3257)?
            };
            let a3259 = f23(v3.clone(), 1u64)?;
            nat_mul(a3258, a3259)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C34(f25(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f109(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f21(v0.clone(), v1.clone()) {
        f21(v0.clone(), v2.clone())
    } else {
        false
    } {
        f31(v0.clone())?
    } else {
        false
    } {
        f31(v1.clone())?
    } else {
        false
    } {
        f31(v2.clone())?
    } else {
        false
    } {
        if f30(v0.clone(), v2.clone())? {
            true
        } else {
            f47(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f30(v1.clone(), v2.clone())? {
            true
        } else {
            f47(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3260 = 6u64;
            let a3261 = f23(v0.clone(), 1u64)?;
            nat_mul(a3260, a3261)?
        }, 2u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C33(f25(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3262 = v2;
    match m3262 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3263, _, _, _, _, _, _, _, _, _) => {
                    h3263
                }
            };
            let v6: u64 = {
                let a3269 = match v1.clone() {
                    Adt2::C0(_, _, h3264, _, _, _, _, _, _, _, _) => {
                        h3264
                    }
                };
                let a3270 = {
                    let a3267 = match v1.clone() {
                        Adt2::C0(_, _, _, h3265, _, _, _, _, _, _, _) => {
                            h3265
                        }
                    };
                    let a3268 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3266, _, _, _, _, _, _) => {
                            h3266
                        }
                    };
                    nat_mul(a3267, a3268)?
                };
                nat_mul(a3269, a3270)?
            };
            if if if if if if if if if if f21(v1.clone(), v4.clone()) {
                f21(v1.clone(), v3.clone())
            } else {
                false
            } {
                f215(v1.clone(), v6)?
            } else {
                false
            } {
                f215(v4.clone(), v6)?
            } else {
                false
            } {
                f215(v3.clone(), v6)?
            } else {
                false
            } {
                if f30(v1.clone(), v3.clone())? {
                    true
                } else {
                    f47(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f30(v4.clone(), v3.clone())? {
                    true
                } else {
                    f47(v4, v3.clone())
                }
            } else {
                false
            } {
                let a3271 = v5;
                let a3272 = 2147483647u64;
                nat_le(a3271, a3272)
            } else {
                false
            } {
                let a3273 = v6;
                let a3274 = 2147483647u64;
                nat_le(a3273, a3274)
            } else {
                false
            } {
                let a3276 = match v0.clone() {
                    Adt1::C0(h3275, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3275
                    }
                };
                let a3277 = 4294967295u64;
                if nat_le(a3276, a3277) {
                    let a3279 = match v0.clone() {
                        Adt1::C0(_, h3278, _, _, _, _, _, _, _, _, _, _) => {
                            h3278
                        }
                    };
                    let a3280 = 4294967295u64;
                    nat_le(a3279, a3280)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3281 = v5;
                    let a3282 = 0u64;
                    if nat_eq(a3281, a3282) {
                        0u64
                    } else {
                        let a3283 = v6;
                        let a3284 = f166(v5)?;
                        nat_mul(a3283, a3284)?
                    }
                }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3285, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3285
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f111(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3286, _, _, _, _, _, _, _, _, _, _, _) => {
            h3286
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3287, _, _, _, _, _, _, _, _, _, _) => {
            h3287
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3288, _, _, _, _, _, _, _, _, _) => {
            h3288
        }
    };
    let v6: u64 = {
        let a3290 = match v1.clone() {
            Adt2::C0(_, h3289, _, _, _, _, _, _, _, _, _) => {
                h3289
            }
        };
        let a3291 = v4;
        let a3292 = 0u64;
        nat_quot(a3290, a3291, a3292)
    };
    let v7: u64 = {
        let a3294 = match v1.clone() {
            Adt2::C0(h3293, _, _, _, _, _, _, _, _, _, _) => {
                h3293
            }
        };
        let a3295 = 0u64;
        if nat_eq(a3294, a3295) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f175(match v1.clone() {
        Adt2::C0(h3296, _, _, _, _, _, _, _, _, _, _) => {
            h3296
        }
    }) {
        let a3299 = match v1.clone() {
            Adt2::C0(h3297, _, _, _, _, _, _, _, _, _, _) => {
                h3297
            }
        };
        let a3300 = match v2.clone() {
            Adt2::C0(h3298, _, _, _, _, _, _, _, _, _, _) => {
                h3298
            }
        };
        nat_eq(a3299, a3300)
    } else {
        false
    } {
        let a3301 = 0u64;
        let a3302 = v3;
        nat_lt(a3301, a3302)
    } else {
        false
    } {
        let a3303 = v3;
        let a3304 = 2147483648u64;
        nat_lt(a3303, a3304)
    } else {
        false
    } {
        let a3305 = 0u64;
        let a3306 = v4;
        nat_lt(a3305, a3306)
    } else {
        false
    } {
        let a3307 = v4;
        let a3308 = 2147483648u64;
        nat_lt(a3307, a3308)
    } else {
        false
    } {
        let a3309 = v5;
        let a3310 = 2147483648u64;
        nat_lt(a3309, a3310)
    } else {
        false
    } {
        let a3311 = 0u64;
        let a3312 = v6;
        nat_lt(a3311, a3312)
    } else {
        false
    } {
        let a3314 = 0u64;
        let a3315 = match v2.clone() {
            Adt2::C0(_, h3313, _, _, _, _, _, _, _, _, _) => {
                h3313
            }
        };
        nat_lt(a3314, a3315)
    } else {
        false
    } {
        let a3319 = match v1.clone() {
            Adt2::C0(_, h3316, _, _, _, _, _, _, _, _, _) => {
                h3316
            }
        };
        let a3320 = {
            let a3317 = v6;
            let a3318 = v4;
            nat_mul(a3317, a3318)?
        };
        nat_eq(a3319, a3320)
    } else {
        false
    } {
        let a3322 = match v2.clone() {
            Adt2::C0(_, _, h3321, _, _, _, _, _, _, _, _) => {
                h3321
            }
        };
        let a3323 = v4;
        nat_eq(a3322, a3323)
    } else {
        false
    } {
        let a3336 = {
            let a3329 = {
                let a3327 = match v2.clone() {
                    Adt2::C0(_, h3324, _, _, _, _, _, _, _, _, _) => {
                        h3324
                    }
                };
                let a3328 = {
                    let a3325 = v5;
                    let a3326 = 2u64;
                    nat_mul(a3325, a3326)?
                };
                nat_add(a3327, a3328)?
            };
            let a3330 = v3;
            nat_add(a3329, a3330)?
        };
        let a3337 = {
            let a3334 = {
                let a3332 = match v1.clone() {
                    Adt2::C0(_, _, h3331, _, _, _, _, _, _, _, _) => {
                        h3331
                    }
                };
                let a3333 = v3;
                nat_mul(a3332, a3333)?
            };
            let a3335 = v6;
            nat_add(a3334, a3335)?
        };
        nat_eq(a3336, a3337)
    } else {
        false
    } {
        f216(v1.clone(), v7)?
    } else {
        false
    } {
        f216(v2.clone(), v7)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3338 = f23(v2.clone(), 1u64)?;
            let a3339 = f189(v8.clone())?;
            nat_mul(a3338, a3339)?
        }, v7, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3340, _, _, _, _, _, _, _, _, _, _) => {
                h3340
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3341 = v1;
    match m3341 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3343 = match v0.clone() {
                Adt2::C0(h3342, _, _, _, _, _, _, _, _, _, _) => {
                    h3342
                }
            };
            let a3344 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3343, a3344) {
                let a3346 = match v3.clone() {
                    Adt2::C0(h3345, _, _, _, _, _, _, _, _, _, _) => {
                        h3345
                    }
                };
                let a3347 = 0u64;
                nat_eq(a3346, a3347)
            } else {
                false
            } {
                let a3349 = match v2.clone() {
                    Adt2::C0(h3348, _, _, _, _, _, _, _, _, _, _) => {
                        h3348
                    }
                };
                let a3350 = 0u64;
                nat_eq(a3349, a3350)
            } else {
                false
            } {
                let a3352 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3351, _, _, _, _, _, _) => {
                        h3351
                    }
                };
                let a3353 = 1u64;
                nat_eq(a3352, a3353)
            } else {
                false
            } {
                let a3355 = match v3.clone() {
                    Adt2::C0(_, _, _, h3354, _, _, _, _, _, _, _) => {
                        h3354
                    }
                };
                let a3356 = 1u64;
                nat_eq(a3355, a3356)
            } else {
                false
            } {
                let a3358 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3357, _, _, _, _, _, _) => {
                        h3357
                    }
                };
                let a3359 = 1u64;
                nat_eq(a3358, a3359)
            } else {
                false
            } {
                let a3362 = match v0.clone() {
                    Adt2::C0(_, _, h3360, _, _, _, _, _, _, _, _) => {
                        h3360
                    }
                };
                let a3363 = match v3.clone() {
                    Adt2::C0(_, _, h3361, _, _, _, _, _, _, _, _) => {
                        h3361
                    }
                };
                nat_eq(a3362, a3363)
            } else {
                false
            } {
                let a3368 = match v3.clone() {
                    Adt2::C0(_, h3364, _, _, _, _, _, _, _, _, _) => {
                        h3364
                    }
                };
                let a3369 = {
                    let a3366 = match v0.clone() {
                        Adt2::C0(_, h3365, _, _, _, _, _, _, _, _, _) => {
                            h3365
                        }
                    };
                    let a3367 = 1u64;
                    nat_add(a3366, a3367)?
                };
                nat_le(a3368, a3369)
            } else {
                false
            } {
                let a3372 = match v2.clone() {
                    Adt2::C0(_, h3370, _, _, _, _, _, _, _, _, _) => {
                        h3370
                    }
                };
                let a3373 = match v0.clone() {
                    Adt2::C0(_, _, h3371, _, _, _, _, _, _, _, _) => {
                        h3371
                    }
                };
                nat_eq(a3372, a3373)
            } else {
                false
            } {
                let a3381 = match v2.clone() {
                    Adt2::C0(_, _, h3374, _, _, _, _, _, _, _, _) => {
                        h3374
                    }
                };
                let a3382 = {
                    let a3379 = {
                        let a3376 = match v0.clone() {
                            Adt2::C0(_, h3375, _, _, _, _, _, _, _, _, _) => {
                                h3375
                            }
                        };
                        let a3377 = 1u64;
                        nat_add(a3376, a3377)?
                    };
                    let a3380 = match v3.clone() {
                        Adt2::C0(_, h3378, _, _, _, _, _, _, _, _, _) => {
                            h3378
                        }
                    };
                    nat_sub(a3379, a3380)
                };
                nat_eq(a3381, a3382)
            } else {
                false
            } {
                let a3385 = match v2.clone() {
                    Adt2::C0(_, _, _, h3383, _, _, _, _, _, _, _) => {
                        h3383
                    }
                };
                let a3386 = match v0.clone() {
                    Adt2::C0(_, _, _, h3384, _, _, _, _, _, _, _) => {
                        h3384
                    }
                };
                nat_eq(a3385, a3386)
            } else {
                false
            } {
                let a3388 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3387, _, _, _, _, _, _) => {
                        h3387
                    }
                };
                let a3389 = 1u64;
                nat_eq(a3388, a3389)
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                let a3391 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3390, _, _, _, _, _) => {
                        h3390
                    }
                };
                let a3392 = 4u64;
                nat_eq(a3391, a3392)
            } else {
                false
            } {
                let a3394 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3393, _, _, _, _, _) => {
                        h3393
                    }
                };
                let a3395 = 4u64;
                nat_eq(a3394, a3395)
            } else {
                false
            } {
                let a3400 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3396, _, _, _, _) => {
                        h3396
                    }
                };
                let a3401 = {
                    let a3398 = match v0.clone() {
                        Adt2::C0(_, h3397, _, _, _, _, _, _, _, _, _) => {
                            h3397
                        }
                    };
                    let a3399 = 4u64;
                    nat_mul(a3398, a3399)?
                };
                nat_eq(a3400, a3401)
            } else {
                false
            } {
                let a3406 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3402, _, _, _, _) => {
                        h3402
                    }
                };
                let a3407 = {
                    let a3404 = match v3.clone() {
                        Adt2::C0(_, h3403, _, _, _, _, _, _, _, _, _) => {
                            h3403
                        }
                    };
                    let a3405 = 4u64;
                    nat_mul(a3404, a3405)?
                };
                nat_eq(a3406, a3407)
            } else {
                false
            } {
                f30(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f30(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3409 = f23(v2.clone(), 1u64)?;
                    let a3410 = f188(match v3.clone() {
                        Adt2::C0(_, h3408, _, _, _, _, _, _, _, _, _) => {
                            h3408
                        }
                    })?;
                    nat_mul(a3409, a3410)?
                }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C29(f25(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f113(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h3411, _, _, _, _, _, _, _, _, _, _) => {
            h3411
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h3412, _, _, _, _, _, _, _, _) => {
                h3412
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3413, _, _, _, _, _, _, _, _, _) => {
                h3413
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h3414, _, _, _, _, _, _) => {
                h3414
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h3415, _, _, _, _, _, _, _, _) => {
                h3415
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h3416, _, _, _, _, _, _, _, _, _) => {
                h3416
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h3417, _, _, _, _, _, _, _) => {
                h3417
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h3418, _, _, _, _, _) => {
                h3418
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3420 = match v1.clone() {
        Adt1::C0(h3419, _, _, _, _, _, _, _, _, _, _, _) => {
            h3419
        }
    };
    let a3421 = 1u64;
    if if if if if if if if if if if nat_le(a3420, a3421) {
        let a3423 = match v2.clone() {
            Adt2::C0(h3422, _, _, _, _, _, _, _, _, _, _) => {
                h3422
            }
        };
        let a3424 = 1u64;
        nat_le(a3423, a3424)
    } else {
        false
    } {
        let a3426 = match v3.clone() {
            Adt2::C0(h3425, _, _, _, _, _, _, _, _, _, _) => {
                h3425
            }
        };
        let a3427 = 0u64;
        nat_eq(a3426, a3427)
    } else {
        false
    } {
        let a3432 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3428, _, _, _, _, _) => {
                h3428
            }
        };
        let a3433 = {
            let a3430 = match v2.clone() {
                Adt2::C0(h3429, _, _, _, _, _, _, _, _, _, _) => {
                    h3429
                }
            };
            let a3431 = 0u64;
            if nat_eq(a3430, a3431) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3432, a3433)
    } else {
        false
    } {
        f38(v3.clone())?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f217(v4.clone(), match v2.clone() {
            Adt2::C0(_, h3434, _, _, _, _, _, _, _, _, _) => {
                h3434
            }
        }, match v3.clone() {
            Adt2::C0(_, h3435, _, _, _, _, _, _, _, _, _) => {
                h3435
            }
        })?
    } else {
        false
    } {
        if v0 {
            f217(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3436, _, _, _, _, _, _, _, _) => {
                    h3436
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h3437, _, _, _, _, _, _, _, _) => {
                    h3437
                }
            })?
        } else {
            let a3440 = match v2.clone() {
                Adt2::C0(_, _, h3438, _, _, _, _, _, _, _, _) => {
                    h3438
                }
            };
            let a3441 = match v3.clone() {
                Adt2::C0(_, _, h3439, _, _, _, _, _, _, _, _) => {
                    h3439
                }
            };
            nat_eq(a3440, a3441)
        }
    } else {
        false
    } {
        let a3444 = match v2.clone() {
            Adt2::C0(_, _, _, h3442, _, _, _, _, _, _, _) => {
                h3442
            }
        };
        let a3445 = match v3.clone() {
            Adt2::C0(_, _, _, h3443, _, _, _, _, _, _, _) => {
                h3443
            }
        };
        nat_eq(a3444, a3445)
    } else {
        false
    } {
        let a3448 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3446, _, _, _, _, _, _) => {
                h3446
            }
        };
        let a3449 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3447, _, _, _, _, _, _) => {
                h3447
            }
        };
        nat_eq(a3448, a3449)
    } else {
        false
    } {
        let a3454 = {
            let a3452 = match v4.clone() {
                Adt6::C0(h3450, _, _, _) => {
                    h3450
                }
            };
            let a3453 = match v5.clone() {
                Adt6::C0(h3451, _, _, _) => {
                    h3451
                }
            };
            nat_mul(a3452, a3453)?
        };
        let a3455 = 2147483648u64;
        nat_lt(a3454, a3455)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a3457 = match v1 {
                Adt1::C0(h3456, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3456
                }
            };
            let a3458 = 0u64;
            nat_eq(a3457, a3458)
        }, match v2.clone() {
            Adt2::C0(h3459, _, _, _, _, _, _, _, _, _, _) => {
                h3459
            }
        });
        Ok(Adt10::C4({
            let a3460 = f23(v3.clone(), 1u64)?;
            let a3461 = f184(v6.clone())?;
            nat_mul(a3460, a3461)?
        }, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3462 = v3;
    match m3462 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3464 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3463, _, _, _, _, _) => {
                        h3463
                    }
                };
                let a3465 = 1u64;
                nat_eq(a3464, a3465)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h3466, _, _, _, _, _, _, _, _, _) => {
                    h3466
                }
            }, match v1.clone() {
                Adt1::C0(h3467, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3467
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3468, _, _, _, _, _, _, _, _, _) => {
                    h3468
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h3469, _, _, _, _, _, _, _) => {
                    h3469
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h3470, _, _, _, _, _, _, _, _) => {
                        h3470
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h3471, _, _, _, _, _, _, _, _, _, _) => {
                        h3471
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3472, _, _, _, _, _, _, _, _) => {
                        h3472
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3473, _, _, _, _, _, _) => {
                        h3473
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3474, _, _, _, _, _, _, _) => {
                        h3474
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h3475, _, _, _, _, _, _, _, _) => {
                        h3475
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h3476, _, _, _, _, _, _) => {
                        h3476
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3477, _, _, _, _, _, _, _) => {
                        h3477
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h3478, _, _, _, _, _, _, _, _) => {
                        h3478
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h3479, _, _, _, _, _, _, _, _, _, _) => {
                    h3479
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h3480, _, _, _, _, _, _, _, _, _, _) => {
                    h3480
                }
            };
            let v15: u64 = {
                let a3481 = v13;
                let a3482 = 0u64;
                if nat_eq(a3481, a3482) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a3484 = match v5.clone() {
                    Adt2::C0(_, h3483, _, _, _, _, _, _, _, _, _) => {
                        h3483
                    }
                };
                let a3485 = v15;
                nat_mul(a3484, a3485)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3486, _, _, _) => {
                        h3486
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3487, _, _, _, _) => {
                        h3487
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3488, _, _) => {
                        h3488
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3489, _, _, _) => {
                        h3489
                    }
                }
            };
            let a3491 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h3490, _, _, _, _, _) => {
                    h3490
                }
            };
            let a3492 = 1u64;
            if if if if if if if if if if nat_le(a3491, a3492) {
                let a3493 = v13;
                let a3494 = 0u64;
                if nat_eq(a3493, a3494) {
                    true
                } else {
                    let a3495 = v13;
                    let a3496 = 1u64;
                    if nat_eq(a3495, a3496) {
                        let a3497 = v14;
                        let a3498 = 1u64;
                        nat_eq(a3497, a3498)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3499 = v14;
                let a3500 = 0u64;
                if nat_eq(a3499, a3500) {
                    true
                } else {
                    let a3501 = v14;
                    let a3502 = 1u64;
                    nat_eq(a3501, a3502)
                }
            } else {
                false
            } {
                let a3504 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3503, _, _, _, _, _) => {
                        h3503
                    }
                };
                let a3505 = v15;
                nat_eq(a3504, a3505)
            } else {
                false
            } {
                let a3513 = match v4.clone() {
                    Adt2::C0(_, h3506, _, _, _, _, _, _, _, _, _) => {
                        h3506
                    }
                };
                let a3514 = {
                    let a3511 = v10;
                    let a3512 = {
                        let a3509 = match v7.clone() {
                            Adt6::C0(h3507, _, _, _) => {
                                h3507
                            }
                        };
                        let a3510 = match v8.clone() {
                            Adt6::C0(h3508, _, _, _) => {
                                h3508
                            }
                        };
                        nat_mul(a3509, a3510)?
                    };
                    nat_mul(a3511, a3512)?
                };
                nat_eq(a3513, a3514)
            } else {
                false
            } {
                let a3517 = match v4.clone() {
                    Adt2::C0(_, _, h3515, _, _, _, _, _, _, _, _) => {
                        h3515
                    }
                };
                let a3518 = f92(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h3516, _, _, _, _, _, _, _, _, _) => {
                        h3516
                    }
                })?;
                nat_eq(a3517, a3518)
            } else {
                false
            } {
                let a3520 = 0u64;
                let a3521 = match v4.clone() {
                    Adt2::C0(_, _, h3519, _, _, _, _, _, _, _, _) => {
                        h3519
                    }
                };
                nat_lt(a3520, a3521)
            } else {
                false
            } {
                if v6 {
                    let a3523 = match v2 {
                        Adt2::C0(_, _, _, h3522, _, _, _, _, _, _, _) => {
                            h3522
                        }
                    };
                    let a3524 = v10;
                    if if if nat_eq(a3523, a3524) {
                        let a3526 = match v4.clone() {
                            Adt2::C0(_, _, _, h3525, _, _, _, _, _, _, _) => {
                                h3525
                            }
                        };
                        let a3527 = f92(v8.clone(), v12)?;
                        nat_eq(a3526, a3527)
                    } else {
                        false
                    } {
                        let a3529 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3528, _, _, _, _, _, _) => {
                                h3528
                            }
                        };
                        let a3530 = v11;
                        nat_eq(a3529, a3530)
                    } else {
                        false
                    } {
                        let a3532 = 0u64;
                        let a3533 = match v4.clone() {
                            Adt2::C0(_, _, _, h3531, _, _, _, _, _, _, _) => {
                                h3531
                            }
                        };
                        nat_lt(a3532, a3533)
                    } else {
                        false
                    }
                } else {
                    let a3535 = match v2 {
                        Adt2::C0(_, _, h3534, _, _, _, _, _, _, _, _) => {
                            h3534
                        }
                    };
                    let a3536 = v10;
                    if if if nat_eq(a3535, a3536) {
                        let a3538 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h3537, _, _, _, _, _, _) => {
                                h3537
                            }
                        };
                        let a3539 = 1u64;
                        nat_eq(a3538, a3539)
                    } else {
                        false
                    } {
                        let a3541 = match v4.clone() {
                            Adt2::C0(_, _, _, h3540, _, _, _, _, _, _, _) => {
                                h3540
                            }
                        };
                        let a3542 = v11;
                        nat_eq(a3541, a3542)
                    } else {
                        false
                    } {
                        let a3544 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3543, _, _, _, _, _, _) => {
                                h3543
                            }
                        };
                        let a3545 = 1u64;
                        nat_eq(a3544, a3545)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f48(match v5.clone() {
                    Adt2::C0(_, h3546, _, _, _, _, _, _, _, _, _) => {
                        h3546
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a3566 = {
                        let a3563 = {
                            let a3561 = {
                                let a3555 = {
                                    let a3549 = {
                                        let a3547 = v11;
                                        let a3548 = 1u64;
                                        nat_sub(a3547, a3548)
                                    };
                                    let a3550 = v18;
                                    nat_mul(a3549, a3550)?
                                };
                                let a3556 = {
                                    let a3553 = {
                                        let a3551 = v10;
                                        let a3552 = 1u64;
                                        nat_sub(a3551, a3552)
                                    };
                                    let a3554 = v17;
                                    nat_mul(a3553, a3554)?
                                };
                                nat_add(a3555, a3556)?
                            };
                            let a3562 = {
                                let a3559 = {
                                    let a3557 = v12;
                                    let a3558 = 1u64;
                                    nat_sub(a3557, a3558)
                                };
                                let a3560 = v16;
                                nat_mul(a3559, a3560)?
                            };
                            nat_add(a3561, a3562)?
                        };
                        let a3564 = v16;
                        nat_add(a3563, a3564)?
                    };
                    let a3567 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3565, _) => {
                            h3565
                        }
                    };
                    nat_le(a3566, a3567)
                }
            } else {
                false
            } {
                f30(v5.clone(), v4.clone())?
            } else {
                false
            } {
                let v19: Adt3 = if v6 {
                    f25(v4.clone(), 1u64)
                } else {
                    Adt3::C0(match v4.clone() {
                        Adt2::C0(_, h3568, _, _, _, _, _, _, _, _, _) => {
                            h3568
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3569, _, _, _, _, _, _, _, _) => {
                            h3569
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h3570, _, _, _, _, _, _, _) => {
                            h3570
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h3571, _, _, _, _, _) => {
                            h3571
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3572, _, _, _, _) => {
                            h3572
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3573, _, _, _) => {
                            h3573
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a3580 = f23(v5.clone(), 1u64)?;
                        let a3581 = {
                            let a3578 = {
                                let a3576 = match v7.clone() {
                                    Adt6::C0(h3574, _, _, _) => {
                                        h3574
                                    }
                                };
                                let a3577 = match v8.clone() {
                                    Adt6::C0(h3575, _, _, _) => {
                                        h3575
                                    }
                                };
                                nat_mul(a3576, a3577)?
                            };
                            let a3579 = 2u64;
                            nat_add(a3578, a3579)?
                        };
                        nat_mul(a3580, a3581)?
                    }, 4u64, v19, f37(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f23(v4, 1u64)?, {
                        let a3582 = v14;
                        let a3583 = 0u64;
                        if nat_eq(a3582, a3583) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h3584, _, _, _, _, _, _, _, _, _) => {
                            h3584
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f115(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3585 = v2;
    match m3585 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3586, _, _, _, _, _, _, _, _, _) => {
                    h3586
                }
            }, match v0.clone() {
                Adt1::C0(h3587, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3587
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3588, _, _, _, _, _, _, _, _) => {
                    h3588
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3589, _, _, _, _, _) => {
                    h3589
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3590, _, _, _, _, _, _, _, _) => {
                    h3590
                }
            }, match v0.clone() {
                Adt1::C0(_, h3591, _, _, _, _, _, _, _, _, _, _) => {
                    h3591
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3592, _, _, _, _, _, _, _) => {
                    h3592
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3593, _, _, _, _) => {
                    h3593
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3594, _, _, _, _, _, _, _) => {
                    h3594
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3595, _, _, _, _, _, _, _, _, _) => {
                    h3595
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3596, _, _, _, _, _, _) => {
                    h3596
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3597, _, _, _) => {
                    h3597
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3598, _, _) => {
                    h3598
                }
            };
            let v9: u64 = f92(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3599, _, _, _, _, _, _, _) => {
                    h3599
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3600, _, _, _, _, _, _, _, _, _, _) => {
                    h3600
                }
            };
            let v11: u64 = {
                let a3608 = v8;
                let a3609 = {
                    let a3606 = match v5.clone() {
                        Adt6::C0(h3601, _, _, _) => {
                            h3601
                        }
                    };
                    let a3607 = {
                        let a3604 = match v6.clone() {
                            Adt6::C0(h3602, _, _, _) => {
                                h3602
                            }
                        };
                        let a3605 = match v7.clone() {
                            Adt6::C0(h3603, _, _, _) => {
                                h3603
                            }
                        };
                        nat_mul(a3604, a3605)?
                    };
                    nat_mul(a3606, a3607)?
                };
                nat_mul(a3608, a3609)?
            };
            let a3611 = match v4.clone() {
                Adt2::C0(h3610, _, _, _, _, _, _, _, _, _, _) => {
                    h3610
                }
            };
            let a3612 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3611, a3612) {
                let a3613 = v10;
                let a3614 = 0u64;
                if nat_eq(a3613, a3614) {
                    true
                } else {
                    let a3615 = v10;
                    let a3616 = 1u64;
                    nat_eq(a3615, a3616)
                }
            } else {
                false
            } {
                let a3617 = 0u64;
                let a3618 = v8;
                nat_lt(a3617, a3618)
            } else {
                false
            } {
                let a3619 = v8;
                let a3620 = 2147483648u64;
                nat_lt(a3619, a3620)
            } else {
                false
            } {
                let a3622 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3621, _, _, _, _, _) => {
                        h3621
                    }
                };
                let a3623 = 4u64;
                nat_eq(a3622, a3623)
            } else {
                false
            } {
                let a3624 = 0u64;
                let a3625 = v9;
                nat_lt(a3624, a3625)
            } else {
                false
            } {
                let a3627 = 0u64;
                let a3628 = match v3.clone() {
                    Adt2::C0(_, _, h3626, _, _, _, _, _, _, _, _) => {
                        h3626
                    }
                };
                nat_lt(a3627, a3628)
            } else {
                false
            } {
                let a3630 = 0u64;
                let a3631 = match v3.clone() {
                    Adt2::C0(_, _, _, h3629, _, _, _, _, _, _, _) => {
                        h3629
                    }
                };
                nat_lt(a3630, a3631)
            } else {
                false
            } {
                let a3633 = match v3.clone() {
                    Adt2::C0(_, h3632, _, _, _, _, _, _, _, _, _) => {
                        h3632
                    }
                };
                let a3634 = v11;
                nat_eq(a3633, a3634)
            } else {
                false
            } {
                let a3637 = match v3.clone() {
                    Adt2::C0(_, _, h3635, _, _, _, _, _, _, _, _) => {
                        h3635
                    }
                };
                let a3638 = f92(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3636, _, _, _, _, _, _, _, _, _) => {
                        h3636
                    }
                })?;
                nat_eq(a3637, a3638)
            } else {
                false
            } {
                let a3641 = match v3.clone() {
                    Adt2::C0(_, _, _, h3639, _, _, _, _, _, _, _) => {
                        h3639
                    }
                };
                let a3642 = f92(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3640, _, _, _, _, _, _, _, _) => {
                        h3640
                    }
                })?;
                nat_eq(a3641, a3642)
            } else {
                false
            } {
                let a3650 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3643, _, _, _, _, _, _) => {
                        h3643
                    }
                };
                let a3651 = {
                    let a3648 = v9;
                    let a3649 = {
                        let a3645 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3644, _, _, _, _, _, _) => {
                                h3644
                            }
                        };
                        let a3646 = v8;
                        let a3647 = 0u64;
                        nat_quot(a3645, a3646, a3647)
                    };
                    nat_mul(a3648, a3649)?
                };
                nat_eq(a3650, a3651)
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3652 = v10;
                    let a3653 = 0u64;
                    if nat_eq(a3652, a3653) {
                        4u64
                    } else {
                        2u64
                    }
                }, f25(v4.clone(), 1u64), f25(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3654, _, _) => {
                        h3654
                    }
                }, {
                    let a3656 = v8;
                    let a3657 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3655, _, _) => {
                            h3655
                        }
                    };
                    nat_mul(a3656, a3657)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f116(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3658 = v1;
    match m3658 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f218(v0, v3, v2)
        }
    }
}

pub fn f117(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3660 = match v0.clone() {
        Adt2::C0(h3659, _, _, _, _, _, _, _, _, _, _) => {
            h3659
        }
    };
    let a3661 = 1u64;
    if if nat_eq(a3660, a3661) {
        true
    } else {
        let a3663 = match v0.clone() {
            Adt2::C0(h3662, _, _, _, _, _, _, _, _, _, _) => {
                h3662
            }
        };
        let a3664 = 30u64;
        nat_eq(a3663, a3664)
    } {
        let a3666 = match v1.clone() {
            Adt2::C0(h3665, _, _, _, _, _, _, _, _, _, _) => {
                h3665
            }
        };
        let a3667 = 1u64;
        if if if if if if if if if if if nat_eq(a3666, a3667) {
            let a3670 = match v0.clone() {
                Adt2::C0(_, h3668, _, _, _, _, _, _, _, _, _) => {
                    h3668
                }
            };
            let a3671 = match v1.clone() {
                Adt2::C0(_, h3669, _, _, _, _, _, _, _, _, _) => {
                    h3669
                }
            };
            nat_eq(a3670, a3671)
        } else {
            false
        } {
            let a3673 = 0u64;
            let a3674 = match v1.clone() {
                Adt2::C0(_, _, h3672, _, _, _, _, _, _, _, _) => {
                    h3672
                }
            };
            nat_lt(a3673, a3674)
        } else {
            false
        } {
            let a3677 = match v1.clone() {
                Adt2::C0(_, _, h3675, _, _, _, _, _, _, _, _) => {
                    h3675
                }
            };
            let a3678 = match v1.clone() {
                Adt2::C0(_, _, _, h3676, _, _, _, _, _, _, _) => {
                    h3676
                }
            };
            nat_eq(a3677, a3678)
        } else {
            false
        } {
            let a3680 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3679, _, _, _, _, _, _) => {
                    h3679
                }
            };
            let a3681 = 1u64;
            nat_eq(a3680, a3681)
        } else {
            false
        } {
            let a3689 = match v0.clone() {
                Adt2::C0(_, _, h3682, _, _, _, _, _, _, _, _) => {
                    h3682
                }
            };
            let a3690 = {
                let a3687 = {
                    let a3685 = match v1.clone() {
                        Adt2::C0(_, _, h3683, _, _, _, _, _, _, _, _) => {
                            h3683
                        }
                    };
                    let a3686 = match v1.clone() {
                        Adt2::C0(_, _, h3684, _, _, _, _, _, _, _, _) => {
                            h3684
                        }
                    };
                    nat_add(a3685, a3686)?
                };
                let a3688 = 1u64;
                nat_sub(a3687, a3688)
            };
            nat_eq(a3689, a3690)
        } else {
            false
        } {
            let a3692 = 0u64;
            let a3693 = match v0.clone() {
                Adt2::C0(_, _, _, h3691, _, _, _, _, _, _, _) => {
                    h3691
                }
            };
            nat_lt(a3692, a3693)
        } else {
            false
        } {
            let a3695 = 0u64;
            let a3696 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3694, _, _, _, _, _, _) => {
                    h3694
                }
            };
            nat_lt(a3695, a3696)
        } else {
            false
        } {
            let a3698 = 2u64;
            let a3699 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3697, _, _, _, _, _) => {
                    h3697
                }
            };
            nat_le(a3698, a3699)
        } else {
            false
        } {
            let a3701 = 2u64;
            let a3702 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3700, _, _, _, _, _) => {
                    h3700
                }
            };
            nat_le(a3701, a3702)
        } else {
            false
        } {
            f30(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f23(v1.clone(), 1u64)?, 2u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f118(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3703 = v1;
    match m3703 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f219(v0, v3, v2)
        }
    }
}

pub fn f119(v0: u64) -> bool {
    let a3704 = v0;
    let a3705 = 1u64;
    if if nat_eq(a3704, a3705) {
        true
    } else {
        let a3706 = v0;
        let a3707 = 33u64;
        nat_eq(a3706, a3707)
    } {
        true
    } else {
        let a3708 = v0;
        let a3709 = 34u64;
        nat_eq(a3708, a3709)
    }
}

pub fn f120(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f47(v1, v2.clone()) {
        let a3710 = v3;
        let a3711 = f23(v2.clone(), 1u64)?;
        nat_eq(a3710, a3711)
    } else {
        false
    } {
        f26(v0.clone(), v3)?
    } else {
        false
    } {
        f26(v2.clone(), v3)?
    } else {
        false
    } {
        let a3712 = v3;
        let a3713 = 0u64;
        if nat_eq(a3712, a3713) {
            true
        } else {
            let a3715 = 4u64;
            let a3716 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3714, _, _, _, _, _) => {
                    h3714
                }
            };
            nat_le(a3715, a3716)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3717 = 2u64;
            let a3718 = v3;
            nat_mul(a3717, a3718)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f121(v0: u64) -> bool {
    if f220(v0) {
        !f221(v0)
    } else {
        false
    }
}

pub fn f122(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3719 = v1;
    match m3719 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3721 = f213(match v0.clone() {
                Adt2::C0(h3720, _, _, _, _, _, _, _, _, _, _) => {
                    h3720
                }
            });
            match m3721 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3722, _) => {
                            h3722
                        }
                    };
                    if if if if if if f47(v3, v2.clone()) {
                        f30(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3723 = f23(v0.clone(), 1u64)?;
                        let a3724 = f23(v2.clone(), 1u64)?;
                        nat_eq(a3723, a3724)
                    } else {
                        false
                    } {
                        let a3729 = {
                            let a3726 = match v0.clone() {
                                Adt2::C0(_, h3725, _, _, _, _, _, _, _, _, _) => {
                                    h3725
                                }
                            };
                            let a3727 = v5;
                            let a3728 = 1u64;
                            nat_rem(a3726, a3727, a3728)
                        };
                        let a3730 = 0u64;
                        if nat_eq(a3729, a3730) {
                            let a3733 = match v4 {
                                Adt19::C0(_, _, h3731) => {
                                    h3731
                                }
                            };
                            let a3734 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3732, _, _, _, _, _) => {
                                    h3732
                                }
                            };
                            nat_le(a3733, a3734)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3735 = v5;
                        let a3736 = 1u64;
                        if nat_eq(a3735, a3736) {
                            let a3738 = 4u64;
                            let a3739 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3737, _, _, _, _, _) => {
                                    h3737
                                }
                            };
                            nat_le(a3738, a3739)
                        } else {
                            let a3741 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3740, _, _, _, _, _) => {
                                    h3740
                                }
                            };
                            let a3742 = 4u64;
                            nat_eq(a3741, a3742)
                        }
                    } else {
                        false
                    } {
                        let a3747 = {
                            let a3744 = match v2.clone() {
                                Adt2::C0(_, h3743, _, _, _, _, _, _, _, _, _) => {
                                    h3743
                                }
                            };
                            let a3745 = v5;
                            let a3746 = 1u64;
                            nat_rem(a3744, a3745, a3746)
                        };
                        let a3748 = 0u64;
                        if nat_eq(a3747, a3748) {
                            true
                        } else {
                            f38(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f25(v0.clone(), v5), f25(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3749, _, _, _, _, _, _, _, _, _, _) => {
                                h3749
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

pub fn f123(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3752 = match v1.clone() {
        Adt2::C0(h3750, _, _, _, _, _, _, _, _, _, _) => {
            h3750
        }
    };
    let a3753 = match v3.clone() {
        Adt2::C0(h3751, _, _, _, _, _, _, _, _, _, _) => {
            h3751
        }
    };
    if nat_eq(a3752, a3753) {
        f222(v0, v1, v2, v3)
    } else {
        if if if if f221(match v1.clone() {
            Adt2::C0(h3754, _, _, _, _, _, _, _, _, _, _) => {
                h3754
            }
        }) {
            f221(match v3.clone() {
                Adt2::C0(h3755, _, _, _, _, _, _, _, _, _, _) => {
                    h3755
                }
            })
        } else {
            false
        } {
            let a3756 = v0;
            let a3757 = 33u64;
            if nat_eq(a3756, a3757) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3758 = f23(v1.clone(), 1u64)?;
            let a3759 = f23(v3.clone(), 1u64)?;
            nat_eq(a3758, a3759)
        } else {
            false
        } {
            let m3761 = f213(match v3.clone() {
                Adt2::C0(h3760, _, _, _, _, _, _, _, _, _, _) => {
                    h3760
                }
            });
            match m3761 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3762, _, _, _, _, _, _, _, _, _, _) => {
                            h3762
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3763, _, _, _, _, _, _, _, _, _, _) => {
                            h3763
                        }
                    };
                    let v7: bool = {
                        let a3764 = v5;
                        let a3765 = 43u64;
                        if nat_eq(a3764, a3765) {
                            true
                        } else {
                            let a3766 = v6;
                            let a3767 = 43u64;
                            nat_eq(a3766, a3767)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3768 = f23(v3.clone(), 1u64)?;
                        let a3769 = if v7 {
                            f164(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3768, a3769)?
                    }, match v4 {
                        Adt19::C0(_, _, h3770) => {
                            h3770
                        }
                    }, f25(v1, 1u64), f25(v3, 1u64), if v7 {
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

pub fn f124(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3772 = match v0.clone() {
        Adt2::C0(h3771, _, _, _, _, _, _, _, _, _, _) => {
            h3771
        }
    };
    let a3773 = 0u64;
    if if if if nat_eq(a3772, a3773) {
        true
    } else {
        let a3775 = match v0.clone() {
            Adt2::C0(h3774, _, _, _, _, _, _, _, _, _, _) => {
                h3774
            }
        };
        let a3776 = 1u64;
        nat_eq(a3775, a3776)
    } {
        true
    } else {
        let a3778 = match v0.clone() {
            Adt2::C0(h3777, _, _, _, _, _, _, _, _, _, _) => {
                h3777
            }
        };
        let a3779 = 25u64;
        if nat_eq(a3778, a3779) {
            true
        } else {
            let a3781 = match v0.clone() {
                Adt2::C0(h3780, _, _, _, _, _, _, _, _, _, _) => {
                    h3780
                }
            };
            let a3782 = 26u64;
            nat_eq(a3781, a3782)
        }
    } {
        true
    } else {
        let a3784 = match v0.clone() {
            Adt2::C0(h3783, _, _, _, _, _, _, _, _, _, _) => {
                h3783
            }
        };
        let a3785 = 30u64;
        nat_eq(a3784, a3785)
    } {
        let a3788 = match v0.clone() {
            Adt2::C0(h3786, _, _, _, _, _, _, _, _, _, _) => {
                h3786
            }
        };
        let a3789 = match v1.clone() {
            Adt2::C0(h3787, _, _, _, _, _, _, _, _, _, _) => {
                h3787
            }
        };
        if if nat_eq(a3788, a3789) {
            f223(match v1.clone() {
                Adt2::C0(_, h3790, _, _, _, _, _, _, _, _, _) => {
                    h3790
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3791, _, _, _, _, _, _, _, _) => {
                    h3791
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3792, _, _, _, _, _, _, _) => {
                    h3792
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3793, _, _, _, _, _, _) => {
                    h3793
                }
            }, match v0.clone() {
                Adt2::C0(_, h3794, _, _, _, _, _, _, _, _, _) => {
                    h3794
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3795, _, _, _, _, _, _, _, _) => {
                    h3795
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3796, _, _, _, _, _, _, _) => {
                    h3796
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3797, _, _, _, _, _, _) => {
                    h3797
                }
            })
        } else {
            false
        } {
            let m3799 = f213(match v0.clone() {
                Adt2::C0(h3798, _, _, _, _, _, _, _, _, _, _) => {
                    h3798
                }
            });
            match m3799 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3800) => {
                            h3800
                        }
                    }, f25(v0, 1u64), f25(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f125(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3801 = v2;
    match m3801 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3802 = v0;
            let a3803 = 4u64;
            if if if nat_lt(a3802, a3803) {
                let a3806 = match v1.clone() {
                    Adt2::C0(h3804, _, _, _, _, _, _, _, _, _, _) => {
                        h3804
                    }
                };
                let a3807 = match v4.clone() {
                    Adt2::C0(h3805, _, _, _, _, _, _, _, _, _, _) => {
                        h3805
                    }
                };
                if nat_eq(a3806, a3807) {
                    let a3810 = match v1.clone() {
                        Adt2::C0(h3808, _, _, _, _, _, _, _, _, _, _) => {
                            h3808
                        }
                    };
                    let a3811 = match v3.clone() {
                        Adt2::C0(h3809, _, _, _, _, _, _, _, _, _, _) => {
                            h3809
                        }
                    };
                    nat_eq(a3810, a3811)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f224(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3812, _, _, _, _, _, _, _, _, _) => {
                        h3812
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3813, _, _, _, _, _, _, _, _, _) => {
                        h3813
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3814, _, _, _, _, _, _, _, _, _) => {
                        h3814
                    }
                })? {
                    f224(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3815, _, _, _, _, _, _, _, _) => {
                            h3815
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3816, _, _, _, _, _, _, _, _) => {
                            h3816
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3817, _, _, _, _, _, _, _, _) => {
                            h3817
                        }
                    })?
                } else {
                    false
                } {
                    f224(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3818, _, _, _, _, _, _, _) => {
                            h3818
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3819, _, _, _, _, _, _, _) => {
                            h3819
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3820, _, _, _, _, _, _, _) => {
                            h3820
                        }
                    })?
                } else {
                    false
                } {
                    f224(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3821, _, _, _, _, _, _) => {
                            h3821
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3822, _, _, _, _, _, _) => {
                            h3822
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3823, _, _, _, _, _, _) => {
                            h3823
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3825 = f213(match v1.clone() {
                    Adt2::C0(h3824, _, _, _, _, _, _, _, _, _, _) => {
                        h3824
                    }
                });
                match m3825 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3826, _) => {
                                h3826
                            }
                        };
                        let a3831 = {
                            let a3828 = match v1.clone() {
                                Adt2::C0(_, h3827, _, _, _, _, _, _, _, _, _) => {
                                    h3827
                                }
                            };
                            let a3829 = v6;
                            let a3830 = 1u64;
                            nat_rem(a3828, a3829, a3830)
                        };
                        let a3832 = 0u64;
                        if if if nat_eq(a3831, a3832) {
                            let a3837 = {
                                let a3834 = match v4.clone() {
                                    Adt2::C0(_, h3833, _, _, _, _, _, _, _, _, _) => {
                                        h3833
                                    }
                                };
                                let a3835 = v6;
                                let a3836 = 1u64;
                                nat_rem(a3834, a3835, a3836)
                            };
                            let a3838 = 0u64;
                            nat_eq(a3837, a3838)
                        } else {
                            false
                        } {
                            let a3843 = {
                                let a3840 = match v3.clone() {
                                    Adt2::C0(_, h3839, _, _, _, _, _, _, _, _, _) => {
                                        h3839
                                    }
                                };
                                let a3841 = v6;
                                let a3842 = 1u64;
                                nat_rem(a3840, a3841, a3842)
                            };
                            let a3844 = 0u64;
                            nat_eq(a3843, a3844)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3845) => {
                                    h3845
                                }
                            }, f25(v1, v6), f25(v3, v6), Adt4::C2(v0, f25(v4, v6))))
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

pub fn f126(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3847 = match v2.clone() {
        Adt2::C0(h3846, _, _, _, _, _, _, _, _, _, _) => {
            h3846
        }
    };
    let a3848 = 0u64;
    if if nat_eq(a3847, a3848) {
        let a3850 = match v3.clone() {
            Adt2::C0(h3849, _, _, _, _, _, _, _, _, _, _) => {
                h3849
            }
        };
        let a3851 = 0u64;
        nat_eq(a3850, a3851)
    } else {
        false
    } {
        let a3852 = v0;
        let a3853 = 61u64;
        if nat_eq(a3852, a3853) {
            let v4: bool = {
                let a3855 = 0u64;
                let a3856 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3854, _, _, _) => {
                        h3854
                    }
                };
                nat_lt(a3855, a3856)
            };
            if if if if if if f225(match v2.clone() {
                Adt2::C0(_, h3857, _, _, _, _, _, _, _, _, _) => {
                    h3857
                }
            }, match v1.clone() {
                Adt1::C0(h3858, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3858
                }
            }, match v1.clone() {
                Adt1::C0(_, h3859, _, _, _, _, _, _, _, _, _, _) => {
                    h3859
                }
            }, match v3.clone() {
                Adt2::C0(_, h3860, _, _, _, _, _, _, _, _, _) => {
                    h3860
                }
            })? {
                f225(match v2.clone() {
                    Adt2::C0(_, _, h3861, _, _, _, _, _, _, _, _) => {
                        h3861
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3862, _, _, _, _, _, _, _, _, _) => {
                        h3862
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3863, _, _, _, _, _, _, _, _) => {
                        h3863
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3864, _, _, _, _, _, _, _, _) => {
                        h3864
                    }
                })?
            } else {
                false
            } {
                f225(match v2.clone() {
                    Adt2::C0(_, _, _, h3865, _, _, _, _, _, _, _) => {
                        h3865
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3866, _, _, _, _, _, _, _) => {
                        h3866
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3867, _, _, _, _, _, _) => {
                        h3867
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3868, _, _, _, _, _, _, _) => {
                        h3868
                    }
                })?
            } else {
                false
            } {
                f225(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3869, _, _, _, _, _, _) => {
                        h3869
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3870, _, _, _, _, _) => {
                        h3870
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3871, _, _, _, _) => {
                        h3871
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3872, _, _, _, _, _, _) => {
                        h3872
                    }
                })?
            } else {
                false
            } {
                let a3874 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3873, _, _, _) => {
                        h3873
                    }
                };
                let a3875 = 4294967295u64;
                nat_le(a3874, a3875)
            } else {
                false
            } {
                if v4 {
                    !f48(match v2.clone() {
                        Adt2::C0(_, h3876, _, _, _, _, _, _, _, _, _) => {
                            h3876
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3877, _, _, _, _, _, _, _, _) => {
                            h3877
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3878, _, _, _, _, _, _, _) => {
                            h3878
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3879, _, _, _, _, _, _) => {
                            h3879
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3880, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3880
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3881, _, _, _, _, _, _, _, _, _) => {
                        h3881
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3882, _, _, _, _, _, _, _) => {
                        h3882
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3883, _, _, _, _, _) => {
                        h3883
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3884 = v0;
            let a3885 = 62u64;
            if nat_eq(a3884, a3885) {
                let a3887 = match v1.clone() {
                    Adt1::C0(h3886, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3886
                    }
                };
                let a3888 = 2147483648u64;
                if if if if if nat_lt(a3887, a3888) {
                    let a3890 = match v1.clone() {
                        Adt1::C0(_, h3889, _, _, _, _, _, _, _, _, _, _) => {
                            h3889
                        }
                    };
                    let a3891 = 2147483648u64;
                    nat_lt(a3890, a3891)
                } else {
                    false
                } {
                    let a3894 = match v1.clone() {
                        Adt1::C0(h3892, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3892
                        }
                    };
                    let a3895 = match v2.clone() {
                        Adt2::C0(_, h3893, _, _, _, _, _, _, _, _, _) => {
                            h3893
                        }
                    };
                    nat_lt(a3894, a3895)
                } else {
                    false
                } {
                    let a3898 = match v1.clone() {
                        Adt1::C0(_, h3896, _, _, _, _, _, _, _, _, _, _) => {
                            h3896
                        }
                    };
                    let a3899 = match v2.clone() {
                        Adt2::C0(_, h3897, _, _, _, _, _, _, _, _, _) => {
                            h3897
                        }
                    };
                    nat_lt(a3898, a3899)
                } else {
                    false
                } {
                    let a3902 = match v2.clone() {
                        Adt2::C0(_, _, h3900, _, _, _, _, _, _, _, _) => {
                            h3900
                        }
                    };
                    let a3903 = match v3.clone() {
                        Adt2::C0(_, _, h3901, _, _, _, _, _, _, _, _) => {
                            h3901
                        }
                    };
                    if if if nat_eq(a3902, a3903) {
                        let a3906 = match v2.clone() {
                            Adt2::C0(_, _, _, h3904, _, _, _, _, _, _, _) => {
                                h3904
                            }
                        };
                        let a3907 = match v3.clone() {
                            Adt2::C0(_, _, _, h3905, _, _, _, _, _, _, _) => {
                                h3905
                            }
                        };
                        nat_eq(a3906, a3907)
                    } else {
                        false
                    } {
                        let a3910 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3908, _, _, _, _, _, _) => {
                                h3908
                            }
                        };
                        let a3911 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3909, _, _, _, _, _, _) => {
                                h3909
                            }
                        };
                        nat_eq(a3910, a3911)
                    } else {
                        false
                    } {
                        let a3920 = match v3.clone() {
                            Adt2::C0(_, h3912, _, _, _, _, _, _, _, _, _) => {
                                h3912
                            }
                        };
                        let a3921 = {
                            let a3918 = {
                                let a3915 = match v2.clone() {
                                    Adt2::C0(_, h3913, _, _, _, _, _, _, _, _, _) => {
                                        h3913
                                    }
                                };
                                let a3916 = match v1.clone() {
                                    Adt1::C0(h3914, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3914
                                    }
                                };
                                nat_add(a3915, a3916)?
                            };
                            let a3919 = match v1.clone() {
                                Adt1::C0(_, h3917, _, _, _, _, _, _, _, _, _, _) => {
                                    h3917
                                }
                            };
                            nat_add(a3918, a3919)?
                        };
                        nat_eq(a3920, a3921)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3922, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3922
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3923 = v0;
                let a3924 = 63u64;
                if nat_eq(a3923, a3924) {
                    if if if if if f226(match v2.clone() {
                        Adt2::C0(_, h3925, _, _, _, _, _, _, _, _, _) => {
                            h3925
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3926, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3926
                        }
                    })? {
                        f226(match v2.clone() {
                            Adt2::C0(_, _, h3927, _, _, _, _, _, _, _, _) => {
                                h3927
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3928, _, _, _, _, _, _, _, _, _, _) => {
                                h3928
                            }
                        })?
                    } else {
                        false
                    } {
                        f226(match v2.clone() {
                            Adt2::C0(_, _, _, h3929, _, _, _, _, _, _, _) => {
                                h3929
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3930, _, _, _, _, _, _, _, _, _) => {
                                h3930
                            }
                        })?
                    } else {
                        false
                    } {
                        f226(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3931, _, _, _, _, _, _) => {
                                h3931
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3932, _, _, _, _, _, _, _, _) => {
                                h3932
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3935 = match v2.clone() {
                            Adt2::C0(_, h3933, _, _, _, _, _, _, _, _, _) => {
                                h3933
                            }
                        };
                        let a3936 = match v3.clone() {
                            Adt2::C0(_, h3934, _, _, _, _, _, _, _, _, _) => {
                                h3934
                            }
                        };
                        if if if nat_eq(a3935, a3936) {
                            let a3939 = match v2.clone() {
                                Adt2::C0(_, _, h3937, _, _, _, _, _, _, _, _) => {
                                    h3937
                                }
                            };
                            let a3940 = match v3.clone() {
                                Adt2::C0(_, _, h3938, _, _, _, _, _, _, _, _) => {
                                    h3938
                                }
                            };
                            nat_eq(a3939, a3940)
                        } else {
                            false
                        } {
                            let a3943 = match v2.clone() {
                                Adt2::C0(_, _, _, h3941, _, _, _, _, _, _, _) => {
                                    h3941
                                }
                            };
                            let a3944 = match v3.clone() {
                                Adt2::C0(_, _, _, h3942, _, _, _, _, _, _, _) => {
                                    h3942
                                }
                            };
                            nat_eq(a3943, a3944)
                        } else {
                            false
                        } {
                            let a3947 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3945, _, _, _, _, _, _) => {
                                    h3945
                                }
                            };
                            let a3948 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3946, _, _, _, _, _, _) => {
                                    h3946
                                }
                            };
                            nat_eq(a3947, a3948)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3949, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3949
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3950, _, _, _, _, _, _, _, _, _, _) => {
                                h3950
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3951, _, _, _, _, _, _, _, _, _) => {
                                h3951
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3952, _, _, _, _, _, _, _, _) => {
                                h3952
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

pub fn f127(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3953 = v3;
    match m3953 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3954 = v0;
                let a3955 = 4u64;
                nat_eq(a3954, a3955)
            };
            let v7: bool = {
                let a3957 = 0u64;
                let a3958 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3956, _, _, _, _, _, _, _) => {
                        h3956
                    }
                };
                nat_lt(a3957, a3958)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3959, _, _, _, _, _, _, _, _, _) => {
                    h3959
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3960, _, _, _, _, _, _, _, _) => {
                    h3960
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3961, _, _, _, _, _, _, _) => {
                    h3961
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3962, _, _, _, _, _, _) => {
                    h3962
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3963, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3963
                }
            }, match v1.clone() {
                Adt1::C0(_, h3964, _, _, _, _, _, _, _, _, _, _) => {
                    h3964
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3965, _, _, _, _, _, _, _, _, _) => {
                    h3965
                }
            });
            if if if if if if if if if if if if v6 {
                let a3967 = match v2.clone() {
                    Adt2::C0(h3966, _, _, _, _, _, _, _, _, _, _) => {
                        h3966
                    }
                };
                let a3968 = 0u64;
                if nat_eq(a3967, a3968) {
                    let a3970 = match v5.clone() {
                        Adt2::C0(h3969, _, _, _, _, _, _, _, _, _, _) => {
                            h3969
                        }
                    };
                    let a3971 = 0u64;
                    nat_eq(a3970, a3971)
                } else {
                    false
                }
            } else {
                let a3973 = match v2.clone() {
                    Adt2::C0(h3972, _, _, _, _, _, _, _, _, _, _) => {
                        h3972
                    }
                };
                let a3974 = 0u64;
                if if nat_eq(a3973, a3974) {
                    true
                } else {
                    let a3976 = match v2.clone() {
                        Adt2::C0(h3975, _, _, _, _, _, _, _, _, _, _) => {
                            h3975
                        }
                    };
                    let a3977 = 26u64;
                    nat_eq(a3976, a3977)
                } {
                    let a3979 = match v5.clone() {
                        Adt2::C0(h3978, _, _, _, _, _, _, _, _, _, _) => {
                            h3978
                        }
                    };
                    let a3980 = 0u64;
                    if nat_eq(a3979, a3980) {
                        true
                    } else {
                        let a3982 = match v5.clone() {
                            Adt2::C0(h3981, _, _, _, _, _, _, _, _, _, _) => {
                                h3981
                            }
                        };
                        let a3983 = 26u64;
                        nat_eq(a3982, a3983)
                    }
                } else {
                    false
                }
            } {
                let a3986 = match v2.clone() {
                    Adt2::C0(h3984, _, _, _, _, _, _, _, _, _, _) => {
                        h3984
                    }
                };
                let a3987 = match v4.clone() {
                    Adt2::C0(h3985, _, _, _, _, _, _, _, _, _, _) => {
                        h3985
                    }
                };
                nat_eq(a3986, a3987)
            } else {
                false
            } {
                f21(v2.clone(), v4.clone())
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a3989 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3988, _, _, _, _, _) => {
                        h3988
                    }
                };
                let a3990 = 4u64;
                nat_eq(a3989, a3990)
            } else {
                false
            } {
                let a3991 = v8;
                let a3992 = f23(v2.clone(), 1u64)?;
                nat_le(a3991, a3992)
            } else {
                false
            } {
                let a3994 = match v1.clone() {
                    Adt1::C0(_, _, _, h3993, _, _, _, _, _, _, _, _) => {
                        h3993
                    }
                };
                let a3995 = 1073741824u64;
                nat_lt(a3994, a3995)
            } else {
                false
            } {
                let a3997 = match v1.clone() {
                    Adt1::C0(h3996, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3996
                    }
                };
                let a3998 = 4294967295u64;
                if if if if nat_le(a3997, a3998) {
                    let a4000 = match v1.clone() {
                        Adt1::C0(_, h3999, _, _, _, _, _, _, _, _, _, _) => {
                            h3999
                        }
                    };
                    let a4001 = 4294967295u64;
                    nat_le(a4000, a4001)
                } else {
                    false
                } {
                    let a4003 = match v1.clone() {
                        Adt1::C0(_, _, h4002, _, _, _, _, _, _, _, _, _) => {
                            h4002
                        }
                    };
                    let a4004 = 4294967295u64;
                    nat_le(a4003, a4004)
                } else {
                    false
                } {
                    let a4006 = match v1.clone() {
                        Adt1::C0(_, _, _, h4005, _, _, _, _, _, _, _, _) => {
                            h4005
                        }
                    };
                    let a4007 = 4294967295u64;
                    nat_le(a4006, a4007)
                } else {
                    false
                } {
                    let a4009 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4008, _, _, _, _, _, _, _) => {
                            h4008
                        }
                    };
                    let a4010 = 4294967295u64;
                    nat_le(a4009, a4010)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4013 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4011) => {
                            h4011
                        }
                    };
                    let a4014 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4012) => {
                            h4012
                        }
                    };
                    nat_eq(a4013, a4014)
                } else {
                    true
                }
            } else {
                false
            } {
                f227(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4015, _, _, _, _, _, _, _, _) => {
                        h4015
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4016, _) => {
                        h4016
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f23(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a4017 = v10;
                    let a4018 = v8;
                    nat_add(a4017, a4018)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4019, _, _, _, _, _, _, _, _) => {
                        h4019
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f128(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4021 = match v0.clone() {
        Adt2::C0(h4020, _, _, _, _, _, _, _, _, _, _) => {
            h4020
        }
    };
    let a4022 = 0u64;
    if if if nat_eq(a4021, a4022) {
        let a4024 = match v1.clone() {
            Adt2::C0(h4023, _, _, _, _, _, _, _, _, _, _) => {
                h4023
            }
        };
        let a4025 = 0u64;
        nat_eq(a4024, a4025)
    } else {
        false
    } {
        f223(match v0.clone() {
            Adt2::C0(_, h4026, _, _, _, _, _, _, _, _, _) => {
                h4026
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4027, _, _, _, _, _, _, _, _) => {
                h4027
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4028, _, _, _, _, _, _, _) => {
                h4028
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4029, _, _, _, _, _, _) => {
                h4029
            }
        }, match v1.clone() {
            Adt2::C0(_, h4030, _, _, _, _, _, _, _, _, _) => {
                h4030
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4031, _, _, _, _, _, _, _, _) => {
                h4031
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4032, _, _, _, _, _, _, _) => {
                h4032
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4033, _, _, _, _, _, _) => {
                h4033
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4034 = v2;
            let a4035 = f23(v0.clone(), 1u64)?;
            nat_add(a4034, a4035)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f129(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4037 = match v1.clone() {
        Adt2::C0(h4036, _, _, _, _, _, _, _, _, _, _) => {
            h4036
        }
    };
    let a4038 = 0u64;
    if if if nat_eq(a4037, a4038) {
        true
    } else {
        let a4040 = match v1.clone() {
            Adt2::C0(h4039, _, _, _, _, _, _, _, _, _, _) => {
                h4039
            }
        };
        let a4041 = 1u64;
        nat_eq(a4040, a4041)
    } {
        true
    } else {
        let a4043 = match v1.clone() {
            Adt2::C0(h4042, _, _, _, _, _, _, _, _, _, _) => {
                h4042
            }
        };
        let a4044 = 30u64;
        nat_eq(a4043, a4044)
    } {
        let a4047 = match v1.clone() {
            Adt2::C0(h4045, _, _, _, _, _, _, _, _, _, _) => {
                h4045
            }
        };
        let a4048 = match v2.clone() {
            Adt2::C0(h4046, _, _, _, _, _, _, _, _, _, _) => {
                h4046
            }
        };
        if if if if if nat_eq(a4047, a4048) {
            let a4051 = match v1.clone() {
                Adt2::C0(_, h4049, _, _, _, _, _, _, _, _, _) => {
                    h4049
                }
            };
            let a4052 = match v2.clone() {
                Adt2::C0(_, h4050, _, _, _, _, _, _, _, _, _) => {
                    h4050
                }
            };
            nat_eq(a4051, a4052)
        } else {
            false
        } {
            let a4055 = match v1.clone() {
                Adt2::C0(_, _, _, h4053, _, _, _, _, _, _, _) => {
                    h4053
                }
            };
            let a4056 = match v2.clone() {
                Adt2::C0(_, _, _, h4054, _, _, _, _, _, _, _) => {
                    h4054
                }
            };
            nat_eq(a4055, a4056)
        } else {
            false
        } {
            let a4059 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4057, _, _, _, _, _, _) => {
                    h4057
                }
            };
            let a4060 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4058, _, _, _, _, _, _) => {
                    h4058
                }
            };
            nat_eq(a4059, a4060)
        } else {
            false
        } {
            let a4061 = v0;
            let a4062 = 4u64;
            if nat_eq(a4061, a4062) {
                let a4064 = match v1.clone() {
                    Adt2::C0(_, _, h4063, _, _, _, _, _, _, _, _) => {
                        h4063
                    }
                };
                let a4065 = 1u64;
                if nat_eq(a4064, a4065) {
                    let a4068 = match v1.clone() {
                        Adt2::C0(_, h4066, _, _, _, _, _, _, _, _, _) => {
                            h4066
                        }
                    };
                    let a4069 = match v2.clone() {
                        Adt2::C0(_, _, h4067, _, _, _, _, _, _, _, _) => {
                            h4067
                        }
                    };
                    nat_eq(a4068, a4069)
                } else {
                    false
                }
            } else {
                let a4070 = v0;
                let a4071 = 4u64;
                if if nat_lt(a4070, a4071) {
                    let a4074 = match v1.clone() {
                        Adt2::C0(_, h4072, _, _, _, _, _, _, _, _, _) => {
                            h4072
                        }
                    };
                    let a4075 = match v1.clone() {
                        Adt2::C0(_, _, h4073, _, _, _, _, _, _, _, _) => {
                            h4073
                        }
                    };
                    nat_eq(a4074, a4075)
                } else {
                    false
                } {
                    let a4078 = match v1.clone() {
                        Adt2::C0(_, _, h4076, _, _, _, _, _, _, _, _) => {
                            h4076
                        }
                    };
                    let a4079 = match v2.clone() {
                        Adt2::C0(_, _, h4077, _, _, _, _, _, _, _, _) => {
                            h4077
                        }
                    };
                    nat_eq(a4078, a4079)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4081 = f213(match v1.clone() {
                Adt2::C0(h4080, _, _, _, _, _, _, _, _, _, _) => {
                    h4080
                }
            });
            match m4081 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4082) => {
                            h4082
                        }
                    }, f25(v1, 1u64), f25(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f130(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f228(match v1.clone() {
        Adt2::C0(h4083, _, _, _, _, _, _, _, _, _, _) => {
            h4083
        }
    }, v0) {
        let a4086 = match v1.clone() {
            Adt2::C0(h4084, _, _, _, _, _, _, _, _, _, _) => {
                h4084
            }
        };
        let a4087 = match v2.clone() {
            Adt2::C0(h4085, _, _, _, _, _, _, _, _, _, _) => {
                h4085
            }
        };
        nat_eq(a4086, a4087)
    } else {
        false
    } {
        let a4090 = match v1.clone() {
            Adt2::C0(_, h4088, _, _, _, _, _, _, _, _, _) => {
                h4088
            }
        };
        let a4091 = match v2.clone() {
            Adt2::C0(_, h4089, _, _, _, _, _, _, _, _, _) => {
                h4089
            }
        };
        if if if nat_eq(a4090, a4091) {
            let a4094 = match v1.clone() {
                Adt2::C0(_, _, h4092, _, _, _, _, _, _, _, _) => {
                    h4092
                }
            };
            let a4095 = match v2.clone() {
                Adt2::C0(_, _, h4093, _, _, _, _, _, _, _, _) => {
                    h4093
                }
            };
            nat_eq(a4094, a4095)
        } else {
            false
        } {
            let a4098 = match v1.clone() {
                Adt2::C0(_, _, _, h4096, _, _, _, _, _, _, _) => {
                    h4096
                }
            };
            let a4099 = match v2.clone() {
                Adt2::C0(_, _, _, h4097, _, _, _, _, _, _, _) => {
                    h4097
                }
            };
            nat_eq(a4098, a4099)
        } else {
            false
        } {
            let a4102 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4100, _, _, _, _, _, _) => {
                    h4100
                }
            };
            let a4103 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4101, _, _, _, _, _, _) => {
                    h4101
                }
            };
            nat_eq(a4102, a4103)
        } else {
            false
        }
    } else {
        false
    } {
        let m4105 = f213(match v1.clone() {
            Adt2::C0(h4104, _, _, _, _, _, _, _, _, _, _) => {
                h4104
            }
        });
        match m4105 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4106) => {
                        h4106
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4107, _, _, _, _, _, _, _, _, _, _) => {
                        h4107
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f131(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4108, _, _, _, _, _, _, _, _, _, _, _) => {
                h4108
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4109, _, _, _, _, _, _, _, _, _) => {
                h4109
            }
        }
    };
    let a4110 = 0u64;
    let a4111 = v4;
    if if nat_lt(a4110, a4111) {
        let a4112 = v4;
        let a4113 = 2147483648u64;
        nat_lt(a4112, a4113)
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
        let v7: u64 = f229(match v5.clone() {
            Adt2::C0(_, _, h4114, _, _, _, _, _, _, _, _) => {
                h4114
            }
        }, v4)?;
        let v8: u64 = f229(match v5.clone() {
            Adt2::C0(_, _, _, h4115, _, _, _, _, _, _, _) => {
                h4115
            }
        }, v4)?;
        let a4117 = match v5.clone() {
            Adt2::C0(h4116, _, _, _, _, _, _, _, _, _, _) => {
                h4116
            }
        };
        let a4118 = 0u64;
        if if if if if nat_eq(a4117, a4118) {
            let a4120 = match v6.clone() {
                Adt2::C0(h4119, _, _, _, _, _, _, _, _, _, _) => {
                    h4119
                }
            };
            let a4121 = 0u64;
            nat_eq(a4120, a4121)
        } else {
            false
        } {
            let a4123 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4122, _, _, _, _, _, _) => {
                    h4122
                }
            };
            let a4124 = 1u64;
            nat_eq(a4123, a4124)
        } else {
            false
        } {
            let a4127 = match v5.clone() {
                Adt2::C0(_, h4125, _, _, _, _, _, _, _, _, _) => {
                    h4125
                }
            };
            let a4128 = match v6.clone() {
                Adt2::C0(_, h4126, _, _, _, _, _, _, _, _, _) => {
                    h4126
                }
            };
            nat_eq(a4127, a4128)
        } else {
            false
        } {
            if v0 {
                let a4130 = match v5.clone() {
                    Adt2::C0(_, _, h4129, _, _, _, _, _, _, _, _) => {
                        h4129
                    }
                };
                let a4131 = 2147483648u64;
                if if if nat_lt(a4130, a4131) {
                    let a4133 = match v5.clone() {
                        Adt2::C0(_, _, _, h4132, _, _, _, _, _, _, _) => {
                            h4132
                        }
                    };
                    let a4134 = 2147483648u64;
                    nat_lt(a4133, a4134)
                } else {
                    false
                } {
                    let a4137 = {
                        let a4135 = v7;
                        let a4136 = v8;
                        nat_mul(a4135, a4136)?
                    };
                    let a4138 = 2147483648u64;
                    nat_le(a4137, a4138)
                } else {
                    false
                } {
                    f230(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4140 = match v1.clone() {
                    Adt1::C0(h4139, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4139
                    }
                };
                let a4141 = v7;
                if if if if nat_eq(a4140, a4141) {
                    let a4143 = match v1 {
                        Adt1::C0(_, h4142, _, _, _, _, _, _, _, _, _, _) => {
                            h4142
                        }
                    };
                    let a4144 = v8;
                    nat_eq(a4143, a4144)
                } else {
                    false
                } {
                    f231(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4146 = match v6.clone() {
                        Adt2::C0(_, _, h4145, _, _, _, _, _, _, _, _) => {
                            h4145
                        }
                    };
                    let a4147 = v4;
                    if nat_eq(a4146, a4147) {
                        let a4149 = match v6.clone() {
                            Adt2::C0(_, _, _, h4148, _, _, _, _, _, _, _) => {
                                h4148
                            }
                        };
                        let a4150 = v4;
                        nat_eq(a4149, a4150)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4154 = match v6 {
                        Adt2::C0(_, _, _, _, h4151, _, _, _, _, _, _) => {
                            h4151
                        }
                    };
                    let a4155 = {
                        let a4152 = v7;
                        let a4153 = v8;
                        nat_mul(a4152, a4153)?
                    };
                    nat_eq(a4154, a4155)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f132(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4157 = match v2.clone() {
        Adt2::C0(h4156, _, _, _, _, _, _, _, _, _, _) => {
            h4156
        }
    };
    let a4158 = 0u64;
    if if if if if nat_eq(a4157, a4158) {
        true
    } else {
        let a4160 = match v2.clone() {
            Adt2::C0(h4159, _, _, _, _, _, _, _, _, _, _) => {
                h4159
            }
        };
        let a4161 = 1u64;
        nat_eq(a4160, a4161)
    } {
        true
    } else {
        let a4163 = match v2.clone() {
            Adt2::C0(h4162, _, _, _, _, _, _, _, _, _, _) => {
                h4162
            }
        };
        let a4164 = 30u64;
        nat_eq(a4163, a4164)
    } {
        let a4167 = match v2.clone() {
            Adt2::C0(h4165, _, _, _, _, _, _, _, _, _, _) => {
                h4165
            }
        };
        let a4168 = match v3.clone() {
            Adt2::C0(h4166, _, _, _, _, _, _, _, _, _, _) => {
                h4166
            }
        };
        nat_eq(a4167, a4168)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a4170 = match v1.clone() {
                Adt1::C0(h4169, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4169
                }
            };
            let a4171 = 4294967296u64;
            nat_lt(a4170, a4171)
        } else {
            false
        } {
            let a4173 = match v1.clone() {
                Adt1::C0(_, h4172, _, _, _, _, _, _, _, _, _, _) => {
                    h4172
                }
            };
            let a4174 = 4294967296u64;
            nat_lt(a4173, a4174)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4176 = match v2.clone() {
                Adt2::C0(h4175, _, _, _, _, _, _, _, _, _, _) => {
                    h4175
                }
            };
            let a4177 = 0u64;
            if nat_eq(a4176, a4177) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a4178 = v0;
            let a4179 = 31u64;
            if nat_eq(a4178, a4179) {
                0u64
            } else {
                let a4180 = v0;
                let a4181 = 47u64;
                if nat_eq(a4180, a4181) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4182, _, _, _, _, _, _, _, _, _, _) => {
                h4182
            }
        }, match v1.clone() {
            Adt1::C0(h4183, _, _, _, _, _, _, _, _, _, _, _) => {
                h4183
            }
        }, match v1 {
            Adt1::C0(_, h4184, _, _, _, _, _, _, _, _, _, _) => {
                h4184
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f133(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4186 = match v0.clone() {
        Adt2::C0(h4185, _, _, _, _, _, _, _, _, _, _) => {
            h4185
        }
    };
    let a4187 = 0u64;
    if if if nat_eq(a4186, a4187) {
        let a4189 = match v1.clone() {
            Adt2::C0(h4188, _, _, _, _, _, _, _, _, _, _) => {
                h4188
            }
        };
        let a4190 = 0u64;
        nat_eq(a4189, a4190)
    } else {
        false
    } {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v1.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f134(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4192 = match v0.clone() {
        Adt2::C0(h4191, _, _, _, _, _, _, _, _, _, _) => {
            h4191
        }
    };
    let a4193 = 0u64;
    if if nat_eq(a4192, a4193) {
        let a4195 = match v1.clone() {
            Adt2::C0(h4194, _, _, _, _, _, _, _, _, _, _) => {
                h4194
            }
        };
        let a4196 = 26u64;
        nat_eq(a4195, a4196)
    } else {
        false
    } {
        let a4198 = match v0.clone() {
            Adt2::C0(_, _, _, h4197, _, _, _, _, _, _, _) => {
                h4197
            }
        };
        let a4199 = 1u64;
        if if if if if if if nat_eq(a4198, a4199) {
            let a4201 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4200, _, _, _, _, _, _) => {
                    h4200
                }
            };
            let a4202 = 1u64;
            nat_eq(a4201, a4202)
        } else {
            false
        } {
            let a4204 = match v0.clone() {
                Adt2::C0(_, h4203, _, _, _, _, _, _, _, _, _) => {
                    h4203
                }
            };
            let a4205 = 2147483647u64;
            nat_le(a4204, a4205)
        } else {
            false
        } {
            let a4208 = match v1.clone() {
                Adt2::C0(_, h4206, _, _, _, _, _, _, _, _, _) => {
                    h4206
                }
            };
            let a4209 = match v0.clone() {
                Adt2::C0(_, _, h4207, _, _, _, _, _, _, _, _) => {
                    h4207
                }
            };
            nat_eq(a4208, a4209)
        } else {
            false
        } {
            let a4211 = match v1.clone() {
                Adt2::C0(_, _, h4210, _, _, _, _, _, _, _, _) => {
                    h4210
                }
            };
            let a4212 = 1u64;
            nat_eq(a4211, a4212)
        } else {
            false
        } {
            let a4214 = match v1.clone() {
                Adt2::C0(_, _, _, h4213, _, _, _, _, _, _, _) => {
                    h4213
                }
            };
            let a4215 = 1u64;
            nat_eq(a4214, a4215)
        } else {
            false
        } {
            let a4217 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4216, _, _, _, _, _, _) => {
                    h4216
                }
            };
            let a4218 = 1u64;
            nat_eq(a4217, a4218)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4221 = match v0.clone() {
                    Adt2::C0(_, _, h4219, _, _, _, _, _, _, _, _) => {
                        h4219
                    }
                };
                let a4222 = f173(match v0.clone() {
                    Adt2::C0(_, h4220, _, _, _, _, _, _, _, _, _) => {
                        h4220
                    }
                })?;
                nat_mul(a4221, a4222)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f135(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4223 = v1;
    match m4223 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f178(f25(v0.clone(), 1u64))?;
            let a4225 = match v0.clone() {
                Adt2::C0(h4224, _, _, _, _, _, _, _, _, _, _) => {
                    h4224
                }
            };
            let a4226 = 26u64;
            if if if nat_eq(a4225, a4226) {
                let a4228 = match v3.clone() {
                    Adt2::C0(h4227, _, _, _, _, _, _, _, _, _, _) => {
                        h4227
                    }
                };
                let a4229 = 26u64;
                nat_eq(a4228, a4229)
            } else {
                false
            } {
                let a4231 = match v2.clone() {
                    Adt2::C0(h4230, _, _, _, _, _, _, _, _, _, _) => {
                        h4230
                    }
                };
                let a4232 = 27u64;
                nat_eq(a4231, a4232)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a4234 = match v2.clone() {
                        Adt2::C0(_, h4233, _, _, _, _, _, _, _, _, _) => {
                            h4233
                        }
                    };
                    let a4235 = 1u64;
                    nat_eq(a4234, a4235)
                } else {
                    false
                } {
                    let a4237 = match v2.clone() {
                        Adt2::C0(_, _, h4236, _, _, _, _, _, _, _, _) => {
                            h4236
                        }
                    };
                    let a4238 = 1u64;
                    nat_eq(a4237, a4238)
                } else {
                    false
                } {
                    let a4240 = match v2.clone() {
                        Adt2::C0(_, _, _, h4239, _, _, _, _, _, _, _) => {
                            h4239
                        }
                    };
                    let a4241 = 1u64;
                    nat_eq(a4240, a4241)
                } else {
                    false
                } {
                    let a4243 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4242, _, _, _, _, _, _) => {
                            h4242
                        }
                    };
                    let a4244 = 1u64;
                    nat_eq(a4243, a4244)
                } else {
                    false
                } {
                    let a4245 = v4;
                    let a4246 = 9223372036854775808u64;
                    nat_lt(a4245, a4246)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4249 = 2u64;
                        let a4250 = {
                            let a4247 = v4;
                            let a4248 = 2u64;
                            nat_mul(a4247, a4248)?
                        };
                        nat_add(a4249, a4250)?
                    }, 8u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C18(v4, f25(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f136(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4251, _, _, _, _, _, _, _, _, _, _) => {
            h4251
        }
    };
    let a4252 = v3;
    let a4253 = 0u64;
    if if nat_eq(a4252, a4253) {
        true
    } else {
        let a4254 = v0;
        let a4255 = 13u64;
        if nat_eq(a4254, a4255) {
            let a4256 = v3;
            let a4257 = 1u64;
            if nat_eq(a4256, a4257) {
                true
            } else {
                let a4258 = v3;
                let a4259 = 30u64;
                nat_eq(a4258, a4259)
            }
        } else {
            false
        }
    } {
        let a4261 = v3;
        let a4262 = match v2.clone() {
            Adt2::C0(h4260, _, _, _, _, _, _, _, _, _, _) => {
                h4260
            }
        };
        if if nat_eq(a4261, a4262) {
            let a4263 = v0;
            let a4264 = 13u64;
            if nat_eq(a4263, a4264) {
                let a4266 = match v2.clone() {
                    Adt2::C0(_, h4265, _, _, _, _, _, _, _, _, _) => {
                        h4265
                    }
                };
                let a4267 = 1u64;
                if if if nat_eq(a4266, a4267) {
                    let a4269 = match v2.clone() {
                        Adt2::C0(_, _, h4268, _, _, _, _, _, _, _, _) => {
                            h4268
                        }
                    };
                    let a4270 = 1u64;
                    nat_eq(a4269, a4270)
                } else {
                    false
                } {
                    let a4272 = match v2.clone() {
                        Adt2::C0(_, _, _, h4271, _, _, _, _, _, _, _) => {
                            h4271
                        }
                    };
                    let a4273 = 1u64;
                    nat_eq(a4272, a4273)
                } else {
                    false
                } {
                    let a4275 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4274, _, _, _, _, _, _) => {
                            h4274
                        }
                    };
                    let a4276 = 1u64;
                    nat_eq(a4275, a4276)
                } else {
                    false
                }
            } else {
                let a4278 = match v2.clone() {
                    Adt2::C0(_, h4277, _, _, _, _, _, _, _, _, _) => {
                        h4277
                    }
                };
                let a4279 = 1u64;
                if if if nat_eq(a4278, a4279) {
                    let a4282 = match v1.clone() {
                        Adt2::C0(_, _, h4280, _, _, _, _, _, _, _, _) => {
                            h4280
                        }
                    };
                    let a4283 = match v2.clone() {
                        Adt2::C0(_, _, h4281, _, _, _, _, _, _, _, _) => {
                            h4281
                        }
                    };
                    nat_eq(a4282, a4283)
                } else {
                    false
                } {
                    let a4286 = match v1.clone() {
                        Adt2::C0(_, _, _, h4284, _, _, _, _, _, _, _) => {
                            h4284
                        }
                    };
                    let a4287 = match v2.clone() {
                        Adt2::C0(_, _, _, h4285, _, _, _, _, _, _, _) => {
                            h4285
                        }
                    };
                    nat_eq(a4286, a4287)
                } else {
                    false
                } {
                    let a4290 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4288, _, _, _, _, _, _) => {
                            h4288
                        }
                    };
                    let a4291 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4289, _, _, _, _, _, _) => {
                            h4289
                        }
                    };
                    nat_eq(a4290, a4291)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt10::C4(f232(v4.clone(), v0, v3)?, {
                let a4292 = v3;
                let a4293 = 0u64;
                if nat_eq(a4292, a4293) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f25(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f137(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4294 = v0;
    let a4295 = 89u64;
    if if nat_eq(a4294, a4295) {
        let a4296 = v1;
        let a4297 = 16u64;
        nat_eq(a4296, a4297)
    } else {
        false
    } {
        Ok(Some(24u64))
    } else {
        let a4298 = v0;
        let a4299 = 89u64;
        if if nat_eq(a4298, a4299) {
            let a4300 = v1;
            let a4301 = 14u64;
            nat_eq(a4300, a4301)
        } else {
            false
        } {
            Ok(Some(23u64))
        } else {
            let a4302 = v0;
            let a4303 = 89u64;
            if if nat_eq(a4302, a4303) {
                let a4304 = v1;
                let a4305 = 15u64;
                nat_eq(a4304, a4305)
            } else {
                false
            } {
                Ok(Some(22u64))
            } else {
                let a4306 = v0;
                let a4307 = 89u64;
                if if nat_eq(a4306, a4307) {
                    let a4308 = 8u64;
                    let a4309 = v1;
                    if nat_le(a4308, a4309) {
                        let a4310 = v1;
                        let a4311 = 9u64;
                        nat_le(a4310, a4311)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some({
                        let a4312 = v1;
                        let a4313 = 12u64;
                        nat_add(a4312, a4313)?
                    }))
                } else {
                    let a4314 = v0;
                    let a4315 = 89u64;
                    if if nat_eq(a4314, a4315) {
                        let a4316 = v1;
                        let a4317 = 4u64;
                        nat_eq(a4316, a4317)
                    } else {
                        false
                    } {
                        Ok(Some(19u64))
                    } else {
                        let a4318 = 10u64;
                        let a4319 = v0;
                        if if nat_le(a4318, a4319) {
                            let a4320 = v0;
                            let a4321 = 12u64;
                            nat_le(a4320, a4321)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4322 = v0;
                                let a4323 = 3u64;
                                nat_add(a4322, a4323)?
                            }))
                        } else {
                            let a4324 = v0;
                            let a4325 = 89u64;
                            if if nat_eq(a4324, a4325) {
                                let a4326 = v1;
                                let a4327 = 7u64;
                                nat_eq(a4326, a4327)
                            } else {
                                false
                            } {
                                Ok(Some(17u64))
                            } else {
                                let a4328 = v0;
                                let a4329 = 89u64;
                                if if nat_eq(a4328, a4329) {
                                    let a4330 = v1;
                                    let a4331 = 10u64;
                                    nat_eq(a4330, a4331)
                                } else {
                                    false
                                } {
                                    Ok(Some(18u64))
                                } else {
                                    let a4332 = v0;
                                    let a4333 = 89u64;
                                    if if nat_eq(a4332, a4333) {
                                        let a4334 = v1;
                                        let a4335 = 13u64;
                                        nat_eq(a4334, a4335)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(16u64))
                                    } else {
                                        let a4336 = v0;
                                        let a4337 = 8u64;
                                        if nat_eq(a4336, a4337) {
                                            Ok(Some(5u64))
                                        } else {
                                            let a4338 = v0;
                                            let a4339 = 9u64;
                                            if nat_eq(a4338, a4339) {
                                                Ok(Some(6u64))
                                            } else {
                                                let a4340 = v0;
                                                let a4341 = 89u64;
                                                if nat_eq(a4340, a4341) {
                                                    let a4342 = v1;
                                                    let a4343 = 0u64;
                                                    if nat_eq(a4342, a4343) {
                                                        Ok(Some(0u64))
                                                    } else {
                                                        let a4344 = v1;
                                                        let a4345 = 1u64;
                                                        if nat_eq(a4344, a4345) {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            let a4346 = v1;
                                                            let a4347 = 2u64;
                                                            if nat_eq(a4346, a4347) {
                                                                Ok(Some(2u64))
                                                            } else {
                                                                let a4348 = v1;
                                                                let a4349 = 3u64;
                                                                if nat_eq(a4348, a4349) {
                                                                    Ok(Some(3u64))
                                                                } else {
                                                                    let a4350 = v1;
                                                                    let a4351 = 6u64;
                                                                    if nat_eq(a4350, a4351) {
                                                                        Ok(Some(4u64))
                                                                    } else {
                                                                        let a4352 = v1;
                                                                        let a4353 = 12u64;
                                                                        if nat_eq(a4352, a4353) {
                                                                            Ok(Some(7u64))
                                                                        } else {
                                                                            let a4354 = v1;
                                                                            let a4355 = 11u64;
                                                                            if nat_eq(a4354, a4355) {
                                                                                Ok(Some(8u64))
                                                                            } else {
                                                                                let a4356 = v1;
                                                                                let a4357 = 18u64;
                                                                                if nat_eq(a4356, a4357) {
                                                                                    Ok(Some(9u64))
                                                                                } else {
                                                                                    let a4358 = v1;
                                                                                    let a4359 = 19u64;
                                                                                    if nat_eq(a4358, a4359) {
                                                                                        Ok(Some(10u64))
                                                                                    } else {
                                                                                        let a4360 = v1;
                                                                                        let a4361 = 20u64;
                                                                                        if nat_eq(a4360, a4361) {
                                                                                            Ok(Some(11u64))
                                                                                        } else {
                                                                                            let a4362 = v1;
                                                                                            let a4363 = 21u64;
                                                                                            if nat_eq(a4362, a4363) {
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
}

pub fn f138(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4364 = f233(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4364 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4365 = v0;
            let a4366 = 2u64;
            if if if if nat_eq(a4365, a4366) {
                true
            } else {
                let a4367 = v0;
                let a4368 = 5u64;
                nat_eq(a4367, a4368)
            } {
                true
            } else {
                let a4369 = v0;
                let a4370 = 6u64;
                nat_eq(a4369, a4370)
            } {
                true
            } else {
                let a4371 = v0;
                let a4372 = 7u64;
                nat_eq(a4371, a4372)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4373 = v3;
                    match m4373 {
                        None => {
                            f25(v2, 1u64)
                        }
                        Some(v14) => {
                            f25(v14, 1u64)
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

pub fn f139(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4375 = match v1.clone() {
        Adt2::C0(h4374, _, _, _, _, _, _, _, _, _, _) => {
            h4374
        }
    };
    let a4376 = 0u64;
    if if if if if nat_eq(a4375, a4376) {
        true
    } else {
        let a4378 = match v1.clone() {
            Adt2::C0(h4377, _, _, _, _, _, _, _, _, _, _) => {
                h4377
            }
        };
        let a4379 = 1u64;
        nat_eq(a4378, a4379)
    } {
        true
    } else {
        let a4381 = match v1.clone() {
            Adt2::C0(h4380, _, _, _, _, _, _, _, _, _, _) => {
                h4380
            }
        };
        let a4382 = 30u64;
        nat_eq(a4381, a4382)
    } {
        let a4385 = match v1.clone() {
            Adt2::C0(h4383, _, _, _, _, _, _, _, _, _, _) => {
                h4383
            }
        };
        let a4386 = match v2.clone() {
            Adt2::C0(h4384, _, _, _, _, _, _, _, _, _, _) => {
                h4384
            }
        };
        if nat_eq(a4385, a4386) {
            true
        } else {
            let a4388 = match v2.clone() {
                Adt2::C0(h4387, _, _, _, _, _, _, _, _, _, _) => {
                    h4387
                }
            };
            let a4389 = 0u64;
            nat_eq(a4388, a4389)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4391 = f23(v2.clone(), 1u64)?;
            let a4392 = f234(v0, match v2.clone() {
                Adt2::C0(h4390, _, _, _, _, _, _, _, _, _, _) => {
                    h4390
                }
            });
            nat_mul(a4391, a4392)?
        }, {
            let a4394 = match v2.clone() {
                Adt2::C0(h4393, _, _, _, _, _, _, _, _, _, _) => {
                    h4393
                }
            };
            let a4395 = 0u64;
            if nat_eq(a4394, a4395) {
                4u64
            } else {
                2u64
            }
        }, f25(v1.clone(), 1u64), f25(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h4396, _, _, _, _, _, _, _, _, _, _) => {
                h4396
            }
        }, match v2 {
            Adt2::C0(h4397, _, _, _, _, _, _, _, _, _, _) => {
                h4397
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4410 = {
        let a4406 = {
            let a4402 = {
                let a4398 = v0;
                let a4399 = v4;
                nat_mul(a4398, a4399)?
            };
            let a4403 = {
                let a4400 = v1;
                let a4401 = v5;
                nat_mul(a4400, a4401)?
            };
            nat_add(a4402, a4403)?
        };
        let a4407 = {
            let a4404 = v2;
            let a4405 = v6;
            nat_mul(a4404, a4405)?
        };
        nat_add(a4406, a4407)?
    };
    let a4411 = {
        let a4408 = v3;
        let a4409 = v7;
        nat_mul(a4408, a4409)?
    };
    nat_add(a4410, a4411)
}

pub fn f141(v0: u64) -> u64 {
    let a4412 = v0;
    let a4413 = 2147483648u64;
    if nat_lt(a4412, a4413) {
        0u64
    } else {
        let a4414 = 4294967296u64;
        let a4415 = v0;
        nat_sub(a4414, a4415)
    }
}

pub fn f142(v0: u64) -> u64 {
    let a4416 = v0;
    let a4417 = 2147483648u64;
    if nat_lt(a4416, a4417) {
        v0
    } else {
        0u64
    }
}

pub fn f143(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4418 = v7;
        let a4419 = 6u64;
        if nat_eq(a4418, a4419) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f89(v3, v4), f89(v5, v6), f90(v7, v8), f172(v1), f172(v9))
}

pub fn f144(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4420 = v0;
    match m4420 {
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
        Adt4::C13(v55, v56, v57, v58) => {
            Adt11::C4(v55, v56, v57, v58)
        }
        Adt4::C14(v59, _, _) => {
            let a4421 = 13u64;
            let a4422 = v59;
            if nat_le(a4421, a4422) {
                Adt11::C7(v59, v1, v2)
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
        Adt4::C27(v79) => {
            Adt11::C2(match v79.clone() {
                Adt5::C0(_, _, _, _, _, h4423, _, _) => {
                    h4423
                }
            }, match v79 {
                Adt5::C0(_, _, _, _, _, _, _, h4424) => {
                    h4424
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
        Adt4::C40(_, _, _) => {
            Adt11::C6
        }
        Adt4::C41 => {
            Adt11::C6
        }
        Adt4::C42(_, _) => {
            Adt11::C6
        }
    }
}

pub fn f145(v0: Adt4) -> u64 {
    let m4425 = v0;
    match m4425 {
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
        Adt4::C14(v57, _, _) => {
            v57
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
        Adt4::C40(_, _, _) => {
            0u64
        }
        Adt4::C41 => {
            0u64
        }
        Adt4::C42(_, _) => {
            0u64
        }
    }
}

pub fn f146(v0: Adt4, v1: u64) -> u64 {
    let m4426 = v0;
    match m4426 {
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
        Adt4::C16(_, v62) => {
            v62
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v65, _) => {
            let a4427 = v1;
            let a4428 = 6u64;
            if nat_eq(a4427, a4428) {
                v65
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
        Adt4::C26(v79, _, _, _) => {
            let a4437 = v1;
            let a4438 = 6u64;
            if if if nat_eq(a4437, a4438) {
                true
            } else {
                let a4439 = v1;
                let a4440 = 7u64;
                nat_eq(a4439, a4440)
            } {
                let a4441 = v79;
                let a4442 = 30u64;
                nat_eq(a4441, a4442)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v78) => {
            match v78 {
                Adt5::C0(_, _, _, _, _, _, _, h4436) => {
                    h4436
                }
            }
        }
        Adt4::C28(v74) => {
            let a4429 = v1;
            let a4430 = 6u64;
            if nat_eq(a4429, a4430) {
                match v74 {
                    Adt7::C0(_, _, _, _, h4431) => {
                        h4431
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v77) => {
            let a4432 = v1;
            let a4433 = 6u64;
            if if nat_eq(a4432, a4433) {
                true
            } else {
                let a4434 = v1;
                let a4435 = 7u64;
                nat_eq(a4434, a4435)
            } {
                v77
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
        Adt4::C40(_, _, _) => {
            0u64
        }
        Adt4::C41 => {
            0u64
        }
        Adt4::C42(_, _) => {
            0u64
        }
    }
}

pub fn f147(v0: Adt4, v1: u64) -> Adt11 {
    let a4443 = v1;
    let a4444 = 4u64;
    if nat_lt(a4443, a4444) {
        Adt11::C0(v1)
    } else {
        let a4445 = v1;
        let a4446 = 4u64;
        if nat_eq(a4445, a4446) {
            Adt11::C1(0u64)
        } else {
            let a4447 = v1;
            let a4448 = 5u64;
            if nat_eq(a4447, a4448) {
                Adt11::C1(3u64)
            } else {
                let a4449 = v1;
                let a4450 = 6u64;
                if nat_eq(a4449, a4450) {
                    Adt11::C2(f146(v0, v1), 0u64, false)
                } else {
                    let a4451 = v1;
                    let a4452 = 7u64;
                    if nat_eq(a4451, a4452) {
                        Adt11::C2(0u64, f146(v0, v1), false)
                    } else {
                        let a4453 = v1;
                        let a4454 = 16u64;
                        if nat_lt(a4453, a4454) {
                            Adt11::C1({
                                let a4455 = v1;
                                let a4456 = 8u64;
                                nat_sub(a4455, a4456)
                            })
                        } else {
                            let a4457 = v1;
                            let a4458 = 20u64;
                            if nat_lt(a4457, a4458) {
                                Adt11::C5({
                                    let a4459 = v1;
                                    let a4460 = 16u64;
                                    nat_sub(a4459, a4460)
                                })
                            } else {
                                let a4461 = v1;
                                let a4462 = 20u64;
                                if nat_eq(a4461, a4462) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4463 = v1;
                                    let a4464 = 21u64;
                                    if nat_eq(a4463, a4464) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4465 = v1;
                                        let a4466 = 22u64;
                                        if nat_eq(a4465, a4466) {
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

pub fn f148(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4484 = {
        let a4479 = f235(v2, {
            let a4470 = {
                let a4467 = v0;
                let a4468 = v1;
                let a4469 = 0u64;
                nat_quot(a4467, a4468, a4469)
            };
            let a4471 = v2;
            let a4472 = 0u64;
            nat_rem(a4470, a4471, a4472)
        }, {
            let a4476 = {
                let a4473 = v0;
                let a4474 = v1;
                let a4475 = 0u64;
                nat_quot(a4473, a4474, a4475)
            };
            let a4477 = v2;
            let a4478 = 0u64;
            nat_quot(a4476, a4477, a4478)
        })?;
        let a4480 = v1;
        nat_mul(a4479, a4480)?
    };
    let a4485 = {
        let a4481 = v0;
        let a4482 = v1;
        let a4483 = 0u64;
        nat_rem(a4481, a4482, a4483)
    };
    nat_add(a4484, a4485)
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4487 = v0;
        let a4488 = match v2.clone() {
            Adt3::C0(h4486, _, _, _, _, _, _, _) => {
                h4486
            }
        };
        let a4489 = 0u64;
        nat_quot(a4487, a4488, a4489)
    };
    let v4: u64 = {
        let a4491 = v3;
        let a4492 = match v2.clone() {
            Adt3::C0(_, h4490, _, _, _, _, _, _) => {
                h4490
            }
        };
        let a4493 = 0u64;
        nat_quot(a4491, a4492, a4493)
    };
    f236({
        let a4495 = v0;
        let a4496 = match v2.clone() {
            Adt3::C0(h4494, _, _, _, _, _, _, _) => {
                h4494
            }
        };
        let a4497 = 0u64;
        nat_rem(a4495, a4496, a4497)
    }, {
        let a4499 = v3;
        let a4500 = match v2.clone() {
            Adt3::C0(_, h4498, _, _, _, _, _, _) => {
                h4498
            }
        };
        let a4501 = 0u64;
        nat_rem(a4499, a4500, a4501)
    }, {
        let a4503 = v4;
        let a4504 = match v2.clone() {
            Adt3::C0(_, _, h4502, _, _, _, _, _) => {
                h4502
            }
        };
        let a4505 = 0u64;
        nat_rem(a4503, a4504, a4505)
    }, {
        let a4507 = v4;
        let a4508 = match v2 {
            Adt3::C0(_, _, h4506, _, _, _, _, _) => {
                h4506
            }
        };
        let a4509 = 0u64;
        nat_quot(a4507, a4508, a4509)
    }, match v1.clone() {
        Adt3::C0(h4510, _, _, _, _, _, _, _) => {
            h4510
        }
    }, match v1.clone() {
        Adt3::C0(_, h4511, _, _, _, _, _, _) => {
            h4511
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4512, _, _, _, _, _) => {
            h4512
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4513, _, _, _, _) => {
            h4513
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4514, _, _, _) => {
            h4514
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4515, _, _) => {
            h4515
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4516, _) => {
            h4516
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4517) => {
            h4517
        }
    })
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4519 = v0;
        let a4520 = match v3.clone() {
            Adt3::C0(h4518, _, _, _, _, _, _, _) => {
                h4518
            }
        };
        let a4521 = 0u64;
        nat_quot(a4519, a4520, a4521)
    };
    let v6: u64 = {
        let a4523 = v5;
        let a4524 = match v3.clone() {
            Adt3::C0(_, h4522, _, _, _, _, _, _) => {
                h4522
            }
        };
        let a4525 = 0u64;
        nat_quot(a4523, a4524, a4525)
    };
    let v7: u64 = {
        let a4527 = v0;
        let a4528 = match v3.clone() {
            Adt3::C0(h4526, _, _, _, _, _, _, _) => {
                h4526
            }
        };
        let a4529 = 0u64;
        nat_rem(a4527, a4528, a4529)
    };
    let v8: u64 = {
        let a4531 = v5;
        let a4532 = match v3.clone() {
            Adt3::C0(_, h4530, _, _, _, _, _, _) => {
                h4530
            }
        };
        let a4533 = 0u64;
        nat_rem(a4531, a4532, a4533)
    };
    let v9: u64 = {
        let a4535 = v6;
        let a4536 = match v3.clone() {
            Adt3::C0(_, _, h4534, _, _, _, _, _) => {
                h4534
            }
        };
        let a4537 = 0u64;
        nat_rem(a4535, a4536, a4537)
    };
    let v10: u64 = {
        let a4539 = v6;
        let a4540 = match v3.clone() {
            Adt3::C0(_, _, h4538, _, _, _, _, _) => {
                h4538
            }
        };
        let a4541 = 0u64;
        nat_quot(a4539, a4540, a4541)
    };
    let v11: u64 = {
        let a4542 = v4;
        let a4543 = 0u64;
        if nat_eq(a4542, a4543) {
            match v1.clone() {
                Adt3::C0(h4544, _, _, _, _, _, _, _) => {
                    h4544
                }
            }
        } else {
            let a4545 = v4;
            let a4546 = 1u64;
            if nat_eq(a4545, a4546) {
                match v1.clone() {
                    Adt3::C0(_, h4547, _, _, _, _, _, _) => {
                        h4547
                    }
                }
            } else {
                let a4548 = v4;
                let a4549 = 2u64;
                if nat_eq(a4548, a4549) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4550, _, _, _, _, _) => {
                            h4550
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4551, _, _, _, _) => {
                            h4551
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4558 = v11;
        let a4559 = {
            let a4552 = v4;
            let a4553 = 0u64;
            if nat_eq(a4552, a4553) {
                v7
            } else {
                let a4554 = v4;
                let a4555 = 1u64;
                if nat_eq(a4554, a4555) {
                    v8
                } else {
                    let a4556 = v4;
                    let a4557 = 2u64;
                    if nat_eq(a4556, a4557) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4558, a4559)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f140(if if v12 {
        let a4560 = v4;
        let a4561 = 0u64;
        nat_eq(a4560, a4561)
    } else {
        false
    } {
        let a4562 = v7;
        let a4563 = v11;
        nat_sub(a4562, a4563)
    } else {
        v7
    }, if if v12 {
        let a4564 = v4;
        let a4565 = 1u64;
        nat_eq(a4564, a4565)
    } else {
        false
    } {
        let a4566 = v8;
        let a4567 = v11;
        nat_sub(a4566, a4567)
    } else {
        v8
    }, if if v12 {
        let a4568 = v4;
        let a4569 = 2u64;
        nat_eq(a4568, a4569)
    } else {
        false
    } {
        let a4570 = v9;
        let a4571 = v11;
        nat_sub(a4570, a4571)
    } else {
        v9
    }, if if v12 {
        let a4572 = v4;
        let a4573 = 3u64;
        nat_eq(a4572, a4573)
    } else {
        false
    } {
        let a4574 = v10;
        let a4575 = v11;
        nat_sub(a4574, a4575)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4576, _, _, _) => {
            h4576
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4577, _, _) => {
            h4577
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4578, _) => {
            h4578
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4579) => {
            h4579
        }
    })?, f91(v0, v3)?))
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f57(v1.clone(), f237(v8, v3, match v1.clone() {
            Adt3::C0(h4580, _, _, _, _, _, _, _) => {
                h4580
            }
        })?, f237(v9, v4, match v1.clone() {
            Adt3::C0(_, h4581, _, _, _, _, _, _) => {
                h4581
            }
        })?, f237(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4582, _, _, _, _, _) => {
                h4582
            }
        })?, f237(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4583, _, _, _, _) => {
                h4583
            }
        })?)?, f91(v0, v2)?))
    } else {
        if if if if f238(v8, v3, match v1.clone() {
            Adt3::C0(h4584, _, _, _, _, _, _, _) => {
                h4584
            }
        })? {
            f238(v9, v4, match v1.clone() {
                Adt3::C0(_, h4585, _, _, _, _, _, _) => {
                    h4585
                }
            })?
        } else {
            false
        } {
            f238(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4586, _, _, _, _, _) => {
                    h4586
                }
            })?
        } else {
            false
        } {
            f238(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4587, _, _, _, _) => {
                    h4587
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f57(v1, f239(v8, v3)?, f239(v9, v4)?, f239(v10, v5)?, f239(v11, v6)?)?, f91(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f91(v0, v2)?))
        }
    }
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f57(v1.clone(), f240(v4, v3, match v1 {
        Adt3::C0(h4588, _, _, _, _, _, _, _) => {
            h4588
        }
    })?, v5, v6, v7)?, f91(v0, v2)?))
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4589 = v0;
    let a4590 = v3;
    if nat_lt(a4589, a4590) {
        Ok(Adt9::C1(false, {
            let a4591 = v0;
            let a4592 = 4u64;
            nat_mul(a4591, a4592)?
        }, {
            let a4593 = v0;
            let a4594 = 4u64;
            nat_mul(a4593, a4594)?
        }))
    } else {
        let v6: u64 = {
            let a4595 = v0;
            let a4596 = v3;
            nat_sub(a4595, a4596)
        };
        let v7: u64 = f91(v6, v1)?;
        let v8: u64 = f241(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4597 = v0;
    let a4598 = v3;
    if nat_lt(a4597, a4598) {
        Ok(Adt9::C1(true, 0u64, f91(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4599 = v0;
            let a4600 = v3;
            nat_sub(a4599, a4600)
        };
        let v5: u64 = f149(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f91(v4, v1)?, 3u64, v5))
    }
}

pub fn f155(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4602 = v1;
        let a4603 = match v3.clone() {
            Adt3::C0(h4601, _, _, _, _, _, _, _) => {
                h4601
            }
        };
        let a4604 = 0u64;
        nat_rem(a4602, a4603, a4604)
    };
    let v5: u64 = {
        let a4610 = {
            let a4606 = v1;
            let a4607 = match v3.clone() {
                Adt3::C0(h4605, _, _, _, _, _, _, _) => {
                    h4605
                }
            };
            let a4608 = 0u64;
            nat_quot(a4606, a4607, a4608)
        };
        let a4611 = match v3.clone() {
            Adt3::C0(_, h4609, _, _, _, _, _, _) => {
                h4609
            }
        };
        let a4612 = 0u64;
        nat_rem(a4610, a4611, a4612)
    };
    let v6: bool = f242(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f149(v1, v2, v3.clone())?
    } else {
        0u64
    }, f91(v1, v3)?))
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4613 = v3;
        let a4614 = 8u64;
        nat_add(a4613, a4614)?
    }, 0u64, f91(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f149(v0, v5, v2.clone())?
    }, 3u64, f91(v0, v2)?))
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4615 = v4;
    let a4616 = 0u64;
    if if nat_eq(a4615, a4616) {
        f121(v3)
    } else {
        false
    } {
        let m4617 = f213(v3);
        match m4617 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4618, _) => {
                        h4618
                    }
                };
                Ok(Adt9::C5(v3, f91(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4619) => {
                        h4619
                    }
                }, f91({
                    let a4620 = v0;
                    let a4621 = v6;
                    nat_mul(a4620, a4621)?
                }, v2.clone())?, {
                    let a4627 = {
                        let a4625 = {
                            let a4622 = v6;
                            let a4623 = 1u64;
                            nat_sub(a4622, a4623)
                        };
                        let a4626 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4624, _, _, _) => {
                                h4624
                            }
                        };
                        nat_mul(a4625, a4626)?
                    };
                    let a4628 = 4u64;
                    nat_add(a4627, a4628)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4629, _, _, _) => {
                        h4629
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f91(v0, v1)?, f91(v0, v2)?))
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4631 = v0;
        let a4632 = match v2.clone() {
            Adt3::C0(h4630, _, _, _, _, _, _, _) => {
                h4630
            }
        };
        let a4633 = 0u64;
        nat_rem(a4631, a4632, a4633)
    };
    let v7: u64 = {
        let a4635 = v0;
        let a4636 = match v2.clone() {
            Adt3::C0(h4634, _, _, _, _, _, _, _) => {
                h4634
            }
        };
        let a4637 = 0u64;
        nat_quot(a4635, a4636, a4637)
    };
    let v8: u64 = {
        let a4639 = v7;
        let a4640 = match v2.clone() {
            Adt3::C0(_, h4638, _, _, _, _, _, _) => {
                h4638
            }
        };
        let a4641 = 0u64;
        nat_rem(a4639, a4640, a4641)
    };
    let v9: u64 = {
        let a4643 = v7;
        let a4644 = match v2.clone() {
            Adt3::C0(_, h4642, _, _, _, _, _, _) => {
                h4642
            }
        };
        let a4645 = 0u64;
        nat_quot(a4643, a4644, a4645)
    };
    let v10: u64 = {
        let a4647 = v9;
        let a4648 = match v2.clone() {
            Adt3::C0(_, _, h4646, _, _, _, _, _) => {
                h4646
            }
        };
        let a4649 = 0u64;
        nat_rem(a4647, a4648, a4649)
    };
    let v11: u64 = {
        let a4651 = v9;
        let a4652 = match v2.clone() {
            Adt3::C0(_, _, h4650, _, _, _, _, _) => {
                h4650
            }
        };
        let a4653 = 0u64;
        nat_quot(a4651, a4652, a4653)
    };
    let v12: u64 = f91(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4662 = {
                let a4657 = {
                    let a4654 = v10;
                    let a4655 = v4;
                    let a4656 = 0u64;
                    nat_quot(a4654, a4655, a4656)
                };
                let a4658 = v5;
                nat_mul(a4657, a4658)?
            };
            let a4663 = {
                let a4659 = v8;
                let a4660 = v4;
                let a4661 = 0u64;
                nat_quot(a4659, a4660, a4661)
            };
            nat_add(a4662, a4663)?
        };
        let v14: u64 = {
            let a4683 = v6;
            let a4684 = {
                let a4681 = match v1.clone() {
                    Adt3::C0(h4664, _, _, _, _, _, _, _) => {
                        h4664
                    }
                };
                let a4682 = {
                    let a4679 = {
                        let a4665 = v8;
                        let a4666 = v4;
                        let a4667 = 0u64;
                        nat_rem(a4665, a4666, a4667)
                    };
                    let a4680 = {
                        let a4677 = match v1.clone() {
                            Adt3::C0(_, h4668, _, _, _, _, _, _) => {
                                h4668
                            }
                        };
                        let a4678 = {
                            let a4675 = {
                                let a4669 = v10;
                                let a4670 = v4;
                                let a4671 = 0u64;
                                nat_rem(a4669, a4670, a4671)
                            };
                            let a4676 = {
                                let a4673 = match v1.clone() {
                                    Adt3::C0(_, _, h4672, _, _, _, _, _) => {
                                        h4672
                                    }
                                };
                                let a4674 = v13;
                                nat_mul(a4673, a4674)?
                            };
                            nat_add(a4675, a4676)?
                        };
                        nat_mul(a4677, a4678)?
                    };
                    nat_add(a4679, a4680)?
                };
                nat_mul(a4681, a4682)?
            };
            nat_add(a4683, a4684)?
        };
        Ok(Adt9::C1(false, f91(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4690 = {
                let a4688 = {
                    let a4685 = v11;
                    let a4686 = v5;
                    let a4687 = 0u64;
                    nat_rem(a4685, a4686, a4687)
                };
                let a4689 = v4;
                nat_mul(a4688, a4689)?
            };
            let a4691 = v8;
            nat_add(a4690, a4691)?
        };
        let v16: u64 = {
            let a4697 = {
                let a4695 = {
                    let a4692 = v11;
                    let a4693 = v5;
                    let a4694 = 0u64;
                    nat_quot(a4692, a4693, a4694)
                };
                let a4696 = v4;
                nat_mul(a4695, a4696)?
            };
            let a4698 = v10;
            nat_add(a4697, a4698)?
        };
        let a4700 = v15;
        let a4701 = match v1.clone() {
            Adt3::C0(_, h4699, _, _, _, _, _, _) => {
                h4699
            }
        };
        if if nat_lt(a4700, a4701) {
            let a4703 = v16;
            let a4704 = match v1.clone() {
                Adt3::C0(_, _, h4702, _, _, _, _, _) => {
                    h4702
                }
            };
            nat_lt(a4703, a4704)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f91({
                let a4713 = v6;
                let a4714 = {
                    let a4711 = match v1.clone() {
                        Adt3::C0(h4705, _, _, _, _, _, _, _) => {
                            h4705
                        }
                    };
                    let a4712 = {
                        let a4709 = v15;
                        let a4710 = {
                            let a4707 = match v1.clone() {
                                Adt3::C0(_, h4706, _, _, _, _, _, _) => {
                                    h4706
                                }
                            };
                            let a4708 = v16;
                            nat_mul(a4707, a4708)?
                        };
                        nat_add(a4709, a4710)?
                    };
                    nat_mul(a4711, a4712)?
                };
                nat_add(a4713, a4714)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f234(23u64, v4);
    let v6: u64 = {
        let a4715 = v0;
        let a4716 = v5;
        let a4717 = 0u64;
        nat_rem(a4715, a4716, a4717)
    };
    let v7: u64 = {
        let a4718 = v0;
        let a4719 = v5;
        let a4720 = 0u64;
        nat_quot(a4718, a4719, a4720)
    };
    let v8: Adt13 = Adt13::C1(f91(v7, v2)?);
    let a4721 = v6;
    let a4722 = 0u64;
    if nat_eq(a4721, a4722) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f91(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f172(v3), 4u64))
    } else {
        let a4723 = v6;
        let a4724 = 1u64;
        if nat_eq(a4723, a4724) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4725 = v4;
                let a4726 = 0u64;
                if nat_eq(a4725, a4726) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f172(v4)))
        }
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f91(v0, v2.clone())?;
    let v4: u64 = f91(v0, v1.clone())?;
    let a4731 = {
        let a4728 = v0;
        let a4729 = match v1 {
            Adt3::C0(h4727, _, _, _, _, _, _, _) => {
                h4727
            }
        };
        let a4730 = 0u64;
        nat_rem(a4728, a4729, a4730)
    };
    let a4732 = 0u64;
    if nat_eq(a4731, a4732) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f91({
            let a4733 = v0;
            let a4734 = 1u64;
            nat_sub(a4733, a4734)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4735 = v3;
    let a4736 = 13u64;
    if nat_eq(a4735, a4736) {
        f243(v0, v1, v4)
    } else {
        let v5: u64 = f244(match v1.clone() {
            Adt3::C0(h4737, _, _, _, _, _, _, _) => {
                h4737
            }
        }, {
            let a4738 = v3;
            let a4739 = 16u64;
            nat_eq(a4738, a4739)
        })?;
        let v6: u64 = {
            let a4740 = v0;
            let a4741 = v5;
            let a4742 = 0u64;
            nat_quot(a4740, a4741, a4742)
        };
        let v7: u64 = {
            let a4743 = v0;
            let a4744 = v5;
            let a4745 = 0u64;
            nat_rem(a4743, a4744, a4745)
        };
        let v8: u64 = {
            let a4749 = 1u64;
            let a4750 = {
                let a4747 = match v1.clone() {
                    Adt3::C0(h4746, _, _, _, _, _, _, _) => {
                        h4746
                    }
                };
                let a4748 = 2u64;
                nat_mul(a4747, a4748)?
            };
            nat_add(a4749, a4750)?
        };
        let a4751 = v7;
        let a4752 = v8;
        if nat_lt(a4751, a4752) {
            f245(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f91(v6, v2)?;
            let a4753 = v3;
            let a4754 = 14u64;
            if nat_eq(a4753, a4754) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4755 = v7;
                let a4756 = v8;
                if nat_eq(a4755, a4756) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4759 = v7;
                    let a4760 = {
                        let a4757 = v8;
                        let a4758 = 1u64;
                        nat_add(a4757, a4758)?
                    };
                    if nat_eq(a4759, a4760) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4761, _, _, _, _, _, _, _) => {
                                h4761
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

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4762 = v0;
    let a4763 = 0u64;
    if nat_eq(a4762, a4763) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4770 = {
            let a4764 = v0;
            let a4765 = 1u64;
            nat_add(a4764, a4765)?
        };
        let a4771 = {
            let a4768 = 2u64;
            let a4769 = {
                let a4766 = v3;
                let a4767 = 2u64;
                nat_mul(a4766, a4767)?
            };
            nat_add(a4768, a4769)?
        };
        if nat_eq(a4770, a4771) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4772 = v0;
                let a4773 = 1u64;
                nat_sub(a4772, a4773)
            };
            let v5: u64 = {
                let a4774 = v4;
                let a4775 = 2u64;
                let a4776 = 0u64;
                nat_quot(a4774, a4775, a4776)
            };
            let a4780 = {
                let a4777 = v4;
                let a4778 = 2u64;
                let a4779 = 0u64;
                nat_rem(a4777, a4778, a4779)
            };
            let a4781 = 0u64;
            if nat_eq(a4780, a4781) {
                Ok(Adt9::C4(17u64, 0u64, f91(v5, v1)?, 4u64, f91(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f163(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4782 = v0;
    let a4783 = f41(v1, v2, v3)?;
    if nat_lt(a4782, a4783) {
        let a4784 = v0;
        let a4785 = v1;
        if nat_lt(a4784, a4785) {
            Ok(Adt9::C1(false, {
                let a4786 = 4u64;
                let a4787 = v0;
                nat_mul(a4786, a4787)?
            }, {
                let a4788 = 4u64;
                let a4789 = v0;
                nat_mul(a4788, a4789)?
            }))
        } else {
            let v4: u64 = {
                let a4790 = v0;
                let a4791 = v1;
                nat_sub(a4790, a4791)
            };
            let v5: u64 = f246(v4, v2)?;
            let a4792 = v5;
            let a4793 = 2u64;
            if nat_lt(a4792, a4793) {
                Ok(Adt9::C4(6u64, {
                    let a4794 = v5;
                    let a4795 = 0u64;
                    if nat_eq(a4794, a4795) {
                        4u64
                    } else {
                        5u64
                    }
                }, f247(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f248(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f249(v4, v2)? {
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

pub fn f164(v0: u64, v1: u64) -> u64 {
    let a4796 = v0;
    let a4797 = 0u64;
    if if nat_eq(a4796, a4797) {
        true
    } else {
        let a4798 = v1;
        let a4799 = 0u64;
        nat_eq(a4798, a4799)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f165(v0: u64, v1: u64) -> Adt11 {
    let a4800 = v0;
    let a4801 = 43u64;
    if nat_eq(a4800, a4801) {
        Adt11::C8(3u64)
    } else {
        let a4802 = v1;
        let a4803 = 43u64;
        if nat_eq(a4802, a4803) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f166(v0: u64) -> R<u64> {
    let a4806 = 2u64;
    let a4807 = {
        let a4804 = 6u64;
        let a4805 = v0;
        nat_mul(a4804, a4805)?
    };
    nat_add(a4806, a4807)
}

pub fn f167(v0: u64, v1: bool) -> R<u64> {
    let a4810 = {
        let a4808 = v0;
        let a4809 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4808, a4809)?
    };
    let a4811 = 1u64;
    nat_add(a4810, a4811)
}

pub fn f168(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4812 = v0;
    let a4813 = 16u64;
    if nat_lt(a4812, a4813) {
        let v9: u64 = {
            let a4814 = v0;
            let a4815 = 4u64;
            let a4816 = 0u64;
            nat_quot(a4814, a4815, a4816)
        };
        let v10: u64 = {
            let a4821 = {
                let a4819 = 8u64;
                let a4820 = {
                    let a4817 = 4u64;
                    let a4818 = v1;
                    nat_mul(a4817, a4818)?
                };
                nat_add(a4819, a4820)?
            };
            let a4822 = v9;
            nat_add(a4821, a4822)?
        };
        let v11: u64 = f57(v6, v9, v1, v2, 0u64)?;
        let a4826 = {
            let a4823 = v0;
            let a4824 = 4u64;
            let a4825 = 0u64;
            nat_rem(a4823, a4824, a4825)
        };
        let a4827 = 0u64;
        if nat_eq(a4826, a4827) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f57(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f57(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4831 = {
                let a4828 = v0;
                let a4829 = 4u64;
                let a4830 = 0u64;
                nat_rem(a4828, a4829, a4830)
            };
            let a4832 = 1u64;
            if nat_eq(a4831, a4832) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f57(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4836 = {
                    let a4833 = v0;
                    let a4834 = 4u64;
                    let a4835 = 0u64;
                    nat_rem(a4833, a4834, a4835)
                };
                let a4837 = 2u64;
                if nat_eq(a4836, a4837) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4838 = v9;
                        let a4839 = 0u64;
                        if nat_eq(a4838, a4839) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4840 = v8;
                        let a4841 = 2u64;
                        if nat_eq(a4840, a4841) {
                            let a4842 = v9;
                            let a4843 = 0u64;
                            if nat_eq(a4842, a4843) {
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
        let a4844 = v0;
        let a4845 = 28u64;
        if nat_lt(a4844, a4845) {
            let v12: u64 = {
                let a4846 = v0;
                let a4847 = 16u64;
                nat_sub(a4846, a4847)
            };
            let v13: u64 = {
                let a4848 = v12;
                let a4849 = 3u64;
                let a4850 = 0u64;
                nat_quot(a4848, a4849, a4850)
            };
            let v14: u64 = f57(v6, v13, v1, v2, 0u64)?;
            let a4854 = {
                let a4851 = v12;
                let a4852 = 3u64;
                let a4853 = 0u64;
                nat_rem(a4851, a4852, a4853)
            };
            let a4855 = 0u64;
            if nat_eq(a4854, a4855) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4859 = {
                    let a4856 = v12;
                    let a4857 = 3u64;
                    let a4858 = 0u64;
                    nat_rem(a4856, a4857, a4858)
                };
                let a4860 = 1u64;
                if nat_eq(a4859, a4860) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4861 = v13;
                        let a4862 = 0u64;
                        if nat_eq(a4861, a4862) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4863 = v0;
            let a4864 = 28u64;
            if nat_eq(a4863, a4864) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4865 = v0;
                    let a4866 = 29u64;
                    nat_sub(a4865, a4866)
                };
                let v16: u64 = {
                    let a4867 = v15;
                    let a4868 = 2u64;
                    let a4869 = 0u64;
                    nat_quot(a4867, a4868, a4869)
                };
                let v17: u64 = f57(v6, v16, v1, v2, 0u64)?;
                let a4873 = {
                    let a4870 = v15;
                    let a4871 = 2u64;
                    let a4872 = 0u64;
                    nat_rem(a4870, a4871, a4872)
                };
                let a4874 = 0u64;
                if nat_eq(a4873, a4874) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f169(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4875 = v0;
        let a4876 = 9u64;
        let a4877 = 0u64;
        nat_rem(a4875, a4876, a4877)
    };
    let v5: u64 = {
        let a4881 = {
            let a4878 = v0;
            let a4879 = 9u64;
            let a4880 = 0u64;
            nat_quot(a4878, a4879, a4880)
        };
        let a4882 = 4u64;
        let a4883 = 0u64;
        nat_rem(a4881, a4882, a4883)
    };
    let v6: bool = {
        let a4890 = {
            let a4887 = {
                let a4884 = v0;
                let a4885 = 36u64;
                let a4886 = 0u64;
                nat_quot(a4884, a4885, a4886)
            };
            let a4888 = 2u64;
            let a4889 = 0u64;
            nat_rem(a4887, a4888, a4889)
        };
        let a4891 = 0u64;
        nat_eq(a4890, a4891)
    };
    let v7: u64 = {
        let a4892 = v4;
        let a4893 = 4u64;
        if nat_lt(a4892, a4893) {
            v4
        } else {
            let a4894 = v4;
            let a4895 = 5u64;
            nat_sub(a4894, a4895)
        }
    };
    let v8: u64 = f57(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a4896 = v4;
    let a4897 = 4u64;
    if nat_lt(a4896, a4897) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4898 = v4;
            let a4899 = 0u64;
            if nat_eq(a4898, a4899) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4900 = v4;
        let a4901 = 4u64;
        if nat_eq(a4900, a4901) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f170(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a4903 = v2;
        let a4904 = f142(match v0.clone() {
            Adt6::C0(_, _, h4902, _) => {
                h4902
            }
        });
        nat_add(a4903, a4904)?
    };
    let v5: u64 = {
        let a4909 = {
            let a4906 = v3;
            let a4907 = match v0.clone() {
                Adt6::C0(_, _, _, h4905) => {
                    h4905
                }
            };
            nat_mul(a4906, a4907)?
        };
        let a4910 = f141(match v0.clone() {
            Adt6::C0(_, _, h4908, _) => {
                h4908
            }
        });
        nat_add(a4909, a4910)?
    };
    let v6: u64 = {
        let a4911 = v4;
        let a4912 = v5;
        nat_sub(a4911, a4912)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h4913, _, _) => {
            h4913
        }
    };
    let v8: u64 = {
        let a4914 = v6;
        let a4915 = v7;
        let a4916 = 0u64;
        nat_quot(a4914, a4915, a4916)
    };
    let a4917 = v5;
    let a4918 = v4;
    if if if nat_le(a4917, a4918) {
        let a4922 = {
            let a4919 = v6;
            let a4920 = v7;
            let a4921 = 0u64;
            nat_rem(a4919, a4920, a4921)
        };
        let a4923 = 0u64;
        nat_eq(a4922, a4923)
    } else {
        false
    } {
        let a4924 = v8;
        let a4925 = v1;
        nat_lt(a4924, a4925)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f171(v0: u64) -> R<u64> {
    let a4928 = v0;
    let a4929 = {
        let a4926 = v0;
        let a4927 = 2u64;
        nat_add(a4926, a4927)?
    };
    nat_mul(a4928, a4929)
}

pub fn f172(v0: u64) -> u64 {
    let m4930 = f213(v0);
    match m4930 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4932 = match v1.clone() {
                Adt19::C0(_, h4931, _) => {
                    h4931
                }
            };
            let a4933 = 1u64;
            if if nat_eq(a4932, a4933) {
                let a4935 = match v1.clone() {
                    Adt19::C0(_, _, h4934) => {
                        h4934
                    }
                };
                let a4936 = 1u64;
                if if if nat_eq(a4935, a4936) {
                    true
                } else {
                    let a4938 = match v1.clone() {
                        Adt19::C0(_, _, h4937) => {
                            h4937
                        }
                    };
                    let a4939 = 2u64;
                    nat_eq(a4938, a4939)
                } {
                    true
                } else {
                    let a4941 = match v1.clone() {
                        Adt19::C0(_, _, h4940) => {
                            h4940
                        }
                    };
                    let a4942 = 4u64;
                    nat_eq(a4941, a4942)
                } {
                    true
                } else {
                    let a4944 = match v1.clone() {
                        Adt19::C0(_, _, h4943) => {
                            h4943
                        }
                    };
                    let a4945 = 8u64;
                    nat_eq(a4944, a4945)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4946) => {
                        h4946
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f173(v0: u64) -> R<u64> {
    let a4949 = 3u64;
    let a4950 = {
        let a4947 = v0;
        let a4948 = 3u64;
        nat_mul(a4947, a4948)?
    };
    nat_add(a4949, a4950)
}

pub fn f174(v0: u64, v1: u64) -> R<u64> {
    if f175(v0) {
        let a4951 = 2u64;
        let a4952 = v1;
        nat_mul(a4951, a4952)
    } else {
        let a4953 = v1;
        let a4954 = f250(v0);
        let a4955 = 0u64;
        Ok(nat_quot(a4953, a4954, a4955))
    }
}

pub fn f175(v0: u64) -> bool {
    let a4956 = v0;
    let a4957 = 0u64;
    if if nat_eq(a4956, a4957) {
        true
    } else {
        let a4958 = v0;
        let a4959 = 1u64;
        nat_eq(a4958, a4959)
    } {
        true
    } else {
        let a4960 = v0;
        let a4961 = 30u64;
        nat_eq(a4960, a4961)
    }
}

pub fn f176(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4962 = f213(v0);
    match m4962 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f140(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4963, _, _, _) => {
                    h4963
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4964, _, _) => {
                    h4964
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4965, _) => {
                    h4965
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4966) => {
                    h4966
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4967) => {
                    h4967
                }
            }, f140({
                let a4969 = v2;
                let a4970 = match v6.clone() {
                    Adt19::C0(_, h4968, _) => {
                        h4968
                    }
                };
                nat_mul(a4969, a4970)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4971, _, _, _) => {
                    h4971
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4972, _, _) => {
                    h4972
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4973, _) => {
                    h4973
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4974) => {
                    h4974
                }
            })?, match v6 {
                Adt19::C0(_, h4975, _) => {
                    h4975
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4976, _, _, _) => {
                    h4976
                }
            }))
        }
    }
}

pub fn f177(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4984 = {
        let a4977 = 2u64;
        let a4978 = f178(v1.clone())?;
        nat_mul(a4977, a4978)?
    };
    let a4985 = {
        let a4982 = f178(v1)?;
        let a4983 = {
            let a4980 = 1u64;
            let a4981 = match v0 {
                Adt3::C0(h4979, _, _, _, _, _, _, _) => {
                    h4979
                }
            };
            nat_add(a4980, a4981)?
        };
        nat_mul(a4982, a4983)?
    };
    nat_add(a4984, a4985)
}

pub fn f178(v0: Adt3) -> R<u64> {
    if f48(match v0.clone() {
        Adt3::C0(h4986, _, _, _, _, _, _, _) => {
            h4986
        }
    }, match v0.clone() {
        Adt3::C0(_, h4987, _, _, _, _, _, _) => {
            h4987
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4988, _, _, _, _, _) => {
            h4988
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4989, _, _, _, _) => {
            h4989
        }
    }) {
        Ok(0u64)
    } else {
        let a4998 = match v0.clone() {
            Adt3::C0(h4990, _, _, _, _, _, _, _) => {
                h4990
            }
        };
        let a4999 = {
            let a4996 = match v0.clone() {
                Adt3::C0(_, h4991, _, _, _, _, _, _) => {
                    h4991
                }
            };
            let a4997 = {
                let a4994 = match v0.clone() {
                    Adt3::C0(_, _, h4992, _, _, _, _, _) => {
                        h4992
                    }
                };
                let a4995 = match v0 {
                    Adt3::C0(_, _, _, h4993, _, _, _, _) => {
                        h4993
                    }
                };
                nat_mul(a4994, a4995)?
            };
            nat_mul(a4996, a4997)?
        };
        nat_mul(a4998, a4999)
    }
}

pub fn f179(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f91(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f180(v0: u64, v1: u64) -> bool {
    let a5000 = v1;
    let a5001 = 2147483647u64;
    if nat_le(a5000, a5001) {
        let a5002 = v1;
        let a5003 = v0;
        nat_lt(a5002, a5003)
    } else {
        false
    }
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5004 = f213(v0);
    match m5004 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5006 = v2;
                let a5007 = match v7.clone() {
                    Adt19::C0(_, h5005, _) => {
                        h5005
                    }
                };
                nat_mul(a5006, a5007)?
            };
            let m5020 = f251(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5008, _, _, _, _, _, _, _) => {
                    h5008
                }
            }, match v5.clone() {
                Adt3::C0(_, h5009, _, _, _, _, _, _) => {
                    h5009
                }
            }, match v5 {
                Adt3::C0(_, _, h5010, _, _, _, _, _) => {
                    h5010
                }
            }, {
                let a5013 = match v3.clone() {
                    Adt3::C0(h5011, _, _, _, _, _, _, _) => {
                        h5011
                    }
                };
                let a5014 = match v7.clone() {
                    Adt19::C0(_, h5012, _) => {
                        h5012
                    }
                };
                nat_mul(a5013, a5014)?
            }, match v3.clone() {
                Adt3::C0(_, h5015, _, _, _, _, _, _) => {
                    h5015
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5016, _, _, _) => {
                    h5016
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5017, _, _) => {
                    h5017
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5018, _) => {
                    h5018
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5019) => {
                    h5019
                }
            })?;
            match m5020 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5021 = v0;
                    let a5022 = 0u64;
                    if if nat_eq(a5021, a5022) {
                        true
                    } else {
                        let a5023 = v0;
                        let a5024 = 26u64;
                        nat_eq(a5023, a5024)
                    } {
                        Ok(Adt9::C1(false, v9, f91({
                            let a5028 = {
                                let a5026 = v1;
                                let a5027 = match v4.clone() {
                                    Adt3::C0(h5025, _, _, _, _, _, _, _) => {
                                        h5025
                                    }
                                };
                                nat_mul(a5026, a5027)?
                            };
                            let a5029 = v8;
                            nat_add(a5028, a5029)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5030) => {
                                h5030
                            }
                        }, f91({
                            let a5034 = {
                                let a5032 = v1;
                                let a5033 = match v4.clone() {
                                    Adt3::C0(h5031, _, _, _, _, _, _, _) => {
                                        h5031
                                    }
                                };
                                nat_mul(a5032, a5033)?
                            };
                            let a5035 = v8;
                            nat_add(a5034, a5035)?
                        }, v4.clone())?, {
                            let a5042 = {
                                let a5040 = {
                                    let a5037 = match v7 {
                                        Adt19::C0(_, h5036, _) => {
                                            h5036
                                        }
                                    };
                                    let a5038 = 1u64;
                                    nat_sub(a5037, a5038)
                                };
                                let a5041 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5039, _, _, _) => {
                                        h5039
                                    }
                                };
                                nat_mul(a5040, a5041)?
                            };
                            let a5043 = 4u64;
                            nat_add(a5042, a5043)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5044, _, _, _) => {
                                h5044
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f182(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5045 = v1;
    let a5046 = 0u64;
    if if nat_eq(a5045, a5046) {
        true
    } else {
        let a5047 = v2;
        let a5048 = 0u64;
        nat_eq(a5047, a5048)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f183(v0: u64, v1: u64) -> bool {
    let a5049 = v0;
    let a5050 = 4294967295u64;
    if nat_eq(a5049, a5050) {
        true
    } else {
        let a5051 = v0;
        let a5052 = 2147483648u64;
        if nat_lt(a5051, a5052) {
            let a5053 = v0;
            let a5054 = v1;
            nat_lt(a5053, a5054)
        } else {
            false
        }
    }
}

pub fn f184(v0: Adt7) -> R<u64> {
    let a5066 = if match v0.clone() {
        Adt7::C0(_, _, _, h5055, _) => {
            h5055
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5067 = {
        let a5064 = {
            let a5062 = {
                let m5058 = match v0.clone() {
                    Adt7::C0(h5057, _, _, _, _) => {
                        h5057
                    }
                };
                match m5058 {
                    Adt6::C0(h5056, _, _, _) => {
                        h5056
                    }
                }
            };
            let a5063 = {
                let m5061 = match v0.clone() {
                    Adt7::C0(_, h5060, _, _, _) => {
                        h5060
                    }
                };
                match m5061 {
                    Adt6::C0(h5059, _, _, _) => {
                        h5059
                    }
                }
            };
            nat_mul(a5062, a5063)?
        };
        let a5065 = f185(v0)?;
        nat_mul(a5064, a5065)?
    };
    nat_add(a5066, a5067)
}

pub fn f185(v0: Adt7) -> R<u64> {
    let a5072 = if match v0.clone() {
        Adt7::C0(_, _, _, h5068, _) => {
            h5068
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5073 = {
        let a5070 = match v0 {
            Adt7::C0(_, _, _, _, h5069) => {
                h5069
            }
        };
        let a5071 = 1u64;
        if nat_eq(a5070, a5071) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5072, a5073)
}

pub fn f186(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5078 = {
            let a5075 = v2;
            let a5076 = match v0.clone() {
                Adt6::C0(_, h5074, _, _) => {
                    h5074
                }
            };
            nat_mul(a5075, a5076)?
        };
        let a5079 = f141(match v0.clone() {
            Adt6::C0(_, _, h5077, _) => {
                h5077
            }
        });
        nat_add(a5078, a5079)?
    };
    let v4: u64 = f142(match v0.clone() {
        Adt6::C0(_, _, h5080, _) => {
            h5080
        }
    });
    let v5: u64 = {
        let a5084 = {
            let a5082 = v3;
            let a5083 = match v0 {
                Adt6::C0(h5081, _, _, _) => {
                    h5081
                }
            };
            nat_add(a5082, a5083)?
        };
        let a5085 = v4;
        nat_sub(a5084, a5085)
    };
    let a5090 = {
        let a5086 = v5;
        let a5087 = v1;
        if nat_lt(a5086, a5087) {
            v5
        } else {
            v1
        }
    };
    let a5091 = {
        let a5088 = v3;
        let a5089 = v4;
        nat_sub(a5088, a5089)
    };
    Ok(nat_sub(a5090, a5091))
}

pub fn f187(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5100 = f60(match v0.clone() {
        Adt7::C0(h5092, _, _, _, _) => {
            h5092
        }
    }, match v1.clone() {
        Adt3::C0(h5093, _, _, _, _, _, _, _) => {
            h5093
        }
    }, f58(v3, v2.clone(), 0u64), {
        let a5097 = v4;
        let a5098 = {
            let m5096 = match v0.clone() {
                Adt7::C0(h5095, _, _, _, _) => {
                    h5095
                }
            };
            match m5096 {
                Adt6::C0(h5094, _, _, _) => {
                    h5094
                }
            }
        };
        let a5099 = 0u64;
        nat_rem(a5097, a5098, a5099)
    })?;
    match m5100 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5109 = f60(match v0.clone() {
                Adt7::C0(_, h5101, _, _, _) => {
                    h5101
                }
            }, match v1.clone() {
                Adt3::C0(_, h5102, _, _, _, _, _, _) => {
                    h5102
                }
            }, f58(v3, v2.clone(), 1u64), {
                let a5106 = v4;
                let a5107 = {
                    let m5105 = match v0 {
                        Adt7::C0(h5104, _, _, _, _) => {
                            h5104
                        }
                    };
                    match m5105 {
                        Adt6::C0(h5103, _, _, _) => {
                            h5103
                        }
                    }
                };
                let a5108 = 0u64;
                nat_quot(a5106, a5107, a5108)
            })?;
            match m5109 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f57(v1, v5, v6, f58(v3, v2.clone(), 2u64), f58(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f188(v0: u64) -> R<u64> {
    let a5112 = 2u64;
    let a5113 = {
        let a5110 = v0;
        let a5111 = 2u64;
        nat_mul(a5110, a5111)?
    };
    nat_add(a5112, a5113)
}

pub fn f189(v0: Adt6) -> R<u64> {
    let a5126 = 2u64;
    let a5127 = {
        let a5124 = 2u64;
        let a5125 = {
            let a5121 = {
                let a5118 = match v0.clone() {
                    Adt6::C0(h5114, _, _, _) => {
                        h5114
                    }
                };
                let a5119 = {
                    let a5116 = match v0.clone() {
                        Adt6::C0(_, h5115, _, _) => {
                            h5115
                        }
                    };
                    let a5117 = 1u64;
                    nat_sub(a5116, a5117)
                };
                nat_add(a5118, a5119)?
            };
            let a5122 = match v0 {
                Adt6::C0(_, h5120, _, _) => {
                    h5120
                }
            };
            let a5123 = 0u64;
            nat_quot(a5121, a5122, a5123)
        };
        nat_mul(a5124, a5125)?
    };
    nat_add(a5126, a5127)
}

pub fn f190(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5128 = v0;
    match m5128 {
        Adt16::C0(v19) => {
            f252(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5129 = v20;
            let a5130 = 66u64;
            if nat_eq(a5129, a5130) {
                f253(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f254(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f191(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5137 = v3;
        let a5138 = {
            let a5134 = f91({
                let a5131 = v0;
                let a5132 = 2u64;
                let a5133 = 0u64;
                nat_quot(a5131, a5132, a5133)
            }, v1)?;
            let a5135 = v2;
            let a5136 = 0u64;
            nat_quot(a5134, a5135, a5136)
        };
        nat_add(a5137, a5138)?
    };
    let a5142 = {
        let a5139 = v0;
        let a5140 = 2u64;
        let a5141 = 0u64;
        nat_rem(a5139, a5140, a5141)
    };
    let a5143 = 0u64;
    if nat_eq(a5142, a5143) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5144 = v4;
        let a5145 = 0u64;
        if nat_eq(a5144, a5145) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f192(v0: u64, v1: u64) -> R<u64> {
    let a5148 = 8u64;
    let a5149 = {
        let a5146 = f255(v0);
        let a5147 = v1;
        nat_mul(a5146, a5147)?
    };
    nat_add(a5148, a5149)
}

pub fn f193(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5150, _, _, _, _, _, _, _) => {
            h5150
        }
    };
    let v7: u64 = f255(v3);
    let v8: u64 = {
        let a5153 = 1u64;
        let a5154 = {
            let a5151 = v6;
            let a5152 = v7;
            nat_mul(a5151, a5152)?
        };
        nat_add(a5153, a5154)?
    };
    let a5155 = v0;
    let a5156 = 0u64;
    if nat_eq(a5155, a5156) {
        Ok(f256(0u64))
    } else {
        let a5157 = v0;
        let a5158 = v8;
        if nat_lt(a5157, a5158) {
            let v9: u64 = {
                let a5159 = v0;
                let a5160 = 1u64;
                nat_sub(a5159, a5160)
            };
            let v10: u64 = {
                let a5161 = v9;
                let a5162 = v7;
                let a5163 = 0u64;
                nat_rem(a5161, a5162, a5163)
            };
            let v11: u64 = f91({
                let a5169 = {
                    let a5164 = v1;
                    let a5165 = v6;
                    nat_mul(a5164, a5165)?
                };
                let a5170 = {
                    let a5166 = v9;
                    let a5167 = v7;
                    let a5168 = 0u64;
                    nat_quot(a5166, a5167, a5168)
                };
                nat_add(a5169, a5170)?
            }, v2)?;
            let a5171 = v3;
            let a5172 = 30u64;
            if nat_eq(a5171, a5172) {
                let a5173 = v10;
                let a5174 = 0u64;
                if nat_eq(a5173, a5174) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5175 = v10;
                    let a5176 = 1u64;
                    if nat_eq(a5175, a5176) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5177 = v10;
                        let a5178 = 2u64;
                        if nat_eq(a5177, a5178) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5179 = v10;
                let a5180 = 0u64;
                if nat_eq(a5179, a5180) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5181 = v10;
                    let a5182 = 1u64;
                    if nat_eq(a5181, a5182) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5183 = v0;
                let a5184 = v8;
                nat_sub(a5183, a5184)
            };
            let a5185 = v12;
            let a5186 = 0u64;
            if nat_eq(a5185, a5186) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5187 = v12;
                let a5188 = 1u64;
                if nat_eq(a5187, a5188) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5189 = v12;
                    let a5190 = 2u64;
                    if nat_eq(a5189, a5190) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5191 = v12;
                        let a5192 = 3u64;
                        if nat_eq(a5191, a5192) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5193 = v12;
                            let a5194 = 4u64;
                            if nat_eq(a5193, a5194) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5195 = v12;
                                let a5196 = 5u64;
                                if nat_eq(a5195, a5196) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5197 = v12;
                                    let a5198 = 6u64;
                                    if nat_eq(a5197, a5198) {
                                        if f257(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5199 = 3u64;
                                                let a5200 = v1;
                                                nat_add(a5199, a5200)?
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

pub fn f194(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5201 = v0;
        let a5202 = f102(v3);
        let a5203 = 0u64;
        nat_quot(a5201, a5202, a5203)
    };
    let v5: u64 = {
        let a5208 = 3u64;
        let a5209 = {
            let a5205 = v4;
            let a5206 = match v1.clone() {
                Adt3::C0(h5204, _, _, _, _, _, _, _) => {
                    h5204
                }
            };
            let a5207 = 0u64;
            nat_quot(a5205, a5206, a5207)
        };
        nat_add(a5208, a5209)?
    };
    let v6: u64 = f91(v4, v1)?;
    let v7: u64 = f91(v4, v2)?;
    let a5210 = v3;
    let a5211 = 30u64;
    if nat_eq(a5210, a5211) {
        let a5215 = {
            let a5212 = v0;
            let a5213 = 3u64;
            let a5214 = 0u64;
            nat_rem(a5212, a5213, a5214)
        };
        let a5216 = 0u64;
        if nat_eq(a5215, a5216) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5220 = {
                let a5217 = v0;
                let a5218 = 3u64;
                let a5219 = 0u64;
                nat_rem(a5217, a5218, a5219)
            };
            let a5221 = 1u64;
            if nat_eq(a5220, a5221) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f195(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5222 = v0;
        let a5223 = 8388608u64;
        let a5224 = 0u64;
        nat_quot(a5222, a5223, a5224)
    };
    let a5225 = v0;
    let a5226 = 2147483648u64;
    if if if nat_lt(a5225, a5226) {
        let a5227 = 127u64;
        let a5228 = v1;
        nat_le(a5227, a5228)
    } else {
        false
    } {
        let a5229 = v1;
        let a5230 = 190u64;
        nat_lt(a5229, a5230)
    } else {
        false
    } {
        let v2: u64 = {
            let a5234 = 8388608u64;
            let a5235 = {
                let a5231 = v0;
                let a5232 = 8388608u64;
                let a5233 = 0u64;
                nat_rem(a5231, a5232, a5233)
            };
            nat_add(a5234, a5235)?
        };
        let a5236 = v1;
        let a5237 = 150u64;
        if nat_lt(a5236, a5237) {
            let v3: u64 = f258({
                let a5238 = 150u64;
                let a5239 = v1;
                nat_sub(a5238, a5239)
            });
            let a5243 = {
                let a5240 = v2;
                let a5241 = v3;
                let a5242 = 0u64;
                nat_rem(a5240, a5241, a5242)
            };
            let a5244 = 0u64;
            if nat_eq(a5243, a5244) {
                let a5245 = v2;
                let a5246 = v3;
                let a5247 = 0u64;
                Ok(nat_quot(a5245, a5246, a5247))
            } else {
                Ok(0u64)
            }
        } else {
            let a5250 = f258({
                let a5248 = v1;
                let a5249 = 150u64;
                nat_sub(a5248, a5249)
            });
            let a5251 = v2;
            nat_mul(a5250, a5251)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f196(v0: Adt14, v1: Adt14) -> bool {
    let a5254 = match v0.clone() {
        Adt14::C0(_, h5252, _, _, _, _, _, _, _, _, _) => {
            h5252
        }
    };
    let a5255 = match v1.clone() {
        Adt14::C0(_, h5253, _, _, _, _, _, _, _, _, _) => {
            h5253
        }
    };
    if if if nat_eq(a5254, a5255) {
        let a5258 = match v0.clone() {
            Adt14::C0(_, _, h5256, _, _, _, _, _, _, _, _) => {
                h5256
            }
        };
        let a5259 = match v1.clone() {
            Adt14::C0(_, _, h5257, _, _, _, _, _, _, _, _) => {
                h5257
            }
        };
        nat_eq(a5258, a5259)
    } else {
        false
    } {
        let a5262 = match v0.clone() {
            Adt14::C0(_, _, _, h5260, _, _, _, _, _, _, _) => {
                h5260
            }
        };
        let a5263 = match v1.clone() {
            Adt14::C0(_, _, _, h5261, _, _, _, _, _, _, _) => {
                h5261
            }
        };
        nat_eq(a5262, a5263)
    } else {
        false
    } {
        let a5266 = match v0 {
            Adt14::C0(_, _, _, _, h5264, _, _, _, _, _, _) => {
                h5264
            }
        };
        let a5267 = match v1 {
            Adt14::C0(_, _, _, _, h5265, _, _, _, _, _, _) => {
                h5265
            }
        };
        nat_eq(a5266, a5267)
    } else {
        false
    }
}

pub fn f197(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5268 = v0;
    let a5269 = 0u64;
    if if if nat_eq(a5268, a5269) {
        true
    } else {
        let a5270 = v1;
        let a5271 = 0u64;
        nat_eq(a5270, a5271)
    } {
        true
    } else {
        let a5272 = v2;
        let a5273 = 0u64;
        nat_eq(a5272, a5273)
    } {
        true
    } else {
        let a5274 = v3;
        let a5275 = 0u64;
        nat_eq(a5274, a5275)
    }
}

pub fn f198(v0: Adt14, v1: Adt14) -> bool {
    if f204(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5276, _, _, _, _, _) => {
            h5276
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5277, _, _, _, _, _) => {
            h5277
        }
    }) {
        let a5279 = match v0 {
            Adt14::C0(_, h5278, _, _, _, _, _, _, _, _, _) => {
                h5278
            }
        };
        let a5280 = 2147483647u64;
        nat_le(a5279, a5280)
    } else {
        true
    }
}

pub fn f199(v0: Adt14) -> bool {
    let a5285 = {
        let a5282 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5281) => {
                h5281
            }
        };
        let a5283 = 4u64;
        let a5284 = 0u64;
        nat_rem(a5282, a5283, a5284)
    };
    let a5286 = 0u64;
    if if nat_eq(a5285, a5286) {
        f259(match v0.clone() {
            Adt14::C0(_, h5287, _, _, _, _, _, _, _, _, _) => {
                h5287
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5288, _, _, _, _, _) => {
                h5288
            }
        })
    } else {
        false
    } {
        f98(match v0.clone() {
            Adt14::C0(_, _, h5289, _, _, _, _, _, _, _, _) => {
                h5289
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5290, _, _, _, _, _, _, _) => {
                h5290
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5291, _, _, _, _, _, _) => {
                h5291
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5292, _, _, _, _) => {
                h5292
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5293, _, _, _) => {
                h5293
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5294, _, _) => {
                h5294
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f200(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5296 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5295, _) => {
            h5295
        }
    };
    let a5297 = 0u64;
    if nat_eq(a5296, a5297) {
        Ok(true)
    } else {
        let a5299 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5298, _) => {
                h5298
            }
        };
        let a5300 = 0u64;
        if nat_eq(a5299, a5300) {
            Ok(true)
        } else {
            let a5302 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5301) => {
                    h5301
                }
            };
            let a5303 = 0u64;
            if nat_eq(a5302, a5303) {
                Ok(true)
            } else {
                let a5305 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5304) => {
                        h5304
                    }
                };
                let a5306 = 0u64;
                if nat_eq(a5305, a5306) {
                    Ok(true)
                } else {
                    let a5312 = {
                        let a5309 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5307) => {
                                h5307
                            }
                        };
                        let a5310 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5308, _) => {
                                h5308
                            }
                        };
                        nat_add(a5309, a5310)?
                    };
                    let a5313 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5311) => {
                            h5311
                        }
                    };
                    if nat_le(a5312, a5313) {
                        Ok(true)
                    } else {
                        let a5319 = {
                            let a5316 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5314) => {
                                    h5314
                                }
                            };
                            let a5317 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5315, _) => {
                                    h5315
                                }
                            };
                            nat_add(a5316, a5317)?
                        };
                        let a5320 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5318) => {
                                h5318
                            }
                        };
                        Ok(nat_le(a5319, a5320))
                    }
                }
            }
        }
    }
}

pub fn f201(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f196(v0.clone(), v1.clone()) {
        let a5323 = match v0.clone() {
            Adt14::C0(h5321, _, _, _, _, _, _, _, _, _, _) => {
                h5321
            }
        };
        let a5324 = match v1.clone() {
            Adt14::C0(h5322, _, _, _, _, _, _, _, _, _, _) => {
                h5322
            }
        };
        nat_eq(a5323, a5324)
    } else {
        false
    } {
        let a5327 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5325) => {
                h5325
            }
        };
        let a5328 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5326) => {
                h5326
            }
        };
        nat_eq(a5327, a5328)
    } else {
        false
    } {
        let a5331 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5329, _, _, _, _, _) => {
                h5329
            }
        };
        let a5332 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5330, _, _, _, _, _) => {
                h5330
            }
        };
        nat_eq(a5331, a5332)
    } else {
        false
    } {
        let a5335 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5333, _, _, _, _) => {
                h5333
            }
        };
        let a5336 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5334, _, _, _, _) => {
                h5334
            }
        };
        nat_eq(a5335, a5336)
    } else {
        false
    } {
        let a5339 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5337, _, _, _) => {
                h5337
            }
        };
        let a5340 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5338, _, _, _) => {
                h5338
            }
        };
        nat_eq(a5339, a5340)
    } else {
        false
    } {
        let a5343 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5341, _, _) => {
                h5341
            }
        };
        let a5344 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5342, _, _) => {
                h5342
            }
        };
        nat_eq(a5343, a5344)
    } else {
        false
    }
}

pub fn f202(v0: Adt14) -> R<bool> {
    if f260(match v0.clone() {
        Adt14::C0(_, h5345, _, _, _, _, _, _, _, _, _) => {
            h5345
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5346, _, _, _, _, _) => {
            h5346
        }
    }) {
        f99(f261(match v0.clone() {
            Adt14::C0(_, h5347, _, _, _, _, _, _, _, _, _) => {
                h5347
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5348, _, _, _, _, _) => {
                h5348
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5349, _, _, _, _, _, _, _, _) => {
                h5349
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5350, _, _, _, _, _, _, _) => {
                h5350
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5351, _, _, _, _, _, _) => {
                h5351
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5352, _, _, _, _) => {
                h5352
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5353, _, _, _) => {
                h5353
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5354, _, _) => {
                h5354
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f203(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5355 = v1;
    let a5356 = 0u64;
    if if nat_eq(a5355, a5356) {
        true
    } else {
        let a5357 = v2;
        let a5358 = 0u64;
        nat_eq(a5357, a5358)
    } {
        Ok(0u64)
    } else {
        let a5361 = f100(v1, v2, v3)?;
        let a5362 = {
            let a5359 = v2;
            let a5360 = f262(v0, v1)?;
            nat_mul(a5359, a5360)?
        };
        nat_add(a5361, a5362)
    }
}

pub fn f204(v0: u64, v1: u64) -> bool {
    let a5363 = v0;
    let a5364 = 4u64;
    if nat_eq(a5363, a5364) {
        let a5365 = v1;
        let a5366 = 4u64;
        nat_eq(a5365, a5366)
    } else {
        false
    }
}

pub fn f205(v0: u64) -> R<u64> {
    let a5367 = 6u64;
    let a5368 = v0;
    nat_add(a5367, a5368)
}

pub fn f206(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5369, _, _, _, _, _, _, _, _, _, _) => {
            h5369
        }
    }, match v0.clone() {
        Adt14::C0(_, h5370, _, _, _, _, _, _, _, _, _) => {
            h5370
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5371, _, _, _, _, _, _, _, _) => {
            h5371
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5372, _, _, _, _, _, _, _) => {
            h5372
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5373, _, _, _, _, _, _) => {
            h5373
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5374, _, _, _, _) => {
            h5374
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5375, _, _, _) => {
            h5375
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5376, _, _) => {
            h5376
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5377, _) => {
            h5377
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5378) => {
            h5378
        }
    })
}

pub fn f207(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5379 = v0;
    let a5380 = 2147483647u64;
    if if if nat_le(a5379, a5380) {
        let a5381 = 0u64;
        let a5382 = v3;
        if nat_lt(a5381, a5382) {
            let a5383 = v3;
            let a5384 = 2147483647u64;
            nat_le(a5383, a5384)
        } else {
            false
        }
    } else {
        false
    } {
        let a5389 = {
            let a5387 = {
                let a5385 = v2;
                let a5386 = v3;
                nat_add(a5385, a5386)?
            };
            let a5388 = 1u64;
            nat_sub(a5387, a5388)
        };
        let a5390 = 2147483647u64;
        nat_le(a5389, a5390)
    } else {
        false
    } {
        let a5395 = {
            let a5393 = {
                let a5391 = v0;
                let a5392 = v1;
                nat_mul(a5391, a5392)?
            };
            let a5394 = f263(v2, v3)?;
            nat_mul(a5393, a5394)?
        };
        let a5396 = 9223372036854775807u64;
        Ok(nat_le(a5395, a5396))
    } else {
        Ok(false)
    }
}

pub fn f208(v0: Adt14) -> R<u64> {
    let a5427 = {
        let a5419 = {
            let a5411 = {
                let a5403 = 4u64;
                let a5404 = {
                    let a5401 = {
                        let a5398 = match v0.clone() {
                            Adt14::C0(_, h5397, _, _, _, _, _, _, _, _, _) => {
                                h5397
                            }
                        };
                        let a5399 = 1u64;
                        nat_sub(a5398, a5399)
                    };
                    let a5402 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5400, _, _, _, _, _) => {
                            h5400
                        }
                    };
                    nat_mul(a5401, a5402)?
                };
                nat_add(a5403, a5404)?
            };
            let a5412 = {
                let a5409 = {
                    let a5406 = match v0.clone() {
                        Adt14::C0(_, _, h5405, _, _, _, _, _, _, _, _) => {
                            h5405
                        }
                    };
                    let a5407 = 1u64;
                    nat_sub(a5406, a5407)
                };
                let a5410 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5408, _, _, _, _) => {
                        h5408
                    }
                };
                nat_mul(a5409, a5410)?
            };
            nat_add(a5411, a5412)?
        };
        let a5420 = {
            let a5417 = {
                let a5414 = match v0.clone() {
                    Adt14::C0(_, _, _, h5413, _, _, _, _, _, _, _) => {
                        h5413
                    }
                };
                let a5415 = 1u64;
                nat_sub(a5414, a5415)
            };
            let a5418 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5416, _, _, _) => {
                    h5416
                }
            };
            nat_mul(a5417, a5418)?
        };
        nat_add(a5419, a5420)?
    };
    let a5428 = {
        let a5425 = {
            let a5422 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5421, _, _, _, _, _, _) => {
                    h5421
                }
            };
            let a5423 = 1u64;
            nat_sub(a5422, a5423)
        };
        let a5426 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5424, _, _) => {
                h5424
            }
        };
        nat_mul(a5425, a5426)?
    };
    nat_add(a5427, a5428)
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5429 = v0;
    let a5430 = 0u64;
    if if if if nat_eq(a5429, a5430) {
        true
    } else {
        let a5431 = v1;
        let a5432 = 0u64;
        nat_eq(a5431, a5432)
    } {
        true
    } else {
        let a5433 = v2;
        let a5434 = 0u64;
        nat_eq(a5433, a5434)
    } {
        true
    } else {
        let a5435 = v3;
        let a5436 = 0u64;
        nat_eq(a5435, a5436)
    } {
        Ok(0u64)
    } else {
        let a5437 = f264(v0, v1, v2, v3, v5)?;
        let a5438 = f265(v0, v1, v2, v3, v4)?;
        nat_add(a5437, a5438)
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5439 = v0;
    let a5440 = 0u64;
    if if nat_eq(a5439, a5440) {
        true
    } else {
        let a5441 = v1;
        let a5442 = 0u64;
        nat_eq(a5441, a5442)
    } {
        Ok(0u64)
    } else {
        let a5445 = f100(v0, v1, v2)?;
        let a5446 = {
            let a5443 = v1;
            let a5444 = f266(v0)?;
            nat_mul(a5443, a5444)?
        };
        nat_add(a5445, a5446)
    }
}

pub fn f211(v0: u64, v1: u64, v2: u64) -> bool {
    let a5447 = v0;
    let a5448 = 1u64;
    if nat_le(a5447, a5448) {
        true
    } else {
        let a5452 = {
            let a5449 = v1;
            let a5450 = v2;
            let a5451 = 1u64;
            nat_rem(a5449, a5450, a5451)
        };
        let a5453 = 0u64;
        nat_eq(a5452, a5453)
    }
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5454 = v1;
    let a5455 = 1u64;
    if if if nat_le(a5454, a5455) {
        true
    } else {
        let a5456 = v0;
        let a5457 = v2;
        nat_le(a5456, a5457)
    } {
        let a5458 = v3;
        let a5459 = 1u64;
        if nat_le(a5458, a5459) {
            true
        } else {
            let a5466 = {
                let a5464 = v0;
                let a5465 = {
                    let a5462 = {
                        let a5460 = v1;
                        let a5461 = 1u64;
                        nat_sub(a5460, a5461)
                    };
                    let a5463 = v2;
                    nat_mul(a5462, a5463)?
                };
                nat_add(a5464, a5465)?
            };
            let a5467 = v4;
            nat_le(a5466, a5467)
        }
    } else {
        false
    } {
        let a5468 = v5;
        let a5469 = 1u64;
        if nat_le(a5468, a5469) {
            Ok(true)
        } else {
            let a5482 = {
                let a5480 = {
                    let a5474 = v0;
                    let a5475 = {
                        let a5472 = {
                            let a5470 = v1;
                            let a5471 = 1u64;
                            nat_sub(a5470, a5471)
                        };
                        let a5473 = v2;
                        nat_mul(a5472, a5473)?
                    };
                    nat_add(a5474, a5475)?
                };
                let a5481 = {
                    let a5478 = {
                        let a5476 = v3;
                        let a5477 = 1u64;
                        nat_sub(a5476, a5477)
                    };
                    let a5479 = v4;
                    nat_mul(a5478, a5479)?
                };
                nat_add(a5480, a5481)?
            };
            let a5483 = v6;
            Ok(nat_le(a5482, a5483))
        }
    } else {
        Ok(false)
    }
}

pub fn f213(v0: u64) -> Adt18 {
    let a5484 = v0;
    let a5485 = 0u64;
    if nat_eq(a5484, a5485) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5486 = v0;
        let a5487 = 1u64;
        if nat_eq(a5486, a5487) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5488 = v0;
            let a5489 = 2u64;
            if nat_eq(a5488, a5489) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5490 = v0;
                let a5491 = 3u64;
                if nat_eq(a5490, a5491) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5492 = v0;
                    let a5493 = 6u64;
                    if nat_eq(a5492, a5493) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5494 = v0;
                        let a5495 = 7u64;
                        if nat_eq(a5494, a5495) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5496 = v0;
                            let a5497 = 8u64;
                            if nat_eq(a5496, a5497) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5498 = v0;
                                let a5499 = 9u64;
                                if nat_eq(a5498, a5499) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5500 = v0;
                                    let a5501 = 10u64;
                                    if nat_eq(a5500, a5501) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5502 = v0;
                                        let a5503 = 11u64;
                                        if nat_eq(a5502, a5503) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5504 = v0;
                                            let a5505 = 12u64;
                                            if nat_eq(a5504, a5505) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5506 = v0;
                                                let a5507 = 13u64;
                                                if nat_eq(a5506, a5507) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5508 = v0;
                                                    let a5509 = 14u64;
                                                    if nat_eq(a5508, a5509) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5510 = v0;
                                                        let a5511 = 15u64;
                                                        if nat_eq(a5510, a5511) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5512 = v0;
                                                            let a5513 = 16u64;
                                                            if nat_eq(a5512, a5513) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5514 = v0;
                                                                let a5515 = 17u64;
                                                                if nat_eq(a5514, a5515) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5516 = v0;
                                                                    let a5517 = 18u64;
                                                                    if nat_eq(a5516, a5517) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5518 = v0;
                                                                        let a5519 = 19u64;
                                                                        if nat_eq(a5518, a5519) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5520 = v0;
                                                                            let a5521 = 20u64;
                                                                            if nat_eq(a5520, a5521) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5522 = v0;
                                                                                let a5523 = 21u64;
                                                                                if nat_eq(a5522, a5523) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5524 = v0;
                                                                                    let a5525 = 22u64;
                                                                                    if nat_eq(a5524, a5525) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5526 = v0;
                                                                                        let a5527 = 23u64;
                                                                                        if nat_eq(a5526, a5527) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5528 = v0;
                                                                                            let a5529 = 24u64;
                                                                                            if nat_eq(a5528, a5529) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5530 = v0;
                                                                                                let a5531 = 25u64;
                                                                                                if nat_eq(a5530, a5531) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5532 = v0;
                                                                                                    let a5533 = 26u64;
                                                                                                    if nat_eq(a5532, a5533) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5534 = v0;
                                                                                                        let a5535 = 27u64;
                                                                                                        if nat_eq(a5534, a5535) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5536 = v0;
                                                                                                            let a5537 = 28u64;
                                                                                                            if nat_eq(a5536, a5537) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5538 = v0;
                                                                                                                let a5539 = 29u64;
                                                                                                                if nat_eq(a5538, a5539) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5540 = v0;
                                                                                                                    let a5541 = 30u64;
                                                                                                                    if nat_eq(a5540, a5541) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5542 = v0;
                                                                                                                        let a5543 = 34u64;
                                                                                                                        if nat_eq(a5542, a5543) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5544 = v0;
                                                                                                                            let a5545 = 35u64;
                                                                                                                            if nat_eq(a5544, a5545) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5546 = v0;
                                                                                                                                let a5547 = 39u64;
                                                                                                                                if nat_eq(a5546, a5547) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5548 = v0;
                                                                                                                                    let a5549 = 40u64;
                                                                                                                                    if nat_eq(a5548, a5549) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5550 = v0;
                                                                                                                                        let a5551 = 41u64;
                                                                                                                                        if nat_eq(a5550, a5551) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5552 = v0;
                                                                                                                                            let a5553 = 42u64;
                                                                                                                                            if nat_eq(a5552, a5553) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5554 = v0;
                                                                                                                                                let a5555 = 43u64;
                                                                                                                                                if nat_eq(a5554, a5555) {
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

pub fn f214(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5557 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5556, _, _, _, _, _, _) => {
                    h5556
                }
            };
            let a5558 = 1u64;
            if if if if if nat_le(a5557, a5558) {
                true
            } else {
                let a5560 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5559, _, _) => {
                        h5559
                    }
                };
                let a5561 = v1;
                nat_eq(a5560, a5561)
            } {
                let a5563 = match v0.clone() {
                    Adt2::C0(_, h5562, _, _, _, _, _, _, _, _, _) => {
                        h5562
                    }
                };
                let a5564 = 1u64;
                if nat_le(a5563, a5564) {
                    true
                } else {
                    let a5569 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5565, _, _, _, _, _) => {
                            h5565
                        }
                    };
                    let a5570 = {
                        let a5567 = v1;
                        let a5568 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5566, _, _, _, _, _, _) => {
                                h5566
                            }
                        };
                        nat_mul(a5567, a5568)?
                    };
                    nat_eq(a5569, a5570)
                }
            } else {
                false
            } {
                let a5572 = match v0.clone() {
                    Adt2::C0(_, _, h5571, _, _, _, _, _, _, _, _) => {
                        h5571
                    }
                };
                let a5573 = 1u64;
                if nat_le(a5572, a5573) {
                    true
                } else {
                    let a5581 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5574, _, _, _, _) => {
                            h5574
                        }
                    };
                    let a5582 = {
                        let a5579 = {
                            let a5576 = v1;
                            let a5577 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5575, _, _, _, _, _, _) => {
                                    h5575
                                }
                            };
                            nat_mul(a5576, a5577)?
                        };
                        let a5580 = match v0.clone() {
                            Adt2::C0(_, h5578, _, _, _, _, _, _, _, _, _) => {
                                h5578
                            }
                        };
                        nat_mul(a5579, a5580)?
                    };
                    nat_eq(a5581, a5582)
                }
            } else {
                false
            } {
                let a5584 = match v0.clone() {
                    Adt2::C0(_, _, _, h5583, _, _, _, _, _, _, _) => {
                        h5583
                    }
                };
                let a5585 = 1u64;
                if nat_le(a5584, a5585) {
                    true
                } else {
                    let a5596 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5586, _, _, _) => {
                            h5586
                        }
                    };
                    let a5597 = {
                        let a5594 = {
                            let a5591 = {
                                let a5588 = v1;
                                let a5589 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5587, _, _, _, _, _, _) => {
                                        h5587
                                    }
                                };
                                nat_mul(a5588, a5589)?
                            };
                            let a5592 = match v0.clone() {
                                Adt2::C0(_, h5590, _, _, _, _, _, _, _, _, _) => {
                                    h5590
                                }
                            };
                            nat_mul(a5591, a5592)?
                        };
                        let a5595 = match v0.clone() {
                            Adt2::C0(_, _, h5593, _, _, _, _, _, _, _, _) => {
                                h5593
                            }
                        };
                        nat_mul(a5594, a5595)?
                    };
                    nat_eq(a5596, a5597)
                }
            } else {
                false
            } {
                let a5611 = {
                    let a5608 = {
                        let a5605 = {
                            let a5602 = {
                                let a5599 = v1;
                                let a5600 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5598, _, _, _, _, _, _) => {
                                        h5598
                                    }
                                };
                                nat_mul(a5599, a5600)?
                            };
                            let a5603 = match v0.clone() {
                                Adt2::C0(_, h5601, _, _, _, _, _, _, _, _, _) => {
                                    h5601
                                }
                            };
                            nat_mul(a5602, a5603)?
                        };
                        let a5606 = match v0.clone() {
                            Adt2::C0(_, _, h5604, _, _, _, _, _, _, _, _) => {
                                h5604
                            }
                        };
                        nat_mul(a5605, a5606)?
                    };
                    let a5609 = match v0.clone() {
                        Adt2::C0(_, _, _, h5607, _, _, _, _, _, _, _) => {
                            h5607
                        }
                    };
                    nat_mul(a5608, a5609)?
                };
                let a5612 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5610, _) => {
                        h5610
                    }
                };
                Ok(nat_le(a5611, a5612))
            } else {
                Ok(false)
            }
        } else {
            let a5614 = match v0.clone() {
                Adt2::C0(_, _, _, h5613, _, _, _, _, _, _, _) => {
                    h5613
                }
            };
            let a5615 = 1u64;
            if if if if if nat_le(a5614, a5615) {
                true
            } else {
                let a5617 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5616, _, _, _) => {
                        h5616
                    }
                };
                let a5618 = v1;
                nat_eq(a5617, a5618)
            } {
                let a5620 = match v0.clone() {
                    Adt2::C0(_, h5619, _, _, _, _, _, _, _, _, _) => {
                        h5619
                    }
                };
                let a5621 = 1u64;
                if nat_le(a5620, a5621) {
                    true
                } else {
                    let a5626 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5622, _, _, _, _, _) => {
                            h5622
                        }
                    };
                    let a5627 = {
                        let a5624 = v1;
                        let a5625 = match v0.clone() {
                            Adt2::C0(_, _, _, h5623, _, _, _, _, _, _, _) => {
                                h5623
                            }
                        };
                        nat_mul(a5624, a5625)?
                    };
                    nat_eq(a5626, a5627)
                }
            } else {
                false
            } {
                let a5629 = match v0.clone() {
                    Adt2::C0(_, _, h5628, _, _, _, _, _, _, _, _) => {
                        h5628
                    }
                };
                let a5630 = 1u64;
                if nat_le(a5629, a5630) {
                    true
                } else {
                    let a5638 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5631, _, _, _, _) => {
                            h5631
                        }
                    };
                    let a5639 = {
                        let a5636 = {
                            let a5633 = v1;
                            let a5634 = match v0.clone() {
                                Adt2::C0(_, _, _, h5632, _, _, _, _, _, _, _) => {
                                    h5632
                                }
                            };
                            nat_mul(a5633, a5634)?
                        };
                        let a5637 = match v0.clone() {
                            Adt2::C0(_, h5635, _, _, _, _, _, _, _, _, _) => {
                                h5635
                            }
                        };
                        nat_mul(a5636, a5637)?
                    };
                    nat_eq(a5638, a5639)
                }
            } else {
                false
            } {
                let a5641 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5640, _, _, _, _, _, _) => {
                        h5640
                    }
                };
                let a5642 = 1u64;
                if nat_le(a5641, a5642) {
                    true
                } else {
                    let a5653 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5643, _, _) => {
                            h5643
                        }
                    };
                    let a5654 = {
                        let a5651 = {
                            let a5648 = {
                                let a5645 = v1;
                                let a5646 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5644, _, _, _, _, _, _, _) => {
                                        h5644
                                    }
                                };
                                nat_mul(a5645, a5646)?
                            };
                            let a5649 = match v0.clone() {
                                Adt2::C0(_, h5647, _, _, _, _, _, _, _, _, _) => {
                                    h5647
                                }
                            };
                            nat_mul(a5648, a5649)?
                        };
                        let a5652 = match v0.clone() {
                            Adt2::C0(_, _, h5650, _, _, _, _, _, _, _, _) => {
                                h5650
                            }
                        };
                        nat_mul(a5651, a5652)?
                    };
                    nat_eq(a5653, a5654)
                }
            } else {
                false
            } {
                let a5668 = {
                    let a5665 = {
                        let a5662 = {
                            let a5659 = {
                                let a5656 = v1;
                                let a5657 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5655, _, _, _, _, _, _, _) => {
                                        h5655
                                    }
                                };
                                nat_mul(a5656, a5657)?
                            };
                            let a5660 = match v0.clone() {
                                Adt2::C0(_, h5658, _, _, _, _, _, _, _, _, _) => {
                                    h5658
                                }
                            };
                            nat_mul(a5659, a5660)?
                        };
                        let a5663 = match v0.clone() {
                            Adt2::C0(_, _, h5661, _, _, _, _, _, _, _, _) => {
                                h5661
                            }
                        };
                        nat_mul(a5662, a5663)?
                    };
                    let a5666 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5664, _, _, _, _, _, _) => {
                            h5664
                        }
                    };
                    nat_mul(a5665, a5666)?
                };
                let a5669 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5667, _) => {
                        h5667
                    }
                };
                Ok(nat_le(a5668, a5669))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5671 = match v0.clone() {
            Adt2::C0(_, h5670, _, _, _, _, _, _, _, _, _) => {
                h5670
            }
        };
        let a5672 = 1u64;
        if if if if if nat_le(a5671, a5672) {
            true
        } else {
            let a5674 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5673, _, _, _, _, _) => {
                    h5673
                }
            };
            let a5675 = v1;
            nat_eq(a5674, a5675)
        } {
            let a5677 = match v0.clone() {
                Adt2::C0(_, _, h5676, _, _, _, _, _, _, _, _) => {
                    h5676
                }
            };
            let a5678 = 1u64;
            if nat_le(a5677, a5678) {
                true
            } else {
                let a5683 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5679, _, _, _, _) => {
                        h5679
                    }
                };
                let a5684 = {
                    let a5681 = v1;
                    let a5682 = match v0.clone() {
                        Adt2::C0(_, h5680, _, _, _, _, _, _, _, _, _) => {
                            h5680
                        }
                    };
                    nat_mul(a5681, a5682)?
                };
                nat_eq(a5683, a5684)
            }
        } else {
            false
        } {
            let a5686 = match v0.clone() {
                Adt2::C0(_, _, _, h5685, _, _, _, _, _, _, _) => {
                    h5685
                }
            };
            let a5687 = 1u64;
            if nat_le(a5686, a5687) {
                true
            } else {
                let a5695 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5688, _, _, _) => {
                        h5688
                    }
                };
                let a5696 = {
                    let a5693 = {
                        let a5690 = v1;
                        let a5691 = match v0.clone() {
                            Adt2::C0(_, h5689, _, _, _, _, _, _, _, _, _) => {
                                h5689
                            }
                        };
                        nat_mul(a5690, a5691)?
                    };
                    let a5694 = match v0.clone() {
                        Adt2::C0(_, _, h5692, _, _, _, _, _, _, _, _) => {
                            h5692
                        }
                    };
                    nat_mul(a5693, a5694)?
                };
                nat_eq(a5695, a5696)
            }
        } else {
            false
        } {
            let a5698 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5697, _, _, _, _, _, _) => {
                    h5697
                }
            };
            let a5699 = 1u64;
            if nat_le(a5698, a5699) {
                true
            } else {
                let a5710 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5700, _, _) => {
                        h5700
                    }
                };
                let a5711 = {
                    let a5708 = {
                        let a5705 = {
                            let a5702 = v1;
                            let a5703 = match v0.clone() {
                                Adt2::C0(_, h5701, _, _, _, _, _, _, _, _, _) => {
                                    h5701
                                }
                            };
                            nat_mul(a5702, a5703)?
                        };
                        let a5706 = match v0.clone() {
                            Adt2::C0(_, _, h5704, _, _, _, _, _, _, _, _) => {
                                h5704
                            }
                        };
                        nat_mul(a5705, a5706)?
                    };
                    let a5709 = match v0.clone() {
                        Adt2::C0(_, _, _, h5707, _, _, _, _, _, _, _) => {
                            h5707
                        }
                    };
                    nat_mul(a5708, a5709)?
                };
                nat_eq(a5710, a5711)
            }
        } else {
            false
        } {
            let a5725 = {
                let a5722 = {
                    let a5719 = {
                        let a5716 = {
                            let a5713 = v1;
                            let a5714 = match v0.clone() {
                                Adt2::C0(_, h5712, _, _, _, _, _, _, _, _, _) => {
                                    h5712
                                }
                            };
                            nat_mul(a5713, a5714)?
                        };
                        let a5717 = match v0.clone() {
                            Adt2::C0(_, _, h5715, _, _, _, _, _, _, _, _) => {
                                h5715
                            }
                        };
                        nat_mul(a5716, a5717)?
                    };
                    let a5720 = match v0.clone() {
                        Adt2::C0(_, _, _, h5718, _, _, _, _, _, _, _) => {
                            h5718
                        }
                    };
                    nat_mul(a5719, a5720)?
                };
                let a5723 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5721, _, _, _, _, _, _) => {
                        h5721
                    }
                };
                nat_mul(a5722, a5723)?
            };
            let a5726 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5724, _) => {
                    h5724
                }
            };
            Ok(nat_le(a5725, a5726))
        } else {
            Ok(false)
        }
    }
}

pub fn f215(v0: Adt2, v1: u64) -> R<bool> {
    let a5728 = match v0.clone() {
        Adt2::C0(h5727, _, _, _, _, _, _, _, _, _, _) => {
            h5727
        }
    };
    let a5729 = 0u64;
    if if nat_eq(a5728, a5729) {
        f38(v0.clone())?
    } else {
        false
    } {
        let a5730 = v1;
        let a5731 = 1u64;
        if nat_le(a5730, a5731) {
            Ok(true)
        } else {
            let a5736 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5732, _, _, _, _) => {
                    h5732
                }
            };
            let a5737 = {
                let a5734 = 4u64;
                let a5735 = match v0 {
                    Adt2::C0(_, h5733, _, _, _, _, _, _, _, _, _) => {
                        h5733
                    }
                };
                nat_mul(a5734, a5735)?
            };
            Ok(nat_eq(a5736, a5737))
        }
    } else {
        Ok(false)
    }
}

pub fn f216(v0: Adt2, v1: u64) -> R<bool> {
    let a5739 = match v0.clone() {
        Adt2::C0(_, _, _, h5738, _, _, _, _, _, _, _) => {
            h5738
        }
    };
    let a5740 = 1u64;
    if if if nat_eq(a5739, a5740) {
        let a5742 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5741, _, _, _, _, _, _) => {
                h5741
            }
        };
        let a5743 = 1u64;
        nat_eq(a5742, a5743)
    } else {
        false
    } {
        let a5745 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5744, _, _, _, _, _) => {
                h5744
            }
        };
        let a5746 = v1;
        nat_eq(a5745, a5746)
    } else {
        false
    } {
        let a5748 = match v0.clone() {
            Adt2::C0(_, _, h5747, _, _, _, _, _, _, _, _) => {
                h5747
            }
        };
        let a5749 = 1u64;
        if nat_eq(a5748, a5749) {
            Ok(true)
        } else {
            let a5754 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5750, _, _, _, _) => {
                    h5750
                }
            };
            let a5755 = {
                let a5752 = match v0 {
                    Adt2::C0(_, h5751, _, _, _, _, _, _, _, _, _) => {
                        h5751
                    }
                };
                let a5753 = v1;
                nat_mul(a5752, a5753)?
            };
            Ok(nat_eq(a5754, a5755))
        }
    } else {
        Ok(false)
    }
}

pub fn f217(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5759 = {
            let a5756 = v2;
            let a5757 = 1u64;
            nat_sub(a5756, a5757)
        };
        let a5760 = match v0.clone() {
            Adt6::C0(_, h5758, _, _) => {
                h5758
            }
        };
        nat_mul(a5759, a5760)?
    };
    let a5762 = 0u64;
    let a5763 = match v0.clone() {
        Adt6::C0(h5761, _, _, _) => {
            h5761
        }
    };
    if if if if if if if if nat_lt(a5762, a5763) {
        let a5765 = match v0.clone() {
            Adt6::C0(h5764, _, _, _) => {
                h5764
            }
        };
        let a5766 = 2147483648u64;
        nat_lt(a5765, a5766)
    } else {
        false
    } {
        let a5768 = 0u64;
        let a5769 = match v0.clone() {
            Adt6::C0(_, h5767, _, _) => {
                h5767
            }
        };
        nat_lt(a5768, a5769)
    } else {
        false
    } {
        let a5771 = match v0.clone() {
            Adt6::C0(_, h5770, _, _) => {
                h5770
            }
        };
        let a5772 = 2147483648u64;
        nat_lt(a5771, a5772)
    } else {
        false
    } {
        let a5773 = v1;
        let a5774 = 2147483648u64;
        nat_lt(a5773, a5774)
    } else {
        false
    } {
        let a5775 = 0u64;
        let a5776 = v2;
        nat_lt(a5775, a5776)
    } else {
        false
    } {
        let a5777 = v3;
        let a5778 = 2147483648u64;
        nat_lt(a5777, a5778)
    } else {
        false
    } {
        let a5790 = {
            let a5785 = {
                let a5782 = v3;
                let a5783 = {
                    let a5780 = match v0.clone() {
                        Adt6::C0(h5779, _, _, _) => {
                            h5779
                        }
                    };
                    let a5781 = 1u64;
                    nat_sub(a5780, a5781)
                };
                nat_add(a5782, a5783)?
            };
            let a5786 = f141(match v0.clone() {
                Adt6::C0(_, _, h5784, _) => {
                    h5784
                }
            });
            nat_add(a5785, a5786)?
        };
        let a5791 = {
            let a5788 = 2147483647u64;
            let a5789 = f142(match v0.clone() {
                Adt6::C0(_, _, h5787, _) => {
                    h5787
                }
            });
            nat_add(a5788, a5789)?
        };
        nat_le(a5790, a5791)
    } else {
        false
    } {
        let a5793 = match v0 {
            Adt6::C0(_, _, h5792, _) => {
                h5792
            }
        };
        let a5794 = 2147483648u64;
        Ok(!nat_eq(a5793, a5794))
    } else {
        Ok(false)
    }
}

pub fn f218(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f267(match v0.clone() {
        Adt2::C0(h5795, _, _, _, _, _, _, _, _, _, _) => {
            h5795
        }
    }, match v0.clone() {
        Adt2::C0(_, h5796, _, _, _, _, _, _, _, _, _) => {
            h5796
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5797, _, _, _, _, _, _, _, _) => {
            h5797
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5798, _, _, _, _, _, _, _) => {
            h5798
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5799, _, _, _, _, _, _) => {
            h5799
        }
    }, match v1.clone() {
        Adt2::C0(h5800, _, _, _, _, _, _, _, _, _, _) => {
            h5800
        }
    }, match v1.clone() {
        Adt2::C0(_, h5801, _, _, _, _, _, _, _, _, _) => {
            h5801
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5802, _, _, _, _, _, _, _, _) => {
            h5802
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5803, _, _, _, _, _, _, _) => {
            h5803
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5804, _, _, _, _, _, _) => {
            h5804
        }
    }, match v2.clone() {
        Adt2::C0(h5805, _, _, _, _, _, _, _, _, _, _) => {
            h5805
        }
    }, match v2.clone() {
        Adt2::C0(_, h5806, _, _, _, _, _, _, _, _, _) => {
            h5806
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5807, _, _, _, _, _, _, _, _) => {
            h5807
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5808, _, _, _, _, _, _, _) => {
            h5808
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5809, _, _, _, _, _, _) => {
            h5809
        }
    }) {
        let m5811 = f213(match v0.clone() {
            Adt2::C0(h5810, _, _, _, _, _, _, _, _, _, _) => {
                h5810
            }
        });
        match m5811 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5817 = {
                    let a5814 = match v0.clone() {
                        Adt2::C0(_, h5812, _, _, _, _, _, _, _, _, _) => {
                            h5812
                        }
                    };
                    let a5815 = match v3.clone() {
                        Adt19::C0(_, h5813, _) => {
                            h5813
                        }
                    };
                    let a5816 = 0u64;
                    nat_rem(a5814, a5815, a5816)
                };
                let a5818 = 0u64;
                if if nat_eq(a5817, a5818) {
                    if f268(match v0.clone() {
                        Adt2::C0(_, h5819, _, _, _, _, _, _, _, _, _) => {
                            h5819
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5820, _, _, _, _, _, _, _, _, _) => {
                            h5820
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5821, _, _, _, _, _, _, _, _) => {
                            h5821
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5822, _, _, _, _, _, _, _) => {
                            h5822
                        }
                    }) {
                        true
                    } else {
                        let a5824 = 4u64;
                        let a5825 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5823, _, _, _, _, _) => {
                                h5823
                            }
                        };
                        if if nat_le(a5824, a5825) {
                            let a5827 = match v3.clone() {
                                Adt19::C0(_, h5826, _) => {
                                    h5826
                                }
                            };
                            let a5828 = 1u64;
                            if nat_eq(a5827, a5828) {
                                true
                            } else {
                                let a5831 = match v3.clone() {
                                    Adt19::C0(_, _, h5829) => {
                                        h5829
                                    }
                                };
                                let a5832 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5830, _, _, _, _, _) => {
                                        h5830
                                    }
                                };
                                nat_le(a5831, a5832)
                            }
                        } else {
                            false
                        } {
                            if f30(v0.clone(), v2.clone())? {
                                f30(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f25(v0.clone(), match v3 {
                        Adt19::C0(_, h5833, _) => {
                            h5833
                        }
                    });
                    Ok(Adt10::C4(if f268(match v0.clone() {
                        Adt2::C0(_, h5834, _, _, _, _, _, _, _, _, _) => {
                            h5834
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5835, _, _, _, _, _, _, _, _, _) => {
                            h5835
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5836, _, _, _, _, _, _, _, _) => {
                            h5836
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5837, _, _, _, _, _, _, _) => {
                            h5837
                        }
                    }) {
                        0u64
                    } else {
                        f177(v4.clone(), f25(v1.clone(), 1u64))?
                    }, 4u64, v4, f25(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5838, _, _, _, _, _, _, _, _, _, _) => {
                            h5838
                        }
                    }, f25(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f219(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f220(match v0.clone() {
        Adt2::C0(h5839, _, _, _, _, _, _, _, _, _, _) => {
            h5839
        }
    }) {
        let a5845 = {
            let a5842 = match v0.clone() {
                Adt2::C0(_, h5840, _, _, _, _, _, _, _, _, _) => {
                    h5840
                }
            };
            let a5843 = f250(match v0.clone() {
                Adt2::C0(h5841, _, _, _, _, _, _, _, _, _, _) => {
                    h5841
                }
            });
            let a5844 = 1u64;
            nat_rem(a5842, a5843, a5844)
        };
        let a5846 = 0u64;
        nat_eq(a5845, a5846)
    } else {
        false
    } {
        let a5848 = match v1.clone() {
            Adt2::C0(h5847, _, _, _, _, _, _, _, _, _, _) => {
                h5847
            }
        };
        let a5849 = 26u64;
        nat_eq(a5848, a5849)
    } else {
        false
    } {
        let a5851 = match v2.clone() {
            Adt2::C0(h5850, _, _, _, _, _, _, _, _, _, _) => {
                h5850
            }
        };
        let a5852 = 0u64;
        nat_eq(a5851, a5852)
    } else {
        false
    } {
        let a5855 = match v0.clone() {
            Adt2::C0(_, h5853, _, _, _, _, _, _, _, _, _) => {
                h5853
            }
        };
        let a5856 = match v2.clone() {
            Adt2::C0(_, h5854, _, _, _, _, _, _, _, _, _) => {
                h5854
            }
        };
        nat_eq(a5855, a5856)
    } else {
        false
    } {
        let a5859 = match v1.clone() {
            Adt2::C0(_, h5857, _, _, _, _, _, _, _, _, _) => {
                h5857
            }
        };
        let a5860 = match v0.clone() {
            Adt2::C0(_, _, h5858, _, _, _, _, _, _, _, _) => {
                h5858
            }
        };
        nat_le(a5859, a5860)
    } else {
        false
    } {
        let a5862 = match v0.clone() {
            Adt2::C0(_, _, _, h5861, _, _, _, _, _, _, _) => {
                h5861
            }
        };
        let a5863 = 1u64;
        nat_eq(a5862, a5863)
    } else {
        false
    } {
        let a5865 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5864, _, _, _, _, _, _) => {
                h5864
            }
        };
        let a5866 = 1u64;
        nat_eq(a5865, a5866)
    } else {
        false
    } {
        let a5868 = match v2.clone() {
            Adt2::C0(_, _, _, h5867, _, _, _, _, _, _, _) => {
                h5867
            }
        };
        let a5869 = 1u64;
        nat_eq(a5868, a5869)
    } else {
        false
    } {
        let a5871 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5870, _, _, _, _, _, _) => {
                h5870
            }
        };
        let a5872 = 1u64;
        nat_eq(a5871, a5872)
    } else {
        false
    } {
        let a5874 = match v1.clone() {
            Adt2::C0(_, _, h5873, _, _, _, _, _, _, _, _) => {
                h5873
            }
        };
        let a5875 = 1u64;
        nat_eq(a5874, a5875)
    } else {
        false
    } {
        let a5877 = match v1.clone() {
            Adt2::C0(_, _, _, h5876, _, _, _, _, _, _, _) => {
                h5876
            }
        };
        let a5878 = 1u64;
        nat_eq(a5877, a5878)
    } else {
        false
    } {
        let a5880 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5879, _, _, _, _, _, _) => {
                h5879
            }
        };
        let a5881 = 1u64;
        nat_eq(a5880, a5881)
    } else {
        false
    } {
        if f48(match v2.clone() {
            Adt2::C0(_, h5882, _, _, _, _, _, _, _, _, _) => {
                h5882
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5883, _, _, _, _, _, _, _, _) => {
                h5883
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5884, _, _, _, _, _, _, _) => {
                h5884
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5885, _, _, _, _, _, _) => {
                h5885
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5887 = match v1.clone() {
                Adt2::C0(_, h5886, _, _, _, _, _, _, _, _, _) => {
                    h5886
                }
            };
            let a5888 = 0u64;
            if nat_eq(a5887, a5888) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5890 = match v0.clone() {
                    Adt2::C0(_, h5889, _, _, _, _, _, _, _, _, _) => {
                        h5889
                    }
                };
                let a5891 = 2147483647u64;
                if if if if if if nat_le(a5890, a5891) {
                    let a5893 = match v1.clone() {
                        Adt2::C0(_, h5892, _, _, _, _, _, _, _, _, _) => {
                            h5892
                        }
                    };
                    let a5894 = 2147483647u64;
                    nat_le(a5893, a5894)
                } else {
                    false
                } {
                    let a5896 = 4u64;
                    let a5897 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5895, _, _, _, _, _) => {
                            h5895
                        }
                    };
                    nat_le(a5896, a5897)
                } else {
                    false
                } {
                    let a5903 = {
                        let a5900 = match v2.clone() {
                            Adt2::C0(_, h5898, _, _, _, _, _, _, _, _, _) => {
                                h5898
                            }
                        };
                        let a5901 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5899, _, _, _, _, _) => {
                                h5899
                            }
                        };
                        nat_mul(a5900, a5901)?
                    };
                    let a5904 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5902, _, _, _, _) => {
                            h5902
                        }
                    };
                    nat_le(a5903, a5904)
                } else {
                    false
                } {
                    f30(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f30(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5917 = {
                            let a5908 = {
                                let a5906 = 2u64;
                                let a5907 = match v1.clone() {
                                    Adt2::C0(_, h5905, _, _, _, _, _, _, _, _, _) => {
                                        h5905
                                    }
                                };
                                nat_mul(a5906, a5907)?
                            };
                            let a5909 = f23(v2.clone(), 1u64)?;
                            nat_add(a5908, a5909)?
                        };
                        let a5918 = {
                            let a5915 = match v1.clone() {
                                Adt2::C0(_, h5910, _, _, _, _, _, _, _, _, _) => {
                                    h5910
                                }
                            };
                            let a5916 = {
                                let a5913 = 1u64;
                                let a5914 = f174(match v0.clone() {
                                    Adt2::C0(h5911, _, _, _, _, _, _, _, _, _, _) => {
                                        h5911
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5912, _, _, _, _, _, _, _, _, _) => {
                                        h5912
                                    }
                                })?;
                                nat_add(a5913, a5914)?
                            };
                            nat_mul(a5915, a5916)?
                        };
                        nat_add(a5917, a5918)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5919, _, _, _, _, _, _, _, _, _) => {
                            h5919
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5920, _, _, _, _, _, _, _, _, _) => {
                            h5920
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5921, _, _, _, _, _) => {
                            h5921
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5922, _, _, _, _) => {
                            h5922
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5923, _, _, _) => {
                            h5923
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5924, _, _) => {
                            h5924
                        }
                    }), f25(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5925, _, _, _, _, _, _, _, _, _, _) => {
                            h5925
                        }
                    }, f25(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f220(v0: u64) -> bool {
    let a5926 = v0;
    let a5927 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5926, a5927) {
        true
    } else {
        let a5928 = v0;
        let a5929 = 1u64;
        nat_eq(a5928, a5929)
    } {
        true
    } else {
        let a5930 = v0;
        let a5931 = 2u64;
        nat_eq(a5930, a5931)
    } {
        true
    } else {
        let a5932 = v0;
        let a5933 = 3u64;
        nat_eq(a5932, a5933)
    } {
        true
    } else {
        let a5934 = v0;
        let a5935 = 6u64;
        nat_eq(a5934, a5935)
    } {
        true
    } else {
        let a5936 = v0;
        let a5937 = 7u64;
        nat_eq(a5936, a5937)
    } {
        true
    } else {
        let a5938 = v0;
        let a5939 = 8u64;
        nat_eq(a5938, a5939)
    } {
        true
    } else {
        let a5940 = v0;
        let a5941 = 10u64;
        nat_eq(a5940, a5941)
    } {
        true
    } else {
        let a5942 = v0;
        let a5943 = 11u64;
        nat_eq(a5942, a5943)
    } {
        true
    } else {
        let a5944 = v0;
        let a5945 = 12u64;
        nat_eq(a5944, a5945)
    } {
        true
    } else {
        let a5946 = v0;
        let a5947 = 13u64;
        nat_eq(a5946, a5947)
    } {
        true
    } else {
        let a5948 = v0;
        let a5949 = 14u64;
        nat_eq(a5948, a5949)
    } {
        true
    } else {
        let a5950 = v0;
        let a5951 = 16u64;
        nat_eq(a5950, a5951)
    } {
        true
    } else {
        let a5952 = v0;
        let a5953 = 17u64;
        nat_eq(a5952, a5953)
    } {
        true
    } else {
        let a5954 = v0;
        let a5955 = 18u64;
        nat_eq(a5954, a5955)
    } {
        true
    } else {
        let a5956 = v0;
        let a5957 = 19u64;
        nat_eq(a5956, a5957)
    } {
        true
    } else {
        let a5958 = v0;
        let a5959 = 20u64;
        nat_eq(a5958, a5959)
    } {
        true
    } else {
        let a5960 = v0;
        let a5961 = 21u64;
        nat_eq(a5960, a5961)
    } {
        true
    } else {
        let a5962 = v0;
        let a5963 = 22u64;
        nat_eq(a5962, a5963)
    } {
        true
    } else {
        let a5964 = v0;
        let a5965 = 23u64;
        nat_eq(a5964, a5965)
    } {
        true
    } else {
        let a5966 = v0;
        let a5967 = 26u64;
        nat_eq(a5966, a5967)
    } {
        true
    } else {
        let a5968 = v0;
        let a5969 = 29u64;
        nat_eq(a5968, a5969)
    } {
        true
    } else {
        let a5970 = v0;
        let a5971 = 30u64;
        nat_eq(a5970, a5971)
    } {
        true
    } else {
        let a5972 = v0;
        let a5973 = 34u64;
        nat_eq(a5972, a5973)
    } {
        true
    } else {
        let a5974 = v0;
        let a5975 = 35u64;
        nat_eq(a5974, a5975)
    } {
        true
    } else {
        let a5976 = v0;
        let a5977 = 39u64;
        nat_eq(a5976, a5977)
    } {
        true
    } else {
        let a5978 = v0;
        let a5979 = 40u64;
        nat_eq(a5978, a5979)
    } {
        true
    } else {
        let a5980 = v0;
        let a5981 = 41u64;
        nat_eq(a5980, a5981)
    } {
        true
    } else {
        let a5982 = v0;
        let a5983 = 42u64;
        nat_eq(a5982, a5983)
    } {
        true
    } else {
        let a5984 = v0;
        let a5985 = 43u64;
        nat_eq(a5984, a5985)
    }
}

pub fn f221(v0: u64) -> bool {
    if f175(v0) {
        true
    } else {
        let a5986 = v0;
        let a5987 = 43u64;
        nat_eq(a5986, a5987)
    }
}

pub fn f222(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5990 = match v1.clone() {
        Adt2::C0(h5988, _, _, _, _, _, _, _, _, _, _) => {
            h5988
        }
    };
    let a5991 = match v3.clone() {
        Adt2::C0(h5989, _, _, _, _, _, _, _, _, _, _) => {
            h5989
        }
    };
    if if nat_eq(a5990, a5991) {
        let a5992 = v0;
        let a5993 = 33u64;
        if nat_eq(a5992, a5993) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5995 = f213(match v1.clone() {
            Adt2::C0(h5994, _, _, _, _, _, _, _, _, _, _) => {
                h5994
            }
        });
        match m5995 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5996, _) => {
                        h5996
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a6001 = {
                    let a5998 = match v1.clone() {
                        Adt2::C0(_, h5997, _, _, _, _, _, _, _, _, _) => {
                            h5997
                        }
                    };
                    let a5999 = v5;
                    let a6000 = 1u64;
                    nat_rem(a5998, a5999, a6000)
                };
                let a6002 = 0u64;
                if if if nat_eq(a6001, a6002) {
                    let a6007 = {
                        let a6004 = match v3.clone() {
                            Adt2::C0(_, h6003, _, _, _, _, _, _, _, _, _) => {
                                h6003
                            }
                        };
                        let a6005 = v5;
                        let a6006 = 1u64;
                        nat_rem(a6004, a6005, a6006)
                    };
                    let a6008 = 0u64;
                    nat_eq(a6007, a6008)
                } else {
                    false
                } {
                    let a6009 = v6;
                    let a6010 = f23(v3.clone(), v5)?;
                    nat_eq(a6009, a6010)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6011) => {
                            h6011
                        }
                    }, f25(v1, v5), f25(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f223(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f48(v4, v5, v6, v7) {
        f48(v0, v1, v2, v3)
    } else {
        let a6015 = {
            let a6012 = v0;
            let a6013 = v4;
            let a6014 = 1u64;
            nat_rem(a6012, a6013, a6014)
        };
        let a6016 = 0u64;
        if if if nat_eq(a6015, a6016) {
            let a6020 = {
                let a6017 = v1;
                let a6018 = v5;
                let a6019 = 1u64;
                nat_rem(a6017, a6018, a6019)
            };
            let a6021 = 0u64;
            nat_eq(a6020, a6021)
        } else {
            false
        } {
            let a6025 = {
                let a6022 = v2;
                let a6023 = v6;
                let a6024 = 1u64;
                nat_rem(a6022, a6023, a6024)
            };
            let a6026 = 0u64;
            nat_eq(a6025, a6026)
        } else {
            false
        } {
            let a6030 = {
                let a6027 = v3;
                let a6028 = v7;
                let a6029 = 1u64;
                nat_rem(a6027, a6028, a6029)
            };
            let a6031 = 0u64;
            nat_eq(a6030, a6031)
        } else {
            false
        }
    }
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6032 = v0;
    let a6033 = v1;
    if nat_eq(a6032, a6033) {
        let a6036 = v4;
        let a6037 = {
            let a6034 = v2;
            let a6035 = v3;
            nat_add(a6034, a6035)?
        };
        Ok(nat_eq(a6036, a6037))
    } else {
        let a6038 = v2;
        let a6039 = v3;
        if nat_eq(a6038, a6039) {
            let a6040 = v4;
            let a6041 = v2;
            Ok(nat_eq(a6040, a6041))
        } else {
            Ok(false)
        }
    }
}

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6044 = {
            let a6042 = v0;
            let a6043 = f142(v1);
            nat_add(a6042, a6043)?
        };
        let a6045 = f142(v2);
        nat_add(a6044, a6045)?
    };
    let v5: u64 = {
        let a6046 = f141(v1);
        let a6047 = f141(v2);
        nat_add(a6046, a6047)?
    };
    let a6048 = v1;
    let a6049 = 4294967295u64;
    if if nat_le(a6048, a6049) {
        let a6050 = v2;
        let a6051 = 4294967295u64;
        nat_le(a6050, a6051)
    } else {
        false
    } {
        let a6052 = v5;
        let a6053 = v4;
        if nat_le(a6052, a6053) {
            let a6056 = v3;
            let a6057 = {
                let a6054 = v4;
                let a6055 = v5;
                nat_sub(a6054, a6055)
            };
            Ok(nat_eq(a6056, a6057))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f226(v0: u64, v1: u64) -> R<bool> {
    let a6058 = v1;
    let a6059 = 4294967295u64;
    if nat_le(a6058, a6059) {
        let a6062 = {
            let a6060 = f142(v1);
            let a6061 = f141(v1);
            nat_add(a6060, a6061)?
        };
        let a6063 = v0;
        Ok(nat_lt(a6062, a6063))
    } else {
        Ok(false)
    }
}

pub fn f227(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f269(v0.clone(), false)?;
    let v6: u64 = f269(v0, true)?;
    let a6064 = v6;
    let a6065 = v1;
    if if nat_le(a6064, a6065) {
        let a6072 = {
            let a6070 = {
                let a6066 = v1;
                let a6067 = v5;
                nat_add(a6066, a6067)?
            };
            let a6071 = {
                let a6068 = v3;
                let a6069 = 0u64;
                if nat_eq(a6068, a6069) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6070, a6071)?
        };
        let a6073 = v2;
        nat_le(a6072, a6073)
    } else {
        false
    } {
        if v4 {
            let a6078 = {
                let a6076 = {
                    let a6074 = v1;
                    let a6075 = v5;
                    nat_add(a6074, a6075)?
                };
                let a6077 = v6;
                nat_sub(a6076, a6077)
            };
            let a6079 = v2;
            Ok(nat_lt(a6078, a6079))
        } else {
            let a6084 = {
                let a6082 = {
                    let a6080 = v1;
                    let a6081 = v5;
                    nat_add(a6080, a6081)?
                };
                let a6083 = v6;
                nat_sub(a6082, a6083)
            };
            let a6085 = v2;
            Ok(nat_le(a6084, a6085))
        }
    } else {
        Ok(false)
    }
}

pub fn f228(v0: u64, v1: u64) -> bool {
    let a6086 = v0;
    let a6087 = 0u64;
    if if nat_eq(a6086, a6087) {
        true
    } else {
        let a6088 = v0;
        let a6089 = 1u64;
        nat_eq(a6088, a6089)
    } {
        let a6090 = v1;
        let a6091 = 4294967296u64;
        nat_lt(a6090, a6091)
    } else {
        false
    }
}

pub fn f229(v0: u64, v1: u64) -> R<u64> {
    let a6096 = {
        let a6094 = v0;
        let a6095 = {
            let a6092 = v1;
            let a6093 = 1u64;
            nat_sub(a6092, a6093)
        };
        nat_add(a6094, a6095)?
    };
    let a6097 = v1;
    let a6098 = 0u64;
    Ok(nat_quot(a6096, a6097, a6098))
}

pub fn f230(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f48(match v0.clone() {
        Adt2::C0(_, h6099, _, _, _, _, _, _, _, _, _) => {
            h6099
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6100, _, _, _, _, _, _, _, _) => {
            h6100
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6101, _, _, _, _, _, _, _) => {
            h6101
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6102, _, _, _, _, _, _) => {
            h6102
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6105 = match v1.clone() {
                Adt2::C0(_, h6103, _, _, _, _, _, _, _, _, _) => {
                    h6103
                }
            };
            let a6106 = match v1.clone() {
                Adt2::C0(_, _, h6104, _, _, _, _, _, _, _, _) => {
                    h6104
                }
            };
            nat_mul(a6105, a6106)?
        };
        let v5: u64 = {
            let a6108 = v4;
            let a6109 = match v1.clone() {
                Adt2::C0(_, _, _, h6107, _, _, _, _, _, _, _) => {
                    h6107
                }
            };
            nat_mul(a6108, a6109)?
        };
        let a6122 = {
            let a6120 = {
                let a6111 = match v0.clone() {
                    Adt2::C0(_, h6110, _, _, _, _, _, _, _, _, _) => {
                        h6110
                    }
                };
                let a6112 = 1u64;
                nat_sub(a6111, a6112)
            };
            let a6121 = {
                let a6118 = f270(match v0.clone() {
                    Adt2::C0(_, _, h6113, _, _, _, _, _, _, _, _) => {
                        h6113
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6114, _, _, _, _, _, _, _, _, _) => {
                        h6114
                    }
                })?;
                let a6119 = f270(match v0 {
                    Adt2::C0(_, _, _, h6115, _, _, _, _, _, _, _) => {
                        h6115
                    }
                }, v2, {
                    let a6116 = v3;
                    let a6117 = v5;
                    nat_mul(a6116, a6117)?
                }, v4)?;
                nat_add(a6118, a6119)?
            };
            nat_add(a6120, a6121)?
        };
        let a6123 = f23(v1, 1u64)?;
        Ok(nat_lt(a6122, a6123))
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6124 = v0;
    let a6125 = 2147483648u64;
    if if if if nat_lt(a6124, a6125) {
        let a6126 = v1;
        let a6127 = 2147483648u64;
        nat_lt(a6126, a6127)
    } else {
        false
    } {
        let a6130 = {
            let a6128 = v0;
            let a6129 = v1;
            nat_mul(a6128, a6129)?
        };
        let a6131 = 2147483648u64;
        nat_lt(a6130, a6131)
    } else {
        false
    } {
        let a6136 = {
            let a6134 = {
                let a6132 = v0;
                let a6133 = 1u64;
                nat_sub(a6132, a6133)
            };
            let a6135 = v2;
            nat_mul(a6134, a6135)?
        };
        let a6137 = 2147483648u64;
        nat_lt(a6136, a6137)
    } else {
        false
    } {
        let a6142 = {
            let a6140 = {
                let a6138 = v1;
                let a6139 = 1u64;
                nat_sub(a6138, a6139)
            };
            let a6141 = v2;
            nat_mul(a6140, a6141)?
        };
        let a6143 = 2147483648u64;
        Ok(nat_lt(a6142, a6143))
    } else {
        Ok(false)
    }
}

pub fn f232(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6144 = v1;
    let a6145 = 13u64;
    if if nat_eq(a6144, a6145) {
        f48(match v0.clone() {
            Adt3::C0(h6146, _, _, _, _, _, _, _) => {
                h6146
            }
        }, match v0.clone() {
            Adt3::C0(_, h6147, _, _, _, _, _, _) => {
                h6147
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6148, _, _, _, _, _) => {
                h6148
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6149, _, _, _, _) => {
                h6149
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6155 = match v0.clone() {
                Adt3::C0(_, h6150, _, _, _, _, _, _) => {
                    h6150
                }
            };
            let a6156 = {
                let a6153 = match v0.clone() {
                    Adt3::C0(_, _, h6151, _, _, _, _, _) => {
                        h6151
                    }
                };
                let a6154 = match v0.clone() {
                    Adt3::C0(_, _, _, h6152, _, _, _, _) => {
                        h6152
                    }
                };
                nat_mul(a6153, a6154)?
            };
            nat_mul(a6155, a6156)?
        };
        let a6157 = v1;
        let a6158 = 13u64;
        if nat_eq(a6157, a6158) {
            let a6167 = 2u64;
            let a6168 = if f271(v0.clone(), v2) {
                let a6160 = v3;
                let a6161 = f244(match v0 {
                    Adt3::C0(h6159, _, _, _, _, _, _, _) => {
                        h6159
                    }
                }, false)?;
                nat_mul(a6160, a6161)?
            } else {
                let a6165 = v3;
                let a6166 = {
                    let a6163 = match v0 {
                        Adt3::C0(h6162, _, _, _, _, _, _, _) => {
                            h6162
                        }
                    };
                    let a6164 = 2u64;
                    nat_mul(a6163, a6164)?
                };
                nat_mul(a6165, a6166)?
            };
            nat_add(a6167, a6168)
        } else {
            let a6172 = v3;
            let a6173 = f244(match v0 {
                Adt3::C0(h6169, _, _, _, _, _, _, _) => {
                    h6169
                }
            }, {
                let a6170 = v1;
                let a6171 = 16u64;
                nat_eq(a6170, a6171)
            })?;
            nat_mul(a6172, a6173)
        }
    }
}

pub fn f233(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6174 = v0;
    let a6175 = 30u64;
    if if nat_eq(a6174, a6175) {
        let a6178 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6176, _, _, _, _) => {
                h6176
            }
        };
        let a6179 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6177, _, _, _, _, _) => {
                h6177
            }
        };
        nat_lt(a6178, a6179)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6180 = v3;
        match m6180 {
            None => {
                f272(v0, v1, match v2.clone() {
                    Adt2::C0(h6181, _, _, _, _, _, _, _, _, _, _) => {
                        h6181
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6182, _, _, _, _, _, _, _, _, _, _) => {
                        h6182
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6183, _, _, _, _, _, _, _, _, _) => {
                        h6183
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6184, _, _, _, _, _, _, _, _) => {
                        h6184
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6185, _, _, _, _, _, _, _) => {
                        h6185
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6186, _, _, _, _, _, _) => {
                        h6186
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6187, _, _, _, _, _, _, _, _, _) => {
                        h6187
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6188, _, _, _, _, _, _, _, _) => {
                        h6188
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6189, _, _, _, _, _, _, _) => {
                        h6189
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6190, _, _, _, _, _, _) => {
                        h6190
                    }
                })
            }
            Some(v5) => {
                f272(v0, v1, match v2.clone() {
                    Adt2::C0(h6191, _, _, _, _, _, _, _, _, _, _) => {
                        h6191
                    }
                }, match v5.clone() {
                    Adt2::C0(h6192, _, _, _, _, _, _, _, _, _, _) => {
                        h6192
                    }
                }, match v4.clone() {
                    Adt2::C0(h6193, _, _, _, _, _, _, _, _, _, _) => {
                        h6193
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6194, _, _, _, _, _, _, _, _, _) => {
                        h6194
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6195, _, _, _, _, _, _, _, _) => {
                        h6195
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6196, _, _, _, _, _, _, _) => {
                        h6196
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6197, _, _, _, _, _, _) => {
                        h6197
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6198, _, _, _, _, _, _, _, _, _) => {
                        h6198
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6199, _, _, _, _, _, _, _, _) => {
                        h6199
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6200, _, _, _, _, _, _, _) => {
                        h6200
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6201, _, _, _, _, _, _) => {
                        h6201
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6202, _, _, _, _, _, _, _, _, _) => {
                        h6202
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6203, _, _, _, _, _, _, _, _) => {
                        h6203
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6204, _, _, _, _, _, _, _) => {
                        h6204
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6205, _, _, _, _, _, _) => {
                        h6205
                    }
                })
            }
        }
    }
}

pub fn f234(v0: u64, v1: u64) -> u64 {
    let a6206 = v0;
    let a6207 = 23u64;
    if nat_eq(a6206, a6207) {
        let a6208 = v1;
        let a6209 = 0u64;
        if nat_eq(a6208, a6209) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f235(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6214 = {
        let a6212 = {
            let a6210 = v0;
            let a6211 = v1;
            nat_sub(a6210, a6211)
        };
        let a6213 = 1u64;
        nat_sub(a6212, a6213)
    };
    let a6215 = v2;
    nat_add(a6214, a6215)
}

pub fn f236(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f140({
        let a6216 = v0;
        let a6217 = v4;
        let a6218 = 0u64;
        nat_rem(a6216, a6217, a6218)
    }, {
        let a6219 = v1;
        let a6220 = v5;
        let a6221 = 0u64;
        nat_rem(a6219, a6220, a6221)
    }, {
        let a6222 = v2;
        let a6223 = v6;
        let a6224 = 0u64;
        nat_rem(a6222, a6223, a6224)
    }, {
        let a6225 = v3;
        let a6226 = v7;
        let a6227 = 0u64;
        nat_rem(a6225, a6226, a6227)
    }, v8, v9, v10, v11)
}

pub fn f237(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6228 = v1;
    let a6229 = 2147483648u64;
    if nat_lt(a6228, a6229) {
        let a6240 = {
            let a6238 = {
                let a6233 = {
                    let a6230 = v0;
                    let a6231 = v2;
                    let a6232 = 0u64;
                    nat_rem(a6230, a6231, a6232)
                };
                let a6234 = v2;
                nat_add(a6233, a6234)?
            };
            let a6239 = {
                let a6235 = v1;
                let a6236 = v2;
                let a6237 = 0u64;
                nat_rem(a6235, a6236, a6237)
            };
            nat_sub(a6238, a6239)
        };
        let a6241 = v2;
        let a6242 = 0u64;
        Ok(nat_rem(a6240, a6241, a6242))
    } else {
        let a6245 = {
            let a6243 = v0;
            let a6244 = f141(v1);
            nat_add(a6243, a6244)?
        };
        let a6246 = v2;
        let a6247 = 0u64;
        Ok(nat_rem(a6245, a6246, a6247))
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6250 = f142(v1);
    let a6251 = {
        let a6248 = v0;
        let a6249 = f141(v1);
        nat_add(a6248, a6249)?
    };
    if nat_le(a6250, a6251) {
        let a6252 = f239(v0, v1)?;
        let a6253 = v2;
        Ok(nat_lt(a6252, a6253))
    } else {
        Ok(false)
    }
}

pub fn f239(v0: u64, v1: u64) -> R<u64> {
    let a6256 = {
        let a6254 = v0;
        let a6255 = f141(v1);
        nat_add(a6254, a6255)?
    };
    let a6257 = f142(v1);
    Ok(nat_sub(a6256, a6257))
}

pub fn f240(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6258 = v0;
    let a6259 = v1;
    if nat_lt(a6258, a6259) {
        let a6260 = v1;
        let a6261 = v0;
        Ok(nat_sub(a6260, a6261))
    } else {
        let a6264 = v0;
        let a6265 = {
            let a6262 = v1;
            let a6263 = v2;
            nat_add(a6262, a6263)?
        };
        if nat_lt(a6264, a6265) {
            let a6266 = v0;
            let a6267 = v1;
            Ok(nat_sub(a6266, a6267))
        } else {
            let a6274 = {
                let a6270 = 2u64;
                let a6271 = {
                    let a6268 = v2;
                    let a6269 = 1u64;
                    nat_sub(a6268, a6269)
                };
                nat_mul(a6270, a6271)?
            };
            let a6275 = {
                let a6272 = v0;
                let a6273 = v1;
                nat_sub(a6272, a6273)
            };
            Ok(nat_sub(a6274, a6275))
        }
    }
}

pub fn f241(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6278 = {
        let a6276 = v2;
        let a6277 = f273(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a6276, a6277)?
    };
    let a6279 = f273(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6278, a6279))
}

pub fn f242(v0: u64, v1: u64, v2: u64) -> bool {
    let a6280 = v0;
    let a6281 = 0u64;
    if nat_eq(a6280, a6281) {
        let a6282 = v2;
        let a6283 = v1;
        nat_le(a6282, a6283)
    } else {
        let a6284 = v0;
        let a6285 = 1u64;
        if nat_eq(a6284, a6285) {
            let a6286 = v2;
            let a6287 = v1;
            nat_lt(a6286, a6287)
        } else {
            let a6288 = v0;
            let a6289 = 2u64;
            if nat_eq(a6288, a6289) {
                let a6290 = v1;
                let a6291 = v2;
                nat_le(a6290, a6291)
            } else {
                let a6292 = v0;
                let a6293 = 3u64;
                if nat_eq(a6292, a6293) {
                    let a6294 = v1;
                    let a6295 = v2;
                    nat_lt(a6294, a6295)
                } else {
                    let a6296 = v1;
                    let a6297 = v2;
                    nat_eq(a6296, a6297)
                }
            }
        }
    }
}

pub fn f243(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6298 = v0;
    let a6299 = 0u64;
    if nat_eq(a6298, a6299) {
        Ok(f256(0u64))
    } else {
        let a6302 = {
            let a6300 = v0;
            let a6301 = 1u64;
            nat_add(a6300, a6301)?
        };
        let a6303 = f232(v1.clone(), 13u64, v2)?;
        if nat_eq(a6302, a6303) {
            Ok(Adt9::C4({
                let a6304 = v2;
                let a6305 = 0u64;
                if nat_eq(a6304, a6305) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6306 = v0;
                let a6307 = 1u64;
                nat_sub(a6306, a6307)
            };
            if f271(v1.clone(), v2) {
                let v4: u64 = f244(match v1.clone() {
                    Adt3::C0(h6308, _, _, _, _, _, _, _) => {
                        h6308
                    }
                }, false)?;
                let v5: u64 = {
                    let a6309 = v3;
                    let a6310 = v4;
                    let a6311 = 0u64;
                    nat_quot(a6309, a6310, a6311)
                };
                let v6: u64 = {
                    let a6312 = v3;
                    let a6313 = v4;
                    let a6314 = 0u64;
                    nat_rem(a6312, a6313, a6314)
                };
                let a6317 = {
                    let a6315 = v6;
                    let a6316 = 1u64;
                    nat_add(a6315, a6316)?
                };
                let a6318 = v4;
                if nat_eq(a6317, a6318) {
                    Ok(Adt9::C4({
                        let a6319 = v2;
                        let a6320 = 0u64;
                        if nat_eq(a6319, a6320) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f245(v6, v5, v1, v2)
                }
            } else {
                f274({
                    let a6321 = v3;
                    let a6322 = 2u64;
                    let a6323 = 0u64;
                    nat_quot(a6321, a6322, a6323)
                }, {
                    let a6324 = v3;
                    let a6325 = 2u64;
                    let a6326 = 0u64;
                    nat_rem(a6324, a6325, a6326)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f244(v0: u64, v1: bool) -> R<u64> {
    let a6329 = {
        let a6327 = v0;
        let a6328 = 2u64;
        nat_mul(a6327, a6328)?
    };
    let a6330 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6329, a6330)
}

pub fn f245(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6331 = v0;
    let a6332 = 0u64;
    if nat_eq(a6331, a6332) {
        Ok(f256(1u64))
    } else {
        f274({
            let a6341 = {
                let a6334 = v1;
                let a6335 = match v2.clone() {
                    Adt3::C0(h6333, _, _, _, _, _, _, _) => {
                        h6333
                    }
                };
                nat_mul(a6334, a6335)?
            };
            let a6342 = {
                let a6338 = {
                    let a6336 = v0;
                    let a6337 = 1u64;
                    nat_sub(a6336, a6337)
                };
                let a6339 = 2u64;
                let a6340 = 0u64;
                nat_quot(a6338, a6339, a6340)
            };
            nat_add(a6341, a6342)?
        }, {
            let a6345 = {
                let a6343 = v0;
                let a6344 = 1u64;
                nat_sub(a6343, a6344)
            };
            let a6346 = 2u64;
            let a6347 = 0u64;
            nat_rem(a6345, a6346, a6347)
        }, v2, v3, 1u64)
    }
}

pub fn f246(v0: u64, v1: u64) -> R<u64> {
    let a6348 = v0;
    let a6349 = f94(v1)?;
    let a6350 = 0u64;
    Ok(nat_rem(a6348, a6349, a6350))
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let a6351 = 4u64;
    let a6352 = f275(v0, v1)?;
    nat_mul(a6351, a6352)
}

pub fn f248(v0: u64, v1: u64) -> R<u64> {
    let a6353 = 4u64;
    let a6354 = f276(v0, v1)?;
    nat_mul(a6353, a6354)
}

pub fn f249(v0: u64, v1: u64) -> R<bool> {
    let a6360 = {
        let a6357 = {
            let a6355 = f246(v0, v1)?;
            let a6356 = 2u64;
            nat_sub(a6355, a6356)
        };
        let a6358 = 2u64;
        let a6359 = 0u64;
        nat_rem(a6357, a6358, a6359)
    };
    let a6361 = 0u64;
    Ok(nat_eq(a6360, a6361))
}

pub fn f250(v0: u64) -> u64 {
    let m6362 = f213(v0);
    match m6362 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6363, _) => {
                    h6363
                }
            }
        }
    }
}

pub fn f251(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6364 = f213(v0);
    match m6364 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f220(v0) {
                let a6365 = v1;
                let a6366 = v7;
                nat_lt(a6365, a6366)
            } else {
                false
            } {
                f180(v8, v2)
            } else {
                false
            } {
                let a6367 = v3;
                let a6368 = f277(v4, v5, v6)?;
                nat_lt(a6367, a6368)
            } else {
                false
            } {
                let a6373 = {
                    let a6370 = v7;
                    let a6371 = match v13.clone() {
                        Adt19::C0(_, h6369, _) => {
                            h6369
                        }
                    };
                    let a6372 = 0u64;
                    nat_rem(a6370, a6371, a6372)
                };
                let a6374 = 0u64;
                nat_eq(a6373, a6374)
            } else {
                false
            } {
                Ok(Some({
                    let a6403 = {
                        let a6393 = {
                            let a6383 = {
                                let a6379 = {
                                    let a6376 = v1;
                                    let a6377 = match v13 {
                                        Adt19::C0(_, h6375, _) => {
                                            h6375
                                        }
                                    };
                                    let a6378 = 0u64;
                                    nat_quot(a6376, a6377, a6378)
                                };
                                let a6380 = v9;
                                nat_mul(a6379, a6380)?
                            };
                            let a6384 = {
                                let a6381 = v2;
                                let a6382 = v10;
                                nat_mul(a6381, a6382)?
                            };
                            nat_add(a6383, a6384)?
                        };
                        let a6394 = {
                            let a6391 = {
                                let a6388 = {
                                    let a6385 = v3;
                                    let a6386 = v4;
                                    let a6387 = 0u64;
                                    nat_quot(a6385, a6386, a6387)
                                };
                                let a6389 = v5;
                                let a6390 = 0u64;
                                nat_rem(a6388, a6389, a6390)
                            };
                            let a6392 = v11;
                            nat_mul(a6391, a6392)?
                        };
                        nat_add(a6393, a6394)?
                    };
                    let a6404 = {
                        let a6401 = {
                            let a6398 = {
                                let a6395 = v3;
                                let a6396 = v4;
                                let a6397 = 0u64;
                                nat_quot(a6395, a6396, a6397)
                            };
                            let a6399 = v5;
                            let a6400 = 0u64;
                            nat_quot(a6398, a6399, a6400)
                        };
                        let a6402 = v12;
                        nat_mul(a6401, a6402)?
                    };
                    nat_add(a6403, a6404)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f252(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f278(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f253(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f279(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f254(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6405 = v0;
    let a6406 = 64u64;
    if nat_eq(a6405, a6406) {
        f280(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6407 = v0;
        let a6408 = 65u64;
        if nat_eq(a6407, a6408) {
            f281(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f255(v0: u64) -> u64 {
    let a6409 = v0;
    let a6410 = 30u64;
    if nat_eq(a6409, a6410) {
        4u64
    } else {
        3u64
    }
}

pub fn f256(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f257(v0: u64) -> bool {
    let a6411 = 0u64;
    let a6412 = v0;
    if nat_lt(a6411, a6412) {
        let a6413 = v0;
        let a6414 = 2139095040u64;
        nat_le(a6413, a6414)
    } else {
        false
    }
}

pub fn f258(v0: u64) -> u64 {
    let a6415 = v0;
    let a6416 = 40u64;
    if nat_lt(a6415, a6416) {
        let a6417 = v0;
        let a6418 = 20u64;
        if nat_lt(a6417, a6418) {
            let a6419 = v0;
            let a6420 = 10u64;
            if nat_lt(a6419, a6420) {
                let a6421 = v0;
                let a6422 = 5u64;
                if nat_lt(a6421, a6422) {
                    let a6423 = v0;
                    let a6424 = 2u64;
                    if nat_lt(a6423, a6424) {
                        let a6425 = v0;
                        let a6426 = 1u64;
                        if nat_lt(a6425, a6426) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6427 = v0;
                        let a6428 = 3u64;
                        if nat_lt(a6427, a6428) {
                            4u64
                        } else {
                            let a6429 = v0;
                            let a6430 = 4u64;
                            if nat_lt(a6429, a6430) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6431 = v0;
                    let a6432 = 7u64;
                    if nat_lt(a6431, a6432) {
                        let a6433 = v0;
                        let a6434 = 6u64;
                        if nat_lt(a6433, a6434) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6435 = v0;
                        let a6436 = 8u64;
                        if nat_lt(a6435, a6436) {
                            128u64
                        } else {
                            let a6437 = v0;
                            let a6438 = 9u64;
                            if nat_lt(a6437, a6438) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6439 = v0;
                let a6440 = 15u64;
                if nat_lt(a6439, a6440) {
                    let a6441 = v0;
                    let a6442 = 12u64;
                    if nat_lt(a6441, a6442) {
                        let a6443 = v0;
                        let a6444 = 11u64;
                        if nat_lt(a6443, a6444) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6445 = v0;
                        let a6446 = 13u64;
                        if nat_lt(a6445, a6446) {
                            4096u64
                        } else {
                            let a6447 = v0;
                            let a6448 = 14u64;
                            if nat_lt(a6447, a6448) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6449 = v0;
                    let a6450 = 17u64;
                    if nat_lt(a6449, a6450) {
                        let a6451 = v0;
                        let a6452 = 16u64;
                        if nat_lt(a6451, a6452) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6453 = v0;
                        let a6454 = 18u64;
                        if nat_lt(a6453, a6454) {
                            131072u64
                        } else {
                            let a6455 = v0;
                            let a6456 = 19u64;
                            if nat_lt(a6455, a6456) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6457 = v0;
            let a6458 = 30u64;
            if nat_lt(a6457, a6458) {
                let a6459 = v0;
                let a6460 = 25u64;
                if nat_lt(a6459, a6460) {
                    let a6461 = v0;
                    let a6462 = 22u64;
                    if nat_lt(a6461, a6462) {
                        let a6463 = v0;
                        let a6464 = 21u64;
                        if nat_lt(a6463, a6464) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6465 = v0;
                        let a6466 = 23u64;
                        if nat_lt(a6465, a6466) {
                            4194304u64
                        } else {
                            let a6467 = v0;
                            let a6468 = 24u64;
                            if nat_lt(a6467, a6468) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6469 = v0;
                    let a6470 = 27u64;
                    if nat_lt(a6469, a6470) {
                        let a6471 = v0;
                        let a6472 = 26u64;
                        if nat_lt(a6471, a6472) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6473 = v0;
                        let a6474 = 28u64;
                        if nat_lt(a6473, a6474) {
                            134217728u64
                        } else {
                            let a6475 = v0;
                            let a6476 = 29u64;
                            if nat_lt(a6475, a6476) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6477 = v0;
                let a6478 = 35u64;
                if nat_lt(a6477, a6478) {
                    let a6479 = v0;
                    let a6480 = 32u64;
                    if nat_lt(a6479, a6480) {
                        let a6481 = v0;
                        let a6482 = 31u64;
                        if nat_lt(a6481, a6482) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6483 = v0;
                        let a6484 = 33u64;
                        if nat_lt(a6483, a6484) {
                            4294967296u64
                        } else {
                            let a6485 = v0;
                            let a6486 = 34u64;
                            if nat_lt(a6485, a6486) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6487 = v0;
                    let a6488 = 37u64;
                    if nat_lt(a6487, a6488) {
                        let a6489 = v0;
                        let a6490 = 36u64;
                        if nat_lt(a6489, a6490) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6491 = v0;
                        let a6492 = 38u64;
                        if nat_lt(a6491, a6492) {
                            137438953472u64
                        } else {
                            let a6493 = v0;
                            let a6494 = 39u64;
                            if nat_lt(a6493, a6494) {
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

pub fn f259(v0: u64, v1: u64) -> bool {
    let a6495 = v0;
    let a6496 = 1u64;
    if nat_le(a6495, a6496) {
        true
    } else {
        let a6500 = {
            let a6497 = v1;
            let a6498 = 4u64;
            let a6499 = 0u64;
            nat_rem(a6497, a6498, a6499)
        };
        let a6501 = 0u64;
        nat_eq(a6500, a6501)
    }
}

pub fn f260(v0: u64, v1: u64) -> bool {
    let a6502 = v0;
    let a6503 = 1u64;
    if nat_le(a6502, a6503) {
        true
    } else {
        let a6504 = 4u64;
        let a6505 = v1;
        nat_le(a6504, a6505)
    }
}

pub fn f261(v0: u64, v1: u64) -> R<u64> {
    let a6506 = v0;
    let a6507 = 0u64;
    if nat_eq(a6506, a6507) {
        Ok(0u64)
    } else {
        let a6512 = 4u64;
        let a6513 = {
            let a6510 = {
                let a6508 = v0;
                let a6509 = 1u64;
                nat_sub(a6508, a6509)
            };
            let a6511 = v1;
            nat_mul(a6510, a6511)?
        };
        nat_add(a6512, a6513)
    }
}

pub fn f262(v0: bool, v1: u64) -> R<u64> {
    let a6518 = {
        let a6516 = {
            let a6514 = f282(v0, v1)?;
            let a6515 = f283(v0, v1)?;
            nat_add(a6514, a6515)?
        };
        let a6517 = f284(v0);
        nat_add(a6516, a6517)?
    };
    let a6519 = v1;
    nat_add(a6518, a6519)
}

pub fn f263(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f285(v0, v1)?;
    let v3: u64 = {
        let a6524 = {
            let a6522 = {
                let a6520 = v1;
                let a6521 = 1u64;
                nat_sub(a6520, a6521)
            };
            let a6523 = v2;
            nat_mul(a6522, a6523)?
        };
        let a6525 = v0;
        nat_sub(a6524, a6525)
    };
    let a6526 = v2;
    let a6527 = v3;
    if nat_lt(a6526, a6527) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f264(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6528 = v4;
    let a6529 = 0u64;
    if nat_eq(a6528, a6529) {
        Ok(0u64)
    } else {
        let a6536 = 2u64;
        let a6537 = {
            let a6534 = {
                let a6530 = v0;
                let a6531 = v1;
                nat_mul(a6530, a6531)?
            };
            let a6535 = {
                let a6532 = v2;
                let a6533 = v3;
                nat_mul(a6532, a6533)?
            };
            nat_mul(a6534, a6535)?
        };
        nat_mul(a6536, a6537)
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6542 = v3;
    let a6543 = {
        let a6540 = {
            let a6538 = f286(v2, v4)?;
            let a6539 = f287(v0, v1, f285(v2, v4)?)?;
            nat_mul(a6538, a6539)?
        };
        let a6541 = f287(v0, v1, f288(v2, v4)?)?;
        nat_add(a6540, a6541)?
    };
    nat_mul(a6542, a6543)
}

pub fn f266(v0: u64) -> R<u64> {
    let a6546 = 13u64;
    let a6547 = {
        let a6544 = 9u64;
        let a6545 = v0;
        nat_mul(a6544, a6545)?
    };
    nat_add(a6546, a6547)
}

pub fn f267(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f289(v0, v1, v6, v7, v8) {
        let a6548 = v5;
        let a6549 = 26u64;
        nat_eq(a6548, a6549)
    } else {
        false
    } {
        let a6550 = v3;
        let a6551 = v7;
        nat_eq(a6550, a6551)
    } else {
        false
    } {
        let a6552 = v4;
        let a6553 = v8;
        nat_eq(a6552, a6553)
    } else {
        false
    } {
        let a6554 = v9;
        let a6555 = 1u64;
        nat_eq(a6554, a6555)
    } else {
        false
    } {
        let a6556 = v10;
        let a6557 = f290(v0);
        nat_eq(a6556, a6557)
    } else {
        false
    } {
        let a6558 = v11;
        let a6559 = v1;
        nat_eq(a6558, a6559)
    } else {
        false
    } {
        let a6560 = v12;
        let a6561 = v6;
        nat_eq(a6560, a6561)
    } else {
        false
    } {
        let a6562 = v13;
        let a6563 = v7;
        nat_eq(a6562, a6563)
    } else {
        false
    } {
        let a6564 = v14;
        let a6565 = v8;
        nat_eq(a6564, a6565)
    } else {
        false
    }
}

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6566 = v0;
    let a6567 = 0u64;
    if if if nat_eq(a6566, a6567) {
        true
    } else {
        let a6568 = v1;
        let a6569 = 0u64;
        nat_eq(a6568, a6569)
    } {
        true
    } else {
        let a6570 = v2;
        let a6571 = 0u64;
        nat_eq(a6570, a6571)
    } {
        true
    } else {
        let a6572 = v3;
        let a6573 = 0u64;
        nat_eq(a6572, a6573)
    }
}

pub fn f269(v0: Adt3, v1: bool) -> R<u64> {
    f273(v0.clone(), {
        let a6575 = match v0.clone() {
            Adt3::C0(h6574, _, _, _, _, _, _, _) => {
                h6574
            }
        };
        let a6576 = 1u64;
        nat_sub(a6575, a6576)
    }, {
        let a6578 = match v0.clone() {
            Adt3::C0(_, h6577, _, _, _, _, _, _) => {
                h6577
            }
        };
        let a6579 = 1u64;
        nat_sub(a6578, a6579)
    }, {
        let a6581 = match v0.clone() {
            Adt3::C0(_, _, h6580, _, _, _, _, _) => {
                h6580
            }
        };
        let a6582 = 1u64;
        nat_sub(a6581, a6582)
    }, {
        let a6584 = match v0 {
            Adt3::C0(_, _, _, h6583, _, _, _, _) => {
                h6583
            }
        };
        let a6585 = 1u64;
        nat_sub(a6584, a6585)
    }, v1)
}

pub fn f270(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6586 = v0;
    let a6587 = 0u64;
    if nat_eq(a6586, a6587) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6588 = v0;
            let a6589 = 1u64;
            nat_sub(a6588, a6589)
        };
        let v5: u64 = {
            let a6590 = v4;
            let a6591 = v1;
            let a6592 = 0u64;
            nat_quot(a6590, a6591, a6592)
        };
        let v6: u64 = {
            let a6600 = {
                let a6593 = v5;
                let a6594 = v2;
                nat_mul(a6593, a6594)?
            };
            let a6601 = {
                let a6598 = {
                    let a6595 = v4;
                    let a6596 = v1;
                    let a6597 = 0u64;
                    nat_rem(a6595, a6596, a6597)
                };
                let a6599 = v3;
                nat_mul(a6598, a6599)?
            };
            nat_add(a6600, a6601)?
        };
        let a6602 = v5;
        let a6603 = 0u64;
        if nat_eq(a6602, a6603) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6612 = {
                    let a6606 = {
                        let a6604 = v5;
                        let a6605 = 1u64;
                        nat_sub(a6604, a6605)
                    };
                    let a6607 = v2;
                    nat_mul(a6606, a6607)?
                };
                let a6613 = {
                    let a6610 = {
                        let a6608 = v1;
                        let a6609 = 1u64;
                        nat_sub(a6608, a6609)
                    };
                    let a6611 = v3;
                    nat_mul(a6610, a6611)?
                };
                nat_add(a6612, a6613)?
            };
            let a6614 = v6;
            let a6615 = v7;
            if nat_lt(a6614, a6615) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f271(v0: Adt3, v1: u64) -> bool {
    let a6616 = v1;
    let a6617 = 0u64;
    if !nat_eq(a6616, a6617) {
        true
    } else {
        let a6619 = match v0 {
            Adt3::C0(_, _, _, _, h6618, _, _, _) => {
                h6618
            }
        };
        let a6620 = 4u64;
        nat_eq(a6619, a6620)
    }
}

pub fn f272(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6621 = f291(v0, v1);
    match m6621 {
        None => {
            if v5 {
                let m6622 = f292(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6622 {
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
                let a6623 = 4u64;
                let a6624 = v28;
                nat_le(a6623, a6624)
            };
            let a6625 = v2;
            let a6626 = 0u64;
            if if if if if nat_eq(a6625, a6626) {
                let a6627 = v4;
                let a6628 = 0u64;
                nat_eq(a6627, a6628)
            } else {
                false
            } {
                let a6629 = v6;
                let a6630 = v14;
                if nat_eq(a6629, a6630) {
                    let a6631 = v7;
                    let a6632 = v15;
                    nat_eq(a6631, a6632)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6633 = v8;
                let a6634 = v16;
                if nat_eq(a6633, a6634) {
                    let a6635 = v9;
                    let a6636 = v17;
                    nat_eq(a6635, a6636)
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
                        let a6637 = v3;
                        let a6638 = 0u64;
                        nat_eq(a6637, a6638)
                    } else {
                        false
                    } {
                        f223(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6643 = {
                        let a6639 = v6;
                        let a6640 = v7;
                        nat_mul(a6639, a6640)?
                    };
                    let a6644 = {
                        let a6641 = v8;
                        let a6642 = v9;
                        nat_mul(a6641, a6642)?
                    };
                    nat_mul(a6643, a6644)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f273(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6665 = {
        let a6659 = {
            let a6653 = {
                let a6647 = v1;
                let a6648 = if v5 {
                    f141(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6645, _, _, _) => {
                            h6645
                        }
                    })
                } else {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6646, _, _, _) => {
                            h6646
                        }
                    })
                };
                nat_mul(a6647, a6648)?
            };
            let a6654 = {
                let a6651 = v2;
                let a6652 = if v5 {
                    f141(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6649, _, _) => {
                            h6649
                        }
                    })
                } else {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6650, _, _) => {
                            h6650
                        }
                    })
                };
                nat_mul(a6651, a6652)?
            };
            nat_add(a6653, a6654)?
        };
        let a6660 = {
            let a6657 = v3;
            let a6658 = if v5 {
                f141(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6655, _) => {
                        h6655
                    }
                })
            } else {
                f142(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6656, _) => {
                        h6656
                    }
                })
            };
            nat_mul(a6657, a6658)?
        };
        nat_add(a6659, a6660)?
    };
    let a6666 = {
        let a6663 = v4;
        let a6664 = if v5 {
            f141(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6661) => {
                    h6661
                }
            })
        } else {
            f142(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6662) => {
                    h6662
                }
            })
        };
        nat_mul(a6663, a6664)?
    };
    nat_add(a6665, a6666)
}

pub fn f274(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6667 = v1;
    let a6668 = 0u64;
    if nat_eq(a6667, a6668) {
        Ok(Adt9::C4({
            let a6669 = v3;
            let a6670 = 0u64;
            if nat_eq(a6669, a6670) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f91(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6671 = v3;
            let a6672 = 0u64;
            if nat_eq(a6671, a6672) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f275(v0: u64, v1: u64) -> R<u64> {
    let a6673 = v0;
    let a6674 = f94(v1)?;
    let a6675 = 0u64;
    Ok(nat_quot(a6673, a6674, a6675))
}

pub fn f276(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f275(v0, v1)?;
    let v3: u64 = {
        let a6678 = {
            let a6676 = f246(v0, v1)?;
            let a6677 = 2u64;
            nat_sub(a6676, a6677)
        };
        let a6679 = 2u64;
        let a6680 = 0u64;
        nat_quot(a6678, a6679, a6680)
    };
    let v4: u64 = {
        let a6681 = v2;
        let a6682 = v1;
        nat_mul(a6681, a6682)?
    };
    if f249(v0, v1)? {
        let a6683 = v4;
        let a6684 = v3;
        nat_add(a6683, a6684)
    } else {
        let a6696 = {
            let a6692 = v4;
            let a6693 = {
                let a6690 = {
                    let a6685 = v1;
                    let a6686 = 1u64;
                    nat_sub(a6685, a6686)
                };
                let a6691 = {
                    let a6687 = v2;
                    let a6688 = v1;
                    let a6689 = 0u64;
                    nat_rem(a6687, a6688, a6689)
                };
                nat_mul(a6690, a6691)?
            };
            nat_sub(a6692, a6693)
        };
        let a6697 = {
            let a6694 = v3;
            let a6695 = v1;
            nat_mul(a6694, a6695)?
        };
        nat_add(a6696, a6697)
    }
}

pub fn f277(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6698 = v0;
    let a6699 = 0u64;
    if if if nat_eq(a6698, a6699) {
        true
    } else {
        let a6700 = v1;
        let a6701 = 0u64;
        nat_eq(a6700, a6701)
    } {
        true
    } else {
        let a6702 = v2;
        let a6703 = 0u64;
        nat_eq(a6702, a6703)
    } {
        Ok(0u64)
    } else {
        let a6706 = {
            let a6704 = v0;
            let a6705 = v1;
            nat_mul(a6704, a6705)?
        };
        let a6707 = v2;
        nat_mul(a6706, a6707)
    }
}

pub fn f278(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f103(match v1.clone() {
        Adt20::C0(h6708, _, _, _, _, _, _, _) => {
            h6708
        }
    })?;
    let v6: u64 = {
        let a6714 = match v1.clone() {
            Adt20::C0(_, h6709, _, _, _, _, _, _) => {
                h6709
            }
        };
        let a6715 = {
            let a6712 = match v1.clone() {
                Adt20::C0(_, _, h6710, _, _, _, _, _) => {
                    h6710
                }
            };
            let a6713 = match v1.clone() {
                Adt20::C0(_, _, _, h6711, _, _, _, _) => {
                    h6711
                }
            };
            nat_mul(a6712, a6713)?
        };
        nat_mul(a6714, a6715)?
    };
    let a6717 = v0;
    let a6718 = f53(match v1.clone() {
        Adt20::C0(h6716, _, _, _, _, _, _, _) => {
            h6716
        }
    }, v6)?;
    if nat_lt(a6717, a6718) {
        f293({
            let a6719 = v0;
            let a6720 = v5;
            let a6721 = 0u64;
            nat_rem(a6719, a6720, a6721)
        }, {
            let a6722 = v0;
            let a6723 = v5;
            let a6724 = 0u64;
            nat_quot(a6722, a6723, a6724)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f279(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6725 = v6;
        let a6726 = 6u64;
        nat_sub(a6725, a6726)
    };
    let v9: Adt20 = f294(v3);
    let a6728 = v0;
    let a6729 = f210(match v1.clone() {
        Adt20::C0(h6727, _, _, _, _, _, _, _) => {
            h6727
        }
    }, v5, v8)?;
    if nat_lt(a6728, a6729) {
        let v10: u64 = f100(match v1.clone() {
            Adt20::C0(h6730, _, _, _, _, _, _, _) => {
                h6730
            }
        }, v5, v8)?;
        let a6731 = v0;
        let a6732 = v10;
        if nat_lt(a6731, a6732) {
            f295(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6733 = v0;
                let a6734 = v10;
                nat_sub(a6733, a6734)
            };
            let v12: u64 = f266(match v1.clone() {
                Adt20::C0(h6735, _, _, _, _, _, _, _) => {
                    h6735
                }
            })?;
            f296({
                let a6736 = v11;
                let a6737 = v12;
                let a6738 = 0u64;
                nat_rem(a6736, a6737, a6738)
            }, {
                let a6739 = v11;
                let a6740 = v12;
                let a6741 = 0u64;
                nat_quot(a6739, a6740, a6741)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f280(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6742 = v5;
        let a6743 = 6u64;
        nat_sub(a6742, a6743)
    };
    let v8: bool = f204(match v1.clone() {
        Adt20::C0(_, _, _, _, h6744, _, _, _) => {
            h6744
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6745, _, _, _) => {
            h6745
        }
    });
    let a6747 = v0;
    let a6748 = f203(v8, match v1.clone() {
        Adt20::C0(h6746, _, _, _, _, _, _, _) => {
            h6746
        }
    }, v4, v7)?;
    if nat_lt(a6747, a6748) {
        let v9: u64 = f100(match v1.clone() {
            Adt20::C0(h6749, _, _, _, _, _, _, _) => {
                h6749
            }
        }, v4, v7)?;
        let a6750 = v0;
        let a6751 = v9;
        if nat_lt(a6750, a6751) {
            f295(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6752 = v0;
                let a6753 = v9;
                nat_sub(a6752, a6753)
            };
            let v11: u64 = f262(v8, match v1.clone() {
                Adt20::C0(h6754, _, _, _, _, _, _, _) => {
                    h6754
                }
            })?;
            f297({
                let a6755 = v10;
                let a6756 = v11;
                let a6757 = 0u64;
                nat_rem(a6755, a6756, a6757)
            }, {
                let a6758 = v10;
                let a6759 = v11;
                let a6760 = 0u64;
                nat_quot(a6758, a6759, a6760)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f281(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6761 = v5;
        let a6762 = 6u64;
        nat_sub(a6761, a6762)
    };
    let v8: Adt20 = f294(v2);
    let v9: u64 = f264(match v1.clone() {
        Adt20::C0(h6763, _, _, _, _, _, _, _) => {
            h6763
        }
    }, match v1.clone() {
        Adt20::C0(_, h6764, _, _, _, _, _, _) => {
            h6764
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6765, _, _, _, _, _) => {
            h6765
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6766, _, _, _, _) => {
            h6766
        }
    }, v7)?;
    let a6771 = v0;
    let a6772 = f209(match v1.clone() {
        Adt20::C0(h6767, _, _, _, _, _, _, _) => {
            h6767
        }
    }, match v1.clone() {
        Adt20::C0(_, h6768, _, _, _, _, _, _) => {
            h6768
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6769, _, _, _, _, _) => {
            h6769
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6770, _, _, _, _) => {
            h6770
        }
    }, v4, v7)?;
    if nat_lt(a6771, a6772) {
        let a6773 = v0;
        let a6774 = v9;
        if nat_lt(a6773, a6774) {
            f295(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f298({
                let a6775 = v0;
                let a6776 = v9;
                nat_sub(a6775, a6776)
            }, match v1.clone() {
                Adt20::C0(h6777, _, _, _, _, _, _, _) => {
                    h6777
                }
            }, match v1.clone() {
                Adt20::C0(_, h6778, _, _, _, _, _, _) => {
                    h6778
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6779, _, _, _, _, _) => {
                    h6779
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6780, _, _, _, _) => {
                    h6780
                }
            }, v4)?;
            f299(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f282(v0: bool, v1: u64) -> R<u64> {
    let a6783 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6784 = {
        let a6781 = f300(v0);
        let a6782 = v1;
        nat_mul(a6781, a6782)?
    };
    nat_add(a6783, a6784)
}

pub fn f283(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6792 = {
            let a6785 = 13u64;
            let a6786 = f301(v1);
            nat_mul(a6785, a6786)?
        };
        let a6793 = {
            let a6790 = 4u64;
            let a6791 = {
                let a6787 = v1;
                let a6788 = 4u64;
                let a6789 = 0u64;
                nat_rem(a6787, a6788, a6789)
            };
            nat_mul(a6790, a6791)?
        };
        nat_add(a6792, a6793)
    } else {
        let a6794 = 3u64;
        let a6795 = v1;
        nat_mul(a6794, a6795)
    }
}

pub fn f284(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f285(v0: u64, v1: u64) -> R<u64> {
    let a6800 = {
        let a6798 = {
            let a6796 = v0;
            let a6797 = v1;
            nat_add(a6796, a6797)?
        };
        let a6799 = 1u64;
        nat_sub(a6798, a6799)
    };
    let a6801 = v1;
    let a6802 = 0u64;
    Ok(nat_quot(a6800, a6801, a6802))
}

pub fn f286(v0: u64, v1: u64) -> R<u64> {
    let a6803 = v0;
    let a6804 = f285(v0, v1)?;
    let a6805 = 0u64;
    Ok(nat_quot(a6803, a6804, a6805))
}

pub fn f287(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6806 = v2;
    let a6807 = 0u64;
    if nat_eq(a6806, a6807) {
        Ok(0u64)
    } else {
        let a6812 = 11u64;
        let a6813 = {
            let a6810 = {
                let a6808 = v1;
                let a6809 = v2;
                nat_mul(a6808, a6809)?
            };
            let a6811 = f302(v0)?;
            nat_mul(a6810, a6811)?
        };
        nat_add(a6812, a6813)
    }
}

pub fn f288(v0: u64, v1: u64) -> R<u64> {
    let a6814 = v0;
    let a6815 = f285(v0, v1)?;
    let a6816 = 0u64;
    Ok(nat_rem(a6814, a6815, a6816))
}

pub fn f289(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f268(v1, v2, v3, v4) {
        f303(v0)
    } else {
        f220(v0)
    }
}

pub fn f290(v0: u64) -> u64 {
    let a6817 = v0;
    let a6818 = 26u64;
    if nat_eq(a6817, a6818) {
        26u64
    } else {
        0u64
    }
}

pub fn f291(v0: u64, v1: u64) -> Option<u64> {
    let a6819 = v0;
    let a6820 = 2u64;
    if nat_eq(a6819, a6820) {
        Some(0u64)
    } else {
        let a6821 = v0;
        let a6822 = 5u64;
        if nat_eq(a6821, a6822) {
            Some(1u64)
        } else {
            let a6823 = v0;
            let a6824 = 6u64;
            if nat_eq(a6823, a6824) {
                Some(2u64)
            } else {
                let a6825 = v0;
                let a6826 = 7u64;
                if nat_eq(a6825, a6826) {
                    Some(3u64)
                } else {
                    let a6827 = v0;
                    let a6828 = 9u64;
                    if nat_eq(a6827, a6828) {
                        Some(4u64)
                    } else {
                        let a6829 = v0;
                        let a6830 = 8u64;
                        if nat_eq(a6829, a6830) {
                            Some(5u64)
                        } else {
                            let a6831 = v0;
                            let a6832 = 89u64;
                            if nat_eq(a6831, a6832) {
                                let a6833 = v1;
                                let a6834 = 2u64;
                                if nat_eq(a6833, a6834) {
                                    Some(6u64)
                                } else {
                                    let a6835 = v1;
                                    let a6836 = 0u64;
                                    if nat_eq(a6835, a6836) {
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

pub fn f292(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6837 = v0;
    let a6838 = 30u64;
    if nat_eq(a6837, a6838) {
        let a6839 = v2;
        let a6840 = 0u64;
        if if if nat_eq(a6839, a6840) {
            true
        } else {
            let a6841 = v2;
            let a6842 = 1u64;
            nat_eq(a6841, a6842)
        } {
            let a6843 = v1;
            let a6844 = 0u64;
            if nat_eq(a6843, a6844) {
                true
            } else {
                f304(v1, v4)
            }
        } else {
            false
        } {
            f305(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f306(v1) {
            f306(v2)
        } else {
            false
        } {
            let a6845 = v1;
            let a6846 = 0u64;
            if if nat_eq(a6845, a6846) {
                true
            } else {
                let a6847 = v2;
                let a6848 = 0u64;
                nat_eq(a6847, a6848)
            } {
                true
            } else {
                let a6849 = v1;
                let a6850 = v2;
                nat_eq(a6849, a6850)
            }
        } else {
            false
        } {
            true
        } else {
            let a6851 = v1;
            let a6852 = 43u64;
            if nat_eq(a6851, a6852) {
                f306(v2)
            } else {
                false
            }
        } {
            f305(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f293(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6853, _, _, _, _, _, _, _) => {
            h6853
        }
    };
    let v7: u64 = {
        let a6856 = 1u64;
        let a6857 = {
            let a6854 = 3u64;
            let a6855 = v6;
            nat_mul(a6854, a6855)?
        };
        nat_add(a6856, a6857)?
    };
    let a6858 = v0;
    let a6859 = 0u64;
    if nat_eq(a6858, a6859) {
        Ok(f307(0u64))
    } else {
        let a6860 = v0;
        let a6861 = v7;
        if nat_lt(a6860, a6861) {
            let v8: u64 = {
                let a6862 = v0;
                let a6863 = 1u64;
                nat_sub(a6862, a6863)
            };
            let v9: u64 = {
                let a6869 = {
                    let a6864 = v1;
                    let a6865 = v6;
                    nat_mul(a6864, a6865)?
                };
                let a6870 = {
                    let a6866 = v8;
                    let a6867 = 3u64;
                    let a6868 = 0u64;
                    nat_quot(a6866, a6867, a6868)
                };
                nat_add(a6869, a6870)?
            };
            let v10: u64 = f308(v9, v2)?;
            let v11: u64 = {
                let a6871 = v8;
                let a6872 = 3u64;
                let a6873 = 0u64;
                nat_rem(a6871, a6872, a6873)
            };
            let a6874 = v11;
            let a6875 = 0u64;
            if nat_eq(a6874, a6875) {
                Ok(f309(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6876 = v11;
                let a6877 = 1u64;
                if nat_eq(a6876, a6877) {
                    Ok(f309(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f309(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6878 = v0;
            let a6879 = v7;
            if nat_eq(a6878, a6879) {
                Ok(f309(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6882 = v0;
                let a6883 = {
                    let a6880 = v7;
                    let a6881 = 1u64;
                    nat_add(a6880, a6881)?
                };
                if nat_eq(a6882, a6883) {
                    Ok(f309(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6886 = v0;
                    let a6887 = {
                        let a6884 = v7;
                        let a6885 = 2u64;
                        nat_add(a6884, a6885)?
                    };
                    if nat_eq(a6886, a6887) {
                        Ok(f309(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6890 = v0;
                        let a6891 = {
                            let a6888 = v7;
                            let a6889 = 3u64;
                            nat_add(a6888, a6889)?
                        };
                        if nat_eq(a6890, a6891) {
                            let a6892 = v5;
                            let a6893 = 1u64;
                            if if nat_eq(a6892, a6893) {
                                true
                            } else {
                                let a6894 = v5;
                                let a6895 = 2u64;
                                nat_eq(a6894, a6895)
                            } {
                                Ok(f309(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f309(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6902 = {
                                    let a6896 = v1;
                                    let a6897 = v6;
                                    nat_mul(a6896, a6897)?
                                };
                                let a6903 = {
                                    let a6900 = v0;
                                    let a6901 = {
                                        let a6898 = v7;
                                        let a6899 = 4u64;
                                        nat_add(a6898, a6899)?
                                    };
                                    nat_sub(a6900, a6901)
                                };
                                nat_add(a6902, a6903)?
                            };
                            Ok(f309(10u64, 0u64, f308(v12, v2)?, 1u64, 2u64, 3u64, f308(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f294(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6904, _, _, _, _, _, _, _) => {
            h6904
        }
    }, match v0.clone() {
        Adt20::C0(_, h6905, _, _, _, _, _, _) => {
            h6905
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6906, _, _, _, _, _) => {
            h6906
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6907, _, _, _, _) => {
            h6907
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6908, _, _) => {
            h6908
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6909, _) => {
            h6909
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6910) => {
            h6910
        }
    })
}

pub fn f295(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6917 = v3;
        let a6918 = {
            let a6914 = f308({
                let a6911 = v0;
                let a6912 = 2u64;
                let a6913 = 0u64;
                nat_quot(a6911, a6912, a6913)
            }, v1)?;
            let a6915 = v2;
            let a6916 = 0u64;
            nat_quot(a6914, a6915, a6916)
        };
        nat_add(a6917, a6918)?
    };
    let a6922 = {
        let a6919 = v0;
        let a6920 = 2u64;
        let a6921 = 0u64;
        nat_rem(a6919, a6920, a6921)
    };
    let a6923 = 0u64;
    if nat_eq(a6922, a6923) {
        Ok(f309(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6924 = v4;
        let a6925 = 0u64;
        if nat_eq(a6924, a6925) {
            Ok(f309(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f296(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f310(match v2.clone() {
        Adt20::C0(h6926, _, _, _, _, _, _, _) => {
            h6926
        }
    })?;
    let v7: u64 = f311(match v2.clone() {
        Adt20::C0(h6927, _, _, _, _, _, _, _) => {
            h6927
        }
    })?;
    let a6928 = v0;
    let a6929 = 2u64;
    if nat_lt(a6928, a6929) {
        Ok(f307(v0))
    } else {
        let a6930 = v0;
        let a6931 = v6;
        if nat_lt(a6930, a6931) {
            f312({
                let a6932 = v0;
                let a6933 = 2u64;
                nat_sub(a6932, a6933)
            }, v1, v2, v3)
        } else {
            let a6934 = v0;
            let a6935 = v7;
            if nat_lt(a6934, a6935) {
                Ok(f313({
                    let a6936 = v0;
                    let a6937 = v6;
                    nat_sub(a6936, a6937)
                }, match v2 {
                    Adt20::C0(h6938, _, _, _, _, _, _, _) => {
                        h6938
                    }
                }, v5))
            } else {
                let a6940 = v0;
                let a6941 = f266(match v2.clone() {
                    Adt20::C0(h6939, _, _, _, _, _, _, _) => {
                        h6939
                    }
                })?;
                if nat_lt(a6940, a6941) {
                    f314({
                        let a6942 = v0;
                        let a6943 = v7;
                        nat_sub(a6942, a6943)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f297(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f282(v5, match v2.clone() {
        Adt20::C0(h6944, _, _, _, _, _, _, _) => {
            h6944
        }
    })?;
    let v7: u64 = {
        let a6946 = v6;
        let a6947 = f283(v5, match v2.clone() {
            Adt20::C0(h6945, _, _, _, _, _, _, _) => {
                h6945
            }
        })?;
        nat_add(a6946, a6947)?
    };
    let v8: u64 = {
        let a6948 = v7;
        let a6949 = f284(v5);
        nat_add(a6948, a6949)?
    };
    let a6950 = v0;
    let a6951 = v6;
    if nat_lt(a6950, a6951) {
        f315(v0, v1, v2, v5)
    } else {
        let a6952 = v0;
        let a6953 = v7;
        if nat_lt(a6952, a6953) {
            f316({
                let a6954 = v0;
                let a6955 = v6;
                nat_sub(a6954, a6955)
            }, v1, v2, v3, v5)
        } else {
            let a6956 = v0;
            let a6957 = v8;
            if nat_lt(a6956, a6957) {
                Ok(f317({
                    let a6958 = v0;
                    let a6959 = v7;
                    nat_sub(a6958, a6959)
                }, match v2 {
                    Adt20::C0(h6960, _, _, _, _, _, _, _) => {
                        h6960
                    }
                }, v4, v5))
            } else {
                let a6964 = v0;
                let a6965 = {
                    let a6962 = v8;
                    let a6963 = match v2.clone() {
                        Adt20::C0(h6961, _, _, _, _, _, _, _) => {
                            h6961
                        }
                    };
                    nat_add(a6962, a6963)?
                };
                if nat_lt(a6964, a6965) {
                    let v9: u64 = f308({
                        let a6971 = {
                            let a6967 = v1;
                            let a6968 = match v2 {
                                Adt20::C0(h6966, _, _, _, _, _, _, _) => {
                                    h6966
                                }
                            };
                            nat_mul(a6967, a6968)?
                        };
                        let a6972 = {
                            let a6969 = v0;
                            let a6970 = v8;
                            nat_sub(a6969, a6970)
                        };
                        nat_add(a6971, a6972)?
                    }, v3)?;
                    Ok(f309(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f298(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f285(v3, v5)?;
    let v7: u64 = f287(v1, v2, v6)?;
    let v8: u64 = {
        let a6975 = {
            let a6973 = f286(v3, v5)?;
            let a6974 = v4;
            nat_mul(a6973, a6974)?
        };
        let a6976 = v7;
        nat_mul(a6975, a6976)?
    };
    let a6977 = v0;
    let a6978 = v8;
    if nat_lt(a6977, a6978) {
        let v9: u64 = {
            let a6979 = v0;
            let a6980 = v7;
            let a6981 = 0u64;
            nat_quot(a6979, a6980, a6981)
        };
        Ok(Adt21::C0({
            let a6985 = {
                let a6982 = v9;
                let a6983 = v4;
                let a6984 = 0u64;
                nat_quot(a6982, a6983, a6984)
            };
            let a6986 = v6;
            nat_mul(a6985, a6986)?
        }, v6, {
            let a6987 = v9;
            let a6988 = v4;
            let a6989 = 0u64;
            nat_rem(a6987, a6988, a6989)
        }, {
            let a6990 = v0;
            let a6991 = v7;
            let a6992 = 0u64;
            nat_rem(a6990, a6991, a6992)
        }))
    } else {
        let v10: u64 = {
            let a6993 = v0;
            let a6994 = v8;
            nat_sub(a6993, a6994)
        };
        let v11: u64 = f287(v1, v2, f288(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6995 = f286(v3, v5)?;
            let a6996 = v6;
            nat_mul(a6995, a6996)?
        }, f288(v3, v5)?, {
            let a6997 = v10;
            let a6998 = v11;
            let a6999 = 0u64;
            nat_quot(a6997, a6998, a6999)
        }, {
            let a7000 = v10;
            let a7001 = v11;
            let a7002 = 0u64;
            nat_rem(a7000, a7001, a7002)
        }))
    }
}

pub fn f299(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7003) => {
            h7003
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7004, _, _, _, _, _, _, _) => {
            h7004
        }
    };
    let v6: u64 = {
        let a7007 = match v0.clone() {
            Adt21::C0(_, h7005, _, _) => {
                h7005
            }
        };
        let a7008 = match v1.clone() {
            Adt20::C0(_, h7006, _, _, _, _, _, _) => {
                h7006
            }
        };
        nat_mul(a7007, a7008)?
    };
    let v7: u64 = {
        let a7009 = v6;
        let a7010 = v5;
        nat_mul(a7009, a7010)?
    };
    let v8: u64 = {
        let a7013 = {
            let a7011 = 2u64;
            let a7012 = v5;
            nat_mul(a7011, a7012)?
        };
        let a7014 = 2u64;
        nat_add(a7013, a7014)?
    };
    let v9: u64 = {
        let a7017 = {
            let a7015 = 4u64;
            let a7016 = v5;
            nat_mul(a7015, a7016)?
        };
        let a7018 = 2u64;
        nat_add(a7017, a7018)?
    };
    let v10: u64 = {
        let a7021 = 1u64;
        let a7022 = {
            let a7019 = v6;
            let a7020 = v8;
            nat_mul(a7019, a7020)?
        };
        nat_add(a7021, a7022)?
    };
    let v11: u64 = {
        let a7023 = v10;
        let a7024 = 4u64;
        nat_add(a7023, a7024)?
    };
    let v12: u64 = {
        let a7027 = v11;
        let a7028 = {
            let a7025 = v6;
            let a7026 = v9;
            nat_mul(a7025, a7026)?
        };
        nat_add(a7027, a7028)?
    };
    let v13: u64 = {
        let a7029 = v12;
        let a7030 = 6u64;
        nat_add(a7029, a7030)?
    };
    let a7031 = v4;
    let a7032 = 0u64;
    if nat_eq(a7031, a7032) {
        Ok(f307(0u64))
    } else {
        let a7033 = v4;
        let a7034 = v10;
        if nat_lt(a7033, a7034) {
            let v14: u64 = {
                let a7035 = v4;
                let a7036 = 1u64;
                nat_sub(a7035, a7036)
            };
            f318(false, {
                let a7037 = v14;
                let a7038 = v8;
                let a7039 = 0u64;
                nat_rem(a7037, a7038, a7039)
            }, {
                let a7040 = v14;
                let a7041 = v8;
                let a7042 = 0u64;
                nat_quot(a7040, a7041, a7042)
            }, v0, v1, v2)
        } else {
            let a7043 = v4;
            let a7044 = v11;
            if nat_lt(a7043, a7044) {
                Ok(f319({
                    let a7045 = v4;
                    let a7046 = v10;
                    nat_sub(a7045, a7046)
                }, v7))
            } else {
                let a7047 = v4;
                let a7048 = v12;
                if nat_lt(a7047, a7048) {
                    let v15: u64 = {
                        let a7049 = v4;
                        let a7050 = v11;
                        nat_sub(a7049, a7050)
                    };
                    f318(true, {
                        let a7051 = v15;
                        let a7052 = v9;
                        let a7053 = 0u64;
                        nat_rem(a7051, a7052, a7053)
                    }, {
                        let a7054 = v15;
                        let a7055 = v9;
                        let a7056 = 0u64;
                        nat_quot(a7054, a7055, a7056)
                    }, v0, v1, v2)
                } else {
                    let a7057 = v4;
                    let a7058 = v13;
                    if nat_lt(a7057, a7058) {
                        Ok(f320({
                            let a7059 = v4;
                            let a7060 = v12;
                            nat_sub(a7059, a7060)
                        }, v7, v3))
                    } else {
                        let a7063 = v4;
                        let a7064 = {
                            let a7061 = v13;
                            let a7062 = v7;
                            nat_add(a7061, a7062)?
                        };
                        if nat_lt(a7063, a7064) {
                            let v16: u64 = {
                                let a7065 = v4;
                                let a7066 = v13;
                                nat_sub(a7065, a7066)
                            };
                            let v17: u64 = {
                                let a7067 = v16;
                                let a7068 = v5;
                                let a7069 = 0u64;
                                nat_quot(a7067, a7068, a7069)
                            };
                            let v18: u64 = {
                                let a7075 = {
                                    let a7070 = f321(v17, v0, v1)?;
                                    let a7071 = v5;
                                    nat_mul(a7070, a7071)?
                                };
                                let a7076 = {
                                    let a7072 = v16;
                                    let a7073 = v5;
                                    let a7074 = 0u64;
                                    nat_rem(a7072, a7073, a7074)
                                };
                                nat_add(a7075, a7076)?
                            };
                            let v19: u64 = f308(v18, v2)?;
                            Ok(f309(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f300(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f301(v0: u64) -> u64 {
    let a7077 = v0;
    let a7078 = 4u64;
    let a7079 = 0u64;
    nat_quot(a7077, a7078, a7079)
}

pub fn f302(v0: u64) -> R<u64> {
    let a7082 = {
        let a7080 = 7u64;
        let a7081 = v0;
        nat_mul(a7080, a7081)?
    };
    let a7083 = 4u64;
    nat_add(a7082, a7083)
}

pub fn f303(v0: u64) -> bool {
    let m7084 = f213(v0);
    match m7084 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f304(v0: u64, v1: u64) -> bool {
    let a7085 = v0;
    let a7086 = 1u64;
    if nat_eq(a7085, a7086) {
        true
    } else {
        let v2: u64 = f322(v0);
        let a7087 = 0u64;
        let a7088 = v2;
        if nat_lt(a7087, a7088) {
            let a7092 = {
                let a7089 = v1;
                let a7090 = v2;
                let a7091 = 1u64;
                nat_rem(a7089, a7090, a7091)
            };
            let a7093 = 0u64;
            nat_eq(a7092, a7093)
        } else {
            false
        }
    }
}

pub fn f305(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7094 = v0;
    let a7095 = 28u64;
    if if if if nat_eq(a7094, a7095) {
        let a7096 = v1;
        let a7097 = 0u64;
        nat_eq(a7096, a7097)
    } else {
        false
    } {
        let a7098 = v2;
        let a7099 = 0u64;
        nat_eq(a7098, a7099)
    } else {
        false
    } {
        let a7100 = v3;
        let a7101 = 0u64;
        nat_eq(a7100, a7101)
    } else {
        false
    } {
        let a7102 = v4;
        let a7103 = v8;
        if if if if if if if if nat_eq(a7102, a7103) {
            let a7104 = v5;
            let a7105 = v12;
            nat_eq(a7104, a7105)
        } else {
            false
        } {
            let a7106 = v9;
            let a7107 = v13;
            nat_eq(a7106, a7107)
        } else {
            false
        } {
            let a7108 = v10;
            let a7109 = v14;
            nat_eq(a7108, a7109)
        } else {
            false
        } {
            let a7110 = v11;
            let a7111 = v15;
            nat_eq(a7110, a7111)
        } else {
            false
        } {
            let a7112 = 0u64;
            let a7113 = v6;
            nat_lt(a7112, a7113)
        } else {
            false
        } {
            let a7114 = 0u64;
            let a7115 = v7;
            nat_lt(a7114, a7115)
        } else {
            false
        } {
            let a7119 = {
                let a7116 = v10;
                let a7117 = v6;
                let a7118 = 1u64;
                nat_rem(a7116, a7117, a7118)
            };
            let a7120 = 0u64;
            if nat_eq(a7119, a7120) {
                let a7124 = {
                    let a7121 = v11;
                    let a7122 = v7;
                    let a7123 = 1u64;
                    nat_rem(a7121, a7122, a7123)
                };
                let a7125 = 0u64;
                nat_eq(a7124, a7125)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7126 = v10;
                let a7127 = v6;
                let a7128 = 0u64;
                nat_quot(a7126, a7127, a7128)
            }, {
                let a7129 = v11;
                let a7130 = v7;
                let a7131 = 0u64;
                nat_quot(a7129, a7130, a7131)
            }, {
                let a7132 = v5;
                let a7133 = v4;
                nat_mul(a7132, a7133)?
            }, {
                let a7134 = v4;
                let a7135 = v9;
                nat_mul(a7134, a7135)?
            }, {
                let a7136 = v5;
                let a7137 = v9;
                nat_mul(a7136, a7137)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f306(v0: u64) -> bool {
    let a7138 = v0;
    let a7139 = 0u64;
    if if nat_eq(a7138, a7139) {
        true
    } else {
        let a7140 = v0;
        let a7141 = 1u64;
        nat_eq(a7140, a7141)
    } {
        true
    } else {
        let a7142 = v0;
        let a7143 = 30u64;
        nat_eq(a7142, a7143)
    }
}

pub fn f307(v0: u64) -> Adt17 {
    f309(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f308(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7145 = v0;
        let a7146 = match v1.clone() {
            Adt20::C0(h7144, _, _, _, _, _, _, _) => {
                h7144
            }
        };
        let a7147 = 0u64;
        nat_quot(a7145, a7146, a7147)
    };
    let v3: u64 = {
        let a7149 = v2;
        let a7150 = match v1.clone() {
            Adt20::C0(_, h7148, _, _, _, _, _, _) => {
                h7148
            }
        };
        let a7151 = 0u64;
        nat_quot(a7149, a7150, a7151)
    };
    f140({
        let a7153 = v0;
        let a7154 = match v1.clone() {
            Adt20::C0(h7152, _, _, _, _, _, _, _) => {
                h7152
            }
        };
        let a7155 = 0u64;
        nat_rem(a7153, a7154, a7155)
    }, {
        let a7157 = v2;
        let a7158 = match v1.clone() {
            Adt20::C0(_, h7156, _, _, _, _, _, _) => {
                h7156
            }
        };
        let a7159 = 0u64;
        nat_rem(a7157, a7158, a7159)
    }, {
        let a7161 = v3;
        let a7162 = match v1.clone() {
            Adt20::C0(_, _, h7160, _, _, _, _, _) => {
                h7160
            }
        };
        let a7163 = 0u64;
        nat_rem(a7161, a7162, a7163)
    }, {
        let a7165 = v3;
        let a7166 = match v1.clone() {
            Adt20::C0(_, _, h7164, _, _, _, _, _) => {
                h7164
            }
        };
        let a7167 = 0u64;
        nat_quot(a7165, a7166, a7167)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7168, _, _, _) => {
            h7168
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7169, _, _) => {
            h7169
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7170, _) => {
            h7170
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7171) => {
            h7171
        }
    })
}

pub fn f309(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7172 = v0;
    let a7173 = 4u64;
    if if if nat_le(a7172, a7173) {
        true
    } else {
        let a7174 = 9u64;
        let a7175 = v0;
        if nat_le(a7174, a7175) {
            let a7176 = v0;
            let a7177 = 12u64;
            nat_le(a7176, a7177)
        } else {
            false
        }
    } {
        true
    } else {
        let a7178 = v0;
        let a7179 = 16u64;
        if if nat_eq(a7178, a7179) {
            true
        } else {
            let a7180 = v0;
            let a7181 = 19u64;
            nat_eq(a7180, a7181)
        } {
            true
        } else {
            let a7182 = v0;
            let a7183 = 21u64;
            if nat_eq(a7182, a7183) {
                true
            } else {
                let a7184 = v0;
                let a7185 = 22u64;
                nat_eq(a7184, a7185)
            }
        }
    } {
        Adt17::C1({
            let a7186 = v0;
            let a7187 = 4u64;
            if if nat_lt(a7186, a7187) {
                true
            } else {
                let a7188 = 21u64;
                let a7189 = v0;
                nat_le(a7188, a7189)
            } {
                0u64
            } else {
                let a7190 = v0;
                let a7191 = 16u64;
                if if nat_eq(a7190, a7191) {
                    true
                } else {
                    let a7192 = v0;
                    let a7193 = 19u64;
                    nat_eq(a7192, a7193)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7194 = v0;
            let a7195 = 4u64;
            if nat_lt(a7194, a7195) {
                v0
            } else {
                let a7196 = v0;
                let a7197 = 4u64;
                if if nat_eq(a7196, a7197) {
                    true
                } else {
                    let a7198 = v0;
                    let a7199 = 16u64;
                    nat_eq(a7198, a7199)
                } {
                    0u64
                } else {
                    let a7200 = v0;
                    let a7201 = 19u64;
                    if nat_eq(a7200, a7201) {
                        3u64
                    } else {
                        let a7202 = 21u64;
                        let a7203 = v0;
                        if nat_le(a7202, a7203) {
                            let a7204 = v0;
                            let a7205 = 17u64;
                            nat_sub(a7204, a7205)
                        } else {
                            let a7206 = v0;
                            let a7207 = 8u64;
                            nat_sub(a7206, a7207)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f310(v0: u64) -> R<u64> {
    let a7210 = 2u64;
    let a7211 = {
        let a7208 = 6u64;
        let a7209 = v0;
        nat_mul(a7208, a7209)?
    };
    nat_add(a7210, a7211)
}

pub fn f311(v0: u64) -> R<u64> {
    let a7212 = f310(v0)?;
    let a7213 = 11u64;
    nat_add(a7212, a7213)
}

pub fn f312(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7214 = v0;
        let a7215 = 6u64;
        let a7216 = 0u64;
        nat_rem(a7214, a7215, a7216)
    };
    let v5: u64 = {
        let a7223 = {
            let a7218 = v1;
            let a7219 = match v2.clone() {
                Adt20::C0(h7217, _, _, _, _, _, _, _) => {
                    h7217
                }
            };
            nat_mul(a7218, a7219)?
        };
        let a7224 = {
            let a7220 = v0;
            let a7221 = 6u64;
            let a7222 = 0u64;
            nat_quot(a7220, a7221, a7222)
        };
        nat_add(a7223, a7224)?
    };
    let v6: u64 = f308(v5, v3)?;
    let v7: u64 = f308(v5, v2)?;
    let a7225 = v4;
    let a7226 = 0u64;
    if nat_eq(a7225, a7226) {
        Ok(f309(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7227 = v4;
        let a7228 = 1u64;
        if nat_eq(a7227, a7228) {
            Ok(f309(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7229 = v4;
            let a7230 = 2u64;
            if nat_eq(a7229, a7230) {
                Ok(f309(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7231 = v4;
                let a7232 = 3u64;
                if nat_eq(a7231, a7232) {
                    Ok(f309(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7233 = v4;
                    let a7234 = 4u64;
                    if nat_eq(a7233, a7234) {
                        Ok(f309(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f309(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f313(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7235 = v0;
    let a7236 = 0u64;
    if nat_eq(a7235, a7236) {
        f309(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7237 = v0;
        let a7238 = 1u64;
        if nat_eq(a7237, a7238) {
            f309(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7239 = v0;
            let a7240 = 2u64;
            if nat_eq(a7239, a7240) {
                f309(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7241 = v0;
                let a7242 = 3u64;
                if nat_eq(a7241, a7242) {
                    f309(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7243 = v0;
                    let a7244 = 4u64;
                    if nat_eq(a7243, a7244) {
                        f309(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7245 = v0;
                        let a7246 = 5u64;
                        if nat_eq(a7245, a7246) {
                            f309(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7247 = v0;
                            let a7248 = 6u64;
                            if nat_eq(a7247, a7248) {
                                f309(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7249 = v0;
                                let a7250 = 7u64;
                                if nat_eq(a7249, a7250) {
                                    f309(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7251 = v0;
                                    let a7252 = 8u64;
                                    if nat_eq(a7251, a7252) {
                                        f309(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7253 = v0;
                                        let a7254 = 9u64;
                                        if nat_eq(a7253, a7254) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7255 = v0;
                                            let a7256 = 10u64;
                                            if nat_eq(a7255, a7256) {
                                                f309(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f314(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7257 = v0;
        let a7258 = 3u64;
        let a7259 = 0u64;
        nat_rem(a7257, a7258, a7259)
    };
    let v6: u64 = {
        let a7266 = {
            let a7261 = v1;
            let a7262 = match v2.clone() {
                Adt20::C0(h7260, _, _, _, _, _, _, _) => {
                    h7260
                }
            };
            nat_mul(a7261, a7262)?
        };
        let a7267 = {
            let a7263 = v0;
            let a7264 = 3u64;
            let a7265 = 0u64;
            nat_quot(a7263, a7264, a7265)
        };
        nat_add(a7266, a7267)?
    };
    let v7: u64 = f308(v6, v3)?;
    let v8: u64 = f308(v6, v2)?;
    let v9: u64 = f308(v6, v4)?;
    let a7268 = v5;
    let a7269 = 0u64;
    if nat_eq(a7268, a7269) {
        Ok(f309(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7270 = v5;
        let a7271 = 1u64;
        if nat_eq(a7270, a7271) {
            Ok(f309(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f309(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f315(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7272, _, _, _, _, _, _, _) => {
            h7272
        }
    };
    let v5: u64 = {
        let a7275 = 1u64;
        let a7276 = {
            let a7273 = v4;
            let a7274 = f300(v3);
            nat_mul(a7273, a7274)?
        };
        nat_add(a7275, a7276)?
    };
    let a7277 = v0;
    let a7278 = 0u64;
    if nat_eq(a7277, a7278) {
        Ok(f307(0u64))
    } else {
        let a7279 = v0;
        let a7280 = v5;
        if nat_lt(a7279, a7280) {
            let v6: u64 = {
                let a7281 = v0;
                let a7282 = 1u64;
                nat_sub(a7281, a7282)
            };
            let v7: u64 = f308({
                let a7288 = {
                    let a7283 = v1;
                    let a7284 = v4;
                    nat_mul(a7283, a7284)?
                };
                let a7289 = {
                    let a7285 = v6;
                    let a7286 = f300(v3);
                    let a7287 = 0u64;
                    nat_quot(a7285, a7286, a7287)
                };
                nat_add(a7288, a7289)?
            }, v2)?;
            if v3 {
                let a7293 = {
                    let a7290 = v6;
                    let a7291 = 2u64;
                    let a7292 = 0u64;
                    nat_rem(a7290, a7291, a7292)
                };
                let a7294 = 0u64;
                if nat_eq(a7293, a7294) {
                    Ok(f309(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f309(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f309(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7295 = v0;
                let a7296 = v5;
                nat_sub(a7295, a7296)
            };
            let a7297 = v8;
            let a7298 = 0u64;
            if nat_eq(a7297, a7298) {
                Ok(f309(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7299 = v8;
                    let a7300 = 1u64;
                    nat_eq(a7299, a7300)
                } else {
                    false
                } {
                    Ok(f309(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7301 = v8;
                    let a7302 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7301, a7302) {
                        Ok(f309(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7303 = v8;
                        let a7304 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7303, a7304) {
                            Ok(f307(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f316(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7306 = 13u64;
        let a7307 = f301(match v2.clone() {
            Adt20::C0(h7305, _, _, _, _, _, _, _) => {
                h7305
            }
        });
        nat_mul(a7306, a7307)?
    } else {
        0u64
    };
    let a7308 = v0;
    let a7309 = v5;
    if nat_lt(a7308, a7309) {
        f323({
            let a7310 = v0;
            let a7311 = 13u64;
            let a7312 = 0u64;
            nat_rem(a7310, a7311, a7312)
        }, v1, {
            let a7313 = v0;
            let a7314 = 13u64;
            let a7315 = 0u64;
            nat_quot(a7313, a7314, a7315)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7316 = v0;
            let a7317 = v5;
            nat_sub(a7316, a7317)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f324({
            let a7318 = v6;
            let a7319 = v7;
            let a7320 = 0u64;
            nat_rem(a7318, a7319, a7320)
        }, {
            let a7330 = {
                let a7325 = {
                    let a7322 = v1;
                    let a7323 = match v2.clone() {
                        Adt20::C0(h7321, _, _, _, _, _, _, _) => {
                            h7321
                        }
                    };
                    nat_mul(a7322, a7323)?
                };
                let a7326 = if v4 {
                    f325(match v2.clone() {
                        Adt20::C0(h7324, _, _, _, _, _, _, _) => {
                            h7324
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7325, a7326)?
            };
            let a7331 = {
                let a7327 = v6;
                let a7328 = v7;
                let a7329 = 0u64;
                nat_quot(a7327, a7328, a7329)
            };
            nat_add(a7330, a7331)?
        }, v2, v3, v4)
    }
}

pub fn f317(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7332 = v0;
    let a7333 = 0u64;
    if nat_eq(a7332, a7333) {
        f309(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7334 = v0;
        let a7335 = 1u64;
        if nat_eq(a7334, a7335) {
            f309(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7336 = v0;
                let a7337 = 2u64;
                nat_eq(a7336, a7337)
            } else {
                false
            } {
                f309(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7338 = v0;
                let a7339 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7338, a7339) {
                    f309(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7340 = v0;
                    let a7341 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7340, a7341) {
                        f309(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7342 = v0;
                        let a7343 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7342, a7343) {
                            f309(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f318(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7347 = 1u64;
        let a7348 = {
            let a7345 = match v4.clone() {
                Adt20::C0(h7344, _, _, _, _, _, _, _) => {
                    h7344
                }
            };
            let a7346 = v6;
            nat_mul(a7345, a7346)?
        };
        nat_add(a7347, a7348)?
    };
    let a7349 = v1;
    let a7350 = 0u64;
    if nat_eq(a7349, a7350) {
        Ok(f307(1u64))
    } else {
        let a7351 = v1;
        let a7352 = v7;
        if nat_lt(a7351, a7352) {
            let v8: u64 = {
                let a7353 = v1;
                let a7354 = 1u64;
                nat_sub(a7353, a7354)
            };
            let v9: u64 = {
                let a7361 = {
                    let a7356 = f321(v2, v3, v4.clone())?;
                    let a7357 = match v4.clone() {
                        Adt20::C0(h7355, _, _, _, _, _, _, _) => {
                            h7355
                        }
                    };
                    nat_mul(a7356, a7357)?
                };
                let a7362 = {
                    let a7358 = v8;
                    let a7359 = v6;
                    let a7360 = 0u64;
                    nat_quot(a7358, a7359, a7360)
                };
                nat_add(a7361, a7362)?
            };
            let v10: u64 = f308(v9, v4)?;
            let v11: u64 = f308(v9, v5)?;
            let v12: u64 = {
                let a7363 = v8;
                let a7364 = v6;
                let a7365 = 0u64;
                nat_rem(a7363, a7364, a7365)
            };
            if v0 {
                let a7366 = v12;
                let a7367 = 0u64;
                if nat_eq(a7366, a7367) {
                    Ok(f309(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7368 = v12;
                    let a7369 = 1u64;
                    if nat_eq(a7368, a7369) {
                        Ok(f309(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7370 = v12;
                        let a7371 = 2u64;
                        if nat_eq(a7370, a7371) {
                            Ok(f309(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f309(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7372 = v12;
                let a7373 = 0u64;
                if nat_eq(a7372, a7373) {
                    Ok(f309(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f309(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7374 = v1;
            let a7375 = v7;
            if nat_eq(a7374, a7375) {
                Ok(f309(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f319(v0: u64, v1: u64) -> Adt17 {
    let a7376 = v0;
    let a7377 = 0u64;
    if nat_eq(a7376, a7377) {
        f309(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7378 = v0;
        let a7379 = 1u64;
        if nat_eq(a7378, a7379) {
            f309(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7380 = v0;
            let a7381 = 2u64;
            if nat_eq(a7380, a7381) {
                f309(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7382 = v0;
                let a7383 = 3u64;
                if nat_eq(a7382, a7383) {
                    f307(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f320(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7384 = v0;
    let a7385 = 0u64;
    if nat_eq(a7384, a7385) {
        f309(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7386 = v0;
        let a7387 = 1u64;
        if nat_eq(a7386, a7387) {
            f309(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7388 = v0;
            let a7389 = 2u64;
            if nat_eq(a7388, a7389) {
                f309(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7390 = v0;
                let a7391 = 3u64;
                if nat_eq(a7390, a7391) {
                    f309(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7392 = v0;
                    let a7393 = 4u64;
                    if nat_eq(a7392, a7393) {
                        f309(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7394 = v0;
                        let a7395 = 5u64;
                        if nat_eq(a7394, a7395) {
                            f309(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f321(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7406 = {
        let a7404 = {
            let a7401 = {
                let a7398 = match v1.clone() {
                    Adt21::C0(_, _, h7396, _) => {
                        h7396
                    }
                };
                let a7399 = match v2.clone() {
                    Adt20::C0(_, _, h7397, _, _, _, _, _) => {
                        h7397
                    }
                };
                nat_mul(a7398, a7399)?
            };
            let a7402 = match v1 {
                Adt21::C0(h7400, _, _, _) => {
                    h7400
                }
            };
            nat_add(a7401, a7402)?
        };
        let a7405 = match v2 {
            Adt20::C0(_, h7403, _, _, _, _, _, _) => {
                h7403
            }
        };
        nat_mul(a7404, a7405)?
    };
    let a7407 = v0;
    nat_add(a7406, a7407)
}

pub fn f322(v0: u64) -> u64 {
    if f220(v0) {
        let m7408 = f213(v0);
        match m7408 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7410 = 1u64;
                let a7411 = match v1.clone() {
                    Adt19::C0(_, h7409, _) => {
                        h7409
                    }
                };
                if if nat_lt(a7410, a7411) {
                    true
                } else {
                    let a7412 = v0;
                    let a7413 = 43u64;
                    nat_eq(a7412, a7413)
                } {
                    match v1 {
                        Adt19::C0(_, h7414, _) => {
                            h7414
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

pub fn f323(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7422 = {
            let a7420 = {
                let a7416 = v1;
                let a7417 = match v3.clone() {
                    Adt20::C0(h7415, _, _, _, _, _, _, _) => {
                        h7415
                    }
                };
                nat_mul(a7416, a7417)?
            };
            let a7421 = {
                let a7418 = v2;
                let a7419 = 4u64;
                nat_mul(a7418, a7419)?
            };
            nat_add(a7420, a7421)?
        };
        let a7423 = f326(v0);
        nat_add(a7422, a7423)?
    };
    let v6: u64 = f308(v5, v3)?;
    let v7: u64 = f308(v5, v4)?;
    let a7424 = v0;
    let a7425 = 0u64;
    if if if if nat_eq(a7424, a7425) {
        true
    } else {
        let a7426 = v0;
        let a7427 = 2u64;
        nat_eq(a7426, a7427)
    } {
        true
    } else {
        let a7428 = v0;
        let a7429 = 5u64;
        nat_eq(a7428, a7429)
    } {
        true
    } else {
        let a7430 = v0;
        let a7431 = 7u64;
        nat_eq(a7430, a7431)
    } {
        Ok(f309(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7432 = v0;
        let a7433 = 1u64;
        if if if if nat_eq(a7432, a7433) {
            true
        } else {
            let a7434 = v0;
            let a7435 = 3u64;
            nat_eq(a7434, a7435)
        } {
            true
        } else {
            let a7436 = v0;
            let a7437 = 6u64;
            nat_eq(a7436, a7437)
        } {
            true
        } else {
            let a7438 = v0;
            let a7439 = 8u64;
            nat_eq(a7438, a7439)
        } {
            Ok(f309(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7440 = v0;
                let a7441 = 1u64;
                if nat_eq(a7440, a7441) {
                    4u64
                } else {
                    let a7442 = v0;
                    let a7443 = 6u64;
                    if nat_eq(a7442, a7443) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7444 = v0;
            let a7445 = 4u64;
            if nat_eq(a7444, a7445) {
                Ok(f309(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7446 = v0;
                let a7447 = 9u64;
                if nat_eq(a7446, a7447) {
                    Ok(f309(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7448 = v0;
                    let a7449 = 10u64;
                    if nat_eq(a7448, a7449) {
                        Ok(f309(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7450 = v0;
                        let a7451 = 11u64;
                        if nat_eq(a7450, a7451) {
                            Ok(f309(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7452 = v0;
                            let a7453 = 12u64;
                            if nat_eq(a7452, a7453) {
                                Ok(f309(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f324(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f308(v1, v2)?;
    let v6: u64 = f308(v1, v3)?;
    let a7454 = v0;
    let a7455 = 0u64;
    if nat_eq(a7454, a7455) {
        Ok(f309(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7456 = v0;
        let a7457 = 1u64;
        if nat_eq(a7456, a7457) {
            Ok(f309(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7458 = v0;
                let a7459 = 2u64;
                if nat_eq(a7458, a7459) {
                    Ok(f309(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7460 = v0;
                    let a7461 = 3u64;
                    if nat_eq(a7460, a7461) {
                        Ok(f309(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7462 = v0;
                let a7463 = 2u64;
                if nat_eq(a7462, a7463) {
                    Ok(f309(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f325(v0: u64) -> R<u64> {
    let a7464 = 4u64;
    let a7465 = f301(v0);
    nat_mul(a7464, a7465)
}

pub fn f326(v0: u64) -> u64 {
    let a7466 = v0;
    let a7467 = 5u64;
    if nat_lt(a7466, a7467) {
        let a7468 = v0;
        let a7469 = 2u64;
        if nat_lt(a7468, a7469) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7470 = v0;
        let a7471 = 7u64;
        if nat_lt(a7470, a7471) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
